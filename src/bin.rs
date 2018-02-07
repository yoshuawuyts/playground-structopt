#[macro_use]
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
  debug: bool,
  speed: f64,
}

fn main() {}
