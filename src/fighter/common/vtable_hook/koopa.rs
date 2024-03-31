use crate::imports::*;

#[skyline::hook(offset = 0xbc2290)]
pub unsafe extern "C" fn koopa_per_frame(_vtable: u64, fighter: &mut Fighter) {
    let module_accessor = (fighter.battle_object).module_accessor;
    let battle_object_slow = singletons::BattleObjectSlow() as *mut u8;
    if (*battle_object_slow.add(0x8) == 0 || *(battle_object_slow as *const u32) == 0)
    && !StopModule::is_stop(module_accessor) {
        let status = StatusModule::status_kind(module_accessor);
        let is_kirby = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY
        && status == *FIGHTER_KIRBY_STATUS_KIND_KOOPA_SPECIAL_N;
        if is_kirby || status == *FIGHTER_STATUS_KIND_SPECIAL_N {
            return;
        }
        let speed_mul_max = WorkModule::get_param_float(module_accessor, hash40("param_special_n"), hash40("fire_speed_mul_max"));
        let speed_mul_min = WorkModule::get_param_float(module_accessor, hash40("param_special_n"), hash40("fire_speed_mul_min"));
        let scale_max = WorkModule::get_param_float(module_accessor, hash40("param_special_n"), hash40("fire_scale_max"));
        let scale_min = WorkModule::get_param_float(module_accessor, hash40("param_special_n"), hash40("fire_scale_min"));
        let speed_max_frame = WorkModule::get_param_float(module_accessor, hash40("param_special_n"), hash40("fire_speed_max_frame"));
        let scale_max_frame = WorkModule::get_param_float(module_accessor, hash40("param_special_n"), hash40("fire_scale_max_frame"));
        let speed_mul = WorkModule::get_float(module_accessor, *FIGHTER_KOOPA_INSTANCE_WORK_ID_FLOAT_BREATH_SPEED_MUL);
        let scale = WorkModule::get_float(module_accessor, *FIGHTER_KOOPA_INSTANCE_WORK_ID_FLOAT_BREATH_SCALE);
        let speed_mul = speed_mul + ((speed_mul_max - speed_mul_min) / speed_max_frame);
        let scale = scale + ((scale_max - scale_min) / scale_max_frame);
        WorkModule::set_float(module_accessor, speed_mul.clamp(speed_mul_min, speed_mul_max), *FIGHTER_KOOPA_INSTANCE_WORK_ID_FLOAT_BREATH_SPEED_MUL);
        WorkModule::set_float(module_accessor, scale.clamp(scale_min, scale_max), *FIGHTER_KOOPA_INSTANCE_WORK_ID_FLOAT_BREATH_SCALE);
    }
}

pub fn install() {
    skyline::install_hooks!(
        koopa_per_frame
    );
}