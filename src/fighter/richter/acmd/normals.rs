use crate::imports::acmd_imports::*;

#[acmd_script( agent = "richter", script = "game_attack11", category = ACMD_GAME, low_priority )]
unsafe fn richter_attack11(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("kneel"), 4.0, 361, 20, 0, 40, 4.0, 4.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneel"), 4.0, 361, 20, 0, 40, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 7.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 7.0, false);
    }
    macros::FT_MOTION_RATE(agent, 5.0 / 9.0);
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 14.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.battle_object, richter::status::flag::ATTACK_JUST_INPUT);
    }
    frame(agent.lua_state_agent, 24.0);
    // if macros::is_excute(agent) {
    //     VarModule::off_flag(agent.battle_object, richter::status::flag::ATTACK_JUST_INPUT);
    // }
    MiscModule::calc_motion_rate_from_cancel_frame(agent, 24.0, -4.0);
}

#[acmd_script( agent = "richter", script = "effect_attack11", category = ACMD_EFFECT, low_priority )]
unsafe fn richter_attack11_eff(agent: &mut L2CAgentBase) {
    // frame(agent.lua_state_agent, 4.0);
    // if macros::is_excute(agent) {
    //     macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 12, 4, 0, 0, 0, 0.6, true);
    // }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_arc"), Hash40::new("top"), 0, 12, 0, -180, 140, -25, 1.1, true);
        macros::LAST_EFFECT_SET_RATE(agent, 0.75);
        macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.4, 0.3);
    }
}

#[acmd_script( agent = "richter", script = "sound_attack11", category = ACMD_SOUND, low_priority )]
unsafe fn richter_attack11_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_punch_kick_swing_s"));
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_richter_rnd_attack"));
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_punch_kick_swing_m"));
    }
}

#[acmd_script( agent = "richter", script = "expression_attack11", category = ACMD_EXPRESSION, low_priority )]
unsafe fn richter_attack11_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohits"),
            4,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
    }
}

#[acmd_script( agent = "richter", script = "game_attack12", category = ACMD_GAME, low_priority )]
unsafe fn richter_attack12(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("kneel"), 5.0, 20, 20, 0, 80, 4.0, 4.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneel"), 5.0, 20, 20, 0, 80, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 5.0, 20, 20, 0, 80, 3.0, 0.0, 8.0, 6.0, Some(0.0), Some(8.0), Some(14.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

#[acmd_script( agent = "richter", script = "effect_attack12", category = ACMD_EFFECT, low_priority )]
unsafe fn richter_attack12_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_arc"), Hash40::new("top"), 0, 12, 4, 20, -90, 0, 1.1, true);
        macros::LAST_EFFECT_SET_RATE(agent, 0.75);
        macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.4, 0.3);
    }
}

#[acmd_script( agent = "richter", script = "sound_attack12", category = ACMD_SOUND, low_priority )]
unsafe fn richter_attack12_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_punch_kick_swing_m"));
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_richter_rnd_attack"));
    }
}

#[acmd_script( agent = "richter", script = "expression_attack12", category = ACMD_EXPRESSION, low_priority )]
unsafe fn richter_attack12_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohits"),
            4,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
    }
}

#[acmd_script( agent = "richter", script = "game_attack12f", category = ACMD_GAME, low_priority )]
unsafe fn richter_attack12f(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("kneel"), 8.0, 74, 20, 0, 90, 4.0, 4.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneel"), 8.0, 74, 20, 0, 90, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

#[acmd_script( agent = "richter", script = "effect_attack12f", category = ACMD_EFFECT, low_priority )]
unsafe fn richter_attack12f_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_arc"), Hash40::new("top"), 0, 3, 0, 0, -90, 180, 1.1, true);
        macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.4, 0.3);
    }
}

#[acmd_script( agent = "richter", script = "sound_attack12f", category = ACMD_SOUND, low_priority )]
unsafe fn richter_attack12f_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_punch_kick_swing_l"));
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_richter_rnd_attack"));
    }
}

#[acmd_script( agent = "richter", script = "expression_attack12f", category = ACMD_EXPRESSION, low_priority )]
unsafe fn richter_attack12f_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        slope!(agent, MA_MSC_CMD_SLOPE_SLOPE_INTP, SLOPE_STATUS_TOP, 5);
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohitl"),
            4,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
    frame(agent.lua_state_agent, 33.0);
    if macros::is_excute(agent) {
        slope!(agent, MA_MSC_CMD_SLOPE_SLOPE_INTP, SLOPE_STATUS_LR, 5);
    }
}

