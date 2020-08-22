mod chip;
mod sound;

fn main() {
    let mut chip: chip::Chip = chip::Chip {
      scratch: chip::set_chip_mem(),
      memstate: chip::set_chip_masks_paused(),
    };
    println!("Hello, world!");
}
