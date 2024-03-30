use {
    crate::imports::*,
    crate::fighter::common::frame::common_fighter_frame
};

unsafe extern "C" fn pzenigame_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, pzenigame_frame);
}