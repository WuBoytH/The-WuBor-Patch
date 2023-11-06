use {
    smash::{
        lua2cpp::*,
        // hash40,
        phx::*,
        app::{lua_bind::*, *},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*,
    custom_var::*,
    wubor_utils::{wua_bind::*, vars::*}
};

// Notes:
// vc_ken_special_l01 is "I hit my boiling point!"
// vc_ken_special_l02 is "Shoryureppa"

unsafe fn ken_reset_vars(fighter: &mut L2CFighterCommon) {
    if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH {
        VarModule::set_int(fighter.module_accessor, ken::instance::int::QUICK_STEP_STATE, ken::QUICK_STEP_STATE_ENABLE);
        VarModule::off_flag(fighter.module_accessor, ken::instance::flag::V_TRIGGER);
        VarModule::set_int(fighter.module_accessor, fighter::instance::int::TARGET_ID, 0);
    }
}

unsafe fn ken_vgauge_flash(fighter: &mut L2CFighterCommon) {
    let v_gauge = VarModule::get_float(fighter.module_accessor, ken::instance::float::V_GAUGE);
    let mut flash_max = VarModule::get_int(fighter.module_accessor, ken::instance::int::FLASH_MAX);
    if v_gauge < 300.0
    && flash_max != 0 {
        macros::COL_NORMAL(fighter);
        VarModule::set_int(fighter.module_accessor, ken::instance::int::FLASH_MAX, 0);
        VarModule::set_int(fighter.module_accessor, ken::instance::int::FLASH_COUNTER, 0);
    }
    else if (300.0..600.0).contains(&v_gauge)
    && flash_max != 60 {
        macros::COL_NORMAL(fighter);
        VarModule::set_int(fighter.module_accessor, ken::instance::int::FLASH_MAX, 60);
    }
    else if (600.0..900.0).contains(&v_gauge)
    && flash_max != 40 {
        macros::COL_NORMAL(fighter);
        VarModule::set_int(fighter.module_accessor, ken::instance::int::FLASH_MAX, 40);
    }
    else if v_gauge == 900.0
    && flash_max != 20 {
        macros::COL_NORMAL(fighter);
        VarModule::set_int(fighter.module_accessor, ken::instance::int::FLASH_MAX, 20);
    }

    flash_max = VarModule::get_int(fighter.module_accessor, ken::instance::int::FLASH_MAX);
    if flash_max != 0 {
        let mut flash_counter = VarModule::get_int(fighter.module_accessor, ken::instance::int::FLASH_COUNTER);
        if flash_counter == 0 {
            VarModule::set_int(fighter.module_accessor, ken::instance::int::FLASH_COUNTER, flash_max);
            flash_counter = flash_max;
        }
        if flash_counter == flash_max {
            macros::FLASH(fighter, 1, 0.2, 0, 0.75);
        }
        if flash_counter == flash_max - 5 {
            macros::COL_NORMAL(fighter);
        }
        VarModule::dec_int(fighter.module_accessor, ken::instance::int::FLASH_COUNTER);
    }
}

unsafe fn ken_vskill(fighter: &mut L2CFighterCommon) {
    if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) {
        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_ATTACK
        || StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_RYU_STATUS_KIND_ATTACK_COMMAND1
        || StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_RYU_STATUS_KIND_ATTACK_COMMAND2
        || StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_RYU_STATUS_KIND_ATTACK_NEAR
        || StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_S3
        || StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_HI3
        || StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_LW3
        || StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_S4
        || StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_LW4
        || StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_HI4
        || StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_DASH
        || StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_AIR {
            VarModule::on_flag(fighter.module_accessor, ken::status::flag::VS1_CANCEL);
        }
    }

    if StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_SPECIAL_LW
    && VarModule::get_int(fighter.module_accessor, ken::instance::int::QUICK_STEP_STATE) == ken::QUICK_STEP_STATE_RUN {
        VarModule::set_int(fighter.module_accessor, ken::instance::int::QUICK_STEP_STATE, ken::QUICK_STEP_STATE_DISABLE);
    }

    if VarModule::get_int(fighter.module_accessor, ken::instance::int::QUICK_STEP_STATE) == ken::QUICK_STEP_STATE_DISABLE
    && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        VarModule::set_int(fighter.module_accessor, ken::instance::int::QUICK_STEP_STATE, ken::QUICK_STEP_STATE_ENABLE);
    }
}

unsafe fn ken_vtrigger(fighter: &mut L2CFighterCommon) {
    if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD)
    || (StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_N | *FIGHTER_RYU_STATUS_KIND_SPECIAL_N_COMMAND
    && MotionModule::frame(fighter.module_accessor) >= 13.0) {
        VarModule::on_flag(fighter.module_accessor, ken::status::flag::VT1_CANCEL);
    }
    
    if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW != 0 {
        if ((VarModule::is_flag(fighter.module_accessor, ken::status::flag::VT1_CANCEL)
        && VarModule::get_float(fighter.module_accessor, ken::instance::float::V_GAUGE) == 900.0)
        || (VarModule::is_flag(fighter.module_accessor, ken::status::flag::VS1_CANCEL)
        && VarModule::get_int(fighter.module_accessor, ken::instance::int::QUICK_STEP_STATE) == ken::QUICK_STEP_STATE_ENABLE))
        && !MiscModule::is_damage_check(fighter.module_accessor, false) {
            fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), false.into());
        }
    }

    if VarModule::is_flag(fighter.module_accessor, ken::instance::flag::V_TRIGGER) {
        let v_trigger_eff_timer = VarModule::get_int(fighter.module_accessor, ken::instance::int::V_TRIGGER_EFF_TIMER);
        if v_trigger_eff_timer == 32 {
            EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_flame"), smash::phx::Hash40::new("footl"), &ZERO_VECTOR, &Vector3f{x: 90.0, y: 0.0, z: 0.0}, 0.2, true, 0, 0, 0, 0, 0, true, true);
        }
        if v_trigger_eff_timer == 16 {
            EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_flame"), smash::phx::Hash40::new("footr"), &ZERO_VECTOR, &Vector3f{x: 90.0, y: 0.0, z: 0.0}, 0.2, true, 0, 0, 0, 0, 0, true, true);
        }
        VarModule::dec_int(fighter.module_accessor, ken::instance::int::V_TRIGGER_EFF_TIMER);
        if VarModule::get_int(fighter.module_accessor, ken::instance::int::V_TRIGGER_EFF_TIMER) <= 0 {
            VarModule::set_int(fighter.module_accessor, ken::instance::int::V_TRIGGER_EFF_TIMER, 32);
        }
    }
}

