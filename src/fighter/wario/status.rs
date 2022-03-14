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
    super::vl,
    wubor_utils::{
        vars::*,
        table_const::*
    }
};

#[status_script(agent = "wario", status = FIGHTER_STATUS_KIND_REBIRTH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn wario_rebirth_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_WARIO_INSTANCE_WORK_ID_INT_FINISH_SIGN);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_FLASHING);
    fighter.status_pre_Rebirth()
}

#[status_script(agent = "wario", status = FIGHTER_STATUS_KIND_THROW, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn wario_throw_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("throw_b") {
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_WARIO_STATUS_THROW_FLAG_MOVE) {
            let mut spin = WorkModule::get_float(fighter.module_accessor, FIGHTER_WARIO_STATUS_THROW_FLOAT_SPIN_SPEED);
            let stickx = ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor);
            spin += vl::param_private::throw_b_accel * stickx;
            if spin.abs() > vl::param_private::throw_b_speed_max.abs() {
                if spin < 0.0 {
                    spin = -vl::param_private::throw_b_speed_max;
                }
                else {
                    spin = vl::param_private::throw_b_speed_max;
                }
            }
            WorkModule::set_float(fighter.module_accessor, spin, FIGHTER_WARIO_STATUS_THROW_FLOAT_SPIN_SPEED);
            macros::SET_SPEED_EX(fighter, spin, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        else {
            macros::SET_SPEED_EX(fighter, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
    }
    0.into()
}

#[status_script(agent = "wario", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn wario_speciallw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_SPECIAL_LW_FLAG_MOT_CHANGE);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_SPECIAL_LW_FLAG_JUMP);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_SPECIAL_LW_FLAG_LANDING_ENABLE);
    wario_speciallw_helper(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(wario_speciallw_main_loop as *const () as _))
}

unsafe extern "C" fn wario_speciallw_helper(fighter: &mut L2CFighterCommon) {
    wario_speciallw_mot_helper(fighter);
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

unsafe extern "C" fn wario_speciallw_mot_helper(fighter: &mut L2CFighterCommon) {
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

unsafe extern "C" fn wario_speciallw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
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
            wario_speciallw_helper(fighter);
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

#[status_script(agent = "wario", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn wario_speciallw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_WARIO_STATUS_KIND_SPECIAL_LW_LANDING {
        WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_WARIO_INSTANCE_WORK_ID_INT_FINISH_SIGN);
    }
    0.into()
}

#[status_script(agent = "wario", status = FIGHTER_WARIO_STATUS_KIND_SPECIAL_LW_LANDING, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn wario_speciallw_landing_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_WARIO_INSTANCE_WORK_ID_INT_FINISH_SIGN);
    0.into()
}

pub fn install() {
    install_status_scripts!(
        wario_rebirth_pre,
        wario_throw_exec,
        wario_speciallw_main,
        wario_speciallw_end,
        wario_speciallw_landing_end
    );
}