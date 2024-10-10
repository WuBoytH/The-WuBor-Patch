use super::*;

unsafe extern "C" fn game_final(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::WHOLE_HIT(agent, *HIT_STATUS_XLU);
        macros::CHECK_VALID_FINAL_START_CAMERA(agent, 0, 7, 20, 0, 0, 0);
        macros::SLOW_OPPONENT(agent, 400.0, 50.0);
    }
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) {
        frame(agent.lua_state_agent, 10.0);
        if macros::is_excute(agent) {
            macros::FT_SET_FINAL_FEAR_FACE(agent, 40);
            macros::REQ_FINAL_START_CAMERA(agent, Hash40::new("d04final.nuanmb"), true);
            macros::FT_START_CUTIN(agent);
        }
    }
    else {
        if macros::is_excute(agent) {
            let scale = PostureModule::scale(agent.module_accessor);
            macros::CAM_ZOOM_IN_arg5(agent, 3.0, 0.0, scale * 1.8, 0.0, 0.0);
            macros::FT_START_CUTIN(agent);
        }
        frame(agent.lua_state_agent, 28.0);
        if macros::is_excute(agent) {
            macros::CAM_ZOOM_OUT(agent);
        }
    }
    // frame(agent.lua_state_agent, 30.0);
    // if macros::is_excute(agent) {
    //     macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 60, 90, 0, 50, 8.0, 0.0, 5.0, 8.0, Some(0.0), Some(9.5), Some(8.0), 1.0, 0.1, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    //     AttackModule::set_no_dead_all(agent.module_accessor, true, false);
    //     AttackModule::set_damage_shake_scale(agent.module_accessor, 0.18);
    // }
    // wait(agent.lua_state_agent, 1.0);
    // if macros::is_excute(agent) {
    //     AttackModule::clear_all(agent.module_accessor);
    // }
    frame(agent.lua_state_agent, 50.0);
    if macros::is_excute(agent) {
        SlowModule::set_whole(agent.module_accessor, 2, 0);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLAG_LOCK_ATTACK);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 80, 95, 0, 50, 8.0, 0.0, 5.0, 10.0, Some(0.0), Some(9.5), Some(10.0), 2.6, 0.1, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_FINAL01, *ATTACK_REGION_PUNCH);
        AttackModule::set_no_dead_all(agent.module_accessor, true, false);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLAG_BRANCH_HIT);
        SlowModule::clear_whole(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 52.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("handr"), 2.0, 367, 100, 120, 0, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.1, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_no_dead_all(agent.module_accessor, true, false);
    }
    wait(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        macros::ATTACK(agent, 0, 0, Hash40::new("handr"), 5.0, 80, 120, 0, 40, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.1, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_no_dead_all(agent.module_accessor, true, false);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn game_final2(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::WHOLE_HIT(agent, *HIT_STATUS_XLU);
        macros::SLOW_OPPONENT(agent, 400.0, 60.0);
        macros::CHECK_VALID_FINAL_START_CAMERA(agent, 0, 7, 20, 0, 0, 0);
    }
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) {
        frame(agent.lua_state_agent, 10.0);
        if macros::is_excute(agent) {
            macros::FT_SET_FINAL_FEAR_FACE(agent, 50);
            macros::REQ_FINAL_START_CAMERA(agent, Hash40::new("d04final.nuanmb"), false);
        }
    }
    else {
        if macros::is_excute(agent) {
            let scale = PostureModule::scale(agent.module_accessor);
            macros::CAM_ZOOM_IN_arg5(agent, 3.0, 0.0, scale * 1.8, 0.0, 0.0);
            macros::FT_START_CUTIN(agent);
        }
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        macros::CAM_ZOOM_OUT(agent);
    }
    frame(agent.lua_state_agent, 60.0);
    if macros::is_excute(agent) {
        HitModule::set_whole(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(agent.lua_state_agent, 70.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_RYU_GENERATE_ARTICLE_SHINKUHADOKEN, false, -1);
    }
    frame(agent.lua_state_agent, 75.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLAG_REMOVE_FINAL_AURA);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_final", game_final, Priority::Low);

    agent.acmd("game_finalair", game_final, Priority::Low);

    agent.acmd("game_final2", game_final2, Priority::Low);

    agent.acmd("game_finalair2", game_final2, Priority::Low);
}