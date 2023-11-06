use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{lua_bind::*, *},
        lib::lua_const::*
    },
    smashline::*,
    custom_var::*,
    wubor_utils::{wua_bind::*, vars::*, table_const::*},
    crate::fighter::common::frame::install_common_frame
};

unsafe extern "C" fn template_frame(fighter: &mut L2CFighterCommon) {
    
}

pub fn install(agent : &mut smashline::Agent) {
    agent.on_line(smashline::Main, template_frame);
    install_common_frame(agent);
}