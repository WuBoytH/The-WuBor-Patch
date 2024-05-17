use super::*;

extern "C" {
    #[link_name = "ryu_attack_main_inner"]
    pub fn ryu_attack_main_inner(fighter: &mut L2CFighterCommon) -> L2CValue;
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK, ryu_attack_main_inner);
}