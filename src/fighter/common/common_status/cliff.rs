use {
    smash::{
        lua2cpp::{L2CFighterCommon, *},
        hash40,
        app::lua_bind::*,
        lib::{lua_const::*, L2CValue}
    },
    wubor_utils::table_const::*
};

#[skyline::hook(replace = L2CFighterCommon_get_cliff_wait_hit_xlu_frame)]
unsafe fn get_cliff_wait_hit_xlu_frame(fighter: &mut L2CFighterCommon) -> L2CValue {
    let air_xlu_max_frame = WorkModule::get_param_int(
        fighter.module_accessor, hash40("common"), hash40("cliff_wait_air_xlu_max_frame")
    ) as f32;
    let air_xlu_max_air_frame = WorkModule::get_param_int(
        fighter.module_accessor, hash40("common"), hash40("cliff_wait_air_xlu_max_air_frame")
    ) as f32;
    let damage_xlu_max_frame = WorkModule::get_param_int(
        fighter.module_accessor, hash40("common"), hash40("cliff_wait_damage_xlu_max_frame")
    ) as f32;
    let damage_xlu_max_damage = WorkModule::get_param_int(
        fighter.module_accessor, hash40("common"), hash40("cliff_wait_damage_xlu_max_damage")
    ) as f32;
    let xlu_min_frame = WorkModule::get_param_int(
        fighter.module_accessor, hash40("common"), hash40("cliff_wait_xlu_min_frame")
    ) as f32;

    // Vanilla Ultimate's Ledge Intangibility Formula
    let air_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_IN_AIR) as f32;
    let xlu_from_air = (air_xlu_max_frame * (air_frame / air_xlu_max_air_frame)).clamp(0.0, air_xlu_max_frame);
    // println!("xlu from air: {}", xlu_from_air);
    let damage = DamageModule::damage(fighter.module_accessor, 0);
    let xlu_from_damage = (damage_xlu_max_frame - ((damage / damage_xlu_max_damage) * damage_xlu_max_frame)).clamp(0.0, damage_xlu_max_frame);
    // println!("xlu from damage: {}", xlu_from_damage);
    let xlu_frame = xlu_from_air + xlu_from_damage;
    let cliff_hang_invincible_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("cliff_hang_invincible_frame"), 0);
    // println!("cliff hang invincible frame? {}", cliff_hang_invincible_frame);
    // println!("final xlu: {}", (xlu_frame * cliff_hang_invincible_frame).clamp(xlu_min_frame, f32::MAX));
    ((xlu_frame * cliff_hang_invincible_frame).clamp(xlu_min_frame, f32::MAX)).into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_cliff_uniq_process_exit_Common)]
unsafe fn sub_cliff_uniq_process_exit_common(fighter: &mut L2CFighterCommon, param_1: L2CValue) {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_CLIFF) {
        let cliff_no_catch_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("cliff_no_catch_frame"));
        WorkModule::set_int(fighter.module_accessor, cliff_no_catch_frame, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_NO_CATCH_FRAME);
        if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_STATUS_KIND_FALL {
            HitModule::set_xlu_frame_global(fighter.module_accessor, 0, 0);
        }
    }
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_CLIFF);
    if param_1.get_bool() {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_CLIFF);
        GroundModule::leave_cliff(fighter.module_accessor);
        if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_STATUS_KIND_FALL {
            HitModule::set_xlu_frame_global(fighter.module_accessor, 0, 0);
        }
    }
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            get_cliff_wait_hit_xlu_frame,
            sub_cliff_uniq_process_exit_common
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}