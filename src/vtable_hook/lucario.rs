use {
    smash::{
        app::{BattleObjectModuleAccessor, BattleObject, lua_bind::*},
        lib::lua_const::*
    },
    wubor_utils::wua_bind::*
};
use wubor_utils::offsets;

extern "C" {
    #[link_name = "lucario_get_aura"]
    fn get_aura(object: *mut BattleObject) -> f32;
}

#[skyline::hook(offset = offsets::fighter::lucario::CHECK_AURA)]
pub unsafe extern "C" fn lucario_check_aura(module_accessor: *mut BattleObjectModuleAccessor) -> f32 {
    if WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) > 7 {
        std::process::abort();
    }
    let object = MiscModule::get_battle_object_from_id((*module_accessor).battle_object_id);
    get_aura(object)
}

#[skyline::hook(offset = offsets::fighter::lucario::CHECK_AURA_2)]
pub unsafe extern "C" fn lucario_check_aura2(module: u64) -> f32 {
    let module_accessor = &mut *(*((module as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    if WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) > 7 {
        std::process::abort();
    }
    let object = MiscModule::get_battle_object_from_id(module_accessor.battle_object_id);
    get_aura(object)
}

pub fn install() {
    skyline::install_hooks!(
        lucario_check_aura,
        lucario_check_aura2
    );
}
