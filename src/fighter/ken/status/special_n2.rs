use crate::imports::status_imports::*;
use crate::fighter::ryu::helper::*;

unsafe extern "C" fn ken_special_n2_command_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let attr = if VarModule::is_flag(fighter.module_accessor, ken::instance::flag::QUICK_STEP_INHERIT) {
        0
    }
    else {
        *FIGHTER_STATUS_ATTR_START_TURN
    };

    if !VarModule::is_flag(fighter.module_accessor, ken::instance::flag::QUICK_STEP_INHERIT) {
        fighter.sub_status_pre_SpecialNCommon();
    }
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
        (
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
            *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON |
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N2_COMMAND
        ) as u64,
        attr as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn ken_special_n2_command_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, ken::instance::flag::QUICK_STEP_INHERIT) {
        VarModule::on_flag(fighter.module_accessor, ken::status::flag::QUICK_STEP_INHERITED);
        VarModule::off_flag(fighter.module_accessor, ken::instance::flag::QUICK_STEP_INHERIT);
    }
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_n2") as i64, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_INT_MOTION_KIND);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_n2") as i64, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_INT_MOTION_KIND_AIR);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND
    || VarModule::is_flag(fighter.module_accessor, ken::status::flag::QUICK_STEP_INHERITED) {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let mut speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let air_speed_y_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("air_speed_y_mul"));
        speed_y *= air_speed_y_mul;
        if speed_y < 0.0 {
            speed_y = 0.0;
        }
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_MOTION,
            ENERGY_MOTION_RESET_TYPE_AIR_TRANS,
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
            speed_y,
            0.0,
            0.0,
            0.0
        );
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        let air_speed_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("air_speed_x_mul"));
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST,
            speed_x * air_speed_x_mul,
            0.0,
            0.0,
            0.0,
            0.0
        );
        let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
        let control_limit_mul_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("control_limit_mul_x"));
        sv_kinetic_energy!(
            set_stable_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            air_speed_x_stable * control_limit_mul_x,
            100.0
        );
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_STOP, fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    }
    0.into()
}

unsafe extern "C" fn ken_special_n2_command_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        ken_special_n2_ground_main(fighter)
    }
    else {
        ken_special_n2_air_main(fighter)
    }
}

unsafe extern "C" fn ken_special_n2_ground_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::set_int64(fighter.module_accessor, ken::status::int64::SPECIAL_N2_GROUND_BRANCH_MOTION, hash40("invalid"));
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_n2_start"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    if !StopModule::is_stop(fighter.module_accessor) {
        ken_special_n2_ground_substatus(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(ken_special_n2_ground_substatus as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(ken_special_n2_ground_main_loop as *const () as _))
}

unsafe extern "C" fn ken_special_n2_ground_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_BUTTON_ON_TIMER);
    }
    if VarModule::is_flag(fighter.module_accessor, ken::status::flag::SPECIAL_N2_GROUND_BRANCH_CHECK) {
        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            let stick_x = fighter.global_table[STICK_X].get_f32();
            let lr = PostureModule::lr(fighter.module_accessor);
            let stick_y = fighter.global_table[STICK_Y].get_f32();
            let squat_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("squat_stick_y"));
            let mot = if stick_y > squat_stick_y.abs() {
                hash40("special_n2_hi")
            }
            else if stick_y < squat_stick_y {
                hash40("special_n2_lw")
            }
            else if stick_x * lr > squat_stick_y.abs() {
                hash40("special_n2_s")
            }
            else {
                hash40("invalid")
            };
            if mot != hash40("invalid") {
                VarModule::set_int64(fighter.module_accessor, ken::status::int64::SPECIAL_N2_GROUND_BRANCH_MOTION, mot);
                VarModule::off_flag(fighter.module_accessor, ken::status::flag::SPECIAL_N2_GROUND_BRANCH_CHECK);
            }
        }
    }
    0.into()
}

unsafe extern "C" fn ken_special_n2_ground_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL)
    && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD | *COLLISION_KIND_MASK_HIT) {
        let sitation = fighter.global_table[SITUATION_KIND].get_i32();
        if ryu_final_hit_cancel(fighter, sitation.into()).get_bool() {
            return 1.into();
        }
    }

    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }

    if VarModule::is_flag(fighter.module_accessor, ken::status::flag::SPECIAL_N2_GROUND_BRANCH_H) {
        let mot = VarModule::get_int64(fighter.module_accessor, ken::status::int64::SPECIAL_N2_GROUND_BRANCH_MOTION);
        if mot == hash40("special_n2_s") {
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new_raw(mot),
                0.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
            VarModule::off_flag(fighter.module_accessor, ken::status::flag::SPECIAL_N2_GROUND_BRANCH_H);
            return 0.into();
        }
    }

    if VarModule::is_flag(fighter.module_accessor, ken::status::flag::SPECIAL_N2_GROUND_BRANCH_LM) {
        let mot = VarModule::get_int64(fighter.module_accessor, ken::status::int64::SPECIAL_N2_GROUND_BRANCH_MOTION);
        if mot == hash40("special_n2_hi")
        || mot == hash40("special_n2_lw") {
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new_raw(mot),
                0.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
            VarModule::off_flag(fighter.module_accessor, ken::status::flag::SPECIAL_N2_GROUND_BRANCH_LM);
            return 0.into();
        }
    }

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return 0.into();
    }

    0.into()
}

unsafe extern "C" fn ken_special_n2_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_air_n2"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    if !StopModule::is_stop(fighter.module_accessor) {
        ken_special_n2_air_substatus(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(ken_special_n2_air_substatus as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(ken_special_n2_air_main_loop as *const () as _))
}

unsafe extern "C" fn ken_special_n2_air_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_BUTTON_ON_TIMER);
    }
    if VarModule::is_flag(fighter.module_accessor, ken::status::flag::SPECIAL_N2_AIR_DISABLE_GRAVITY) {
        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_GRAVITY, fighter.module_accessor);
        VarModule::off_flag(fighter.module_accessor, ken::status::flag::SPECIAL_N2_AIR_DISABLE_GRAVITY);
    }
    0.into()
}

unsafe extern "C" fn ken_special_n2_air_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL)
    && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD | *COLLISION_KIND_MASK_HIT) {
        let sitation = fighter.global_table[SITUATION_KIND].get_i32();
        if ryu_final_hit_cancel(fighter, sitation.into()).get_bool() {
            return 1.into();
        }
    }

    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && VarModule::is_flag(fighter.module_accessor, ken::status::flag::SPECIAL_N2_AIR_ENABLE_LANDING) {
        WorkModule::set_float(fighter.module_accessor, 10.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
        return 0.into();
    }

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }

    0.into()
}

pub unsafe extern "C" fn ken_special_n2_command_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    EffectModule::kill_kind(
        fighter.module_accessor,
        Hash40::new("ken_syoryuken_fire"),
        false,
        true
    );
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Pre, *FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND, ken_special_n2_command_pre);
    agent.status(smashline::Init, *FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND, ken_special_n2_command_init);
    agent.status(smashline::Main, *FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND, ken_special_n2_command_main);
    agent.status(smashline::End, *FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND, ken_special_n2_command_end);
}