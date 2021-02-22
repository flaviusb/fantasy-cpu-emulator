extern crate proc_macro;
extern crate syn;
#[macro_use] extern crate quote;
extern crate proc_macro2;

#[macro_use] extern crate fantasy_cpu_emulator_macros;
extern crate getopts;

//extern crate arraydeque;
//extern crate generic_array;
extern crate dasp_ring_buffer;

mod potato;
use potato::potato_chip;

mod unpipelined_potato;
use unpipelined_potato::unpipelined_potato_chip;

pub mod jackfruit;
use jackfruit::jackfruit_chip;

pub mod sound;
//use sound::RingBuffer;

pub mod clock;
use clock::Clocked;
