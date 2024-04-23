use crate::imports::*;

unsafe extern "C" fn daisy_fall_special_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_FALL_SPECIAL);
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("fall_special"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_fall_common_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(daisy_fall_special_substatus as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(daisy_fall_special_main_loop as *const () as _))
}

unsafe extern "C" fn daisy_fall_special_substatus(fighter: &mut L2CFighterCommon, param2 : L2CValue) -> L2CValue {
    fighter.sub_fall_common_uniq(param2)
}

unsafe extern "C" fn daisy_fall_special_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.sub_transition_group_check_air_cliff().get_bool()
    && !fighter.sub_fall().get_bool() {
        // let parasol_timer = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PEACH_STATUS_SPECIAL_HI_WORK_INT_PARASOL_LIMIT_TIME_COUNTER);
        // if !(0 < parasol_timer) {
            return daisy_fall_special_main_loop_helper(fighter);
        // }
        // let stick_y = fighter.global_table[STICK_Y].get_f32();
        // let jump_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("jump_stick_y"));
        // let special_hi_float;
        // if jump_stick_y > stick_y {
        //     special_hi_float = false;
        // }
        // else {
        //     special_hi_float = ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_PEACH_GENERATE_ARTICLE_KASSAR);
        // }
        // if !special_hi_float {
        //     return daisy_fall_special_main_loop_helper(fighter);
        // }
        // fighter.change_status(FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_FALL.into(), true.into());
        // return 1.into();
    }
    0.into()
}

unsafe extern "C" fn daisy_fall_special_main_loop_helper(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_FALL_SPECIAL)
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
        return 1.into();
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_FALL_SPECIAL, daisy_fall_special_main);
}