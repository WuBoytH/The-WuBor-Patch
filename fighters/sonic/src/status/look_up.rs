use super::*;

extern "C" {
    fn look_up_pre(fighter: &mut L2CFighterCommon) -> L2CValue;
    fn look_up_main_common(fighter: &mut L2CFighterCommon, start_status: L2CValue, wait_status: L2CValue);
    fn look_up_main_loop_common(fighter: &mut L2CFighterCommon, wait_status: L2CValue) -> L2CValue;
}

unsafe extern "C" fn sonic_look_up_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    look_up_main_common(fighter, vars::sonic::status::LOOK_UP.into(), vars::sonic::status::LOOK_UP_WAIT.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(sonic_look_up_main_loop as *const () as _))
}

unsafe extern "C" fn sonic_look_up_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    look_up_main_loop_common(fighter, vars::sonic::status::LOOK_UP_WAIT.into())
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, vars::sonic::status::LOOK_UP, look_up_pre);
    agent.status(Main, vars::sonic::status::LOOK_UP, sonic_look_up_main);
}