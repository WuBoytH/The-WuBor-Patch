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

#[skyline::hook(offset = 0x33bdff8, inline)]
unsafe extern "C" fn force_reflect_full_lifetime(ctx: &mut skyline::hooks::InlineCtx) {
    *ctx.registers[8].x.as_mut() = 0;
}

#[skyline::hook(offset = 0x6416e8, inline)]
unsafe extern "C" fn shield_break_lr_set(ctx: &mut skyline::hooks::InlineCtx) {
    let fighter = *ctx.registers[19].x.as_mut() as *mut Fighter;
    let module_accessor = (*fighter).battle_object.module_accessor;
    let lr = *(fighter as *const f32).add(0xf738 / 0x4);
    WorkModule::set_float(module_accessor, lr, *FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLOAT_SHIELD_LR);
}

#[skyline::hook(offset = 0x614c0c, inline)]
unsafe extern "C" fn shield_health_recovery_check_max(ctx: &mut skyline::hooks::InlineCtx) {
    let fighter = *ctx.registers[19].x.as_mut() as *mut Fighter;
    shield_recovery_burnout_check(fighter);
}

#[skyline::hook(offset = 0x614b9c, inline)]
unsafe extern "C" fn shield_health_recovery_check_less_than_max(ctx: &mut skyline::hooks::InlineCtx) {
    let fighter = *ctx.registers[19].x.as_mut() as *mut Fighter;
    shield_recovery_burnout_check(fighter);
}

unsafe extern "C" fn shield_recovery_burnout_check(fighter: *mut Fighter) {
    let module_accessor = (*fighter).battle_object.module_accessor;
    if VarModule::is_flag(module_accessor, fighter::instance::flag::BURNOUT) {
        let shield_recovery1 = WorkModule::get_param_float(module_accessor, hash40("common"), hash40("shield_recovery1"));
        let shield_recovery = WorkModule::get_param_float(module_accessor, hash40("shield_recovery"), 0);
        let mut shield_health = WorkModule::get_float(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);
        let shield_health_max = WorkModule::get_float(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD_MAX);
        shield_health = (shield_health + (shield_recovery1 * shield_recovery)).min(shield_health_max);
        if shield_health >= shield_health_max {
            EffectModule::remove_common(module_accessor, Hash40::new("burnout"));
            SoundModule::play_se(
                module_accessor,
                Hash40::new("se_common_burnout_recover"),
                true,
                false,
                false,
                false,
                enSEType(0)
            );
            ColorBlendModule::cancel_main_color(module_accessor, 0);
            VarModule::set_int(module_accessor, fighter::instance::int::BURNOUT_EFF_FRAME, 0);
            VarModule::off_flag(module_accessor, fighter::instance::flag::BURNOUT);
        }
        WorkModule::set_float(module_accessor, shield_health, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);
    }
}

#[skyline::hook(offset = 0x614630)]
unsafe extern "C" fn fighter_global_per_frame(fighter: &mut Fighter) {
    original!()(fighter);
    let battle_object_slow = singletons::BattleObjectSlow() as *mut u8;
    if *battle_object_slow.add(0x8) == 0 || *(battle_object_slow as *const u32) == 0 {
        VarModule::countdown_int(fighter.battle_object.module_accessor, fighter::instance::int::GUARD_CANCEL_PASS_FRAME, 0);
    }
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
    let _ = skyline::patching::Patch::in_text(0x21d7d1c).nop();

    // Disables Reverse Special Command calls
    let _ = skyline::patching::Patch::in_text(0x69ad9c).nop();
    let _ = skyline::patching::Patch::in_text(0x974d00).nop();
    let _ = skyline::patching::Patch::in_text(0x934a4c).nop();
    let _ = skyline::patching::Patch::in_text(0x21d7cfc).nop();

    // Removes the 3f delay on backdashing for Ryu/Ken/Terry/Kazuya
    let _ = skyline::patching::Patch::in_text(0x69aef8).data(0x14000008u32);

    // Removes the ledge grab limit
    let _ = skyline::patching::Patch::in_text(0x618cc8).data(0x14000054u32);
    let _ = skyline::patching::Patch::in_text(0x62f0b4).nop();
    let _ = skyline::patching::Patch::in_text(0x62f0b8).nop();

    // Patches shield health recovery
    let _ = skyline::patching::Patch::in_text(0x614b9c).nop();
    let _ = skyline::patching::Patch::in_text(0x614ba0).data(0x1400001Au32);

    skyline::install_hooks!(
        change_elec_hitlag_for_attacker,
        // autoturn_handler,
        set_parry_hitlag,
        reverse_trump_logic,
        force_reflect_full_lifetime,
        shield_break_lr_set,
        shield_health_recovery_check_max,
        shield_health_recovery_check_less_than_max,
        fighter_global_per_frame
    );
}