#[get("/")]
pub fn root() -> &'static str {
  "Root server"
}
