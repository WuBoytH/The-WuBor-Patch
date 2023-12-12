use crate::imports::acmd_imports::*;

unsafe extern "C" fn ken_specialn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, ken::status::flag::SPECIAL_DECIDE_STRENGTH);
    }
    let strength = WorkModule::get_int(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH);
    if strength == *FIGHTER_RYU_STRENGTH_W {
        macros::FT_MOTION_RATE(agent, 8.0 / 5.0);
    }
    else if strength == *FIGHTER_RYU_STRENGTH_M {
        macros::FT_MOTION_RATE(agent, 6.0 / 5.0);
    }
    else {
        macros::FT_MOTION_RATE(agent, 4.0 / 5.0);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SHOOT);
    }
    let strength = WorkModule::get_int(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH);
    if strength == *FIGHTER_RYU_STRENGTH_W {
        MiscModule::calc_motion_rate_from_cancel_frame(agent, 13.0, -12.0);
    }
    else if strength == *FIGHTER_RYU_STRENGTH_M {
        MiscModule::calc_motion_rate_from_cancel_frame(agent, 13.0, -10.0);
    }
    else {
        MiscModule::calc_motion_rate_from_cancel_frame(agent, 13.0, -8.0);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
    }
}

unsafe extern "C" fn ken_specialn2start(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    macros::FT_MOTION_RATE(agent, 7.0 / 12.0);
    frame(agent.lua_state_agent, 16.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 80, 100, 50, 0, 3.0, 0.0, 12.5, 3.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 6.0, 90, 100, 50, 0, 3.0, 0.0, 15.5, 8.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 5.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 1, 5.0, false);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, ken::status::flag::SPECIAL_N2_GROUND_BRANCH_CHECK);
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, ken::status::flag::SPECIAL_N2_GROUND_BRANCH_H);
    }
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, ken::status::flag::SPECIAL_N2_GROUND_BRANCH_LM);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        VarModule::off_flag(agent.module_accessor, ken::status::flag::SPECIAL_N2_GROUND_BRANCH_CHECK);
        VarModule::off_flag(agent.module_accessor, ken::status::flag::SPECIAL_N2_GROUND_BRANCH_LM);
        VarModule::off_flag(agent.module_accessor, ken::status::flag::SPECIAL_N2_GROUND_BRANCH_H);
    }
}

unsafe extern "C" fn ken_specialn2start_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("ken_syoryuken_fire"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 0.7, false);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("ken_syoryuken_fire"), false, true);
    }
}

unsafe extern "C" fn ken_specialn2start_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_ken_jinrai_kick"));
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ken_step_right_ll"));
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ken_special_h02"));
    }
}

unsafe extern "C" fn ken_specialn2start_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::AREA_WIND_2ND_arg10(agent, 0, 1, 90, 8, 0.4, 0, 2, 16, 36, 80);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitll"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        AreaModule::erase_wind(agent.module_accessor, 0);
    }
    frame(agent.lua_state_agent, 38.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 5);
    }
}

unsafe extern "C" fn ken_specialn2lw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 75, 50, 0, 40, 2.8, 0.0, 8.0, 3.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 6.0, 75, 50, 0, 40, 2.8, 0.0, 6.0, 6.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 6.0, 75, 50, 0, 40, 3.0, 0.0, 4.0, 9.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn ken_specialn2lw_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 9.5, 4, 0, 0, 0, 0, 0.65, 0, 0, 0, 0, 0, 360, false, 0.3);
        macros::LAST_EFFECT_SET_COLOR(agent, 0.8, 0.2, 0.2);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("ken_syoryuken_fire"), false, true);
    }
}

unsafe extern "C" fn ken_specialn2lw_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_ken_jinrai_kick_l"));
        macros::PLAY_SE(agent, Hash40::new("se_ken_swing_kick_l"));
    }
}

unsafe extern "C" fn ken_specialn2lw_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 5);
    }
}

unsafe extern "C" fn ken_specialn2hi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("footr"), 10.0, 361, 60, 0, 60, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneer"), 10.0, 361, 60, 0, 60, 2.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn ken_specialn2hi_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("ken_syoryuken_fire"), false, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("ken_syoryuken_fire"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 0.7, false);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("ken_syoryuken_fire"), false, true);
    }
}

unsafe extern "C" fn ken_specialn2hi_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_ken_jinrai_kick_m"));
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ken_swing_kick_l"));
    }
}

unsafe extern "C" fn ken_specialn2hi_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_R, 5);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
    frame(agent.lua_state_agent, 33.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 5);
    }
}

unsafe extern "C" fn ken_specialn2s(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    macros::FT_MOTION_RATE(agent, 6.0 / 4.0);
    frame(agent.lua_state_agent, 6.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 45, 80, 0, 80, 3.0, 0.0, 12.5, 3.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 45, 80, 0, 80, 3.0, 0.0, 15.5, 8.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn ken_specialn2s_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("ken_syoryuken_fire"), false, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("ken_syoryuken_fire"), Hash40::new("footr"), 0, 0, 0, 0, 180, 0, 0.7, false);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("ken_syoryuken_fire"), false, true);
    }
}

unsafe extern "C" fn ken_specialn2s_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ken_swing_kick_l"));
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_ken_jinrai_kick_h"));
    }
}

unsafe extern "C" fn ken_specialn2s_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_R, 5);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 5);
    }
}

