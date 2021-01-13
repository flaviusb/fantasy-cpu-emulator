use fantasy_cpu_emulator::jackfruit;

use getopts::{Options,Matches};
use std::{env,process};

fn main() {
  let mut opts = Options::new();
  //opts.optopt("f", "foo", "do foo stuff", "NAME");
  //opts.optflag("f", "foo", "do foo stuff");
  opts.optflag("h", "help", "print this help menu");
  opts.optopt("c", "chip", "select chip", "jackfruit");
  opts.optopt("i", "in", "assembly language file", "INPUT");
  opts.optopt("o", "out", "output file", "OUTPUT");
  let args: Vec<String> = env::args().skip(1).collect();
  let matches = match opts.parse(args.clone()) {
    Err(err) => {
      println!("{}", err);
      process::exit(1);
    },
    Ok(matches) => {
      matches
    },
  };
  let chip   = matches.opt_str("c");
  let input  = matches.opt_str("i");
  let output = matches.opt_str("o");
  if matches.opt_present("h") | (chip == None) | (input == None) | (output == None) {
    // Print usage
    print!("{}", opts.usage("Usage: fantasy-assembler [OPTION]"));
    process::exit(if matches.opt_present("h") | (matches.free.len() == args.len()) { 0 } else { 1 });
  }
  match chip {
    Some(x) => {
      let foo = x.as_str();
      match foo {
          "jackfruit" => {
        },
        x => {
          println!("No such chip: {}\n Options: jackfruit", x);
          process::exit(1);
        },
      }
    },
    None => {
      panic!("No chip given.");
    },
  }
}
