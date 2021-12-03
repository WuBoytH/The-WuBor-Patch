use {
    smash::{
        app::{lua_bind::*, *}
    },
    crate::vars::*
};

#[inline(always)]
pub unsafe fn add_vgauge(module_accessor: *mut BattleObjectModuleAccessor, amount: f32) {
    let mut v_gauge = WorkModule::get_float(module_accessor, FIGHTER_KEN_INSTANCE_WORK_ID_FLOAT_V_GAUGE);
    v_gauge += amount;
    if v_gauge < 0.0 {
        v_gauge = 0.0;
    }
    if v_gauge > 900.0 {
        v_gauge = 900.0;
    }
    WorkModule::set_float(module_accessor, v_gauge, FIGHTER_KEN_INSTANCE_WORK_ID_FLOAT_V_GAUGE);
}