unsafe extern "C" fn ken_specialairn2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if VarModule::is_flag(agent.module_accessor, ken::status::flag::QUICK_STEP_INHERITED) {
        macros::FT_MOTION_RATE(agent, 8.0 / 14.0);
    }
    else {
        macros::FT_MOTION_RATE(agent, 7.0 / 5.0);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, ken::status::flag::SPECIAL_DECIDE_STRENGTH);
    }
    let strength = WorkModule::get_int(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH);
    if !VarModule::is_flag(agent.module_accessor, ken::status::flag::QUICK_STEP_INHERITED) {
        if strength == *FIGHTER_RYU_STRENGTH_M {
            macros::FT_MOTION_RATE(agent, 16.0 / 21.0);
        }
        else if strength == *FIGHTER_RYU_STRENGTH_W {
            macros::FT_MOTION_RATE(agent, 13.0 / 21.0);
        }
        else {
            macros::FT_MOTION_RATE(agent, 16.0 / 17.0);
        }
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, ken::status::flag::SPECIAL_N2_AIR_DISABLE_GRAVITY);
    }
    frame(agent.lua_state_agent, 15.0);
    if VarModule::is_flag(agent.module_accessor, ken::status::flag::QUICK_STEP_INHERITED) {
        macros::FT_MOTION_RATE(agent, 1.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 100, 60, 0, 3.0, 0.0, 10.0, 5.0, None, None, None, 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_KICK);
        }
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    // let strength = WorkModule::get_int(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH);
    // if VarModule::is_flag(agent.module_accessor, ken::status::flag::QUICK_STEP_INHERITED)
    // || strength == *FIGHTER_RYU_STRENGTH_S {
    //     macros::FT_MOTION_RATE(agent, 7.0 / 4.0);
    // }
    frame(agent.lua_state_agent, 23.0);
    let strength = WorkModule::get_int(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH);
    if VarModule::is_flag(agent.module_accessor, ken::status::flag::QUICK_STEP_INHERITED)
    || strength == *FIGHTER_RYU_STRENGTH_S {
        macros::FT_MOTION_RATE(agent, 1.0);
    }
    if VarModule::is_flag(agent.module_accessor, ken::status::flag::QUICK_STEP_INHERITED) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 250, 40, 0, 60, 3.0, 0.0, 9.0, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_KICK);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 7.0, 250, 40, 0, 60, 3.0, 0.0, 11.0, 8.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_KICK);
        }
    }
    else if strength == *FIGHTER_RYU_STRENGTH_S {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 250, 40, 0, 60, 3.0, 0.0, 9.0, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 7.0, 250, 40, 0, 60, 3.0, 0.0, 11.0, 8.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
        }
    }
    frame(agent.lua_state_agent, 24.0);
    let strength = WorkModule::get_int(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH);
    if VarModule::is_flag(agent.module_accessor, ken::status::flag::QUICK_STEP_INHERITED) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 250, 40, 0, 60, 3.0, 0.0, 8.5, 4.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_KICK);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 7.0, 250, 40, 0, 60, 3.0, 0.0, 8.0, 7.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_KICK);
        }
    }
    else if strength == *FIGHTER_RYU_STRENGTH_S {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 250, 40, 0, 60, 3.0, 0.0, 8.5, 4.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 7.0, 250, 40, 0, 60, 3.0, 0.0, 8.0, 7.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
        }
    }
    frame(agent.lua_state_agent, 27.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    let strength = WorkModule::get_int(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH);
    if VarModule::is_flag(agent.module_accessor, ken::status::flag::QUICK_STEP_INHERITED) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 250, 40, 0, 60, 3.0, 0.0, 5.0, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_KICK);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 7.0, 250, 40, 0, 60, 3.0, 0.0, 7.0, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_KICK);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 12.0, 361, 40, 0, 60, 4.0, 0.0, 5.0, 6.0, Some(0.0), Some(7.0), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_KICK);
        }
    }
    else if strength == *FIGHTER_RYU_STRENGTH_S {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 250, 40, 0, 60, 3.0, 0.0, 5.0, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 7.0, 250, 40, 0, 60, 3.0, 0.0, 7.0, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 12.0, 361, 40, 0, 60, 4.0, 0.0, 5.0, 6.0, Some(0.0), Some(7.0), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
        }
    }
    else if strength == *FIGHTER_RYU_STRENGTH_M {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 361, 40, 0, 60, 4.0, 0.0, 5.0, 6.0, Some(0.0), Some(7.0), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 361, 40, 0, 60, 4.0, 0.0, 5.0, 6.0, Some(0.0), Some(7.0), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
        }
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, ken::status::flag::SPECIAL_N2_AIR_ENABLE_LANDING);
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn ken_specialairn2_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if VarModule::is_flag(agent.module_accessor, ken::status::flag::QUICK_STEP_INHERITED) {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("ken_syoryuken_fire"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 0.7, false);
            EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        }
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("ken_attack_arc"), Hash40::new("ken_attack_arc"), Hash40::new("top"), -2, 8, 3, 30, -20, -170, 1.1, true, *EF_FLIP_YZ, 0.35);
        if VarModule::is_flag(agent.module_accessor, ken::status::flag::QUICK_STEP_INHERITED) {
            macros::LAST_EFFECT_SET_COLOR(agent, 0.8, 0.2, 0.2);
        }
        let strength = WorkModule::get_int(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH);
        if VarModule::is_flag(agent.module_accessor, ken::status::flag::QUICK_STEP_INHERITED)
        || strength == *FIGHTER_RYU_STRENGTH_S {
            macros::LAST_EFFECT_SET_RATE(agent, 0.75);
        }
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("ken_syoryuken_fire"), false, true);
    }
}

