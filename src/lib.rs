//! A simple exporter for [prometrics](https://docs.rs/prometrics/) to create an
//! endpoint for Prometheus to scrape.
//!
//! # Example
//! ```rust
//! fn main() {
//!     prometrics_exporter::start("127.0.0.1:9091").unwrap();
//!
//!     my_mod::do_thing();
//! }
//!
//! mod my_mod {
//!     use prometrics::metrics::CounterBuilder;
//!
//!     pub fn do_thing() {
//!         let counter = CounterBuilder::new("tasks").finish().unwrap();
//!
//!         loop {
//!             let result = work();
//!             counter.add_u64(result.completed);
//!         }
//!     }
//! }
//! ```
//!
//! This will listen on http://localhost:9091/metrics and serve all metrics available
//! to the default getherer.
#[macro_use] extern crate log;
#[macro_use] extern crate rouille;

extern crate prometrics;

use std::thread;
use std::error::Error;
use std::fmt::Display;
use std::net::ToSocketAddrs;
use std::sync::Mutex;

/// Start a HTTP thread serving metrics accessible by the default gatherer
pub fn start<T: ToSocketAddrs + Display>(addr: T) -> Result<(), Box<dyn Error + Send + Sync>> {
    debug!("Starting prometrics_exporter on {}", addr);

    _start_server(addr, prometrics::default_gatherer())
}

/// Start a HTTP thread serving metrics accessible by a user-specified gatherer
pub fn start_with_gatherer<T: ToSocketAddrs + Display>(addr: T, gatherer: &'static Mutex<prometrics::Gatherer>) -> Result<(), Box<dyn Error + Send + Sync>> {
    debug!("Starting prometrics_exporter on {}", addr);

    _start_server(addr, gatherer)
}

fn _start_server<T: ToSocketAddrs>(addr: T, gatherer: &'static Mutex<prometrics::Gatherer>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let server = rouille::Server::new(addr, move |req| {
        // Quick "router"
        router!(
            req,
            (GET) (/metrics) => {
                trace!("{}: {} {}", req.remote_addr(), req.method(), req.raw_url());

                let data = {
                    let d = gatherer.lock().unwrap().gather();
                    d.to_text()
                };

                rouille::Response::text(data)
            },

            _ => rouille::Response::empty_404()
        )
    })?;

    thread::spawn(move || server.run());
    Ok(())
}
