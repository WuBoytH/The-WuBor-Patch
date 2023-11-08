use crate::imports::status_imports::*;

pub unsafe extern "C" fn ken_special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, ken::instance::flag::QUICK_STEP_INHERIT) {
        VarModule::on_flag(fighter.module_accessor, ken::status::flag::QUICK_STEP_INHERITED);
        VarModule::off_flag(fighter.module_accessor, ken::instance::flag::QUICK_STEP_INHERIT);
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(ken_special_s_substatus as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(ken_special_s_main_loop as *const () as _))
}

unsafe extern "C" fn ken_special_s_substatus(fighter: &mut L2CFighterCommon, param_2: L2CValue) -> L2CValue {
    if param_2.get_bool() {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_BUTTON_ON_TIMER);
    }
    0.into()
}

unsafe extern "C" fn ken_special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_changing(fighter.module_accessor)
    || StatusModule::is_situation_changed(fighter.module_accessor) {
        ken_special_s_mot_helper(fighter);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn ken_special_s_mot_helper(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if StatusModule::is_changing(fighter.module_accessor) {
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new("special_air_s_start"),
                0.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
        }
        if StatusModule::is_changing(fighter.module_accessor) {
            return;
        }
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("air_accel_y"));
        sv_kinetic_energy!(
            set_accel,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            -air_accel_y
        );
        let air_max_speed_y = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND) {
            1.0
        }
        else {
            WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("air_max_speed_y"))
        };
        sv_kinetic_energy!(
            set_limit_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            air_max_speed_y
        );
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        if StatusModule::is_changing(fighter.module_accessor) {
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new("special_s_start"),
                0.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
        }
        if StatusModule::is_changing(fighter.module_accessor) {
            return;
        }
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        sv_kinetic_energy!(
            set_brake,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            0.0,
            0.0
        );
    }
}

pub unsafe extern "C" fn ken_special_s_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP {
        EffectModule::kill_kind(
            fighter.module_accessor,
            Hash40::new("ken_syoryuken_fire"),
            false,
            true
        );
    }
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Main, *FIGHTER_STATUS_KIND_SPECIAL_S, ken_special_s_main);
    agent.status(smashline::Main, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND, ken_special_s_main);
    agent.status(smashline::End, *FIGHTER_STATUS_KIND_SPECIAL_S, ken_special_s_end);
    agent.status(smashline::End, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND, ken_special_s_end);
}