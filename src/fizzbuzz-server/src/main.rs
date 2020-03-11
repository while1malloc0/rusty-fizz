#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;

extern crate fizzbuzz;
extern crate metrics;

use fizzbuzz::FizzBuzzable;
use metrics::Metrics;

lazy_static! {
    static ref METRICS: Metrics = Metrics::new();
}

#[get("/")]
fn index() -> &'static str {
    METRICS.total_requests.inc();
    "You've reached John's first Rust project."
}

#[get("/fizzbuzz/<num>")]
fn fizzbuzz_handler(num: i64) -> String {
    METRICS.total_requests.inc();
    num.fizzbuzz()
}

fn main() {
    rocket::ignite().mount("/", routes![index, fizzbuzz_handler]).launch();
}
