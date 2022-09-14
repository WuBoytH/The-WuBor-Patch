#![allow(non_snake_case)]

use {
    smash::{
        lua2cpp::*,
        app::{lua_bind::*, *},
        lib::lua_const::*
    },
    smashline::*,
    custom_var::*,
    wubor_utils::{wua_bind::*, vars::*}
};

#[fighter_reset]
fn fighter_reset(fighter: &mut L2CFighterCommon) {
    // println!("[CustomVarManager] Fighter Reset!");
    CustomVarManager::reset_var_module(fighter.battle_object);
}

#[agent_reset]
fn agent_reset(agent: &mut L2CFighterBase) {
    // println!("[CustomVarManager] Agent Reset!");
    unsafe {
        if utility::get_category(&mut *agent.module_accessor) == *BATTLE_OBJECT_CATEGORY_WEAPON {
            if !WorkModule::is_flag(agent.module_accessor, *WEAPON_INSTANCE_WORK_ID_FLAG_SWALLOWED) {
                CustomVarManager::reset_var_module(agent.battle_object);
            }
        }
    }
}

#[agent_init]
fn agent_init(agent: &mut L2CFighterBase) {
    // println!("[CustomVarManager] Agent Init!");
    unsafe {
        if utility::get_category(&mut *agent.module_accessor) == *BATTLE_OBJECT_CATEGORY_WEAPON {
            if !VarModule::is_flag(agent.battle_object, weapon::instance::flag::FROM_POCKET) {
                // println!("Weapon! Grabbing vars from pocket...");
                MiscModule::get_vars_from_pocket(agent.battle_object);
                // println!("Was this outta pocket? {}", VarModule::is_flag(agent.battle_object, weapon::instance::flag::FROM_POCKET));
            }
            if !VarModule::is_flag(agent.battle_object, weapon::instance::flag::FROM_POCKET) {
                let owner_object_id = WorkModule::get_int(agent.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
                VarModule::set_int(agent.battle_object, weapon::instance::int::ORIGINAL_OWNER, owner_object_id as i32);
            }
        }
    }
}

#[skyline::hook(offset = 0x3afde0)]
pub unsafe fn battleobjectmoduleaccessor__end_modules(module_accessor: *mut BattleObjectModuleAccessor) {
    let object_id = (*module_accessor).battle_object_id;
    // println!("[CustomVarManager] Module Count before removing: {}", CustomVarManager::count());
    // println!("[CustomVarManager] Ending Modules for {:#x}", object_id);
    CustomVarManager::remove_var_module_by_object_id(object_id);
    // println!("[CustomVarManager] Module Count after removing: {}", CustomVarManager::count());
    original!()(module_accessor);
}

pub fn install() {
    install_agent_resets!(
        fighter_reset,
        agent_reset
    );
    install_agent_init_callbacks!(
        agent_init
    );
    skyline::install_hooks!(
        battleobjectmoduleaccessor__end_modules
    );
}
