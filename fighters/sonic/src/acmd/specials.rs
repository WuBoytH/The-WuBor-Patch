use super::*;

unsafe extern "C" fn game_specialnhit(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        KineticModule::add_speed(agent.module_accessor, &Vector3f{x: -0.7, y: 0.0, z: 0.0});
    }
    macros::FT_MOTION_RATE(agent, 0.6);
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, vars::sonic::status::flag::ENABLE_TRICK);
    }
}

unsafe extern "C" fn game_specials(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 60, 60, 0, 50, 4.0, 0.0, 6.0, 1.0, None, None, None, 0.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, -10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, true);
    }
    MiscModule::calc_motion_rate_from_cancel_frame(agent, 36.0, 6.0);
}

unsafe extern "C" fn effect_specials(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    for _ in 0..2 {
        if macros::is_excute(agent) {
            macros::FLASH(agent, 0.7, 0.7, 1.0, 1.0);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::FLASH(agent, 0.55, 0.55, 0.78, 1.0);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::COL_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 2.0);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("sonic_spintrace"), Hash40::new("top"), 0, 6, 0, 0, 0, 0, 1.25, true);
    }
    wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sonic_spintrace"), false, false);
    }
}

unsafe extern "C" fn sound_specials(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_sonic_special_s03"));
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_sonic_special_s01"));
    }
}

unsafe extern "C" fn expression_specials(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

unsafe extern "C" fn effect_specialairsstart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    for _ in 0..2 {
        if macros::is_excute(agent) {
            macros::FLASH(agent, 0.7, 0.7, 1.0, 1.0);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::FLASH(agent, 0.55, 0.55, 0.78, 1.0);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::COL_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 2.0);
    }
}

unsafe extern "C" fn sound_specialairsstart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_sonic_special_s03"));
    }
}

unsafe extern "C" fn game_specialairshold(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    game_specialairshold_inner(agent);
    frame(agent.lua_state_agent, 9.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    game_specialairshold_inner(agent);
    frame(agent.lua_state_agent, 15.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    game_specialairshold_inner(agent);
}

unsafe extern "C" fn game_specialairshold_inner(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        let last = VarModule::get_int(agent.module_accessor, vars::sonic::status::int::SPECIAL_AIR_S_HOLD_COUNT_REMAIN) == 1;
        let status = StatusModule::status_kind(agent.module_accessor) == vars::sonic::status::SPECIAL_AIR_S_END;
        if status
        && last {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 85, 100, 30, 30, 4.0, 0.0, 8.0, 6.0, Some(0.0), Some(8.0), Some(7.0), 0.5, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, -5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
        else {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 368, 100, 10, 30, 4.0, 0.0, 8.0, 6.0, Some(0.0), Some(8.0), Some(7.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_vec_target_pos(agent.module_accessor, 0, Hash40::new("top"), &Vector2f{x: 10.0, y: 3.0}, 6, false);
        }
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, vars::sonic::status::flag::SPECIAL_AIR_S_CHECK_END);
    }
    macros::FT_MOTION_RATE(agent, 1.0 / 3.0);
}

unsafe extern "C" fn effect_specialairshold(agent: &mut L2CAgentBase) {
    if VarModule::is_flag(agent.module_accessor, vars::sonic::status::flag::SPECIAL_AIR_S_FIRST) {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("sonic_spintrace"), Hash40::new("top"), 0, 7, 0, 0, 0, 0, 1.25, true);
        }
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 3, 7.8, -2, 30, 0, 0, 0.9, true, *EF_FLIP_YZ);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 3, 7.8, -2, 0, 0, 0, 0.9, true, *EF_FLIP_YZ);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 3, 7.8, -2, -30, 0, 0, 0.9, true, *EF_FLIP_YZ);
    }
}

unsafe extern "C" fn sound_specialairshold(agent: &mut L2CAgentBase) {
    if VarModule::is_flag(agent.module_accessor, vars::sonic::status::flag::SPECIAL_AIR_S_FIRST) {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("vc_sonic_attack06"));
            macros::PLAY_SE(agent, Hash40::new("se_sonic_special_s01"));
        }
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_sonic_swing_s"));
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_sonic_swing_s"));
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_sonic_swing_s"));
    }
}

