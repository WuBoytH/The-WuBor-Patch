use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        app::lua_bind::*,
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    custom_var::*,
    wubor_utils::{vars::*, table_const::*}
};

#[status_script(agent = "dolly", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn dolly_speciallw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    dolly_speciallw_end_main(fighter)
}

#[status_script(agent = "dolly", status = FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_COMMAND, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn dolly_speciallw_command_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    dolly_speciallw_end_main(fighter)
}

unsafe extern "C" fn dolly_speciallw_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if status != *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_ATTACK {
        VarModule::off_flag(fighter.battle_object, dolly::instance::flag::IS_SPECIAL_CANCEL);
    }
    0.into()
}

#[status_script(agent = "dolly", status = FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn dolly_speciallw_attack_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let param = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_LW_WORK_FLAG_HIT) {
        hash40("landing_frame_hit")
    }
    else {
        hash40("landing_frame_fail")
    };
    let landing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"), param);
    WorkModule::set_float(fighter.module_accessor, landing_frame as f32, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    VarModule::off_flag(fighter.battle_object, dolly::instance::flag::IS_SPECIAL_CANCEL);
    0.into()
}

pub fn install() {
    install_status_scripts!(
        dolly_speciallw_end,
        dolly_speciallw_command_end,
        dolly_speciallw_attack_end
    );
}