/* * * * * * * * * * * * * * * * * * * *
 * I N C L U D E S
 */

pub mod register;

#[macro_use] extern crate serde_derive;
extern crate reqwest;
extern crate serde_json;
extern crate serde_urlencoded;
extern crate serde;

extern crate hyper;

use std::collections::HashMap;
use reqwest::Client;
use serde::{ Serialize, Deserialize };

/* * * * * * * * * * * * * * * * * * * *
 * T Y P E S
 */

#[derive(Serialize, Deserialize, Debug)]
pub enum Status {
    Ok,
    MatrixError(u16),
    TransportError,
    QueryError,
}

#[derive(Eq, PartialEq, Hash, Serialize, Deserialize, Debug)]
pub enum QueryKeys {
    Kind,
}

#[derive(Eq, PartialEq, Serialize, Deserialize, Debug)]
pub enum QueryValues {
    Guest,
    User,
}

/*
 * Request is the base Trait that all API endpoint requests body structs implement
 */
pub trait RequestBody {
}

/*
 * Request is the base Trait that all API endpoint response body structs implement
 */
pub trait ResponseBody {
}

/*
 * Specifies the type of a request
 */
pub enum RequestType {
    GET,
    POST,
}

/*
 *
 */
pub struct Request<T: RequestBody + Serialize, E: ResponseBody + Deserialize> {
    pub url: String,
    pub request_type: RequestType,
    pub arguments: HashMap<QueryKeys, QueryValues>,
    pub request: T,
    pub response: E,
    pub status_code: Status,
}

/*
 *
 */
impl<T: RequestBody + Serialize, E: ResponseBody + Deserialize> Request<T, E> {
    pub fn issue(&mut self, base_url: String) {
        let client = Client::new();
        let assembled_url = format!(
            "http://{}/{}?{}",
            base_url,
            self.url,
            serde_urlencoded::to_string(&self.arguments).unwrap(),
        );

        println!("{}", assembled_url);
        let body;
        let client = reqwest::Client::new().unwrap();
        let mut res = match self.request_type {
            RequestType::GET => client.get(&assembled_url),
            RequestType::POST => client.post(&assembled_url),
        };

        body = serde_json::to_string(&self.request).unwrap();
        res = res.body(body);
        self.decode_response(res.send());
    }

    fn decode_response(&mut self, response: reqwest::Result<reqwest::Response>){
        self.status_code = Status::Ok;
        match response {
            Ok(mut v) => {
                let mut body = String::new();
                match *v.status() {
                    hyper::Ok => {
                        match v.json() {
                            Ok(v) => self.response = v,
                            Err(_) => self.status_code = Status::QueryError,
                        };
                    },
                    hyper::BadRequest => {
                        match v.json() {
                            Ok(v) => self.response = v,
                            Err(_) => self.status_code = Status::QueryError,
                        };
                    },
                    _ => self.status_code = Status::MatrixError(v.status().to_u16())
                };
            },
            _ => self.status_code = Status::TransportError,
        };
    }
}