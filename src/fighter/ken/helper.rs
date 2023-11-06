use {
    smash::{
        hash40,
        app::{lua_bind::*, *},
        lib::lua_const::*
    },
    custom_var::*,
    wubor_utils::{wua_bind::*, vars::*,}
};

pub unsafe extern "C" fn add_vgauge(module_accessor: *mut BattleObjectModuleAccessor, mut amount: f32) {
    let meter_max = 900.0;
    let meter_const = ken::instance::float::V_GAUGE;
    if MotionModule::motion_kind(module_accessor) != hash40("special_lw")
    && !VarModule::is_flag(module_accessor, ken::instance::flag::V_TRIGGER) {
        if MotionModule::motion_kind(module_accessor) == hash40("attack_s3_s_w")
        && StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_LW {
            FGCModule::update_meter(module_accessor, 100.0, meter_max, meter_const);
        }
        else {
            amount *= 5.0;
            FGCModule::update_meter(module_accessor, amount, meter_max, meter_const);
        }
    }
}
