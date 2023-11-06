use crate::imports::acmd_imports::*;

#[acmd_script( agent = "samusd_missile", script = "game_homing", category = ACMD_GAME, low_priority )]
unsafe extern "C" fn samusd_missile_homing(_agent: &mut L2CAgentBase) {
}

#[acmd_script( agent = "samusd_missile", script = "game_hburst", category = ACMD_GAME, low_priority )]
unsafe extern "C" fn samusd_missile_hburst(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 30, 25, 0, 45, 15.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x199c462b5d));
    }
}

pub fn install(agent : &mut smashline::Agent) {
    agent.game_acmd("game_homing", samusd_missile_homing);

    agent.game_acmd("game_hburst", samusd_missile_hburst);
}