#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![feature(asm)]

// mod api;
mod function_hooks;
mod fighter;

#[skyline::main(name = "the_wubor_patch")]
pub fn main() {
    // api::install();
    wubor_utils::vars::install();
    function_hooks::install();
    fighter::install();
}