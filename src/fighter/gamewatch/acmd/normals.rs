use crate::imports::acmd_imports::*;

#[acmd_script( agent = "gamewatch", script = "game_attack11", category = ACMD_GAME, low_priority )]
unsafe fn gamewatch_attack11(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 1.5);
    if macros::is_excute(agent) {
        WorkModule::set_int(agent.module_accessor, *WEAPON_GAMEWATCH_NORMAL_WEAPON_KIND_SPRAY, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_INT_NORMAL_WEAPON_KIND);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, false, -1);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, Hash40::new("attack_11"), false, -1.0);
    }
    frame(agent.lua_state_agent, 4.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 65, 12, 0, 48, 3.8, 0.0, 5.0, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 65, 12, 0, 48, 3.8, 0.0, 5.2, 12.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

#[acmd_script( agent = "gamewatch", script = "game_attacklw3", category = ACMD_GAME, low_priority )]
unsafe fn gamewatch_attacklw3(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::set_int(agent.module_accessor, *WEAPON_GAMEWATCH_NORMAL_WEAPON_KIND_MANHOLE, *FIGHTER_GAMEWATCH_INSTANCE_WORK_ID_INT_NORMAL_WEAPON_KIND);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, false, -1);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, Hash40::new("attack_lw3"), false, -1.0);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("handl"), *HIT_STATUS_XLU);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 30, 112, 0, 45, 1.2, 0.0, 2.1, 6.7, Some(0.0), Some(2.1), Some(16.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 30, 112, 0, 45, 1.2, 0.0, 9.3, 6.7, Some(0.0), Some(2.1), Some(16.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        ArticleModule::remove(agent.module_accessor, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_NORMAL_WEAPON, ArticleOperationTarget(0));
        AttackModule::clear_all(agent.module_accessor);
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

pub fn install() {
    install_acmd_scripts!(
        gamewatch_attack11,
        gamewatch_attacklw3
    );
}