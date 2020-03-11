#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

extern crate fizzbuzz;

#[get("/")]
fn index() -> &'static str {
    "Hello, world"
}

#[get("/fizzbuzz/<num>")]
fn fizzbuzz(num: i64) -> String {
    fizzbuzz::fizzbuzz(num)
}

fn main() {
    rocket::ignite().mount("/", routes![index, fizzbuzz]).launch();
}
