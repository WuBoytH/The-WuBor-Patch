use crate::imports::status_imports::*;

#[status_script(agent = "gamewatch", status = FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe extern "C" fn gamewatch_landing_attack_air_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_LandingAttackAir()
}

pub fn install(agent : &mut smashline::Agent) {
    agent.status(smashline::Pre, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, gamewatch_landing_attack_air_pre);
}