use crate::imports::acmd_imports::*;

#[acmd_script( agent = "duckhunt_clay", script = "game_fly", category = ACMD_GAME, low_priority )]
unsafe extern "C" fn duckhunt_clay_fly(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 75, 50, 0, 20, 2.0, 0.0, 0.0, -3.0, Some(0.0), Some(0.0), Some(3.0), 2.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -1, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *WEAPON_DUCKHUNT_CLAY_INSTANCE_WORK_ID_FLAG_IS_ADD_ACCEL_Y);
    }
}

#[acmd_script( agent = "duckhunt", script = "game_specialhi", category = ACMD_GAME, low_priority )]
unsafe extern "C" fn duckhunt_specialhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 39.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DUCKHUNT_INSTANCE_WORK_ID_FLAG_REQUEST_SPECIAL_HI_CANCEL);
    }
}

pub fn install(agent : &mut smashline::Agent) {
    let clay = &mut smashline::Agent::new("duckhunt_clay");
    clay.game_acmd("game_fly", duckhunt_clay_fly);
    clay.install();

    agent.game_acmd("game_specialhi", duckhunt_specialhi);
}