use crate::imports::status_imports::*;

#[status_script(agent = "falco", status = FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe extern "C" fn falco_attacks4hold_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.module_accessor, falco::instance::flag::KAA);
    fighter.status_end_AttackLw4Hold()
}

#[status_script(agent = "falco", status = FIGHTER_STATUS_KIND_ATTACK_LW4, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe extern "C" fn falco_attacks4_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.module_accessor, falco::instance::flag::KAA);
    fighter.status_end_AttackLw4()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Main, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, falco_attacks4hold_end);

    agent.status(smashline::End, *FIGHTER_STATUS_KIND_ATTACK_LW4, falco_attacks4_end);
}