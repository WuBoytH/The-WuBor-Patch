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
pub mod system;
mod fighter;
mod custom_vars;

mod imports;

extern "C" {
    fn change_version_string(arg: u64, string: *const c_char);
}

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

std::arch::global_asm!(
    r#"
    .section .nro_header
    .global __nro_header_start
    .word 0
    .word _mod_header
    .word 0
    .word 0
    
    .section .rodata.module_name
        .word 0
        .word 5
        .ascii "wubor"
    .section .rodata.mod0
    .global _mod_header
    _mod_header:
        .ascii "MOD0"
        .word __dynamic_start - _mod_header
        .word __bss_start - _mod_header
        .word __bss_end - _mod_header
        .word __eh_frame_hdr_start - _mod_header
        .word __eh_frame_hdr_end - _mod_header
        .word __nx_module_runtime - _mod_header // runtime-generated module object offset
    .global IS_NRO
    IS_NRO:
        .word 1
    
    .section .bss.module_runtime
    __nx_module_runtime:
    .space 0xD0
    "#
);

#[no_mangle]
pub extern "C" fn main() {
    system::install();
    fighter::install();
    custom_vars::install();
    skyline::install_hooks!(change_version_string_hook);
}