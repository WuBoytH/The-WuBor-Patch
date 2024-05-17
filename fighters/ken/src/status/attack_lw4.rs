use super::*;

extern "C" {
    #[link_name = "ryu_attack_reset"]
    pub fn ryu_attack_reset(fighter: &mut L2CFighterCommon);
}

unsafe extern "C" fn ken_attack_lw4_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ryu_attack_reset(fighter);
    fighter.status_AttackLw4()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_LW4, ken_attack_lw4_main);
}