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
    let boost_power_current = VarModule::get_float(module_accessor, vars::captain::instance::float::BOOST_POWER);
    let boost_power_new = (boost_power_current + damage).clamp(0.0, vars::captain::BOOST_POWER_MAX);
    VarModule::set_float(module_accessor, vars::captain::instance::float::BOOST_POWER, boost_power_new);
    if (boost_power_current / vars::captain::BOOST_POWER_THRESHOLD) as i32
    != (boost_power_new / vars::captain::BOOST_POWER_THRESHOLD) as i32 {
        if boost_power_new > boost_power_current {
            SoundModule::play_se(module_accessor, Hash40::new("se_captain_boostpower"), true, false, false, false, enSEType(0));
        }
        captain_update_boost_power(module_accessor);
    }
}

#[no_mangle]
pub unsafe extern "C" fn captain_update_boost_power(module_accessor: *mut BattleObjectModuleAccessor) {
    let boost_power = VarModule::get_float(module_accessor, vars::captain::instance::float::BOOST_POWER);
    let level = (boost_power / vars::captain::BOOST_POWER_THRESHOLD) as i32;
    match level {
        0 => MiscModule::disable_lightweight(module_accessor),
        _ => {
            let mul = 1.0 + (0.1 * level as f32);

            let mut changes = Vec::new();
            changes.push(StatChange::new(hash40("dash_speed"), mul));
            changes.push(StatChange::new(hash40("run_speed_max"), mul));
            // changes.push(StatChange::new(hash40("ground_brake"), mul));
            changes.push(StatChange::new(hash40("air_speed_x_stable"), mul));
            // changes.push(StatChange::new(hash40("air_brake_x"), mul));
            changes.push(StatChange::new(hash40("air_speed_y_stable"), mul));
            changes.push(StatChange::new(hash40("dive_speed_y"), mul));
            MiscModule::set_lightweight(module_accessor, changes);
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