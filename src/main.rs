extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic-example")]
struct Opt {
  #[structopt(short = "d", long = "debug")]
  debug: bool,

  #[structopt(short = "s", long = "speed", default_value = "42")]
  speed: f64,
}

fn main() {
  let opt = Opt::from_args();
  println!("{:?}", opt);
}
