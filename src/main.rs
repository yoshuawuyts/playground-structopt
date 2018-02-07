extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "boats")]
enum Cmds {
  #[structopt(name = "add", about = "Add a thing")]
  Add {
    #[structopt(short = "i")]
    interactive: bool,
    #[structopt(short = "p")]
    patch: bool,
    files: Vec<String>,
  },
  #[structopt(name = "fetch", about = "Fetch a thing")]
  Fetch {
    #[structopt(long = "dry-run")]
    dry_run: bool,
    #[structopt(long = "all")]
    all: bool,
    repository: Option<String>,
  },
  #[structopt(name = "commit", about = "Commit a thing")]
  Commit {
    #[structopt(short = "m")]
    message: Option<String>,
    #[structopt(short = "a")]
    all: bool,
  },
}

fn main() {
  match Cmds::from_args() {
    Cmds::Add {
      interactive,
      patch,
      files,
    } => {
      let opts = Cmds::Add {
        interactive,
        patch,
        files,
      };
      // this is where we'd pass the struct to some_function
      println!("{:?}", opts);
      1
    }
    Cmds::Fetch {
      repository,
      dry_run,
      all,
    } => {
      let opts = Cmds::Fetch {
        dry_run,
        all,
        repository,
      };
      // this is where we'd pass the struct to some_other_function
      println!("{:?}", opts);
      1
    }
    _ => {
      println!("nop");
      1
    }
  };
}
