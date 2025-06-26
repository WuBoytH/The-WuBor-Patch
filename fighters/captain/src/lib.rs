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

#[no_mangle]
pub unsafe extern "C" fn captain_check_boost_power(module_accessor: *mut BattleObjectModuleAccessor, damage: f32) {
    if !VarModule::is_flag(module_accessor, vars::captain::instance::flag::HAS_BOOST_POWER) {
        VarModule::add_float(module_accessor, vars::captain::instance::float::BOOST_POWER, damage);
        if VarModule::get_float(module_accessor, vars::captain::instance::float::BOOST_POWER) <= 0.0 {
            VarModule::set_float(module_accessor, vars::captain::instance::float::BOOST_POWER, 0.0);
        }
        if VarModule::get_float(module_accessor, vars::captain::instance::float::BOOST_POWER) >= 50.0 {
            VarModule::set_float(module_accessor, vars::captain::instance::float::BOOST_POWER, 0.0);
            SoundModule::play_se_no3d(
                module_accessor,
                Hash40::new("se_captain_boostpower"),
                false,
                false
            );
            VarModule::on_flag(module_accessor, vars::captain::instance::flag::HAS_BOOST_POWER);
        }
    }
}

pub fn install() {
    let agent = &mut Agent::new("captain");
    acmd::install(agent);
    status::install(agent);
    frame::install(agent);
    agent.install();
}