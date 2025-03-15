use super::*;

#[skyline::hook(replace = L2CFighterCommon_status_pre_DamageFlyReflectLR)]
unsafe extern "C" fn status_pre_damageflyreflectlr(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_DAMAGE_AIR_REFLECT_LR,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_DAMAGE_FLY_REFLECT_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_DAMAGE_FLY_REFLECT_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_DAMAGE_FLY_REFLECT_FLOAT,
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
        (
            *FIGHTER_STATUS_ATTR_DAMAGE |
            *FIGHTER_STATUS_ATTR_DISABLE_SHIELD_RECOVERY
        ) as u32,
        0,
        0
    );
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_pre_DamageFlyReflectU)]
unsafe extern "C" fn status_pre_damageflyreflectu(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_DAMAGE_AIR_REFLECT_U,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_DAMAGE_FLY_REFLECT_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_DAMAGE_FLY_REFLECT_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_DAMAGE_FLY_REFLECT_FLOAT,
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
        (
            *FIGHTER_STATUS_ATTR_DAMAGE |
            *FIGHTER_STATUS_ATTR_DISABLE_SHIELD_RECOVERY
        ) as u32,
        0,
        0
    );
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_pre_DamageFlyReflectD)]
unsafe extern "C" fn status_pre_damageflyreflectd(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_DAMAGE_AIR_REFLECT_D,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_DAMAGE_FLY_REFLECT_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_DAMAGE_FLY_REFLECT_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_DAMAGE_FLY_REFLECT_FLOAT,
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
        (
            *FIGHTER_STATUS_ATTR_DAMAGE |
            *FIGHTER_STATUS_ATTR_DISABLE_SHIELD_RECOVERY
        ) as u32,
        0,
        0
    );
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_pre_DamageFlyReflectJumpBoard)]
unsafe extern "C" fn status_pre_damageflyreflectjumpboard(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_NONE,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_DAMAGE_FLY_REFLECT_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_DAMAGE_FLY_REFLECT_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_DAMAGE_FLY_REFLECT_FLOAT,
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
        (
            *FIGHTER_STATUS_ATTR_DAMAGE |
            *FIGHTER_STATUS_ATTR_DISABLE_SHIELD_RECOVERY
        ) as u32,
        0,
        0
    );
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pre_damageflyreflectlr,
            status_pre_damageflyreflectu,
            status_pre_damageflyreflectd,
            status_pre_damageflyreflectjumpboard
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}