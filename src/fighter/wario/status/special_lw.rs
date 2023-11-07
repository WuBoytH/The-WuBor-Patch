use crate::imports::status_imports::*;

unsafe extern "C" fn wario_special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_SPECIAL_LW_FLAG_MOT_CHANGE);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_SPECIAL_LW_FLAG_JUMP);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_SPECIAL_LW_FLAG_LANDING_ENABLE);
    wario_special_lw_helper(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(wario_special_lw_main_loop as *const () as _))
}

unsafe extern "C" fn wario_special_lw_helper(fighter: &mut L2CFighterCommon) {
    wario_special_lw_mot_helper(fighter);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_SPECIAL_LW_FLAG_MOT_CHANGE);
}

unsafe extern "C" fn wario_special_lw_mot_helper(fighter: &mut L2CFighterCommon) {
    let level = WorkModule::get_int(fighter.module_accessor, *FIGHTER_WARIO_INSTANCE_WORK_ID_INT_GASS_LEVEL);
    let mot : u64;
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        match level {
            3 => mot = hash40("special_air_lw_fly_r"),
            2 => mot = hash40("special_air_lw_lr"),
            1 => mot = hash40("special_air_lw_mr"),
            _ => mot = hash40("special_air_lw_sr")
        }
    }
    else {
        match level {
            3 => mot = hash40("special_lw_fly_r"),
            2 => mot = hash40("special_lw_lr"),
            1 => mot = hash40("special_lw_mr"),
            _ => mot = hash40("special_lw_sr")
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_SPECIAL_LW_FLAG_MOT_CHANGE) {
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
    else {
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
    }
}

unsafe extern "C" fn wario_special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    let level = WorkModule::get_int(fighter.module_accessor, *FIGHTER_WARIO_INSTANCE_WORK_ID_INT_GASS_LEVEL);
    if !MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            if level <= *FIGHTER_WARIO_GASS_LEVEL_L
            && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_SPECIAL_LW_FLAG_LANDING_ENABLE) {
                fighter.change_status(FIGHTER_WARIO_STATUS_KIND_SPECIAL_LW_LANDING.into(), false.into());
                return 0.into();
            }
        }
        if !StatusModule::is_changing(fighter.module_accessor)
        && StatusModule::is_situation_changed(fighter.module_accessor) {
            wario_special_lw_helper(fighter);
        }
    }
    else {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 0.into();
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            return 0.into();
        }
    }
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, wario_special_lw_main);
}