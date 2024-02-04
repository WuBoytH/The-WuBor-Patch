use crate::imports::acmd_imports::*;

unsafe extern "C" fn koopa_breath_move(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 50, 60, 0, 72, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.8, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        AttackModule::enable_safe_pos(agent.module_accessor);
    }
}

unsafe extern "C" fn koopa_breath_move_eff(agent: &mut L2CAgentBase) {
    loop {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire_fly"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, true);
            macros::EFFECT_FOLLOW(agent, Hash40::new("koopa_breath_m_fire"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, true);
        }
        wait(agent.lua_state_agent, 15.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire_fly"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, true);
        }
        wait(agent.lua_state_agent, 15.0);
    }
}

unsafe extern "C" fn koopa_breath_move_snd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_STATUS(agent, Hash40::new("se_koopa_special_n02"));
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_move", koopa_breath_move);
    agent.acmd("effect_move", koopa_breath_move_eff);
    agent.acmd("sound_move", koopa_breath_move_snd);
}