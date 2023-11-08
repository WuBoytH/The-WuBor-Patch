#![allow(non_snake_case)]

use {
    smash::app::*,
    custom_var::*
};

#[skyline::hook(offset = 0x3af2e0)]
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

#[skyline::hook(offset = 0x3af9f0)]
pub unsafe fn battleobjectmoduleaccessor__start_modules(module_accessor: *mut BattleObjectModuleAccessor, param_1: u32) {
    original!()(module_accessor, param_1);
    // let object_id = (*module_accessor).battle_object_id;
    // println!("[CustomVarManager] Start");
    // println!("[CustomVarManager] Starting VarModule for {:#x}", object_id);
    VarModule::start(module_accessor);
}

#[skyline::hook(offset = 0x3afde0)]
pub unsafe fn battleobjectmoduleaccessor__end_modules(module_accessor: *mut BattleObjectModuleAccessor, param_1: u32) {
    // println!("[CustomVarManager] End");
    // let object_id = (*module_accessor).battle_object_id;
    // println!("[CustomVarManager] Ending VarModule for {:#x} (not really)", object_id);
    CustomVarManager::reset_var_module(module_accessor, true);
    original!()(module_accessor, param_1)
}

#[skyline::hook(offset = 0x3af700)]
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
        battleobjectmoduleaccessor__end_modules,
        battleobjectmoduleaccessor__finalize_modules
    );
}
