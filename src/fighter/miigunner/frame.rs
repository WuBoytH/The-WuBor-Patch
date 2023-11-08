use {
    crate::imports::status_imports::*,
    crate::fighter::common::frame::common_fighter_frame
};

unsafe extern "C" fn miigunner_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, miigunner_frame);
}