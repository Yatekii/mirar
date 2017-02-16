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
use std::collections::HashMap;

use iron::prelude::*;
extern crate serde_urlencoded;

use mirar::{ Request, RequestType, Status, QueryKeys, QueryValues };
use mirar::register::{ ResponseBody, RequestBody, ErrorResponseBody, Auth };

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
                let url = request.url.to_string();
                let mut args: Result<HashMap<QueryKeys, QueryValues>, serde_urlencoded::de::Error>;
                let args_split: Vec<&str> = (&*url).split("&").collect();
                if args_split.len() > 1 {
                    args = serde_urlencoded::from_str(args_split[1]);
                    match args {
                        Ok(v) => {
                            if v.contains_key(&QueryKeys::Kind){
                                kind = match v.get(&QueryKeys::Kind) {
                                    Some(v) => { println!("{}", "USER"); QueryValues::User },
                                    _ => QueryValues::Guest,
                                };
                            }
                        },
                        Err(v) => {
                            println!("KEKE");
                            println!("{}", v);
                            let response_body = ErrorResponseBody{
                                errorcode: "M_BROKEN_QUERY".into(),
                                error: "QUERY could not be decoded".into()
                            };
                            return Ok(iron::Response::with((iron::status::BadRequest, serde_json::to_string(&response_body).unwrap())))
                        },
                    };
                }

                // TODO: Will be used later, but not now
                let _ = kind;

                let mut payload = String::new();
                let _ = request.body.read_to_string(&mut payload);
                println!("payload={}", payload);
                let request_body: Result<mirar::register::RequestBody, serde_json::error::Error> = serde_json::from_str(payload.as_str());
                match request_body {
                    Ok(v) => {
                        let _ = v;
                        let response_body = ResponseBody {
                            access_token: "YOLO".into(),
                            home_server: "darkchannel.net".into(),
                            user_id: "yatekii".into(),
                            refresh_token: "KEK".into(),
                        };
                        Ok(iron::Response::with((iron::status::Ok, serde_json::to_string(&response_body).unwrap())))
                    },
                    _ => {
                        let payload = ErrorResponseBody {
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
            let mut args = HashMap::new();
            args.insert(QueryKeys::Kind, QueryValues::User);
            let mut reg = Request {
                url: "register".into(),
                request_type: RequestType::POST,
                arguments: args,
                request: RequestBody {
                        username: "KEK".to_string(),
                        bind_email: false,
                        password: "YOLO".to_string(),
                        auth: Auth {
                            session: "FOO".to_string(),
                            typ: "bar".to_string()
                        },
                    },
                response: ResponseBody {
                    access_token: "".into(),
                    home_server: "".into(),
                    user_id: "".into(),
                    refresh_token: "".into(),
                },
                status_code: Status::Ok,
            };

            reg.issue(URL.to_string());
            match reg.status_code {
                Status::Ok => println!("Asked register for user: {}", reg.response.user_id),
                Status::MatrixError(v) => println!("Error! Matrix response code was: {}", v),
                Status::TransportError => println!("TransportError!"),
                Status::QueryError => println!("QuerryError!"),
                //_ => println!("{}", "Fail!")
            };
            
            let serialized = serde_json::to_string(&reg.response).unwrap();
            println!("serialized = {}", serialized);
        }

        _ => {
            println!("Argument was not understood. Please enter s/c.");
            return;
        }
    }
}