#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![feature(asm)]

use skyline::libc::c_char;

mod function_hooks;
mod fighter;

extern "C" {
    fn change_version_string(arg: u64, string: *const c_char);
}

#[skyline::hook(replace = change_version_string)]
fn change_version_string_hook(arg: u64, string: *const c_char) {
    let original_str = unsafe { skyline::from_c_str(string) };
    if original_str.contains("Ver.") {
        let new_str = format!(
            "{}, WuBor Patch Ver. {}\0",
            original_str,
            env!("CARGO_PKG_VERSION")
        );

        call_original!(arg, skyline::c_str(&new_str))
    } else {
        call_original!(arg, string)
    }
}

#[skyline::main(name = "the_wubor_patch")]
pub fn main() {
    wubor_utils::vars::install();
    function_hooks::install();
    fighter::install();
    skyline::install_hooks!(change_version_string_hook);
}