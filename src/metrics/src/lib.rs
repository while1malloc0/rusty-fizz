extern crate prometheus;

use prometheus::{Counter, Opts};

pub struct Metrics {
    pub total_requests: Counter
}

impl Metrics {
    pub fn new() -> Self {
        let counter_opts = Opts::new("total_requests", "The number of requests made");
        let total_requests = Counter::with_opts(counter_opts).unwrap();
        Metrics{total_requests: total_requests}
    }
}
