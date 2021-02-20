/*#![feature(min_const_generics)]

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
//use arraydeque::{ArrayDeque, Wrapping, Array};
use dasp_ring_buffer::Bounded as RingBuffer;

pub struct SoundConnection<W, S, const D: u8> {
  dac: DAC<W, D>,
  rb:  RingBuffer<S>,
}

pub trait DAC<W, const D: u8> {
  pub fn put(&mut self, out: W);
}

// A RingBuffer is an ArrayDeque<[T; N], Wrapping> for N elements of T
// let mut rb: ArrayDeque<[u16; 64], Wrapping> = ArrayDeque::new();
// rb.push_back(x) to put stuff on
// rb.pop_front().unwrap_or(0) to get the next line level amount

// DAC is a sink.
//

/*
pub trait Sample {}


pub trait DACType {
}

pub trait<W> DAC<W> {
  pub fn put(&mut self, out: W);
}

pub struct DevNull {}
impl DACType for DevNull {}

pub struct FileOutput {
  file: Arc<File>,
}
impl DACType for FileOutput {}

pub struct DAC<W = u16, T: DACType = DevNull> {
  x: T,
  phantom: std::marker::PhantomData::<W>,
}

impl<W> DAC<W, DevNull> {
  pub fn new() -> Self {
    Self { x: DevNull{}, phantom: std::marker::PhantomData::<W>, }
  }
  pub fn put(&mut self, out: W) {
  }
}

impl<W> DAC<W, FileOutput> {
  pub fn new(path: String) -> Self {
    let mut file: Arc<File> = Arc::new(File::create(path).unwrap());
    Self { x: FileOutput{file: file}, phantom: std::marker::PhantomData::<W>, }
  }
}
impl DAC<u32, FileOutput> {
  pub fn put(&mut self, out: u32) {
    (*Arc::get_mut(&mut self.x.file).unwrap()).write(&u32::to_le_bytes(out));
  }
}

pub trait ToLeBytes<const N: usize> {
  fn to_le_bytes(self) -> GenericArray<u8, N>;
}
impl ToLeBytes<4> for u32 {
  fn to_le_bytes(self) -> [u8; 4] {
    u32::to_le_bytes(self)
  }
}


pub struct RingBuffer<W, F: DACType, T, const N: usize, const DAC_BITS: u8, const RB_BITS: u8> {
  pub dac: DAC<W, F>,
  pub rb:  Arc<ArrayDeque<[T; N], Wrapping>>,
}

impl <F: DACType, const N: usize, const DAC_BITS: u8, const RB_BITS: u8> RingBuffer<u32, F, u32, N, DAC_BITS, RB_BITS> {
  pub fn go(&mut self) {
    //let num_bytes = rb_bitwidth
    let mask_rb: u32 = (1 << RB_BITS) - 1;
    let mask_dac: u32 = (1 << DAC_BITS) - 1;
    let out = mask_rb & mask_dac & (*Arc::get_mut(&mut (self.rb)).unwrap()).pop_front().unwrap_or(0);
    self.dac.put(out);
  }
}
*/
*/
