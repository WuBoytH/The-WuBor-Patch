use crate::imports::acmd_imports::*;
use super::super::helper::*;

#[acmd_script( agent = "ryu", scripts = [ "game_specialn", "game_specialairn" ], category = ACMD_GAME, low_priority )]
unsafe fn ryu_specialn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.battle_object, ryu::status::flag::SPECIAL_DECIDE_STRENGTH);
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
        MiscModule::calc_motion_rate_from_cancel_frame(agent, 13.0, -14.0);
    }
    else if strength == *FIGHTER_RYU_STRENGTH_M {
        MiscModule::calc_motion_rate_from_cancel_frame(agent, 13.0, -12.0);
    }
    else {
        MiscModule::calc_motion_rate_from_cancel_frame(agent, 13.0, -10.0);
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

#[acmd_script( agent = "ryu", scripts = [ "sound_specialn", "sound_specialairn" ], category = ACMD_SOUND, low_priority )]
unsafe fn ryu_specialn_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_FAILED) {
        wait(agent.lua_state_agent, 10.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_ryu_special_n03"));
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("vc_ryu_special_n03"));
        }
    }
    else {
        if macros::is_excute(agent) {
            if WorkModule::is_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND) {
                macros::PLAY_SE(agent, Hash40::new("se_ryu_command_success"));
            }
            macros::PLAY_SE(agent, Hash40::new("se_ryu_special_n01"));
        }
        wait(agent.lua_state_agent, 10.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_ryu_special_n03"));
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            let se = if VarModule::is_flag(agent.battle_object, ryu::status::flag::USED_DENJIN_CHARGE) {
                hash40("vc_ryu_hadoken_denjin")
            }
            else if WorkModule::is_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND) {
                hash40("vc_ryu_special_n01_command")
            }
            else {
                hash40("vc_ryu_special_n01")
            };
            macros::PLAY_SE(agent, Hash40::new_raw(se));
        }
    }
}

#[acmd_script( agent = "ryu_hadoken", scripts = [ "game_movew", "game_movem", "game_moves" ], category = ACMD_GAME, low_priority )]
unsafe fn ryu_hadoken_move(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 65, 10, 0, 65, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, -1.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
        AttackModule::enable_safe_pos(agent.module_accessor);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 2.2);
    }
}

#[acmd_script( agent = "ryu_hadoken", scripts = [ "game_movespw", "game_movespm", "game_movesps" ], category = ACMD_GAME, low_priority )]
unsafe fn ryu_hadoken_movesp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 361, 10, 0, 42, 4.0, 0.0, 0.0, 0.0, None, None, None, 3.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        AttackModule::enable_safe_pos(agent.module_accessor);
    }
}

#[acmd_script( agent = "ryu_hadoken", scripts = ["game_movespw_last", "game_movespm_last", "game_movesps_last"], category = ACMD_GAME, low_priority )]
unsafe fn ryu_hadoken_movesp_last(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 361, 40, 0, 60, 5.0, 0.0, 0.0, -1.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 2.2);
    }
}

