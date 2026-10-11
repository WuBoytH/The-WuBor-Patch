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
pub mod status;
mod frame;
mod agent_init;
pub mod vl;

mod sword;
mod wave;

pub fn install() {
    unsafe {
        let wave = clone_weapon("ike", "wave", *WEAPON_KIND_DOLLY_WAVE, true);
        vars::ike::generate::WAVE = wave.generate_id;
        vars::ike::weapon_id::WAVE = wave.weapon_id;
        add_param_object("ike", "map_collision_wave");
    }

    let agent = &mut Agent::new("ike");
    acmd::install(agent);
    status::install(agent);
    frame::install(agent);
    agent_init::install(agent);
    agent.install();

    sword::install();
    wave::install();
}