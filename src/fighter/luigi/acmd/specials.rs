use crate::imports::*;

unsafe extern "C" fn luigi_specialsstart(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_thunder"), smash::phx::Hash40::new("havel"), &ZERO_VECTOR, &ZERO_VECTOR, 0.6, true, 0, 0, 0, 0, 0, true, true);
        EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_thunder"), smash::phx::Hash40::new("handr"), &Vector3f { x: 3.0, y: 0.0, z: 0.0 }, &ZERO_VECTOR, 0.6, true, 0, 0, 0, 0, 0, true, true);
        EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_thunder"), smash::phx::Hash40::new("havel"), &ZERO_VECTOR, &ZERO_VECTOR, 0.6, true, 0, 0, 0, 0, 0, true, true);
        EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_thunder"), smash::phx::Hash40::new("handr"), &Vector3f { x: 3.0, y: 0.0, z: 0.0 }, &ZERO_VECTOR, 0.6, true, 0, 0, 0, 0, 0, true, true);
        EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_thunder"), smash::phx::Hash40::new("havel"), &ZERO_VECTOR, &ZERO_VECTOR, 0.6, true, 0, 0, 0, 0, 0, true, true);
        EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_thunder"), smash::phx::Hash40::new("handr"), &Vector3f { x: 3.0, y: 0.0, z: 0.0 }, &ZERO_VECTOR, 0.6, true, 0, 0, 0, 0, 0, true, true);
    }
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.4);
    frame(agent.lua_state_agent, 16.0);
    macros::FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn luigi_specialshold(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.0, 366, 100, 45, 0, 7.0, 0.0, 9.0, 10.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 1, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.0, 180, 120, 45, 0, 6.0, 0.0, 8.0, 35.0, Some(0.0), Some(8.0), Some(8.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 1, false, false, true, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 0.0, 180, 100, 45, 0, 6.0, 0.0, 8.0, 62.0, Some(0.0), Some(8.0), Some(35.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 1, false, false, true, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 4, 0, Hash40::new("top"), 0.0, 180, 80, 45, 0, 10.0, 0.0, 10.0, 35.0, Some(0.0), Some(10.0), Some(8.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 1, false, false, true, true, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 5, 0, Hash40::new("top"), 0.0, 180, 60, 45, 0, 10.0, 0.0, 10.0, 62.0, Some(0.0), Some(10.0), Some(35.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 1, false, false, true, true, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
}

unsafe extern "C" fn luigi_specialshold_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("luigi_rocket_hold"), Hash40::new("top"), 0, 10, 11, 0, 0, 0, 1, true);
        macros::LAST_EFFECT_SET_RATE(agent, 0.6);
    }
    for _ in 0..9 {
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 10, 0, 4, 0, 0, 0, false);
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0.0, 10.0, 60.0, 180.0, 0.0, 0.0, 0.8, true);
            macros::LAST_EFFECT_SET_ALPHA(agent, 0.15);
            macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 1.0, 0.7);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0.0, 10.0, 48.0, 180.0, 0.0, 0.0, 0.8, true);
            macros::LAST_EFFECT_SET_ALPHA(agent, 0.15);
            macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 1.0, 0.7);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0.0, 10.0, 36.0, 180.0, 0.0, 0.0, 0.8, true);
            macros::LAST_EFFECT_SET_ALPHA(agent, 0.15);
            macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 1.0, 0.7);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0.0, 10.0, 24.0, 180.0, 0.0, 0.0, 0.8, true);
            macros::LAST_EFFECT_SET_ALPHA(agent, 0.15);
            macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 1.0, 0.7);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0.0, 10.0, 12.0, 180.0, 0.0, 0.0, 0.8, true);
            macros::LAST_EFFECT_SET_ALPHA(agent, 0.15);
            macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 1.0, 0.7);
        }
        wait(agent.lua_state_agent, 2.0);
    }
}

unsafe extern "C" fn luigi_specialsend(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.0, 366, 100, 45, 0, 7.0, 0.0, 9.0, 10.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 1, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.0, 180, 120, 45, 0, 6.0, 0.0, 8.0, 35.0, Some(0.0), Some(8.0), Some(8.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 1, false, false, true, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 0.0, 180, 100, 45, 0, 6.0, 0.0, 8.0, 62.0, Some(0.0), Some(8.0), Some(35.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 1, false, false, true, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 4, 0, Hash40::new("top"), 0.0, 180, 80, 45, 0, 10.0, 0.0, 10.0, 35.0, Some(0.0), Some(10.0), Some(8.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 1, false, false, true, true, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 5, 0, Hash40::new("top"), 0.0, 180, 60, 45, 0, 10.0, 0.0, 10.0, 62.0, Some(0.0), Some(10.0), Some(35.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 1, false, false, true, true, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 90, 60, 0, 45, 5.0, 0.0, 14.0, 9.0, Some(0.0), Some(5.0), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 4.2);
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("luigi_rocket_hold"), false, true);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn luigi_specialsend_eff(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("luigi_rocket_hold"), Hash40::new("top"), 0, 10, 11, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("luigi_rocket_hold"), false, true);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_thunder"), false, true);
    }
}

unsafe extern "C" fn luigi_specialsend_snd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_luigi_special_s04"));
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_luigi_004"));
        macros::PLAY_SE(agent, Hash40::new("se_luigi_special_s03_s"));
    }
}

