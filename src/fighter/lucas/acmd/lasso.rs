use crate::imports::acmd_imports::*;

#[acmd_script( agent = "lucas", script = "game_aircatch", category = ACMD_GAME, low_priority )]
unsafe fn lucas_aircatch(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_HIMOHEBI, false, -1);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_HIMOHEBI, Hash40::new("shoot"), false, -1.0);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK);
        macros::ATTACK(fighter, 0, 0, Hash40::new("throw"), 5.0, 45, 30, 0, 60, 2.3, 0.0, 0.0, -0.5, Some(0.0), Some(0.0), Some(-6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 3.0, 45, 30, 0, 60, 1.5, 3.5, -0.5, 0.0, Some(8.5), Some(3.8), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_AIR_LASSO_FLAG_LANDING);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("throw"), 3.0, 45, 30, 0, 40, 1.5, 0.0, 0.0, -0.5, Some(0.0), Some(0.0), Some(-6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 3.0, 45, 30, 0, 40, 1.5, 3.5, -0.5, 0.0, Some(8.5), Some(3.8), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, 2, 0, Hash40::new("throw"), 3.0, 45, 30, 0, 40, 1.5, 4.0, 0.0, -0.5, Some(4.0), Some(0.0), Some(-6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 25.0);
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_AIR_LASSO_FLAG_LANDING);
    }
    frame(fighter.lua_state_agent, 50.0);
    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_HIMOHEBI, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

pub fn install() {
    install_acmd_scripts!(
        lucas_aircatch
    );
}