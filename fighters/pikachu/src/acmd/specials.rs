use super::*;

unsafe extern "C" fn game_specialn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 4.0 / 3.0);
    frame(agent.lua_state_agent, 13.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 38, 100, 70, 0, 3.0, 0.0, 5.0, 16.0, Some(0.0), Some(5.0), Some(6.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SLAP, *ATTACK_REGION_TAIL);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 8.0, false);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_PIKACHU_GENERATE_ARTICLE_DENGEKIDAMA, false, -1);
    }
}

unsafe extern "C" fn effect_specialn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pikachu_cheek"), Hash40::new("head"), 0, 0, 0, 0, -90, -90, 1, true);
        let lr = PostureModule::lr(agent.module_accessor);
        macros::EFFECT_FOLLOW(agent, Hash40::new("pikachu_dengeki"), Hash40::new("tail3"), 0, -3.0 * lr, 0, 0, 0, 0, 0.8, true);
    }
    for _ in 0..3 {
        if macros::is_excute(agent) {
            macros::FLASH(agent, 0, 0, 1, 0.196);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::FLASH(agent, 0, 0, 1, 0.353);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::COL_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 1.0);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("pikachu_dengeki"), true, true);
    }
}

unsafe extern "C" fn effect_specialairn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pikachu_cheek"), Hash40::new("head"), 0, 0, 0, 0, -90, -90, 1, true);
        let lr = PostureModule::lr(agent.module_accessor);
        macros::EFFECT_FOLLOW(agent, Hash40::new("pikachu_dengeki"), Hash40::new("tail3"), 0, -3.0 * lr, 0, 0, 0, 0, 0.8, true);
    }
    for _ in 0..3 {
        if macros::is_excute(agent) {
            macros::FLASH(agent, 0, 0, 1, 0.196);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::FLASH(agent, 0, 0, 1, 0.353);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::COL_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 1.0);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("pikachu_dengeki"), true, true);
    }
}

unsafe extern "C" fn sound_specialn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_pikachu_002"));
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pikachu_special_n02"));
    }
}

unsafe extern "C" fn expression_specialn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        slope!(agent, MA_MSC_CMD_SLOPE_SLOPE_INTP, SLOPE_STATUS_TOP, 3);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_attackm"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
    }
}

// Actually Side Speceial Lmao

unsafe extern "C" fn game_specialhistart(agent: &mut L2CAgentBase) {
    MiscModule::calc_motion_rate_from_end_frame(agent, 0.0, 4.0);
}

unsafe extern "C" fn game_specialhi1(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("neck"), 2.0, 70, 50, 0, 20, 1.6, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
        JostleModule::set_status(agent.module_accessor, false);
    }
}

unsafe extern "C" fn game_specialhi2(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("neck"), 3.0, 70, 95, 0, 75, 1.6, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
        JostleModule::set_status(agent.module_accessor, false);
    }
}

// Actually Up Special Lmao

unsafe extern "C" fn game_speciallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 10.0 / 6.0);
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 90, 60, 100, 60, 7.0, 0.0, 3.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
    }
    macros::FT_MOTION_RATE(agent, 0.5);
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        sv_kinetic_energy!(
            set_speed_mul,
            agent,
            FIGHTER_KINETIC_ENERGY_ID_MOTION,
            1.2,
            1.2
        );
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 90, 60, 100, 10, 7.0, 0.0, 3.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 21.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
    }
    frame(agent.lua_state_agent, 28.0);
    macros::FT_MOTION_RATE(agent, 3.0);
    frame(agent.lua_state_agent, 30.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 33.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 368, 60, 0, 55, 12.0, 0.0, 3.0, 0.0, None, None, None, 0.25, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
        AttackModule::set_vec_target_pos(agent.module_accessor, 0, Hash40::new("top"), &Vector2f{x: 0.0, y: -20.0}, 6, false);
        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
    }
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_PIKACHU_GENERATE_ARTICLE_CLOUD, false, -1);
    }
    frame(agent.lua_state_agent, 56.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
    }
}

unsafe extern "C" fn effect_speciallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pikachu_cheek"), Hash40::new("head"), 0, 0, 0, 0, -90, -90, 1, true);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pikachu_elec"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 1.0, true);
    }
    frame(agent.lua_state_agent, 33.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("pikachu_elec"), false, false);
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pikachu_kaminari_hit2"), Hash40::new("top"), 0, -2, 0, 0, 90, 0, 0.7, true);
    }
    for _ in 0..4 {
        if macros::is_excute(agent) {
            macros::FLASH(agent, 1, 1, 1, 0.314);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::FLASH(agent, 0, 0, 0, 0.314);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::COL_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 1.0);
    }
    frame(agent.lua_state_agent, 46.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("pikachu_kaminari_hit2"), true, true);
    }
}

