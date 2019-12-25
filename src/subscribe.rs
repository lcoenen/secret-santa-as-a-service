extern crate hyper;

use hyper::{Body, Request, Response};
use crate::hyper::rt::Stream;
use std::str;
use crate::hyper::rt::Future;
use std::thread;

use serde::{Serialize, Deserialize};

extern crate redis;
use redis::Commands;
use toml::ser::to_string;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    username: String,
    password: String,
    email: String
}

fn data(_req: Request<Body>) -> String {
    let (_parts, body) = _req.into_parts();
    let chunks = body.collect().wait().unwrap(); 
    let strings: Vec<String> = chunks.into_iter().map(|chunk| {
        let buf: Vec<u8> = chunk.into_bytes().into_iter().collect();
        String::from(str::from_utf8(&buf).unwrap())
    }).collect();
    String::from(strings.concat())
}

pub fn handle(_req: Request<Body>) -> Response<Body> {
    thread::spawn(|| {

        let client = redis::Client::open("redis://127.0.0.1/").unwrap();
        let mut con = client.get_connection().unwrap();

        let request_string = data(_req);
        let user: User = toml::from_str(&request_string).unwrap();

        println!("Request data is {}", request_string);
        println!("username is {}", user.username);
        println!("password is {}", user.password);
        println!("email is {}", user.email);

        let user_string = to_string(&user).unwrap();

        let _ : () = con.hset("users", user.username, user_string).unwrap();
    });

    Response::new(Body::from("Subscribed"))
}