unsafe extern "C" fn ken_specialairn2_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    let strength = WorkModule::get_int(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH);
    if VarModule::is_flag(agent.module_accessor, ken::status::flag::QUICK_STEP_INHERITED) {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("vc_ken_dragonlash_step"));
        }
    }
    else if strength == *FIGHTER_RYU_STRENGTH_S {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("vc_ken_dragonlash_h"));
        }
    }
    else if strength == *FIGHTER_RYU_STRENGTH_M {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("vc_ken_dragonlash_m"));
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("vc_ken_dragonlash_l"));
        }
    }
    frame(agent.lua_state_agent, 13.0);
    let strength = WorkModule::get_int(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH);
    if VarModule::is_flag(agent.module_accessor, ken::status::flag::QUICK_STEP_INHERITED)
    || strength == *FIGHTER_RYU_STRENGTH_S {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_ken_swing_kick_m"));
        }
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ken_swing_kick_l"));
    }
}

unsafe extern "C" fn ken_specialsstart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(agent.module_accessor, 1.0, 3.5, 8.5, 8.5);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        if VarModule::is_flag(agent.module_accessor, ken::status::flag::QUICK_STEP_INHERITED) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 45, 30, 50, 40, 4.5, 0.0, 9.0, 4.5, Some(0.0), Some(9.0), Some(4.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_KICK);
        }
        else {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 0, 50, 100, 0, 4.5, 0.0, 9.0, 4.5, Some(0.0), Some(9.0), Some(4.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
        }
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(agent.module_accessor, 3.0, 3.5, 8.5, 4.5);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::set_target_category(agent.module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(agent.module_accessor, 0, 0.1);
    }
}

unsafe extern "C" fn ken_specialsstart_eff(agent: &mut L2CAgentBase) {
    if VarModule::is_flag(agent.module_accessor, ken::status::flag::QUICK_STEP_INHERITED) {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("ken_syoryuken_fire"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 0.7, false);
            EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        }
    }
    frame(agent.lua_state_agent, 6.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND) {
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 11, 12, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 10.5, 6, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true, 0.7);
    }
}

unsafe extern "C" fn ken_specialsstart_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if VarModule::is_flag(agent.module_accessor, ken::status::flag::QUICK_STEP_INHERITED) {
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND) {
            if macros::is_excute(agent) {
                macros::PLAY_SE(agent, Hash40::new("se_ken_command_success"));
            }
        }
        wait(agent.lua_state_agent, 7.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("vc_ken_tatsumaki_step01"));
        }
    }
    else if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND) {
        wait(agent.lua_state_agent, 7.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("vc_ken_special_s01"));
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_ken_command_success"));
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("vc_ken_special_s01_command"));
        }
    }
}

unsafe extern "C" fn ken_specialairsstart_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if VarModule::is_flag(agent.module_accessor, ken::status::flag::QUICK_STEP_INHERITED) {
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND) {
            if macros::is_excute(agent) {
                macros::PLAY_SE(agent, Hash40::new("se_ken_command_success"));
            }
        }
        wait(agent.lua_state_agent, 7.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("vc_ken_tatsumaki_step01"));
        }
    }
    else if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND) {
        wait(agent.lua_state_agent, 7.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("vc_ken_special_s01"));
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_ken_command_success"));
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("vc_ken_special_s02_command"));
        }
    }
}

unsafe extern "C" fn ken_specialsend(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    wait(agent.lua_state_agent, 1.0);
    let strength = WorkModule::get_int(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH);
    if strength == *FIGHTER_RYU_STRENGTH_S {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 40, 30, 0, 90, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, -2.0, false);
        }
    }
    else if strength == *FIGHTER_RYU_STRENGTH_M {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 50, 30, 0, 80, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, -2.0, false);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 60, 30, 0, 60, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, -2.0, false);
        }
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn ken_specialsend_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_landl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
    }
}

unsafe extern "C" fn ken_specialairsend(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    wait(agent.lua_state_agent, 1.0);
    let strength = WorkModule::get_int(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH);
    if strength == *FIGHTER_RYU_STRENGTH_S {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 40, 30, 0, 90, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, -2.0, false);
        }
    }
    else if strength == *FIGHTER_RYU_STRENGTH_M {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 45, 30, 0, 80, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, -2.0, false);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 50, 30, 0, 70, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, -2.0, false);
        }
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn ken_specialairsend_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
    }
}

unsafe extern "C" fn ken_specialairs2(agent: &mut L2CAgentBase) {
    wait(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.6);
    if macros::is_excute(agent) {
        // notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        macros::HIT_NODE(agent, Hash40::new("kneel"), *HIT_STATUS_XLU);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 35, 30, 50, 40, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::set_target_category(agent.module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(agent.module_accessor, 0, 0.1);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 30, 20, 60, 45, 3.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::set_target_category(agent.module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(agent.module_accessor, 0, 0.1);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
}

unsafe extern "C" fn ken_specialairs2_eff(_agent: &mut L2CAgentBase) {
    // if macros::is_excute(agent) {
    //     macros::EFFECT_FOLLOW(agent, Hash40::new("ken_syoryuken_fire"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 1, false);
    //     EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    // }
}

unsafe extern "C" fn ken_specialairs2_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ken_special_s01"));
    }
}

unsafe extern "C" fn ken_specialairs2_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 4);
    }
}

