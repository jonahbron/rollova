#![no_main]
#![no_std]

use rollova as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("info");
    defmt::trace!("trace");
    defmt::warn!("warn");
    defmt::debug!("debug");
    defmt::error!("error");

    rollova::exit()
}
