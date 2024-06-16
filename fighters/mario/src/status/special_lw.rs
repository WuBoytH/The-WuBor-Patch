use super::*;
use super::super::vl;

unsafe extern "C" fn mario_special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PREV_STATUS_KIND].get_i32() != *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_CHARGE {
        VarModule::off_flag(fighter.module_accessor, vars::mario::instance::flag::SPECIAL_LW_BLJ_PREV);
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        VarModule::set_int(fighter.module_accessor, vars::mario::instance::int::SPECIAL_LW_KIND, vars::mario::SPECIAL_LW_KIND_LONG_JUMP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_lw_start"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
    }
    else {
        VarModule::off_flag(fighter.module_accessor, vars::mario::instance::flag::SPECIAL_LW_BLJ_PREV);
        VarModule::set_int(fighter.module_accessor, vars::mario::instance::int::SPECIAL_LW_KIND, vars::mario::SPECIAL_LW_KIND_GROUND_POUND);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_air_lw_start"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(mario_special_lw_main_loop as *const () as _))
}

unsafe extern "C" fn mario_special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT.into(), true.into());
    }
    0.into()
}

unsafe extern "C" fn mario_special_lw_shoot_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS),
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
        *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn mario_special_lw_shoot_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    // macros::SA_SET(fighter, *SITUATION_KIND_AIR);
    if VarModule::get_int(fighter.module_accessor, vars::mario::instance::int::SPECIAL_LW_KIND) == vars::mario::SPECIAL_LW_KIND_LONG_JUMP {
        VarModule::off_flag(fighter.module_accessor, vars::mario::status::flag::SPECIAL_LW_BLJ);
        let dir = FGCModule::get_command_stick_direction(fighter, true);
        let speed_x : f32;
        let speed_y : f32;
        if [6, 3, 9].contains(&dir) {
            speed_x = vl::param_special_lw::long_jump_speed_strong_x;
            speed_y = vl::param_special_lw::long_jump_speed_strong_y;
            VarModule::set_int(fighter.module_accessor, vars::mario::status::int::SPECIAL_LW_LONG_JUMP_KIND, vars::mario::LONG_JUMP_S);
            VarModule::off_flag(fighter.module_accessor, vars::mario::instance::flag::SPECIAL_LW_BLJ_PREV);
        }
        else if [4, 7, 1].contains(&dir) {
            VarModule::on_flag(fighter.module_accessor, vars::mario::status::flag::SPECIAL_LW_BLJ);
            if VarModule::is_flag(fighter.module_accessor, vars::mario::instance::flag::SPECIAL_LW_BLJ_PREV) {
                speed_x = -vl::param_special_lw::long_jump_speed_strong_x;
                speed_y = 0.0;
                VarModule::set_int(fighter.module_accessor, vars::mario::status::int::SPECIAL_LW_LONG_JUMP_KIND, vars::mario::LONG_JUMP_B);
            }
            else {
                speed_x = vl::param_special_lw::long_jump_speed_weak_x;
                speed_y = vl::param_special_lw::long_jump_speed_weak_y;
                VarModule::set_int(fighter.module_accessor, vars::mario::status::int::SPECIAL_LW_LONG_JUMP_KIND, vars::mario::LONG_JUMP_W);
            }
            VarModule::on_flag(fighter.module_accessor, vars::mario::instance::flag::SPECIAL_LW_BLJ_PREV);
        }
        else {
            speed_x = vl::param_special_lw::long_jump_speed_mid_x;
            speed_y = vl::param_special_lw::long_jump_speed_mid_y;
                VarModule::set_int(fighter.module_accessor, vars::mario::status::int::SPECIAL_LW_LONG_JUMP_KIND, vars::mario::LONG_JUMP_M);
            VarModule::off_flag(fighter.module_accessor, vars::mario::instance::flag::SPECIAL_LW_BLJ_PREV);
        }
        macros::SET_SPEED_EX(fighter, speed_x, speed_y, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    0.into()
}

unsafe extern "C" fn mario_special_lw_shoot_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    if VarModule::get_int(fighter.module_accessor, vars::mario::instance::int::SPECIAL_LW_KIND) == vars::mario::SPECIAL_LW_KIND_LONG_JUMP {
        GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_lw_jump"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        fighter.sub_shift_status_main(L2CValue::Ptr(mario_special_lw_longjump_jump_main_loop as *const () as _))
    }
    else {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_air_lw_fall"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        fighter.sub_shift_status_main(L2CValue::Ptr(mario_special_lw_groundpound_fall_main_loop as *const () as _))
    }
}

