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
        let scale_prev = WorkModule::get_float(module_accessor, *FIGHTER_KOOPA_INSTANCE_WORK_ID_FLOAT_BREATH_SCALE);
        let speed_mul = speed_mul + ((speed_mul_max - speed_mul_min) / speed_max_frame);
        let speed_mul = speed_mul.clamp(speed_mul_min, speed_mul_max);
        let scale = scale_prev + ((scale_max - scale_min) / scale_max_frame);
        let scale = scale.clamp(scale_min, scale_max);
        WorkModule::set_float(module_accessor, speed_mul, *FIGHTER_KOOPA_INSTANCE_WORK_ID_FLOAT_BREATH_SPEED_MUL);
        WorkModule::set_float(module_accessor, scale, *FIGHTER_KOOPA_INSTANCE_WORK_ID_FLOAT_BREATH_SCALE);

        let charge_ratio = (scale - scale_min) / (scale_max - scale_min);
        if charge_ratio >= 0.5 {
            if scale_prev != scale && charge_ratio == 1.0 {
                let bone = if WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
                    Hash40::new("mouth1")
                }
                else {
                    Hash40::new("jaw")
                };
                EffectModule::req_follow(module_accessor, Hash40::new("sys_flash"), bone, &ZERO_VECTOR, &ZERO_VECTOR, 1.0, true, 0, 0, 0, 0, 0, false, false);
                let eff_handle = VarModule::get_int(module_accessor, koopa::instance::int::FIREBALL_CHARGE_EFF_ID) as u32;
                if !EffectModule::is_exist_effect(module_accessor, eff_handle) {
                    EffectModule::kill(module_accessor, eff_handle, true, true);
                    VarModule::set_int(module_accessor, koopa::instance::int::FIREBALL_CHARGE_EFF_ID, 0);
                }
            }

            let eff_handle = VarModule::get_int(module_accessor, koopa::instance::int::FIREBALL_CHARGE_EFF_ID) as u32;
            if !EffectModule::is_exist_effect(module_accessor, eff_handle) {
                let bone = if WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
                    Hash40::new("mouth1")
                }
                else {
                    Hash40::new("jaw")
                };
                let eff_handle = EffectModule::req_follow(
                    module_accessor,
                    Hash40::new("koopa_breath_m_fire"),
                    bone,
                    &Vector3f{x: 0.0, y: 1.0, z: 0.0},
                    &Vector3f{x: 180.0, y: 0.0, z: 50.0},
                    1.0,
                    true,
                    0,
                    0,
                    0,
                    0,
                    0,
                    false,
                    false
                ) as u32;
                let (eff_scale, eff_rate) = if charge_ratio == 1.0 {
                    (&Vector3f{x: 1.1, y: 1.3, z: 1.1}, 0.6)
                }
                else {
                    (&Vector3f{x: 0.7, y: 0.9, z: 0.7}, 1.0)
                };
                EffectModule::set_scale(module_accessor, eff_handle, eff_scale);
                EffectModule::set_rate(module_accessor, eff_handle, eff_rate);
                VarModule::set_int(module_accessor, koopa::instance::int::FIREBALL_CHARGE_EFF_ID, eff_handle as i32);
            }
        }
        else {
            let eff_handle = VarModule::get_int(module_accessor, koopa::instance::int::FIREBALL_CHARGE_EFF_ID) as u32;
            if !EffectModule::is_exist_effect(module_accessor, eff_handle) {
                EffectModule::kill(module_accessor, eff_handle, true, true);
                VarModule::set_int(module_accessor, koopa::instance::int::FIREBALL_CHARGE_EFF_ID, 0);
            }
        }
    }
}

pub fn install() {
    skyline::install_hooks!(
        koopa_per_frame
    );
}