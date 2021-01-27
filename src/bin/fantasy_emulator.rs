use fantasy_cpu_emulator::jackfruit;
use fantasy_cpu_emulator::sound;

use getopts::{Options,Matches};
use std::{env,process};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::sync::{Arc,Mutex};

fn main() {
  //ticker();
}


/*fn ticker() {
  use fantasy_cpu_emulator::jackfruit::jackfruit_chip as jc;
  let mems: Arc<Mutex<jc::Memories::t>> = Arc::new(Mutex::new(jc::Memories::t{registers: jc::Memories::registers{ip:0}, connectors: jc::Memories::connectors { one: 0, two: 0, three: 0, four: 0 }, base:[0; 512], stall:[0; 512],}));
  let dmems = Arc::clone(&mems);
  let drn = Arc::new(move |val:u64| -> ()  {
    let mut idx: usize = 0;
    let mut data = dmems.lock().unwrap();
    idx = data.connectors.one.clone() as usize;
    data.base[idx] = val;
  });
  let smems = Arc::clone(&mems);
  let src = Arc::new(move || -> u64 { 
    let r = smems.lock().unwrap();
    r.base[r.connectors.two as usize]
  });
  let mut rb = sound::RingBuffer {
    index: 0,
    size: 60,
    contents: vec![0; 60],
    speed: 100,
    progress: 0,
    source: src,
    drain: drn,
    width: 12,
  };
  let tmems = Arc::clone(&mems);
  // initialise
  {
    let mut inner = tmems.lock().unwrap();
    inner.connectors.one = 500;
    inner.connectors.two = 502;
    inner.base[500] = 1;
    inner.base[502] = 4;
  }
  fantasy_cpu_emulator::sound::ticker(6101, &mut rb);
  {
    let inner = tmems.lock().unwrap();
    println!("Mems {:#?}, rb contents {:#?}", inner.base, rb.contents);
  }
}*/
