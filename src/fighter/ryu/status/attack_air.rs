use crate::imports::*;

unsafe extern "C" fn ryu_attack_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, ryu::instance::flag::DENJIN_RUSH_INHERIT) {
        VarModule::on_flag(fighter.module_accessor, ryu::status::flag::USED_DENJIN_CHARGE);
        VarModule::off_flag(fighter.module_accessor, ryu::instance::flag::DENJIN_RUSH_INHERIT);
    }
    let original = original_status(Main, fighter, *FIGHTER_STATUS_KIND_ATTACK_AIR);
    original(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_AIR, ryu_attack_air_main);
}