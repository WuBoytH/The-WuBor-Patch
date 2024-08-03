use super::*;

extern "C" {
    fn look_up_rv_pre(fighter: &mut L2CFighterCommon) -> L2CValue;
    fn look_up_rv_main_common(fighter: &mut L2CFighterCommon, start_status: L2CValue, wait_status: L2CValue) -> L2CValue;
}

unsafe extern "C" fn sonic_look_up_rv_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    look_up_rv_main_common(fighter, vars::sonic::status::LOOK_UP.into(), vars::sonic::status::LOOK_UP_WAIT.into())
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, vars::sonic::status::LOOK_UP_RV, look_up_rv_pre);
    agent.status(Main, vars::sonic::status::LOOK_UP_RV, sonic_look_up_rv_main);
}