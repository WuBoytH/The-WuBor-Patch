use super::*;

unsafe extern "C" fn game_wolfspecialn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_WOLF_GENERATE_ARTICLE_BLASTER, false, -1);
    }
    if macros::IS_EXIST_ARTICLE(agent, *FIGHTER_WOLF_GENERATE_ARTICLE_BLASTER) {
        if macros::is_excute(agent) {
            ArticleModule::change_motion(agent.module_accessor, *FIGHTER_WOLF_GENERATE_ARTICLE_BLASTER, Hash40::new("open"), false, -1.0);
        }
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, vars::wolf::status::flag::SPECIAL_N_CHECK_ANGLE);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 2.0, 60, 37, 0, 80, 2.5, 0.0, 0.0, -3.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 0, Hash40::new("haver"), 2.0, 60, 37, 0, 80, 2.5, 0.0, 0.0, 1.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 2, 0, Hash40::new("haver"), 2.0, 60, 37, 0, 80, 2.5, 0.0, 0.0, 5.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_WOLF_GENERATE_ARTICLE_BLASTER_BULLET, false, -1);
        VarModule::on_flag(agent.module_accessor, vars::wolf::status::flag::SPECIAL_N_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 38.0);
    if macros::IS_EXIST_ARTICLE(agent, *FIGHTER_WOLF_GENERATE_ARTICLE_BLASTER) {
        if macros::is_excute(agent) {
            ArticleModule::change_motion(agent.module_accessor, *FIGHTER_WOLF_GENERATE_ARTICLE_BLASTER, Hash40::new("close"), false, -1.0);
        }
    }
    frame(agent.lua_state_agent, 45.0);
    if macros::is_excute(agent) {
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_WOLF_GENERATE_ARTICLE_BLASTER, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn game_wolfspecialairn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_WOLF_GENERATE_ARTICLE_BLASTER, false, -1);
    }
    if macros::IS_EXIST_ARTICLE(agent, *FIGHTER_WOLF_GENERATE_ARTICLE_BLASTER) {
        if macros::is_excute(agent) {
            ArticleModule::change_motion(agent.module_accessor, *FIGHTER_WOLF_GENERATE_ARTICLE_BLASTER, Hash40::new("open"), false, -1.0);
        }
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, vars::wolf::status::flag::SPECIAL_N_CHECK_ANGLE);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::SET_SPEED_EX(agent, -0.3, 1.2, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 2.0, 60, 37, 0, 80, 2.5, 0.0, 0.0, -3.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 0, Hash40::new("haver"), 2.0, 60, 37, 0, 80, 2.5, 0.0, 0.0, 1.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 2, 0, Hash40::new("haver"), 2.0, 60, 37, 0, 80, 2.5, 0.0, 0.0, 5.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_WOLF_GENERATE_ARTICLE_BLASTER_BULLET, false, -1);
        VarModule::on_flag(agent.module_accessor, vars::wolf::status::flag::SPECIAL_N_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 38.0);
    macros::FT_MOTION_RATE(agent, 1.8);
    if macros::IS_EXIST_ARTICLE(agent, *FIGHTER_WOLF_GENERATE_ARTICLE_BLASTER) {
        if macros::is_excute(agent) {
            ArticleModule::change_motion(agent.module_accessor, *FIGHTER_WOLF_GENERATE_ARTICLE_BLASTER, Hash40::new("close"), false, -1.0);
        }
    }
    frame(agent.lua_state_agent, 45.0);
    if macros::is_excute(agent) {
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_WOLF_GENERATE_ARTICLE_BLASTER, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn effect_wolfspecialn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("wolf_bayonet"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_wolf_bayonet1"), Hash40::new("tex_wolf_bayonet2"), 3, Hash40::new("haver"), 0, -0.3, 3, Hash40::new("haver"), 0, 0.77, 6.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 4);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("wolf_bayonet"), false, false);
        macros::EFFECT(agent, Hash40::new("wolf_blaster_shot"), Hash40::new("haver"), 0, 1, 4.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -8, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("wolf_bayonet"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_wolf_bayonet1"), Hash40::new("tex_wolf_bayonet2"), 4, Hash40::new("haver"), 0, -0.3, 2.5, Hash40::new("haver"), 0, 0.77, 6.3, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 3);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("wolf_bayonet"), false, false);
    }
}

unsafe extern "C" fn effect_wolfspecialairn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("wolf_bayonet"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_wolf_bayonet1"), Hash40::new("tex_wolf_bayonet2"), 3, Hash40::new("haver"), 0, -0.3, 3, Hash40::new("haver"), 0, 0.77, 6.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 4);
        macros::EFFECT(agent, Hash40::new("wolf_blaster_shot"), Hash40::new("haver"), 0, 1, 4.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("wolf_bayonet"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_wolf_bayonet1"), Hash40::new("tex_wolf_bayonet2"), 4, Hash40::new("haver"), 0, -0.3, 2.5, Hash40::new("haver"), 0, 0.77, 6.3, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 3);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("wolf_bayonet"), false, false);
    }
}

unsafe extern "C" fn sound_wolfspecialn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_wolf_special_n03"));
    }
    wait(agent.lua_state_agent, 6.0);
    wait(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_wolf_special_n02"));
    }
}

unsafe extern "C" fn expression_wolfspecialn(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("blaster") as i64, hash40("blaster_hide") as i64);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_beamm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 45.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("blaster") as i64, hash40("blaster_normal") as i64);
    }
    frame(agent.lua_state_agent, 48.0);
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
    }
}

