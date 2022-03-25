use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        phx::{Hash40, Vector3f},
        app::{lua_bind::*, *},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*,
    super::helper::*,
    wubor_utils::{
        wua_bind::*,
        vars::*
    }
};

// Notes:
// vc_ken_special_l01 is "I hit my boiling point!"
// vc_ken_special_l02 is "Shoryureppa"

unsafe fn ken_reset_vars(fighter: &mut L2CFighterCommon) {
    if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH {
        WorkModule::set_int(fighter.module_accessor, FIGHTER_KEN_QUICK_STEP_STATE_ENABLE, FIGHTER_KEN_INSTANCE_WORK_ID_INT_QUICK_STEP_STATE);
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_KEN_INSTANCE_WORK_ID_FLAG_V_TRIGGER);
        WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_KEN_INSTANCE_WORK_ID_INT_SHORYUREPPA);
        WorkModule::set_int64(fighter.module_accessor, 0, FIGHTER_INSTANCE_WORK_ID_INT_TARGET_ID);
    }
}

unsafe fn ken_meter_controller(fighter: &mut L2CFighterCommon) {
    if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD)
    && MotionModule::motion_kind(fighter.module_accessor) != hash40("special_lw")
    && !WorkModule::is_flag(fighter.module_accessor, FIGHTER_KEN_INSTANCE_WORK_ID_FLAG_V_TRIGGER) {
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_s3_s_w")
        && WorkModule::get_int(fighter.module_accessor, FIGHTER_KEN_INSTANCE_WORK_ID_INT_QUICK_STEP_STATE) == FIGHTER_KEN_QUICK_STEP_STATE_RUN {
            add_vgauge(fighter.module_accessor, 50.0);
        }
        else {
            let amount = AttackModule::get_power(fighter.module_accessor, 0, false, 1.0, false) * 2.0;
            add_vgauge(fighter.module_accessor, amount);
        }
    }

    let damage = DamageModule::damage(fighter.module_accessor, 0);
    let damage_prev = WorkModule::get_float(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_PREV);
    if damage > damage_prev
    && MotionModule::motion_kind(fighter.module_accessor) != hash40("special_lw_step_b") {
        let amount = (damage - damage_prev) * 2.0;
        add_vgauge(fighter.module_accessor, amount);
    }
    WorkModule::set_float(fighter.module_accessor, damage, FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_PREV);
}

unsafe fn ken_vgauge_flash(fighter: &mut L2CFighterCommon) {
    let v_gauge = WorkModule::get_float(fighter.module_accessor, FIGHTER_KEN_INSTANCE_WORK_ID_FLOAT_V_GAUGE);
    let mut flash_max = WorkModule::get_int(fighter.module_accessor, FIGHTER_KEN_INSTANCE_WORK_ID_INT_FLASH_MAX);
    if v_gauge < 300.0
    && flash_max != 0 {
        macros::COL_NORMAL(fighter);
        WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_KEN_INSTANCE_WORK_ID_INT_FLASH_MAX);
        WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_KEN_INSTANCE_WORK_ID_INT_FLASH_COUNTER);
    }
    else if v_gauge >= 300.0
    && v_gauge < 600.0
    && flash_max != 60 {
        macros::COL_NORMAL(fighter);
        WorkModule::set_int(fighter.module_accessor, 60, FIGHTER_KEN_INSTANCE_WORK_ID_INT_FLASH_MAX);
    }
    else if v_gauge >= 600.0
    && v_gauge < 900.0
    && flash_max != 40 {
        macros::COL_NORMAL(fighter);
        WorkModule::set_int(fighter.module_accessor, 40, FIGHTER_KEN_INSTANCE_WORK_ID_INT_FLASH_MAX);
    }
    else if v_gauge == 900.0
    && flash_max != 20 {
        macros::COL_NORMAL(fighter);
        WorkModule::set_int(fighter.module_accessor, 20, FIGHTER_KEN_INSTANCE_WORK_ID_INT_FLASH_MAX);
    }

    flash_max = WorkModule::get_int(fighter.module_accessor, FIGHTER_KEN_INSTANCE_WORK_ID_INT_FLASH_MAX);
    if flash_max != 0 {
        let mut flash_counter = WorkModule::get_int(fighter.module_accessor, FIGHTER_KEN_INSTANCE_WORK_ID_INT_FLASH_COUNTER);
        if flash_counter == 0 {
            WorkModule::set_int(fighter.module_accessor, flash_max, FIGHTER_KEN_INSTANCE_WORK_ID_INT_FLASH_COUNTER);
            flash_counter = flash_max;
        }
        if flash_counter == flash_max {
            macros::FLASH(fighter, 1, 0.2, 0, 0.75);
        }
        if flash_counter == flash_max - 5 {
            macros::COL_NORMAL(fighter);
        }
        WorkModule::dec_int(fighter.module_accessor, FIGHTER_KEN_INSTANCE_WORK_ID_INT_FLASH_COUNTER);
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
            WorkModule::on_flag(fighter.module_accessor, FIGHTER_KEN_STATUS_ATTACK_FLAG_VS1_CANCEL);
        }
    }

    if StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_SPECIAL_LW
    && WorkModule::get_int(fighter.module_accessor, FIGHTER_KEN_INSTANCE_WORK_ID_INT_QUICK_STEP_STATE) == FIGHTER_KEN_QUICK_STEP_STATE_RUN {
        WorkModule::set_int(fighter.module_accessor, FIGHTER_KEN_QUICK_STEP_STATE_DISABLE, FIGHTER_KEN_INSTANCE_WORK_ID_INT_QUICK_STEP_STATE);
    }

    if WorkModule::get_int(fighter.module_accessor, FIGHTER_KEN_INSTANCE_WORK_ID_INT_QUICK_STEP_STATE) == FIGHTER_KEN_QUICK_STEP_STATE_DISABLE
    && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        WorkModule::set_int(fighter.module_accessor, FIGHTER_KEN_QUICK_STEP_STATE_ENABLE, FIGHTER_KEN_INSTANCE_WORK_ID_INT_QUICK_STEP_STATE);
    }
}

