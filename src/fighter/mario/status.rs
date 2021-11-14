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
        vars::*,
        table_const::*,
        gameplay::*
    }
};

#[status_script(agent = "mario", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn mario_specials_init(_fighter: &mut L2CFighterCommon) -> L2CValue {
    L2CValue::I32(0)
}

#[status_script(agent = "mario", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn mario_specials_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
        let air_accel_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y_stable"), 0);
        let air_accel_x = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_mul"), 0);
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_S_HOP) {
            fighter.clear_lua_stack();
            lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -air_accel_y);
            sv_kinetic_energy::set_accel(fighter.lua_state_agent);
            fighter.clear_lua_stack();
            lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_accel_x * 0.5);
            sv_kinetic_energy::set_accel_x_mul(fighter.lua_state_agent);
            fighter.clear_lua_stack();
            lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, air_accel_y_stable);
            sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
        }
        else {
            fighter.clear_lua_stack();
            lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -air_accel_y / 1.5);
            sv_kinetic_energy::set_accel(fighter.lua_state_agent);
            fighter.clear_lua_stack();
            lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_accel_x * 0.2);
            sv_kinetic_energy::set_accel_x_mul(fighter.lua_state_agent);
            fighter.clear_lua_stack();
            lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, air_accel_y_stable / 2.0);
            sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
        }
    }
    L2CValue::I32(0)
}

#[status_script(agent = "mario", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn mario_speciallw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PREV_STATUS_KIND].get_i32() != *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_CHARGE {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_BLJ_PREV);
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        WorkModule::set_int(fighter.module_accessor, FIGHTER_MARIO_SPECIAL_LW_KIND_LONG_JUMP, FIGHTER_MARIO_INSTANCE_WORK_ID_INT_SPECIAL_LW_KIND);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_lw_start"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
    }
    else {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_BLJ_PREV);
        WorkModule::set_int(fighter.module_accessor, FIGHTER_MARIO_SPECIAL_LW_KIND_GROUND_POUND, FIGHTER_MARIO_INSTANCE_WORK_ID_INT_SPECIAL_LW_KIND);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_air_lw_start"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(mario_speciallw_main_loop as *const () as _))
}

unsafe extern "C" fn mario_speciallw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT.into(), true.into());
    }
    0.into()
}

#[status_script(agent = "mario", status = FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn mario_speciallw_shoot_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    macros::SA_SET(fighter, *SITUATION_KIND_AIR);
    if WorkModule::get_int(fighter.module_accessor, FIGHTER_MARIO_INSTANCE_WORK_ID_INT_SPECIAL_LW_KIND) == FIGHTER_MARIO_SPECIAL_LW_KIND_LONG_JUMP {
        ControlModule::clear_command(fighter.module_accessor, true);
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_MARIO_STATUS_SPECIAL_LW_FLAG_BLJ);
        let dir = get_command_stick_direction(fighter.module_accessor, true);
        let speed_x : f32;
        let speed_y : f32;
        if [6, 3, 9].contains(&dir) {
            speed_x = 1.7;
            speed_y = 1.6;
            WorkModule::set_int(fighter.module_accessor, FIGHTER_MARIO_LONG_JUMP_S, FIGHTER_MARIO_STATUS_SPECIAL_LW_INT_LONG_JUMP_KIND);
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_BLJ_PREV);
        }
        else if [4, 7, 1].contains(&dir) {
            WorkModule::on_flag(fighter.module_accessor, FIGHTER_MARIO_STATUS_SPECIAL_LW_FLAG_BLJ);
            if WorkModule::is_flag(fighter.module_accessor, FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_BLJ_PREV) {
                speed_x = -1.7;
                speed_y = 0.0;
                WorkModule::set_int(fighter.module_accessor, FIGHTER_MARIO_LONG_JUMP_B, FIGHTER_MARIO_STATUS_SPECIAL_LW_INT_LONG_JUMP_KIND);
            }
            else {
                speed_x = 1.1;
                speed_y = 1.4;
                WorkModule::set_int(fighter.module_accessor, FIGHTER_MARIO_LONG_JUMP_W, FIGHTER_MARIO_STATUS_SPECIAL_LW_INT_LONG_JUMP_KIND);
            }
            WorkModule::on_flag(fighter.module_accessor, FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_BLJ_PREV);
        }
        else {
            speed_x = 1.4;
            speed_y = 1.4;
            WorkModule::set_int(fighter.module_accessor, FIGHTER_MARIO_LONG_JUMP_M, FIGHTER_MARIO_STATUS_SPECIAL_LW_INT_LONG_JUMP_KIND);
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_BLJ_PREV);
        }
        macros::SET_SPEED_EX(fighter, speed_x, speed_y, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    L2CValue::I32(0)
}

