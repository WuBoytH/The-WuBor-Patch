use crate::imports::status_imports::*;

fn daisy_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        if VarModule::is_flag(fighter.module_accessor, fighter::instance::flag::DISABLE_SPECIAL_S)
        && (StatusModule::situation_kind(fighter.module_accessor) != *SITUATION_KIND_AIR
        || MiscModule::is_damage_check(fighter.module_accessor, false)) {
            VarModule::off_flag(fighter.module_accessor, fighter::instance::flag::DISABLE_SPECIAL_S);
        }
    }
}

pub fn install(agent : &mut smashline::Agent) {
    agent.on_line(smashline::Main, daisy_frame);
}