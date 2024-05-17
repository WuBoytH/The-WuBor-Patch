use super::*;

unsafe extern "C" fn game_specialsstart(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_HAMMER, false, 0);
    }
    frame(agent.lua_state_agent, 3.0);
    macros::FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 15.0);
    macros::FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_specialairsstart(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_HAMMER, false, 0);
    }
    macros::FT_MOTION_RATE(agent, 9.0 / 17.0);
}

unsafe extern "C" fn game_specialairs(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 19.0, 50, 78, 0, 60, 5.4, 0.0, 4.3, 11.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HAMMER);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 19.0, 50, 78, 0, 60, 3.2, 0.0, 4.3, 5.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HAMMER);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 19.0, 50, 78, 0, 60, 5.4, 0.0, 4.3, 11.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HAMMER);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 19.0, 50, 78, 0, 60, 3.2, 0.0, 4.3, 5.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HAMMER);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, true);
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 54.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_HAMMER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn game_specialairss(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 19.0, 50, 78, 0, 60, 5.4, 0.0, 4.3, 11.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HAMMER);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 19.0, 50, 78, 0, 60, 3.2, 0.0, 4.3, 5.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HAMMER);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 19.0, 50, 78, 0, 60, 5.4, 0.0, 4.3, 11.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HAMMER);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 19.0, 50, 78, 0, 60, 3.2, 0.0, 4.3, 5.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HAMMER);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, true);
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 54.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_HAMMER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, false, 0);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, Hash40::new("special_hi"), false, 0.0);
    }
    frame(agent.lua_state_agent, 5.0);
    macros::FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 19.0);
    macros::FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_specialhi4(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, Hash40::new("special_hi4"), false, -1.0);
        ArticleModule::set_rate(agent.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, 31.0 / 21.0);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTERSHOT, false, -1);
    }
    macros::FT_MOTION_RATE(agent, 21.0 / 31.0);
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialsstart", game_specialsstart, Priority::Low);

    agent.acmd("game_specialairsstart", game_specialairsstart, Priority::Low);

    agent.acmd("game_specialairs", game_specialairs, Priority::Low);

    agent.acmd("game_specialairss", game_specialairss, Priority::Low);

    agent.acmd("game_specialhi", game_specialhi, Priority::Low);

    agent.acmd("game_specialairhi", game_specialhi, Priority::Low);

    agent.acmd("game_specialhi4", game_specialhi4, Priority::Low);

    agent.acmd("game_specialairhi4", game_specialhi4, Priority::Low);
}