unsafe extern "C" fn ken_specialairs2end(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 50, 30, 50, 40, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 4.0);
    macros::FT_MOTION_RATE(agent, 1.5);
    frame(agent.lua_state_agent, 12.0);
    macros::FT_MOTION_RATE(agent, 1.5);
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("footr"), 5.0, 340, 70, 0, 50, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneer"), 5.0, 340, 70, 0, 50, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 24.0);
    macros::FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn ken_specialairs2end_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("ken_syoryuken_fire"), false, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("ken_syoryuken_fire"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 1, false);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("ken_syoryuken_fire"), false, true);
    }
}

unsafe extern "C" fn ken_specialairs2end_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ken_special_s01"));
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ken_special_s01"));
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_ken_tatsumaki_step02"));
    }
}

unsafe extern "C" fn ken_specialairs2end_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

unsafe extern "C" fn ken_specialhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_REVERSE_LR);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_DECIDE_STRENGTH);
    }
    let strength = WorkModule::get_int(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH);
    if strength == *FIGHTER_RYU_STRENGTH_W {
        if macros::is_excute(agent) {
            VarModule::on_flag(agent.module_accessor, ken::status::flag::SPECIAL_HI_SPECIAL_EFFECT);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 13.0, 80, 58, 0, 80, 4.6, 0.0, 10.0, 7.6, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    else if strength == *FIGHTER_RYU_STRENGTH_M {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 80, 100, 100, 0, 4.6, 0.0, 10.0, 7.6, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 100, 100, 100, 0, 4.6, 0.0, 10.0, 7.6, None, None, None, 0.9, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    wait(agent.lua_state_agent, 1.0);
    let strength = WorkModule::get_int(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH);
    if strength == *FIGHTER_RYU_STRENGTH_S {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.2, 95, 10, 0, 95, 4.6, 0.0, 14.5, 7.1, Some(0.0), Some(12.5), Some(8.1), 0.5, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
    }
    let strength = WorkModule::get_int(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH);
    if strength == *FIGHTER_RYU_STRENGTH_W {
        if macros::is_excute(agent) {
            VarModule::off_flag(agent.module_accessor, ken::status::flag::SPECIAL_HI_SPECIAL_EFFECT);
            macros::ATTACK(agent, 0, 0, Hash40::new("armr"), 7.0, 70, 90, 0, 60, 5.0, 4.0, -0.4, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    else if strength == *FIGHTER_RYU_STRENGTH_M {
        if macros::is_excute(agent) {
            VarModule::on_flag(agent.module_accessor, ken::status::flag::SPECIAL_HI_SPECIAL_EFFECT);
            macros::ATTACK(agent, 0, 0, Hash40::new("armr"), 6.0, 70, 121, 0, 78, 5.5, 4.0, -0.4, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    else {
        if macros::is_excute(agent) {
            VarModule::on_flag(agent.module_accessor, ken::status::flag::SPECIAL_HI_SPECIAL_EFFECT);
            macros::ATTACK(agent, 0, 0, Hash40::new("armr"), 6.5, 70, 126, 0, 80, 5.5, 4.0, -0.4, 0.0, Some(-3.0), Some(-0.4), Some(0.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        VarModule::off_flag(agent.module_accessor, ken::status::flag::SPECIAL_HI_SPECIAL_EFFECT);
        macros::ATTACK(agent, 0, 0, Hash40::new("armr"), 7.0, 70, 90, 0, 60, 5.0, 4.0, -0.4, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn ken_specialhi_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 0.7);
    }
}

unsafe extern "C" fn ken_specialhicommand_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 17, 12, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 0.7);
    }
    frame(agent.lua_state_agent, 5.0);
    if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) >= 0.0 {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("ken_syoryuken_arc"), Hash40::new("trans"), 6.5, 5, 0, 5, 0, 25, 1, false);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("ken_syoryuken_arc"), Hash40::new("trans"), -6.5, 5, 0, 5, 0, -25, 1, false);
        }
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("ken_syoryuken_arc"), false, true);
    }
}

unsafe extern "C" fn ken_specialhi_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_ken_special_h01"));
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ken_special_h03"));
    }
}

unsafe extern "C" fn ken_specialhicommand_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ken_command_success"));
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_ken_special_h01_command"));
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ken_special_h03"));
    }
}

