use crate::imports::status_imports::*;

unsafe extern "C" fn falco_appeal_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = fighter.status_Appeal();
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("appeal_lw_l")
    || MotionModule::motion_kind(fighter.module_accessor) == hash40("appeal_lw_r") {
        VarModule::on_flag(fighter.module_accessor, falco::instance::flag::KAA);
    }
    else {
        VarModule::off_flag(fighter.module_accessor, falco::instance::flag::KAA);
    }
    ret
}

unsafe extern "C" fn falco_appeal_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_STATUS_KIND_ATTACK_LW4_START {
        VarModule::off_flag(fighter.module_accessor, falco::instance::flag::KAA);
    }
    fighter.status_end_Appeal()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Main, *FIGHTER_STATUS_KIND_APPEAL, falco_appeal_main);
    agent.status(smashline::End, *FIGHTER_STATUS_KIND_APPEAL, falco_appeal_end);
}