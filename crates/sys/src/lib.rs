//! Low-level bindings for the Flipper Zero.

#![no_std]

// Features that identify thumbv7em-none-eabihf.
// Until target_abi is stable, this also permits thumbv7em-none-eabi.
#[cfg(not(all(
    target_arch = "arm",
    target_feature = "thumb2",
    target_feature = "v7",
    target_feature = "dsp",
    target_os = "none",
    //target_abi = "eabihf",
)))]
core::compile_error!("This crate requires `--target thumbv7em-none-eabihf`");

pub mod furi;
mod inlines;

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
mod bindings;

/// Create a static C string.
/// Will automatically add a NUL terminator.
#[macro_export]
macro_rules! c_string {
    ($str:expr $(,)?) => {{
        concat!($str, "\0").as_ptr() as *const core::ffi::c_char
    }};
}

/// Crash the system.
#[macro_export]
macro_rules! crash {
    ($msg:expr $(,)?) => {
        unsafe {
            // Crash message is passed via r12
            let msg = $crate::c_string!($msg);
            core::arch::asm!("", in("r12") msg, options(nomem, nostack));

            $crate::__furi_crash();
            core::hint::unreachable_unchecked();
        }
    };
}

// Re-export bindings
pub use bindings::*;

// Definition of inline functions
pub use inlines::furi_hal_gpio::*;
