use super::*;

unsafe extern "C" fn dolly_squat_wait_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_SquatWait()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_SQUAT_WAIT, dolly_squat_wait_main);
}