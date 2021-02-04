#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]

//mod custom;
mod daisy;
mod samusd;
mod lucina;
mod littlemac;
mod gaogaen;
mod dedede;

#[skyline::main(name = "the_bor_patch")]
pub fn main() {
    //custom::install();
    daisy::install();
    samusd::install();
    lucina::install();
    littlemac::install();
    gaogaen::install();
    dedede::install();
}