unsafe extern "C" fn sound_speciallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pikachu_special_s01"));
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_pikachu_001"));
    }
    frame(agent.lua_state_agent, 33.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pikachu_special_l01"));
    }
}

unsafe extern "C" fn expression_speciallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
    }
    frame(agent.lua_state_agent, 33.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
    }
}

// Down Special For Real Lmao

unsafe extern "C" fn game_speciallwstrike(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, vars::pikachu::status::flag::SPECIAL_LW_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 277, 30, 0, 75, 4.0, 0.0, -3.0, 5.0, Some(0.0), Some(7.0), Some(12.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_TAIL);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 15.0, 277, 30, 0, 75, 3.0, 0.0, 2.0, 1.0, Some(0.0), Some(7.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_TAIL);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 15.0, 290, 45, 0, 40, 4.0, 0.0, -3.0, 5.0, Some(0.0), Some(7.0), Some(12.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_TAIL);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 15.0, 290, 45, 0, 40, 3.0, 0.0, 2.0, 1.0, Some(0.0), Some(7.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_TAIL);
        macros::ATTACK(agent, 4, 0, Hash40::new("top"), 15.0, 290, 45, 0, 40, 4.0, 0.0, 17.0, 8.0, Some(0.0), Some(7.0), Some(12.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_TAIL);
        macros::ATTACK(agent, 5, 0, Hash40::new("top"), 15.0, 290, 45, 0, 40, 3.0, 0.0, 12.0, 3.0, Some(0.0), Some(7.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_TAIL);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, vars::pikachu::status::flag::SPECIAL_LW_ENABLE_GRAVITY);
    }
}

unsafe extern "C" fn effect_speciallwstrike(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 6.0, 0, 0, 0, -90, 0.6, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_RATE(agent, 2.0);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 6.0, 0, 0, 0, -90, 0.8, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_RATE(agent, 2.0);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 6.0, 0, 0, 0, -90, 1.0, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_RATE(agent, 2.0);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("pikachu_tail_arc2"), Hash40::new("pikachu_tail_arc2"), Hash40::new("top"), 1, 9, 2, 0, 0, -90, 0.95, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_RATE(agent, 1.25);
    }
}

unsafe extern "C" fn sound_speciallwstrike(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_swing_02"));
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_swing_02"));
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pikachu_swing_s"));
    }
}

unsafe extern "C" fn expression_speciallwstrike(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohits"),
            7,
            true,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohits"),
            7,
            true,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohits"),
            7,
            true,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialn", game_specialn, Priority::Low);
    agent.acmd("effect_specialn", effect_specialn, Priority::Low);
    agent.acmd("sound_specialn", sound_specialn, Priority::Low);
    agent.acmd("expression_specialn", expression_specialn, Priority::Low);

    agent.acmd("game_specialairn", game_specialn, Priority::Low);
    agent.acmd("effect_specialairn", effect_specialairn, Priority::Low);
    agent.acmd("sound_specialairn", sound_specialn, Priority::Low);
    agent.acmd("expression_specialairn", expression_specialn, Priority::Low);

    agent.acmd("game_specialhistart", game_specialhistart, Priority::Low);

    agent.acmd("game_specialairhistart", game_specialhistart, Priority::Low);

    agent.acmd("game_specialhi1", game_specialhi1, Priority::Low);

    agent.acmd("game_specialairhi1", game_specialhi1, Priority::Low);

    agent.acmd("game_specialhi2", game_specialhi2, Priority::Low);

    agent.acmd("game_specialairhi2", game_specialhi2, Priority::Low);

    agent.acmd("game_speciallw", game_speciallw, Priority::Low);
    agent.acmd("effect_speciallw", effect_speciallw, Priority::Low);
    agent.acmd("sound_speciallw", sound_speciallw, Priority::Low);
    agent.acmd("expression_speciallw", expression_speciallw, Priority::Low);

    agent.acmd("game_specialairlw", game_speciallw, Priority::Low);
    agent.acmd("effect_specialairlw", effect_speciallw, Priority::Low);
    agent.acmd("sound_specialairlw", sound_speciallw, Priority::Low);
    agent.acmd("expression_specialairlw", expression_speciallw, Priority::Low);

    agent.acmd("game_speciallwstrike", game_speciallwstrike, Priority::Low);
    agent.acmd("effect_speciallwstrike", effect_speciallwstrike, Priority::Low);
    agent.acmd("sound_speciallwstrike", sound_speciallwstrike, Priority::Low);
    agent.acmd("expression_speciallwstrike", expression_speciallwstrike, Priority::Low);
}