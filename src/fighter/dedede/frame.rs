use {
    crate::imports::*,
    crate::fighter::common::frame::common_fighter_frame
};

unsafe extern "C" fn dedede_jet_hammer_movement(fighter: &mut L2CFighterCommon) {
    // Jet Hammer Movement
    
    if [
        hash40("special_lw_hold"),
        hash40("special_lw_jump"),
        hash40("special_lw_fall"),
        hash40("special_lw_hold_max")
    ].contains(&MotionModule::motion_kind(fighter.module_accessor)) {
        let dedespeedy = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let speed = 1.88;
        macros::SET_SPEED_EX(fighter, speed, dedespeedy, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
}

unsafe extern "C" fn on_main(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    dedede_jet_hammer_movement(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, on_main);
}