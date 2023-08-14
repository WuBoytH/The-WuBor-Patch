use {
    crate::imports::status_imports::*,
    super::super::common::status::movement::dash::*,
    super::helper::*
};

#[status("ryu", FIGHTER_RYU_STATUS_KIND_DASH_BACK)]
unsafe fn ryu_dashback_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fgc_dashback_pre(fighter)
}

#[status("ryu", FIGHTER_RYU_STATUS_KIND_DASH_BACK)]
unsafe fn ryu_dashback_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fgc_dashback_main(fighter)
}

#[status("ryu", FIGHTER_STATUS_KIND_ATTACK)]
unsafe fn ryu_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_AttackCommon();
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_NEAR_OPPONENT) {
        WorkModule::set_int64(fighter.module_accessor, 0x10556e6036, *FIGHTER_STATUS_ATTACK_WORK_INT_ATTACK11_MOTION);
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_LOG_ATTACK_KIND_ATTACK_NEAR, *FIGHTER_RYU_STATUS_ATTACK_INT_LOG_KIND);
    }
    else {
        WorkModule::set_int64(fighter.module_accessor, 0xb4f4e6f8f, *FIGHTER_STATUS_ATTACK_WORK_INT_ATTACK11_MOTION);
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_LOG_ATTACK_KIND_ATTACK11, *FIGHTER_RYU_STATUS_ATTACK_INT_LOG_KIND);
    }
    if !StopModule::is_stop(fighter.module_accessor) {
        ryu_attack_main_uniq_chk(fighter);
    }
    fighter.global_table[SUB_STATUS3].assign(&L2CValue::Ptr(ryu_attack_main_uniq_chk as *const () as _));
    if !StopModule::is_stop(fighter.module_accessor) {
        ryu_attack_main_uniq_chk4(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(ryu_attack_main_uniq_chk4 as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(ryu_attack_main_loop as *const () as _))
}

unsafe extern "C" fn ryu_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !CancelModule::is_enable_cancel(fighter.module_accessor) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL)
        && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD | *COLLISION_KIND_MASK_HIT)
        && ryu_final_hit_cancel(fighter, SITUATION_KIND_GROUND.into()).get_bool() {
            return 1.into();
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL)
        && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD | *COLLISION_KIND_MASK_HIT)
        && ryu_hit_cancel(fighter, SITUATION_KIND_GROUND.into()).get_bool() {
            return 1.into();
        }
    }
    if ComboModule::count(fighter.module_accessor) == 1
    && !CancelModule::is_enable_cancel(fighter.module_accessor) {
        let current_frame = fighter.global_table[STATUS_FRAME].get_f32();
        let attack_start_cancel_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_private"), hash40("attack_start_cancel_frame"));
        if current_frame < attack_start_cancel_frame
        && ryu_kara_cancel(fighter).get_bool() {
            return 1.into();
        }
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        return 1.into();
    }
    let mot = MotionModule::motion_kind(fighter.module_accessor);
    if [
        hash40("attack_11_w"),
        hash40("attack_11_s"),
        hash40("attack_11_near_s")
    ].contains(&mot) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_WEAK_CANCEL)
        && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_BUTTON_TRIGGER)
        && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            let stick_y = fighter.global_table[STICK_Y].get_f32();
            let attack_hi3_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("attack_hi3_stick_y"));
            let cont = if stick_y >= attack_hi3_stick_y {
                false
            }
            else {
                let attack_lw3_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("attack_lw3_stick_y"));
                if attack_lw3_stick_y >= stick_y {
                    false
                }
                else {
                    let stick_x = fighter.global_table[STICK_X].get_f32();
                    let attack_s3_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("attack_s3_stick_x"));
                    stick_x < attack_s3_stick_x
                }
            };
            if cont {
                fighter.change_status(FIGHTER_STATUS_KIND_ATTACK.into(), false.into());
                return 1.into();
            }
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_SAME_ATTACK_CANCEL) {
            let button_on_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_INT_BUTTON_ON_FRAME);
            let attack_11_s_button_on_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_private"), hash40("attack_11_s_button_on_frame"));
            if attack_11_s_button_on_frame <= button_on_frame as f32 {
                fighter.change_status(FIGHTER_STATUS_KIND_ATTACK.into(), false.into());
                return 1.into();
            }
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }
    if 0 < WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME)
    && !StopModule::is_stop(fighter.module_accessor)
    && fighter.sub_check_button_jump().get_bool() {
        // let mot = MotionModule::motion_kind(fighter.module_accessor);
        // let func = ryu_get_attack_cancel_function(fighter, mot.into()).get_u64();
        // MotionAnimcmdModule::call_script_single(
        //     fighter.module_accessor,
        //     *FIGHTER_ANIMCMD_EXPRESSION,
        //     Hash40::new_raw(func),
        //     -1
        // );
        WorkModule::set_int64(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_CHANGE_LOG);
        fighter.change_status_jump_mini_attack(true.into());
        return 1.into();
    }
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_RESTART) {
        if !MotionModule::is_end(fighter.module_accessor) {
            ryu_idkwhatthisis2(fighter);
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    else {
        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK.into(), false.into());
    }
    0.into()
}

