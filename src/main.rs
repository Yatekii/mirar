extern crate mirar;
extern crate iron;
extern crate router;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use mirar::request::Register;
use mirar::request::Auth;

fn main() {
    let reg = Register {
        username: "KEK".to_string(),
        bind_email: false,
        password: "YOLO".to_string(),
        auth: Auth {
            session: "FOO".to_string(),
            typ: "bar".to_string()
        },
    };

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&reg).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);
}