unsafe extern "C" fn mario_special_lw_longjump_jump_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, vars::mario::status::flag::SPECIAL_LW_LANDING)
    && !fighter.sub_air_check_fall_common().get_bool() {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_CHARGE.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn mario_special_lw_groundpound_fall_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if FGCModule::get_command_stick_direction(fighter, false) == 8 {
        VarModule::set_int(fighter.module_accessor, vars::mario::instance::int::SPECIAL_LW_KIND, vars::mario::SPECIAL_LW_KIND_GROUND_POUND_CANCEL);
        fighter.change_status(FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_CHARGE.into(), false.into());
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_CHARGE.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn mario_special_lw_shoot_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::get_int(fighter.module_accessor, vars::mario::instance::int::SPECIAL_LW_KIND) == vars::mario::SPECIAL_LW_KIND_LONG_JUMP {
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        let air_accel_mul = if KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) > 0.0 {
            1.0
        }
        else {
            vl::param_special_lw::long_jump_air_accel_y_mul
        };
        let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0) * air_accel_mul;
        let air_accel_x = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_mul"), 0);
        sv_kinetic_energy!(
            set_accel,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            -air_accel_y
        );
        sv_kinetic_energy!(
            set_accel_x_mul,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            air_accel_x * vl::param_special_lw::long_jump_air_accel_x_mul
        );
    }
    else {
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        let gravity = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        lua_bind::FighterKineticEnergyGravity::set_speed(
            gravity as *mut smash::app::FighterKineticEnergyGravity,
            -vl::param_special_lw::ground_pound_fall_speed
        );
        lua_bind::FighterKineticEnergyGravity::set_accel(gravity as *mut smash::app::FighterKineticEnergyGravity, 0.0);
    }
    0.into()
}

unsafe extern "C" fn mario_special_lw_charge_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::get_int(fighter.module_accessor, vars::mario::instance::int::SPECIAL_LW_KIND) == vars::mario::SPECIAL_LW_KIND_LONG_JUMP {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_lw_landing"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        fighter.sub_shift_status_main(L2CValue::Ptr(mario_special_lw_longjump_end_main_loop as *const () as _))
    }
    else if VarModule::get_int(fighter.module_accessor, vars::mario::instance::int::SPECIAL_LW_KIND) == vars::mario::SPECIAL_LW_KIND_GROUND_POUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_air_lw_landing"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        fighter.sub_shift_status_main(L2CValue::Ptr(mario_special_lw_groundpound_land_main_loop as *const () as _))
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        macros::SET_SPEED_EX(fighter, 0.0, -1.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_air_lw_cancel"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        fighter.sub_shift_status_main(L2CValue::Ptr(mario_special_lw_groundpound_cancel_main_loop as *const () as _))
    }
}

unsafe extern "C" fn mario_special_lw_longjump_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if FGCModule::cancel_exceptions(fighter, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT, *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW, false).get_bool() {
        return 1.into();
    }
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn mario_special_lw_groundpound_land_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn mario_special_lw_groundpound_cancel_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
    }
    else if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, mario_special_lw_main);

    agent.status(Pre, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT, mario_special_lw_shoot_pre);
    agent.status(Init, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT, mario_special_lw_shoot_init);
    agent.status(Main, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT, mario_special_lw_shoot_main);
    agent.status(Exec, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT, mario_special_lw_shoot_exec);

    agent.status(Main, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_CHARGE, mario_special_lw_charge_main);
}