#[status("ryu", FIGHTER_STATUS_KIND_ATTACK_HI3)]
unsafe fn ryu_attackhi3_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, fighter::status::flag::JUMP_CANCEL) {
        FGCModule::jump_cancel_check_hit(fighter, false);
    }
    0.into()
}

#[status("ryu", FIGHTER_STATUS_KIND_SPECIAL_S)]
unsafe fn ryu_specials_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    ryu_specials_init_main(fighter)
}

#[status("ryu", FIGHTER_STATUS_KIND_SPECIAL_S, main)]
unsafe fn ryu_specials(fighter: &mut L2CFighterCommon) -> L2CValue {
    ryu_specials_main(fighter);
    0.into()
}

#[status("ryu", FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND)]
unsafe fn ryu_specials_command_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND);
    ryu_specials_init_main(fighter)
}

#[status("ryu", FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND, main)]
unsafe fn ryu_specials_command(fighter: &mut L2CFighterCommon) -> L2CValue {
    ryu_specials_main(fighter);
    0.into()
}

#[status("ryu", FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP)]
unsafe fn ryu_specials_loop_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    ryu_specials_loop_init_main(fighter)
}

#[status("ryu", FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP)]
unsafe fn ryu_specials_loop_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let start_sit = WorkModule::get_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_INT_START_SITUATION);
    if start_sit != *SITUATION_KIND_GROUND {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_air_s"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
    }
    else {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_s"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
    }
    let eff = if !MotionModule::is_flip(fighter.module_accessor) {
        Hash40::new("ryu_tatsumaki_wind_r")
    }
    else {
        Hash40::new("ryu_tatsumaki_wind_l")
    };
    fighter.clear_lua_stack();
    lua_args!(fighter, MA_MSC_EFFECT_REQUEST_FOLLOW, eff, hash40("rot"), 0.0, 1.5, 0.0, 0.0, 0.0, 0.0, 1.0, false, *EFFECT_SUB_ATTRIBUTE_SYNC_STOP, 0, -1);
    sv_module_access::effect(fighter.lua_state_agent);
    let spineffect = fighter.pop_lua_stack(1).get_u32();
    WorkModule::set_int(fighter.module_accessor, spineffect as i32, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_INT_EFFECT_HANDLE);
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND) {
        let alpha = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("wind_alpha")) * 0.01;
        EffectModule::set_alpha(fighter.module_accessor, spineffect, alpha);
    }
    else {
        let alpha = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("command_wind_alpha")) * 0.01;
        EffectModule::set_alpha(fighter.module_accessor, spineffect, alpha);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(ryu_specials_loop_main_loop as *const () as _))
}

unsafe extern "C" fn ryu_specials_loop_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            }
            else {
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND) {
                    WorkModule::set_float(fighter.module_accessor, 10.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
                    fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
                    return 1.into();
                }
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            }
        }
    }
    else {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            let start_sit = WorkModule::get_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_INT_START_SITUATION);
            if start_sit != *SITUATION_KIND_GROUND {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                sv_kinetic_energy!(
                    set_accel,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_STOP,
                    0.0,
                    0.0
                );
                sv_kinetic_energy!(
                    set_brake,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_STOP,
                    0.0,
                    0.0
                );
                sv_kinetic_energy!(
                    set_stable_speed,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_STOP,
                    0.0,
                    0.0
                );
            }
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        WorkModule::dec_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_INT_LOOP_COUNT);
        let loop_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_INT_LOOP_COUNT);
        if loop_count > 0 {
            let start_sit = WorkModule::get_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_INT_START_SITUATION);
            if start_sit != *SITUATION_KIND_GROUND {
                MotionModule::change_motion(
                    fighter.module_accessor,
                    Hash40::new("special_air_s"),
                    0.0,
                    1.0,
                    false,
                    0.0,
                    false,
                    false
                );
            }
            else {
                MotionModule::change_motion(
                    fighter.module_accessor,
                    Hash40::new("special_s"),
                    0.0,
                    1.0,
                    false,
                    0.0,
                    false,
                    false
                );
            }
        }
        else {
            fighter.change_status(FIGHTER_RYU_STATUS_KIND_SPECIAL_S_END.into(), false.into());
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_FLAG_GROUND);
    }
    else {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_FLAG_GROUND);
    }
    0.into()
}

