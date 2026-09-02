use crate::imports::*;

// Only extra elec hitlag for hit character
#[skyline::hook(offset = 0x406824, inline)]
unsafe fn change_elec_hitlag_for_attacker(ctx: &mut skyline::hooks::InlineCtx) {
    let is_attacker = ctx.registers[4].w() & 1 == 0;
    if ctx.registers[8].x() == hash40("collision_attr_elec") && is_attacker {
        ctx.registers[8].set_x(hash40("collision_attr_normal"));
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
    let parry_hitlag = ctx.registers[28].w();
    ctx.registers[26].set_w(parry_hitlag);
}

#[skyline::hook(offset = 0x617aa4, inline)]
unsafe extern "C" fn reverse_trump_logic(ctx: &mut skyline::hooks::InlineCtx) {
    let object = ctx.registers[23].x() as *mut BattleObject;
    WorkModule::on_flag((*object).module_accessor, *FIGHTER_STATUS_CLIFF_FLAG_TO_ROB);
}

#[skyline::hook(offset = 0x33bdd88 + 0x5b0, inline)]
unsafe extern "C" fn force_reflect_full_lifetime(ctx: &mut skyline::hooks::InlineCtx) {
    ctx.registers[8].set_x(0);
}

#[skyline::hook(offset = 0x6416e8, inline)]
unsafe extern "C" fn shield_break_lr_set(ctx: &mut skyline::hooks::InlineCtx) {
    let fighter = ctx.registers[19].x() as *mut Fighter;
    let module_accessor = (*fighter).battle_object.module_accessor;
    let lr = *(fighter as *const f32).add(0xf738 / 0x4);
    WorkModule::set_float(module_accessor, lr, *FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLOAT_SHIELD_LR);
}

#[skyline::hook(offset = 0x6418f8, inline)]
unsafe extern "C" fn shield_set_facing_lr(ctx: &mut skyline::hooks::InlineCtx) {
    let fighter = ctx.registers[19].x() as *mut Fighter;
    let module_accessor = (*fighter).battle_object.module_accessor;
    let opponent_object_id = ctx.registers[23].w();
    let dir = if sv_battle_object::category(opponent_object_id) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        let opponent_module_accessor = sv_battle_object::module_accessor(opponent_object_id);
        let pos_x = PostureModule::pos_x(module_accessor);
        let opponent_pos_x = PostureModule::pos_x(opponent_module_accessor);
        if pos_x > opponent_pos_x {
            -1.0
        }
        else if pos_x < opponent_pos_x {
            1.0
        }
        else {
            0.0
        }
    }
    else {
        0.0
    };
    VarModule::set_float(module_accessor, guard::float::GUARD_DAMAGE_FACING_DIR, dir);
}

#[skyline::hook(offset = 0x614c0c, inline)]
unsafe extern "C" fn shield_health_recovery_check_max(ctx: &mut skyline::hooks::InlineCtx) {
    let fighter = ctx.registers[19].x() as *mut Fighter;
    shield_recovery_burnout_check(fighter);
}

#[skyline::hook(offset = 0x614b9c, inline)]
unsafe extern "C" fn shield_health_recovery_check_less_than_max(ctx: &mut skyline::hooks::InlineCtx) {
    let fighter = ctx.registers[19].x() as *mut Fighter;
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

#[skyline::hook(offset = 0x403ca4, inline)]
unsafe extern "C" fn damage_level_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let module_accessor = ctx.registers[19].x() as *mut BattleObjectModuleAccessor;
    // if VarModule::is_flag(module_accessor, thrown::flag::FORCE_LAUNCHED) {
    if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_THROWN {
        ctx.registers[0].set_w(3);
    }
}

#[skyline::hook(offset = 0x6d249c, inline)]
unsafe fn hitstun_gravity_1(ctx: &mut skyline::hooks::InlineCtx) {
    // println!("grav1");
    hitstun_gravity_inner(ctx);
}

#[skyline::hook(offset = 0x6c39a0, inline)]
unsafe fn hitstun_gravity_2(ctx: &mut skyline::hooks::InlineCtx) {
    // println!("grav2");
    hitstun_gravity_inner(ctx);
}

#[skyline::hook(offset = 0x6d5924, inline)]
unsafe fn hitstun_gravity_3(ctx: &mut skyline::hooks::InlineCtx) {
    // println!("grav3");
    hitstun_gravity_inner(ctx);
}

unsafe extern "C" fn hitstun_gravity_inner(ctx: &mut skyline::hooks::InlineCtx) {
    let gravity = ctx.registers_f[0].s();
    // println!("Gravity: {}", gravity);
    let clamped = gravity.clamp(0.125, 0.15);
    // if clamped != gravity {
    //     println!("it's clampin' time: {}", clamped);
    // }
    ctx.registers_f[0].set_s(clamped);
}

