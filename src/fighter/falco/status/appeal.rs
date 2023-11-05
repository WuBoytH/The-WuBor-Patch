use crate::imports::status_imports::*;

#[status_script(agent = "falco", status = FIGHTER_STATUS_KIND_APPEAL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
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

#[status_script(agent = "falco", status = FIGHTER_STATUS_KIND_APPEAL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe extern "C" fn falco_appeal_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_STATUS_KIND_ATTACK_LW4_START {
        VarModule::off_flag(fighter.module_accessor, falco::instance::flag::KAA);
    }
    fighter.status_end_Appeal()
}

pub fn install() {
    install_status_scripts!(
        falco_appeal_main,
        falco_appeal_end
    );
}