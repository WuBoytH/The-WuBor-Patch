#![feature(repr_simd)]
#![allow(
    unused_macros,
    unused_must_use
)]

use {
    smash::{
        lua2cpp::*,
        hash40,
        phx::*,
        app::{lua_bind::*, sv_animcmd::*, *},
        lib::{lua_const::*, L2CValue, L2CAgent}
    },
    smash_script::*,
    custom_var::*,
    custom_cancel::*,
    wubor_utils::{cancels::*, wua_bind::*, vars, table_const::*}
};

pub mod frame;
pub mod status;
pub mod agent_inits;
pub mod param;
// pub mod command_inputs;
mod energy;

pub fn install() {
    status::install();
    energy::install();
}