#![allow(unused_must_use)]

use {
    smash::{
        lua2cpp::{L2CFighterCommon, *},
        phx::Hash40,
        app::lua_bind::*,
        lib::{lua_const::*, L2CAgent, L2CValue}
    },
    wubor_utils::{
        wua_bind::*,
        vars::*,
        table_const::*
    }
};

#[skyline::hook(replace = L2CFighterCommon_sub_attack_air_common)]
unsafe fn sub_attack_air_common(fighter: &mut L2CFighterCommon, param_1: L2CValue) {
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_AIR_ATTACK_WHIFF);
    ControlModule::reset_trigger(fighter.module_accessor);
    ControlModule::reset_flick_y(fighter.module_accessor);
    ControlModule::reset_flick_sub_y(fighter.module_accessor);
    fighter.global_table[FLICK_Y].assign(&0xfe.into());
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.attack_air_uniq(false.into());
    }
    let bind_address_call_attack_air_uniq = L2CFighterCommon_bind_address_call_attack_air_uniq;
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(bind_address_call_attack_air_uniq as *const () as _));
    if param_1.get_bool() == true {
        fighter.sub_attack_air_kind();
    }
    ControlModule::reset_attack_air_kind(fighter.module_accessor);
}

#[skyline::hook(replace = L2CFighterCommon_status_AttackAir_Main_common)]
unsafe fn status_attackair_main_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
    || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_AIR_ATTACK_WHIFF);
    }
    else if AttackModule::is_attack(fighter.module_accessor, 0, false) {
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_AIR_ATTACK_WHIFF);
    }
    if fighter.attack_air_common_strans().get_bool() == false {
        if CancelModule::is_enable_cancel(fighter.module_accessor) == false {
            if MotionModule::is_end(fighter.module_accessor) == false {
                return false.into();
            }
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            if fighter.sub_wait_ground_check_common(false.into()).get_bool() == false {
                if fighter.sub_air_check_fall_common().get_bool() {
                    return true.into();
                }
                if MotionModule::is_end(fighter.module_accessor) == false {
                    return false.into();
                }
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
        }
    }
    true.into()
}

#[skyline::hook(replace = L2CFighterCommon_bind_address_call_status_end_AttackAir)]
unsafe fn status_end_attackair(fighter: &mut L2CFighterCommon, _agent: &mut L2CAgent) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_STATUS_KIND_ATTACK_AIR {
        FGCModule::reset_used_aerials(fighter);
    }
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_landing_attack_air_init)]
unsafe fn sub_landing_attack_air_init(fighter: &mut L2CFighterCommon, param_1: L2CValue, param_2: L2CValue, param_3: L2CValue) {
    let mot = param_1.get_int();
    let mut landing_lag = WorkModule::get_param_float(fighter.module_accessor, param_2.get_int(), 0) + param_3.get_f32();
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_AIR_ATTACK_WHIFF) {
        landing_lag += 4.0;
    }
    let mut motion_rate = 1.0;
    if landing_lag != 0.0 {
        motion_rate = fighter.sub_get_landing_motion_rate(mot.into(), landing_lag.into()).get_f32();
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_LANDING_CANCEL);
    }
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new_raw(mot),
        0.0,
        motion_rate,
        false,
        0.0,
        false,
        false
    );
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_attack_air_common,
            status_attackair_main_common,
            status_end_attackair,
            sub_landing_attack_air_init
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}