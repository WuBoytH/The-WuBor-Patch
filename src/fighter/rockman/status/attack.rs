use crate::imports::status_imports::*;

#[status("rockman", FIGHTER_STATUS_KIND_ATTACK)]
unsafe fn rockman_attack_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_Attack()
}

pub fn install() {
    rockman_attack_pre::install();
}