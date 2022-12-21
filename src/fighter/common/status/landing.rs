use {
    smash::{
        lua2cpp::{L2CFighterCommon, *},
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    }
};

#[skyline::hook(replace = L2CFighterCommon_status_pre_Landing_param)]
unsafe fn status_pre_landing_param(fighter: &mut L2CFighterCommon, param_1: L2CValue,  param_2: L2CValue,  param_3: L2CValue,  param_4: L2CValue,  param_5: L2CValue) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        param_4.get_i32(),
        *GROUND_CORRECT_KIND_GROUND as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        param_1.get_i32(),
        param_2.get_i32(),
        param_3.get_i32(),
        param_5.get_i32()
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        true,
        *FIGHTER_TREADED_KIND_ENABLE,
        false,
        false,
        false,
        0,
        *FIGHTER_STATUS_ATTR_INTO_DOOR as u32,
        0,
        0
    );
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_pre_LandingLight_param)]
unsafe fn status_pre_landinglight_param(fighter: &mut L2CFighterCommon, param_1: L2CValue,  param_2: L2CValue,  param_3: L2CValue,  param_4: L2CValue,  param_5: L2CValue) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        param_4.get_i32(),
        *GROUND_CORRECT_KIND_GROUND as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        param_1.get_i32(),
        param_2.get_i32(),
        param_3.get_i32(),
        param_5.get_i32()
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        true,
        *FIGHTER_TREADED_KIND_ENABLE,
        false,
        false,
        false,
        0,
        *FIGHTER_STATUS_ATTR_INTO_DOOR as u32,
        0,
        0
    );
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_pre_landing_fall_special_common)]
unsafe fn status_pre_landing_fall_special_common(fighter: &mut L2CFighterCommon, param_1: L2CValue,  param_2: L2CValue,  param_3: L2CValue) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        fighter.sub_pre_landing_kinetic_type().get_i32(),
        *GROUND_CORRECT_KIND_GROUND as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        param_1.get_i32(),
        param_2.get_i32(),
        param_3.get_i32(),
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        true,
        *FIGHTER_TREADED_KIND_ENABLE,
        false,
        false,
        false,
        0,
        *FIGHTER_STATUS_ATTR_INTO_DOOR as u32,
        0,
        0
    );
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pre_landing_param,
            status_pre_landinglight_param,
            status_pre_landing_fall_special_common
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}