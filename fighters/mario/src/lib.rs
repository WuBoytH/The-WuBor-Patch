#[allow(unused_imports)]
use {
    smash::{
        lua2cpp::*,
        hash40,
        phx::*,
        app::{lua_bind::*, sv_animcmd::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    smash_script::*,
    custom_var::*,
    custom_cancel::*,
    wubor_utils::{app::*, cancels::*, wua_bind::*, vars, table_const::*}
};

mod acmd;
mod frame;
mod status;
mod agent_init;
pub mod helper;
pub mod vl;

pub fn install() {
    let agent = &mut smashline::Agent::new("mario");
    acmd::install(agent);
    frame::install(agent);
    status::install(agent);
    agent_init::install(agent);
    agent.install();

    wubor_utils::wua_bind::MiscModule::patch_vtable_function(0x51e4630, smash::hash40("trans"));
    wubor_utils::wua_bind::MiscModule::patch_vtable_function(0x51e4638, smash::hash40("trans"));
}