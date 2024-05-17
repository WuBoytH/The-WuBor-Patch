use super::*;

unsafe extern "C" fn lucina_special_s2_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_CONTINUE_MOT);
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_air_s2_hi"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(lucina_special_s2_main_loop as *const () as _))
}

unsafe extern "C" fn lucina_special_s2_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if CustomCancelManager::execute_cancel(fighter) {
        return 1.into();
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new("special_s2_hi"),
                0.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    else {
        if MotionModule::is_end(fighter.module_accessor) {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_SQUAT_WAIT.into(), false.into());
            }
        }
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2, lucina_special_s2_main);
}