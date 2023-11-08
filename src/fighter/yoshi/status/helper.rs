use crate::imports::status_imports::*;

pub unsafe extern "C" fn yoshi_guard_exec_helper(fighter: &mut L2CFighterCommon) {
    let shield_scale = WorkModule::get_param_float(fighter.module_accessor, hash40("shield_radius"), 0);
    ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("throw"), &Vector3f{x: shield_scale, y: shield_scale, z: shield_scale});
    // notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2dc1210b69));
    let shield_hp = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);
    let shield_max = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD_MAX);
    let model_color = (shield_hp / shield_max).clamp(0.2, 1.0);
    ModelModule::set_color_rgb(fighter.module_accessor, model_color, model_color, model_color, MODEL_COLOR_TYPE{_address: 0});
}