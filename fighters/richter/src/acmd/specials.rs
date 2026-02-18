use super::*;

unsafe extern "C" fn game_specialn(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SIMON_STATUS_SPECIAL_LW_FLAG_GENERATE_HOLYWATER);
    }
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 12.0 / 8.0);
    frame(agent.lua_state_agent, 9.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SIMON_STATUS_SPECIAL_LW_FLAG_SHOOT_HOLYWATER);
    }
}

unsafe extern "C" fn effect_specialn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("richter_bottle_appear"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.4, true);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("richter_bottle_release"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn effect_specialairn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("richter_bottle_appear"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.4, true);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("richter_bottle_release"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_specialn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        let kind = VarModule::get_int(agent.module_accessor, vars::richter::instance::int::SPECIAL_N_VOICE_KIND);
        let voice = match kind {
            0 => Hash40::new("vc_richter_special_l01"),
            1 => Hash40::new("vc_richter_special_n01"),
            2 => Hash40::new("vc_richter_special_s01"),
            3 => Hash40::new("vc_richter_special_s02"),
            _ => unreachable!()
        };
        let next = if kind + 1 <= 3 {
            kind + 1
        }
        else {
            0
        };
        VarModule::set_int(agent.module_accessor, vars::richter::instance::int::SPECIAL_N_VOICE_KIND, next);
        macros::PLAY_SE(agent, voice);
        macros::PLAY_SE(agent, Hash40::new("se_richter_special_l01"));
    }
}

unsafe extern "C" fn expression_specialn(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_lightthrowitem"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn effect_specialnblank(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("richter_bottle_blank"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_specialnblank(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_richter_special_l04"));
    }
}

unsafe extern "C" fn expression_specialnblank(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_specials(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, vars::richter::status::flag::SPECIAL_S_START_DASH);
    }
    for x in 0..5 {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.5, 10, 10, 0, 55, 4.0, 0.0, 13.0, 6.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.5, 367, 10, 0, 70, 4.0, 0.0, 13.0, -6.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 40, 10, 0, 50, 4.0, 0.0, 4.0, 6.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
            macros::ATTACK(agent, 3, 0, Hash40::new("top"), 1.5, 367, 10, 0, 70, 4.0, 0.0, 4.0, -6.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            if x == 3 {
                VarModule::on_flag(agent.module_accessor, vars::richter::status::flag::SPECIAL_S_BRAKE_SHIFT);
            }
        }
        wait(agent.lua_state_agent, 1.0);
    }
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.5, 47, 90, 0, 70, 9.5, 0.0, 8.8, -1.5, Some(0.0), Some(8.8), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        VarModule::on_flag(agent.module_accessor, vars::richter::status::flag::SPECIAL_S_ENABLE_FALL);
        VarModule::on_flag(agent.module_accessor, vars::richter::status::flag::SPECIAL_S_ENABLE_LANDING);
    }
    wait(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
}

unsafe extern "C" fn effect_specials(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLW_POS(agent, Hash40::new("richter_whip_dash"), Hash40::new("top"), -2.4, 9, 1, 0, 0, 0, 1, true);
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 8, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 8, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 8, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn effect_specialairs(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLW_POS(agent, Hash40::new("richter_whip_dash"), Hash40::new("top"), -2.4, 9, 1, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_specials(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_richter_whip_holding"));
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_richter_rnd_attack"));
        macros::PLAY_SE(agent, Hash40::new("se_richter_attackdash"));
    }
}

unsafe extern "C" fn expression_specials(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_rush"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackss"), 3);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        GroundModule::select_cliff_hangdata(agent.module_accessor, 9);
    }
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 7.0 / 4.0);
    frame(agent.lua_state_agent, 5.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 95, 100, 155, 0, 8.5, 0.0, 9.5, 10.5, None, None, None, 1.4, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
        AttackModule::set_no_damage_fly_smoke_all(agent.module_accessor, true, false);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE);
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 367, 100, 80, 0, 6.0, 0.0, 26.0, 9.5, Some(0.0), Some(6.0), Some(7.5), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
        AttackModule::set_no_damage_fly_smoke_all(agent.module_accessor, true, false);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 0, false);
        macros::ATTACK(agent, 1, 1, Hash40::new("top"), 6.0, 45, 30, 0, 85, 6.0, 0.0, 26.0, 9.5, Some(0.0), Some(6.0), Some(7.5), 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 1, 3.0, false);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        VarModule::on_flag(agent.module_accessor, vars::fighter::instance::flag::DISABLE_SPECIAL_HI);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SIMON_STATUS_SPECIAL_HI_FLAG_MOVE);
    }
}

unsafe extern "C" fn game_speciallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("kneel"), 5.0, 50, 40, 0, 50, 3.0, -0.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("footl"), 5.0, 50, 40, 0, 50, 2.9, -2.5, 0.0, -1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("hip"), 5.0, 50, 40, 0, 50, 3.1, 0.0, 0.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 2, false);
        VarModule::on_flag(agent.module_accessor, vars::richter::status::flag::SPECIAL_LW_SLIDE_ENABLE_JUMP);
        GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        VarModule::off_flag(agent.module_accessor, vars::richter::status::flag::SPECIAL_LW_SLIDE_ENABLE_JUMP);
        GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
    }
}