unsafe extern "C" fn game_wolfspecialncancel(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 7.0, 60, 37, 0, 80, 2.5, 0.0, 0.0, -3.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 0, Hash40::new("haver"), 7.0, 60, 37, 0, 80, 2.5, 0.0, 0.0, 1.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 2, 0, Hash40::new("haver"), 7.0, 60, 37, 0, 80, 2.5, 0.0, 0.0, 5.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::IS_EXIST_ARTICLE(agent, *FIGHTER_WOLF_GENERATE_ARTICLE_BLASTER) {
        if macros::is_excute(agent) {
            ArticleModule::change_motion(agent.module_accessor, *FIGHTER_WOLF_GENERATE_ARTICLE_BLASTER, Hash40::new("close"), false, -1.0);
        }
    }
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_WOLF_GENERATE_ARTICLE_BLASTER, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn effect_wolfspecialncancel(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("wolf_bayonet"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_wolf_bayonet1"), Hash40::new("tex_wolf_bayonet2"), 3, Hash40::new("haver"), 0, -0.3, 3, Hash40::new("haver"), 0, 0.77, 6.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.1);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 3);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("wolf_bayonet"), false, false);
    }
}

unsafe extern "C" fn sound_wolfspecialncancel(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_wolf_special_n03"));
    }
}

unsafe extern "C" fn expression_wolfspecialncancel(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        VisibilityModule::set_int64(agent.module_accessor, hash40("blaster") as i64, hash40("blaster_hide") as i64);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
    }
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("blaster") as i64, hash40("blaster_normal") as i64);
    }
    frame(agent.lua_state_agent, 48.0);
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_wolfspecialnhi", game_wolfspecialn, Priority::Low);
    agent.acmd("effect_wolfspecialnhi", effect_wolfspecialn, Priority::Low);
    agent.acmd("sound_wolfspecialnhi", sound_wolfspecialn, Priority::Low);
    agent.acmd("expression_wolfspecialnhi", expression_wolfspecialn, Priority::Low);
    
    agent.acmd("game_wolfspecialairnhi", game_wolfspecialairn, Priority::Low);
    agent.acmd("effect_wolfspecialairnhi", effect_wolfspecialairn, Priority::Low);
    agent.acmd("sound_wolfspecialairnhi", sound_wolfspecialn, Priority::Low);
    agent.acmd("expression_wolfspecialairnhi", expression_wolfspecialn, Priority::Low);

    agent.acmd("game_wolfspecialnlw", game_wolfspecialn, Priority::Low);
    agent.acmd("effect_wolfspecialnlw", effect_wolfspecialn, Priority::Low);
    agent.acmd("sound_wolfspecialnlw", sound_wolfspecialn, Priority::Low);
    agent.acmd("expression_wolfspecialnlw", expression_wolfspecialn, Priority::Low);
    
    agent.acmd("game_wolfspecialairnlw", game_wolfspecialairn, Priority::Low);
    agent.acmd("effect_wolfspecialairnlw", effect_wolfspecialairn, Priority::Low);
    agent.acmd("sound_wolfspecialairnlw", sound_wolfspecialn, Priority::Low);
    agent.acmd("expression_wolfspecialairnlw", expression_wolfspecialn, Priority::Low);

    agent.acmd("game_wolfspecialncancel", game_wolfspecialncancel, Priority::Low);
    agent.acmd("effect_wolfspecialncancel", effect_wolfspecialncancel, Priority::Low);
    agent.acmd("sound_wolfspecialncancel", sound_wolfspecialncancel, Priority::Low);
    agent.acmd("expression_wolfspecialncancel", expression_wolfspecialncancel, Priority::Low);
}