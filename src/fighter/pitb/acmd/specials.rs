use crate::imports::*;

unsafe extern "C" fn game_specialnstart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_PITB_GENERATE_ARTICLE_BOWARROW, false, 0);
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_PITB_GENERATE_ARTICLE_BOWARROW, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn expression_specialnstart(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_status_default_int64(agent.module_accessor, hash40("agent") as i64, hash40("agent_final") as i64);
    }
}

unsafe extern "C" fn effect_specialnhold(agent: &mut L2CAgentBase) {
    for _ in 0..10 {
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 15, 0, 4, 0, 0, 0, false);
        }
        wait(agent.lua_state_agent, 5.0);
    }
}

unsafe extern "C" fn effect_specialairnhold(_agent: &mut L2CAgentBase) {
}

unsafe extern "C" fn game_specialnfire(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_PITB_GENERATE_ARTICLE_BOWARROW, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::shoot(agent.module_accessor, *FIGHTER_PITB_GENERATE_ARTICLE_BOWARROW, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
}

unsafe extern "C" fn effect_specialnfirehi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_v"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_FLAG_CHARGE_MAX) {
        macros::EFFECT(agent, Hash40::new("pitb_pa_max_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT(agent, Hash40::new("pitb_pa_max"), Hash40::new("top"), 0, 14, 9, -22, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn effect_specialnairfirehi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_FLAG_CHARGE_MAX) {
        macros::EFFECT(agent, Hash40::new("pitb_pa_max_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT(agent, Hash40::new("pitb_pa_max"), Hash40::new("top"), 0, 14, 9, -22, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn expression_specialnfires(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_FLAG_CHARGE_MAX) {
        if macros::is_excute(agent) {
            macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        }
    }
    frame(agent.lua_state_agent, 5.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_FLAG_CHARGE_MAX) {
        if macros::is_excute(agent) {
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
    }
    else {
        if macros::is_excute(agent) {
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attacks"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("agent") as i64, hash40("agent_none") as i64);
    }
    frame(agent.lua_state_agent, 52.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_status_default_int64(agent.module_accessor, hash40("agent") as i64, hash40("agent_normal") as i64);
    }
}

unsafe extern "C" fn game_specialsend(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_PIT_REFLECTOR_KIND_SPECIAL_S, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_S);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 40, 67, 0, 99, 7.0, 0.0, 4.0, 9.0, Some(0.0), Some(10.0), Some(9.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_PIT_REFLECTOR_KIND_SPECIAL_S, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_S);
    }
}

unsafe extern "C" fn game_specialairsend(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_PIT_REFLECTOR_KIND_SPECIAL_S, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_S);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 9.5, 40, 58, 0, 90, 7.0, 5.0, 4.0, 9.0, Some(5.0), Some(10.0), Some(9.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        KineticModule::add_speed(agent.module_accessor, &Vector3f{x: -0.5, y: 2.5, z: 0.0});
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_PIT_REFLECTOR_KIND_SPECIAL_S, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_S);
    }
    frame(agent.lua_state_agent, 22.0);
    macros::FT_MOTION_RATE(agent, 0.83);
}

unsafe extern "C" fn game_specialhistart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    macros::FT_MOTION_RATE(agent, 2.0/3.0);
    frame(agent.lua_state_agent, 12.0);
    macros::FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
        macros::ATTACK(agent, 0, 1, Hash40::new("rot"), 1.4, 100, 100, 150, 0, 4.0, 0.0, 0.0, 6.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 1, Hash40::new("rot"), 1.4, 100, 100, 150, 0, 4.0, 0.0, 0.0, -6.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 2, 1, Hash40::new("rot"), 1.4, 100, 100, 170, 0, 3.5, 0.0, -5.0, 6.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 3, 1, Hash40::new("rot"), 1.4, 100, 100, 170, 0, 3.5, 0.0, -5.0, -6.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        AttackModule::set_no_damage_fly_smoke_all(agent.module_accessor, true, false);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        // notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        AttackModule::clear_all(agent.module_accessor);
        macros::ATTACK(agent, 0, 1, Hash40::new("rot"), 6.0, 80, 70, 0, 105, 5.0, 0.0, 3.0, 6.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 1, Hash40::new("rot"), 6.0, 80, 70, 0, 105, 5.0, 0.0, 3.0, -6.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    macros::FT_MOTION_RATE(agent, 1.0/11.0);
    frame(agent.lua_state_agent, 44.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 45.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLAG_FIX_ANGLE);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLAG_BACK_ANGLE);
        JostleModule::set_status(agent.module_accessor, false);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn game_specialhiend(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn game_dspecialhold(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_LW);
        shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, 1, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_LW);
        macros::ATTACK(agent, 0, 0, Hash40::new_raw(0x10489b2b69), 5.0, 45, 30, 0, 110, 3.0, 0.0, -2.0, 6.0, Some(0.0), Some(6.0), Some(6.0), 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 0, Hash40::new_raw(0x104ff6ef70), 5.0, 45, 30, 0, 110, 3.0, 0.0, -2.0, -6.0, Some(0.0), Some(6.0), Some(-6.0), 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_OBJECT);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialnstart", game_specialnstart, Priority::Low);
    agent.acmd("expression_specialnstart", expression_specialnstart, Priority::Low);

    agent.acmd("game_specialairnstart", game_specialnstart, Priority::Low);
    agent.acmd("expression_specialairnstart", expression_specialnstart, Priority::Low);

    agent.acmd("effect_specialnholds", effect_specialnhold, Priority::Low);

    agent.acmd("effect_specialnholdhi", effect_specialnhold, Priority::Low);

    agent.acmd("effect_specialnstos", effect_specialairnhold, Priority::Low);

    agent.acmd("effect_specialairnstos", effect_specialairnhold, Priority::Low);

    agent.acmd("effect_specialairnholds", effect_specialairnhold, Priority::Low);

    agent.acmd("effect_specialairnholdhi", effect_specialairnhold, Priority::Low);

    agent.acmd("effect_specialnstohi", effect_specialairnhold, Priority::Low);

    agent.acmd("effect_specialairnstohi", effect_specialairnhold, Priority::Low);

    agent.acmd("effect_specialnhitos", effect_specialairnhold, Priority::Low);

    agent.acmd("effect_specialairnhitos", effect_specialairnhold, Priority::Low);

    agent.acmd("game_specialnfires", game_specialnfire, Priority::Low);
    agent.acmd("expression_specialnfires", expression_specialnfires, Priority::Low);

    agent.acmd("game_specialairnfires", game_specialnfire, Priority::Low);
    agent.acmd("expression_specialairnfires", expression_specialnfires, Priority::Low);

    agent.acmd("game_specialnfirehi", game_specialnfire, Priority::Low);
    agent.acmd("effect_specialnfirehi", effect_specialnfirehi, Priority::Low);
    agent.acmd("expression_specialnfirehi", expression_specialnfires, Priority::Low);

    agent.acmd("game_specialairnfirehi", game_specialnfire, Priority::Low);
    agent.acmd("effect_specialairnfirehi", effect_specialnairfirehi, Priority::Low);
    agent.acmd("expression_specialairnfirehi", expression_specialnfires, Priority::Low);

    agent.acmd("game_specialsend", game_specialsend, Priority::Low);

    agent.acmd("game_specialairsend", game_specialairsend, Priority::Low);

    agent.acmd("game_specialhistart", game_specialhistart, Priority::Low);

    agent.acmd("game_specialairhistart", game_specialhistart, Priority::Low);

    agent.acmd("game_specialhi", game_specialhi, Priority::Low);

    agent.acmd("game_specialairhiend", game_specialhiend, Priority::Low);

    agent.acmd("game_speciallwhold", game_dspecialhold, Priority::Low);

    agent.acmd("game_specialairlwhold", game_dspecialhold, Priority::Low);
}