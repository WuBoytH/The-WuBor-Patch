use crate::imports::status_imports::*;

unsafe extern "C" fn ryu_attack_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, ryu::instance::flag::DENJIN_RUSH_INHERIT) {
        VarModule::on_flag(fighter.module_accessor, ryu::status::flag::USED_DENJIN_CHARGE);
        VarModule::off_flag(fighter.module_accessor, ryu::instance::flag::DENJIN_RUSH_INHERIT);
    }
    let original = smashline::original_status(smashline::Main, fighter, *FIGHTER_STATUS_KIND_ATTACK_AIR);
    original(fighter)
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Main, *FIGHTER_STATUS_KIND_ATTACK_AIR, ryu_attack_air_main);
}