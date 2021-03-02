#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![feature(asm)]

pub static mut FIGHTER_CUTIN_MANAGER_ADDR: usize = 0;

macro_rules! c_str {
    ($l:tt) => {
        [$l.as_bytes(), "\u{0}".as_bytes()].concat().as_ptr();
    };
}

mod custom;
mod daisy;
// mod samusd;
mod lucina;
mod littlemac;
mod gaogaen;
mod dedede;
mod lucas;
mod jack;
mod kirby;
mod cloud;
mod lucario;
// mod bayonetta;
// mod dolly;
mod shulk;
mod pikachu;
mod robot;
mod snake;
mod palutena;
mod master;
mod ryu;
mod toonlink;
mod zelda;
mod buddy;
mod ridley;
// mod edge;
mod koopajr;

#[skyline::main(name = "the_bor_patch")]
pub fn main() {
    unsafe{
        skyline::nn::ro::LookupSymbol(&mut FIGHTER_CUTIN_MANAGER_ADDR, c_str!("_ZN3lib9SingletonIN3app19FighterCutInManagerEE9instance_E"));
    }
    custom::install();
    daisy::install();
    // samusd::install();
    lucina::install();
    littlemac::install();
    gaogaen::install();
    dedede::install();
    lucas::install();
    jack::install();
    kirby::install();
    cloud::install();
    lucario::install();
    // bayonetta::install();
    // dolly::install();
    shulk::install();
    pikachu::install();
    robot::install();
    snake::install();
    palutena::install();
    master::install();
    ryu::install();
    toonlink::install();
    zelda::install();
    buddy::install();
    ridley::install();
    // edge::install();
    koopajr::install();
}