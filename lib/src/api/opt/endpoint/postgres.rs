use crate::api::engine::local::Db;
use crate::api::engine::local::Postgres;
use crate::api::err::Error;
use crate::api::opt::Endpoint;
use crate::api::opt::IntoEndpoint;
use crate::api::opt::Strict;
use crate::api::Result;
use std::net::SocketAddr;
use url::Url;

impl IntoEndpoint<Postgres> for &str {
	type Client = Db;

	fn into_endpoint(self) -> Result<Endpoint> {
		let url = format!("postgres://{self}");
		Ok(Endpoint {
			endpoint: Url::parse(&url).map_err(|_| Error::InvalidUrl(url))?,
			strict: false,
			#[cfg(any(feature = "native-tls", feature = "rustls"))]
			tls_config: None,
		})
	}
}

impl IntoEndpoint<Postgres> for SocketAddr {
	type Client = Db;

	fn into_endpoint(self) -> Result<Endpoint> {
		let url = format!("postgres://{self}");
		Ok(Endpoint {
			endpoint: Url::parse(&url).map_err(|_| Error::InvalidUrl(url))?,
			strict: false,
			#[cfg(any(feature = "native-tls", feature = "rustls"))]
			tls_config: None,
		})
	}
}

impl IntoEndpoint<Postgres> for String {
	type Client = Db;

	fn into_endpoint(self) -> Result<Endpoint> {
		let url = format!("postgres://{self}");
		Ok(Endpoint {
			endpoint: Url::parse(&url).map_err(|_| Error::InvalidUrl(url))?,
			strict: false,
			#[cfg(any(feature = "native-tls", feature = "rustls"))]
			tls_config: None,
		})
	}
}

impl<T> IntoEndpoint<Postgres> for (T, Strict)
where
	T: IntoEndpoint<Postgres>,
{
	type Client = Db;

	fn into_endpoint(self) -> Result<Endpoint> {
		let mut address = self.0.into_endpoint()?;
		address.strict = true;
		Ok(address)
	}
}
