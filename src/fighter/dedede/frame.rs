use {
    smash::{
        lua2cpp::*,
        hash40,
        app::lua_bind::*,
        lib::lua_const::*
    },
    smash_script::*
};

unsafe extern "C" fn dedede_frame(fighter: &mut L2CFighterCommon) {
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

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, dedede_frame);
}