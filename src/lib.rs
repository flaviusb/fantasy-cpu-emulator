extern crate proc_macro;
extern crate syn;
#[macro_use] extern crate quote;
extern crate proc_macro2;

#[macro_use] extern crate fantasy_cpu_emulator_macros;
extern crate getopts;
mod potato;
use potato::potato_chip;

mod unpipelined_potato;
use unpipelined_potato::unpipelined_potato_chip;

pub mod jackfruit;
use jackfruit::jackfruit_chip;

pub mod sound;
use sound::RingBuffer;
