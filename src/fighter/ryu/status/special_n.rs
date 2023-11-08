use crate::imports::status_imports::*;
use super::super::helper::*;

unsafe extern "C" fn ryu_special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, ryu::instance::flag::DENJIN_CHARGE) {
        WorkModule::set_int(fighter.module_accessor, *WEAPON_RYU_HADOKEN_TYPE_SYAKUNETU, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_INT_TYPE);
        ryu_denjin_remover(fighter);
        VarModule::on_flag(fighter.module_accessor, ryu::status::flag::USED_DENJIN_CHARGE);
    }
    if !StopModule::is_stop(fighter.module_accessor) {
        ryu_special_n_substatus(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(ryu_special_n_substatus as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(ryu_special_n_main_loop as *const () as _))
}

unsafe extern "C" fn ryu_special_n_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_BUTTON_ON_TIMER);
    }
    else {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SHOOT) {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_FAILED) {
                ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_RYU_GENERATE_ARTICLE_HADOKEN, false, -1);
            }
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SHOOT);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_FIRED);
        }
    }
    0.into()
}

unsafe extern "C" fn ryu_special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
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
    agent.status(smashline::Main, *FIGHTER_STATUS_KIND_SPECIAL_N, ryu_special_n_main);
    agent.status(smashline::Main, *FIGHTER_RYU_STATUS_KIND_SPECIAL_N_COMMAND, ryu_special_n_main);
}