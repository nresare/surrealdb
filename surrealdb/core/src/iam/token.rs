use std::collections::HashMap;
use std::sync::LazyLock;

use anyhow::{Result, bail};
use jsonwebtoken::{Algorithm, Header};
use surrealdb_types::SurrealValue;

use crate::dbs::Session;
use crate::err::Error;
use crate::kvs::Datastore;
use crate::val::{Object, Value, convert_object_to_public_map};
use crate::sql::expression::convert_public_value_to_internal;
use crate::iam;

pub use surrealdb_sdk_core::iam::token::Token;

pub static HEADER: LazyLock<Header> = LazyLock::new(|| Header::new(Algorithm::HS512));

fn decode_access_token_claims(token: &str) -> Result<jsonwebtoken::TokenData<Claims>> {
	Ok(jsonwebtoken::dangerous::insecure_decode::<Claims>(token)?)
}

pub async fn refresh(token: Token, kvs: &Datastore, session: &mut Session) -> Result<Token> {
	match token {
		Token::Access(_) => bail!(Error::InvalidFunctionArguments {
			name: "refresh".into(),
			message: "Token is an access token, cannot refresh".into(),
		}),
		Token::WithRefresh {
			access,
			refresh,
		} => {
			let token_data = decode_access_token_claims(&access)?;
			let claims = token_data.claims.into_claims_object();
			let mut vars = convert_object_to_public_map(claims)?;
			vars.insert("refresh".to_string(), refresh.into_value());
			iam::signin::signin(kvs, session, vars.into()).await
		}
	}
}

pub async fn revoke_refresh_token(token: Token, kvs: &Datastore) -> Result<()> {
	match token {
		Token::Access(_) => bail!(Error::InvalidFunctionArguments {
			name: "refresh".into(),
			message: "Token is an access token, cannot revoke refresh token".into(),
		}),
		Token::WithRefresh {
			access,
			refresh,
		} => {
			let grant_id = iam::signin::validate_grant_bearer(&refresh)?;
			let token_data = decode_access_token_claims(&access)?;
			let ns = token_data.claims.ns.ok_or_else(|| Error::InvalidFunctionArguments {
				name: "ns".into(),
				message: "Token does not contain a namespace".into(),
			})?;
			let db = token_data.claims.db.ok_or_else(|| Error::InvalidFunctionArguments {
				name: "db".into(),
				message: "Token does not contain a database".into(),
			})?;
			let ac = token_data.claims.ac.ok_or_else(|| Error::InvalidFunctionArguments {
				name: "ac".into(),
				message: "Token does not contain an access name".into(),
			})?;
			iam::access::revoke_refresh_token_record(kvs, grant_id, ac, &ns, &db).await?;
			Ok(())
		}
	}
}

#[derive(Default, Clone, Debug)]
pub(crate) struct Claims {
	pub(crate) iss: Option<String>,
	pub(crate) iat: Option<i64>,
	pub(crate) nbf: Option<i64>,
	pub(crate) exp: Option<i64>,
	pub(crate) jti: Option<String>,
	pub(crate) ns: Option<String>,
	pub(crate) db: Option<String>,
	pub(crate) ac: Option<String>,
	pub(crate) id: Option<String>,
	pub(crate) roles: Option<Vec<String>>,
	pub(crate) custom_claims: HashMap<String, surrealdb_types::Value>,
}

