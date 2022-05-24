use {
    smash::{
        lua2cpp::{L2CFighterCommon, *},
        app::lua_bind::*,
        lib::{lua_const::*, L2CValue}
    },
    wubor_utils::wua_bind::*
};

#[skyline::hook(replace = L2CFighterCommon_status_end_AttackS3)]
unsafe fn status_end_attacks3(fighter: &mut L2CFighterCommon) -> L2CValue {
    FGCModule::reset_used_ground_normals(fighter, false);
    let attack_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    if 0 < attack_kind {
        FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, attack_kind);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    }
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_end_AttackHi3)]
unsafe fn status_end_attackhi3(fighter: &mut L2CFighterCommon) -> L2CValue {
    FGCModule::reset_used_ground_normals(fighter, false);
    let attack_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    if 0 < attack_kind {
        FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, attack_kind);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    }
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_end_AttackLw3)]
unsafe fn status_end_attacklw3(fighter: &mut L2CFighterCommon) -> L2CValue {
    FGCModule::reset_used_ground_normals(fighter, false);
    let attack_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    if 0 < attack_kind {
        FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, attack_kind);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    }
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_end_attacks3,
            status_end_attackhi3,
            status_end_attacklw3
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}