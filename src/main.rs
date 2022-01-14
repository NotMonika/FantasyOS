#![no_std]
#![no_main]
#![feature(abi_efiapi)]

#[macro_use]
use log::*;

use uefi::prelude::*;

#[entry]
fn efi_main(handle: Handle, system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&system_table).expect_success("failed to initialize utilities");
    info!("Hello UEFI!");
    loop {}
}
