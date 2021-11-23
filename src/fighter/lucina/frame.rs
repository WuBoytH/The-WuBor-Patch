use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        phx::{Hash40/*, Vector3f*/},
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
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_N_S);
            _TIME_COUNTER[entry_id(fighter.module_accessor)] = 0;
            WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_YU_INSTANCE_WORK_ID_INT_SP_EFFECT_TIMER);
            if shadow_id(fighter.module_accessor) {
                if WorkModule::is_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_SHADOW_FRENZY) {
                    let sp_gauge = WorkModule::get_float(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAUGE);
                    WorkModule::set_float(fighter.module_accessor, sp_gauge / 2.0, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAUGE);
                }
            }
            else {
                WorkModule::set_float(fighter.module_accessor, 100.0, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAUGE_MAX);
                WorkModule::set_float(fighter.module_accessor, 0.0, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAUGE);
            }
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_AWAKENING);
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_SHADOW_FRENZY);
        }
        if sv_information::is_ready_go() == false {
            DamageModule::set_damage_mul(fighter.module_accessor, 1.0);
        }

        // Meter Controller

        let damage = DamageModule::damage(fighter.module_accessor, 0);
        if damage > DAMAGE_TAKEN_PREV[entry_id(fighter.module_accessor)]
        && !WorkModule::is_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_SHADOW_FRENZY) {
            let add = (damage - DAMAGE_TAKEN_PREV[entry_id(fighter.module_accessor)]) * (1.0/6.0);
            add_sp(fighter.module_accessor, add);
        }
        DAMAGE_TAKEN_PREV[entry_id(fighter.module_accessor)] = damage;

        if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
        && !WorkModule::is_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_SHADOW_FRENZY) {
            if StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END_MAX
            && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_SPECIAL_S
            && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2
            && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_SPECIAL_HI {
                WorkModule::off_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_IS_EX);
            }
            if !WorkModule::is_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_IS_EX) {
                let mut meter_gain = AttackModule::get_power(fighter.module_accessor, 0, false, 1.0, false);
                if shadow_id(fighter.module_accessor) == false {
                    meter_gain *= 0.75;
                }
                if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FUNNY) {
                    meter_gain *= 3.0;
                }
                if WorkModule::get_float(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAIN_PENALTY) > 0.0 {
                    meter_gain *= 0.1;
                }
                add_sp(fighter.module_accessor, meter_gain);
            }
        }

        if WorkModule::get_float(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAUGE) >= 25.0
        || WorkModule::is_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_SHADOW_FRENZY) {
            if _TIME_COUNTER[entry_id(fighter.module_accessor)] == 0 {
                sp_glow_handler(fighter.module_accessor);
                _TIME_COUNTER[entry_id(fighter.module_accessor)] = 4;
            }
            _TIME_COUNTER[entry_id(fighter.module_accessor)] -= 1;
        }

        // Training Mode Tools

        if smashball::is_training_mode() {
            if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L) {
                add_sp(fighter.module_accessor, -25.0);
            }
            if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
                add_sp(fighter.module_accessor, 25.0);
            }
            // if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) {
            //     if TRAINING_TOOLS[entry_id(fighter.module_accessor)] {
            //         TRAINING_TOOLS[entry_id(fighter.module_accessor)] = false;
            //         let pos = Vector3f{x: 0.0, y: 13.0, z: 0.0};
            //         let rot = Vector3f{x: 0.0, y: 90.0, z: 0.0};
            //         let onemoreeff: u32 = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_counter_flash"), Hash40::new("top"), &pos, &rot, 1.0, false, 0, 0, 0, 0, 0, false, false) as u32;
            //         EffectModule::set_rgb(fighter.module_accessor, onemoreeff, 0.0, 0.0, 5.0);
            //     }
            //     else {
            //         TRAINING_TOOLS[entry_id(fighter.module_accessor)] = true;
            //         let pos = Vector3f{x: 0.0, y: 13.0, z: 0.0};
            //         let rot = Vector3f{x: 0.0, y: 90.0, z: 0.0};
            //         let onemoreeff: u32 = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_counter_flash"), Hash40::new("top"), &pos, &rot, 1.0, false, 0, 0, 0, 0, 0, false, false) as u32;
            //         EffectModule::set_rgb(fighter.module_accessor, onemoreeff, 5.0, 0.0, 0.0);
            //     }
            // }
        }

        if WorkModule::get_int(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_INT_SP_LEVEL) != (WorkModule::get_float(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAUGE) / 25.0) as i32 {
            sp_diff_checker(fighter.module_accessor);
        }

        if WorkModule::get_float(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAIN_PENALTY) > 0.0 {
            count_down(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAIN_PENALTY, 1.0);
        }

        if WorkModule::get_int(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_INT_SP_EFFECT_TIMER) > 0 {
            WorkModule::dec_int(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_INT_SP_EFFECT_TIMER);
            if WorkModule::get_int(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_INT_SP_EFFECT_TIMER) == 0 {
                sp_gauge_handler(fighter.module_accessor, true);
            }
        }

        // Normal vs Shadow Effects

        if shadow_id(fighter.module_accessor) == true {
            DamageModule::set_damage_mul(fighter.module_accessor, 0.92);
            DMG_RATIO[entry_id(fighter.module_accessor)] = 0.8;
            if WorkModule::is_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_SHADOW_FRENZY) {
                // if !TRAINING_TOOLS[entry_id(fighter.module_accessor)] {
                    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FUNNY) {
                        add_sp(fighter.module_accessor, -1.0/64.0);
                    }
                    else {
                        add_sp(fighter.module_accessor, -1.0/16.0);
                    }
                // }
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
                if !WorkModule::is_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_AWAKENING)
                && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND
                && (!is_damage_check(fighter.module_accessor, false)
                || WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FUNNY))
                && sv_information::is_ready_go() {
                    DamageModule::set_damage_mul(fighter.module_accessor, 0.8);
                    WorkModule::set_float(fighter.module_accessor, 150.0, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAUGE_MAX);
                    add_sp(fighter.module_accessor, 50.0);
                    WorkModule::on_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_AWAKENING);
                    macros::FT_START_CUTIN(fighter);
                }
            }
        }

        if WorkModule::get_float(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAUGE) <= 0.0 {
            WorkModule::set_float(fighter.module_accessor, 0.0, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAUGE);
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_SHADOW_FRENZY);
        }

        // Special Lw Check

        if StatusModule::prev_status_kind(fighter.module_accessor, 0) != StatusModule::status_kind(fighter.module_accessor) {
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_CAN_ONE_MORE);
        }

        if StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_SPECIAL_LW {
            if (AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
            || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD))
            && MotionModule::motion_kind(fighter.module_accessor) != hash40("catch_attack")
            && (MotionModule::motion_kind(fighter.module_accessor) != hash40("special_hi")
            || WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FUNNY)) {
                WorkModule::on_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_CAN_ONE_MORE);
            }

            if WorkModule::is_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_CAN_ONE_MORE) {
                if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW != 0 {
                    if spent_meter(fighter.module_accessor, true) {
                        WorkModule::on_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_ROMAN_ON_HIT);
                        fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), false.into());
                        sp_diff_checker(fighter.module_accessor);
                    }
                }
                else if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI)
                && WorkModule::get_float(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAUGE) == 100.0
                && !WorkModule::is_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_SHADOW_FRENZY)
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
                && !WorkModule::is_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_CAN_ONE_MORE) {
                    WorkModule::on_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_CAN_ONE_MORE);
                }
                if WorkModule::is_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_CAN_ONE_MORE) {
                    if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW != 0 {
                        if spent_meter(fighter.module_accessor, true) {
                            WorkModule::on_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_ROMAN_ON_HIT);
                            fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), false.into());
                            sp_diff_checker(fighter.module_accessor);
                        }
                    }
                    else if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI)
                    && WorkModule::get_float(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAUGE) == 100.0
                    && !WorkModule::is_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_SHADOW_FRENZY)
                    && shadow_id(fighter.module_accessor) {
                        fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT.into(), false.into());
                    }
                }
            }
            else if (!is_damage_check(fighter.module_accessor, false)
            && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_SPECIAL_HI)
            || WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FUNNY) {
                if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW != 0 {
                    if spent_meter(fighter.module_accessor, true) {
                        WorkModule::off_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_ROMAN_ON_HIT);
                        fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), false.into());
                        sp_diff_checker(fighter.module_accessor);
                    }
                }
            }
        }

        if WorkModule::get_int(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_INT_SP_FLASH_TIMER) > 0 {
            if WorkModule::get_int(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_INT_SP_FLASH_TIMER) % 10 == 0 {
                if WorkModule::is_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_SHADOW_FRENZY) {
                    macros::FLASH(fighter, 0.4, 0.0, 1.0, 1.0);
                }
                else {
                    macros::FLASH(fighter, 1.0, 1.0, 0.0, 0.75);
                }
            }
            else if WorkModule::get_int(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_INT_SP_FLASH_TIMER) % 5 == 0 {
                macros::COL_NORMAL(fighter);
            }
            WorkModule::dec_int(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_INT_SP_FLASH_TIMER);
        }

        // Air Action Reset

        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND
        || StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_CLIFF {
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_N_S);
        }

        // Shadow Frenzy Check

        if MotionModule::motion_kind(fighter.module_accessor) == hash40("appeal_hi_l") || MotionModule::motion_kind(fighter.module_accessor) == hash40("appeal_hi_r") {
            if WorkModule::get_float(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAUGE) == 100.0 && shadow_id(fighter.module_accessor) && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                WorkModule::on_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_SHADOW_FRENZY);
            }
        }
        else if WorkModule::get_float(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAUGE) == 0.0 {
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_SHADOW_FRENZY);
            sp_gauge_handler(fighter.module_accessor, true);
        }

        // Move Effects

        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_12") {
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
            || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP)
                && (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0 {
                    StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), true);
                    fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_S.into(), true.into());
                }
                if QCB[entry_id(fighter.module_accessor)] > 3 {
                    fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_DASH.into(), true.into());
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

        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_HEROIC_GRAB) {
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
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_HEROIC_GRAB);
        }

        // Jump Cancels

        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_hi3")
        || (MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_dash")
        && WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FUNNY))
        || MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_n")
        || MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_hi")
        || MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_lw") {
            jump_cancel_check_hit(fighter, false);
        }

        // println!("SP: {}", WorkModule::get_float(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAUGE));
        // println!("SP Penalty Timer: {}", WorkModule::get_float(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SP_GAIN_PENALTY));
    }
}

pub fn install() {
    install_agent_frames!(
        lucina_frame
    );
}