impl Claims {
	pub(crate) fn into_claims_object(self) -> Object {
		let mut out = Object::default();
		if let Some(v) = self.iss {
			out.insert("iss".to_string(), Value::from(v));
		}
		if let Some(v) = self.iat {
			out.insert("iat".to_string(), Value::from(v));
		}
		if let Some(v) = self.nbf {
			out.insert("nbf".to_string(), Value::from(v));
		}
		if let Some(v) = self.exp {
			out.insert("exp".to_string(), Value::from(v));
		}
		if let Some(v) = self.jti {
			out.insert("jti".to_string(), Value::from(v));
		}
		if let Some(v) = self.ns {
			out.insert("ns".to_string(), Value::from(v));
		}
		if let Some(v) = self.db {
			out.insert("db".to_string(), Value::from(v));
		}
		if let Some(v) = self.ac {
			out.insert("ac".to_string(), Value::from(v));
		}
		if let Some(v) = self.id {
			out.insert("id".to_string(), Value::from(v));
		}
		if let Some(v) = self.roles {
			let roles = v.into_iter().map(Value::from).collect::<Vec<_>>();
			out.insert("roles".to_string(), Value::from(roles));
		}
		for (k, v) in self.custom_claims {
			let v = convert_public_value_to_internal(v);
			out.insert(k, v);
		}
		out
	}
}

impl serde::Serialize for Claims {
	fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		let mut claims = serde_json::Map::new();
		if let Some(v) = &self.iss {
			claims.insert("iss".into(), v.clone().into());
		}
		if let Some(v) = self.iat {
			claims.insert("iat".into(), v.into());
		}
		if let Some(v) = self.nbf {
			claims.insert("nbf".into(), v.into());
		}
		if let Some(v) = self.exp {
			claims.insert("exp".into(), v.into());
		}
		if let Some(v) = &self.jti {
			claims.insert("jti".into(), v.clone().into());
		}
		if let Some(v) = &self.ns {
			claims.insert("ns".into(), v.clone().into());
		}
		if let Some(v) = &self.db {
			claims.insert("db".into(), v.clone().into());
		}
		if let Some(v) = &self.ac {
			claims.insert("ac".into(), v.clone().into());
		}
		if let Some(v) = &self.id {
			claims.insert("id".into(), v.clone().into());
		}
		if let Some(v) = &self.roles {
			claims.insert("roles".into(), serde_json::to_value(v).map_err(serde::ser::Error::custom)?);
		}
		for (k, v) in &self.custom_claims {
			claims.insert(k.clone(), serde_json::to_value(v).map_err(serde::ser::Error::custom)?);
		}
		claims.serialize(serializer)
	}
}

impl<'de> serde::Deserialize<'de> for Claims {
	fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let mut map = serde_json::Map::<String, serde_json::Value>::deserialize(deserializer)?;
		let iss = map.remove("iss").map(serde_json::from_value).transpose().map_err(serde::de::Error::custom)?;
		let iat = map.remove("iat").map(serde_json::from_value).transpose().map_err(serde::de::Error::custom)?;
		let nbf = map.remove("nbf").map(serde_json::from_value).transpose().map_err(serde::de::Error::custom)?;
		let exp = map.remove("exp").map(serde_json::from_value).transpose().map_err(serde::de::Error::custom)?;
		let jti = map.remove("jti").map(serde_json::from_value).transpose().map_err(serde::de::Error::custom)?;
		let ns = map.remove("ns").map(serde_json::from_value).transpose().map_err(serde::de::Error::custom)?;
		let db = map.remove("db").map(serde_json::from_value).transpose().map_err(serde::de::Error::custom)?;
		let ac = map.remove("ac").map(serde_json::from_value).transpose().map_err(serde::de::Error::custom)?;
		let id = map.remove("id").map(serde_json::from_value).transpose().map_err(serde::de::Error::custom)?;
		let roles = map.remove("roles").map(serde_json::from_value).transpose().map_err(serde::de::Error::custom)?;
		let custom_claims = map
			.into_iter()
			.map(|(k, v)| {
				let value =
					serde_json::from_value::<surrealdb_types::Value>(v).map_err(serde::de::Error::custom)?;
				Ok((k, value))
			})
			.collect::<std::result::Result<HashMap<_, _>, D::Error>>()?;
		Ok(Self {
			iss,
			iat,
			nbf,
			exp,
			jti,
			ns,
			db,
			ac,
			id,
			roles,
			custom_claims,
		})
	}
}
