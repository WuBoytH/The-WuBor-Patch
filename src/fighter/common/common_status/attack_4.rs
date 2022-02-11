#![allow(unused_must_use)]

use {
    smash::{
        lua2cpp::{L2CFighterCommon, *},
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CAgent, L2CValue}
    },
    smash_script::*,
    wubor_utils::{
        wua_bind::*,
        table_const::*
    }
};

#[skyline::hook(replace = L2CFighterCommon_status_end_AttackXX4Start)]
unsafe fn status_end_attackxx4start(fighter: &mut L2CFighterCommon) {
    let restart_frame = MotionModule::frame(fighter.module_accessor);
    WorkModule::set_float(fighter.module_accessor, restart_frame, *FIGHTER_STATUS_ATTACK_WORK_FLOAT_SMASH_RESTART_FRAME);
    if fighter.global_table[STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD {
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.0, 0.0);
        sv_kinetic_energy::set_speed(fighter.lua_state_agent);
    }
    attack_4_reset_ground_normals(fighter);
}

#[skyline::hook(replace = L2CFighterCommon_status_end_AttackS4Hold)]
unsafe fn status_end_attacks4hold(fighter: &mut L2CFighterCommon) -> L2CValue {
    attack_4_hold(fighter);
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_end_AttackHi4Hold)]
unsafe fn status_end_attackhi4hold(fighter: &mut L2CFighterCommon) -> L2CValue {
    attack_4_hold(fighter);
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_end_AttackLw4Hold)]
unsafe fn status_end_attacklw4hold(fighter: &mut L2CFighterCommon) -> L2CValue {
    attack_4_hold(fighter);
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_bind_address_call_status_end_AttackS4)]
unsafe fn bind_address_call_status_end_attacks4(fighter: &mut L2CFighterCommon, _agent: &mut L2CAgent) -> L2CValue {
    FGCModule::reset_used_ground_normals(fighter, false);
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_bind_address_call_status_end_AttackHi4)]
unsafe fn bind_address_call_status_end_attackhi4(fighter: &mut L2CFighterCommon, _agent: &mut L2CAgent) -> L2CValue {
    FGCModule::reset_used_ground_normals(fighter, false);
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_bind_address_call_status_end_AttackLw4)]
unsafe fn bind_address_call_status_end_attacklw4(fighter: &mut L2CFighterCommon, _agent: &mut L2CAgent) -> L2CValue {
    FGCModule::reset_used_ground_normals(fighter, false);
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_end_AttackS4)]
unsafe fn status_end_attacks4(fighter: &mut L2CFighterCommon) -> L2CValue {
    FGCModule::reset_used_ground_normals(fighter, false);
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_end_AttackHi4)]
unsafe fn status_end_attackhi4(fighter: &mut L2CFighterCommon) -> L2CValue {
    FGCModule::reset_used_ground_normals(fighter, false);
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_end_AttackLw4)]
unsafe fn status_end_attacklw4(fighter: &mut L2CFighterCommon) -> L2CValue {
    FGCModule::reset_used_ground_normals(fighter, false);
    0.into()
}

unsafe fn attack_4_reset_ground_normals(fighter: &mut L2CFighterCommon) {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if ![
        *FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_S4,
        *FIGHTER_STATUS_KIND_ATTACK_HI4_START, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_HI4,
        *FIGHTER_STATUS_KIND_ATTACK_LW4_START, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_LW4
    ].contains(&status) {
        FGCModule::reset_used_ground_normals(fighter, true);
    }
}

unsafe fn attack_4_hold(fighter: &mut L2CFighterCommon) {
    fighter.clear_lua_stack();
    lua_args!(fighter, MA_MSC_CMD_PHYSICS_STOP_CHARGE);
    sv_module_access::physics(fighter.lua_state_agent);
    fighter.pop_lua_stack(1);
    attack_4_reset_ground_normals(fighter);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_end_attackxx4start,
            status_end_attacks4hold, status_end_attackhi4hold, status_end_attacklw4hold,
            bind_address_call_status_end_attacks4, bind_address_call_status_end_attackhi4, bind_address_call_status_end_attacklw4,
            status_end_attacks4, status_end_attackhi4, status_end_attacklw4
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}