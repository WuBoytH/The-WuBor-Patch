use crate::imports::*;

unsafe extern "C" fn samusd_wait_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_Wait()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_WAIT, samusd_wait_main);
}