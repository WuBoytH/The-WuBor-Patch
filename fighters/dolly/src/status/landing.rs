use super::*;

unsafe extern "C" fn dolly_landing_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_landing_uniq_process_init();
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Init, *FIGHTER_STATUS_KIND_LANDING, dolly_landing_init);

    agent.status(Init, *FIGHTER_STATUS_KIND_LANDING_LIGHT, dolly_landing_init);
}