unsafe extern "C" fn ken_specialhi_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::AREA_WIND_2ND_arg10(agent, 0, 1, 70, 8, 0.8, 0, 7, 32, 14, 80);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    let strength = WorkModule::get_int(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH);
    if strength == *FIGHTER_RYU_STRENGTH_W {
        frame(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitll"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(agent.lua_state_agent, 9.0);
        if macros::is_excute(agent) {
            AreaModule::erase_wind(agent.module_accessor, 0);
            macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
        }
    }
    else if strength == *FIGHTER_RYU_STRENGTH_M {
        frame(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitll"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(agent.lua_state_agent, 9.0);
        if macros::is_excute(agent) {
            AreaModule::erase_wind(agent.module_accessor, 0);
            macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackll"), 0);
        }
    }
    else {
        frame(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitll"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(agent.lua_state_agent, 9.0);
        if macros::is_excute(agent) {
            AreaModule::erase_wind(agent.module_accessor, 0);
            macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackll"), 0);
        }
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::AREA_WIND_2ND_arg10(agent, 0, 1, 90, 8, 0.4, 0, 2, 16, 36, 80);
    }
}

unsafe extern "C" fn ken_specialairhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_REVERSE_LR);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_DECIDE_STRENGTH);
    }
    let strength = WorkModule::get_int(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH);
    if strength == *FIGHTER_RYU_STRENGTH_W {
        if macros::is_excute(agent) {
            VarModule::on_flag(agent.module_accessor, ken::status::flag::SPECIAL_HI_SPECIAL_EFFECT);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 80, 55, 0, 80, 4.6, 0.0, 10.0, 7.6, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    else if strength == *FIGHTER_RYU_STRENGTH_M {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 80, 54, 0, 80, 4.6, 0.0, 10.0, 7.6, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 90, 100, 90, 0, 4.6, 0.0, 10.0, 7.6, None, None, None, 0.9, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    wait(agent.lua_state_agent, 1.0);
    let strength = WorkModule::get_int(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH);
    if strength == *FIGHTER_RYU_STRENGTH_S {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.2, 80, 100, 100, 0, 4.6, 0.0, 14.5, 7.1, Some(0.0), Some(12.5), Some(8.1), 0.5, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
    }
    let strength = WorkModule::get_int(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH);
    if strength == *FIGHTER_RYU_STRENGTH_W {
        if macros::is_excute(agent) {
            VarModule::off_flag(agent.module_accessor, ken::status::flag::SPECIAL_HI_SPECIAL_EFFECT);
            macros::ATTACK(agent, 0, 0, Hash40::new("armr"), 7.0, 70, 70, 0, 60, 5.0, 4.0, -0.4, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    else if strength == *FIGHTER_RYU_STRENGTH_M {
        if macros::is_excute(agent) {
            VarModule::on_flag(agent.module_accessor, ken::status::flag::SPECIAL_HI_SPECIAL_EFFECT);
            macros::ATTACK(agent, 0, 0, Hash40::new("armr"), 6.0, 70, 112, 0, 78, 5.5, 4.0, -0.4, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    else {
        if macros::is_excute(agent) {
            VarModule::on_flag(agent.module_accessor, ken::status::flag::SPECIAL_HI_SPECIAL_EFFECT);
            macros::ATTACK(agent, 0, 0, Hash40::new("armr"), 6.0, 70, 107, 0, 76, 6.0, 4.0, -0.4, 0.0, Some(-3.0), Some(-0.4), Some(0.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        VarModule::off_flag(agent.module_accessor, ken::status::flag::SPECIAL_HI_SPECIAL_EFFECT);
        macros::ATTACK(agent, 0, 0, Hash40::new("armr"), 7.0, 70, 70, 0, 60, 5.0, 4.0, -0.4, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn ken_specialairhi_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("ken_syoryuken_aerial"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn ken_specialairhicommand_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 17, 12, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 5.0);
    if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) >= 0.0 {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("ken_syoryuken_arc"), Hash40::new("trans"), 6.5, 5, 0, 5, 0, 25, 1, false);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("ken_syoryuken_arc"), Hash40::new("trans"), -6.5, 5, 0, 5, 0, -25, 1, false);
        }
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("ken_syoryuken_aerial"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("ken_syoryuken_arc"), false, true);
    }
}

unsafe extern "C" fn ken_specialairhi_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_ken_special_h01"));
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ken_special_h03"));
    }
}

unsafe extern "C" fn ken_specialairhicommand_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ken_command_success"));
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_ken_special_h01_command"));
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ken_special_h03"));
    }
}

unsafe extern "C" fn ken_specialairhi_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::AREA_WIND_2ND_arg10(agent, 0, 1, 70, 8, 0.8, 0, 7, 32, 14, 80);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    let strength = WorkModule::get_int(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH);
    if strength == *FIGHTER_RYU_STRENGTH_W {
        frame(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitll"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(agent.lua_state_agent, 9.0);
        if macros::is_excute(agent) {
            AreaModule::erase_wind(agent.module_accessor, 0);
            macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
        }
    }
    else if strength == *FIGHTER_RYU_STRENGTH_M {
        frame(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitll"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(agent.lua_state_agent, 9.0);
        if macros::is_excute(agent) {
            AreaModule::erase_wind(agent.module_accessor, 0);
            macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackll"), 0);
        }
    }
    else {
        frame(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitll"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(agent.lua_state_agent, 9.0);
        if macros::is_excute(agent) {
            AreaModule::erase_wind(agent.module_accessor, 0);
            macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackll"), 0);
        }
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::AREA_WIND_2ND_arg10(agent, 0, 1, 90, 8, 0.4, 0, 2, 16, 36, 80);
    }
}

unsafe extern "C" fn ken_specialhi2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        // WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_REVERSE_LR);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_DECIDE_STRENGTH);
    }
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.2, 100, 100, 100, 0, 4.6, 0.0, 10.0, 7.6, None, None, None, 1.5, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 366, 10, 0, 95, 4.6, 0.0, 14.5, 7.1, Some(0.0), Some(12.5), Some(8.1), 0.5, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
    }
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("armr"), 6.5, 70, 126, 0, 80, 5.5, 4.0, -0.4, 0.0, Some(-3.0), Some(-0.4), Some(0.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn ken_specialhi2_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 0.7);
    }
    frame(agent.lua_state_agent, 5.0);
    let strength = WorkModule::get_int(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH);
    if strength == *FIGHTER_RYU_STRENGTH_S {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("ken_syoryuken_fire"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1, false);
            EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        }
    }
    if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) >= 0.0 {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("ken_syoryuken_firearc"), Hash40::new("trans"), 3, 2, 2, 5, 0, 5, 1, false);
            macros::EFFECT_FOLLOW(agent, Hash40::new("ken_syoryuken_firearc2"), Hash40::new("trans"), 3, 2, 2, 5, 0, 5, 1, false);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("ken_syoryuken_firearc"), Hash40::new("trans"), -3, 2, 2, 5, 0, -5, 1, false);
            macros::EFFECT_FOLLOW(agent, Hash40::new("ken_syoryuken_firearc2"), Hash40::new("trans"), -3, 2, 2, 5, 0, -5, 1, false);
            EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        }
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::EFFECT_DETACH_KIND(agent, Hash40::new("ken_syoryuken_firearc"), -1);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("ken_syoryuken_fire"), false, true);
    }
}

