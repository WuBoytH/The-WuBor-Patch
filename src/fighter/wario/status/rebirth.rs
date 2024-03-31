use crate::imports::*;

unsafe extern "C" fn wario_rebirth_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(fighter.module_accessor, 0, 0x100000bf); // FIGHTER_WARIO_INSTANCE_WORK_ID_INT_GASS_COUNT
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_FLASHING);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_CHARGE_MAX);
    EffectModule::remove_common(fighter.module_accessor, Hash40::new("wario_charge_max"));
    fighter.status_pre_Rebirth()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Pre, *FIGHTER_STATUS_KIND_REBIRTH, wario_rebirth_pre);
}