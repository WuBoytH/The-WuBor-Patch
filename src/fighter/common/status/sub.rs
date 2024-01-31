use crate::imports::status_imports::*;
use wubor_utils::controls::*;

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_is_dive)]
pub unsafe extern "C" fn sub_is_dive(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) {
        return false.into();
    }

    let status_kind = fighter.global_table[STATUS_KIND_INTERRUPT].get_i32();
    let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();

    // if status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR
    // && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {
    //     return false.into();
    // }

    if [*FIGHTER_STATUS_KIND_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
        *FIGHTER_STATUS_KIND_SAVING_DAMAGE_FLY,
    ].contains(&status_kind) {
        return false.into();
    }

    let cliff_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_COUNT);
    let cliff_dive_count_max = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), 0x189f0b0c96);
    if cliff_dive_count_max < cliff_count {
        return false.into();
    }

    if !KineticModule::is_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL) {
        return false.into();
    }

    if KineticModule::is_suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL) {
        return false.into();
    }

    let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if speed_y >= 0.0 {
        return false.into();
    }

    let mut dive_cont_value = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("dive_cont_value"));
    let mut dive_flick_frame_value = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("dive_flick_frame_value"));
    if [*FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE,
        *FIGHTER_STATUS_KIND_CLIFF_CATCH,
        *FIGHTER_STATUS_KIND_CLIFF_WAIT,
    ].contains(&prev_status_kind) {
        dive_cont_value = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("cliff_dive_cont_value"));
        dive_flick_frame_value = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("cliff_dive_flick_frame_value"));
    }

    let left_stick_y = if Buttons::from_bits_retain(ControlModule::get_button(fighter.module_accessor)).intersects(Buttons::CStickOverride) {
        ControlModule::get_sub_stick_y(fighter.module_accessor)
    }
    else {
        ControlModule::get_stick_y(fighter.module_accessor)
    };

    if left_stick_y > dive_cont_value
    || fighter.global_table[FLICK_Y].get_i32() >= dive_flick_frame_value
    || fighter.global_table[FLICK_Y_DIR].get_i32() >= 0 {
        return false.into();
    }

    let dive_speed_y = WorkModule::get_param_float(fighter.module_accessor, hash40("dive_speed_y"), 0);
    (speed_y >= -dive_speed_y).into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_is_dive
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}