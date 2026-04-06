//! Shared query runtime utilities extracted from `surrealdb-core`.

pub mod access_mode {
	#![allow(dead_code)]

	/// Access mode for a plan or expression.
	#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
	pub enum AccessMode {
		#[default]
		ReadOnly,
		ReadWrite,
	}

	impl AccessMode {
		#[inline]
		pub fn combine(self, other: Self) -> Self {
			match (self, other) {
				(Self::ReadOnly, Self::ReadOnly) => Self::ReadOnly,
				_ => Self::ReadWrite,
			}
		}

		#[inline]
		pub fn is_read_only(self) -> bool {
			matches!(self, Self::ReadOnly)
		}

		#[inline]
		pub fn is_read_write(self) -> bool {
			matches!(self, Self::ReadWrite)
		}
	}

	pub trait CombineAccessModes: Iterator<Item = AccessMode> {
		fn combine_all(self) -> AccessMode;
	}

	impl<I: Iterator<Item = AccessMode>> CombineAccessModes for I {
		fn combine_all(self) -> AccessMode {
			self.fold(AccessMode::ReadOnly, AccessMode::combine)
		}
	}
}

pub mod join_buffered {
	use std::future::Future;
	use std::mem;
	use std::pin::Pin;
	use std::task::{Context, Poll};

	use futures::future::IntoFuture;
	use futures::stream::FuturesOrdered;
	use futures::{TryFuture, TryFutureExt, TryStream, ready};
	use pin_project_lite::pin_project;

	pin_project! {
		#[must_use = "futures do nothing unless you `.await` or poll them"]
		pub struct TryJoinAllBuffered<F, I>
		where
			F: TryFuture,
			I: Iterator<Item = F>,
		{
			input: I,
			#[pin]
			active: FuturesOrdered<IntoFuture<F>>,
			output: Vec<F::Ok>,
		}
	}

	pub fn try_join_all_buffered_with_limit<I>(
		iter: I,
		limit: usize,
	) -> TryJoinAllBuffered<I::Item, I::IntoIter>
	where
		I: IntoIterator,
		I::Item: TryFuture,
	{
		let mut input = iter.into_iter();
		let (lo, hi) = input.size_hint();
		let initial_capacity = hi.unwrap_or(lo);
		let mut active = FuturesOrdered::new();

		while active.len() < limit {
			if let Some(next) = input.next() {
				active.push_back(TryFutureExt::into_future(next));
			} else {
				break;
			}
		}

		TryJoinAllBuffered {
			input,
			active,
			output: Vec::with_capacity(initial_capacity),
		}
	}

	impl<F, I> Future for TryJoinAllBuffered<F, I>
	where
		F: TryFuture,
		I: Iterator<Item = F>,
	{
		type Output = Result<Vec<F::Ok>, F::Error>;

		fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
			let mut this = self.project();
			Poll::Ready(Ok(loop {
				match ready!(this.active.as_mut().try_poll_next(cx)?) {
					Some(x) => {
						if let Some(next) = this.input.next() {
							this.active.push_back(TryFutureExt::into_future(next));
						}
						this.output.push(x)
					}
					None => break mem::take(this.output),
				}
			}))
		}
	}
}
