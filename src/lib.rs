#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![feature(asm)]

mod vars;
mod function_hooks;
mod fighter;
mod gameplay;
mod cancels;
mod common_funcs;
mod table_const;

#[skyline::main(name = "the_wubor_patch")]
pub fn main() {
    vars::install();
    function_hooks::install();
    fighter::install();
}