unsafe extern "C" fn ken_specialhi2command_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 17, 12, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 0.7);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("ken_syoryuken_fire"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1, false);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
    if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) >= 0.0 {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("ken_syoryuken_firearc"), Hash40::new("trans"), 3, 2, 2, 5, 0, 5, 1, false);
            macros::EFFECT_FOLLOW(agent, Hash40::new("ken_syoryuken_firearc2"), Hash40::new("trans"), 3, 2, 2, 5, 0, 5, 1, false);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("ken_syoryuken_firearc"), Hash40::new("trans"), -3, 2, 2, 5, 0, -5, 1, false);
            macros::EFFECT_FOLLOW(agent, Hash40::new("ken_syoryuken_firearc2"), Hash40::new("trans"), -3, 2, 2, 5, 0, -5, 1, false);
            EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        }
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::EFFECT_DETACH_KIND(agent, Hash40::new("ken_syoryuken_firearc"), -1);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("ken_syoryuken_fire"), false, true);
    }
}

unsafe extern "C" fn ken_specialhi2_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_ken_shoryuken_step"));
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ken_special_h01"));
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ken_special_h02"));
    }
}

unsafe extern "C" fn ken_specialhi2command_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ken_command_success"));
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_ken_shoryuken_step"));
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ken_special_h01"));
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ken_special_h02"));
    }
}

unsafe extern "C" fn ken_specialhi2_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::AREA_WIND_2ND_arg10(agent, 0, 1, 70, 8, 0.8, 0, 7, 32, 14, 80);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    let strength = WorkModule::get_int(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH);
    if strength == *FIGHTER_RYU_STRENGTH_W {
        frame(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitll"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(agent.lua_state_agent, 9.0);
        if macros::is_excute(agent) {
            AreaModule::erase_wind(agent.module_accessor, 0);
            macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
        }
    }
    else if strength == *FIGHTER_RYU_STRENGTH_M {
        frame(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitll"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(agent.lua_state_agent, 9.0);
        if macros::is_excute(agent) {
            AreaModule::erase_wind(agent.module_accessor, 0);
            macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackll"), 0);
        }
    }
    else {
        frame(agent.lua_state_agent, 5.0);
        if macros::is_excute(agent) {
            macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
        }
        frame(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitll"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(agent.lua_state_agent, 9.0);
        if macros::is_excute(agent) {
            AreaModule::erase_wind(agent.module_accessor, 0);
            macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackll"), 0);
        }
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::AREA_WIND_2ND_arg10(agent, 0, 1, 90, 8, 0.4, 0, 2, 16, 36, 80);
    }
}

unsafe extern "C" fn ken_specialairhi2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        // WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_REVERSE_LR);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_DECIDE_STRENGTH);
    }
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.2, 90, 100, 90, 0, 4.6, 0.0, 10.0, 7.6, None, None, None, 1.5, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 366, 100, 100, 0, 4.6, 0.0, 14.5, 7.1, Some(0.0), Some(12.5), Some(9.1), 0.5, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
    }
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("armr"), 6.0, 70, 107, 0, 76, 6.0, 4.0, -0.4, 0.0, Some(-3.0), Some(-0.4), Some(0.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn ken_specialairhi2_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 0.7);
    }
    frame(agent.lua_state_agent, 5.0);
    let strength = WorkModule::get_int(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH);
    if strength == *FIGHTER_RYU_STRENGTH_S {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("ken_syoryuken_fire"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1, false);
            EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        }
    }
    if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) >= 0.0 {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("ken_syoryuken_firearc"), Hash40::new("trans"), 3, 2, 2, 5, 0, 5, 1, false);
            macros::EFFECT_FOLLOW(agent, Hash40::new("ken_syoryuken_firearc2"), Hash40::new("trans"), 3, 2, 2, 5, 0, 5, 1, false);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("ken_syoryuken_firearc"), Hash40::new("trans"), -3, 2, 2, 5, 0, -5, 1, false);
            macros::EFFECT_FOLLOW(agent, Hash40::new("ken_syoryuken_firearc2"), Hash40::new("trans"), -3, 2, 2, 5, 0, -5, 1, false);
            EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        }
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::EFFECT_DETACH_KIND(agent, Hash40::new("ken_syoryuken_firearc"), -1);
        macros::EFFECT(agent, Hash40::new("ken_syoryuken_aerial"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("ken_syoryuken_fire"), false, true);
    }
}

unsafe extern "C" fn ken_specialairhi2command_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 17, 12, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("ken_syoryuken_fire"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1, false);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
    if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) >= 0.0 {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("ken_syoryuken_firearc"), Hash40::new("trans"), 3, 2, 2, 5, 0, 5, 1, false);
            macros::EFFECT_FOLLOW(agent, Hash40::new("ken_syoryuken_firearc2"), Hash40::new("trans"), 3, 2, 2, 5, 0, 5, 1, false);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("ken_syoryuken_firearc"), Hash40::new("trans"), -3, 2, 2, 5, 0, -5, 1, false);
            macros::EFFECT_FOLLOW(agent, Hash40::new("ken_syoryuken_firearc2"), Hash40::new("trans"), -3, 2, 2, 5, 0, -5, 1, false);
            EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        }
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::EFFECT_DETACH_KIND(agent, Hash40::new("ken_syoryuken_firearc"), -1);
        macros::EFFECT(agent, Hash40::new("ken_syoryuken_aerial"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("ken_syoryuken_fire"), false, true);
    }
}

