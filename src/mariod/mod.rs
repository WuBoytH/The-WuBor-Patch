//use smash::phx::Hash40;
use smash::lua2cpp::L2CAgentBase;
//use smash::app::sv_animcmd;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash_script::*;
//use smash_script::macros;

#[fighter_frame( agent = FIGHTER_KIND_MARIOD )]
unsafe fn mariod_frame(fighter: &mut L2CAgentBase) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);

    // Cancel Frames

    if MotionModule::motion_kind(boma) == smash::hash40("special_air_lw") {
        if MotionModule::frame(boma) >= 57.0 {
            CancelModule::enable_cancel(boma);
        }
    }
}

pub fn install() {
    smash_script::replace_fighter_frames!(
        mariod_frame
    );
}