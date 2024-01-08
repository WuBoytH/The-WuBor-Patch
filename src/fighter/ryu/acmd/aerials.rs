use crate::imports::acmd_imports::*;
use super::super::helper::*;

unsafe extern "C" fn ryu_attackairn(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        if VarModule::is_flag(agent.module_accessor, ryu::status::flag::USED_DENJIN_CHARGE) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 361, 100, 10, 20, 4.3, 0.0, 4.2, 4.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 361, 100, 10, 20, 4.3, 0.0, 4.2, 4.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 8.0, 45, 100, 10, 20, 4.3, 0.0, 8.7, -1.6, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(agent, 3, 0, Hash40::new("top"), 8.0, 45, 100, 10, 20, 4.3, 0.0, 8.7, -1.6, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
        }
        else {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 361, 100, 0, 20, 4.3, 0.0, 4.2, 4.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 361, 100, 0, 20, 4.3, 0.0, 8.7, -1.6, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
        }
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        if VarModule::is_flag(agent.module_accessor, ryu::status::flag::USED_DENJIN_CHARGE) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.5, 361, 90, 10, 13, 3.0, 0.0, 4.2, 4.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 4.5, 361, 90, 10, 13, 3.0, 0.0, 4.2, 4.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 4.5, 45, 90, 10, 13, 3.0, 0.0, 8.7, -1.6, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(agent, 3, 0, Hash40::new("top"), 4.5, 45, 90, 10, 13, 3.0, 0.0, 8.7, -1.6, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
        }
        else {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.5, 361, 90, 0, 13, 3.0, 0.0, 4.2, 4.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 4.5, 361, 90, 0, 13, 3.0, 0.0, 8.7, -1.6, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
        }
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn ryu_attackairn_eff(agent: &mut L2CAgentBase) {
    if VarModule::is_flag(agent.module_accessor, ryu::status::flag::USED_DENJIN_CHARGE) {
        if macros::is_excute(agent) {
            ryu_saving_aura_handler(agent, 0.1, 1.0, 0.2);
        }
    }
    frame(agent.lua_state_agent, 32.0);
    if VarModule::is_flag(agent.module_accessor, ryu::status::flag::USED_DENJIN_CHARGE) {
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("ryu_savingattack_aura"), false, false);
        }
    }
}

unsafe extern "C" fn ryu_attackairf(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 12.0);
    macros::FT_MOTION_RATE(agent, 15.0 / 9.0);
    if macros::is_excute(agent) {
        if VarModule::is_flag(agent.module_accessor, ryu::status::flag::USED_DENJIN_CHARGE) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 61, 56, 10, 70, 4.3, 0.0, 5.5, 10.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 71, 25, 10, 65, 3.8, 0.0, 5.5, 2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 15, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
        }
        else {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 61, 56, 0, 70, 4.3, 0.0, 5.5, 10.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 70, 25, 0, 65, 3.8, 0.0, 5.5, 2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 15, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
        }
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        if VarModule::is_flag(agent.module_accessor, ryu::status::flag::USED_DENJIN_CHARGE) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 61, 56, 10, 70, 4.0, 0.0, 5.5, 10.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 70, 25, 10, 65, 4.6, 0.0, 5.5, 2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 15, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
        }
        else {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 61, 56, 0, 70, 4.0, 0.0, 5.5, 10.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 70, 25, 0, 65, 4.6, 0.0, 5.5, 2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 15, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
        }
    }
    frame(agent.lua_state_agent, 21.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 41.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn ryu_attackairf_eff(agent: &mut L2CAgentBase) {
    if VarModule::is_flag(agent.module_accessor, ryu::status::flag::USED_DENJIN_CHARGE) {
        if macros::is_excute(agent) {
            ryu_saving_aura_handler(agent, 0.1, 1.0, 0.2);
        }
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 1, 5, 12, 0, 0, 0, 1, false, 0.8);
    }
    frame(agent.lua_state_agent, 21.0);
    if VarModule::is_flag(agent.module_accessor, ryu::status::flag::USED_DENJIN_CHARGE) {
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("ryu_savingattack_aura"), false, false);
        }
    }
}

unsafe extern "C" fn ryu_attackairf_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ryu_swing_kick_l"));
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_ryu_rnd_attack_l"));
    }
}

