use crate::imports::*;

// Only extra elec hitlag for hit character
#[skyline::hook(offset = 0x406824, inline)]
unsafe fn change_elec_hitlag_for_attacker(ctx: &mut skyline::hooks::InlineCtx) {
    let is_attacker = *ctx.registers[4].w.as_ref() & 1 == 0;
    if *ctx.registers[8].x.as_ref() == hash40("collision_attr_elec") && is_attacker {
        *ctx.registers[8].x.as_mut() = hash40("collision_attr_normal");
    }
}

// Autoturn for Ryu, Ken, Terry, and Kazuya
// #[skyline::hook(offset = 0x69a6e0)]
// unsafe fn autoturn_handler(
//     module_accessor: *mut BattleObjectModuleAccessor,
//     is_landing_special: bool,
//     status: i32,
//     some_uint: u32
// ) -> f32 {
//     let kind = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND);
//     if [*FIGHTER_KIND_DOLLY, *FIGHTER_KIND_DEMON].contains(&kind)
//     && !(0x0..0xA).contains(&status)
//     && !(0x1E4..0x1EB).contains(&status) {
//         return 0.0;
//     }
//     original!()(module_accessor, is_landing_special, status, some_uint)
// }

// Forces parry hitlag to be a constant value
#[skyline::hook(offset = 0x641d84, inline)]
unsafe fn set_parry_hitlag(ctx: &mut skyline::hooks::InlineCtx) {
    let parry_hitlag = *ctx.registers[28].w.as_ref();
    *ctx.registers[26].x.as_mut() = parry_hitlag as u64;
}

#[skyline::hook(offset = 0x617aa4, inline)]
unsafe extern "C" fn reverse_trump_logic(ctx: &mut skyline::hooks::InlineCtx) {
    let object = *ctx.registers[23].x.as_ref() as *mut BattleObject;
    WorkModule::on_flag((*object).module_accessor, *FIGHTER_STATUS_CLIFF_FLAG_TO_ROB);
}

pub fn install() {
    // Stubs parry hitlag calculation
    let _ = skyline::patching::Patch::in_text(0x641d84).nop();

    // Removes Phantom Hits
    let _ = skyline::patching::Patch::in_text(0x3e6d08).data(0x14000012u32);

    // Removes the vanilla ledge trump check
    let _ = skyline::patching::Patch::in_text(0x617a90).nop();
    let _ = skyline::patching::Patch::in_text(0x617aa4).nop();

    // Removes the forced change to HIT_STATUS_OFF during Final Smash
    let _ = skyline::patching::Patch::in_text(0x62d5ac).nop();

    // The following disables the reversed stick values when autoturn runs
    let _ = skyline::patching::Patch::in_text(0x69ae20).nop();
    let _ = skyline::patching::Patch::in_text(0x934a6c).nop();
    let _ = skyline::patching::Patch::in_text(0x974d20).nop();
    let _ = skyline::patching::Patch::in_text(0x21d7cfc).nop();

    // Removes the 3f delay on backdashing for Ryu/Ken/Terry/Kazuya
    let _ = skyline::patching::Patch::in_text(0x69aef8).data(0x14000008u32);

    // Removes the ledge grab limit
    let _ = skyline::patching::Patch::in_text(0x618cc8).data(0x14000054u32);
    let _ = skyline::patching::Patch::in_text(0x62f0b4).nop();
    let _ = skyline::patching::Patch::in_text(0x62f0b8).nop();

    skyline::install_hooks!(
        change_elec_hitlag_for_attacker,
        // autoturn_handler,
        set_parry_hitlag,
        reverse_trump_logic
    );
}