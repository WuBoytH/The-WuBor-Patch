use super::*;

unsafe extern "C" fn game_guardcancelattack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 0, 0, 80, 6.5, 0.0, 8.5, 8.0, None, None, None, 1.5, 0.7, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 361, 0, 0, 80, 6.5, 0.0, 8.5, 13.0, None, None, None, 1.5, 0.7, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 5.0, 361, 0, 0, 80, 6.5, 0.0, 8.5, 18.0, None, None, None, 1.5, 0.7, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_guardcancelattack(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_flash"), Hash40::new("top"), 0.0, 12.0, 6.0, 0.0, 0.0, 0.0, 1.4, true);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_WORK(agent, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_ATTACK_100END, Hash40::new("trans"), 1.5, 8, 0, 0, 0, 120, 1.25, true);
        EffectModule::set_disable_render_offset_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("bayonetta_butterfly_walk"), Hash40::new("top"), 4, 0, 4, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_COSTUME_KIND) != *FIGHTER_BAYONETTA_COSTUME_KIND_BAYONETTA_1 {
        if macros::is_excute(agent) {
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("bayonetta_attack_wind"), Hash40::new("trans"), 0, 10, 20, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true, 0.745, 0.941, 1);
            EFFECT_COLOR(agent.lua_state_agent);
            macros::LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(agent.lua_state_agent, 22.0);
        if macros::is_excute(agent) {
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("bayonetta_beretta_rotation"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true, 0.059, 0.38, 1);
            EFFECT_FOLLOW_COLOR(agent.lua_state_agent);
        }
    }
    else {
        if macros::is_excute(agent) {
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("bayonetta_attack_wind"), Hash40::new("trans"), 0, 10, 20, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true, 0.745, 0.941, 1);
            EFFECT_COLOR(agent.lua_state_agent);
            macros::LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(agent.lua_state_agent, 22.0);
        if macros::is_excute(agent) {
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("bayonetta_beretta_rotation"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true, 1, 0.118, 0.118);
            EFFECT_FOLLOW_COLOR(agent.lua_state_agent);
        }
    }
    frame(agent.lua_state_agent, 42.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("bayonetta_beretta_rotation"), false, true);
    }
}

unsafe extern "C" fn sound_guardcancelattack(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_guard_cancel_attack"));
    }
    agent.clear_lua_stack();
    GET_VOICE_REGION(agent.lua_state_agent);
    let voice = agent.pop_lua_stack(1).get_i32();
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_COSTUME_KIND) <= *FIGHTER_BAYONETTA_COSTUME_KIND_BAYONETTA_1
    || voice == *VOICE_DEFAULT {
        frame(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            sound!(agent, *MA_MSC_CMD_SOUND_STOP_SE_STATUS);
        }
        frame(agent.lua_state_agent, 8.0);
        if macros::is_excute(agent) {
            macros::PLAY_SEQUENCE(agent, Hash40::new("seq_bayonetta_rnd_attack_smash_s01_jp"));
        }
        frame(agent.lua_state_agent, 10.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_bayonetta_attack100_02"));
        }
        frame(agent.lua_state_agent, 22.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_bayonetta_loveisblue_spin"));
        }
        frame(agent.lua_state_agent, 55.0);
        if macros::is_excute(agent) {
            macros::PLAY_STEP(agent, Hash40::new("se_bayonetta_step_right_s"));
        }
    }
    else {
        frame(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            sound!(agent, *MA_MSC_CMD_SOUND_STOP_SE_STATUS);
        }
        frame(agent.lua_state_agent, 8.0);
        if macros::is_excute(agent) {
            macros::PLAY_SEQUENCE(agent, Hash40::new("seq_bayonetta_rnd_attack_smash_s01_en"));
        }
        frame(agent.lua_state_agent, 10.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_bayonetta_attack100_02"));
        }
        frame(agent.lua_state_agent, 22.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_bayonetta_loveisblue_spin"));
        }
        frame(agent.lua_state_agent, 55.0);
        if macros::is_excute(agent) {
            macros::PLAY_STEP(agent, Hash40::new("se_bayonetta_step_right_s"));
        }
    }
}

unsafe extern "C" fn expression_guardcancelattack(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_guardcancelattack", game_guardcancelattack, Priority::Low);
    agent.acmd("effect_guardcancelattack", effect_guardcancelattack, Priority::Low);
    agent.acmd("sound_guardcancelattack", sound_guardcancelattack, Priority::Low);
    agent.acmd("expression_guardcancelattack", expression_guardcancelattack, Priority::Low);
}