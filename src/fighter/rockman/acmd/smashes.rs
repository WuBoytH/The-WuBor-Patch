use crate::imports::acmd_imports::*;

#[acmd_script( agent = "rockman", script = "game_attacks4", category = ACMD_GAME, low_priority )]
unsafe fn rockman_attacks4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("footr"), 10.0, 361, 70, 0, 80, 4.0, 0.0, -2.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("footr"), 7.0, 361, 70, 0, 80, 4.0, 0.0, -2.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "rockman", script = "effect_attacks4", category = ACMD_EFFECT, low_priority )]
unsafe fn rockman_attacks4_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_b"), Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 0, 9, 3, 90, -90, -40, 1.0, true, *EF_FLIP_YZ);
    }
}

#[acmd_script( agent = "rockman", script = "sound_attacks4", category = ACMD_SOUND, low_priority )]
unsafe fn rockman_attacks4_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_punch_kick_swing_m"))
    }
}

#[acmd_script( agent = "rockman", script = "expression_attacks4", category = ACMD_EXPRESSION, low_priority )]
unsafe fn rockman_attacks4_exp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f5b14bb65), *FIGHTER_ROCKMAN_ARM_LEFT, *FIGHTER_ROCKMAN_ARMFORM_HAND, 10);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f5b14bb65), *FIGHTER_ROCKMAN_ARM_RIGHT, *FIGHTER_ROCKMAN_ARMFORM_HAND, 10);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_L);
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    }
    frame(fighter.lua_state_agent, 12.0);
    execute(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_L);
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(
            fighter.module_accessor,
            Hash40::new("rbkind_nohitl"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_NONE);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
    frame(fighter.lua_state_agent, 35.0);
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
    }
    frame(fighter.lua_state_agent, 52.0);
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

#[acmd_script( agent = "rockman", script = "game_attacks4hi", category = ACMD_GAME, low_priority )]
unsafe fn rockman_attacks4hi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("footr"), 12.0, 75, 60, 0, 100, 4.0, 0.0, -2.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "rockman", script = "effect_attacks4hi", category = ACMD_EFFECT, low_priority )]
unsafe fn rockman_attacks4hi_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), -2.5, 0, 3, -80, 0, 10, 1, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 32.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script( agent = "rockman", script = "sound_attacks4hi", category = ACMD_SOUND, low_priority )]
unsafe fn rockman_attacks4hi_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_punch_kick_swing_l"))
    }
}

#[acmd_script( agent = "rockman", script = "expression_attacks4hi", category = ACMD_EXPRESSION, low_priority )]
unsafe fn rockman_attacks4hi_exp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f5b14bb65), *FIGHTER_ROCKMAN_ARM_LEFT, *FIGHTER_ROCKMAN_ARMFORM_HAND, 10);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f5b14bb65), *FIGHTER_ROCKMAN_ARM_RIGHT, *FIGHTER_ROCKMAN_ARMFORM_HAND, 10);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_L);
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    }
    frame(fighter.lua_state_agent, 12.0);
    execute(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_L);
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(
            fighter.module_accessor,
            Hash40::new("rbkind_nohitl"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_NONE);
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
    }
    frame(fighter.lua_state_agent, 32.0);
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
    }
    frame(fighter.lua_state_agent, 52.0);
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

#[acmd_script( agent = "rockman", script = "game_attacks4lw", category = ACMD_GAME, low_priority )]
unsafe fn rockman_attacks4lw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("footr"), 9.0, 361, 70, 0, 60, 4.0, 0.0, -2.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("footr"), 6.0, 361, 70, 0, 60, 4.0, 0.0, -2.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "rockman", script = "effect_attacks4lw", category = ACMD_EFFECT, low_priority )]
unsafe fn rockman_attacks4lw_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_sliding_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), -2, 1.5, -7, 0, 8, -5, 1, true, *EF_FLIP_YZ);
    }
}

#[acmd_script( agent = "rockman", script = "sound_attacks4lw", category = ACMD_SOUND, low_priority )]
unsafe fn rockman_attacks4lw_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_punch_kick_swing_m"))
    }
}

#[acmd_script( agent = "rockman", script = "expression_attacks4lw", category = ACMD_EXPRESSION, low_priority )]
unsafe fn rockman_attacks4lw_exp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f5b14bb65), *FIGHTER_ROCKMAN_ARM_LEFT, *FIGHTER_ROCKMAN_ARMFORM_HAND, 10);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f5b14bb65), *FIGHTER_ROCKMAN_ARM_RIGHT, *FIGHTER_ROCKMAN_ARMFORM_HAND, 10);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_L);
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    }
    frame(fighter.lua_state_agent, 12.0);
    execute(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_L);
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(
            fighter.module_accessor,
            Hash40::new("rbkind_nohitm"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_NONE);
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attacks"), 0);
    }
    frame(fighter.lua_state_agent, 32.0);
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
    }
    frame(fighter.lua_state_agent, 46.0);
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

#[acmd_script( agent = "rockman", script = "effect_attacks4charge", category = ACMD_EFFECT, low_priority )]
unsafe fn rockman_attacks4charge_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 12, 0, 4, 0, 0, 0, false);
    }
    for _ in 0..i32::MAX {
        wait(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, 4, 4, 4, 0, 0, 0, false);
        }
    }
}

#[acmd_script( agent = "rockman", script = "sound_attacks4charge", category = ACMD_SOUND, low_priority )]
unsafe fn rockman_attacks4charge_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_smash_start"));
    }
}

#[acmd_script( agent = "rockman", script = "expression_attacks4charge", category = ACMD_EXPRESSION, low_priority )]
unsafe fn rockman_attacks4charge_exp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        physics!(fighter, *MA_MSC_CMD_PHYSICS_START_CHARGE, 0.8, 0.8, -1, 0.8, 0.8, -1, Hash40::new("invalid"));
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_L);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_smashhold1"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    }
    frame(fighter.lua_state_agent, 61.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_smashhold2"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    install_acmd_scripts!(
        rockman_attacks4, rockman_attacks4_eff, rockman_attacks4_snd, rockman_attacks4_exp,
        rockman_attacks4hi, rockman_attacks4hi_eff, rockman_attacks4hi_snd, rockman_attacks4hi_exp,
        rockman_attacks4lw, rockman_attacks4lw_eff, rockman_attacks4lw_snd, rockman_attacks4lw_exp,
        rockman_attacks4charge_eff, rockman_attacks4charge_snd, rockman_attacks4charge_exp
    );
}