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
    let object = &mut fighter.battle_object;
    if [
        0x255, *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_END
    ].contains(&StatusModule::status_kind(module_accessor))
    && VarModule::is_flag(object, ike::status::flag::SPECIAL_N_ENABLE_CRITICAL) {
        VarModule::off_flag(object, ike::status::flag::SPECIAL_N_ENABLE_CRITICAL);
        let param_hash = if WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
            hash40("param_special_n_kirby")
        }
        else {
            hash40("param_special_n")
        };
        MiscModule::call_critical(module_accessor, unk, 0x23, param_hash, 1, 0, 0, 0);
    }
}

pub fn install() {
    skyline::install_hooks!(
        ike_critical_zoom
    );
}