#[acmd_script( agent = "ryu", scripts = [ "game_specialn2", "game_specialairn2" ], category = ACMD_GAME, low_priority )]
unsafe fn ryu_specialn2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 8.0 / 14.0);
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.battle_object, ryu::status::flag::SPECIAL_DECIDE_STRENGTH);
    }
    let strength = WorkModule::get_int(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH);
    if VarModule::is_flag(agent.battle_object, ryu::status::flag::USED_DENJIN_CHARGE) {
        macros::FT_MOTION_RATE(agent, 11.0 / 9.0);
    }
    else if strength == *FIGHTER_RYU_STRENGTH_W {
        macros::FT_MOTION_RATE(agent, 4.0 / 9.0);
    }
    else if strength == *FIGHTER_RYU_STRENGTH_M {
        macros::FT_MOTION_RATE(agent, 9.0 / 9.0);
    }
    else {
        macros::FT_MOTION_RATE(agent, 20.0 / 9.0);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
    }
    frame(agent.lua_state_agent, 24.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    let strength = WorkModule::get_int(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH);
    if VarModule::is_flag(agent.battle_object, ryu::status::flag::USED_DENJIN_CHARGE) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 367, 40, 30, 0, 4.0, 0.0, 11.0, 8.0, None, None, None, 1.2, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        }
    }
    else if strength == *FIGHTER_RYU_STRENGTH_W {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 361, 50, 0, 50, 4.0, 0.0, 11.0, 8.0, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
        }
    }
    else if strength == *FIGHTER_RYU_STRENGTH_M {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 361, 75, 0, 62, 4.0, 0.0, 11.0, 8.0, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 367, 40, 30, 0, 4.0, 0.0, 11.0, 8.0, None, None, None, 1.2, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
        }
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    let strength = WorkModule::get_int(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH);
    if VarModule::is_flag(agent.battle_object, ryu::status::flag::USED_DENJIN_CHARGE) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 69, 15, 0, 80, 5.0, 0.0, 11.0, 8.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        }
    }
    else if strength == *FIGHTER_RYU_STRENGTH_S {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 69, 15, 0, 80, 5.0, 0.0, 11.0, 8.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
        }
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    if !VarModule::is_flag(agent.battle_object, ryu::status::flag::USED_DENJIN_CHARGE) {
        let strength = WorkModule::get_int(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH);
        if strength == *FIGHTER_RYU_STRENGTH_W {
            MiscModule::calc_motion_rate_from_cancel_frame(agent, 30.0, -1.0);
        }
        else if strength == *FIGHTER_RYU_STRENGTH_M {
            MiscModule::calc_motion_rate_from_cancel_frame(agent, 30.0, -2.0);
        }
        else {
            MiscModule::calc_motion_rate_from_cancel_frame(agent, 30.0, 1.0);
        }
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(agent.lua_state_agent, 50.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
    }
}

#[acmd_script( agent = "ryu", scripts = [ "effect_specialn2", "effect_specialairn2" ], category = ACMD_EFFECT, low_priority )]
unsafe fn ryu_specialn2_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 11, -7, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("ryu_hadoken_hold"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::LAST_EFFECT_SET_RATE(agent, 0.4);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        if VarModule::is_flag(agent.battle_object, ryu::status::flag::USED_DENJIN_CHARGE) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_hit_elec"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.3, true);
            macros::LAST_EFFECT_SET_RATE(agent, 0.5);
        }
    }
    let strength = WorkModule::get_int(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH);
    let shot_frame = if VarModule::is_flag(agent.battle_object, ryu::status::flag::USED_DENJIN_CHARGE) {
        20.0
    }
    else if strength == *FIGHTER_RYU_STRENGTH_W {
        15.0
    }
    else if strength == *FIGHTER_RYU_STRENGTH_M {
        16.0
    }
    else {
        22.0
    };
    frame(agent.lua_state_agent, shot_frame);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("ryu_hadoken_shot"), Hash40::new("top"), 0, 11.5, 8, 0, 0, 0, 1, true);
        macros::LAST_EFFECT_SET_RATE(agent, 0.6);
    }
}

#[acmd_script( agent = "ryu", scripts = [ "sound_specialn2", "sound_specialairn2" ], category = ACMD_SOUND, low_priority )]
unsafe fn ryu_specialn2_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ryu_hashogeki_start"));
    }
    frame(agent.lua_state_agent, 24.0);
    let strength = WorkModule::get_int(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH);
    if VarModule::is_flag(agent.battle_object, ryu::status::flag::USED_DENJIN_CHARGE) {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_ryu_hashogeki_denjin"));
            macros::PLAY_SE(agent, Hash40::new("vc_ryu_hashogeki_denjin"));
        }
    }
    else if strength == *FIGHTER_RYU_STRENGTH_W {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_ryu_hashogeki"));
            macros::PLAY_SE(agent, Hash40::new("vc_ryu_hashogeki_l"));
        }
    }
    else if strength == *FIGHTER_RYU_STRENGTH_M {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_ryu_hashogeki"));
            macros::PLAY_SE(agent, Hash40::new("vc_ryu_hashogeki_m"));
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_ryu_hashogeki"));
            macros::PLAY_SE(agent, Hash40::new("vc_ryu_hashogeki_s"));
        }
    }
}

