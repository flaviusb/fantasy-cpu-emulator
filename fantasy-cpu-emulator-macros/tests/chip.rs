#[macro_use]
extern crate fantasy_cpu_emulator_macros;

#[test]
fn define_blank_chip() {
  define_chip! {
    #test_potato

  };
  assert_eq!(true, true);
}