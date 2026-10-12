use super::*;
use wubor_utils::offsets;

#[skyline::from_offset(offsets::fighter::wario::VTABLE_FUNC)]
unsafe extern "C" fn wario_vtable_func_inner(vtable: u64, fighter: &mut Fighter);

pub unsafe extern "C" fn wario_vtable_func(vtable: u64, fighter: &mut Fighter) {
    let module_accessor = (fighter.battle_object).module_accessor;
    let battle_object_slow = singletons::BattleObjectSlow() as *mut u8;
    if (*battle_object_slow.add(0x8) == 0 || *(battle_object_slow as *const u32) == 0)
    && !StopModule::is_stop(module_accessor) && !WorkModule::is_flag(module_accessor, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_CHARGE_STOP) {
        let gas_count = WorkModule::get_int(module_accessor, 0x100000bf); // FIGHTER_WARIO_INSTANCE_WORK_ID_INT_GASS_COUNT
        let gas_level;
        let gas_max = WorkModule::get_param_float(module_accessor, hash40("param_special_lw"), hash40("gass_max_time")) as i32;
        let gas_large = WorkModule::get_param_float(module_accessor, hash40("param_special_lw"), hash40("gass_large_time")) as i32;
        let gas_middle = WorkModule::get_param_float(module_accessor, hash40("param_special_lw"), hash40("gass_middle_time")) as i32;
        if gas_count >= gas_max {
            gas_level = *FIGHTER_WARIO_GASS_LEVEL_FLY;
            if !WorkModule::is_flag(module_accessor, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_CHARGE_MAX) {
                WorkModule::on_flag(module_accessor, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_CHARGE_MAX);
                EffectModule::req_common(module_accessor, Hash40::new("wario_charge_max"), 0.0);
                EffectModule::req_follow(
                    module_accessor,
                    Hash40::new("wario_ppe_flash"),
                    Hash40::new("bust"),
                    &vars::ZERO_VECTOR,
                    &vars::ZERO_VECTOR,
                    1.0,
                    false,
                    0,
                    0,
                    -1,
                    0,
                    0,
                    false,
                    false
                );
            }
        }
        else if gas_count >= gas_large {
            gas_level = *FIGHTER_WARIO_GASS_LEVEL_L;
        }
        else if gas_count >= gas_middle {
            gas_level = *FIGHTER_WARIO_GASS_LEVEL_M;
        }
        else { gas_level = 0; } // FIGHTER_WARIO_GASS_LEVEL_S

        WorkModule::set_int(module_accessor, gas_level, *FIGHTER_WARIO_INSTANCE_WORK_ID_INT_GASS_LEVEL);
        WorkModule::set_float(module_accessor, (gas_count as f32 * 100.0) / gas_max as f32, 0x4F); // FIGHTER_WARIO_INSTANCE_WORK_ID_FLOAT_STOMACH_MOTION_FRAME
        if !LinkModule::is_link(module_accessor, 0xd) {
            let bike_time = WorkModule::get_int(module_accessor, 0x100000C1); // FIGHTER_WARIO_INSTANCE_WORK_ID_INT_BIKE_TIME
            let interval_time = WorkModule::get_param_int(module_accessor, hash40("param_special_s"), hash40("interval_time"));
            if bike_time == interval_time {
                EffectModule::req_follow(
                    module_accessor,
                    Hash40::new_raw(0x10e3efcdd6),
                    Hash40::new("head"),
                    &vars::ZERO_VECTOR,
                    &vars::ZERO_VECTOR,
                    1.0,
                    false,
                    0,
                    0,
                    -1,
                    0,
                    0,
                    false,
                    false
                );
            }
            WorkModule::inc_int(module_accessor, 0x100000C1); // FIGHTER_WARIO_INSTANCE_WORK_ID_INT_BIKE_TIME
        }
        if WorkModule::get_int(module_accessor, *FIGHTER_WARIO_INSTANCE_WORK_ID_INT_SPECIAL_N_INTERVAL) != 0 {
            WorkModule::dec_int(module_accessor, *FIGHTER_WARIO_INSTANCE_WORK_ID_INT_SPECIAL_N_INTERVAL);
        }
    }

    let stop = WorkModule::is_flag(module_accessor, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_CHARGE_STOP);
    WorkModule::on_flag(module_accessor, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_CHARGE_STOP);
    wario_vtable_func_inner(vtable, fighter);
    WorkModule::set_flag(module_accessor, stop, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_CHARGE_STOP);
}

pub fn install() {
    replace_fighter_vtable(
        *FIGHTER_KIND_WARIO,
        13,
        wario_vtable_func as *const () as _
    );
}
