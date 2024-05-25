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

mod slingshot;
mod bullet;
mod clayrocket;

pub fn install() {
    let agent = &mut Agent::new("shizue");
    acmd::install(agent);
    status::install(agent);
    frame::install(agent);
    agent_init::install(agent);
    agent.install();
    
    slingshot::install();
    bullet::install();
    clayrocket::install();
}