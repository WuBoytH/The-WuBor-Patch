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

#[fighter_frame( agent = FIGHTER_KIND_FALCO )]
fn falco_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        
    }
}

pub fn install() {
    install_agent_frames!(
        falco_frame
    );
}