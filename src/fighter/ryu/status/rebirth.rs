use crate::imports::status_imports::*;

unsafe extern "C" fn ryu_rebirth_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.module_accessor, ryu::instance::flag::DENJIN_CHARGE);
    VarModule::set_int(fighter.module_accessor, ryu::instance::int::DENJIN_EFF_HANDLE, 0);
    fighter.status_pre_Rebirth()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Pre, *FIGHTER_STATUS_KIND_REBIRTH, ryu_rebirth_pre);
}