// unsafe fn ken_vshift(fighter: &mut L2CFighterCommon) {
//     if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_GUARD
//     && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
//     && WorkModule::get_float(fighter.module_accessor, FIGHTER_KEN_INSTANCE_WORK_ID_FLOAT_V_GAUGE) >= 300.0 {
//         let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
//         if stick_x * PostureModule::lr(fighter.module_accessor) < -0.5 {
//             add_vgauge(fighter.module_accessor, -300.0);
//             fighter.change_status(FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_B.into(), false.into());
//         }
//     }
//     if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_lw_step_b") {
//         if MotionModule::frame(fighter.module_accessor) <= 1.0
//         && !WorkModule::is_flag(fighter.module_accessor, FIGHTER_KEN_STATUS_GUARD_FLAG_V_SHIFT) {
//             macros::EFFECT_FOLLOW(fighter, Hash40::new("ken_savingattack_aura"), Hash40::new("hip"), -2, 0, 0, 0, 0, 0, 1.4, true);
//             macros::EFFECT_FOLLOW(fighter, Hash40::new("ken_savingattack_aura"), Hash40::new("neck"), 0, 0, 0, 0, 0, 0, 1, true);
//             macros::EFFECT_FOLLOW(fighter, Hash40::new("ken_savingattack_aura"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1, true);
//             macros::EFFECT_FOLLOW(fighter, Hash40::new("ken_savingattack_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1, true);
//             macros::EFFECT_FOLLOW(fighter, Hash40::new("ken_savingattack_aura"), Hash40::new("kneel"), 4, 0, 0, 0, 0, 0, 1.1, true);
//             macros::EFFECT_FOLLOW(fighter, Hash40::new("ken_savingattack_aura"), Hash40::new("kneer"), 4, 0, 0, 0, 0, 0, 1.1, true);
//         }
//         if MotionModule::frame(fighter.module_accessor) == 9.375 {
//             if WorkModule::is_flag(fighter.module_accessor, FIGHTER_KEN_STATUS_GUARD_FLAG_V_SHIFT) {
//                 add_vgauge(fighter.module_accessor, 150.0);
//                 SlowModule::set_whole(fighter.module_accessor, 5, 0);
//                 macros::SLOW_OPPONENT(fighter, 10.0, 2.0);
//                 macros::FILL_SCREEN_MODEL_COLOR(fighter, 0, 3, 0.2, 0.2, 0.2, 0, 0, 0, 1, 1, *smash::lib::lua_const::EffectScreenLayer::GROUND, 205);
//             }
//         }
//         if MotionModule::frame(fighter.module_accessor) == 12.5 {
//             SlowModule::clear_whole(fighter.module_accessor);
//             if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
//             && WorkModule::is_flag(fighter.module_accessor, FIGHTER_KEN_STATUS_GUARD_FLAG_V_SHIFT) {
//                 MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw"), 0.0, 1.0, false, 0.0, false, false);
//             }
//             else if WorkModule::is_flag(fighter.module_accessor, FIGHTER_KEN_STATUS_GUARD_FLAG_V_SHIFT) {
//                 macros::CANCEL_FILL_SCREEN(fighter, 0, 5);
//                 WorkModule::off_flag(fighter.module_accessor, FIGHTER_KEN_STATUS_GUARD_FLAG_V_SHIFT);
//             }
//         }
//     }
//     if MotionModule::motion_kind(fighter.module_accessor) != hash40("special_lw_step_b") & hash40("special_lw") {
//         WorkModule::off_flag(fighter.module_accessor, FIGHTER_KEN_STATUS_GUARD_FLAG_V_SHIFT);
//     }
// }

unsafe fn ken_training_tools(fighter: &mut L2CFighterCommon) {
    if smashball::is_training_mode()
    && [
        *FIGHTER_STATUS_KIND_GUARD_ON,
        *FIGHTER_STATUS_KIND_GUARD
    ].contains(&StatusModule::status_kind(fighter.module_accessor)) {
        if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L) {
            FGCModule::update_meter(fighter.module_accessor, -300.0, 900.0, ken::instance::float::V_GAUGE);
        }
        if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
            FGCModule::update_meter(fighter.module_accessor, 300.0, 900.0, ken::instance::float::V_GAUGE);
        }
    }
}

unsafe extern "C" fn ken_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        ken_reset_vars(fighter);
        ken_vgauge_flash(fighter);
        ken_vskill(fighter);
        ken_vtrigger(fighter);
        // ken_vshift(fighter);
        ken_training_tools(fighter);
    }
}

pub fn install() {
    install_agent_frames!(
        ken_frame
    );
}