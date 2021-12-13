use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        app::lua_bind::*,
        lib::lua_const::*
    },
    smashline::*,
    crate::table_const::*
};

#[fighter_frame( agent = FIGHTER_KIND_JACK )]
fn jack_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        if [
            hash40("special_s1"),
            hash40("special_air_s1")
        ].contains(&MotionModule::motion_kind(fighter.module_accessor)) {
            if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_s1")
            && fighter.global_table[PREV_SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND
            && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
            && MotionModule::frame(fighter.module_accessor) >= 34.0 {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, true);
            }
            if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_RESET);
            }
        }
    }
}

pub fn install() {
    install_agent_frames!(
        jack_frame
    );
}