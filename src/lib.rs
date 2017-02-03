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

pub trait Request {
    type R: Response;
    fn issue<T>(&self, url: String, args: T) -> Result<Self::R, Error>;
}

pub trait Response {
    fn is_valid() -> bool;
}