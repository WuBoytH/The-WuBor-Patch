use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        app::lua_bind::*,
        lib::lua_const::*
    },
    smashline::*,
    crate::{
        common_funcs::*,
        vars::*
    }
};

// Turn this into a status check instead

#[fighter_frame( agent = FIGHTER_KIND_FALCO )]
fn falco_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("appeal_lw_l")
        || MotionModule::motion_kind(fighter.module_accessor) == hash40("appeal_lw_r") {
            KAA[entry_id(fighter.module_accessor)] = true;
        }
        else if MotionModule::motion_kind(fighter.module_accessor) != hash40("attack_lw4")
        && MotionModule::motion_kind(fighter.module_accessor) != hash40("appeal_lw_l")
        && MotionModule::motion_kind(fighter.module_accessor) != hash40("appeal_lw_r")
        && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_SQUAT {
            KAA[entry_id(fighter.module_accessor)] = false;
        }
    }
}

pub fn install() {
    install_agent_frames!(
        falco_frame
    );
}