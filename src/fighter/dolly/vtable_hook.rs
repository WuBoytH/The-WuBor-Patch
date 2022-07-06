use {
    smash::{
        app::{lua_bind::*, *},
        lib::lua_const::*
    },
    custom_var::*,
    crate::function_hooks::get_battle_object_from_id,
    wubor_utils::vars::*
};

#[skyline::hook(offset = 0x971230)]
pub unsafe extern "C" fn dolly_check_super_special(work: u64, _damage: u64) -> u64 {
    let module_accessor = &mut *(*((work as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    if WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) > 7 {
        std::process::abort();
    }
    if smashball::is_training_mode() {
        return 1;
    }
    let object_id = (*module_accessor).battle_object_id;
    let object = get_battle_object_from_id(object_id);
    let go_meter = VarModule::get_float(object, dolly::instance::float::GO_METER);
    // println!("go_meter: {}", go_meter);
    if go_meter >= 100.0 {
        return 1;
    }
    0
}

#[skyline::hook(offset = 0x970fd0)]
pub unsafe extern "C" fn dolly_check_super_special_pre(module_accessor: *mut BattleObjectModuleAccessor, param_2: u8) {
    original!()(module_accessor, param_2)
}

pub fn install() {
    skyline::install_hooks!(
        dolly_check_super_special,
        dolly_check_super_special_pre
    );
}