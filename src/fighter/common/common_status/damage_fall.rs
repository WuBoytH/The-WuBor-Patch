use {
    smash::{
        lua2cpp::*,
        phx::Vector3f,
        app::lua_bind::*,
        lib::{lua_const::*, *}
    },
    wubor_utils::table_const::*
};

#[skyline::hook(replace = L2CFighterCommon_bind_address_call_status_DamageFall)]
unsafe fn bind_address_call_status_damagefall(fighter: &mut L2CFighterCommon, _agent: &mut L2CAgent) -> L2CValue {
    fighter.sub_DamageFall_common();
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_AIR) {
        KineticModule::add_speed(fighter.module_accessor, &Vector3f {x: 1.25, y: 0.0, z: 0.0});
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_status_DamageFall_Main as *const () as _))
}

#[skyline::hook(replace = L2CFighterCommon_status_DamageFall)]
unsafe fn status_damagefall(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_DamageFall_common();
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_AIR) {
        KineticModule::add_speed(fighter.module_accessor, &Vector3f {x: 1.25, y: 0.0, z: 0.0});
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_status_DamageFall_Main as *const () as _))
}

#[skyline::hook(replace = L2CFighterCommon_status_DamageFall_Main)]
unsafe fn status_damagefall_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 0.into();
    }
    if fighter.sub_AirChkPassive().get_bool()
    || fighter.sub_AirChkPassiveWall().get_bool()
    || fighter.sub_AirChkPassiveWallJump().get_bool() {
        return 0.into();
    }
    if fighter.check_damage_fall_transition().get_bool() {
        return 0.into();
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DOWN)
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_DOWN.into(), true.into());
        return 0.into();
    }
    fighter.sub_damage_fall_uniq_process_exec_fix_pos();
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            bind_address_call_status_damagefall,
            status_damagefall,
            status_damagefall_main
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}