#![no_std]
#![no_main]

use wio_terminal_probe_run;

use wio_terminal as wio;
use wio::entry;

#[entry]
fn main() -> ! {
    defmt::info!("Hello, world!");
    wio_terminal_probe_run::exit()
}
