extern crate proc_macro;
extern crate syn;
#[macro_use] extern crate quote;
extern crate proc_macro2;

#[macro_use] extern crate fantasy_cpu_emulator_macros;

mod potato;
use potato::potato_chip;

mod unpipelined_potato;
use unpipelined_potato::unpipelined_potato_chip;
