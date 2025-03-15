use super::*;

extern "C" {
    #[link_name = "guard_cancel_attack_common"]
    fn guard_cancel_attack_common(fighter: &mut L2CFighterCommon);

    #[link_name = "guard_cancel_attack_main_loop_common"]
    fn guard_cancel_attack_main_loop_common(fighter: &mut L2CFighterCommon, is_crouch: L2CValue) -> L2CValue;
}

unsafe extern "C" fn guard_cancel_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    guard_cancel_attack_common(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(guard_cancel_attack_main_loop as *const () as _))
}

unsafe extern "C" fn guard_cancel_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    guard_cancel_attack_main_loop_common(fighter, true.into())
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, vars::fighter::status::GUARD_CANCEL_ATTACK, guard_cancel_attack_main);
}