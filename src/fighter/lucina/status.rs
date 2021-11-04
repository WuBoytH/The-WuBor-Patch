use {
    smash::{
        lua2cpp::L2CFighterCommon,
        phx::{Hash40, Vector2f},
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
    super::helper::*
};

#[status_script(agent = "lucina", status = FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_LOOP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn lucina_specialnloop_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_loop"), 1.0, 1.0, false, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_loop"), 1.0, 1.0, false, 0.0, false, false);
    }
    if COMMAND[entry_id(fighter.module_accessor)]
    && spent_meter(fighter.module_accessor, false) {
        SP_GAUGE[entry_id(fighter.module_accessor)] -= SPENT_SP[entry_id(fighter.module_accessor)];
        SP_FLASH[entry_id(fighter.module_accessor)] = 40;
        IS_EX[entry_id(fighter.module_accessor)] = true;
        sp_diff_checker(fighter.module_accessor);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(lucina_specialnloop_loop as *const () as _))
}

unsafe extern "C" fn lucina_specialnloop_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
    && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        if IS_EX[entry_id(fighter.module_accessor)] {
            fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END_MAX.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END.into(), false.into());
        }
    }
    else {
        if MotionModule::is_end(fighter.module_accessor) {
            if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
                if IS_EX[entry_id(fighter.module_accessor)] {
                    fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END_MAX.into(), false.into());
                }
                else {
                    fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END.into(), false.into());
                }
            }
            else {
                HEROIC_GRAB[entry_id(fighter.module_accessor)] = true;
                fighter.change_status(FIGHTER_STATUS_KIND_CATCH_DASH.into(), false.into());
            }
        }
    }
    L2CValue::I32(0)
}

#[status_script(agent = "lucina", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn lucina_specials_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_CONTINUE_MOT);
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_RESET);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), true);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        START_SITUATION[entry_id(fighter.module_accessor)] = *SITUATION_KIND_AIR;
        AIR_ACTION[entry_id(fighter.module_accessor)] = true;
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s1"), 1.0, 1.0, false, 0.0, false, false);
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        START_SITUATION[entry_id(fighter.module_accessor)] = *SITUATION_KIND_GROUND;
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s1"), 1.0, 1.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(lucina_specials_loop as *const () as _))
}

unsafe extern "C" fn lucina_specials_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if START_SITUATION[entry_id(fighter.module_accessor)] == *SITUATION_KIND_AIR
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
    }
    else if START_SITUATION[entry_id(fighter.module_accessor)] == *SITUATION_KIND_GROUND
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    else if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        if CancelModule::is_enable_cancel(fighter.module_accessor) {
            fighter.sub_wait_ground_check_common(L2CValue::I32(0));
        }
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    else if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        if CancelModule::is_enable_cancel(fighter.module_accessor) {
            fighter.sub_air_check_fall_common();
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE) {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL_RAW)
            || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK_RAW) {
                fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2.into(), false.into());
            }
        }
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    L2CValue::I32(0)
}

#[status_script(agent = "lucina", status = FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn lucina_specials2_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_CONTINUE_MOT);
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK_RAW) {
        START_SITUATION[entry_id(fighter.module_accessor)] = 1;
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s2_lw"), 1.0, 1.0, false, 0.0, false, false);
    }
    else {
        START_SITUATION[entry_id(fighter.module_accessor)] = 0;
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s2_hi"), 1.0, 1.0, false, 0.0, false, false);
    }   
    fighter.sub_shift_status_main(L2CValue::Ptr(lucina_specials2_loop as *const () as _))
}

unsafe extern "C" fn lucina_specials2_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        fighter.sub_wait_ground_check_common(L2CValue::I32(0));
        fighter.sub_air_check_fall_common();
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            if START_SITUATION[entry_id(fighter.module_accessor)] == 0 {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s2_hi"), 1.0, 1.0, false, 0.0, false, false);
            }
            if START_SITUATION[entry_id(fighter.module_accessor)] == 1 {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s2_lw"), 1.0, 1.0, false, 0.0, false, false);
            }
        }
    }
    else {
        if MotionModule::is_end(fighter.module_accessor) {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR
            && START_SITUATION[entry_id(fighter.module_accessor)] == 1 {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
            else if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                if START_SITUATION[entry_id(fighter.module_accessor)] == 1 {
                    fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
                }
                else {
                    fighter.change_status(FIGHTER_STATUS_KIND_SQUAT_WAIT.into(), false.into());
                }
            }
        }
    }
    L2CValue::I32(0)
}

#[status_script(agent = "lucina", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn lucina_speciallw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    IS_ROMAN_MOVE[entry_id(fighter.module_accessor)] = false;
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw"), 0.0, 1.0, false, 0.0, false, false);
    }   
    fighter.sub_shift_status_main(L2CValue::Ptr(lucina_speciallw_loop as *const () as _))
}

unsafe extern "C" fn lucina_speciallw_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        fighter.sub_wait_ground_check_common(L2CValue::I32(0));
        fighter.sub_air_check_fall_common();
    }
    if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
    || AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
        macros::SLOW_OPPONENT(fighter, 10.0, 20.0);
        AttackModule::clear_all(fighter.module_accessor);
    }
    if IS_ROMAN_MOVE[entry_id(fighter.module_accessor)] {
        PostureModule::add_pos_2d(fighter.module_accessor, &Vector2f{
            x: ROMAN_MOVE[entry_id(fighter.module_accessor)],
            y: 0.0
        });
        ROMAN_MOVE[entry_id(fighter.module_accessor)] *= 0.9;
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    L2CValue::I32(0)
}

pub fn install() {
    install_status_scripts!(
        lucina_specialnloop_main,
        lucina_specials_main,
        lucina_specials2_main,
        lucina_speciallw_main
    );
}