unsafe extern "C" fn expression_specialairshold(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
    }
}

unsafe extern "C" fn game_specialairslaunch(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 6.0 / 11.0);
    frame(agent.lua_state_agent, 12.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        KineticModule::enable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        macros::SET_SPEED_EX(agent, 0.8, 1.6, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 40, 110, 0, 45, 5.0, 0.0, 8.0, 5.0, Some(0.0), Some(15.0), Some(5.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, -5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        VarModule::on_flag(agent.module_accessor, vars::sonic::status::flag::ENABLE_TRICK);
    }
    MiscModule::calc_motion_rate_from_cancel_frame(agent, 18.0, -8.0);
}

unsafe extern "C" fn effect_specialairslaunch(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 11, 2, 0, 0, 90, 0.92, true, *EFFECT_FLIP_YZ);
        macros::LAST_EFFECT_SET_RATE(agent, 2.0);
    }
}

unsafe extern "C" fn sound_specialairslaunch(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_sonic_attack07"));
        macros::PLAY_SE(agent, Hash40::new("se_sonic_swing_l"));
    }
}

unsafe extern "C" fn expression_specialairslaunch(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
    }
}

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::shoot_exist(agent.module_accessor, *FIGHTER_SONIC_GENERATE_ARTICLE_GIMMICKJUMP, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP); // Was ALWAYS
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, vars::sonic::status::flag::ENABLE_TRICK);
    }
}

unsafe extern "C" fn game_specialairlw2start(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 10.0 / 4.0);
    frame(agent.lua_state_agent, 5.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        MotionModule::set_rate(agent.module_accessor, 0.5);
        VarModule::on_flag(agent.module_accessor, vars::sonic::status::flag::SPECIAL_AIR_LW_TO_LOOP);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("sphere"), 8.0, 80, 40, 0, 95, 5.0, 0.0, 1.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
}

unsafe extern "C" fn effect_specialairlw2start(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    for _ in 0..2 {
        if macros::is_excute(agent) {
            macros::FLASH(agent, 1.0, 1.0, 0.5, 1.0);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::FLASH(agent, 0.77, 0.77, 0.22, 1.0);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::COL_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 1.0);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sonic_spinblur_max"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1, false);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sonic_spintrace_max"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1, false);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
}

unsafe extern "C" fn sound_specialairlw2start(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_sonic_special_s03"));
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_sonic_attack05"));
        macros::PLAY_SE(agent, Hash40::new("se_sonic_special_s01"));
    }
}

unsafe extern "C" fn expression_specialairlw2start(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("body") as i64, hash40("body_sphere") as i64);
        macros::HIT_NODE(agent, Hash40::new("waist"), *HIT_STATUS_OFF);
        macros::HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_OFF);
        macros::HIT_NODE(agent, Hash40::new("s_stingd1"), *HIT_STATUS_OFF);
        macros::HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_OFF);
        macros::HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_OFF);
        macros::HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_OFF);
        macros::HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_OFF);
        macros::HIT_NODE(agent, Hash40::new("legr"), *HIT_STATUS_OFF);
        macros::HIT_NODE(agent, Hash40::new("legl"), *HIT_STATUS_OFF);
        macros::HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_OFF);
        macros::HIT_NODE(agent, Hash40::new("kneel"), *HIT_STATUS_OFF);
        macros::HIT_NODE(agent, Hash40::new("footr"), *HIT_STATUS_OFF);
        macros::HIT_NODE(agent, Hash40::new("footl"), *HIT_STATUS_OFF);
        macros::HIT_NODE(agent, Hash40::new("rot"), *HIT_STATUS_NORMAL);
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
    }
}

unsafe extern "C" fn game_specialairlw2bound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, vars::sonic::status::flag::SPECIAL_AIR_LW_BOUND_START);
    }
    frame(agent.lua_state_agent, 10.0);
    if VarModule::is_flag(agent.module_accessor, vars::sonic::status::flag::SPECIAL_AIR_LW_HIT) {
        if macros::is_excute(agent) {
            VarModule::on_flag(agent.module_accessor, vars::sonic::status::flag::ENABLE_TRICK);
        }
    }
}

