use crate::imports::acmd_imports::*;

#[acmd_script( agent = "demon", script = "game_catchattack", category = ACMD_GAME, low_priority )]
unsafe extern "C" fn demon_catchattack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.7, 361, 100, 30, 0, 5.0, 0.0, 10.0, 10.0, None, None, None, 2.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_CATCHATTACK, *ATTACK_REGION_PUNCH);
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

#[acmd_script( agent = "demon", script = "game_throwlw", category = ACMD_GAME, low_priority )]
unsafe extern "C" fn demon_throwlw(agent: &mut L2CAgentBase) {
    if !smash_rs::app::FighterCutInManager::is_vr_mode() {
        if smash_rs::app::FighterCutInManager::is_one_on_one_including_thrown(&*(agent.module_accessor as *const smash_rs::app::BattleObjectModuleAccessor)) {
            if macros::is_excute(agent) {
                FighterSpecializer_Demon::check_disabled_motion_camera_of_scale(agent.module_accessor);
                FighterSpecializer_Demon::check_disabled_motion_camera_of_stage(agent.module_accessor);
            }
            if WorkModule::is_flag(agent.module_accessor, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ENABLE_THROW_MOTION_CAMERA) {
                if macros::is_excute(agent) {
                    macros::CHECK_VALID_START_CAMERA(agent, 0, 0, 0, 0, 0, 0, false);
                }
                if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DISABLE_THROW_MOTION_CAMERA) {
                    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) {
                        if macros::is_excute(agent) {
                            macros::REQ_MOTION_CAMERA(agent, Hash40::new("e01throwlw.nuanmb"), false);
                        }
                    }
                }
                if macros::is_excute(agent) {
                    let scale = PostureModule::scale(agent.module_accessor);
                    macros::CAM_ZOOM_IN_arg5(agent, 7.0, 0.0, scale * 1.5, 0.0, 0.0);
                }
            }
        }
    }
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 1.0, 72, 100, 0, 53, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 50, 65, 0, 50, 6.0, 0.0, 12.0, 7.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_HEAD);
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
        macros::CHECK_FINISH_CAMERA(agent, 9, 4);
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        macros::CAM_ZOOM_OUT(agent);
    }
}

#[acmd_script( agent = "demon", script = "game_catchcommand", category = ACMD_GAME, low_priority )]
unsafe extern "C" fn demon_catchcommand(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, false);
    }
    macros::FT_MOTION_RATE(agent, 1.6);
    frame(agent.lua_state_agent, 3.0);
    macros::FT_START_ADJUST_MOTION_FRAME_arg1(agent, 1.0);
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 4.0, 0.0, 6.6, 5.0, Some(0.0), Some(6.6), Some(11.2), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(agent, 1, Hash40::new("top"), 2.0, 0.0, 6.6, 3.0, Some(0.0), Some(6.6), Some(13.2), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
    }
    macros::game_CaptureCutCommon(agent);
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DEMON_STATUS_CATCH_COMMAND_FLAG_CHANGE_THROW);
    }
}

pub fn install(agent : &mut smashline::Agent) {
    agent.game_acmd("game_catchattack", demon_catchattack);

    agent.game_acmd("game_throwlw", demon_throwlw);

    agent.game_acmd("game_catchcommand", demon_catchcommand);
}