#[acmd_script( agent = "richter", script = "game_attackdash", category = ACMD_GAME, low_priority )]
unsafe fn richter_attackdash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.battle_object, attack_dash::flag::ENABLE_AIR_FALL);
        VarModule::on_flag(agent.battle_object, attack_dash::flag::ENABLE_AIR_CONTINUE);
        VarModule::set_float(agent.battle_object, attack_dash::float::FALL_SPEED_Y_MUL, 0.0);
    }
    for _ in 0..5 {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 10, 10, 0, 55, 4.0, 0.0, 13.0, 6.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 367, 10, 0, 70, 4.0, 0.0, 13.0, -6.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 40, 10, 0, 50, 4.0, 0.0, 4.0, 6.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
            macros::ATTACK(agent, 3, 0, Hash40::new("top"), 2.0, 367, 10, 0, 70, 4.0, 0.0, 4.0, -6.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        wait(agent.lua_state_agent, 1.0);
    }
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.5, 47, 105, 0, 80, 11.5, 0.0, 8.8, 0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        VarModule::off_flag(agent.battle_object, attack_dash::flag::ENABLE_AIR_FALL);
        VarModule::off_flag(agent.battle_object, attack_dash::flag::ENABLE_AIR_CONTINUE);
        VarModule::set_float(agent.battle_object, attack_dash::float::FALL_SPEED_Y_MUL, 1.0);
        VarModule::on_flag(agent.battle_object, attack_dash::flag::ENABLE_GRAVITY);
    }
}

#[acmd_script( agent = "richter", script = "game_attacks3", category = ACMD_GAME, low_priority )]
unsafe fn richter_attacks3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 5.0 / 3.0);
    frame(agent.lua_state_agent, 4.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::SEARCH(agent, 0, 0, Hash40::new("top"), 2.5, 0.0, 10.0, 5.0, Some(0.0), Some(10.0), Some(31.0), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_IG, *COLLISION_PART_MASK_ALL, false);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        agent.clear_lua_stack();
        let object = sv_system::battle_object(agent.lua_state_agent) as *mut BattleObject;
        if !object.is_null() {
            FighterSpecializer_Simon::set_whip_reflect_attack_off_id(
                object as *mut Fighter,
                0,
                1,
                -1,
                -1,
                -1,
                -1,
                -1,
                -1,
                -1,
                -1
            );
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 361, 65, 0, 65, 2.5, 0.0, 10.0, 31.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 361, 65, 0, 65, 2.5, 0.0, 10.0, 7.0, Some(0.0), Some(10.0), Some(31.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 361, 65, 0, 65, 4.5, 0.0, 10.0, 7.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        search!(agent, MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SIMON_STATUS_ATTACK_FLAG_ENABLE_HOLD);
    }
}

#[acmd_script( agent = "richter", script = "effect_attacks3", category = ACMD_EFFECT, low_priority )]
unsafe fn richter_attacks3_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("richter_whip_straight"), Hash40::new("haver"), 0, 0, 0, 4, 30, 4, 0.8, true);
    }
}

#[acmd_script( agent = "richter_whip", script = "game_attacks3", category = ACMD_GAME, low_priority )]
unsafe fn richter_whip_attacks3(weapon: &mut L2CAgentBase) {
    frame(weapon.lua_state_agent, 1.0);
    if macros::is_excute(weapon) {
        PhysicsModule::set_2nd_status(weapon.module_accessor, *PH2NDARY_CRAW_NONE);
        weapon.clear_lua_stack();
        let object = sv_system::battle_object(weapon.lua_state_agent) as *mut BattleObject;
        if !object.is_null() {
            WeaponSpecializer_SimonWhip::reset_node_fix_flag_list(
                object as *mut smash::app::Weapon
            );
        }
    }
    macros::FT_MOTION_RATE(weapon, 5.0 / 3.0);
    frame(weapon.lua_state_agent, 4.0);
    macros::FT_MOTION_RATE(weapon, 1.0);
    frame(weapon.lua_state_agent, 11.0);
    if macros::is_excute(weapon) {
        PhysicsModule::set_2nd_status(weapon.module_accessor, *PH2NDARY_CRAW_COLLIDE);
    }
    frame(weapon.lua_state_agent, 23.0);
    if macros::is_excute(weapon) {
        PhysicsModule::set_2nd_status(weapon.module_accessor, *PH2NDARY_CRAW_MOVE);
    }
}

