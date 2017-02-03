/* * * * * * * * * * * * * * * * * * * *
 * I N C L U D E S
 */

pub mod register;

#[macro_use] extern crate serde_derive;

extern crate hyper;

use self::hyper::{
    Client,
    status
};

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

pub trait Request {
    fn issue<T, R>(&self, url: String, args: T) -> Result<R, Error>;
}

pub trait Response {
    fn is_valid() -> bool;
}