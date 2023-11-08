use crate::imports::status_imports::*;
use super::super::helper::*;

unsafe extern "C" fn ryu_special_n2_command_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_pre_SpecialNCommon();
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
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn ryu_special_n2_command_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_n2") as i64, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_INT_MOTION_KIND);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_n2") as i64, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_INT_MOTION_KIND_AIR);
    let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let mut speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        let speed_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("speed_x_mul"));
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_MOTION,
            ENERGY_MOTION_RESET_TYPE_GROUND_TRANS,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0
        );
        sv_kinetic_energy!(
            set_speed_mul,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_MOTION,
            1.5
        );
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            ENERGY_STOP_RESET_TYPE_GROUND,
            speed_x * speed_x_mul,
            0.0,
            0.0,
            0.0,
            0.0
        );
        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, fighter.module_accessor);
    }
    else {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_SPECIAL_AIR_N) {
            let air_speed_y_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("air_speed_y_mul"));
            speed_y *= air_speed_y_mul;
        }
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
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_SPECIAL_AIR_N);
    }
    0.into()
}

unsafe extern "C" fn ryu_special_n2_command_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, ryu::instance::flag::DENJIN_CHARGE) {
        ryu_denjin_remover(fighter);
        VarModule::on_flag(fighter.module_accessor, ryu::status::flag::USED_DENJIN_CHARGE);
    }
    if !StopModule::is_stop(fighter.module_accessor) {
        ryu_special_n2_substatus(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(ryu_special_n2_substatus as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(ryu_special_n2_main_loop as *const () as _))
}

unsafe extern "C" fn ryu_special_n2_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_BUTTON_ON_TIMER);
    }
    0.into()
}

unsafe extern "C" fn ryu_special_n2_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
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
    if StatusModule::is_changing(fighter.module_accessor)
    || StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            let mot_air = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_INT_MOTION_KIND_AIR);
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_MOTION_FIRST) {
                MotionModule::change_motion(
                    fighter.module_accessor,
                    Hash40::new_raw(mot_air),
                    0.0,
                    1.0,
                    false,
                    0.0,
                    false,
                    false
                );
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_MOTION_FIRST);
            }
            else {
                MotionModule::change_motion_inherit_frame(
                    fighter.module_accessor,
                    Hash40::new_raw(mot_air),
                    -1.0,
                    1.0,
                    0.0,
                    false,
                    false
                );
            }
            if !StatusModule::is_changing(fighter.module_accessor) {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
                let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
                let control_limit_mul_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("control_limit_mul_x"));
                sv_kinetic_energy!(
                    set_limit_speed,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                    air_speed_x_stable * control_limit_mul_x,
                    0.0
                );
            }
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            let mot_ground = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_INT_MOTION_KIND);
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_MOTION_FIRST) {
                MotionModule::change_motion(
                    fighter.module_accessor,
                    Hash40::new_raw(mot_ground),
                    0.0,
                    1.0,
                    false,
                    0.0,
                    false,
                    false
                );
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_MOTION_FIRST);
            }
            else {
                MotionModule::change_motion_inherit_frame(
                    fighter.module_accessor,
                    Hash40::new_raw(mot_ground),
                    -1.0,
                    1.0,
                    0.0,
                    false,
                    false
                );
            }
            if !StatusModule::is_changing(fighter.module_accessor) {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
                sv_kinetic_energy!(
                    set_speed_mul,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_MOTION,
                    1.5
                );
            }
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        let status = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            FIGHTER_STATUS_KIND_WAIT
        }
        else {
            FIGHTER_STATUS_KIND_FALL
        };
        fighter.change_status(status.into(), false.into());
    }
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Pre, *FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND, ryu_special_n2_command_pre);
    agent.status(smashline::Init, *FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND, ryu_special_n2_command_init);
    agent.status(smashline::Main, *FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND, ryu_special_n2_command_main);
}