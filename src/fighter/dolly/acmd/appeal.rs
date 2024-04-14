use crate::imports::*;

unsafe extern "C" fn game_appealhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 0.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);  
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);       
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("handl"), 3.0, 90, 32, 16, 32, 2.8, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 5.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    wait(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) { 
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 42.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 1, 1, Hash40::new("handl"), 5.0, 90, 64, 0, 32, 3.2, 0.0, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 15.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);  
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    wait(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) { 
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn game_appeallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 0.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);  
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);       
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("hat"), 0.4, 367, 30, 10, 10, 2.0, 0.0, 0.0, 0.0, None, None, None, 0.1, 0.4, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.2, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_G_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("hat"), 0.4, 367, 10, 5, 5, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.1, 0.4, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.2, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);       
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    wait(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) { 
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_appealhir", game_appealhi);
    agent.acmd("game_appealhil", game_appealhi);

    agent.acmd("game_appeallwr", game_appeallw);
    agent.acmd("game_appeallwl", game_appeallw);
}