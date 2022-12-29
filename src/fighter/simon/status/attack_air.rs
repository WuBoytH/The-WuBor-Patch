use {
    smash::{
        lua2cpp::L2CFighterCommon,
        lib::{lua_const::*, L2CValue}
    },
    smashline::*
};

#[status_script(agent = "simon", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn simon_attack_air_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air_uniq_process_exec()
}

#[status_script(agent = "simon", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_CHECK_ATTACK)]
unsafe fn simon_attack_air_check_attack(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    install_status_scripts!(
        simon_attack_air_exec, simon_attack_air_check_attack
    );
}