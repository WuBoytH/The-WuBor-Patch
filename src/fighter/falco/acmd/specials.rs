use crate::imports::acmd_imports::*;

#[acmd_script( agent = "falco", script = "game_specialsstart", category = ACMD_GAME, low_priority )]
unsafe extern "C" fn falco_specialsstart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.88);
}

#[acmd_script( agent = "falco", scripts = [ "game_speciallw", "game_specialairlw" ], category = ACMD_GAME, low_priority )]
unsafe extern "C" fn falco_speciallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("reflector"), 5.0, 361, 40, 0, 48, 3.5, 1.5, 0.0, 0.0, None, None, None, 0.25, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        AttackModule::enable_safe_pos(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.game_acmd("game_specialsstart", falco_specialsstart);

    agent.game_acmd("game_speciallw", falco_speciallw);

    agent.game_acmd("game_specialairlw", falco_speciallw);
}