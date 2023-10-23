use crate::imports::acmd_imports::*;

#[acmd_script( agent = "kirby", script = "game_specialsstart", category = ACMD_GAME, low_priority )]
unsafe fn kirby_specialsstart(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_HAMMER, false, 0);
    }
    frame(agent.lua_state_agent, 3.0);
    macros::FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 15.0);
    macros::FT_MOTION_RATE(agent, 1.0);
}

#[acmd_script( agent = "kirby", script = "game_specialairsstart", category = ACMD_GAME, low_priority )]
unsafe fn kirby_specialairsstart(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_HAMMER, false, 0);
    }
    macros::FT_MOTION_RATE(agent, 9.0 / 17.0);
}

#[acmd_script( agent = "kirby", script = "game_specialairs", category = ACMD_GAME, low_priority )]
unsafe fn kirby_specialairs(agent: &mut L2CAgentBase) {
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

#[acmd_script( agent = "kirby", script = "game_specialairss", category = ACMD_GAME, low_priority )]
unsafe fn kirby_specialairss(agent: &mut L2CAgentBase) {
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

#[acmd_script( agent = "kirby", scripts = [ "game_specialhi", "game_specialairhi" ], category = ACMD_GAME, low_priority )]
unsafe fn kirby_specialhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, false, 0);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, Hash40::new("special_hi"), false, 0.0);
    }
    frame(agent.lua_state_agent, 5.0);
    macros::FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 19.0);
    macros::FT_MOTION_RATE(agent, 1.0);
}

pub fn install() {
    install_acmd_scripts!(
        kirby_specialsstart,

        kirby_specialairsstart,

        kirby_specialairs,

        kirby_specialairss,

        kirby_specialhi
    );
}