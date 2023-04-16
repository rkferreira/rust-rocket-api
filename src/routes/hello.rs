
use crate::services;

#[get("/sayhello")]
pub fn hello() -> &'static str {
  services::hello::say_hello()
}
