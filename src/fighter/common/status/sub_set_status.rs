use {
    crate::imports::*,
    super::{
        escape::escape_air_slide::*,
        cliff::cliff_jump1::*
    }
};

#[skyline::hook(replace = L2CFighterCommon_sub_set_status_pre_msc_common_table)]
unsafe extern "C" fn sub_set_status_pre_msc_common_table(fighter: &mut L2CFighterCommon) {
    original!()(fighter);
    fighter.sv_set_status_func(
        FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE.into(),
        LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(),
        &mut *(escape_air_slide_pre as *const () as *mut libc::c_void)
    );
}

#[skyline::hook(replace = L2CFighterCommon_sub_set_init_status_msc_common_table)]
unsafe extern "C" fn sub_set_init_status_msc_common_table(fighter: &mut L2CFighterCommon) {
    original!()(fighter);
    fighter.sv_set_status_func(
        FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE.into(),
        LUA_SCRIPT_STATUS_FUNC_INIT_STATUS.into(),
        &mut *(escape_air_slide_init as *const () as *mut libc::c_void)
    );
}

#[skyline::hook(replace = L2CFighterCommon_sub_set_status_main_msc_common_table)]
unsafe extern "C" fn sub_set_status_main_msc_common_table(fighter: &mut L2CFighterCommon) {
    original!()(fighter);
    fighter.sv_set_status_func(
        FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE.into(),
        LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN.into(),
        &mut *(escape_air_slide_main as *const () as *mut libc::c_void)
    );
}

#[skyline::hook(replace = L2CFighterCommon_sub_set_exec_status_msc_common_table)]
unsafe extern "C" fn sub_set_exec_status_msc_common_table(fighter: &mut L2CFighterCommon) {
    original!()(fighter);
    fighter.sv_set_status_func(
        FIGHTER_STATUS_KIND_CLIFF_JUMP1.into(),
        LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS.into(),
        &mut *(sub_cliff_jump1_uniq_process_exec as *const () as *mut libc::c_void)
    );
}

#[skyline::hook(replace = L2CFighterCommon_sub_set_status_end_msc_common_table)]
unsafe extern "C" fn sub_set_status_end_msc_common_table(fighter: &mut L2CFighterCommon) {
    original!()(fighter);
    fighter.sv_set_status_func(
        FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE.into(),
        LUA_SCRIPT_STATUS_FUNC_STATUS_END.into(),
        &mut *(escape_air_slide_end as *const () as *mut libc::c_void)
    );
}

#[skyline::hook(replace = L2CFighterCommon_sub_set_calc_param_common_table)]
unsafe extern "C" fn sub_set_calc_param_common_table(fighter: &mut L2CFighterCommon) {
    original!()(fighter);
    fighter.sv_set_status_func(
        FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE.into(),
        LUA_SCRIPT_STATUS_FUNC_CALC_PARAM.into(),
        &mut *(escape_air_slide_calc_param as *const () as *mut libc::c_void)
    );
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_set_status_pre_msc_common_table,
            sub_set_init_status_msc_common_table,
            sub_set_status_main_msc_common_table,
            sub_set_exec_status_msc_common_table,
            sub_set_status_end_msc_common_table,
            sub_set_calc_param_common_table
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}