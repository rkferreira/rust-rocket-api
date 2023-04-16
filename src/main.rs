#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod routes;
mod services;

use crate::routes::hello::*;
use crate::routes::root::*;


fn main() {
  let rs = rocket::ignite().mount("/api", routes![hello]);
  rs.mount("/", routes![root]).launch();
}