#[acmd_script( agent = "ryu", scripts = [ "expression_specialn2", "expression_specialairn2" ], category = ACMD_EXPRESSION, low_priority )]
unsafe fn ryu_specialn2_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 20.0);
    let strength = WorkModule::get_int(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH);
    if VarModule::is_flag(agent.battle_object, ryu::status::flag::USED_DENJIN_CHARGE)
    || strength == *FIGHTER_RYU_STRENGTH_S {
        if macros::is_excute(agent) {
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attackll"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
    }
    else if strength == *FIGHTER_RYU_STRENGTH_M {
        if macros::is_excute(agent) {
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attackl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
    }
    else {
        if macros::is_excute(agent) {
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
    }
    if macros::is_excute(agent) {
        macros::AREA_WIND_2ND_arg10(agent, 0, 1.2, 110, 8, 0.8, -2, 8, 24, 16, 80);
    }
    frame(agent.lua_state_agent, 24.0);
    let strength = WorkModule::get_int(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH);
    if VarModule::is_flag(agent.battle_object, ryu::status::flag::USED_DENJIN_CHARGE)
    || strength == *FIGHTER_RYU_STRENGTH_S {
        if macros::is_excute(agent) {
            macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
        }
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        AreaModule::erase_wind(agent.module_accessor, 0);
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
    }
}

#[acmd_script( agent = "ryu", scripts = [ "game_speciallw", "game_specialairlw" ], category = ACMD_GAME, low_priority )]
unsafe fn ryu_speciallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 51.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.battle_object, ryu::instance::flag::DENJIN_CHARGE);
    }
}

#[acmd_script( agent = "ryu", scripts = [ "effect_speciallw", "effect_specialairlw" ], category = ACMD_EFFECT, low_priority )]
unsafe fn ryu_speciallw_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        agent.clear_lua_stack();
        EFFECT_STENCIL_ON(agent.lua_state_agent);
        macros::BURN_COLOR(agent, 0.02, 0.15, 2.0, 0);
        macros::BURN_COLOR_FRAME(agent, 1, 0.02, 0.15, 2.0, 0.7);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        let eff_handle = VarModule::get_int(agent.battle_object, ryu::instance::int::DENJIN_EFF_HANDLE) as u32;
        if EffectModule::is_exist_effect(agent.module_accessor, eff_handle) {
            EffectModule::kill(agent.module_accessor, eff_handle, true, true);
        }
        EffectModule::req_follow(
            agent.module_accessor,
            Hash40::new("ryu_savingattack_aura"),
            Hash40::new("bust"),
            &ZERO_VECTOR,
            &ZERO_VECTOR,
            1.5,
            false,
            0,
            0,
            0,
            0,
            0,
            false,
            false
        );
        let eff_handle = EffectModule::get_last_handle(agent.module_accessor);
        EffectModule::set_rgb(agent.module_accessor, eff_handle as u32, 0.3, 0.3, 2.0);
        EffectModule::set_rate(agent.module_accessor, eff_handle as u32, 1.5);
        VarModule::set_int(agent.battle_object, ryu::instance::int::DENJIN_EFF_HANDLE, eff_handle as i32);
    }
    frame(agent.lua_state_agent, 52.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_hit_elec"), Hash40::new("top"), 0.0, 10.0, 0.0, 0.0, 0.0, 0.0, 0.6, true);
        macros::BURN_COLOR_NORMAL(agent);
        agent.clear_lua_stack();
        EFFECT_STENCIL_OFF(agent.lua_state_agent);
    }
}

#[acmd_script( agent = "ryu", scripts = [ "sound_speciallw", "sound_specialairlw" ], category = ACMD_SOUND, low_priority )]
unsafe fn ryu_speciallw_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ryu_denjin_charge"));
        macros::PLAY_SE(agent, Hash40::new("vc_ryu_denjin_charge"));
    }
}

#[acmd_script( agent = "ryu", scripts = [ "expression_speciallw", "expression_specialairlw" ], category = ACMD_EXPRESSION, low_priority )]
unsafe fn ryu_speciallw_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

