use super::*;

extern "C" {
    pub fn guard_cancel_attack_main_common(fighter: &mut L2CFighterCommon) -> L2CValue;
}

unsafe extern "C" fn guard_cancel_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let func = smashline::api::get_target_function("lua2cpp_inkling.nrs", 0x52b0).unwrap();
    let func : fn(&mut L2CFighterCommon, L2CValue) = std::mem::transmute(func);
    let ink : f32 = 10.0;
    func(fighter, ink.into());
    guard_cancel_attack_main_common(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, vars::fighter::status::GUARD_CANCEL_ATTACK, guard_cancel_attack_main);
}