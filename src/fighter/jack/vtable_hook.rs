#![allow(non_snake_case)]

use crate::imports::status_imports::*;

#[skyline::hook(offset = 0xb36ec0)]
pub unsafe extern "C" fn jack_damage_callback(_vtable: u64, _fighter: &mut Fighter, _event: u64) {
    // stub gaining rebel's gauge from getting hit?
}

#[skyline::hook(offset = 0xb36e80)]
pub unsafe extern "C" fn jack_damage_callback2(_vtable: u64, _fighter: &mut Fighter, _event: u64) {
    // stub gaining rebel's gauge from getting hit?
}

#[skyline::hook(offset = 0x21b2940)]
pub unsafe extern "C" fn jack_damage_callback3(_stack: u64) {
    // stub gaining rebel's gauge from getting hit?
}

#[skyline::hook(offset = 0xb34430)]
pub unsafe extern "C" fn jack_handle_gun_dodge_staling(_vtable: u64, _fighter: &mut Fighter) {
    // stub gaining rebel's gauge from getting hit?
}

#[skyline::hook(offset = 0x21b2af0)]
pub unsafe extern "C" fn jack_call_summon_dispatch(_stack: u64) {
    // stub gaining rebel's gauge from getting hit?
}

pub fn install() {
    // Disables passive meter gain
    skyline::patching::Patch::in_text(0xb31600).data(0x17FFFF6Eu32);
    // Disables automatically summoning Arsene
    skyline::patching::Patch::in_text(0xb3151c).data(0x14000035u32);
    skyline::patching::Patch::in_text(0xb30db4).data(0x14000031u32);
    skyline::install_hooks!(
        jack_damage_callback,
        jack_damage_callback2,
        jack_damage_callback3,
        jack_handle_gun_dodge_staling,
        jack_call_summon_dispatch
    );
}