use super::*;
use super::super::helper::*;

unsafe extern "C" fn ryu_attack_lw3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, vars::ryu::instance::flag::DENJIN_RUSH_INHERIT) {
        VarModule::on_flag(fighter.module_accessor, vars::ryu::status::flag::USED_DENJIN_CHARGE);
        VarModule::off_flag(fighter.module_accessor, vars::ryu::instance::flag::DENJIN_RUSH_INHERIT);
    }
    ryu_attack_lw3_main_inner(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_LW3, ryu_attack_lw3_main);
}