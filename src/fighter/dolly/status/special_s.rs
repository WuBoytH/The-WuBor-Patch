use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::lua_bind::*,
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    custom_var::*,
    wubor_utils::{vars::*, table_const::*}
};

#[status_script(agent = "dolly", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn dolly_specials_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_cancel = VarModule::is_flag(fighter.battle_object, dolly::status::flag::IS_SPECIAL_CANCEL);
    let ret = original!(fighter);
    VarModule::set_flag(fighter.battle_object, dolly::status::flag::IS_SPECIAL_CANCEL, is_cancel);
    ret
}

#[status_script(agent = "dolly", status = FIGHTER_DOLLY_STATUS_KIND_SPECIAL_S_COMMAND, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn dolly_specialscommand_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_cancel = VarModule::is_flag(fighter.battle_object, dolly::status::flag::IS_SPECIAL_CANCEL);
    let ret = original!(fighter);
    VarModule::set_flag(fighter.battle_object, dolly::status::flag::IS_SPECIAL_CANCEL, is_cancel);
    ret
}

#[status_script(agent = "dolly", status = FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn dolly_specialb_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_cancel = VarModule::is_flag(fighter.battle_object, dolly::status::flag::IS_SPECIAL_CANCEL);
    let ret = original!(fighter);
    VarModule::set_flag(fighter.battle_object, dolly::status::flag::IS_SPECIAL_CANCEL, is_cancel);
    ret
}

#[status_script(agent = "dolly", status = FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_COMMAND, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn dolly_specialbcommand_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_cancel = VarModule::is_flag(fighter.battle_object, dolly::status::flag::IS_SPECIAL_CANCEL);
    let ret = original!(fighter);
    VarModule::set_flag(fighter.battle_object, dolly::status::flag::IS_SPECIAL_CANCEL, is_cancel);
    ret
}

#[status_script(agent = "dolly", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn dolly_specials_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    dolly_specials_end_main(fighter)
}

#[status_script(agent = "dolly", status = FIGHTER_DOLLY_STATUS_KIND_SPECIAL_S_COMMAND, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn dolly_specials_command_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    dolly_specials_end_main(fighter)
}

#[status_script(agent = "dolly", status = FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn dolly_specialf_attack_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.battle_object, dolly::status::flag::IS_SPECIAL_CANCEL);
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_END {
        WorkModule::set_customize_no(fighter.module_accessor, 0, 1);
    }
    0.into()
}

#[status_script(agent = "dolly", status = FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn dolly_specialb_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    dolly_specials_end_main(fighter)
}

#[status_script(agent = "dolly", status = FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_COMMAND, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn dolly_specialb_command_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    dolly_specials_end_main(fighter)
}

#[status_script(agent = "dolly", status = FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn dolly_specialb_attack_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_LANDING {
        VarModule::off_flag(fighter.battle_object, dolly::status::flag::IS_SPECIAL_CANCEL);
        WorkModule::set_customize_no(fighter.module_accessor, 0, 1);
    }
    0.into()
}

unsafe extern "C" fn dolly_specials_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if status != *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_ATTACK
    && status != *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_ATTACK {
        VarModule::off_flag(fighter.battle_object, dolly::status::flag::IS_SPECIAL_CANCEL);
        WorkModule::set_customize_no(fighter.module_accessor, 0, 1);
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        dolly_specials_pre, dolly_specialscommand_pre,
        dolly_specialb_pre, dolly_specialbcommand_pre,
        dolly_specials_end,
        dolly_specials_command_end,
        dolly_specialf_attack_end,
        dolly_specialb_end,
        dolly_specialb_command_end,
        dolly_specialb_attack_end
    );
}