#![allow(non_snake_case)]

use {
    smash::app::*,
    custom_var::*,
    wubor_utils::wua_bind::*
};

#[skyline::hook(offset = 0x3af300)]
pub unsafe fn battleobjectmoduleaccessor__initialize_modules(module_accessor: *mut BattleObjectModuleAccessor, param_1: *const u64) {
    original!()(module_accessor, param_1);
    // println!("[CustomVarManager] Initialize");
    let object_id = (*module_accessor).battle_object_id;
    // println!("[CustomVarManager] Initializing VarModule for {:#x}", object_id);
    // println!("[CustomVarManager] VarModule Count before adding: {}", CustomVarManager::count());
    if object_id != 0x50000000 {
        // println!("[CustomVarManager] Object ID is not invalid! Adding...");
        CustomVarManager::reset_var_module(module_accessor, false);
    }
    // println!("[CustomVarManager] VarModule Count after adding: {}", CustomVarManager::count());
}

#[skyline::hook(offset = 0x3afa10)]
pub unsafe fn battleobjectmoduleaccessor__start_modules(module_accessor: *mut BattleObjectModuleAccessor, param_1: u32) {
    original!()(module_accessor, param_1);
    // let object_id = (*module_accessor).battle_object_id;
    // println!("[CustomVarManager] Start");
    // println!("[CustomVarManager] Starting VarModule for {:#x}", object_id);
    VarModule::start(module_accessor);
}

#[skyline::hook(offset = 0x33a07d0)]
pub unsafe fn weapon_init_hook(weapon: &mut smash::app::Weapon, param_2: u64) {
    original!()(weapon, param_2);
    MiscModule::get_vars_from_pocket(weapon.battle_object.module_accessor);
}

#[skyline::hook(offset = 0x3afe00)]
pub unsafe fn battleobjectmoduleaccessor__end_modules(module_accessor: *mut BattleObjectModuleAccessor, param_1: u32) {
    // println!("[CustomVarManager] End");
    // let object_id = (*module_accessor).battle_object_id;
    // println!("[CustomVarManager] Ending VarModule for {:#x} (not really)", object_id);
    CustomVarManager::reset_var_module(module_accessor, true);
    original!()(module_accessor, param_1)
}

#[skyline::hook(offset = 0x3af720)]
pub unsafe fn battleobjectmoduleaccessor__finalize_modules(module_accessor: *mut BattleObjectModuleAccessor) {
    // let object_id = (*module_accessor).battle_object_id;
    // println!("[CustomVarManager] Finalize");
    // println!("[CustomVarManager] Finalizing VarModule for {:#x}", object_id);
    // println!("[CustomVarManager] VarModule Count before removing: {}", CustomVarManager::count());
    CustomVarManager::remove_var_module(module_accessor);
    // println!("[CustomVarManager] VarModule Count after removing: {}", CustomVarManager::count());
    original!()(module_accessor)
}

pub fn install() {
    skyline::install_hooks!(
        battleobjectmoduleaccessor__initialize_modules,
        battleobjectmoduleaccessor__start_modules,
        weapon_init_hook,
        battleobjectmoduleaccessor__end_modules,
        battleobjectmoduleaccessor__finalize_modules
    );
}
