use {
    smash::{
        lua2cpp::{L2CFighterCommon, L2CWeaponCommon},
        hash40,
        phx::Hash40,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smash_script::*,
    smashline::*,
    super::vl,
    wubor_utils::{
        vars::*,
        table_const::*
    }
};

#[status_script(agent = "toonlink", status = FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn toonlink_specialhi_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let spin = vl::param_special_hi::rslash_charge_max_speed;
    WorkModule::set_float(fighter.module_accessor, spin, FIGHTER_TOONLINK_STATUS_WORK_ID_FLOAT_SPECIAL_HI_SPIN_SPEED);
    WorkModule::set_float(fighter.module_accessor, spin, FIGHTER_TOONLINK_STATUS_WORK_ID_FLOAT_SPECIAL_HI_SPIN_SPEED_MAX);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL_SPECIAL);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(toonlink_specialhi_end_substatus as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(toonlink_specialhi_end_main_loop as *const () as _))
}

unsafe extern "C" fn toonlink_specialhi_end_substatus(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StopModule::is_stop(fighter.module_accessor)
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && WorkModule::get_float(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_WORK_HOLD_FRAME) >= WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("rslash_hold_frame")) as f32 {
        if MotionModule::frame(fighter.module_accessor) > 46.0 {
            macros::SET_SPEED_EX(fighter, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        else if MotionModule::frame(fighter.module_accessor) > 6.0 {
            let mut spin = WorkModule::get_float(fighter.module_accessor, FIGHTER_TOONLINK_STATUS_WORK_ID_FLOAT_SPECIAL_HI_SPIN_SPEED);
            let stickx = ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor);
            let spin_max = WorkModule::get_float(fighter.module_accessor, FIGHTER_TOONLINK_STATUS_WORK_ID_FLOAT_SPECIAL_HI_SPIN_SPEED_MAX);
            spin += vl::param_special_hi::rslash_charge_max_accel * stickx;
            if spin > spin_max {
                spin = spin_max;
            }
            WorkModule::set_float(fighter.module_accessor, spin, FIGHTER_TOONLINK_STATUS_WORK_ID_FLOAT_SPECIAL_HI_SPIN_SPEED);
            macros::SET_SPEED_EX(fighter, spin, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
    }
    0.into()
}

unsafe extern "C" fn toonlink_specialhi_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() == false {
        if !CancelModule::is_enable_cancel(fighter.module_accessor) {
            if toonlink_specialhi_end_situation_check(fighter).get_bool() {
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
            if toonlink_specialhi_end_situation_check(fighter).get_bool() {
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
                    fighter.shift(L2CValue::Ptr(toonlink_specialhi_end_shift as *const () as _));
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
            if fighter.sub_wait_ground_check_common(false.into()).get_bool() == false
            && fighter.sub_air_check_fall_common().get_bool() == false {
                if toonlink_specialhi_end_situation_check(fighter).get_bool() {
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
                if toonlink_specialhi_end_situation_check(fighter).get_bool() {
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
                        fighter.shift(L2CValue::Ptr(toonlink_specialhi_end_shift as *const () as _));
                        return 0.into();
                    }
                    fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
                    return 1.into();
                }
                if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL_SPECIAL)
                && MotionModule::is_end(fighter.module_accessor) {
                    fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
                    return 1.into();
                }
                if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT)
                && MotionModule::is_end(fighter.module_accessor) {
                    fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
                    return 1.into();
                }
            }
        }
        return 0.into();
    }
    0.into()
}

unsafe extern "C" fn toonlink_specialhi_end_situation_check(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_changing(fighter.module_accessor) {
        return 1.into();
    }
    else {
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
        && fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            return 1.into();
        }
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND
        && fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            return 1.into();
        }
    }
    0.into()
}

unsafe extern "C" fn toonlink_specialhi_end_shift(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL_SPECIAL) {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), true.into());
            return 1.into();
        }
    }
    else {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT)
        && MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            return 1.into();
        }
        if toonlink_specialhi_end_situation_check(fighter).get_bool()
        && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), true.into());
            return 1.into();
        }
    }
    0.into()
}

#[status_script(agent = "toonlink_boomerang", status = WN_LINK_BOOMERANG_STATUS_KIND_FLY, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn toonlink_boomerang_fly_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::off_flag(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_INFLICTION);
    WorkModule::off_flag(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_TO_HOP);
    if !StopModule::is_stop(weapon.module_accessor) {
        toonlink_boomerang_fly_substatus(weapon, false.into());
    }
    weapon.global_table[SUB_STATUS].assign(&L2CValue::Ptr(toonlink_boomerang_fly_substatus as *const () as _));
    MotionModule::change_motion(
        weapon.module_accessor,
        Hash40::new("fly"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    weapon.fastshift(L2CValue::Ptr(toonlink_boomerang_fly_main_fastshift as *const () as _))
}

unsafe extern "C" fn toonlink_boomerang_fly_substatus(weapon: &mut L2CWeaponCommon, param_3: L2CValue) -> L2CValue {
    if param_3.get_bool() {
        if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
            notify_event_msc_cmd!(weapon, Hash40::new_raw(0x18b78d41a0));
        }
        toonlink_boomerang_dec_life(weapon);
    }
    0.into()
}

unsafe extern "C" fn toonlink_boomerang_dec_life(weapon: &mut L2CWeaponCommon) {
    WorkModule::dec_int(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_LIFE);
    let life = WorkModule::get_int(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_LIFE);
    if life <= 0 {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
}

unsafe extern "C" fn toonlink_boomerang_fly_main_fastshift(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let sum_speed_length = KineticModule::get_sum_speed_length(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let speed_min = WorkModule::get_param_float(weapon.module_accessor, hash40("param_boomerang"), hash40("speed_min"));
    let speed_mul = WorkModule::get_param_float(weapon.module_accessor, hash40("param_boomerang"), hash40("speed_mul"));
    if !WorkModule::is_flag(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_TO_HOP) {
        if sum_speed_length <= speed_min * speed_mul {
            if WorkModule::is_flag(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_APPLY_FLY_SPEED) {
                weapon.change_status(WN_LINK_BOOMERANG_STATUS_KIND_TURN.into(), false.into());
                return 1.into();
            }
        }
    }
    else {
        weapon.change_status(WN_LINK_BOOMERANG_STATUS_KIND_HOP.into(), false.into());
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        toonlink_specialhi_end_main,
        toonlink_boomerang_fly_main
    );
}