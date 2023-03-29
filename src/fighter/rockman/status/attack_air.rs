use crate::imports::status_imports::*;

#[status_script(agent = "rockman", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn rockman_attack_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_AttackAir()
}

#[status_script(agent = "rockman", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn rockman_attack_air_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_AttackAir()
}

pub fn install() {
    install_status_scripts!(
        rockman_attack_air_main, rockman_attack_air_end
    );
}