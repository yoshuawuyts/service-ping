#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]
#![cfg_attr(test, deny(warnings))]
// #![cfg_attr(test, feature(plugin))]
// #![cfg_attr(test, plugin(clippy))]

#[macro_use]
extern crate quicli;
#[macro_use]
extern crate human_panic;

extern crate actix_web;

use actix_web::{http, middleware, server, App, Path};
use quicli::prelude::*;

#[derive(Debug, StructOpt)]
struct Args {
  /// Port to listen to.
  #[structopt(short = "p", long = "port", default_value = "8080")]
  port: u16,
  /// Toggle logging to stdout.
  #[structopt(short = "s", long = "silent")]
  silent: bool,
}

fn ping(_info: Path<()>) -> String {
  String::from("pong")
}

main!(|args: Args| {
  setup_panic!();

  let url = format!("127.0.0.1:{}", args.port);
  let make_app = || {
    App::new()
      .middleware(middleware::Logger::default()) // FIXME: tweak env vars
      .route("/", http::Method::GET, ping)
  };

  let server = server::new(make_app).bind(&url)?;
  println!("listening on {}", &url);
  server.run();
});