unsafe extern "C" fn luigi_specialsend_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

unsafe extern "C" fn luigi_specialhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        VarModule::off_flag(agent.module_accessor, luigi::instance::flag::SPECIAL_HI_CANCEL);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 20.0, 80, 55, 0, 80, 2.2, 1.2, 6.0, 7.0, Some(-1.2), Some(6.0), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_PUNCH);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_HI_FLAG_CRITICAL_HIT);
    }
    wait(agent.lua_state_agent, 1.0);
    if AttackModule::is_infliction_status(agent.module_accessor, *COLLISION_KIND_MASK_HIT) {
        VarModule::on_flag(agent.module_accessor, luigi::instance::flag::SPECIAL_HI_CANCEL);
    }
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("head"), 1.0, 80, 1, 0, 1, 5.8, 2.0, 2.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("hip"), 1.0, 80, 1, 0, 1, 4.7, 0.0, 4.8, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_HI_FLAG_CRITICAL_HIT);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::SA_SET(agent, *SITUATION_KIND_AIR);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
}

unsafe extern "C" fn luigi_specialairhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        VarModule::off_flag(agent.module_accessor, luigi::instance::flag::SPECIAL_HI_CANCEL);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 18.0, 80, 55, 0, 80, 2.7, 0.0, 6.0, 5.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        // WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_HI_FLAG_CRITICAL_HIT);
    }
    wait(agent.lua_state_agent, 1.0);
    if AttackModule::is_infliction_status(agent.module_accessor, *COLLISION_KIND_MASK_HIT) {
        VarModule::on_flag(agent.module_accessor, luigi::instance::flag::SPECIAL_HI_CANCEL);
    }
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("head"), 1.0, 80, 1, 0, 1, 5.8, 2.0, 2.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("hip"), 1.0, 80, 1, 0, 1, 4.7, 0.0, 4.8, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
        // WorkModule::off_flag(agent.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_HI_FLAG_CRITICAL_HIT);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::SA_SET(agent, *SITUATION_KIND_AIR);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
}

unsafe extern "C" fn luigi_speciallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    macros::FT_MOTION_RATE(agent, 3.0 / 7.0);
    frame(agent.lua_state_agent, 10.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_RISE);
        macros::ATTACK(agent, 0, 0, Hash40::new("arml"), 8.0, 90, 80, 0, 50, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("armr"), 8.0, 90, 80, 0, 50, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    macros::FT_MOTION_RATE(agent, 13.0 / 10.0);
    frame(agent.lua_state_agent, 44.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_RISE);
        macros::ATTACK(agent, 0, 0, Hash40::new("arml"), 12.0, 90, 80, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("armr"), 12.0, 90, 80, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 12.0, 90, 80, 0, 50, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_LIMIT_X_DEC);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_RISE);
        AttackModule::clear_all(agent.module_accessor);
    }
    MiscModule::calc_motion_rate_from_end_frame(agent, 45.0, -15.0);
}

unsafe extern "C" fn luigi_specialairlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    macros::FT_MOTION_RATE(agent, 3.0 / 7.0);
    frame(agent.lua_state_agent, 10.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_RISE);
        macros::ATTACK(agent, 0, 0, Hash40::new("arml"), 9.0, 90, 80, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("armr"), 9.0, 90, 80, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 90, 80, 0, 50, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    macros::FT_MOTION_RATE(agent, 13.0 / 10.0);
    frame(agent.lua_state_agent, 44.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_RISE);
        macros::ATTACK(agent, 0, 0, Hash40::new("arml"), 12.0, 90, 80, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("armr"), 12.0, 90, 80, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 12.0, 90, 80, 0, 50, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_LIMIT_X_DEC);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_RISE);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_BUOYANCY);
        AttackModule::clear_all(agent.module_accessor);
    }
    MiscModule::calc_motion_rate_from_end_frame(agent, 45.0, -15.0);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_specialsstart", luigi_specialsstart);

    agent.acmd("game_specialairsstart", luigi_specialsstart);

    agent.acmd("game_specialshold", luigi_specialshold);
    agent.acmd("effect_specialshold", luigi_specialshold_eff);

    agent.acmd("game_specialairshold", luigi_specialshold);
    agent.acmd("effect_specialairshold", luigi_specialshold_eff);

    agent.acmd("game_specialsend", luigi_specialsend);
    agent.acmd("effect_specialsend", luigi_specialsend_eff);
    agent.acmd("sound_specialsend", luigi_specialsend_snd);
    agent.acmd("expression_specialsend", luigi_specialsend_exp);

    agent.acmd("game_specialairsend", luigi_specialsend);
    agent.acmd("effect_specialairsend", luigi_specialsend_eff);
    agent.acmd("sound_specialairsend", luigi_specialsend_snd);
    agent.acmd("expression_specialairsend", luigi_specialsend_exp);

    agent.acmd("game_specialhi", luigi_specialhi);

    agent.acmd("game_specialairhi", luigi_specialairhi);

    agent.acmd("game_speciallw", luigi_speciallw);

    agent.acmd("game_specialairlw", luigi_specialairlw);
}