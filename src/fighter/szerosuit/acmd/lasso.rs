use crate::imports::acmd_imports::*;

#[acmd_script( agent = "szerosuit", script = "game_aircatch", category = ACMD_GAME, low_priority )]
unsafe extern "C" fn szerosuit_aircatch(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_SZEROSUIT_GENERATE_ARTICLE_WHIP2, false, -1);
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_SZEROSUIT_GENERATE_ARTICLE_WHIP2, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK);
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_SZEROSUIT_GENERATE_ARTICLE_WHIP2, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::change_status(agent.module_accessor, *FIGHTER_SZEROSUIT_GENERATE_ARTICLE_WHIP2, *WEAPON_SZEROSUIT_WHIP2_STATUS_KIND_SHOOT, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        macros::ATTACK(agent, 0, 0, Hash40::new("throw"), 5.0, 45, 30, 0, 60, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_AIR_LASSO_FLAG_LANDING);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        ArticleModule::change_status_exist(agent.module_accessor, *FIGHTER_SZEROSUIT_GENERATE_ARTICLE_WHIP2, *WEAPON_SZEROSUIT_WHIP2_STATUS_KIND_REWIND);
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_SZEROSUIT_GENERATE_ARTICLE_WHIP2, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_AIR_LASSO_FLAG_LANDING);
    }
}

pub fn install(agent : &mut smashline::Agent) {
    agent.game_acmd("game_aircatch", szerosuit_aircatch);
}