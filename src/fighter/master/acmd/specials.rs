use crate::imports::acmd_imports::*;

unsafe extern "C" fn master_specialairhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.3);
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 2.0, 6.0);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_SWORD, false, -1);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_SWORD, Hash40::new("special_air_hi_start"), false, -1.0);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_HI_FLAG_TURN_CHECK);
    }
    frame(agent.lua_state_agent, 10.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_AIR_LASSO_FLAG_HANG_IMMIDIATE);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::SEARCH(agent, 0, 0, Hash40::new("top"), 1.7, 0.0, 30.0, 9.0, Some(0.0), Some(46.0), Some(16.0), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_BODY, false);
        GroundModule::select_cliff_hangdata(agent.module_accessor, *FIGHTER_MASTER_CLIFF_HANG_DATA_AIR_LASSO as u32);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_AIR_LASSO_FLAG_HANG_IMMIDIATE);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK);
        ArticleModule::change_status(agent.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_SWORD, *WEAPON_MASTER_SWORD_STATUS_KIND_EXTEND, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        macros::ATTACK(agent, 0, 0, Hash40::new("throw"), 0.01, 368, 100, 30, 0, 5.5, 2.0, -1.0, -1.0, Some(2.0), Some(-1.0), Some(-1.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("throw"), 3.0, 75, 100, 0, 40, 4.5, 2.0, -1.0, -1.0, Some(2.0), Some(-1.0), Some(-1.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_vec_target_pos(agent.module_accessor, 0, Hash40::new("top"), &Vector2f{x: 16.0, y: 40.0}, 4, false);
        AttackModule::set_no_finish_camera_ex(agent.module_accessor, 0, true, false);
        AttackModule::set_no_dead_all(agent.module_accessor, true, false);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("throw"), 0.01, 368, 100, 30, 0, 4.5, 0.0, 2.0, 0.75, Some(0.0), Some(-4.0), Some(-1.5), 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 2, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("throw"), 3.0, 75, 100, 0, 40, 2.0, 0.0, 2.0, 0.75, Some(0.0), Some(-4.0), Some(-1.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_vec_target_pos(agent.module_accessor, 0, Hash40::new("top"), &Vector2f{x: 16.0, y: 41.0}, 2, false);
        AttackModule::set_no_dead_all(agent.module_accessor, true, false);
        AttackModule::set_no_finish_camera_ex(agent.module_accessor, 0, true, false);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_HI_FLAG_ENABLE_CATCH);
        macros::ATTACK(agent, 0, 0, Hash40::new("throw"), 3.0, 75, 100, 0, 40, 4.5, 0.0, 2.0, 0.75, Some(0.0), Some(-4.0), Some(-1.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_BODY, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("throw"), 3.0, 75, 100, 0, 40, 2.0, 0.0, 2.0, 0.75, Some(0.0), Some(-4.0), Some(-1.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_no_dead_all(agent.module_accessor, true, false);
        AttackModule::set_no_finish_camera_ex(agent.module_accessor, 0, true, false);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("throw"), 3.0, 75, 100, 0, 40, 4.5, 0.0, 5.5, 2.0, Some(0.0), Some(-6.0), Some(-2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_BODY, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("throw"), 3.0, 75, 100, 0, 40, 2.0, 0.0, 8.0, 3.0, Some(0.0), Some(-4.0), Some(-1.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_no_dead_all(agent.module_accessor, true, false);
        AttackModule::set_no_finish_camera_ex(agent.module_accessor, 0, true, false);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        ArticleModule::change_status(agent.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_SWORD, *WEAPON_MASTER_SWORD_STATUS_KIND_BACK, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn master_speciallw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 3.0, 3.0);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_INHERIT_LANDING_1);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_TURN_CHECK);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_START_SUPER_ARMOR);
    }
    frame(agent.lua_state_agent, 42.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_TURN_CHECK);
    }
    frame(agent.lua_state_agent, 51.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_INHERIT_LANDING_1);
    }
    frame(agent.lua_state_agent, 60.0);
    if macros::is_excute(agent) {
        AttackModule::set_attack_height(agent.module_accessor, 0, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_FORBID_LANDING);
    }
    frame(agent.lua_state_agent, 64.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_END_SUPER_ARMOR);
        ArticleModule::set_flag(agent.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_AXE, true, *WEAPON_PIERCE_INSTANCE_WORK_ID_FLAG_PIERCE_GROUND);
    }
    frame(agent.lua_state_agent, 65.0);
    if macros::is_excute(agent) {
        ArticleModule::set_flag(agent.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_AXE, false, *WEAPON_PIERCE_INSTANCE_WORK_ID_FLAG_PIERCE_GROUND);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_FORBID_LANDING);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_INHERIT_LANDING_2);
    }
    frame(agent.lua_state_agent, 96.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_CONTROL_ENERGY);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_REVERT_FALL_SPEED);
    }
    frame(agent.lua_state_agent, 117.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_INHERIT_LANDING_2);
    }
    frame(agent.lua_state_agent, 118.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_AXE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn master_specialairlw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 3.0, 3.0);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_INHERIT_LANDING_1);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_TURN_CHECK);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_START_SUPER_ARMOR);
    }
    frame(agent.lua_state_agent, 42.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_TURN_CHECK);
    }
    frame(agent.lua_state_agent, 51.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_INHERIT_LANDING_1);
    }
    frame(agent.lua_state_agent, 53.0);
    if macros::is_excute(agent) {
        KineticModule::add_speed(agent.module_accessor, &Vector3f { x: 0.0, y: 1.0, z: 0.0});
    }
    frame(agent.lua_state_agent, 60.0);
    if macros::is_excute(agent) {
        AttackModule::set_attack_height(agent.module_accessor, 0, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_FORBID_LANDING);
    }
    frame(agent.lua_state_agent, 64.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_END_SUPER_ARMOR);
        ArticleModule::set_flag(agent.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_AXE, true, *WEAPON_PIERCE_INSTANCE_WORK_ID_FLAG_PIERCE_GROUND);
    }
    frame(agent.lua_state_agent, 65.0);
    if macros::is_excute(agent) {
        ArticleModule::set_flag(agent.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_AXE, false, *WEAPON_PIERCE_INSTANCE_WORK_ID_FLAG_PIERCE_GROUND);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_FORBID_LANDING);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_INHERIT_LANDING_2);
    }
    frame(agent.lua_state_agent, 96.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_CONTROL_ENERGY);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_REVERT_FALL_SPEED);
    }
    frame(agent.lua_state_agent, 117.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_INHERIT_LANDING_2);
    }
    frame(agent.lua_state_agent, 118.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_AXE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_specialairhi", master_specialairhi);

    agent.acmd("game_speciallw", master_speciallw);

    agent.acmd("game_specialairlw", master_specialairlw);
}