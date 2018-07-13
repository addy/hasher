extern crate actix_web;
extern crate fasthash;

use fasthash::*;
use actix_web::{server, App, Result, http, Path};

fn hasher(value: Path<(String)>) -> Result<String> {
    Ok(format!("{}", spooky::hash128(&value.into_inner())))
}

fn main() {
    server::new(
        || App::new()
            .resource("/{value}", |r| r.method(http::Method::GET).with(hasher)))
            .bind("0.0.0.0:1338").expect("Can not bind to 0.0.0.0:1338")
            .run();
}
