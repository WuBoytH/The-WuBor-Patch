use crate::imports::status_imports::*;

unsafe extern "C" fn ken_special_s_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_MOTION_FIRST);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_s_end") as i64, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_INT_MOTION_GROUND);
    let mot_air = if VarModule::is_flag(fighter.module_accessor, ken::status::flag::QUICK_STEP_INHERITED) {
        hash40("special_air_s2_end")
    }
    else {
        hash40("special_air_s_end")
    };
    WorkModule::set_int64(fighter.module_accessor, mot_air as i64, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_INT_MOTION_AIR);
    if !StopModule::is_stop(fighter.module_accessor) {
        ken_special_s_end_substatus(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(ken_special_s_end_substatus as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(ken_special_s_end_main_loop as *const () as _))
}

unsafe extern "C" fn ken_special_s_end_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if !param_1.get_bool() {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_FLAG_GROUND);
        }
        else {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_FLAG_GROUND);
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_FLAG_PLAY_LANDING_SE) {
                SoundModule::play_landing_se(fighter.module_accessor, Hash40::new("se_ken_landing02"));
            }
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_FLAG_PLAY_LANDING_SE);
        }
    }
    0.into()
}

unsafe extern "C" fn ken_special_s_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }

    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }

    let sit = fighter.global_table[SITUATION_KIND].get_i32();

    if StatusModule::is_changing(fighter.module_accessor)
    || StatusModule::is_situation_changed(fighter.module_accessor) {
        let mot;
        if sit != *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_INT_MOTION_AIR);
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_MOTION_FIRST) {
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
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_MOTION_FIRST);
            }
            else {
                MotionModule::change_motion_inherit_frame(
                    fighter.module_accessor,
                    Hash40::new_raw(mot),
                    -1.0,
                    1.0,
                    0.0,
                    false,
                    false
                );
            }
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            let air_end_brake_x = if VarModule::is_flag(fighter.module_accessor, ken::status::flag::QUICK_STEP_INHERITED) {
                0.01
            }
            else {
                WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("air_end_brake_x"))
            };
            sv_kinetic_energy!(
                set_brake,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_STOP,
                air_end_brake_x,
                0.0
            );
            let end_air_max_speed_y = if !VarModule::is_flag(fighter.module_accessor, ken::status::flag::QUICK_STEP_INHERITED) {
                WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("end_air_max_speed_y"))
            }
            else {
                WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0)
            };
            sv_kinetic_energy!(
                set_limit_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                end_air_max_speed_y
            );
            let end_air_accel_y = if !VarModule::is_flag(fighter.module_accessor, ken::status::flag::QUICK_STEP_INHERITED) {
                WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("end_air_accel_y"))
            }
            else {
                0.02
            };
            sv_kinetic_energy!(
                set_accel,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                -end_air_accel_y
            );
        }
        else {
            if VarModule::is_flag(fighter.module_accessor, ken::status::flag::QUICK_STEP_INHERITED) {
                WorkModule::set_float(fighter.module_accessor, 16.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
                return 1.into();
            }
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_INT_MOTION_GROUND);
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_MOTION_FIRST) {
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
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_MOTION_FIRST);
            }
            else {
                MotionModule::change_motion_inherit_frame(
                    fighter.module_accessor,
                    Hash40::new_raw(mot),
                    -1.0,
                    1.0,
                    0.0,
                    false,
                    false
                );
            }
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            let end_brake_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("end_brake_x"));
            sv_kinetic_energy!(
                set_brake,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_STOP,
                end_brake_x,
                0.0
            );
        }
        if StatusModule::is_situation_changed(fighter.module_accessor)
        && sit == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        let status = if sit == *SITUATION_KIND_GROUND {
            FIGHTER_STATUS_KIND_WAIT
        }
        else {
            FIGHTER_STATUS_KIND_FALL
        };
        fighter.change_status(status.into(), false.into());
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn ken_special_s_end_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionAnimcmdModule::flush(fighter.module_accessor, false);
    EffectModule::kill_kind(
        fighter.module_accessor,
        Hash40::new("ken_syoryuken_fire"),
        false,
        true
    );
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Main, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_END, ken_special_s_end_main);
    agent.status(smashline::End, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_END, ken_special_s_end_end);
}