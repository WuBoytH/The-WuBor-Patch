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
mod status;
mod frame;
mod agent_init;
pub mod helper;
mod fgc;

mod auraball;
mod qigong;

pub fn install() {
    let agent = &mut Agent::new("lucario");
    acmd::install(agent);
    status::install(agent);
    frame::install(agent);
    agent_init::install(agent);
    fgc::install();
    smashline::add_param_object("lucario", "param_auracharge");
    agent.install();

    auraball::install();
    qigong::install();
}