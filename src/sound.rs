// A ring buffer connected to a DAC
// We need three things to configure it
// Number of samples, sample bit size, and drain rate in Hz
// Like usual, we drive the thing with a stepper
// It should drain once every n steps, where m is the
// world clock Hz, defined as the LCM of all clocks in the system,
// d is the drain rate in Hz, and n = m / d
//use std::cell::RefCell;
use std::sync::Arc;

pub struct RingBuffer {
  pub index: usize,
  pub size: usize,
  pub contents: Vec<u64>,
  pub speed: u32, // ticks per action
  pub progress: u32,
  pub source: Arc<(FnMut() -> (u64))>,
  pub drain: Arc<(FnMut(u64) -> ())>,
  pub width: u8,
}

pub fn ticker(forward_by: u32, mut rb: RingBuffer) {
  let mask: u64 = if rb.width >= 64 {
    0b11111111111111111111111111111111_11111111111111111111111111111111
  } else {
    (1 << rb.width) - 1
  };
  let mut time = forward_by + rb.progress;
  while (time > rb.speed) {
    time -= rb.speed;
    match (Arc::get_mut(&mut rb.drain)) {
      None => panic!("!!!"),
      Some(x) => x(mask & rb.contents[rb.index]),
    };
    rb.contents[rb.index] = mask & (match Arc::get_mut(&mut rb.source) { None => panic!("!!!"), Some(x) => x(), });
    if rb.index < rb.size {
      rb.index += 1;
    } else {
      rb.index = 0;
    }
  }
}
