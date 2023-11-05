use crate::imports::acmd_imports::*;

#[acmd_script( agent = "gamewatch", script = "game_catchattack", category = ACMD_GAME, low_priority )]
unsafe extern "C" fn gamewatch_catchattack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.6, 361, 100, 30, 0, 6.0, 0.0, 6.6, 8.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

#[acmd_script( agent = "gamewatch", script = "sound_catchattack", category = ACMD_SOUND, low_priority )]
unsafe extern "C" fn gamewatch_catchattack_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_gamewatch_wave02_lo"));
    }
}

pub fn install(agent : &mut smashline::Agent) {
    agent.game_acmd("game_catchattack", gamewatch_catchattack);
    agent.sound_acmd("sound_catchattack", gamewatch_catchattack_snd);
}