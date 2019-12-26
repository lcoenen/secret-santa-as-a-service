extern crate hyper;

use hyper::{Body, Request, Response};
use crate::hyper::rt::Stream;
use crate::hyper::rt::Future;
use std::str;
use std::thread;

use serde::{Serialize, Deserialize};

extern crate redis;
use redis::Commands;
// use toml::ser::to_string;

use crate::interface::User;

fn data(_req: Request<Body>) -> String {
        
    let (_parts, body) = _req.into_parts();
    let chunks = body.collect().wait().unwrap(); 
    let strings: Vec<String> = chunks.into_iter().map(|chunk| {
        let buf: Vec<u8> = chunk.into_bytes().into_iter().collect();
        String::from(str::from_utf8(&buf).unwrap())
    }).collect();
    String::from(strings.concat())
}


#[derive(Serialize, Deserialize, Debug)]
struct CheckResult {
   partner: String 
}

pub fn handle(_req: Request<Body>) -> Response<Body> {
    let handle = thread::spawn(|| {

        let client = redis::Client::open("redis://127.0.0.1/").unwrap();
        let mut con = client.get_connection().unwrap();

        let request_string = data(_req);
        let user: User = toml::from_str(&request_string).unwrap();

        let found_string = con.hget("partners", user.username).unwrap();
        
        let result = CheckResult {
            partner: found_string
        };

        toml::to_string(&result).unwrap()
    });

    Response::new(Body::from(handle.join().unwrap()))
}
