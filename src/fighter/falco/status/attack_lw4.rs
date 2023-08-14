use crate::imports::status_imports::*;

#[status("falco", FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD)]
unsafe fn falco_attacks4hold_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.battle_object, falco::instance::flag::KAA);
    fighter.status_end_AttackLw4Hold()
}

#[status("falco", FIGHTER_STATUS_KIND_ATTACK_LW4)]
unsafe fn falco_attacks4_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.battle_object, falco::instance::flag::KAA);
    fighter.status_end_AttackLw4()
}

pub fn install() {
    falco_attacks4hold_end::install();
    falco_attacks4_end::install();
}