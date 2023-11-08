use crate::imports::status_imports::*;
use super::super::helper::*;

unsafe extern "C" fn ryu_attack_s3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, ryu::instance::flag::DENJIN_RUSH_INHERIT) {
        VarModule::on_flag(fighter.module_accessor, ryu::status::flag::USED_DENJIN_CHARGE);
        VarModule::off_flag(fighter.module_accessor, ryu::instance::flag::DENJIN_RUSH_INHERIT);
    }
    ryu_attack_s3_main_inner(fighter)
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Main, *FIGHTER_STATUS_KIND_ATTACK_S3, ryu_attack_s3_main);
}