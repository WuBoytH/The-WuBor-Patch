use super::*;

unsafe extern "C" fn daisy_attack_air_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_AttackAir()
}

unsafe extern "C" fn daisy_attack_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air_common(true.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(daisy_attack_air_main_loop as *const () as _))
}

unsafe extern "C" fn daisy_attack_air_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_AttackAir_Main()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_ATTACK_AIR, daisy_attack_air_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_AIR, daisy_attack_air_main);
}