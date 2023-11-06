use crate::imports::acmd_imports::*;

#[acmd_script( agent = "wolf", scripts = [ "game_specialsend", "game_specialairsend" ], category = ACMD_GAME, low_priority )]
unsafe extern "C" fn wolf_specialsend(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_inflict_kind_status(agent.module_accessor);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 28, 85, 0, 30, 7.0, 0.0, 5.5, 5.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 18.0);
    if AttackModule::is_infliction_status(agent.module_accessor, *COLLISION_KIND_MASK_HIT) {
        if macros::is_excute(agent) {
            CancelModule::enable_cancel(agent.module_accessor);
        }
    }
}

pub fn install(agent : &mut smashline::Agent) {
    agent.game_acmd("game_specialsend", wolf_specialsend);

    agent.game_acmd("game_specialairsend", wolf_specialsend);
}