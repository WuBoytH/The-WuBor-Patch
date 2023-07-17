use crate::imports::acmd_imports::*;

#[acmd_script( agent = "younglink", script = "game_aircatch", category = ACMD_GAME, low_priority )]
unsafe fn younglink_aircatch(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 4.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT, false, -1);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, false, -1);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK);
        macros::ATTACK(agent, 0, 0, Hash40::new("throw"), 4.0, 60, 20, 0, 70, 2.7, 0.0, 0.0, -0.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 4.0, false);
        ArticleModule::change_status(agent.module_accessor, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT, *WEAPON_TOONLINK_HOOKSHOT_STATUS_KIND_SHOOT, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, Hash40::new("shoot"), false, -1.0);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_AIR_LASSO_FLAG_LANDING);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 41.0);
    if macros::is_excute(agent) {
        ArticleModule::change_status_exist(agent.module_accessor, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT, *WEAPON_TOONLINK_HOOKSHOT_STATUS_KIND_REWIND);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, Hash40::new("back"), false, -1.0);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_OFF_MAP_COLL_OFFSET);
    }
    frame(agent.lua_state_agent, 46.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_AIR_LASSO_FLAG_LANDING);
    }
    frame(agent.lua_state_agent, 53.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(agent.lua_state_agent, 72.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

pub fn install() {
    install_acmd_scripts!(
        younglink_aircatch
    );
}