use crate::imports::acmd_imports::*;


#[acmd_script( agent = "rockman", script = "game_attack11melee", category = ACMD_GAME, low_priority )]
unsafe fn rockman_attack11(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 361, 15, 0, 30, 3.0, 0.0, 8.0, 5.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 180, 15, 0, 30, 3.0, 0.0, 8.0, 8.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART);
    }
}

#[acmd_script( agent = "rockman", script = "effect_attack11melee", category = ACMD_EFFECT, low_priority )]
unsafe fn rockman_attack11_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FLIP(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), -4, 8, 0, 0, 0, 0, 0.95, 0, 1, 0, 0, 0, 0, false, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 8, 8, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 360, false);
        macros::LAST_EFFECT_SET_ALPHA(fighter, 0.7);
    }
}

#[acmd_script( agent = "rockman", script = "sound_attack11melee", category = ACMD_SOUND, low_priority )]
unsafe fn rockman_attack11_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_punch_kick_swing_s"));
    }
}

#[acmd_script( agent = "rockman", script = "expression_attack11melee", category = ACMD_EXPRESSION, low_priority )]
unsafe fn rockman_attack11_exp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        AttackModule::set_attack_reference_joint_id(fighter.module_accessor, Hash40::new("top"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_NONE), AttackDirectionAxis(*ATTACK_DIRECTION_NONE));
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f5b14bb65), *FIGHTER_ROCKMAN_ARM_LEFT, *FIGHTER_ROCKMAN_ARMFORM_HAND, 5);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f5b14bb65), *FIGHTER_ROCKMAN_ARM_RIGHT, *FIGHTER_ROCKMAN_ARMFORM_HAND, 5);
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 4, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attacks"), 0);
    }
}

#[acmd_script( agent = "rockman", script = "game_attack12", category = ACMD_GAME, low_priority )]
unsafe fn rockman_attack12(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 361, 15, 0, 30, 3.0, 0.0, 7.0, 5.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 180, 15, 0, 30, 4.0, 0.0, 7.0, 10.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 2.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 2.0, false);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART);
    }
}

#[acmd_script( agent = "rockman", script = "effect_attack12", category = ACMD_EFFECT, low_priority )]
unsafe fn rockman_attack12_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc"), Hash40::new("sys_attack_arc"), Hash40::new("top"), 1, 8, 2, 10, -39, 10, 0.95, true, *EF_FLIP_YZ);
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script( agent = "rockman", script = "sound_attack12", category = ACMD_SOUND, low_priority )]
unsafe fn rockman_attack12_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_punch_kick_swing_m"));
    }
}

#[acmd_script( agent = "rockman", script = "expression_attack12", category = ACMD_EXPRESSION, low_priority )]
unsafe fn rockman_attack12_exp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f5b14bb65), *FIGHTER_ROCKMAN_ARM_LEFT, *FIGHTER_ROCKMAN_ARMFORM_HAND, 5);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f5b14bb65), *FIGHTER_ROCKMAN_ARM_RIGHT, *FIGHTER_ROCKMAN_ARMFORM_HAND, 5);
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_L, 5);
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 5);
    }
}

#[acmd_script( agent = "rockman", script = "game_attack13", category = ACMD_GAME, low_priority )]
unsafe fn rockman_attack13(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 74, 70, 0, 70, 4.0, 0.0, 6.0, 4.0, Some(0.0), Some(13.0), Some(4.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "rockman", script = "effect_attack13", category = ACMD_EFFECT, low_priority )]
unsafe fn rockman_attack13_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 1, 0, 1.5, -80, 0, 0, 1.0, true, *EF_FLIP_YZ);
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 1, 0, 4, -80, 0, 0, 1.0, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -1, 5, 1, -80, 0, 0, 1.0, true);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 1.5, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script( agent = "rockman", script = "sound_attack13", category = ACMD_SOUND, low_priority )]
unsafe fn rockman_attack13_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_punch_kick_swing_l"));
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_rockman_step_right_m"));
    }
}

#[acmd_script( agent = "rockman", script = "expression_attack13", category = ACMD_EXPRESSION, low_priority )]
unsafe fn rockman_attack13_exp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f5b14bb65), *FIGHTER_ROCKMAN_ARM_LEFT, *FIGHTER_ROCKMAN_ARMFORM_HAND, 5);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f5b14bb65), *FIGHTER_ROCKMAN_ARM_RIGHT, *FIGHTER_ROCKMAN_ARMFORM_HAND, 5);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_lands"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

#[acmd_script( agent = "rockman", script = "game_attacks3melee", category = ACMD_GAME, low_priority )]
unsafe fn rockman_attacks3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    macros::FT_MOTION_RATE(fighter, 0.5);
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 295, 70, 0, 55, 6.0, 0.0, 7.0, 5.0, Some(0.0), Some(7.0), Some(7.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 55, 70, 0, 55, 6.0, 0.0, 7.0, 5.0, Some(0.0), Some(7.0), Some(7.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent, 21.0);
    macros::FT_MOTION_RATE(fighter, 29.0 / 32.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 53.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
}

#[acmd_script( agent = "rockman", script = "effect_attacks3melee", category = ACMD_EFFECT, low_priority )]
unsafe fn rockman_attacks3_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_jump_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc"), Hash40::new("sys_attack_arc"), Hash40::new("top"), 1, 8, -1, 10, -39, -140, 0.95, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script( agent = "rockman", script = "sound_attacks3melee", category = ACMD_SOUND, low_priority )]
unsafe fn rockman_attacks3_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_rockman_jump01"));
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_punch_kick_swing_l"));
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_rockman_step_left_m"));
    }
}

#[acmd_script( agent = "rockman", script = "expression_attacks3melee", category = ACMD_EXPRESSION, low_priority )]
unsafe fn rockman_attacks3_exp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_L);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f5b14bb65), *FIGHTER_ROCKMAN_ARM_LEFT, *FIGHTER_ROCKMAN_ARMFORM_HAND, 5);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f5b14bb65), *FIGHTER_ROCKMAN_ARM_RIGHT, *FIGHTER_ROCKMAN_ARMFORM_HAND, 5);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 5);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_lands_hv"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    install_acmd_scripts!(
        rockman_attack11, rockman_attack11_eff, rockman_attack11_snd, rockman_attack11_exp,
        rockman_attack12, rockman_attack12_eff, rockman_attack12_snd, rockman_attack12_exp,
        rockman_attack13, rockman_attack13_eff, rockman_attack13_snd, rockman_attack13_exp,
        rockman_attacks3, rockman_attacks3_eff, rockman_attacks3_snd, rockman_attacks3_exp
    );
}