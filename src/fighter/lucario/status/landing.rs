use crate::imports::status_imports::*;

#[status_script(agent = "lucario", status = FIGHTER_STATUS_KIND_LANDING, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe extern "C" fn lucario_landing_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, lucario::instance::flag::FORCE_LANDING_FALL_SPECIAL) {
        VarModule::off_flag(fighter.module_accessor, lucario::instance::flag::FORCE_LANDING_FALL_SPECIAL);
        StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL);
        return 1.into();
    }
    fighter.status_pre_Landing()
}

#[status_script(agent = "lucario", status = FIGHTER_STATUS_KIND_LANDING_LIGHT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe extern "C" fn lucario_landing_light_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, lucario::instance::flag::FORCE_LANDING_FALL_SPECIAL) {
        VarModule::off_flag(fighter.module_accessor, lucario::instance::flag::FORCE_LANDING_FALL_SPECIAL);
        StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL);
        return 1.into();
    }
    fighter.status_pre_LandingLight()
}

#[status_script(agent = "lucario", status = FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe extern "C" fn lucario_landing_attack_air_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, lucario::instance::flag::FORCE_LANDING_FALL_SPECIAL) {
        VarModule::off_flag(fighter.module_accessor, lucario::instance::flag::FORCE_LANDING_FALL_SPECIAL);
        StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL);
        return 1.into();
    }
    fighter.status_pre_LandingAttackAir()
}

pub fn install(agent : &mut smashline::Agent) {
    agent.status(smashline::Pre, *FIGHTER_STATUS_KIND_LANDING, lucario_landing_pre);

    agent.status(smashline::Pre, *FIGHTER_STATUS_KIND_LANDING_LIGHT, lucario_landing_light_pre);

    agent.status(smashline::Pre, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, lucario_landing_attack_air_pre);
}