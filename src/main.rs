extern crate mirar;
extern crate iron;
extern crate router;
extern crate urlencoded;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use std::env;
use std::io::Read;

use iron::prelude::*;
use urlencoded::UrlEncodedQuery;

use mirar::Request;

const URL: &'static str = "localhost:8008";

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments. Please enter s/c.");
        return;
    }

    match args[1].as_str() {
        "s" => {
            let mut router = router::Router::new();

            router.post("/register", handle_register, "Register new User");

            fn handle_register(request: &mut iron::Request) -> iron::IronResult<iron::Response> {
                println!("Got /register request.");
                let kind;
                match request.get_ref::<UrlEncodedQuery>() {
                    Ok(args) => {
                        if args.contains_key("kind"){
                            kind = match args.get("kind") {
                                Some(v) => {println!("{}", v[0]); mirar::register::Kind::User},
                                _ => mirar::register::Kind::Guest,
                            };
                        }
                    },
                    Err(args) => {
                        match args {
                            urlencoded::UrlDecodingError::EmptyQuery => {

                            },
                            _ => {
                                let payload = mirar::Error::Matrix{
                                    rest_errorcode: 400,
                                    errorcode: "M_BROKEN_QUERY".into(),
                                    error: "QUERY could not be decoded".into()
                                };
                                return Ok(iron::Response::with((iron::status::BadRequest, serde_json::to_string(&payload).unwrap())))
                            }
                        }
                    },
                };

                let _ = kind;

                let mut payload = String::new();
                let _ = request.body.read_to_string(&mut payload);
                let request: Result<mirar::register::Request, serde_json::error::Error> = serde_json::from_str(payload.as_str());
                match request {
                    Ok(v) => {
                        let _ = v;
                        let response = mirar::register::Response {
                            access_token: "YOLO".into(),
                            home_server: "darkchannel.net".into(),
                            user_id: "yatekii".into(),
                            refresh_token: "KEK".into(),
                        };
                        
                        let body = serde_json::to_string(&response).unwrap();
                        Ok(iron::Response::with((iron::status::Ok, body)))
                    },
                    _ => {
                        let payload = mirar::Error::Matrix{
                            rest_errorcode: 400,
                            errorcode: "M_BROKEN_QUERY".into(),
                            error: "JSON could not be decoded".into()
                        };
                        Ok(iron::Response::with((iron::status::BadRequest, serde_json::to_string(&payload).unwrap())))
                    }
                }
            }

            println!("Running on {}", URL);
            iron::Iron::new(router).http(URL).unwrap();
        }
        "c" => {
            let reg = mirar::register::Request {
                username: "KEK".to_string(),
                bind_email: false,
                password: "YOLO".to_string(),
                auth: mirar::register::Auth {
                    session: "FOO".to_string(),
                    typ: "bar".to_string()
                },
            };

            match reg.issue(URL.to_string(), mirar::register::Kind::Guest){
                Ok(v) => {println!("{}", v.user_id)},
                Err(v) => {
                    match v {
                        mirar::Error::Matrix { rest_errorcode: _, errorcode: _, error } => println!("{}", error),
                        _ => {},
                    };
                }
            };

            let serialized = serde_json::to_string(&reg).unwrap();
            println!("serialized = {}", serialized);
        }

        _ => {
            println!("Argument was not understood. Please enter s/c.");
            return;
        }
    }
}