#[status_script(agent = "mario", status = FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn mario_speciallw_shoot_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::get_int(fighter.module_accessor, FIGHTER_MARIO_INSTANCE_WORK_ID_INT_SPECIAL_LW_KIND) == FIGHTER_MARIO_SPECIAL_LW_KIND_LONG_JUMP {
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        let air_accel_mul : f32;
        if KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) > 0.0 {
            air_accel_mul = 1.0;
        }
        else {
            air_accel_mul = 0.4;
        }
        let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0) * air_accel_mul;
        let air_accel_x = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_mul"), 0);
        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -air_accel_y);
        sv_kinetic_energy::set_accel(fighter.lua_state_agent);
        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_accel_x * 0.6);
        sv_kinetic_energy::set_accel_x_mul(fighter.lua_state_agent);
    }
    else {
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        let gravity = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        let mystery_param : f32 = -4.0;
        lua_bind::FighterKineticEnergyGravity::set_speed(gravity as *mut smash::app::FighterKineticEnergyGravity, mystery_param);
        lua_bind::FighterKineticEnergyGravity::set_accel(gravity as *mut smash::app::FighterKineticEnergyGravity, 0.0);
    }
    L2CValue::I32(0)
}

#[status_script(agent = "mario", status = FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn mario_speciallw_shoot_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    if WorkModule::get_int(fighter.module_accessor, FIGHTER_MARIO_INSTANCE_WORK_ID_INT_SPECIAL_LW_KIND) == FIGHTER_MARIO_SPECIAL_LW_KIND_LONG_JUMP {
        GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_lw_light"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        fighter.sub_shift_status_main(L2CValue::Ptr(mario_longjump_jump_main_loop as *const () as _))
    }
    else {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_air_lw_light"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        fighter.sub_shift_status_main(L2CValue::Ptr(mario_groundpound_fall_main_loop as *const () as _))
    }
}

unsafe extern "C" fn mario_longjump_jump_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_MARIO_STATUS_SPECIAL_LW_FLAG_LANDING)
    && fighter.sub_air_check_fall_common().get_bool() == false {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_CHARGE.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn mario_groundpound_fall_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    if !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_CATEGORY_MASK_ALL)
    && get_command_stick_direction(fighter.module_accessor, false) == 8 {
        WorkModule::set_int(fighter.module_accessor, FIGHTER_MARIO_SPECIAL_LW_KIND_GROUND_POUND_CANCEL, FIGHTER_MARIO_INSTANCE_WORK_ID_INT_SPECIAL_LW_KIND);
        fighter.change_status(FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_CHARGE.into(), false.into());
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_CHARGE.into(), false.into());
    }
    0.into()
}

#[status_script(agent = "mario", status = FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_CHARGE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn mario_speciallw_charge_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::get_int(fighter.module_accessor, FIGHTER_MARIO_INSTANCE_WORK_ID_INT_SPECIAL_LW_KIND) == FIGHTER_MARIO_SPECIAL_LW_KIND_LONG_JUMP {
        // KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_lw_hold"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        fighter.sub_shift_status_main(L2CValue::Ptr(mario_speciallw_longjump_end_main_loop as *const () as _))
    }
    else if WorkModule::get_int(fighter.module_accessor, FIGHTER_MARIO_INSTANCE_WORK_ID_INT_SPECIAL_LW_KIND) == FIGHTER_MARIO_SPECIAL_LW_KIND_GROUND_POUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_air_lw_hold"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        fighter.sub_shift_status_main(L2CValue::Ptr(mario_groundpound_land_main_loop as *const () as _))
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        macros::SET_SPEED_EX(fighter, 0.0, -2.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_air_lw_heavy"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        fighter.sub_shift_status_main(L2CValue::Ptr(mario_groundpound_cancel_main_loop as *const () as _))
    }
}

unsafe extern "C" fn mario_speciallw_longjump_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        fighter.sub_wait_ground_check_common(L2CValue::I32(0));
        fighter.sub_air_check_fall_common();
    }
    cancel_exceptions(fighter, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT, *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW, false);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn mario_groundpound_land_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        fighter.sub_wait_ground_check_common(L2CValue::I32(0));
        fighter.sub_air_check_fall_common();
    }
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn mario_groundpound_cancel_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
    }
    else if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        mario_specials_init,
        mario_specials_exec,
        // mario_speciallw_pre,
        mario_speciallw_main,
        // mario_speciallw_shoot_pre,
        mario_speciallw_shoot_init,
        mario_speciallw_shoot_exec,
        mario_speciallw_shoot_main,
        // mario_speciallw_charge_pre,
        mario_speciallw_charge_main
    );
}