unsafe fn ken_vtrigger(fighter: &mut L2CFighterCommon) {
    if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD)
    || (StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_N | *FIGHTER_RYU_STATUS_KIND_SPECIAL_N_COMMAND
    && MotionModule::frame(fighter.module_accessor) >= 13.0) {
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_KEN_STATUS_ATTACK_FLAG_VT1_CANCEL);
    }
    
    if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW != 0 {
        if ((WorkModule::is_flag(fighter.module_accessor, FIGHTER_KEN_STATUS_ATTACK_FLAG_VT1_CANCEL)
        && WorkModule::get_float(fighter.module_accessor, FIGHTER_KEN_INSTANCE_WORK_ID_FLOAT_V_GAUGE) == 900.0)
        || (WorkModule::is_flag(fighter.module_accessor, FIGHTER_KEN_STATUS_ATTACK_FLAG_VS1_CANCEL)
        && WorkModule::get_int(fighter.module_accessor, FIGHTER_KEN_INSTANCE_WORK_ID_INT_QUICK_STEP_STATE) == FIGHTER_KEN_QUICK_STEP_STATE_ENABLE))
        && !MiscModule::is_damage_check(fighter.module_accessor, false) {
            fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), false.into());
        }
    }

    if StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_SPECIAL_HI
    && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_COMMAND
    && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_JUMP
    && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_LANDING
    && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_FALL {
        WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_KEN_INSTANCE_WORK_ID_INT_SHORYUREPPA);
    }

    if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_LANDING
    && WorkModule::get_int(fighter.module_accessor, FIGHTER_KEN_INSTANCE_WORK_ID_INT_SHORYUREPPA) == 1 {
        fighter.change_status(FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_COMMAND.into(), false.into());
    }

    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_KEN_INSTANCE_WORK_ID_FLAG_V_TRIGGER) {
        let v_trigger_eff_timer = WorkModule::get_int(fighter.module_accessor, FIGHTER_KEN_INSTANCE_WORK_ID_INT_V_TRIGGER_EFF_TIMER);
        if v_trigger_eff_timer == 32 {
            EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_flame"), smash::phx::Hash40::new("footl"), &ZERO_VECTOR, &Vector3f{x: 90.0, y: 0.0, z: 0.0}, 0.2, true, 0, 0, 0, 0, 0, true, true);
        }
        if v_trigger_eff_timer == 16 {
            EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_flame"), smash::phx::Hash40::new("footr"), &ZERO_VECTOR, &Vector3f{x: 90.0, y: 0.0, z: 0.0}, 0.2, true, 0, 0, 0, 0, 0, true, true);
        }
        WorkModule::dec_int(fighter.module_accessor, FIGHTER_KEN_INSTANCE_WORK_ID_INT_V_TRIGGER_EFF_TIMER);
        if WorkModule::get_int(fighter.module_accessor, FIGHTER_KEN_INSTANCE_WORK_ID_INT_V_TRIGGER_EFF_TIMER) <= 0 {
            WorkModule::set_int(fighter.module_accessor, 32, FIGHTER_KEN_INSTANCE_WORK_ID_INT_V_TRIGGER_EFF_TIMER);
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
            add_vgauge(fighter.module_accessor, -300.0);
        }
        if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
            add_vgauge(fighter.module_accessor, 300.0);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_KEN )]
fn ken_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        ken_reset_vars(fighter);
        ken_meter_controller(fighter);
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