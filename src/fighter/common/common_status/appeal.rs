#![allow(unused_must_use)]

use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    }
};

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_pre_Appeal_common)]
unsafe fn status_pre_appeal_common(fighter: &mut L2CFighterCommon, param_1: L2CValue) {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_MOTION,
        *GROUND_CORRECT_KIND_GROUND as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_APPEAL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_APPEAL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_APPEAL_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_ENABLE,
        false,
        false,
        false,
        0,
        0,
        param_1.get_u32(),
        0
    );
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pre_appeal_common
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}