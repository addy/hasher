extern crate actix_web;
extern crate fasthash;
extern crate redis;
extern crate dotenv;

use fasthash::*;
use redis::Commands;
use actix_web::{server, error, App, Json, Result, http, Path};

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate failure;

#[derive(Serialize, Deserialize)]
struct Pair {
    key: String,
    value: String,
}

#[derive(Fail, Debug)]
#[fail(display="get error")]
struct GetError {
   name: &'static str
}

impl error::ResponseError for GetError {}

fn set(conn: &redis::Connection, key: String, value: String) -> redis::RedisResult<()> {
    let _ : () = conn.set_ex(key, value, 86400).expect("Could not set value");
    Ok(())
}

fn get(conn: &redis::Connection, key: String) -> Option<Json<Pair>> {
    if let Ok(value) = conn.get(&key) {
        return Some(Json(Pair{key: key, value: value}));
    }
    None
}
    
fn put_value(value: Path<(String)>) -> Result<String> {
    let client = redis::Client::open("redis://127.0.0.1/").expect("Could not open connection to Redis");
    let conn = client.get_connection().expect("Could not open connection to Redis");
    let val = value.into_inner();
    let key = spooky::hash128(&val);
    set(&conn, key.to_string(), val).expect("Could not set value");
    Ok(key.to_string())
}

fn get_key(key: Path<(String)>) -> Result<Json<Pair>, GetError> {
    let client = redis::Client::open("redis://127.0.0.1/").expect("Could not open connection to Redis");
    let conn = client.get_connection().expect("Could not open connection to Redis");
    let value = get(&conn, key.into_inner());
    let ret = match value {
        Some(x) => Ok(x),
	None => Err(GetError{name: "Could not get key"})
    };
    return ret;
}

fn main() {
    server::new(
        || App::new()
            .resource("/put/{value}", |r| r.method(http::Method::POST).with(put_value))
            .resource("/get/{key}", |r| r.method(http::Method::GET).with(get_key)))
            .bind("0.0.0.0:1338").expect("Can not bind to 0.0.0.0:1338")
            .run();
}
