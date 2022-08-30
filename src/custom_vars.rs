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

pub fn install() {
    install_agent_resets!(
        fighter_reset,
        agent_reset
    );
}
