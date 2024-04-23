use crate::imports::*;
use super::super::super::vl;
use super::super::helper::*;

// FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_ENTER

unsafe extern "C" fn marth_speciallw_enter_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        0,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        0,
        0
    );
    0.into()
}

unsafe extern "C" fn marth_speciallw_enter_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mot;
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        mot = Hash40::new("special_lw_air_enter");
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        if speed_y < 0.0
        && !VarModule::is_flag(fighter.module_accessor, marth::instance::flag::AIR_STANCE) {
            sv_kinetic_energy!(
                set_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                0.0
            );
        }
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
        sv_kinetic_energy!(
            set_limit_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            air_speed_x_stable * vl::param_stance::enter_air_speed_x_mul
        );
        sv_kinetic_energy!(
            mul_x_speed_max,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            vl::param_stance::fall_speed_x_mul
        );
        VarModule::on_flag(fighter.module_accessor, marth::instance::flag::AIR_STANCE);
    }
    else {
        mot = Hash40::new("special_lw_enter");
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    }
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
    fighter.sub_shift_status_main(L2CValue::Ptr(marth_speciallw_enter_main_loop as *const () as _))
}

unsafe extern "C" fn marth_speciallw_enter_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if marth_ground_air_cancel_helper(fighter, true).get_bool() {
        return 1.into();
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        let mot;
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            mot = Hash40::new("special_lw_air_enter");
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
            sv_kinetic_energy!(
                mul_x_speed_max,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                vl::param_stance::fall_speed_x_mul
            );
        }
        else {
            mot = Hash40::new("special_lw_enter");
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        }
        MotionModule::change_motion_inherit_frame(
            fighter.module_accessor,
            mot,
            -1.0,
            1.0,
            0.0,
            false,
            false
        );
    }
    marth_stance_mot_end_helper(fighter);
    0.into()
}

// FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_WAIT

unsafe extern "C" fn marth_speciallw_wait_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        0,
        (*FIGHTER_STATUS_KIND_ITEM_ASSIST_HOIST | *FIGHTER_STATUS_ATTR_INTO_DOOR | *FIGHTER_STATUS_ATTR_ENABLE_ROCKETBELT_EJECT) as u32,
        0,
        0
    );
    0.into()
}

unsafe extern "C" fn marth_speciallw_wait_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mot;
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        mot = Hash40::new("special_lw_air_wait");
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
        sv_kinetic_energy!(
            mul_x_speed_max,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            vl::param_stance::fall_speed_x_mul
        );
    }
    else {
        mot = Hash40::new("special_lw_wait");
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    }
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
    fighter.sub_shift_status_main(L2CValue::Ptr(marth_speciallw_wait_main_loop as *const () as _))
}

unsafe extern "C" fn marth_speciallw_wait_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if marth_ground_air_cancel_helper(fighter, false).get_bool() {
        return 1.into();
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        if fighter.sub_check_command_squat().get_bool() {
            fighter.change_status(marth::status::STANCE_SQUAT.into(), false.into());
            return 0.into();
        }
    }
    if !VarModule::is_flag(fighter.module_accessor, marth::instance::flag::IS_STANCE) {
        fighter.change_status(marth::status::STANCE_EXIT.into(), true.into());
        return 1.into();
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            MotionModule::change_motion_inherit_frame(
                fighter.module_accessor,
                Hash40::new("special_lw_air_wait"),
                -1.0,
                1.0,
                0.0,
                false,
                false
            );
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
            sv_kinetic_energy!(
                mul_x_speed_max,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                vl::param_stance::fall_speed_x_mul
            );
        }
        else {
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
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        }
    }
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_lw_landing")
    && MotionModule::is_end(fighter.module_accessor) {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_lw_wait"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
    }
    0.into()
}

// FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_SQUAT

unsafe extern "C" fn marth_speciallw_squat_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_MOTION,
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
        0,
        (*FIGHTER_STATUS_ATTR_INTO_DOOR | *FIGHTER_STATUS_ATTR_ENABLE_ROCKETBELT_EJECT) as u32,
        0,
        0
    );
    0.into()
}

unsafe extern "C" fn marth_speciallw_squat_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_lw_squat"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(marth_speciallw_squat_main_loop as *const () as _))
}

unsafe extern "C" fn marth_speciallw_squat_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if marth_ground_air_cancel_helper(fighter, false).get_bool() {
        return 1.into();
    }
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        fighter.change_status(marth::status::STANCE_WAIT.into(), true.into());
        return 0.into();
    }
    if !VarModule::is_flag(fighter.module_accessor, marth::instance::flag::IS_STANCE) {
        fighter.change_status(marth::status::STANCE_EXIT.into(), true.into());
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(marth::status::STANCE_SQUAT_WAIT.into(), false.into());
    }
    0.into()
}

// FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_SQUAT_WAIT

unsafe extern "C" fn marth_speciallw_squat_wait_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_lw_squat_wait"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(marth_speciallw_squat_wait_main_loop as *const () as _))
}

unsafe extern "C" fn marth_speciallw_squat_wait_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if marth_ground_air_cancel_helper(fighter, false).get_bool() {
        return 1.into();
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        if !fighter.sub_check_command_squat().get_bool() {
            fighter.change_status(marth::status::STANCE_SQUAT_RV.into(), false.into());
            return 0.into();
        }
    }
    else {
        fighter.change_status(marth::status::STANCE_WAIT.into(), true.into());
        return 0.into();
    }
    if !VarModule::is_flag(fighter.module_accessor, marth::instance::flag::IS_STANCE) {
        fighter.change_status(marth::status::STANCE_EXIT.into(), true.into());
        return 1.into();
    }
    0.into()
}

// FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_SQUAT_RV

unsafe extern "C" fn marth_speciallw_squat_rv_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_lw_squat_rv"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(marth_speciallw_squat_rv_main_loop as *const () as _))
}

unsafe extern "C" fn marth_speciallw_squat_rv_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if marth_ground_air_cancel_helper(fighter, false).get_bool() {
        return 1.into();
    }
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        fighter.change_status(marth::status::STANCE_WAIT.into(), true.into());
        return 0.into();
    }
    if !VarModule::is_flag(fighter.module_accessor, marth::instance::flag::IS_STANCE) {
        fighter.change_status(marth::status::STANCE_EXIT.into(), true.into());
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(marth::status::STANCE_WAIT.into(), false.into());
    }
    0.into()
}

// FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_EXIT

unsafe extern "C" fn marth_speciallw_exit_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        0,
        0,
        0,
        0
    );
    0.into()
}

unsafe extern "C" fn marth_speciallw_exit_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mot = if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        Hash40::new("special_lw_air_exit")
    }
    else {
        Hash40::new("special_lw_exit")
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
    fighter.sub_shift_status_main(L2CValue::Ptr(marth_speciallw_exit_main_loop as *const () as _))
}

unsafe extern "C" fn marth_speciallw_exit_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        let mot = if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            Hash40::new("special_lw_air_exit")
        }
        else {
            Hash40::new("special_lw_exit")
        };
        MotionModule::change_motion_inherit_frame(
            fighter.module_accessor,
            mot,
            -1.0,
            1.0,
            0.0,
            false,
            false
        );
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, marth::status::STANCE_ENTER, marth_speciallw_enter_pre);
    agent.status(Main, marth::status::STANCE_ENTER, marth_speciallw_enter_main);
    agent.status(End, marth::status::STANCE_ENTER, marth_stance_common_end);

    agent.status(Pre, marth::status::STANCE_WAIT, marth_speciallw_wait_pre);
    agent.status(Main, marth::status::STANCE_WAIT, marth_speciallw_wait_main);
    agent.status(End, marth::status::STANCE_WAIT, marth_stance_common_end);

    agent.status(Pre, marth::status::STANCE_SQUAT, marth_speciallw_squat_pre);
    agent.status(Main, marth::status::STANCE_SQUAT, marth_speciallw_squat_main);
    agent.status(End, marth::status::STANCE_SQUAT, marth_stance_common_end);

    agent.status(Pre, marth::status::STANCE_SQUAT_WAIT, marth_speciallw_squat_pre);
    agent.status(Main, marth::status::STANCE_SQUAT_WAIT, marth_speciallw_squat_wait_main);
    agent.status(End, marth::status::STANCE_SQUAT_WAIT, marth_stance_common_end);

    agent.status(Pre, marth::status::STANCE_SQUAT_RV, marth_speciallw_squat_pre);
    agent.status(Main, marth::status::STANCE_SQUAT_RV, marth_speciallw_squat_rv_main);
    agent.status(End, marth::status::STANCE_SQUAT_RV, marth_stance_common_end);

    agent.status(Pre, marth::status::STANCE_EXIT, marth_speciallw_exit_pre);
    agent.status(Main, marth::status::STANCE_EXIT, marth_speciallw_exit_main);
    agent.status(End, marth::status::STANCE_EXIT, marth_stance_common_end);
}