use crate::imports::status_imports::*;

unsafe extern "C" fn trail_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_AttackCommon();
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.check_attack_mtrans();
    }
    fighter.global_table[SUB_STATUS3].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_check_attack_mtrans as *const () as _));
    fighter.sub_status_AttackComboCommon();
    let clone = fighter.global_table[SUB_STATUS].clone();
    fighter.global_table["attack_substatus"].assign(&clone);
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(trail_attack_substatus as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_Attack_Main as *const () as _))
}

unsafe extern "C" fn trail_attack_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) {
        let normal_cancels = [
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3
        ].to_vec();
        if normal_cancel_common(fighter, normal_cancels).get_bool() {
            return 0.into();
        }
    }
    let callable: extern "C" fn(&mut L2CFighterCommon, L2CValue) -> L2CValue = std::mem::transmute(fighter.global_table["attack_substatus"].get_ptr());
    callable(fighter, param_1);
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Main, *FIGHTER_STATUS_KIND_ATTACK, trail_attack_main);
}