#[acmd_script( agent = "ryu", script = "game_speciallwimpact", category = ACMD_GAME, low_priority )]
unsafe fn ryu_speciallwimpact(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 26.0 / 21.0);
    frame(agent.lua_state_agent, 21.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.battle_object, ryu::status::flag::SPECIAL_LW_IMPACT_REMOVE_ARMOR);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_INT_SUPER_ARMOUR_COUNT) == 2 {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 361, 100, 10, 65, 4.0, 0.0, 10.0, 3.0, Some(0.0), Some(10.0), Some(8.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 10, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 361, 100, 10, 65, 4.0, 0.0, 10.0, 3.0, Some(0.0), Some(10.0), Some(8.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 10, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_PUNCH);
            AttackModule::set_attack_level(agent.module_accessor, 0, *FIGHTER_RYU_SAVING_LV_2 as u8);
        }
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

#[acmd_script( agent = "ryu", script = "effect_speciallwimpact", category = ACMD_EFFECT, low_priority )]
unsafe fn ryu_speciallwimpact_eff(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.4, true);
        macros::LAST_EFFECT_SET_RATE(agent, 0.75);
        ryu_saving_aura_handler(agent, 0.1, 0.1, 0.1);
    }
    frame(agent.lua_state_agent, 18.0);
    if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("ryu_savingattack_line_l"), Hash40::new("top"), 4, 11.5, 2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
    else{
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("ryu_savingattack_line_r"), Hash40::new("top"), -5.2, 11.5, 2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        macros::BURN_COLOR_NORMAL(agent);
        agent.clear_lua_stack();
        EFFECT_STENCIL_OFF(agent.lua_state_agent);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("ryu_savingattack_aura"), false, false);
    }
}

#[acmd_script( agent = "ryu", script = "sound_speciallwimpact", category = ACMD_SOUND, low_priority )]
unsafe fn ryu_speciallwimpact_snd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ryu_drive_impact"));
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_ryu_drive_impact"));
    }
    frame(agent.lua_state_agent, 23.0);
    if VarModule::is_flag(agent.battle_object, ryu::status::flag::SPECIAL_LW_IMPACT_HIT) {
        if macros::is_excute(agent) {
            let mut vc_type = VarModule::get_int(agent.battle_object, ryu::instance::int::IMPACT_PUNISH_VC_TYPE);
            let vc = if vc_type == 0 {
                hash40("vc_ryu_drive_impact_counter01")
            }
            else if vc_type == 1 {
                hash40("vc_ryu_drive_impact_counter02")
            }
            else {
                hash40("vc_ryu_drive_impact_counter03")
            };
            vc_type += 1;
            if vc_type > 2 {
                vc_type = 0;
            }
            VarModule::set_int(agent.battle_object, ryu::instance::int::IMPACT_PUNISH_VC_TYPE, vc_type);
            macros::PLAY_SE(agent, Hash40::new_raw(vc));
        }
    }
}

#[acmd_script( agent = "ryu", script = "expression_speciallwimpact", category = ACMD_EXPRESSION, low_priority )]
unsafe fn ryu_speciallwimpact_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitll_l"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
}

#[acmd_script( agent = "ryu", script = "effect_speciallwimpactarmor", category = ACMD_EFFECT, low_priority )]
unsafe fn ryu_speciallwimpactarmor_eff(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        agent.clear_lua_stack();
        EFFECT_STENCIL_ON(agent.lua_state_agent);
        macros::BURN_COLOR(agent, 2.0, 0.4, 0.4, 1);
        macros::BURN_COLOR_FRAME(agent, 1, 2.0, 0.4, 0.4, 1.0);
    }
}

#[acmd_script( agent = "ryu", script = "game_speciallwimpactonshield", category = ACMD_GAME, low_priority )]
unsafe fn ryu_speciallwimpactonshield(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 6, 1, Hash40::new("top"), 0.0, 50, 100, 65, 0, 12.0, 0.0, 10.0, 10.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, f32::NAN, -1.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 6, 10.0, false);
    }
}

#[acmd_script( agent = "ryu", script = "game_speciallwreversal", category = ACMD_GAME, low_priority )]
unsafe fn ryu_speciallwreversal(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 361, 0, 0, 80, 4.0, 0.0, 10.0, 3.0, Some(0.0), Some(10.0), Some(6.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(agent.module_accessor);
    }
}

