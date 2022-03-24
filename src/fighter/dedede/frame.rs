use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        app::lua_bind::*,
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*
};

#[fighter_frame( agent = FIGHTER_KIND_DEDEDE )]
fn dedede_frame(fighter: &mut L2CFighterCommon) {
    unsafe {

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
}

pub fn install() {
    install_agent_frames!(
        dedede_frame
    );
}