// A ring buffer connected to a DAC
// We need three things to configure it
// Number of samples, sample bit size, and drain rate in Hz
// Like usual, we drive the thing with a stepper
// It should drain once every n steps, where m is the
// world clock Hz, defined as the LCM of all clocks in the system,
// d is the drain rate in Hz, and n = m / d
//use std::cell::RefCell;
use std::sync::Arc;
use std::fs::File;
use std::io::Write;
use arraydeque::{ArrayDeque, Wrapping, Array};

// A RingBuffer is an ArrayDeque<[T; N], Wrapping> for N elements of T
// let mut rb: ArrayDeque<[u16; 64], Wrapping> = ArrayDeque::new();
// rb.push_back(x) to put stuff on
// rb.pop_front().unwrap_or(0) to get the next line level amount

// DAC is a sink.
//

pub trait DACType {
}

pub struct DevNull {}
impl DACType for DevNull {}

pub struct FileOutput {
  file: Arc<File>,
}
impl DACType for FileOutput {}

pub struct DAC<T: DACType = DevNull> {
  x: T,
}

impl DAC<DevNull> {
  pub fn new() -> Self {
    DAC { x: DevNull{} }
  }
  pub fn put(&mut self, out: &[u8]) {
  }
}

impl DAC<FileOutput> {
  pub fn new(path: String) -> Self {
    let mut file: Arc<File> = Arc::new(File::create(path).unwrap());
    DAC { x: FileOutput{file: file} }
  }
  pub fn put(&mut self, out: &[u8]) {
    (*Arc::get_mut(&mut self.x.file).unwrap()).write(out);
  }
}


use generic_array::{ArrayLength, GenericArray};

pub struct RingBuffer<N: ArrayLength<u32>> {
  pub dac: DAC,
  pub rb:  Arc<ArrayDeque<GenericArray<u32, N>, Wrapping>>,
}

impl<N: ArrayLength<u32>> RingBuffer<N> {
  pub fn go(&mut self) {
    
    //self.dac.put(&u32::to_le_bytes(self.rb.pop_front().unwrap_or(0)));
  }
}