unsafe extern "C" fn effect_specialairlw2bound(agent: &mut L2CAgentBase) {
    if VarModule::is_flag(agent.module_accessor, vars::sonic::status::flag::SPECIAL_AIR_LW_BOUND_IS_GROUND) {
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
}

unsafe extern "C" fn sound_specialairlw2bound(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_sonic_landing03"));
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_sonic_jump01"));
    }
}

unsafe extern "C" fn expression_specialairlw2bound(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        VisibilityModule::set_int64(agent.module_accessor, hash40("body") as i64, hash40("body_sphere") as i64);
        macros::HIT_NODE(agent, Hash40::new("waist"), *HIT_STATUS_OFF);
        macros::HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_OFF);
        macros::HIT_NODE(agent, Hash40::new("s_stingd1"), *HIT_STATUS_OFF);
        macros::HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_OFF);
        macros::HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_OFF);
        macros::HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_OFF);
        macros::HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_OFF);
        macros::HIT_NODE(agent, Hash40::new("legr"), *HIT_STATUS_OFF);
        macros::HIT_NODE(agent, Hash40::new("legl"), *HIT_STATUS_OFF);
        macros::HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_OFF);
        macros::HIT_NODE(agent, Hash40::new("kneel"), *HIT_STATUS_OFF);
        macros::HIT_NODE(agent, Hash40::new("footr"), *HIT_STATUS_OFF);
        macros::HIT_NODE(agent, Hash40::new("footl"), *HIT_STATUS_OFF);
        macros::HIT_NODE(agent, Hash40::new("rot"), *HIT_STATUS_NORMAL);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("body") as i64, hash40("body_normal") as i64);
        macros::HIT_NODE(agent, Hash40::new("waist"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("s_stingd1"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("legr"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("legl"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("kneel"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("footr"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("footl"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("rot"), *HIT_STATUS_OFF);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialnhit", game_specialnhit, Priority::Low);

    agent.acmd("game_specials", game_specials, Priority::Low);
    agent.acmd("effect_specials", effect_specials, Priority::Low);
    agent.acmd("sound_specials", sound_specials, Priority::Low);
    agent.acmd("expression_specials", expression_specials, Priority::Low);

    agent.acmd("effect_specialairsstart", effect_specialairsstart, Priority::Low);
    agent.acmd("sound_specialairsstart", sound_specialairsstart, Priority::Low);

    agent.acmd("game_specialairshold", game_specialairshold, Priority::Low);
    agent.acmd("effect_specialairshold", effect_specialairshold, Priority::Low);
    agent.acmd("sound_specialairshold", sound_specialairshold, Priority::Low);
    agent.acmd("expression_specialairshold", expression_specialairshold, Priority::Low);

    agent.acmd("game_specialairslaunch", game_specialairslaunch, Priority::Low);
    agent.acmd("effect_specialairslaunch", effect_specialairslaunch, Priority::Low);
    agent.acmd("sound_specialairslaunch", sound_specialairslaunch, Priority::Low);
    agent.acmd("expression_specialairslaunch", expression_specialairslaunch, Priority::Low);

    agent.acmd("game_specialhi", game_specialhi, Priority::Low);

    agent.acmd("game_specialairlw2start", game_specialairlw2start, Priority::Low);
    agent.acmd("effect_specialairlw2start", effect_specialairlw2start, Priority::Low);
    agent.acmd("sound_specialairlw2start", sound_specialairlw2start, Priority::Low);
    agent.acmd("expression_specialairlw2start", expression_specialairlw2start, Priority::Low);

    agent.acmd("game_specialairlw2bound", game_specialairlw2bound, Priority::Low);
    agent.acmd("effect_specialairlw2bound", effect_specialairlw2bound, Priority::Low);
    agent.acmd("sound_specialairlw2bound", sound_specialairlw2bound, Priority::Low);
    agent.acmd("expression_specialairlw2bound", expression_specialairlw2bound, Priority::Low);
}