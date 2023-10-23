use crate::imports::acmd_imports::*;

#[acmd_script( agent = "shizue", script = "game_attackairf", category = ACMD_GAME, low_priority )]
unsafe fn shizue_attackairf(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_SLINGSHOT, true, -1);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    macros::FT_MOTION_RATE(agent, 8.0 / 6.0);
    frame(agent.lua_state_agent, 9.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "shizue_slingshot", script = "game_attackairf", category = ACMD_GAME, low_priority )]
unsafe fn shizue_slingshot_attackairf(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *WEAPON_SHIZUE_SLINGSHOT_GENERATE_ARTICLE_BULLET, false, -1);
    }
    frame(agent.lua_state_agent, 3.0);
    macros::FT_MOTION_RATE(agent, 8.0 / 6.0);
    frame(agent.lua_state_agent, 9.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        ArticleModule::shoot(agent.module_accessor, *WEAPON_SHIZUE_SLINGSHOT_GENERATE_ARTICLE_BULLET, ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false);
    }
}

#[acmd_script( agent = "shizue_bullet", script = "game_shootf", category = ACMD_GAME, low_priority )]
unsafe fn shizue_bullet_shootf(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.5, 361, 100, 0, 55, 3.1, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(agent.module_accessor);
    }
}

#[acmd_script( agent = "shizue", script = "game_attackairb", category = ACMD_GAME, low_priority )]
unsafe fn shizue_attackairb(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_SLINGSHOT, true, -1);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    macros::FT_MOTION_RATE(agent, 11.0 / 9.0);
    frame(agent.lua_state_agent, 12.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "shizue_slingshot", script = "game_attackairb", category = ACMD_GAME, low_priority )]
unsafe fn shizue_slingshot_attackairb(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *WEAPON_SHIZUE_SLINGSHOT_GENERATE_ARTICLE_BULLET, false, -1);
    }
    frame(agent.lua_state_agent, 3.0);
    macros::FT_MOTION_RATE(agent, 11.0 / 9.0);
    frame(agent.lua_state_agent, 12.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        ArticleModule::shoot(agent.module_accessor, *WEAPON_SHIZUE_SLINGSHOT_GENERATE_ARTICLE_BULLET, ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false);
    }
}

#[acmd_script( agent = "shizue_bullet", script = "game_shootb", category = ACMD_GAME, low_priority )]
unsafe fn shizue_bullet_shootb(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 361, 94, 0, 55, 3.1, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(agent.module_accessor);
    }
}

pub fn install() {
    install_acmd_scripts!(
        shizue_attackairf,

        shizue_slingshot_attackairf,

        shizue_bullet_shootf,

        shizue_attackairb,

        shizue_slingshot_attackairb,

        shizue_bullet_shootb
    );
}