unsafe extern "C" fn ryu_attackairf_exp(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

unsafe extern "C" fn ryu_attackairb(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 16.0, 361, 90, 0, 21, 5.6, 0.0, 11.0, -13.0, Some(0.0), Some(11.0), Some(-7.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 13.0, 361, 90, 0, 21, 4.6, 0.0, 11.0, -15.0, Some(0.0), Some(11.0), Some(-7.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn ryu_attackairb_eff(agent: &mut L2CAgentBase) {
    if VarModule::is_flag(agent.module_accessor, ryu::status::flag::USED_DENJIN_CHARGE) {
        if macros::is_excute(agent) {
            ryu_saving_aura_handler(agent, 0.1, 1.0, 0.2);
        }
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("ryu_attack_arc"), Hash40::new("ryu_attack_arc"), Hash40::new("top"), 0, 9.5, -6, -8, 140, -20, 1, true, *EF_FLIP_YZ, 0.5);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 10.0);
    if VarModule::is_flag(agent.module_accessor, ryu::status::flag::USED_DENJIN_CHARGE) {
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("ryu_savingattack_aura"), false, false);
        }
    }
}

unsafe extern "C" fn ryu_attackairhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    macros::FT_MOTION_RATE(agent, 6.0 / 4.0);
    frame(agent.lua_state_agent, 6.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 366, 100, 20, 0, 3.5, 0.0, 13.0, 5.1, Some(0.0), Some(16.0), Some(5.1), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
        macros::HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 75, 172, 0, 0, 4.5, 0.0, 15.0, 4.0, Some(0.0), Some(17.0), Some(4.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn ryu_attackairhi_eff(agent: &mut L2CAgentBase) {
    if VarModule::is_flag(agent.module_accessor, ryu::status::flag::USED_DENJIN_CHARGE) {
        if macros::is_excute(agent) {
            ryu_saving_aura_handler(agent, 0.1, 1.0, 0.2);
        }
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("ryu_attack_arc"), Hash40::new("ryu_attack_arc"), Hash40::new("top"), 1, 13, 0, 0, -20, 120, 0.6, true, *EF_FLIP_YZ, 0.5);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 4, 18, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 360, false, 0.4);
    }
    wait(agent.lua_state_agent, 3.0);
    if VarModule::is_flag(agent.module_accessor, ryu::status::flag::USED_DENJIN_CHARGE) {
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("ryu_savingattack_aura"), false, false);
        }
    }
}

unsafe extern "C" fn ryu_attackairlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    macros::FT_MOTION_RATE(agent, 5.0 / 3.0);
    frame(agent.lua_state_agent, 6.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 300, 80, 0, 10, 2.7, 0.0, 3.0, 9.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 12.0, 84, 14, 0, 60, 4.4, 0.0, 3.0, 9.5, Some(0.0), Some(8.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 11.0, 48, 80, 0, 20, 4.4, 0.0, 3.0, 9.5, Some(0.0), Some(8.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 33.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn ryu_attackairlw_eff(agent: &mut L2CAgentBase) {
    if VarModule::is_flag(agent.module_accessor, ryu::status::flag::USED_DENJIN_CHARGE) {
        if macros::is_excute(agent) {
            ryu_saving_aura_handler(agent, 0.1, 1.0, 0.2);
        }
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 13.8, 0, 50, 0, 0, 0.8, true, *EF_FLIP_YZ, 0.5);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLIP_ALPHA(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), -0.5, 10.7, 2.5, 50, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ, 0.8);
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 2.5, 9, 0, 0, 0, 0.8, false, 0.5);
    }
    wait(agent.lua_state_agent, 5.0);
    if VarModule::is_flag(agent.module_accessor, ryu::status::flag::USED_DENJIN_CHARGE) {
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("ryu_savingattack_aura"), false, false);
        }
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.game_acmd("game_attackairn", ryu_attackairn);
    agent.effect_acmd("effect_attackairn", ryu_attackairn_eff);

    agent.game_acmd("game_attackairf", ryu_attackairf);
    agent.effect_acmd("effect_attackairf", ryu_attackairf_eff);
    agent.sound_acmd("sound_attackairf", ryu_attackairf_snd);
    agent.expression_acmd("expression_attackairf", ryu_attackairf_exp);

    agent.game_acmd("game_attackairb", ryu_attackairb);
    agent.effect_acmd("effect_attackairb", ryu_attackairb_eff);

    agent.game_acmd("game_attackairhi", ryu_attackairhi);
    agent.effect_acmd("effect_attackairhi", ryu_attackairhi_eff);

    agent.game_acmd("game_attackairlw", ryu_attackairlw);
    agent.effect_acmd("effect_attackairlw", ryu_attackairlw_eff);
}