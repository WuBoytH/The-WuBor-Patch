use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        phx::Hash40,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smash_script::*,
    smashline::*,
    crate::{
        common_funcs::*,
        vars::*,
        table_const::*
    },
    super::vl::*
};

#[status_script(agent = "toonlink", status = FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn toonlink_specialhiend_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let spin : f32;
    if IS_FUNNY[entry_id(fighter.module_accessor)] {
        spin = rslash_charge_max_speed_funny;
    }
    else {
        spin = rslash_charge_max_speed;
    }
    WorkModule::set_float(fighter.module_accessor, spin, FIGHTER_TOONLINK_STATUS_WORK_ID_FLOAT_SPECIAL_HI_SPIN_SPEED);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL_SPECIAL);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(toonlink_specialhiend_stop_or_something as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(toonlink_specialhiend_main_loop as *const () as _))
}

unsafe extern "C" fn toonlink_specialhiend_stop_or_something(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StopModule::is_stop(fighter.module_accessor)
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && WorkModule::get_float(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_WORK_HOLD_FRAME) >= WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("rslash_hold_frame")) as f32 {
        if MotionModule::frame(fighter.module_accessor) > 46.0 {
            macros::SET_SPEED_EX(fighter, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        else if MotionModule::frame(fighter.module_accessor) > 6.0 {
            let mut spin = WorkModule::get_float(fighter.module_accessor, FIGHTER_TOONLINK_STATUS_WORK_ID_FLOAT_SPECIAL_HI_SPIN_SPEED);
            let stickx = ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor);
            spin += rslash_charge_max_accel * stickx;
            if IS_FUNNY[entry_id(fighter.module_accessor)]
            && spin > rslash_charge_max_speed_funny {
                spin = rslash_charge_max_speed_funny;
            }
            else if spin > rslash_charge_max_speed {
                spin = rslash_charge_max_speed;
            }
            WorkModule::set_float(fighter.module_accessor, spin, FIGHTER_TOONLINK_STATUS_WORK_ID_FLOAT_SPECIAL_HI_SPIN_SPEED);
            macros::SET_SPEED_EX(fighter, spin, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
    }
    0.into()
}

unsafe extern "C" fn toonlink_specialhiend_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() == false {
        if !CancelModule::is_enable_cancel(fighter.module_accessor) {
            if toonlink_specialhiend_situation_check(fighter).get_bool() {
                if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
                    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_END_FLAG_FIRST) {
                        MotionModule::change_motion_inherit_frame(
                            fighter.module_accessor,
                            Hash40::new("special_hi"),
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
                            Hash40::new("special_hi"),
                            0.0,
                            1.0,
                            false,
                            0.0,
                            false,
                            false
                        );
                        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_END_FLAG_FIRST);
                    }
                    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL_SPECIAL);
                    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
                    return 0.into();
                }
            }
            if toonlink_specialhiend_situation_check(fighter).get_bool() {
                if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_LINK_SPECIAL_AIR_HI);
                    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_END_FLAG_FIRST) {
                        MotionModule::change_motion_inherit_frame(
                            fighter.module_accessor,
                            Hash40::new("special_air_hi"),
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
                            Hash40::new("special_air_hi"),
                            0.0,
                            1.0,
                            false,
                            0.0,
                            false,
                            false
                        );
                        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_END_FLAG_FIRST);
                    }
                    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL_SPECIAL);
                    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
                    fighter.shift(L2CValue::Ptr(toonlink_specialhiend_shift as *const () as _));
                    return 0.into();
                }
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
                return 1.into();
            }
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL_SPECIAL) {
                if MotionModule::is_end(fighter.module_accessor) {
                    fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
                    return 1.into();
                }
            }
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT) {
                if MotionModule::is_end(fighter.module_accessor) {
                    fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
                    return 1.into();
                }
            }
        }
        else {
            if fighter.sub_wait_ground_check_common(false.into()).get_bool() == false {
                if fighter.sub_air_check_fall_common().get_bool() == false {
                    if toonlink_specialhiend_situation_check(fighter).get_bool() {
                        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
                            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_END_FLAG_FIRST) {
                                MotionModule::change_motion_inherit_frame(
                                    fighter.module_accessor,
                                    Hash40::new("special_hi"),
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
                                    Hash40::new("special_hi"),
                                    0.0,
                                    1.0,
                                    false,
                                    0.0,
                                    false,
                                    false
                                );
                                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_END_FLAG_FIRST);
                            }
                            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL_SPECIAL);
                            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
                            return 0.into();
                        }
                    }
                    if toonlink_specialhiend_situation_check(fighter).get_bool() {
                        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
                            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_LINK_SPECIAL_AIR_HI);
                            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_END_FLAG_FIRST) {
                                MotionModule::change_motion_inherit_frame(
                                    fighter.module_accessor,
                                    Hash40::new("special_air_hi"),
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
                                    Hash40::new("special_air_hi"),
                                    0.0,
                                    1.0,
                                    false,
                                    0.0,
                                    false,
                                    false
                                );
                                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_END_FLAG_FIRST);
                            }
                            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL_SPECIAL);
                            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
                            fighter.shift(L2CValue::Ptr(toonlink_specialhiend_shift as *const () as _));
                            return 0.into();
                        }
                        fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
                        return 1.into();
                    }
                    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL_SPECIAL) {
                        if MotionModule::is_end(fighter.module_accessor) {
                            fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
                            return 1.into();
                        }
                    }
                    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT) {
                        if MotionModule::is_end(fighter.module_accessor) {
                            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
                            return 1.into();
                        }
                    }
                }
            }
        }
        return 0.into();
    }
    0.into()
}

unsafe extern "C" fn toonlink_specialhiend_situation_check(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_changing(fighter.module_accessor) {
        return 1.into();
    }
    else {
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
                return 1.into();
            }
        }
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                return 1.into();
            }
        }
    }
    0.into()
}

unsafe extern "C" fn toonlink_specialhiend_shift(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL_SPECIAL) {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), true.into());
            return 1.into();
        }
    }
    else {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT) {
            if MotionModule::is_end(fighter.module_accessor) {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
                return 1.into();
            }
        }
        if toonlink_specialhiend_situation_check(fighter).get_bool() {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), true.into());
                return 1.into();
            }
        }
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        toonlink_specialhiend_main
    );
}