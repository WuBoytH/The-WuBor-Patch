use {
    smash::{
        lua2cpp::*,
        app::{lua_bind::*, *},
        lib::lua_const::*
    },
    smashline::*,
    custom_var::*
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
    unsafe {
        if utility::get_category(&mut *agent.module_accessor) == *BATTLE_OBJECT_CATEGORY_WEAPON {
            // println!("Weapon ID: {:#x}", agent.battle_object_id);
            let owner_object_id = WorkModule::get_int(agent.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
            // println!("Owner ID: {:#x}", owner_object_id);
            let owner_cat = sv_battle_object::category(owner_object_id);
            // println!("Owner Category: {:#x}", owner_cat);
            let owner_kind = sv_battle_object::kind(owner_object_id);
            // println!("Owner Kind: {:#x}", owner_kind);
            if owner_cat == *BATTLE_OBJECT_CATEGORY_FIGHTER
            && [*FIGHTER_KIND_MURABITO, *FIGHTER_KIND_SHIZUE].contains(&owner_kind) {
                // println!("Owner objects are either Villager or Isabelle!");
                let owner_module_accessor = sv_battle_object::module_accessor(owner_object_id);
                let pocket_object_id = WorkModule::get_int(owner_module_accessor, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_SPECIAL_N_OBJECT_ID) as u32;
                // println!("Pocketed Item Object ID: {:#x}", pocket_object_id);
                if agent.battle_object_id == pocket_object_id {
                    // println!("Pocket object and new object match! Retrieving pocketed vars...");
                    VarModule::retrieve_pocketed_vars(agent.battle_object, owner_object_id);
                }
            }
        }
    }
}

pub fn install() {
    install_agent_resets!(
        fighter_reset,
        agent_reset
    );
    install_agent_init_callbacks!(
        agent_init
    );
}
