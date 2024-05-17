#![feature(
    concat_idents,
    proc_macro_hygiene,
    simd_ffi,
    repr_simd
)]
#![allow(
    unused_macros,
    unused_must_use
)]

extern crate skyline;
extern crate smash;
extern crate smash_rs;
extern crate smash_script;
extern crate wubor_utils;
extern crate custom_var;

#[cfg(feature = "main_nro")]
use skyline::libc::c_char;
#[cfg(feature = "main_nro")]
mod imports;
#[cfg(feature = "main_nro")]
pub mod system;
#[cfg(feature = "main_nro")]
mod custom_vars;
#[cfg(feature = "main_nro")]
mod vtable_hook;
#[cfg(feature = "main_nro")]
mod items;

mod fighters;

#[cfg(feature = "main_nro")]
extern "C" {
    fn change_version_string(arg: u64, string: *const c_char);
}

#[no_mangle]
pub extern "C" fn is_wubor_patch() {}

#[cfg(not(feature = "main_nro"))]
#[no_mangle]
pub fn smashline_install() {
    fighters::install();
}

#[cfg(feature = "main_nro")]
#[skyline::hook(replace = change_version_string)]
fn change_version_string_hook(arg: u64, string: *const c_char) {
    let original_str = unsafe { skyline::from_c_str(string) };
    if original_str.contains("Ver.") {
        let version = if cfg!(feature = "pr") {
            format!("{}-PR", env!("CARGO_PKG_VERSION"))
        }
        else if cfg!(feature = "dev") {
            format!("{}-Dev", env!("CARGO_PKG_VERSION"))
        }
        else if cfg!(feature = "local") {
            format!("{}-Local", env!("CARGO_PKG_VERSION"))
        }
        else {
            env!("CARGO_PKG_VERSION").to_string()
        };
        let new_str = format!(
            "{}, WuBor Patch Ver. {}\0",
            original_str,
            version
        );

        call_original!(arg, skyline::c_str(&new_str))
    } else {
        call_original!(arg, string)
    }
}

pub fn is_on_ryujinx() -> bool {
    unsafe {
        // Ryujinx skip based on text addr
        let text_addr = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;
        text_addr == 0x8504000 || text_addr == 0x80004000
    }
}

#[skyline::main(name = "wubor")]
pub fn main() {
    #[cfg(feature = "main_nro")]
    {
        system::install();
        custom_vars::install();
        vtable_hook::install();
        items::install();
        skyline::install_hooks!(change_version_string_hook);
    }

    fighters::install();
}