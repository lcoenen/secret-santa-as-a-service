extern crate hyper;

use hyper::{Body, Request, Response};
use std::thread;
use rand::prelude::*;

extern crate redis;
use redis::Commands;
use redis::PipelineCommands;

pub fn handle(_req: Request<Body>) -> Response<Body> {
    thread::spawn(|| {

        let mut rng = rand::thread_rng();

        let client = redis::Client::open("redis://127.0.0.1/").unwrap();
        let mut con = client.get_connection().unwrap();

        let users: Vec<String> = con.hkeys("users").unwrap();
        
        let mut partners = users.clone();
        partners.shuffle(&mut rng);

        let mut pipeline = redis::pipe();
        pipeline.atomic();

        let len = users.len();

        for i in 0..len {
            let key = &partners[i];
            let partner = &partners[if i != len - 1 { i + 1 } else { 0 }];
            pipeline.hset("partners", key, partner).ignore();
        }

        let _: () = pipeline.query(&mut con).unwrap(); 

    });

    Response::new(Body::from("Started"))
}
