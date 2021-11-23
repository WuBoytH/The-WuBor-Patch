use {
    smash::{
        lua2cpp::L2CFighterCommon,
        phx::{Hash40, Vector2f},
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    // smash_script::*,
    smashline::*,
    crate::{
        vars::*,
        table_const::*
    },
    super::helper::*
};

#[status_script(agent = "lucina", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn lucina_specialn_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_N_FLAG_CHARGE_MAX);
    lucina_specialn_mmot_helper(fighter);
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_COMMAND)
    && spent_meter(fighter.module_accessor, false) {
        let spent = WorkModule::get_float(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SPENT_SP);
        add_sp(fighter.module_accessor, -spent);
        WorkModule::set_int(fighter.module_accessor, 40, FIGHTER_YU_INSTANCE_WORK_ID_INT_SP_FLASH_TIMER);
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_IS_EX);
        sp_diff_checker(fighter.module_accessor);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(lucina_specialn_main_loop as *const () as _))
}

unsafe extern "C" fn lucina_specialn_mmot_helper(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT) {
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new("special_air_n_start"),
                1.0,
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
                Hash40::new("special_air_n_start"),
                -1.0,
                1.0,
                0.0,
                false,
                false
            );
        }
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT) {
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new("special_n_start"),
                1.0,
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
                Hash40::new("special_n_start"),
                -1.0,
                1.0,
                0.0,
                false,
                false
            );
        }
    }
}

unsafe extern "C" fn lucina_specialn_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        lucina_specialn_mmot_helper(fighter);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_LOOP.into(), false.into());
    }
    L2CValue::I32(0)
}

#[status_script(agent = "lucina", status = FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_LOOP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn lucina_specialnloop_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_n_loop"),
        1.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(lucina_specialnloop_main_loop as *const () as _))
}

unsafe extern "C" fn lucina_specialnloop_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
    && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_IS_EX) {
            fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END_MAX.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END.into(), false.into());
        }
    }
    else {
        if MotionModule::is_end(fighter.module_accessor) {
            if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
                if WorkModule::is_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_IS_EX) {
                    fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END_MAX.into(), false.into());
                }
                else {
                    fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END.into(), false.into());
                }
            }
            else {
                WorkModule::on_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_HEROIC_GRAB);
                fighter.change_status(FIGHTER_STATUS_KIND_CATCH_DASH.into(), false.into());
            }
        }
    }
    L2CValue::I32(0)
}

#[status_script(agent = "lucina", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn lucina_specials_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_CONTINUE_MOT);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), true);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_AIR, FIGHTER_YU_STATUS_SPECIAL_HI_WORK_INT_START_SITUATION);
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_N_S);
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_air_s1"),
            1.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        fighter.sub_shift_status_main(L2CValue::Ptr(lucina_raginglion_loop as *const () as _))
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_GROUND, FIGHTER_YU_STATUS_SPECIAL_HI_WORK_INT_START_SITUATION);
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_s1"),
            1.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        fighter.sub_shift_status_main(L2CValue::Ptr(lucina_lightningflash_loop as *const () as _))
    }
}

unsafe extern "C" fn lucina_lightningflash_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_COMMAND) {
        ControlModule::reset_flick_sub_x(fighter.module_accessor);
        ControlModule::reset_main_stick(fighter.module_accessor);
        ControlModule::reset_turn_lr(fighter.module_accessor);
    }
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        if CancelModule::is_enable_cancel(fighter.module_accessor) {
            fighter.sub_wait_ground_check_common(0.into());
        }
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn lucina_raginglion_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_COMMAND) {
        ControlModule::reset_flick_sub_x(fighter.module_accessor);
        ControlModule::reset_main_stick(fighter.module_accessor);
        ControlModule::reset_turn_lr(fighter.module_accessor);
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        fighter.sub_air_check_fall_common();
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE) {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL_RAW)
        || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK_RAW) {
            fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2.into(), false.into());
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    0.into()
}

#[status_script(agent = "lucina", status = FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn lucina_specials2_main(fighter: &mut L2CFighterCommon) -> L2CValue {
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
    fighter.sub_shift_status_main(L2CValue::Ptr(lucina_specials2_main_loop as *const () as _))
}

unsafe extern "C" fn lucina_specials2_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        fighter.sub_wait_ground_check_common(L2CValue::I32(0));
        fighter.sub_air_check_fall_common();
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
    }
    else {
        if MotionModule::is_end(fighter.module_accessor) {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_SQUAT_WAIT.into(), false.into());
            }
        }
    }
    L2CValue::I32(0)
}

#[status_script(agent = "lucina", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn lucina_speciallw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_YU_SPECIAL_LW_FLAG_ROMAN_MOVE);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_lw"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
    }
    else {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_air_lw"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
    }   
    fighter.sub_shift_status_main(L2CValue::Ptr(lucina_speciallw_main_loop as *const () as _))
}

unsafe extern "C" fn lucina_speciallw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        fighter.sub_wait_ground_check_common(L2CValue::I32(0));
        fighter.sub_air_check_fall_common();
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_YU_SPECIAL_LW_FLAG_ROMAN_MOVE) {
        let move_x = WorkModule::get_float(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_ROMAN_MOVE);
        PostureModule::add_pos_2d(fighter.module_accessor, &Vector2f{
            x: move_x,
            y: 0.0
        });
        WorkModule::set_float(fighter.module_accessor, move_x * 0.9, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_ROMAN_MOVE);
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
        lucina_specialn_main,
        lucina_specialnloop_main,
        lucina_specials_main,
        lucina_specials2_main,
        lucina_speciallw_main
    );
}