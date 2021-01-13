use fantasy_cpu_emulator::jackfruit;

use getopts::{Options,Matches};
use std::{env,process};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

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
  let thingin = match input {
    Some(x) => x,
    None    => panic!("No input path!"),
  };
  let path_in = Path::new(&thingin);
  let thingout = match output {
    Some(x) => x,
    None    => panic!("No output path!"),
  };
  let path_out = Path::new(&thingout);
  match chip {
    Some(x) => {
      let foo = x.as_str();
      match foo {
        "jackfruit" => {
          let display = path_in.display();
          // Open the path in read-only mode, returns `io::Result<File>`
          let mut file_in = match File::open(&Path::new(path_in)) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file_in) => file_in,
          };

          // Read the file contents into a string, returns `io::Result<usize>`
          let mut s = String::new();
          match file_in.read_to_string(&mut s) {
              Err(why) => panic!("couldn't read {}: {}", display, why),
              Ok(_) => {},
          }
          //println!("{}", s.clone());
          let text = String::from(s.trim());
          let mut object: [u8; 2560] = [0; 2560];
          let mut i = 0;
          for cell in jackfruit::assemble(text).iter() {
            let bytes = cell.to_le_bytes();
            object[i] = bytes[0];
            i+=1;
            object[i] = bytes[1];
            i+=1;
            object[i] = bytes[2];
            i+=1;
            object[i] = bytes[3];
            i+=1;
            object[i] = bytes[4];
            i+=1;
          }
          let display = path_out.display();
          let mut file_out = match File::create(&path_out) {
              Err(why) => panic!("couldn't create {}: {}", display, why),
              Ok(file) => file,
          };

          // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
          match file_out.write_all(&object) {
              Err(why) => panic!("couldn't write to {}: {}", display, why),
              Ok(_) => println!("successfully wrote to {}", display),
          }
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
