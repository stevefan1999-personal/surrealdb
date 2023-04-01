use once_cell::sync::OnceCell;
use std::net::SocketAddr;

pub static CF: OnceCell<Config> = OnceCell::new();

#[derive(Clone, Debug)]
pub struct Config {
	pub strict: bool,
	pub bind: SocketAddr,
	pub path: String,
	pub user: String,
	pub pass: Option<String>,
	pub crt: Option<String>,
	pub key: Option<String>,
}
