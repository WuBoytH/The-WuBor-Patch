use {
    smash::{
        hash40,
        app::{lua_bind::*, *},
        lib::lua_const::*
    },
    wubor_utils::wua_bind::*,
    super::vars::*,
};

pub unsafe fn add_vgauge(object: *mut BattleObject, module_accessor: *mut BattleObjectModuleAccessor, mut amount: f32) {
    let meter_max = 900.0;
    let meter_const = FIGHTER_KEN_INSTANCE_WORK_ID_FLOAT_V_GAUGE;
    if MotionModule::motion_kind(module_accessor) != hash40("special_lw")
    && !WorkModule::is_flag(module_accessor, FIGHTER_KEN_INSTANCE_WORK_ID_FLAG_V_TRIGGER) {
        if MotionModule::motion_kind(module_accessor) == hash40("attack_s3_s_w")
        && StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_LW {
            FGCModule::update_meter(object, 100.0, meter_max, meter_const);
        }
        else {
            amount *= 5.0;
            FGCModule::update_meter(object, amount, meter_max, meter_const);
        }
    }
}
