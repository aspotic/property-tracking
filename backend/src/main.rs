use rocket::fs::{FileServer, relative};

// // #[launch]
// // fn main() {
// //     let _ = rocket::build().mount("/", FileServer::from(relative!("../frontend/public")))
// // }


#[macro_use] extern crate rocket;

#[get("/v1.1")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from(relative!("../frontend/build/")))
        .mount("/api", routes![index])
}
