use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        app::lua_bind::*,
        lib::{lua_const::*, L2CValue}
    },
    smash_script::*,
    smashline::*,
    crate::{
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
    if fighter.global_table[IN_HITLAG].get_bool() == false {
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("throw_b") {
            if MotionModule::frame(fighter.module_accessor) >= 10.0 && MotionModule::frame(fighter.module_accessor) < 57.0 {
                let stickx = ControlModule::get_stick_x(fighter.module_accessor);
                let lr = PostureModule::lr(fighter.module_accessor);
                macros::SET_SPEED_EX(fighter, 1.1 * lr * stickx, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            }
            if MotionModule::frame(fighter.module_accessor) == 57.0 {
                macros::SET_SPEED_EX(fighter, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            }
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
unsafe fn wario_speciallwlanding_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_WARIO_INSTANCE_WORK_ID_INT_FINISH_SIGN);
    0.into()
}

pub fn install() {
    install_status_scripts!(
        wario_rebirth_pre,
        wario_throw_exec,
        wario_speciallw_end,
        wario_speciallwlanding_end
    );
}