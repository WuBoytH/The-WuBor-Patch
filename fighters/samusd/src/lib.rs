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
pub mod vl;

mod cshot;
mod missile;
mod supermissile;

pub fn install() {
    let agent = &mut Agent::new("samusd");
    acmd::install(agent);
    frame::install(agent);
    status::install(agent);
    agent_init::install(agent);
    agent.install();

    cshot::install();
    missile::install();
    supermissile::install();
}