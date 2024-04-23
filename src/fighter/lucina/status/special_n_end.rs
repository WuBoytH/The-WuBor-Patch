use crate::imports::*;

unsafe extern "C" fn lucina_special_n_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT);
    if VarModule::is_flag(fighter.module_accessor, yu::status::flag::IS_EX) {
        WorkModule::set_int64(fighter.module_accessor, hash40("special_n_end_max") as i64, *FIGHTER_MARTH_STATUS_SPECIAL_N_WORK_INT_END_MOTION);
        WorkModule::set_int64(fighter.module_accessor, hash40("special_air_n_end_max") as i64, *FIGHTER_MARTH_STATUS_SPECIAL_N_WORK_INT_END_AIR_MOTION);
    }
    else {
        WorkModule::set_int64(fighter.module_accessor, hash40("special_n_end") as i64, *FIGHTER_MARTH_STATUS_SPECIAL_N_WORK_INT_END_MOTION);
        WorkModule::set_int64(fighter.module_accessor, hash40("special_air_n_end") as i64, *FIGHTER_MARTH_STATUS_SPECIAL_N_WORK_INT_END_AIR_MOTION);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(lucina_special_n_end_main_loop as *const () as _))
}

unsafe extern "C" fn lucina_special_n_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if CustomCancelManager::execute_cancel(fighter) {
        return 1.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            lucina_special_n_end_mot_helper(fighter);
        }
    }
    else {
        lucina_special_n_end_mot_helper(fighter);
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

unsafe extern "C" fn lucina_special_n_end_mot_helper(fighter: &mut L2CFighterCommon) {
    let ground_mot;
    let air_mot;
    if VarModule::is_flag(fighter.module_accessor, yu::status::flag::IS_EX) {
        ground_mot = Hash40::new("special_n_end_max");
        air_mot = Hash40::new("special_air_n_end_max");
    }
    else {
        ground_mot = Hash40::new("special_n_end");
        air_mot = Hash40::new("special_air_n_end");
    }
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        let air_brake_x = sv_fighter_util::get_default_fighter_param_air_brake_x(fighter.lua_state_agent);
        sv_kinetic_energy!(
            set_brake,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            air_brake_x,
            0.0
        );
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT) {
            MotionModule::change_motion(
                fighter.module_accessor,
                air_mot,
                0.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT);
        }
        else {
            MotionModule::change_motion_inherit_frame(
                fighter.module_accessor,
                air_mot,
                -1.0,
                1.0,
                0.0,
                false,
                false
            );
        }
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        let ground_brake = sv_fighter_util::get_default_fighter_param_ground_brake(fighter.lua_state_agent);
        sv_kinetic_energy!(
            set_brake,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            ground_brake,
            0.0
        );
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT) {
            MotionModule::change_motion(
                fighter.module_accessor,
                ground_mot,
                0.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT);
        }
        else {
            MotionModule::change_motion_inherit_frame(
                fighter.module_accessor,
                ground_mot,
                -1.0,
                1.0,
                0.0,
                false,
                false
            );
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END, lucina_special_n_end_main);
}