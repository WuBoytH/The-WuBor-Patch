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
pub mod vl;

mod bowarrow;
mod boomerang;

pub fn install() {
    let agent = &mut Agent::new("toonlink");
    acmd::install(agent);
    frame::install(agent);
    status::install(agent);
    agent.install();

    bowarrow::install();
    boomerang::install();
}