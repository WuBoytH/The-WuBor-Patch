use super::*;

unsafe extern "C" fn richter_attack_lw3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_AttackLw3()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_LW3, richter_attack_lw3_main);
}