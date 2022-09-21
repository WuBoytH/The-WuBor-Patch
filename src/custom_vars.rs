#![allow(non_snake_case)]

use {
    smash::{
        app:: *,
    },
    custom_var::*
};

#[skyline::hook(offset = 0x3af9f0)]
pub unsafe fn battleobjectmoduleaccessor__start_modules(module_accessor: *mut BattleObjectModuleAccessor, param_1: u32) {
    original!()(module_accessor, param_1);
    let object_id = (*module_accessor).battle_object_id;
    // println!("[CustomVarManager] Module Count before adding: {}", CustomVarManager::count());
    // println!("[CustomVarManager] Reset Modules for {:#x}", object_id);
    CustomVarManager::reset_var_module_by_object_id(object_id, false);
    // println!("[CustomVarManager] Module Count after adding: {}", CustomVarManager::count());
}

#[skyline::hook(offset = 0x3afde0)]
pub unsafe fn battleobjectmoduleaccessor__end_modules(module_accessor: *mut BattleObjectModuleAccessor, param_1: u32) {
    let object_id = (*module_accessor).battle_object_id;
    // println!("[CustomVarManager] Module Count before removing: {}", CustomVarManager::count());
    // println!("[CustomVarManager] Ending Modules for {:#x}", object_id);
    CustomVarManager::remove_var_module_by_object_id(object_id);
    // println!("[CustomVarManager] Module Count after removing: {}", CustomVarManager::count());
    original!()(module_accessor, param_1)
}

pub fn install() {
    skyline::install_hooks!(
        battleobjectmoduleaccessor__start_modules,
        battleobjectmoduleaccessor__end_modules
    );
}