#[status("ryu", FIGHTER_STATUS_KIND_SPECIAL_HI, main)]
unsafe fn ryu_specialhi(fighter: &mut L2CFighterCommon) -> L2CValue {
    ryu_specialhi_main(fighter);
    0.into()
}

#[status("ryu", FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_COMMAND, main)]
unsafe fn ryu_specialhi_command(fighter: &mut L2CFighterCommon) -> L2CValue {
    ryu_specialhi_main(fighter);
    0.into()
}

#[status("ryu", FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F)]
unsafe fn ryu_speciallw_step_f_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    ryu_speciallw_step_pre(fighter)
}

#[status("ryu", FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_B)]
unsafe fn ryu_speciallw_step_b_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    ryu_speciallw_step_pre(fighter)
}

unsafe fn ryu_speciallw_step_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let keep_flag;
    let keep_int;
    let keep_float;
    if fighter.global_table[PREV_STATUS_KIND].get_i32() != *FIGHTER_STATUS_KIND_SPECIAL_LW {
        keep_flag = *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG;
        keep_int = *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT;
        keep_float = *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT;
    }
    else {
        keep_flag = *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG;
        keep_int = *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT;
        keep_float = *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT;
    }
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        keep_flag,
        keep_int,
        keep_float,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    0.into()
}

// #[status("ryu", FIGHTER_STATUS_KIND_WAIT)]
// unsafe fn ryu_wait_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
//     fighter.status_pre_Wait()
// }

// #[status("ryu", FIGHTER_STATUS_KIND_WAIT)]
// unsafe fn ryu_wait_main(fighter: &mut L2CFighterCommon) -> L2CValue {
//     fighter.status_Wait()
// }

// #[status("ryu", FIGHTER_STATUS_KIND_SQUAT_WAIT)]
// unsafe fn ryu_squatwait_main(fighter: &mut L2CFighterCommon) -> L2CValue {
//     fighter.status_SquatWait()
// }

// #[status("ryu", FIGHTER_STATUS_KIND_WALK)]
// unsafe fn ryu_walk_main(fighter: &mut L2CFighterCommon) -> L2CValue {
//     fighter.status_Walk()
// }

// #[status("ryu", FIGHTER_STATUS_KIND_TURN)]
// unsafe fn ryu_turn_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
//     fighter.status_pre_Turn()
// }

// #[status("ryu", FIGHTER_STATUS_KIND_TURN_DASH)]
// unsafe fn ryu_turndash_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
//     fighter.status_pre_TurnDash()
// }

// #[status("ryu", FIGHTER_STATUS_KIND_LANDING)]
// unsafe fn ryu_landing_init(fighter: &mut L2CFighterCommon) -> L2CValue {
//     fighter.sub_landing_uniq_process_init()
// }

// #[status("ryu", FIGHTER_STATUS_KIND_LANDING_LIGHT)]
// unsafe fn ryu_landinglight_init(fighter: &mut L2CFighterCommon) -> L2CValue {
//     fighter.sub_landing_uniq_process_init()
// }

// #[status("ryu", FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL)]
// unsafe fn ryu_landingfallspecial_init(fighter: &mut L2CFighterCommon) -> L2CValue {
//     fighter.sub_landing_uniq_process_init()
// }

pub fn install() {
    ryu_dashback_pre::install();
    ryu_dashback_main::install();
    ryu_attack_main::install();
    ryu_attackhi3_exec::install();
    ryu_specials_init::install();
    ryu_specials::install();
    ryu_specials_command_init::install();
    ryu_specials_command::install();
    ryu_specials_loop_init::install();
    ryu_specials_loop_main::install();
    ryu_specialhi::install();
    ryu_specialhi_command::install();
    ryu_speciallw_step_f_pre::install();
    ryu_speciallw_step_b_pre::install();
    //ryu_wait_pre::install();
    //ryu_wait_main::install();
    //ryu_squatwait_main::install();
    //ryu_walk_main::install();
    //ryu_turn_pre::install();
    //ryu_turndash_pre::install();
    //ryu_landing_init::install();
    //ryu_landinglight_init::install();
    //ryu_landingfallspecial_init::install();
}