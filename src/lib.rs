/* * * * * * * * * * * * * * * * * * * *
 * I N C L U D E S
 */

pub mod register;

#[macro_use] extern crate serde_derive;

extern crate hyper;

/* * * * * * * * * * * * * * * * * * * *
 * T Y P E S
 */

#[derive(Serialize, Deserialize, Debug)]
pub enum Error {
    Matrix {
        #[serde(skip_serializing, skip_deserializing)]
        rest_errorcode: u16,
        errorcode: String,
        error: String,
    },
    Transport,
    Query,
}

/*
 * Request is the base Trait that all API endpoint requests implement
 */
pub trait Request {
    type R: Response;
    type A: Arguments;
    fn issue(&self, url: String, args: Self::A) -> Result<Self::R, Error>;
}

/*
 * Request is the base Trait that all API endpoint responses implement
 */
pub trait Response {
    fn is_valid() -> bool;
}


pub struct Query<A: Arguments, R: Request> {
    url: String,
    type: 
    arguments: A,
    request: R,
}

pub trait Arguments {

}