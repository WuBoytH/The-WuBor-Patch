use crate::imports::acmd_imports::*;

unsafe extern "C" fn rockman_airshooter_regular(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.75, 367, 100, 70, 0, 5.0, 0.0, 3.0, 0.0, None, None, None, 0.2, 0.7, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 3, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 0.5);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, rockman_airshooter::status::flag::MOVE);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 92, 50, 0, 90, 7.0, 0.0, 3.0, 0.0, None, None, None, 0.2, 0.7, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 0.5);
        WorkModule::on_flag(agent.module_accessor, *WEAPON_ROCKMAN_AIRSHOOTER_INSTANCE_WORK_ID_FLAG_ATTACK_VECTOR_REVERSE_UD_CHECK);
    }
}

pub fn install(agent : &mut smashline::Agent) {
    agent.game_acmd("game_regular", rockman_airshooter_regular);
}