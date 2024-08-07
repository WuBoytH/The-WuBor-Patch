use super::*;

unsafe extern "C" fn dolly_wait_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_Wait()
}

unsafe extern "C" fn dolly_wait_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_Wait()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_WAIT, dolly_wait_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_WAIT, dolly_wait_main);
}