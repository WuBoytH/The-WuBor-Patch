use smash::{
    app::{lua_bind::*, *},
    lib::lua_const::*
};

pub static mut IS_CHANGE_ATTACK : [bool; 8] = [false; 8];

pub unsafe extern "C" fn element_set_change_attack(module_accessor: *mut BattleObjectModuleAccessor, is_attack: bool) {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if (0..8).contains(&entry_id) {
        IS_CHANGE_ATTACK[entry_id] = is_attack;
    }
}

pub unsafe extern "C" fn element_is_change_attack(module_accessor: *mut BattleObjectModuleAccessor) -> bool {
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if (0..8).contains(&entry_id) {
        IS_CHANGE_ATTACK[entry_id]
    }
    else {
        false
    }
}
