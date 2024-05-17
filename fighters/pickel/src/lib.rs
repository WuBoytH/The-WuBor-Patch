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

mod forge;
mod trolley;

pub fn install() {
    let agent = &mut smashline::Agent::new("pickel");
    acmd::install(agent);
    status::install(agent);
    frame::install(agent);
    agent.install();

    forge::install();
    trolley::install();
}