unsafe extern "C" fn ken_specialairhi2_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_ken_shoryuken_step"));
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ken_special_h01"));
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ken_special_h02"));
    }
}

unsafe extern "C" fn ken_specialairhi2command_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ken_command_success"));
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_ken_shoryuken_step"));
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ken_special_h01"));
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ken_special_h02"));
    }
}

unsafe extern "C" fn ken_specialairhi2_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::AREA_WIND_2ND_arg10(agent, 0, 1, 70, 8, 0.8, 0, 7, 32, 14, 80);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    let strength = WorkModule::get_int(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH);
    if strength == *FIGHTER_RYU_STRENGTH_W {
        frame(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitll"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(agent.lua_state_agent, 9.0);
        if macros::is_excute(agent) {
            AreaModule::erase_wind(agent.module_accessor, 0);
            macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
        }
    }
    else if strength == *FIGHTER_RYU_STRENGTH_M {
        frame(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitll"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(agent.lua_state_agent, 9.0);
        if macros::is_excute(agent) {
            AreaModule::erase_wind(agent.module_accessor, 0);
            macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackll"), 0);
        }
    }
    else {
        frame(agent.lua_state_agent, 5.0);
        if macros::is_excute(agent) {
            macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
        }
        frame(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitll"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(agent.lua_state_agent, 9.0);
        if macros::is_excute(agent) {
            AreaModule::erase_wind(agent.module_accessor, 0);
            macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackll"), 0);
        }
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::AREA_WIND_2ND_arg10(agent, 0, 1, 90, 8, 0.4, 0, 2, 16, 36, 80);
    }
}

unsafe extern "C" fn ken_speciallwstart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, ken::status::flag::SPECIAL_LW_ENABLE_ACTION);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, ken::status::flag::SPECIAL_LW_RESET_GRAVITY);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, ken::status::flag::SPECIAL_LW_UNABLE_ACTION);
    }
}

unsafe extern "C" fn ken_speciallwstart_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 1.0, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 1.0, true);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 7.5, 0, 0, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn ken_speciallwstart_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_ken_quick_step"));
    }
}

unsafe extern "C" fn ken_speciallwstart_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 5);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_run"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_run"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_run"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn ken_specialairlwstart_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 5);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_run"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn ken_speciallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, ken::status::flag::SPECIAL_LW_RESET_GRAVITY);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 45, 10, 0, 70, 3.5, 0.0, 9.5, 10.0, Some(0.0), Some(9.5), Some(2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 15, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn ken_speciallw_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 13, 12, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 360, false, 0.4);
    }
}

unsafe extern "C" fn ken_speciallw_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_ken_quick_step_kick"));
    }
}

unsafe extern "C" fn ken_speciallw_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_R, 4);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
    }
}

unsafe extern "C" fn ken_speciallwstepf(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, ken::status::flag::SPECIAL_LW_RESET_GRAVITY);
    }
}

unsafe extern "C" fn ken_speciallwstepf_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 7.5, 0, 0, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn ken_speciallwstepf_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_ken_quick_step_stop"));
    }
}

