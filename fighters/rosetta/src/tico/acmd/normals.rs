use super::*;

unsafe extern "C" fn game_attack11(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::SEARCH(agent, *WEAPON_ROSETTA_TICO_SEARCH_KIND_FREE_ATTACK_LR as u64, 0, Hash40::new("top"), 7.0, 0.0, 6.0, 3.0, Some(0.0), Some(6.0), Some(-3.0), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 60, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    if macros::is_excute(agent) {
        JostleModule::set_priority(agent.module_accessor, *JOSTLE_PRI_HIGH);
        agent.clear_lua_stack();
        lua_args!(agent, *WEAPON_ROSETTA_TICO_KINETIC_ENERGY_ID_JOSTLE);
        sv_animcmd::CLR_SPEED(agent.lua_state_agent);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        MotionModule::set_rate(agent.module_accessor, 1.5);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        MotionModule::set_rate(agent.module_accessor, 1.0);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 70, 100, 50, 0, 2.5, 0.0, 4.0, 2.0, Some(0.0), Some(4.0), Some(10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *WEAPON_ROSETTA_TICO_STATUS_ATTACK_WORK_FLAG_ENABLE_COMBO);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        MotionModule::set_rate(agent.module_accessor, 1.5);
    }
}

unsafe extern "C" fn game_attack12(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        JostleModule::set_priority(agent.module_accessor, *JOSTLE_PRI_HIGH);
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 90, 100, 50, 0, 2.5, 0.0, 4.0, 4.0, Some(0.0), Some(4.0), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *WEAPON_ROSETTA_TICO_STATUS_ATTACK_WORK_FLAG_ENABLE_100);
        WorkModule::on_flag(agent.module_accessor, *WEAPON_ROSETTA_TICO_STATUS_ATTACK_WORK_FLAG_ENABLE_COMBO);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        MotionModule::set_rate(agent.module_accessor, 1.5);
    }
}

unsafe extern "C" fn game_attack13(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        JostleModule::set_priority(agent.module_accessor, *JOSTLE_PRI_HIGH);
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("footl"), 2.0, 80, 150, 0, 60, 3.5, 1.2, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("footl"), 2.0, 80, 150, 0, 60, 2.5, -3.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        MotionModule::set_rate(agent.module_accessor, 1.5);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attack11", game_attack11, Priority::Low);

    agent.acmd("game_attack12", game_attack12, Priority::Low);

    agent.acmd("game_attack13", game_attack13, Priority::Low);
}