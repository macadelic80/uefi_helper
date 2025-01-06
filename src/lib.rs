#![no_std]

extern crate lib_efi;
pub mod protocols;

pub mod uefi_helper {
    pub use crate::protocols;
}