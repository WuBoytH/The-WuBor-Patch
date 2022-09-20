#![allow(non_snake_case)]

use {
    smash::{
        // lua2cpp::*,
        app::{lua_bind::*, *},
        lib::lua_const::*
    },
    // smashline::*,
    custom_var::*,
    wubor_utils::{wua_bind::*, vars::*}
};

// #[fighter_reset]
// fn fighter_reset(fighter: &mut L2CFighterCommon) {
//     // println!("[CustomVarManager] Fighter Reset!");
//     CustomVarManager::reset_var_module(fighter.battle_object);
// }

// #[agent_reset]
// fn agent_reset(agent: &mut L2CFighterBase) {
//     // println!("[CustomVarManager] Agent Reset!");
//     unsafe {
//         if utility::get_category(&mut *agent.module_accessor) == *BATTLE_OBJECT_CATEGORY_WEAPON {
//             if !WorkModule::is_flag(agent.module_accessor, *WEAPON_INSTANCE_WORK_ID_FLAG_SWALLOWED) {
//                 CustomVarManager::reset_var_module(agent.battle_object);
//             }
//         }
//     }
// }

// #[agent_init]
// fn agent_init(agent: &mut L2CFighterBase) {
//     // println!("[CustomVarManager] Agent Init!");
//     unsafe {
//         if utility::get_category(&mut *agent.module_accessor) == *BATTLE_OBJECT_CATEGORY_WEAPON {
//             if !VarModule::is_flag(agent.battle_object, weapon::instance::flag::FROM_POCKET) {
//                 // println!("Weapon! Grabbing vars from pocket...");
//                 MiscModule::get_vars_from_pocket(agent.battle_object);
//                 // println!("Was this outta pocket? {}", VarModule::is_flag(agent.battle_object, weapon::instance::flag::FROM_POCKET));
//             }
//             if !VarModule::is_flag(agent.battle_object, weapon::instance::flag::FROM_POCKET) {
//                 let owner_object_id = WorkModule::get_int(agent.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
//                 VarModule::set_int(agent.battle_object, weapon::instance::int::ORIGINAL_OWNER, owner_object_id as i32);
//             }
//         }
//     }
// }

#[skyline::hook(offset = 0x3af9f0)]
pub unsafe fn battleobjectmoduleaccessor__start_modules(module_accessor: *mut BattleObjectModuleAccessor, param_1: u32) {
    original!()(module_accessor, param_1);
    let object_id = (*module_accessor).battle_object_id;
    let object = MiscModule::get_battle_object_from_id(object_id);
    // println!("[CustomVarManager] Module Count before adding: {}", CustomVarManager::count());
    // println!("[CustomVarManager] Reset Modules for {:#x}", object_id);
    CustomVarManager::reset_var_module(object);
    // println!("[CustomVarManager] Module Count after adding: {}", CustomVarManager::count());
    if utility::get_category(&mut *module_accessor) == *BATTLE_OBJECT_CATEGORY_WEAPON {
        if !VarModule::is_flag(object, weapon::instance::flag::FROM_POCKET) {
            // println!("Weapon! Grabbing vars from pocket...");
            MiscModule::get_vars_from_pocket(object);
            // println!("Was this outta pocket? {}", VarModule::is_flag(object, weapon::instance::flag::FROM_POCKET));
        }
        if !VarModule::is_flag(object, weapon::instance::flag::FROM_POCKET) {
            let owner_object_id = WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
            // println!("Founder: {:#x}", owner_object_id);
            VarModule::set_int(object, weapon::instance::int::ORIGINAL_OWNER, owner_object_id as i32);
        }
        if utility::get_kind(&mut *module_accessor) == *WEAPON_KIND_LUCARIO_AURABALL {
            let owner_id = WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
            let owner_object = MiscModule::get_battle_object_from_id(owner_id);
            if !VarModule::is_flag(object, weapon::instance::flag::FROM_POCKET) {
                if sv_battle_object::category(owner_id) == *BATTLE_OBJECT_CATEGORY_FIGHTER
                && sv_battle_object::kind(owner_id) == *FIGHTER_KIND_LUCARIO {
                    if VarModule::is_flag(owner_object, lucario::status::flag::SPECIAL_N_SPIRIT_BOMB) {
                        VarModule::on_flag(object, lucario_auraball::instance::flag::SPIRIT_BOMB);
                    }
                }
            }
            if VarModule::is_flag(owner_object, lucario_auraball::instance::flag::SPIRIT_BOMB) {
                if StatusModule::situation_kind((*owner_object).module_accessor) == *SITUATION_KIND_GROUND {
                    WorkModule::set_customize_no(module_accessor, 1, 0);
                }
                else {
                    WorkModule::set_customize_no(module_accessor, 2, 0);
                }
            }
        }
    }
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
    // install_agent_resets!(
    //     fighter_reset,
    //     agent_reset
    // );
    // install_agent_init_callbacks!(
    //     agent_init
    // );
    skyline::install_hooks!(
        battleobjectmoduleaccessor__start_modules,
        battleobjectmoduleaccessor__end_modules
    );
}