#[acmd_script( agent = "ryu", script = "effect_speciallwreversal", category = ACMD_EFFECT, low_priority )]
unsafe fn ryu_speciallwreversal_eff(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_flash"), Hash40::new("top"), 0.0, 12.0, 6.0, 0.0, 0.0, 0.0, 1.4, true);
        ryu_saving_aura_handler(agent, 0.1, 0.1, 0.1);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("ryu_savingattack_aura"), false, false);
    }
}

#[acmd_script( agent = "ryu", script = "sound_speciallwreversal", category = ACMD_SOUND, low_priority )]
unsafe fn ryu_speciallwreversal_snd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ryu_drive_reversal"));
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_ryu_drive_reversal"));
    }
}

#[acmd_script( agent = "ryu", script = "expression_speciallwimpact", category = ACMD_EXPRESSION, low_priority )]
unsafe fn ryu_speciallwreversal_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitll"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
}

#[acmd_script( agent = "ryu", scripts = [ "game_speciallwrush", "game_specialairlwrush" ], category = ACMD_GAME, low_priority )]
unsafe fn ryu_speciallwrush(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::SEARCH(agent, 0, 0, Hash40::new("top"), 10.0, 0.0, 10.0, 0.0, Some(0.0), Some(10.0), Some(5.0), *COLLISION_KIND_MASK_AHS, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.battle_object, ryu::status::flag::SPECIAL_LW_RUSH_RESUME_ENERGY);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.battle_object, ryu::instance::flag::DENJIN_RUSH_INHERIT);
        VarModule::on_flag(agent.battle_object, ryu::status::flag::SPECIAL_LW_RUSH_ENABLE_ATTACK);
    }
    frame(agent.lua_state_agent, 50.0);
    if macros::is_excute(agent) {
        VarModule::off_flag(agent.battle_object, ryu::instance::flag::DENJIN_RUSH_INHERIT);
    }
}

#[acmd_script( agent = "ryu", scripts = [ "effect_speciallwrush", "effect_specialairlwrush" ], category = ACMD_EFFECT, low_priority )]
unsafe fn ryu_speciallwrush_eff(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ryu_saving_aura_handler(agent, 0.7, 1.0, 0.7);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 50.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("ryu_savingattack_aura"), false, false);
    }
}

#[acmd_script( agent = "ryu", scripts = [ "sound_speciallwrush", "sound_specialairlwrush" ], category = ACMD_SOUND, low_priority )]
unsafe fn ryu_speciallwrush_snd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ryu_drive_rush01"));
        let mut vc_type = VarModule::get_int(agent.battle_object, ryu::instance::int::RUSH_VC_TYPE);
        let vc = if vc_type == 0 {
            hash40("vc_ryu_drive_rush01")
        }
        else {
            hash40("vc_ryu_drive_rush02")
        };
        vc_type += 1;
        if vc_type > 1 {
            vc_type = 0;
        }
        VarModule::set_int(agent.battle_object, ryu::instance::int::RUSH_VC_TYPE, vc_type);
        macros::PLAY_SE(agent, Hash40::new_raw(vc));
    }
}

pub fn install() {
    install_acmd_scripts!(
        ryu_specialn,
        ryu_specialn_snd,

        ryu_hadoken_move,
        ryu_hadoken_movesp,
        ryu_hadoken_movesp_last,

        ryu_specialn2,
        ryu_specialn2_eff,
        ryu_specialn2_snd,
        ryu_specialn2_exp,

        ryu_speciallw,
        ryu_speciallw_eff,
        ryu_speciallw_snd,
        ryu_speciallw_exp,

        ryu_speciallwimpact,
        ryu_speciallwimpact_eff,
        ryu_speciallwimpact_snd,
        ryu_speciallwimpact_exp,

        ryu_speciallwimpactarmor_eff,

        ryu_speciallwimpactonshield,

        ryu_speciallwreversal,
        ryu_speciallwreversal_eff,
        ryu_speciallwreversal_snd,
        ryu_speciallwreversal_exp,

        ryu_speciallwrush,
        ryu_speciallwrush_eff,
        ryu_speciallwrush_snd
    );
}