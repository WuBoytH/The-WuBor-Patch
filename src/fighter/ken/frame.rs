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
    crate::{
        common_funcs::*,
        vars::*
    }
};

// Notes:
// vc_ken_special_l01 is "I hit my boiling point!"
// vc_ken_special_l02 is "Shoryureppa"

#[fighter_frame( agent = FIGHTER_KIND_KEN )]
fn ken_frame(fighter: &mut L2CFighterCommon) {
    unsafe {

        // Reset Vars

        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH {
            QUICK_STEP_STATE[entry_id(fighter.module_accessor)] = 0;
            VS1_CANCEL[entry_id(fighter.module_accessor)] = false;
            V_SHIFT[entry_id(fighter.module_accessor)] = false;
            V_TRIGGER[entry_id(fighter.module_accessor)] = false;
            VT1_CANCEL[entry_id(fighter.module_accessor)] = false;
            SHORYUREPPA[entry_id(fighter.module_accessor)] = 0;
            OPPONENT_BOMA[entry_id(fighter.module_accessor)] = 0;
        }

        // V Gauge Building (only for blocked moves and getting hit)

        if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD)
        && MotionModule::motion_kind(fighter.module_accessor) != hash40("special_lw")
        && V_TRIGGER[entry_id(fighter.module_accessor)] == false {
            if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_s3_s_w")
            && QUICK_STEP_STATE[entry_id(fighter.module_accessor)] == 1 {
                V_GAUGE[entry_id(fighter.module_accessor)] += 50;
            }
            else {
                V_GAUGE[entry_id(fighter.module_accessor)] += AttackModule::get_power(fighter.module_accessor, 0, false, 1.0, false) as i32 * 2;
                if V_GAUGE[entry_id(fighter.module_accessor)] > 900 {
                    V_GAUGE[entry_id(fighter.module_accessor)] = 900;
                }
            }
            if V_GAUGE[entry_id(fighter.module_accessor)] > 900 {
                V_GAUGE[entry_id(fighter.module_accessor)] = 900;
            }
        }

        DAMAGE_TAKEN[entry_id(fighter.module_accessor)] = DamageModule::damage(fighter.module_accessor, 0);
        if DAMAGE_TAKEN[entry_id(fighter.module_accessor)] > DAMAGE_TAKEN_PREV[entry_id(fighter.module_accessor)]
        && MotionModule::motion_kind(fighter.module_accessor) != hash40("special_lw_step_b") {
            V_GAUGE[entry_id(fighter.module_accessor)] += (DAMAGE_TAKEN[entry_id(fighter.module_accessor)] - DAMAGE_TAKEN_PREV[entry_id(fighter.module_accessor)]) as i32 * 2;
            if V_GAUGE[entry_id(fighter.module_accessor)] > 900 {
                V_GAUGE[entry_id(fighter.module_accessor)] = 900;
            }
        }
        DAMAGE_TAKEN_PREV[entry_id(fighter.module_accessor)] = DAMAGE_TAKEN[entry_id(fighter.module_accessor)];

        // V-Gauge Effects

        if V_GAUGE[entry_id(fighter.module_accessor)] < 300
        && FLASH_MAX[entry_id(fighter.module_accessor)] != 0 {
            macros::COL_NORMAL(fighter);
            FLASH_MAX[entry_id(fighter.module_accessor)] = 0;
            FLASH_COUNTER[entry_id(fighter.module_accessor)] = 0;
        }
        else if V_GAUGE[entry_id(fighter.module_accessor)] >= 300
        && V_GAUGE[entry_id(fighter.module_accessor)] < 600
        && FLASH_MAX[entry_id(fighter.module_accessor)] != 60 {
            macros::COL_NORMAL(fighter);
            FLASH_MAX[entry_id(fighter.module_accessor)] = 60;
        }
        else if V_GAUGE[entry_id(fighter.module_accessor)] >= 600
        && V_GAUGE[entry_id(fighter.module_accessor)] < 900
        && FLASH_MAX[entry_id(fighter.module_accessor)] != 40 {
            macros::COL_NORMAL(fighter);
            FLASH_MAX[entry_id(fighter.module_accessor)] = 40;
        }
        else if V_GAUGE[entry_id(fighter.module_accessor)] == 900
        && FLASH_MAX[entry_id(fighter.module_accessor)] != 20 {
            macros::COL_NORMAL(fighter);
            FLASH_MAX[entry_id(fighter.module_accessor)] = 20;
        }

        if FLASH_MAX[entry_id(fighter.module_accessor)] != 0 {
            if FLASH_COUNTER[entry_id(fighter.module_accessor)] == 0 {
                FLASH_COUNTER[entry_id(fighter.module_accessor)] = FLASH_MAX[entry_id(fighter.module_accessor)];
            }
            if FLASH_COUNTER[entry_id(fighter.module_accessor)] == FLASH_MAX[entry_id(fighter.module_accessor)] {
                macros::FLASH(fighter, 1, 0.2, 0, 0.75);
            }
            if FLASH_COUNTER[entry_id(fighter.module_accessor)] == FLASH_MAX[entry_id(fighter.module_accessor)] - 5 {
                macros::COL_NORMAL(fighter);
            }
            FLASH_COUNTER[entry_id(fighter.module_accessor)] -= 1;
        }

        // V Skill 1

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
                VS1_CANCEL[entry_id(fighter.module_accessor)] = true;
            }
        }
        else {
            VS1_CANCEL[entry_id(fighter.module_accessor)] = false;
        }

        if (StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_RUN
        && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_SPECIAL_LW)
        && QUICK_STEP_STATE[entry_id(fighter.module_accessor)] == 1 {
            QUICK_STEP_STATE[entry_id(fighter.module_accessor)] = 2;
        }

        if QUICK_STEP_STATE[entry_id(fighter.module_accessor)] == 2
        && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
            QUICK_STEP_STATE[entry_id(fighter.module_accessor)] = 0;
        }

        // V Trigger 1

        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD)
        || (StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_N | *FIGHTER_RYU_STATUS_KIND_SPECIAL_N_COMMAND
        && MotionModule::frame(fighter.module_accessor) >= 13.0) {
            VT1_CANCEL[entry_id(fighter.module_accessor)] = true;
        }
        else {
            VT1_CANCEL[entry_id(fighter.module_accessor)] = false;
        }
        
        if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW != 0 {
            if ((VT1_CANCEL[entry_id(fighter.module_accessor)] == true
            && V_GAUGE[entry_id(fighter.module_accessor)] == 900)
            || (VS1_CANCEL[entry_id(fighter.module_accessor)] == true
            && QUICK_STEP_STATE[entry_id(fighter.module_accessor)] == 0))
            && !is_damage_check(fighter.module_accessor, false) {
                fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), false.into());
            }
        }

        if StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_SPECIAL_HI
        && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_COMMAND
        && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_JUMP
        && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_LANDING
        && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_FALL {
            SHORYUREPPA[entry_id(fighter.module_accessor)] = 0;
        }

        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_LANDING
        && SHORYUREPPA[entry_id(fighter.module_accessor)] == 1 {
            fighter.change_status(FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_COMMAND.into(), false.into());
        }

        if V_TRIGGER[entry_id(fighter.module_accessor)] {
            if _TIME_COUNTER[entry_id(fighter.module_accessor)] < 0 {
                _TIME_COUNTER[entry_id(fighter.module_accessor)] = 32;
            }
            if _TIME_COUNTER[entry_id(fighter.module_accessor)] == 32 {
                EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_flame"), smash::phx::Hash40::new("footl"), &Vector3f{x: 0.0, y: 0.0, z: 0.0}, &Vector3f{x: 90.0, y: 0.0, z: 0.0}, 0.2, true, 0, 0, 0, 0, 0, true, true);
            }
            if _TIME_COUNTER[entry_id(fighter.module_accessor)] == 16 {
                EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_flame"), smash::phx::Hash40::new("footr"), &Vector3f{x: 0.0, y: 0.0, z: 0.0}, &Vector3f{x: 90.0, y: 0.0, z: 0.0}, 0.2, true, 0, 0, 0, 0, 0, true, true);
            }
            _TIME_COUNTER[entry_id(fighter.module_accessor)] -= 1;
        }

        // V Shift

        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_GUARD
        && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
        && V_GAUGE[entry_id(fighter.module_accessor)] >= 300 {
            let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
            if stick_x * PostureModule::lr(fighter.module_accessor) < -0.5 {
                V_GAUGE[entry_id(fighter.module_accessor)] -= 300;
                if V_GAUGE[entry_id(fighter.module_accessor)] < 0 {
                    V_GAUGE[entry_id(fighter.module_accessor)] = 0;
                }
                fighter.change_status(FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_B.into(), false.into());
            }
        }

        if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_lw_step_b") {
            if MotionModule::frame(fighter.module_accessor) <= 1.0
            && V_SHIFT[entry_id(fighter.module_accessor)] == false {
                macros::EFFECT_FOLLOW(fighter, Hash40::new("ken_savingattack_aura"), Hash40::new("hip"), -2, 0, 0, 0, 0, 0, 1.4, true);
                macros::EFFECT_FOLLOW(fighter, Hash40::new("ken_savingattack_aura"), Hash40::new("neck"), 0, 0, 0, 0, 0, 0, 1, true);
                macros::EFFECT_FOLLOW(fighter, Hash40::new("ken_savingattack_aura"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1, true);
                macros::EFFECT_FOLLOW(fighter, Hash40::new("ken_savingattack_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1, true);
                macros::EFFECT_FOLLOW(fighter, Hash40::new("ken_savingattack_aura"), Hash40::new("kneel"), 4, 0, 0, 0, 0, 0, 1.1, true);
                macros::EFFECT_FOLLOW(fighter, Hash40::new("ken_savingattack_aura"), Hash40::new("kneer"), 4, 0, 0, 0, 0, 0, 1.1, true);
            }
            if MotionModule::frame(fighter.module_accessor) == 9.375 {
                if V_SHIFT[entry_id(fighter.module_accessor)] {
                    V_GAUGE[entry_id(fighter.module_accessor)] += 150;
                    SlowModule::set_whole(fighter.module_accessor, 5, 0);
                    macros::SLOW_OPPONENT(fighter, 10.0, 2.0);
                    macros::FILL_SCREEN_MODEL_COLOR(fighter, 0, 3, 0.2, 0.2, 0.2, 0, 0, 0, 1, 1, *smash::lib::lua_const::EffectScreenLayer::GROUND, 205);
                }
            }
            if MotionModule::frame(fighter.module_accessor) == 12.5 {
                SlowModule::clear_whole(fighter.module_accessor);
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
                && V_SHIFT[entry_id(fighter.module_accessor)] {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw"), 0.0, 1.0, false, 0.0, false, false);
                }
                else if V_SHIFT[entry_id(fighter.module_accessor)] {
                    macros::CANCEL_FILL_SCREEN(fighter, 0, 5);
                    V_SHIFT[entry_id(fighter.module_accessor)] = false;
                }
            }
        }
        
        if MotionModule::motion_kind(fighter.module_accessor) != hash40("special_lw_step_b") & hash40("special_lw") {
            V_SHIFT[entry_id(fighter.module_accessor)] = false;
        }
        
        // Tatsu in da air

        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP {
            if SPECIAL_S_START_SIT[entry_id(fighter.module_accessor)] == 1
            && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND
            && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND) {
                fighter.change_status(FIGHTER_RYU_STATUS_KIND_SPECIAL_S_END.into(), false.into());
            }
        }

        // Training Mode Tools

        if smashball::is_training_mode(){
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
            && ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L) {
                if V_GAUGE[entry_id(fighter.module_accessor)] > 300 {
                    V_GAUGE[entry_id(fighter.module_accessor)] -= 300
                }
                else {
                    V_GAUGE[entry_id(fighter.module_accessor)] = 0;
                }
            }
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
            && ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
                if V_GAUGE[entry_id(fighter.module_accessor)] < 900 {
                    V_GAUGE[entry_id(fighter.module_accessor)] += 300;
                }
                else {
                    V_GAUGE[entry_id(fighter.module_accessor)] = 900;
                }
            }
        }
    }
}

pub fn install() {
    install_agent_frames!(
        ken_frame
    );
}