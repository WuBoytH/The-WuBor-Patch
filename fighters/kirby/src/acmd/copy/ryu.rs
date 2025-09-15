use super::*;

extern "C" {
    #[link_name = "ryu_expression_specialn2"]
    fn ryu_expression_specialn2(agent: &mut L2CAgentBase);
}

unsafe extern "C" fn game_ryuspecialn2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 8.0 / 14.0);
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, vars::ryu::status::flag::SPECIAL_DECIDE_STRENGTH);
    }
    let strength = WorkModule::get_int(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH);
    if VarModule::is_flag(agent.module_accessor, vars::ryu::status::flag::USED_DENJIN_CHARGE) {
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
    if VarModule::is_flag(agent.module_accessor, vars::ryu::status::flag::USED_DENJIN_CHARGE) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 367, 40, 30, 0, 4.0, 0.0, 5.0, 6.0, None, None, None, 1.2, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
        }
    }
    else if strength == *FIGHTER_RYU_STRENGTH_W {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 361, 50, 0, 50, 4.0, 0.0, 5.0, 6.0, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
        }
    }
    else if strength == *FIGHTER_RYU_STRENGTH_M {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 361, 75, 0, 62, 4.0, 0.0, 5.0, 6.0, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 367, 40, 30, 0, 4.0, 0.0, 5.0, 6.0, None, None, None, 1.2, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
        }
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    let strength = WorkModule::get_int(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH);
    if VarModule::is_flag(agent.module_accessor, vars::ryu::status::flag::USED_DENJIN_CHARGE) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 69, 15, 0, 80, 7.0, 0.0, 5.0, 6.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
            macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 2.0);
        }
    }
    else if strength == *FIGHTER_RYU_STRENGTH_S {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 69, 15, 0, 80, 7.0, 0.0, 5.0, 6.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
            macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 2.0);
        }
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    if !VarModule::is_flag(agent.module_accessor, vars::ryu::status::flag::USED_DENJIN_CHARGE) {
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

unsafe extern "C" fn effect_ryuspecialn2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 11, -7, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("ryu_hadoken_hold"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::LAST_EFFECT_SET_RATE(agent, 0.4);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        if VarModule::is_flag(agent.module_accessor, vars::ryu::status::flag::USED_DENJIN_CHARGE) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_hit_elec"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.3, true);
            macros::LAST_EFFECT_SET_RATE(agent, 0.5);
        }
    }
    let strength = WorkModule::get_int(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH);
    let shot_frame = if VarModule::is_flag(agent.module_accessor, vars::ryu::status::flag::USED_DENJIN_CHARGE) {
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
        macros::EFFECT_FOLLOW(agent, Hash40::new("ryu_hadoken_shot"), Hash40::new("top"), 0, 5.5, 6, 0, 0, 0, 1, true);
        macros::LAST_EFFECT_SET_RATE(agent, 0.6);
    }
}

unsafe extern "C" fn sound_ryuspecialn2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ryu_hashogeki_start"));
    }
    frame(agent.lua_state_agent, 24.0);
    let strength = WorkModule::get_int(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH);
    if VarModule::is_flag(agent.module_accessor, vars::ryu::status::flag::USED_DENJIN_CHARGE) {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_ryu_hashogeki_denjin"));
            // macros::PLAY_SE(agent, Hash40::new("vc_ryu_hashogeki_denjin"));
        }
    }
    else if strength == *FIGHTER_RYU_STRENGTH_W {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_ryu_hashogeki"));
            // macros::PLAY_SE(agent, Hash40::new("vc_ryu_hashogeki_l"));
        }
    }
    else if strength == *FIGHTER_RYU_STRENGTH_M {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_ryu_hashogeki"));
            // macros::PLAY_SE(agent, Hash40::new("vc_ryu_hashogeki_m"));
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_ryu_hashogeki"));
            // macros::PLAY_SE(agent, Hash40::new("vc_ryu_hashogeki_s"));
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_ryuspecialn2", game_ryuspecialn2, Priority::Low);
    agent.acmd("effect_ryuspecialn2", effect_ryuspecialn2, Priority::Low);
    agent.acmd("sound_ryuspecialn2", sound_ryuspecialn2, Priority::Low);
    agent.acmd("expression_ryuspecialn2", ryu_expression_specialn2, Priority::Low);

    agent.acmd("game_ryuspecialairn2", game_ryuspecialn2, Priority::Low);
    agent.acmd("effect_ryuspecialairn2", effect_ryuspecialn2, Priority::Low);
    agent.acmd("sound_ryuspecialairn2", sound_ryuspecialn2, Priority::Low);
    agent.acmd("expression_ryuspecialairn2", ryu_expression_specialn2, Priority::Low);
}