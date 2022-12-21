use smash::app::sv_fighter_util;

use {
    smash::{
        lua2cpp::{L2CFighterCommon, *},
        hash40,
        app::lua_bind::*,
        lib::lua_const::*
    },
    smash_script::*,
    custom_var::*,
    wubor_utils::{vars::*, table_const::*}
};

#[skyline::hook(replace = L2CFighterCommon_sub_fighter_pre_end_status)]
unsafe fn sub_fighter_pre_end_status(fighter: &mut L2CFighterCommon) {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) {
        let some = fighter.global_table[0x24].get_f32();
        if some <= 2.0 {
            let status = fighter.global_table[STATUS_KIND].get_i32();
            fighter.clear_lua_stack();
            lua_args!(fighter, status);
            if sv_fighter_util::is_attack_air_status(fighter.lua_state_agent, status)
            || [
                *FIGHTER_STATUS_KIND_SPECIAL_N,
                *FIGHTER_STATUS_KIND_SPECIAL_S,
                *FIGHTER_STATUS_KIND_SPECIAL_HI,
                *FIGHTER_STATUS_KIND_SPECIAL_LW
            ].contains(&status) {
                let air_speed_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
                sv_kinetic_energy!(
                    set_speed,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                    -air_speed_y_stable
                );
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
            }
        }
    }

    // <WuBor>
    // Handles removing ledge intangibility when not falling or double jumping.
    // This makes a big assumption where if you ledge drop, you will have had ledge intangibility.
    // May cause unintended side effects but the logic should be pretty air-tight.

    if VarModule::is_flag(fighter.battle_object, commons::instance::flag::LEDGE_INTANGIBILITY) {
        let status = fighter.global_table[STATUS_KIND].get_i32();
        if ![
            *FIGHTER_STATUS_KIND_FALL,
            *FIGHTER_STATUS_KIND_JUMP_AERIAL,
            *FIGHTER_STATUS_KIND_FALL_AERIAL,
            *FIGHTER_STATUS_KIND_FLY
        ].contains(&status) {
            VarModule::off_flag(fighter.battle_object, commons::instance::flag::LEDGE_INTANGIBILITY);
            HitModule::set_xlu_frame_global(fighter.module_accessor, 0, 0);
        }
    }
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_fighter_pre_end_status
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}