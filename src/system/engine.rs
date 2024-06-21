use crate::imports::*;

// Only extra elec hitlag for hit character
#[skyline::hook(offset = 0x406824, inline)]
unsafe fn change_elec_hitlag_for_attacker(ctx: &mut skyline::hooks::InlineCtx) {
    let is_attacker = *ctx.registers[4].w.as_ref() & 1 == 0;
    if *ctx.registers[8].x.as_ref() == hash40("collision_attr_elec") && is_attacker {
        *ctx.registers[8].x.as_mut() = hash40("collision_attr_normal");
    }
}

// Turns off Autoturn for Ryu, Ken, Terry, and Kazuya
#[skyline::hook(offset = 0x69a6e0)]
unsafe fn autoturn_handler(_module_accessor: *mut BattleObjectModuleAccessor, _some_bool: bool, _some_int: i32, _some_uint: u32) -> f32 {
    0.0
}

// Forces parry hitlag to be a constant value
#[skyline::hook(offset = 0x641d84, inline)]
unsafe fn set_parry_hitlag(ctx: &mut skyline::hooks::InlineCtx) {
    let parry_hitlag = *ctx.registers[28].w.as_ref();
    *ctx.registers[26].x.as_mut() = parry_hitlag as u64;
}

pub fn install() {
    // Stubs parry hitlag calculation
    skyline::patching::Patch::in_text(0x641d84).nop();

    // Removes Phantom Hits
    skyline::patching::Patch::in_text(0x3e6d08).data(0x14000012u32);

    skyline::install_hooks!(
        change_elec_hitlag_for_attacker,
        autoturn_handler,
        set_parry_hitlag
    );
}