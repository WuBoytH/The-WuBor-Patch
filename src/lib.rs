#![feature(
    concat_idents,
    proc_macro_hygiene,
    simd_ffi,
    repr_simd
)]
#![allow(
    unused_macros,
    unused_must_use,
    clippy::borrow_interior_mutable_const,
    clippy::collapsible_if,
    clippy::collapsible_else_if,
    clippy::absurd_extreme_comparisons,
    clippy::cmp_null,
    clippy::if_same_then_else
)]

use skyline::libc::c_char;
mod function_hooks;
mod fighter;
mod custom_vars;

extern "C" {
    fn change_version_string(arg: u64, string: *const c_char);
}

#[skyline::hook(replace = change_version_string)]
fn change_version_string_hook(arg: u64, string: *const c_char) {
    let original_str = unsafe { skyline::from_c_str(string) };
    if original_str.contains("Ver.") {
        let version = if cfg!(feature = "pr") {
            format!("{}-pr", env!("CARGO_PKG_VERSION"))
        }
        else if cfg!(feature = "dev") {
            format!("{}-dev", env!("CARGO_PKG_VERSION"))
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

#[skyline::main(name = "the_wubor_patch")]
pub fn main() {
    function_hooks::install();
    fighter::install();
    custom_vars::install();
    skyline::install_hooks!(change_version_string_hook);
}