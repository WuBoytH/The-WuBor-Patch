use crate::imports::status_imports::*;

unsafe extern "C" fn ken_speciallw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn ken_speciallw_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    sv_kinetic_energy!(
        reset_energy,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        ENERGY_STOP_RESET_TYPE_AIR,
        0.0,
        0.0,
        0.0,
        0.0
    );
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    let reset_type = if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        ENERGY_MOTION_RESET_TYPE_AIR_TRANS
    }
    else {
        ENERGY_MOTION_RESET_TYPE_GROUND_TRANS
    };
    sv_kinetic_energy!(
        reset_energy,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_MOTION,
        reset_type,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0
    );
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    sv_kinetic_energy!(
        reset_energy,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        ENERGY_GRAVITY_RESET_TYPE_GRAVITY,
        0.0,
        0.0,
        0.0,
        0.0
    );
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    else {
        let accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("accel_y"));
        sv_kinetic_energy!(
            set_accel,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            -accel_y
        );
        let max_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("max_speed_y"));
        sv_kinetic_energy!(
            set_limit_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            max_y
        );
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    0.into()
}

unsafe extern "C" fn ken_speciallw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_SPECIAL_LW);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        let mot = if VarModule::get_float(fighter.module_accessor, ken::instance::float::V_GAUGE) < 900.0
        || VarModule::is_flag(fighter.module_accessor, ken::instance::flag::V_TRIGGER) {
            VarModule::set_int(
                fighter.module_accessor,
                ken::instance::int::SPECIAL_LW_TYPE,
                ken::SPECIAL_LW_TYPE_QUICK_STEP
            );
            VarModule::set_int(
                fighter.module_accessor,
                ken::instance::int::QUICK_STEP_STATE,
                ken::QUICK_STEP_STATE_RUN
            );
            Hash40::new("run")
        }
        else {
            VarModule::set_int(
                fighter.module_accessor,
                ken::instance::int::SPECIAL_LW_TYPE,
                ken::SPECIAL_LW_TYPE_HEAT_RUSH
            );
            Hash40::new("special_lw_step_f")
        };
        MotionModule::change_motion(
            fighter.module_accessor,
            mot,
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
    }
    else {
        if VarModule::get_float(fighter.module_accessor, ken::instance::float::V_GAUGE) < 900.0
        || VarModule::is_flag(fighter.module_accessor, ken::instance::flag::V_TRIGGER) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_UNIQ);
            macros::SET_SPEED_EX(fighter, 0.8, 0.2, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            VarModule::set_int(
                fighter.module_accessor,
                ken::instance::int::SPECIAL_LW_TYPE,
                ken::SPECIAL_LW_TYPE_QUICK_STEP
            );
            VarModule::set_int(
                fighter.module_accessor,
                ken::instance::int::QUICK_STEP_STATE,
                ken::QUICK_STEP_STATE_DISABLE
            );
        }
        else {
            VarModule::set_int(
                fighter.module_accessor,
                ken::instance::int::SPECIAL_LW_TYPE,
                ken::SPECIAL_LW_TYPE_HEAT_RUSH
            );
        }
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_air_lw_step_f"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
    }
    if VarModule::get_int(fighter.module_accessor, ken::instance::int::SPECIAL_LW_TYPE) == ken::SPECIAL_LW_TYPE_QUICK_STEP {
        fighter.sub_shift_status_main(L2CValue::Ptr(ken_quickstep_loop as *const () as _))
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_RESET);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
        SlowModule::set_whole(fighter.module_accessor, 6, 0);
        macros::SLOW_OPPONENT(fighter, 100.0, 12.0);
        macros::FILL_SCREEN_MODEL_COLOR(fighter, 0, 3, 0.2, 0.2, 0.2, 0, 0, 0, 1, 1, 0, 205);
        let target_id = VarModule::get_int(fighter.module_accessor, fighter::instance::int::TARGET_ID) as u32;
        if sv_battle_object::is_active(target_id) {
            let target_boma = sv_battle_object::module_accessor(target_id);
            let mut diff_x = PostureModule::pos_x(target_boma) - PostureModule::pos_x(fighter.module_accessor);
            if (diff_x > 0.0 && PostureModule::lr(fighter.module_accessor) < 0.0)
            || (diff_x < 0.0 && PostureModule::lr(fighter.module_accessor) > 0.0) {
                PostureModule::reverse_lr(fighter.module_accessor);
                PostureModule::reverse_rot_y_lr(fighter.module_accessor);
            }
            diff_x = diff_x.abs();
            if diff_x > 50.0 {
                diff_x -= 5.0;
            }
            else {
                diff_x = 0.0;
            }
            VarModule::set_float(fighter.module_accessor, ken::instance::float::DIFF_X, diff_x);
            VarModule::set_int(fighter.module_accessor, fighter::instance::int::TARGET_ID, 0);
        }
        else {
            VarModule::set_float(fighter.module_accessor, ken::instance::float::DIFF_X, 0.0);
        }
        macros::PLAY_SE(fighter, Hash40::new("se_ken_special_l01"));
        macros::PLAY_SE(fighter, Hash40::new("vc_ken_special_l01"));
        VarModule::on_flag(fighter.module_accessor, ken::instance::flag::V_TRIGGER);
        VarModule::set_float(fighter.module_accessor, ken::instance::float::V_GAUGE, 0.0);
        fighter.sub_shift_status_main(L2CValue::Ptr(ken_heatrush_loop as *const () as _))
    }
}

unsafe extern "C" fn ken_quickstep_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("run") {
        if CancelModule::is_enable_cancel(fighter.module_accessor) {
            if fighter.sub_transition_group_check_ground_jump_mini_attack().get_bool()
            || fighter.sub_transition_group_check_ground_item().get_bool()
            || fighter.sub_transition_group_check_ground_catch().get_bool()
            || fighter.sub_transition_group_check_ground_escape().get_bool()
            || fighter.sub_transition_group_check_ground_guard().get_bool()
            || fighter.sub_transition_group_check_ground_attack().get_bool()
            || fighter.sub_transition_group_check_ground_jump().get_bool() {
                return 0.into();
            }
        }
    }
    else if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if VarModule::is_flag(fighter.module_accessor, ken::status::flag::SPECIAL_LW_STEP_KICK) {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("attack_s3_s_w"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        VarModule::off_flag(fighter.module_accessor, ken::status::flag::SPECIAL_LW_STEP_KICK);
    }
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_lw_step_f") {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn ken_heatrush_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        fighter.sub_wait_ground_check_common(false.into());
        fighter.sub_air_check_fall_common();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        ken_speciallw_pre,
        ken_speciallw_init,
        ken_speciallw_main
    );
}