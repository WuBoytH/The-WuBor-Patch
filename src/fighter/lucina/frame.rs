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
        vars::*,
        gameplay::*
    },
    super::helper::*
};

#[fighter_frame( agent = FIGHTER_KIND_LUCINA )]
fn lucina_frame(fighter: &mut L2CFighterCommon) {
    unsafe {

        // Reset Vars
        
        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH {
            AIR_ACTION[entry_id(fighter.module_accessor)] = false;
            _TIME_COUNTER[entry_id(fighter.module_accessor)] = 0;
            SP_GAUGE_TIMER[entry_id(fighter.module_accessor)] = 0;
            if shadow_id(fighter.module_accessor) {
                if SHADOW_FRENZY[entry_id(fighter.module_accessor)] {
                    SP_GAUGE[entry_id(fighter.module_accessor)] = SP_GAUGE[entry_id(fighter.module_accessor)] / 2.0;
                    SHADOW_FRENZY[entry_id(fighter.module_accessor)] = false;
                }
            }
            else {
                SP_GAUGE_MAX[entry_id(fighter.module_accessor)] = 100.0;
                SP_GAUGE[entry_id(fighter.module_accessor)] = 0.0;
                AWAKENING[entry_id(fighter.module_accessor)] = false;
            }
        }
        if sv_information::is_ready_go() == false {
            DamageModule::set_damage_mul(fighter.module_accessor, 1.0);
        }

        // Meter Controller

        DAMAGE_TAKEN[entry_id(fighter.module_accessor)] = DamageModule::damage(fighter.module_accessor, 0);
        if DAMAGE_TAKEN[entry_id(fighter.module_accessor)] > DAMAGE_TAKEN_PREV[entry_id(fighter.module_accessor)]
        && SHADOW_FRENZY[entry_id(fighter.module_accessor)] == false {
            SP_GAUGE[entry_id(fighter.module_accessor)] += (DAMAGE_TAKEN[entry_id(fighter.module_accessor)] - DAMAGE_TAKEN_PREV[entry_id(fighter.module_accessor)]) * (1.0/6.0);
        }
        DAMAGE_TAKEN_PREV[entry_id(fighter.module_accessor)] = DAMAGE_TAKEN[entry_id(fighter.module_accessor)];

        if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
        && SHADOW_FRENZY[entry_id(fighter.module_accessor)] == false {
            if StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END_MAX
            && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_SPECIAL_S
            && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2
            && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_SPECIAL_HI {
                IS_EX[entry_id(fighter.module_accessor)] = false;
            }
            if IS_EX[entry_id(fighter.module_accessor)] == false {
                METER_GAIN[entry_id(fighter.module_accessor)] = AttackModule::get_power(fighter.module_accessor, 0, false, 1.0, false);
                if shadow_id(fighter.module_accessor) == false {
                    METER_GAIN[entry_id(fighter.module_accessor)] *= 0.75;
                }
                if IS_FUNNY[entry_id(fighter.module_accessor)] {
                    METER_GAIN[entry_id(fighter.module_accessor)] *= 3.0;
                }
                if METER_PENALTY[entry_id(fighter.module_accessor)] > 0.0 {
                    METER_GAIN[entry_id(fighter.module_accessor)] *= 0.1;
                }
                SP_GAUGE[entry_id(fighter.module_accessor)] += METER_GAIN[entry_id(fighter.module_accessor)];
            }
        }

        if SP_GAUGE[entry_id(fighter.module_accessor)] > SP_GAUGE_MAX[entry_id(fighter.module_accessor)] {
            SP_GAUGE[entry_id(fighter.module_accessor)] = SP_GAUGE_MAX[entry_id(fighter.module_accessor)];
        }

        if SP_GAUGE[entry_id(fighter.module_accessor)] >= 25.0
        || SHADOW_FRENZY[entry_id(fighter.module_accessor)] == true {
            if _TIME_COUNTER[entry_id(fighter.module_accessor)] == 0 {
                sp_glow_handler(fighter.module_accessor);
                _TIME_COUNTER[entry_id(fighter.module_accessor)] = 4;
            }
            _TIME_COUNTER[entry_id(fighter.module_accessor)] -= 1;
        }

        // Training Mode Tools

        if smashball::is_training_mode(){
            if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L) {
                if SP_GAUGE[entry_id(fighter.module_accessor)] > 25.0 {
                    SP_GAUGE[entry_id(fighter.module_accessor)] -= 25.0
                }
                else {
                    SP_GAUGE[entry_id(fighter.module_accessor)] = 0.0;
                }
            }
            if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
                if SP_GAUGE[entry_id(fighter.module_accessor)] < SP_GAUGE_MAX[entry_id(fighter.module_accessor)] - 25.0 {
                    SP_GAUGE[entry_id(fighter.module_accessor)] += 25.0
                }
                else {
                    SP_GAUGE[entry_id(fighter.module_accessor)] = SP_GAUGE_MAX[entry_id(fighter.module_accessor)];
                }
            }
            if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) {
                if TRAINING_TOOLS[entry_id(fighter.module_accessor)] {
                    TRAINING_TOOLS[entry_id(fighter.module_accessor)] = false;
                    let pos = Vector3f{x: 0.0, y: 13.0, z: 0.0};
                    let rot = Vector3f{x: 0.0, y: 90.0, z: 0.0};
                    let onemoreeff: u32 = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_counter_flash"), Hash40::new("top"), &pos, &rot, 1.0, false, 0, 0, 0, 0, 0, false, false) as u32;
                    EffectModule::set_rgb(fighter.module_accessor, onemoreeff, 0.0, 0.0, 5.0);
                }
                else {
                    TRAINING_TOOLS[entry_id(fighter.module_accessor)] = true;
                    let pos = Vector3f{x: 0.0, y: 13.0, z: 0.0};
                    let rot = Vector3f{x: 0.0, y: 90.0, z: 0.0};
                    let onemoreeff: u32 = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_counter_flash"), Hash40::new("top"), &pos, &rot, 1.0, false, 0, 0, 0, 0, 0, false, false) as u32;
                    EffectModule::set_rgb(fighter.module_accessor, onemoreeff, 5.0, 0.0, 0.0);
                }
            }
        }

        if SP_LEVEL[entry_id(fighter.module_accessor)] != (SP_GAUGE[entry_id(fighter.module_accessor)] / 25.0) as i32 {
            sp_diff_checker(fighter.module_accessor);
        }

        if METER_PENALTY[entry_id(fighter.module_accessor)] > 0.0 {
            METER_PENALTY[entry_id(fighter.module_accessor)] -= 1.0;
        }

        if SP_GAUGE_TIMER[entry_id(fighter.module_accessor)] > 0 {
            SP_GAUGE_TIMER[entry_id(fighter.module_accessor)] -= 1;
            if SP_GAUGE_TIMER[entry_id(fighter.module_accessor)] == 0 {
                sp_gauge_handler(fighter.module_accessor, true);
            }
        }

        // Normal vs Shadow Effects

        if shadow_id(fighter.module_accessor) == true {
            DamageModule::set_damage_mul(fighter.module_accessor, 0.92);
            DMG_RATIO[entry_id(fighter.module_accessor)] = 0.8;
            if SHADOW_FRENZY[entry_id(fighter.module_accessor)] == true {
                if !TRAINING_TOOLS[entry_id(fighter.module_accessor)] {
                    if IS_FUNNY[entry_id(fighter.module_accessor)] {
                        SP_GAUGE[entry_id(fighter.module_accessor)] -= 1.0/64.0;
                    }
                    else {
                        SP_GAUGE[entry_id(fighter.module_accessor)] -= 1.0/16.0;
                    }
                }
            }
            if SoundModule::is_playing(fighter.module_accessor, Hash40::new("vc_lucina_missfoot01")) {
                SoundModule::stop_se(fighter.module_accessor, Hash40::new("vc_lucina_missfoot01"), 0);
                macros::PLAY_SE(fighter, Hash40::new("vc_shadow_missfoot01"));
            }
            if SoundModule::is_playing(fighter.module_accessor, Hash40::new("vc_lucina_missfoot02")) {
                SoundModule::stop_se(fighter.module_accessor, Hash40::new("vc_lucina_missfoot02"), 0);
                macros::PLAY_SE(fighter, Hash40::new("vc_shadow_missfoot02"));
            }
            if SoundModule::is_playing(fighter.module_accessor, Hash40::new("vc_lucina_damage_twinkle")) {
                SoundModule::stop_se(fighter.module_accessor, Hash40::new("vc_lucina_damage_twinkle"), 0);
                macros::PLAY_SE(fighter, Hash40::new("vc_shadow_damage_twinkle"));
            }
            if SoundModule::is_playing(fighter.module_accessor, Hash40::new("vc_lucina_knockout")) {
                SoundModule::stop_se(fighter.module_accessor, Hash40::new("vc_lucina_knockout"), 0);
                macros::PLAY_SE(fighter, Hash40::new("vc_shadow_knockout"));
            }
        }
        else {
            DMG_RATIO[entry_id(fighter.module_accessor)] = 1.0;
            if DamageModule::damage(fighter.module_accessor, 0) >= 100.0 {
                if AWAKENING[entry_id(fighter.module_accessor)] == false
                && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND
                && (!is_damage_check(fighter.module_accessor, false)
                || IS_FUNNY[entry_id(fighter.module_accessor)])
                && sv_information::is_ready_go() {
                    DamageModule::set_damage_mul(fighter.module_accessor, 0.8);
                    SP_GAUGE_MAX[entry_id(fighter.module_accessor)] = 150.0;
                    SP_GAUGE[entry_id(fighter.module_accessor)] += 50.0;
                    AWAKENING[entry_id(fighter.module_accessor)] = true;
                    macros::FT_START_CUTIN(fighter);
                }
            }
        }

        if SP_GAUGE[entry_id(fighter.module_accessor)] <= 0.0{
            SP_GAUGE[entry_id(fighter.module_accessor)] = 0.0;
            SHADOW_FRENZY[entry_id(fighter.module_accessor)] = false;
        }

        // Special Lw Check

        if StatusModule::prev_status_kind(fighter.module_accessor, 0) != StatusModule::status_kind(fighter.module_accessor) {
            CAN_ONE_MORE[entry_id(fighter.module_accessor)] = false;
        }

        if StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_SPECIAL_LW {
            if (AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
            || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD))
            && MotionModule::motion_kind(fighter.module_accessor) != hash40("catch_attack")
            && (MotionModule::motion_kind(fighter.module_accessor) != hash40("special_hi") || IS_FUNNY[entry_id(fighter.module_accessor)]) {
                CAN_ONE_MORE[entry_id(fighter.module_accessor)] = true;
            }

            if CAN_ONE_MORE[entry_id(fighter.module_accessor)] == true {
                if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW != 0 {
                    if spent_meter(fighter.module_accessor, true) {
                        ROMAN_ON_HIT[entry_id(fighter.module_accessor)] = true;
                        fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), false.into());
                        sp_diff_checker(fighter.module_accessor);
                    }
                }
                else if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI)
                && SP_GAUGE[entry_id(fighter.module_accessor)] == 100.0
                && SHADOW_FRENZY[entry_id(fighter.module_accessor)] == false
                && shadow_id(fighter.module_accessor) {
                    fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT.into(), false.into());
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT, true);
                }
            }
            else if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_THROW {
                let throwframe : f32;
                if MotionModule::motion_kind(fighter.module_accessor) == hash40("throw_f") {
                    throwframe = 18.0;
                }
                else if MotionModule::motion_kind(fighter.module_accessor) == hash40("throw_b") {
                    throwframe = 19.0;
                }
                else if MotionModule::motion_kind(fighter.module_accessor) == hash40("throw_hi") {
                    throwframe = 13.0;
                }
                else if MotionModule::motion_kind(fighter.module_accessor) == hash40("throw_lw") {
                    throwframe = 20.0;
                }
                else{
                    throwframe = 20.0;
                }
                if MotionModule::frame(fighter.module_accessor) > throwframe
                && CAN_ONE_MORE[entry_id(fighter.module_accessor)] == false {
                    CAN_ONE_MORE[entry_id(fighter.module_accessor)] = true;
                }
                if CAN_ONE_MORE[entry_id(fighter.module_accessor)] == true {
                    if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW != 0 {
                        if spent_meter(fighter.module_accessor, true) {
                            ROMAN_ON_HIT[entry_id(fighter.module_accessor)] = true;
                            fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), false.into());
                            sp_diff_checker(fighter.module_accessor);
                        }
                    }
                    else if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI)
                    && SP_GAUGE[entry_id(fighter.module_accessor)] == 100.0
                    && SHADOW_FRENZY[entry_id(fighter.module_accessor)] == false
                    && shadow_id(fighter.module_accessor) {
                        fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT.into(), false.into());
                    }
                }
            }
            else if (!is_damage_check(fighter.module_accessor, false)
            && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_SPECIAL_HI)
            || IS_FUNNY[entry_id(fighter.module_accessor)] {
                if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW != 0 {
                    if spent_meter(fighter.module_accessor, true) {
                        ROMAN_ON_HIT[entry_id(fighter.module_accessor)] = false;
                        fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), false.into());
                        sp_diff_checker(fighter.module_accessor);
                    }
                }
            }
        }

        if SP_FLASH[entry_id(fighter.module_accessor)] > 0 {
            if SP_FLASH[entry_id(fighter.module_accessor)] % 10 == 0 {
                if SHADOW_FRENZY[entry_id(fighter.module_accessor)] {
                    macros::FLASH(fighter, 0.4, 0.0, 1.0, 1.0);
                }
                else {
                    macros::FLASH(fighter, 1.0, 1.0, 0.0, 0.75);
                }
            }
            else if SP_FLASH[entry_id(fighter.module_accessor)] % 5 == 0 {
                macros::COL_NORMAL(fighter);
            }
            SP_FLASH[entry_id(fighter.module_accessor)] -= 1;
        }

        // Air Action Reset

        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND
        || StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_CLIFF {
            AIR_ACTION[entry_id(fighter.module_accessor)] = false;
        }

        // Shadow Frenzy Check

        if MotionModule::motion_kind(fighter.module_accessor) == hash40("appeal_hi_l") || MotionModule::motion_kind(fighter.module_accessor) == hash40("appeal_hi_r") {
            if SP_GAUGE[entry_id(fighter.module_accessor)] == 100.0 && shadow_id(fighter.module_accessor) && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                SHADOW_FRENZY[entry_id(fighter.module_accessor)] = true;
            }
        }
        else if SP_GAUGE[entry_id(fighter.module_accessor)] == 0.0 {
            SHADOW_FRENZY[entry_id(fighter.module_accessor)] = false;
        }

        // Move Effects

        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_12") {
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
            || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP)
                && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
                && get_command_stick_direction(fighter.module_accessor, true) == 6 {
                    StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), true);
                    fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_S.into(), false.into());
                }
                if QCB[entry_id(fighter.module_accessor)] == 3
                && (ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
                || ControlModule::check_button_on_release(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)) {
                    fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_DASH.into(), false.into());
                }
                else {
                    let allowed_cancels = [
                        *FIGHTER_STATUS_KIND_ATTACK_S3,
                        *FIGHTER_STATUS_KIND_ATTACK_LW3,
                        *FIGHTER_STATUS_KIND_ATTACK_HI3,
                        *FIGHTER_STATUS_KIND_SPECIAL_N,
                        *FIGHTER_STATUS_KIND_SPECIAL_S,
                        *FIGHTER_STATUS_KIND_SPECIAL_HI
                    ].to_vec();
                    cancel_system(fighter, *FIGHTER_STATUS_KIND_ATTACK, allowed_cancels);
                }
            }
        }

        if HEROIC_GRAB[entry_id(fighter.module_accessor)] {
            if MotionModule::motion_kind(fighter.module_accessor) == hash40("catch_wait")
            || StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_CATCH_ATTACK {
                fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), true.into());
            }
            if MotionModule::motion_kind(fighter.module_accessor) == hash40("throw_hi")
            && MotionModule::frame(fighter.module_accessor) > 22.0 {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_S3, true);
            }
        }
        if StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_CATCH_WAIT
        && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_CATCH_DASH
        && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_CATCH_TURN
        && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_CATCH_PULL
        && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_CATCH_JUMP
        && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_CATCH_CUT
        && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_CATCH
        && CatchModule::is_catch(fighter.module_accessor) == false
        && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_THROW {
            HEROIC_GRAB[entry_id(fighter.module_accessor)] = false;
        }

        // Jump Cancels

        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_hi3")
        || (MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_dash")
        && IS_FUNNY[entry_id(fighter.module_accessor)])
        || MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_n")
        || MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_hi")
        || MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_lw") {
            jump_cancel_check_hit(fighter, false);
        }
    }
}

pub fn install() {
    install_agent_frames!(
        lucina_frame
    );
}