unsafe extern "C" fn effect_speciallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 8, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 0.7);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_sliding_smoke"), Hash40::new("top"), 7, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    wait(agent.lua_state_agent, 5.0);
}

unsafe extern "C" fn sound_speciallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_richter_attackhard_l01"));
    }
}

unsafe extern "C" fn expression_speciallw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 5);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 2, true);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 9, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_dash"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 6);
    }
}

// Slide Jump

unsafe extern "C" fn game_attacklw32(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 8.0 / 5.0);
    frame(agent.lua_state_agent, 6.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SIMON_STATUS_ATTACK_LW32_WORK_ID_FLAG_LANDING_AIR);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("kneel"), 7.0, 72, 20, 0, 90, 3.5, 0.0, 1.0, -1.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("footl"), 7.0, 72, 20, 0, 90, 3.0, -1.5, 1.5, -2.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("hip"), 7.0, 72, 20, 0, 90, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("kneel"), 3.5, 72, 20, 0, 90, 2.5, 0.0, 1.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("footl"), 3.5, 72, 20, 0, 90, 2.5, -1.5, 0.0, -1.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::clear(agent.module_accessor, 2, false);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        JostleModule::set_status(agent.module_accessor, true);
    }
}

unsafe extern "C" fn game_specialairlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    macros::FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 10.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, vars::richter::status::flag::SPECIAL_LW_DIVE);
        VarModule::on_flag(agent.module_accessor, vars::richter::status::flag::SPECIAL_LW_DIVE_ENABLE_LANDING);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 72, 20, 0, 90, 3.5, 0.0, 0.5, 2.5, Some(0.0), Some(0.5), Some(2.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 7.0, 72, 20, 0, 90, 4.0, 0.0, 3.5, -0.5, Some(0.0), Some(3.5), Some(-0.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        JostleModule::set_status(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 50.0);
    if macros::is_excute(agent) {
        VarModule::off_flag(agent.module_accessor, vars::richter::status::flag::SPECIAL_LW_DIVE_ENABLE_LANDING);
        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
}

unsafe extern "C" fn effect_specialairlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, -2.5, 4, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("richter_air_lw"), Hash40::new("top"), 0, -0.5, 2, -27, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
}

unsafe extern "C" fn sound_specialairlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_richter_attack04"));
        macros::PLAY_SE(agent, Hash40::new("se_richter_attackair_l01"));
    }
}

unsafe extern "C" fn expression_specialairlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

unsafe extern "C" fn effect_specialairlwlanding(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 1.2);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_sliding_smoke"), Hash40::new("top"), 7, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_sliding_smoke"), Hash40::new("top"), 10, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 12, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_specialairlwlanding(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_richter_attackair_l01"));
        macros::PLAY_SE(agent, Hash40::new("se_richter_attackair_l02"));
    }
}

unsafe extern "C" fn expression_specialairlwlanding(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_landl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_dash"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

// Special Lw Bounce

unsafe extern "C" fn game_attackairlw2(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 1.0);
    MiscModule::calc_motion_rate_from_cancel_frame(agent, 1.0, -30.0);
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

    agent.acmd("effect_specialnblank", effect_specialnblank, Priority::Low);
    agent.acmd("sound_specialnblank", sound_specialnblank, Priority::Low);
    agent.acmd("expression_specialnblank", expression_specialnblank, Priority::Low);

    agent.acmd("effect_specialairnblank", effect_specialnblank, Priority::Low);
    agent.acmd("sound_specialairnblank", sound_specialnblank, Priority::Low);
    agent.acmd("expression_specialairnblank", expression_specialnblank, Priority::Low);

    agent.acmd("game_specials", game_specials, Priority::Low);
    agent.acmd("effect_specials", effect_specials, Priority::Low);
    agent.acmd("sound_specials", sound_specials, Priority::Low);
    agent.acmd("expression_specials", expression_specials, Priority::Low);

    agent.acmd("game_specialairs", game_specials, Priority::Low);
    agent.acmd("effect_specialairs", effect_specialairs, Priority::Low);
    agent.acmd("sound_specialairs", sound_specials, Priority::Low);
    agent.acmd("expression_specialairs", expression_specials, Priority::Low);

    agent.acmd("game_specialhi", game_specialhi, Priority::Low);

    agent.acmd("game_specialairhi", game_specialhi, Priority::Low);

    agent.acmd("game_speciallw", game_speciallw, Priority::Low);
    agent.acmd("effect_speciallw", effect_speciallw, Priority::Low);
    agent.acmd("sound_speciallw", sound_speciallw, Priority::Low);
    agent.acmd("expression_speciallw", expression_speciallw, Priority::Low);

    agent.acmd("game_attacklw32", game_attacklw32, Priority::Low);

    agent.acmd("game_specialairlw", game_specialairlw, Priority::Low);
    agent.acmd("effect_specialairlw", effect_specialairlw, Priority::Low);
    agent.acmd("sound_specialairlw", sound_specialairlw, Priority::Low);
    agent.acmd("expression_specialairlw", expression_specialairlw, Priority::Low);

    agent.acmd("effect_specialairlwlanding", effect_specialairlwlanding, Priority::Low);
    agent.acmd("sound_specialairlwlanding", sound_specialairlwlanding, Priority::Low);
    agent.acmd("expression_specialairlwlanding", expression_specialairlwlanding, Priority::Low);

    agent.acmd("game_attackairlw2", game_attackairlw2, Priority::Low);
}