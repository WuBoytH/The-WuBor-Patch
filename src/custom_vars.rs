use {
    smash::{
        lua2cpp::*
    },
    smashline::*,
    custom_var::*
};

#[agent_reset]
fn agent_reset(fighter: &mut L2CFighterBase) {
    CustomVarManager::reset_var_module(fighter.battle_object);
}

pub fn install() {
    install_agent_resets!(
        agent_reset
    );
}
