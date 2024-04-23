use crate::imports::*;

unsafe extern "C" fn game_throwlw(agent: &mut L2CAgentBase) {
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

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_throwlw", game_throwlw, Priority::Low);
}