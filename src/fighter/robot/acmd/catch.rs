use crate::imports::acmd_imports::*;

#[acmd_script( agent = "robot", script = "game_catchattack", category = ACMD_GAME, low_priority )]
unsafe fn robot_catchattack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.9, 361, 100, 30, 0, 5.0, 0.0, 8.0, 11.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

#[acmd_script( agent = "robot", script = "effect_catchattack", category = ACMD_EFFECT, low_priority )]
unsafe fn robot_catchattack_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLIP_ALPHA(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 18, -2, 25, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ, 0.3);
    }
}

pub fn install() {
    install_acmd_scripts!(
        robot_catchattack, robot_catchattack_eff
    );
}