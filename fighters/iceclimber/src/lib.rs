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

pub fn install() {
    let popo = &mut smashline::Agent::new("popo");
    let nana = &mut smashline::Agent::new("nana");
    acmd::install(popo);
    acmd::install(nana);
    frame::install(popo);
    frame::install(nana);
    popo.install();
    nana.install();
}