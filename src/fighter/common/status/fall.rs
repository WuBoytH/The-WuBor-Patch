use crate::imports::*;

#[skyline::hook(replace = L2CFighterCommon_bind_address_call_status_pre_Fall)]
unsafe extern "C" fn bind_address_call_status_pre_fall(fighter: &mut L2CFighterCommon, _agent: &mut L2CAgent) -> L2CValue {
    fighter.status_pre_Fall()
}

#[skyline::hook(replace = L2CFighterCommon_status_pre_Fall)]
unsafe extern "C" fn status_pre_fall(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_pre_fall().get_bool() {
        return 1.into();
    }
    let jump_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    let jump_count_max = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
    if jump_count_max <= jump_count {
        StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL_AERIAL);
        return 1.into();
    }
    fighter.status_pre_Fall_main()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            bind_address_call_status_pre_fall,
            status_pre_fall
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}