use super::*;

unsafe extern "C" fn ken_landing_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_landing_uniq_process_init();
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Init, *FIGHTER_STATUS_KIND_LANDING, ken_landing_init);

    agent.status(Init, *FIGHTER_STATUS_KIND_LANDING_LIGHT, ken_landing_init);

    agent.status(Init, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, ken_landing_init);
}