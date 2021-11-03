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

#[fighter_frame( agent = FIGHTER_KIND_RIDLEY )]
fn ridley_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        if IS_FUNNY[entry_id(fighter.module_accessor)] {
            FUNNY_RIDLEY[entry_id(fighter.module_accessor)] = true;
        }
        else if MotionModule::motion_kind(fighter.module_accessor) != hash40("special_n_shoot")
        && MotionModule::motion_kind(fighter.module_accessor) != hash40("special_air_n_shoot") {
            FUNNY_RIDLEY[entry_id(fighter.module_accessor)] = false;
        }
    }
}

pub fn install() {
    install_agent_frames!(
        ridley_frame
    );
}