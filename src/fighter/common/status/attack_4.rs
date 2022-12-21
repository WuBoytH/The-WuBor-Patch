use {
    smash::{
        lua2cpp::{L2CFighterCommon, *},
        hash40,
        app::lua_bind::*,
        lib::{lua_const::*, L2CAgent, L2CValue}
    },
    smash_script::*,
    custom_cancel::*,
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
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_MOTION,
            0.0,
            0.0
        );
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

#[skyline::hook(replace = L2CFighterCommon_status_AttackS4_Main)]
unsafe fn status_attacks4_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StatusModule::is_changing(fighter.module_accessor) {
        let combo = ComboModule::count(fighter.module_accessor) as i32;
        let s4_combo_max = WorkModule::get_param_int(fighter.module_accessor, hash40("s4_combo_max"), 0);
        if combo < s4_combo_max
        && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0
        && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) {
            fighter.attack_s4_mtrans();
        }
    }
    else {
        fighter.attack_s4_mtrans();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        return true.into();
    }
    if CustomCancelManager::execute_cancel(fighter) {
        return true.into();
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return true.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return true.into();
    }
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_bind_address_call_status_end_AttackS4)]
unsafe fn bind_address_call_status_end_attacks4(fighter: &mut L2CFighterCommon, _agent: &mut L2CAgent) -> L2CValue {
    FGCModule::reset_used_ground_normals(fighter, false);
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_end_AttackS4)]
unsafe fn status_end_attacks4(fighter: &mut L2CFighterCommon) -> L2CValue {
    FGCModule::reset_used_ground_normals(fighter, false);
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_AttackHi4_Main)]
unsafe fn status_attackhi4_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        return true.into();
    }
    if CustomCancelManager::execute_cancel(fighter) {
        return true.into();
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return true.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return true.into();
    }
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_bind_address_call_status_end_AttackHi4)]
unsafe fn bind_address_call_status_end_attackhi4(fighter: &mut L2CFighterCommon, _agent: &mut L2CAgent) -> L2CValue {
    FGCModule::reset_used_ground_normals(fighter, false);
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_end_AttackHi4)]
unsafe fn status_end_attackhi4(fighter: &mut L2CFighterCommon) -> L2CValue {
    FGCModule::reset_used_ground_normals(fighter, false);
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_bind_address_call_status_end_AttackLw4)]
unsafe fn bind_address_call_status_end_attacklw4(fighter: &mut L2CFighterCommon, _agent: &mut L2CAgent) -> L2CValue {
    FGCModule::reset_used_ground_normals(fighter, false);
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_AttackLw4_Main_param)]
unsafe fn status_attacklw4_main_param(fighter: &mut L2CFighterCommon, param_1: L2CValue) {
    if !StatusModule::is_changing(fighter.module_accessor) {
        let combo = ComboModule::count(fighter.module_accessor) as i32;
        let lw4_combo_max = WorkModule::get_param_int(fighter.module_accessor, hash40("lw4_combo_max"), 0);
        if combo < lw4_combo_max
        && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0
        && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) {
            fighter.attack_lw4_mtrans_common(hash40("attack_lw4").into());
        }
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        return;
    }
    if CustomCancelManager::execute_cancel(fighter) {
        return;
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return;
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(param_1, false.into());
    }
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
    physics!(fighter, MA_MSC_CMD_PHYSICS_STOP_CHARGE);
    fighter.pop_lua_stack(1);
    attack_4_reset_ground_normals(fighter);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_end_attackxx4start,
            status_end_attacks4hold, status_end_attackhi4hold, status_end_attacklw4hold,
            status_attacks4_main, bind_address_call_status_end_attacks4, status_end_attacks4,
            status_attackhi4_main, bind_address_call_status_end_attackhi4, status_end_attackhi4,
            status_attacklw4_main_param, bind_address_call_status_end_attacklw4, status_end_attacklw4
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}