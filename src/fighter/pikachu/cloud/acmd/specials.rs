use crate::imports::acmd_imports::*;

#[acmd_script( agent = "pikachu_cloud", script = "game_regular", category = ACMD_GAME, low_priority )]
unsafe extern "C" fn pikachu_cloud_regular(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *WEAPON_PIKACHU_CLOUD_INSTANCE_WORK_ID_FLAG_ACTIVATE_KAMINARI);
    }
}

#[acmd_script( agent = "pikachu_cloud", script = "effect_regular", category = ACMD_EFFECT, low_priority )]
unsafe extern "C" fn pikachu_cloud_regular_eff(_agent: &mut L2CAgentBase) {
    // if macros::is_excute(agent) {
    //     macros::EFFECT(agent, Hash40::new("pikachu_kaminari_cloud"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    // }
}

pub fn install(agent : &mut smashline::Agent) {
    agent.game_acmd("game_regular", pikachu_cloud_regular);
    agent.effect_acmd("effect_regular", pikachu_cloud_regular_eff);
}