unsafe extern "C" fn ken_speciallwstepf_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 5);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_dash"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.game_acmd("game_specialn", ken_specialn);

    agent.game_acmd("game_specialairn", ken_specialn);

    agent.game_acmd("game_specialn2start", ken_specialn2start);
    agent.effect_acmd("effect_specialn2start", ken_specialn2start_eff);
    agent.sound_acmd("sound_specialn2start", ken_specialn2start_snd);
    agent.expression_acmd("expression_specialn2start", ken_specialn2start_exp);

    agent.game_acmd("game_specialn2lw", ken_specialn2lw);
    agent.effect_acmd("effect_specialn2lw", ken_specialn2lw_eff);
    agent.sound_acmd("sound_specialn2lw", ken_specialn2lw_snd);
    agent.expression_acmd("expression_specialn2lw", ken_specialn2lw_exp);

    agent.game_acmd("game_specialn2hi", ken_specialn2hi);
    agent.effect_acmd("effect_specialn2hi", ken_specialn2hi_eff);
    agent.sound_acmd("sound_specialn2hi", ken_specialn2hi_snd);
    agent.expression_acmd("expression_specialn2hi", ken_specialn2hi_exp);

    agent.game_acmd("game_specialn2s", ken_specialn2s);
    agent.effect_acmd("effect_specialn2s", ken_specialn2s_eff);
    agent.sound_acmd("sound_specialn2s", ken_specialn2s_snd);
    agent.expression_acmd("expression_specialn2s", ken_specialn2s_exp);

    agent.game_acmd("game_specialairn2", ken_specialairn2);
    agent.effect_acmd("effect_specialairn2", ken_specialairn2_eff);
    agent.sound_acmd("sound_specialairn2", ken_specialairn2_snd);

    agent.game_acmd("game_specialsstart", ken_specialsstart);
    agent.effect_acmd("effect_specialsstart", ken_specialsstart_eff);
    agent.sound_acmd("sound_specialsstart", ken_specialsstart_snd);

    agent.game_acmd("game_specialairsstart", ken_specialsstart);
    agent.effect_acmd("effect_specialairsstart", ken_specialsstart_eff);
    agent.sound_acmd("sound_specialairsstart", ken_specialairsstart_snd);

    agent.game_acmd("game_specialsend", ken_specialsend);
    agent.expression_acmd("expression_specialsend", ken_specialsend_exp);

    agent.game_acmd("game_specialairsend", ken_specialairsend);
    agent.expression_acmd("expression_specialairsend", ken_specialairsend_exp);

    agent.game_acmd("game_specialairs2", ken_specialairs2);
    agent.effect_acmd("effect_specialairs2", ken_specialairs2_eff);
    agent.sound_acmd("sound_specialairs2", ken_specialairs2_snd);
    agent.expression_acmd("expression_specialairs2", ken_specialairs2_exp);

    agent.game_acmd("game_specialairs2end", ken_specialairs2end);
    agent.effect_acmd("effect_specialairs2end", ken_specialairs2end_eff);
    agent.sound_acmd("sound_specialairs2end", ken_specialairs2end_snd);
    agent.expression_acmd("expression_specialairs2end", ken_specialairs2end_exp);

    agent.game_acmd("game_specialhi", ken_specialhi);
    agent.effect_acmd("effect_specialhi", ken_specialhi_eff);
    agent.sound_acmd("sound_specialhi", ken_specialhi_snd);
    agent.expression_acmd("expression_specialhi", ken_specialhi_exp);

    agent.game_acmd("game_specialhicommand", ken_specialhi);
    agent.effect_acmd("effect_specialhicommand", ken_specialhicommand_eff);
    agent.sound_acmd("sound_specialhicommand", ken_specialhicommand_snd);
    agent.expression_acmd("expression_specialhicommand", ken_specialhi_exp);

    agent.game_acmd("game_specialairhi", ken_specialairhi);
    agent.effect_acmd("effect_specialairhi", ken_specialairhi_eff);
    agent.sound_acmd("sound_specialairhi", ken_specialairhi_snd);
    agent.expression_acmd("expression_specialairhi", ken_specialairhi_exp);

    agent.game_acmd("game_specialairhicommand", ken_specialairhi);
    agent.effect_acmd("effect_specialairhicommand", ken_specialairhicommand_eff);
    agent.sound_acmd("sound_specialairhicommand", ken_specialairhicommand_snd);
    agent.expression_acmd("expression_specialairhicommand", ken_specialairhi_exp);

    agent.game_acmd("game_specialhi2", ken_specialhi2);
    agent.effect_acmd("effect_specialhi2", ken_specialhi2_eff);
    agent.sound_acmd("sound_specialhi2", ken_specialhi2_snd);
    agent.expression_acmd("expression_specialhi2", ken_specialhi2_exp);

    agent.game_acmd("game_specialhi2command", ken_specialhi2);
    agent.effect_acmd("effect_specialhi2command", ken_specialhi2command_eff);
    agent.sound_acmd("sound_specialhi2command", ken_specialhi2command_snd);
    agent.expression_acmd("expression_specialhi2command", ken_specialhi2_exp);

    agent.game_acmd("game_specialairhi2", ken_specialairhi2);
    agent.effect_acmd("effect_specialairhi2", ken_specialairhi2_eff);
    agent.sound_acmd("sound_specialairhi2", ken_specialairhi2_snd);
    agent.expression_acmd("expression_specialairhi2", ken_specialairhi2_exp);

    agent.game_acmd("game_specialairhi2command", ken_specialairhi2);
    agent.effect_acmd("effect_specialairhi2command", ken_specialairhi2command_eff);
    agent.sound_acmd("sound_specialairhi2command", ken_specialairhi2command_snd);
    agent.expression_acmd("expression_specialairhi2command", ken_specialairhi2_exp);

    agent.game_acmd("game_speciallwstart", ken_speciallwstart);
    agent.effect_acmd("effect_speciallwstart", ken_speciallwstart_eff);
    agent.sound_acmd("sound_speciallwstart", ken_speciallwstart_snd);
    agent.expression_acmd("expression_speciallwstart", ken_speciallwstart_exp);

    agent.game_acmd("game_specialairlwstart", ken_speciallwstart);
    agent.effect_acmd("effect_specialairlwstart", ken_speciallwstart_eff);
    agent.sound_acmd("sound_specialairlwstart", ken_speciallwstart_snd);
    agent.expression_acmd("expression_specialairlwstart", ken_specialairlwstart_exp);

    agent.game_acmd("game_speciallw", ken_speciallw);
    agent.effect_acmd("effect_speciallw", ken_speciallw_eff);
    agent.sound_acmd("sound_speciallw", ken_speciallw_snd);
    agent.expression_acmd("expression_speciallw", ken_speciallw_exp);

    agent.game_acmd("game_specialairlw", ken_speciallw);
    agent.effect_acmd("effect_specialairlw", ken_speciallw_eff);
    agent.sound_acmd("sound_specialairlw", ken_speciallw_snd);
    agent.expression_acmd("expression_specialairlw", ken_speciallw_exp);

    agent.game_acmd("game_speciallwstepf", ken_speciallwstepf);
    agent.effect_acmd("effect_speciallwstepf", ken_speciallwstepf_eff);
    agent.sound_acmd("sound_speciallwstepf", ken_speciallwstepf_snd);
    agent.expression_acmd("expression_speciallwstepf", ken_speciallwstepf_exp);

    agent.game_acmd("game_specialairlwstepf", ken_speciallwstepf);
    agent.effect_acmd("effect_specialairlwstepf", ken_speciallwstepf_eff);
    agent.sound_acmd("sound_specialairlwstepf", ken_speciallwstepf_snd);
    agent.expression_acmd("expression_specialairlwstepf", ken_speciallwstepf_exp);
}