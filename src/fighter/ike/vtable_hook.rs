use {
    smash::{
        hash40,
        app::{lua_bind::*, *},
        lib::lua_const::*
    },
    custom_var::*,
    wubor_utils::{wua_bind::*, vars::*},
};

#[skyline::hook(offset = 0xaf9330)]
pub unsafe extern "C" fn ike_critical_zoom(_vtable: u64, fighter: &mut Fighter, unk: u64) {
    let module_accessor = fighter.battle_object.module_accessor;
    let status;
    let param_hash;
    if WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        status = *FIGHTER_KIRBY_STATUS_KIND_IKE_SPECIAL_N_END;
        param_hash = hash40("param_special_n_kirby");
    }
    else {
        status = *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_END;
        param_hash = hash40("param_special_n");
    }
    let object = &mut fighter.battle_object;
    if StatusModule::status_kind(module_accessor) == status
    && VarModule::is_flag(object, ike::status::flag::SPECIAL_N_ENABLE_CRITICAL) {
        VarModule::off_flag(object, ike::status::flag::SPECIAL_N_ENABLE_CRITICAL);
        MiscModule::call_critical(module_accessor, unk, 0x23, param_hash, 1, 0, 0, 0);
    }
}

pub fn install() {
    skyline::install_hooks!(
        ike_critical_zoom
    );
}