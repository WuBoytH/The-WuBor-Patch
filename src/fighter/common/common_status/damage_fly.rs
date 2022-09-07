use {
    smash::{
        lua2cpp::{L2CFighterCommon, *},
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    wubor_utils::table_const::*
};

#[skyline::hook(replace = L2CFighterCommon_status_pre_DamageFly)]
unsafe fn status_pre_damagefly(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.is_enable_passive().get_bool() {
        StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL);
        return 1.into();
    }
    let mut attr = *FIGHTER_STATUS_ATTR_DAMAGE;
    let mut flag_keep = *FIGHTER_STATUS_WORK_KEEP_FLAG_DAMAGE_FLY_FLAG;
    let prev_status = fighter.global_table[PREV_STATUS_KIND].get_i32();
    if [
        *FIGHTER_STATUS_KIND_BAYONETTA_FINAL_TARGET_END,
        *FIGHTER_STATUS_KIND_DEDEDE_FINAL_TARGET_END,
        *FIGHTER_STATUS_KIND_FALCO_FINAL_TARGET_END,
        *FIGHTER_STATUS_KIND_FOX_FINAL_TARGET_END,
        *FIGHTER_STATUS_KIND_GAOGAEN_FINAL_TARGET_END,
        *FIGHTER_STATUS_KIND_KAMUI_FINAL_TARGET_END,
        *FIGHTER_STATUS_KIND_KROOL_FINAL_TARGET_END,
        *FIGHTER_STATUS_KIND_RIDLEY_FINAL_TARGET_END,
        *FIGHTER_STATUS_KIND_ROCKMAN_FINAL_TARGET_END,
        *FIGHTER_STATUS_KIND_SIMON_FINAL_TARGET_END,
        *FIGHTER_STATUS_KIND_WARIO_FINAL_TARGET_END,
        *FIGHTER_STATUS_KIND_YOSHI_FINAL_TARGET_END,
        *FIGHTER_STATUS_KIND_SHEIK_FINAL_CAPTURE
    ].contains(&prev_status) {
        attr |= *FIGHTER_STATUS_ATTR_DISABLE_ITEM_INTERRUPT;
    }
    if prev_status != *FIGHTER_STATUS_KIND_THROWN {
        flag_keep = *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG;
    }
    else {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_THROWN_WORK_FLAG_DISABLE_PASSIVE) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_FLY_DISABLE_PASSIVE);
        }
    }
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_DAMAGE_FLY,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_DAMAGE_FLY_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_DAMAGE_FLY_FLOAT,
        flag_keep,
        0
    );
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_NO_DROP_ITEM) {
        attr |= *FIGHTER_STATUS_ATTR_NO_DROP_ITEM;
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_NO_DROP_ITEM);
    }
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_ENABLE,
        false,
        false,
        false,
        0,
        attr as u32,
        0,
        0
    );
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pre_damagefly
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}