#[skyline::hook(offset = 0x6d24c4, inline)]
unsafe fn hitstun_fall_speed_1(ctx: &mut skyline::hooks::InlineCtx) {
    // println!("fall1");
    hitstun_fall_speed_inner(ctx);
}

#[skyline::hook(offset = 0x6c39c8, inline)]
unsafe fn hitstun_fall_speed_2(ctx: &mut skyline::hooks::InlineCtx) {
    // println!("fall2");
    hitstun_fall_speed_inner(ctx);
}

#[skyline::hook(offset = 0x6d594c, inline)]
unsafe fn hitstun_fall_speed_3(ctx: &mut skyline::hooks::InlineCtx) {
    // println!("fall3");
    hitstun_fall_speed_inner(ctx);
}

unsafe extern "C" fn hitstun_fall_speed_inner(ctx: &mut skyline::hooks::InlineCtx) {
    let fall_speed = ctx.registers_f[0].s();
    // println!("Fall Speed: {}", fall_speed);
    let clamped = fall_speed.clamp(1.5, 1.8);
    // if clamped != fall_speed {
    //     println!("it's clampin' time: {}", clamped);
    // }
    ctx.registers_f[0].set_s(clamped);
}

#[skyline::hook(offset = 0x6d1654, inline)]
unsafe fn force_normal_hitstun_fallspeeds1(ctx: &mut skyline::hooks::InlineCtx) {
    ctx.registers[24].set_w(1);
}

#[skyline::hook(offset = 0x6c35f8, inline)]
unsafe fn force_normal_hitstun_fallspeeds2(ctx: &mut skyline::hooks::InlineCtx) {
    ctx.registers[8].set_w(1);
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

    // Use fall speed for vertical launchers
    let accel = [
        0x6c3988,
        0x6d2480,
        0x6d590c
    ];
    for idx in 0..accel.len() {
        let _ = skyline::patching::Patch::in_text(accel[idx]).data(0xD28AFB21u32);
        let _ = skyline::patching::Patch::in_text(accel[idx] + 0x4).data(0xF2B43001u32);
        let offset = if idx == 1 {
            0xC
        }
        else {
            0x8
        };
        let _ = skyline::patching::Patch::in_text(accel[idx] + offset).data(0xF2C00161u32);
    }
    let speed = [
        0x6c39b0,
        0x6d24ac,
        0x6d5934
    ];
    for idx in 0..speed.len() {
        let _ = skyline::patching::Patch::in_text(speed[idx]).data(0xD284DFC1u32);
        let _ = skyline::patching::Patch::in_text(speed[idx] + 0x4).data(0xF2BD8AC1u32);
        let _ = skyline::patching::Patch::in_text(speed[idx] + 0x8).data(0xF2C00241u32);
    }

    // Patches shield health recovery
    let _ = skyline::patching::Patch::in_text(0x614b9c).nop();
    let _ = skyline::patching::Patch::in_text(0x614ba0).data(0x1400001Au32);

    // Changes the >= check to > when reducing shield health to exactly 0.0
    let _ = skyline::patching::Patch::in_text(0x64160c).data(0x54000A4C_u32);

    // Disables getting airdodge back on hit
    let _ = skyline::patching::Patch::in_text(0x632530).nop();

    // Allow 0 CPUs in Training Mode menu
    // Allow UI to decrement to 0
    let _ = skyline::patching::Patch::in_text(0x1bb46a4).data(0xb907fa7f_u32);
    // Change set-value handler clamp to 0
    let _ = skyline::patching::Patch::in_text(0x1bbad14).data(0x7100011f_u32);
    // Fix clamp logic to clamp underflow to 0 instead of 1
    let _ = skyline::patching::Patch::in_text(0x1bbad18).data(0x1a9fa114_u32);

    skyline::install_hooks!(
        change_elec_hitlag_for_attacker,
        // autoturn_handler,
        set_parry_hitlag,
        reverse_trump_logic,
        force_reflect_full_lifetime,
        shield_break_lr_set,
        shield_set_facing_lr,
        shield_health_recovery_check_max,
        shield_health_recovery_check_less_than_max,
        fighter_global_per_frame,
        damage_level_hook,
        hitstun_gravity_1,
        hitstun_gravity_2,
        hitstun_gravity_3,
        hitstun_fall_speed_1,
        hitstun_fall_speed_2,
        hitstun_fall_speed_3,
        force_normal_hitstun_fallspeeds1,
        force_normal_hitstun_fallspeeds2
    );
}
