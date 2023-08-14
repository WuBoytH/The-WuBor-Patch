use crate::imports::status_imports::*;

#[status("rockman", FIGHTER_STATUS_KIND_ATTACK_S4)]
unsafe fn rockman_attack_s4_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_AttackS4()
}

pub fn install() {
    rockman_attack_s4_main::install();
}