#[acmd_script( agent = "richter", script = "game_attackhi3", category = ACMD_GAME, low_priority )]
unsafe fn richter_attackhi3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 2.0);
    frame(agent.lua_state_agent, 6.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 367, 57, 10, 0, 5.0, 0.0, 25.5, 3.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 367, 57, 10, 0, 4.0, 0.0, 25.5, -4.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 367, 57, 10, 0, 4.0, 0.0, 25.5, 10.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 2.0, 367, 57, 10, 0, 4.5, 0.0, 17.5, 3.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 87, 57, 0, 88, 5.0, 0.0, 25.5, 3.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 87, 57, 0, 88, 4.0, 0.0, 25.5, -4.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 5.0, 87, 57, 0, 88, 4.0, 0.0, 25.5, 10.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 5.0, 87, 57, 0, 88, 4.5, 0.0, 17.5, 3.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

#[acmd_script( agent = "richter_whip", script = "game_attackhi3", category = ACMD_GAME, low_priority )]
unsafe fn richter_whip_attackhi3(weapon: &mut L2CAgentBase) {
    frame(weapon.lua_state_agent, 1.0);
    if macros::is_excute(weapon) {
        PhysicsModule::set_2nd_status(weapon.module_accessor, *PH2NDARY_CRAW_NONE);
    }
    macros::FT_MOTION_RATE(weapon, 2.0);
    frame(weapon.lua_state_agent, 6.0);
    macros::FT_MOTION_RATE(weapon, 1.0);
    frame(weapon.lua_state_agent, 31.0);
    if macros::is_excute(weapon) {
        PhysicsModule::set_2nd_status(weapon.module_accessor, *PH2NDARY_CRAW_COLLIDE);
    }
    frame(weapon.lua_state_agent, 32.0);
    if macros::is_excute(weapon) {
        PhysicsModule::set_2nd_status(weapon.module_accessor, *PH2NDARY_CRAW_MOVE);
        weapon.clear_lua_stack();
        let object = sv_system::battle_object(weapon.lua_state_agent) as *mut BattleObject;
        if !object.is_null() {
            WeaponSpecializer_SimonWhip::reset_node_fix_flag_list(
                object as *mut smash::app::Weapon
            );
            WeaponSpecializer_SimonWhip::set_node_fix_flag_list(
                object as *mut smash::app::Weapon,
                4,
                5,
                9,
                10,
                -1,
                -1,
                -1,
                -1,
                -1,
                -1,
                -1,
                -1,
                -1,
                -1,
                -1,
                -1,
                -1,
                -1,
                -1,
                -1,
                -1,
                -1,
                -1,
                -1,
                -1,
                -1,
                -1,
                -1,
                -1,
                -1
            );
        }
    }
}

#[acmd_script( agent = "richter", script = "game_attacklw3", category = ACMD_GAME, low_priority )]
unsafe fn richter_attacklw3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SIMON_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("kneel"), 5.0, 40, 20, 0, 65, 3.0, 0.0, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("footl"), 5.0, 40, 20, 0, 55, 2.9, -1.5, 0.0, -1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_SIMON_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
}

#[acmd_script( agent = "richter", script = "game_attacklw32", category = ACMD_GAME, low_priority )]
unsafe fn richter_attacklw32(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SIMON_STATUS_ATTACK_LW32_WORK_ID_FLAG_LANDING_AIR);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("kneel"), 1.0, 366, 55, 10, 60, 3.5, 0.0, 1.0, -1.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("footl"), 1.0, 366, 55, 10, 60, 3.0, 1.5, 1.5, -2.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("hip"), 1.0, 366, 55, 10, 60, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        macros::ATTACK(agent, 0, 0, Hash40::new("kneel"), 3.5, 50, 40, 0, 60, 2.5, 0.0, 1.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("footl"), 3.5, 50, 40, 0, 60, 2.5, 1.5, 0.0, -1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("hip"), 3.5, 50, 40, 0, 60, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        JostleModule::set_status(agent.module_accessor, true);
    }
}

pub fn install() {
    install_acmd_scripts!(
        richter_attack11, richter_attack11_eff, richter_attack11_snd, richter_attack11_exp,
        richter_attack12, richter_attack12_eff, richter_attack12_snd, richter_attack12_exp,
        richter_attack12f, richter_attack12f_eff, richter_attack12f_snd, richter_attack12f_exp,
        richter_attackdash,
        richter_attacks3, richter_attacks3_eff,
        richter_whip_attacks3,
        richter_attackhi3,
        richter_whip_attackhi3,
        richter_attacklw3,
        richter_attacklw32
    );
}