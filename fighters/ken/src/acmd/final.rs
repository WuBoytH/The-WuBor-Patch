use super::*;

unsafe extern "C" fn game_final(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        // new
        macros::WHOLE_HIT(agent, *HIT_STATUS_XLU);
        macros::CHECK_VALID_FINAL_START_CAMERA(agent, 0, 0, 20, 0, 0, 0);
        macros::SLOW_OPPONENT(agent, 400.0, 55.0);
    }
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) {
        frame(agent.lua_state_agent, 5.0);
        if macros::is_excute(agent) {
            macros::FT_SET_FINAL_FEAR_FACE(agent, 40);
            macros::REQ_FINAL_START_CAMERA(agent, Hash40::new("d04final.nuanmb"), true);
            macros::FT_START_CUTIN(agent);
        }
        if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
            if macros::is_excute(agent) {
                camera!(agent, *MA_MSC_CMD_CAMERA_CAM_OFFSET, 0, 0);
                let scale = PostureModule::scale(agent.module_accessor);
                macros::CAM_ZOOM_IN_arg5(agent, 3.0, 0.0, scale * 1.8, 0.0, 0.0);
            }
        }
        else {
            if macros::is_excute(agent) {
                camera!(agent, *MA_MSC_CMD_CAMERA_CAM_OFFSET, 0, 0);
                let scale = PostureModule::scale(agent.module_accessor);
                macros::CAM_ZOOM_IN_arg5(agent, 3.0, 0.0, scale * 1.8, 0.0, 0.0);
            }
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::FT_START_CUTIN(agent);
        }
    }
    frame(agent.lua_state_agent, 10.0);
    macros::FT_MOTION_RATE(agent, 15.0);
    // if macros::is_excute(agent) {
    //     macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 365, 100, 48, 17, 11.0, 0.0, 8.0, 8.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    //     macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 365, 100, 48, 17, 11.0, 0.0, 8.0, 8.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    //     AttackModule::set_no_dead_all(agent.module_accessor, true, false);
    //     AttackModule::set_no_uniq_effect_all(agent.module_accessor, true, false);
    //     AttackModule::set_damage_shake_scale(agent.module_accessor, 0.18);
    // }
    // let scale = PostureModule::scale(agent.module_accessor);
    // if scale >= 1.4 {
    //     if macros::is_excute(agent) {
    //         AttackModule::set_vec_target_pos(agent.module_accessor, 0, Hash40::new("top"), &Vector2f{x: 15.0, y: 0.0}, 5, false);
    //         AttackModule::set_vec_target_pos(agent.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 15.0, y: 0.0}, 5, false);
    //     }
    // }
    // else if scale <= 0.5 {
    //     if macros::is_excute(agent) {
    //         AttackModule::set_vec_target_pos(agent.module_accessor, 0, Hash40::new("top"), &Vector2f{x: 20.0, y: 0.0}, 2, false);
    //         AttackModule::set_vec_target_pos(agent.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 20.0, y: 0.0}, 2, false);
    //     }
    // }
    // else {
    //     if macros::is_excute(agent) {
    //         AttackModule::set_vec_target_pos(agent.module_accessor, 0, Hash40::new("top"), &Vector2f{x: 13.0, y: 5.0}, 5, false);
    //         AttackModule::set_vec_target_pos(agent.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 15.0, y: 4.0}, 10, false);
    //     }
    // }
    // wait(agent.lua_state_agent, 1.0);
    // if macros::is_excute(agent) {
    //     AttackModule::clear_all(agent.module_accessor);
    // }
    frame(agent.lua_state_agent, 13.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 22.0);
    frame(agent.lua_state_agent, 25.0);
    game_finalsub(agent);
    let scale = PostureModule::scale(agent.module_accessor);
    if scale >= 1.4 {
        if macros::is_excute(agent) {
            AttackModule::set_vec_target_pos(agent.module_accessor, 0, Hash40::new("top"), &Vector2f{x: 12.0, y: 5.0}, 13, false);
            AttackModule::set_vec_target_pos(agent.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 15.0, y: 20.0}, 5, false);
        }
    }
    else if scale <= 0.5{
        if macros::is_excute(agent) {
            AttackModule::set_vec_target_pos(agent.module_accessor, 0, Hash40::new("top"), &Vector2f{x: 28.0, y: 5.0}, 13, false);
            AttackModule::set_vec_target_pos(agent.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 37.0, y: 10.0}, 15, false);
        }
    }
    else {
        if macros::is_excute(agent) {
            AttackModule::set_vec_target_pos(agent.module_accessor, 0, Hash40::new("top"), &Vector2f{x: 15.0, y: 1.0}, 9, false);
            AttackModule::set_vec_target_pos(agent.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 19.0, y: 5.0}, 15, false);
        }
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        HitModule::set_whole(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(agent.lua_state_agent, 40.0);
    game_finalsub(agent);
    let scale = PostureModule::scale(agent.module_accessor);
    if scale >= 1.4 {
        if macros::is_excute(agent) {
            AttackModule::set_vec_target_pos(agent.module_accessor, 0, Hash40::new("top"), &Vector2f{x: 13.0, y: 4.0}, 13, false);
            AttackModule::set_vec_target_pos(agent.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 13.0, y: 7.0}, 15, false);
        }
    }
    else if scale <= 0.5 {
        if macros::is_excute(agent) {
            AttackModule::set_vec_target_pos(agent.module_accessor, 0, Hash40::new("top"), &Vector2f{x: 37.0, y: 4.0}, 13, false);
            AttackModule::set_vec_target_pos(agent.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 37.0, y: 7.0}, 15, false);
        }
    }
    else {
        if macros::is_excute(agent) {
            AttackModule::set_vec_target_pos(agent.module_accessor, 0, Hash40::new("top"), &Vector2f{x: 20.0, y: 2.0}, 13, false);
            AttackModule::set_vec_target_pos(agent.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 20.0, y: 4.0}, 15, false);
        }
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 55.0);
    game_finalsub(agent);
    let scale = PostureModule::scale(agent.module_accessor);
    if scale >= 1.4 {
        if macros::is_excute(agent) {
            AttackModule::set_vec_target_pos(agent.module_accessor, 0, Hash40::new("top"), &Vector2f{x: 13.0, y: 8.0}, 10, false);
            AttackModule::set_vec_target_pos(agent.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 13.0, y: 10.0}, 15, false);
        }
    }
    else if scale <= 0.5 {
        if macros::is_excute(agent) {
            AttackModule::set_vec_target_pos(agent.module_accessor, 0, Hash40::new("top"), &Vector2f{x: 46.0, y: 8.0}, 10, false);
            AttackModule::set_vec_target_pos(agent.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 46.0, y: 12.0}, 15, false);
        }
    }
    else {
        if macros::is_excute(agent) {
            AttackModule::set_vec_target_pos(agent.module_accessor, 0, Hash40::new("top"), &Vector2f{x: 22.0, y: 4.0}, 10, false);
            AttackModule::set_vec_target_pos(agent.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 22.0, y: 7.0}, 15, false);
        }
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 64.0);
    game_finalsub(agent);
    let scale = PostureModule::scale(agent.module_accessor);
    if scale >= 1.4 {
        if macros::is_excute(agent) {
            AttackModule::set_vec_target_pos(agent.module_accessor, 0, Hash40::new("top"), &Vector2f{x: 14.0, y: 8.0}, 14, false);
            AttackModule::set_vec_target_pos(agent.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 16.0, y: 10.0}, 15, false);
        }
    }
    else if scale <= 0.5 {
        if macros::is_excute(agent) {
            AttackModule::set_vec_target_pos(agent.module_accessor, 0, Hash40::new("top"), &Vector2f{x: 44.0, y: 8.0}, 14, false);
            AttackModule::set_vec_target_pos(agent.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 52.0, y: 10.0}, 15, false);
        }
    }
    else {
        if macros::is_excute(agent) {
            AttackModule::set_vec_target_pos(agent.module_accessor, 0, Hash40::new("top"), &Vector2f{x: 22.0, y: 4.0}, 14, false);
            AttackModule::set_vec_target_pos(agent.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 26.0, y: 5.0}, 15, false);
        }
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 76.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLAG_LOCK_ATTACK);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 50, 95, 40, 0, 11.0, 0.0, 8.0, 8.0, Some(0.0), Some(10.0), Some(8.0), 3.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_FINAL01, *ATTACK_REGION_KICK);
        AttackModule::set_no_dead_all(agent.module_accessor, true, false);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLAG_BRANCH_HIT);
        SlowModule::clear_whole(agent.module_accessor);
    }
}

unsafe extern "C" fn game_finalair(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        // new
        macros::WHOLE_HIT(agent, *HIT_STATUS_XLU);
        macros::CHECK_VALID_FINAL_START_CAMERA(agent, 0, 0, 20, 0, 0, 0);
        macros::SLOW_OPPONENT(agent, 400.0, 55.0);
    }
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) {
        frame(agent.lua_state_agent, 5.0);
        if macros::is_excute(agent) {
            macros::FT_SET_FINAL_FEAR_FACE(agent, 40);
            macros::REQ_FINAL_START_CAMERA(agent, Hash40::new("d04final.nuanmb"), true);
            macros::FT_START_CUTIN(agent);
        }
        if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
            if macros::is_excute(agent) {
                camera!(agent, *MA_MSC_CMD_CAMERA_CAM_OFFSET, 0, 0);
                let scale = PostureModule::scale(agent.module_accessor);
                macros::CAM_ZOOM_IN_arg5(agent, 3.0, 0.0, scale * 1.8, 0.0, 0.0);
            }
        }
        else {
            if macros::is_excute(agent) {
                camera!(agent, *MA_MSC_CMD_CAMERA_CAM_OFFSET, 0, 0);
                let scale = PostureModule::scale(agent.module_accessor);
                macros::CAM_ZOOM_IN_arg5(agent, 3.0, 0.0, scale * 1.8, 0.0, 0.0);
            }
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::FT_START_CUTIN(agent);
        }
    }
    frame(agent.lua_state_agent, 10.0);
    macros::FT_MOTION_RATE(agent, 15.0);
    // if macros::is_excute(agent) {
    //     macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 365, 100, 48, 17, 11.0, 0.0, 8.0, 8.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    //     macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 365, 100, 48, 17, 11.0, 0.0, 8.0, 8.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    //     AttackModule::set_no_dead_all(agent.module_accessor, true, false);
    //     AttackModule::set_no_uniq_effect_all(agent.module_accessor, true, false);
    //     AttackModule::set_damage_shake_scale(agent.module_accessor, 0.18);
    // }
    // let scale = PostureModule::scale(agent.module_accessor);
    // if scale >= 1.4 {
    //     if macros::is_excute(agent) {
    //         AttackModule::set_vec_target_pos(agent.module_accessor, 0, Hash40::new("top"), &Vector2f{x: 15.0, y: 3.0}, 5, false);
    //         AttackModule::set_vec_target_pos(agent.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 15.0, y: 3.0}, 5, false);
    //     }
    // }
    // else if scale <= 0.5 {
    //     if macros::is_excute(agent) {
    //         AttackModule::set_vec_target_pos(agent.module_accessor, 0, Hash40::new("top"), &Vector2f{x: 15.0, y: 1.0}, 3, false);
    //         AttackModule::set_vec_target_pos(agent.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 15.0, y: 1.0}, 3, false);
    //     }
    // }
    // else {
    //     if macros::is_excute(agent) {
    //         AttackModule::set_vec_target_pos(agent.module_accessor, 0, Hash40::new("top"), &Vector2f{x: 12.0, y: 3.0}, 15, false);
    //         AttackModule::set_vec_target_pos(agent.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 13.0, y: 6.0}, 15, false);
    //     }
    // }
    // wait(agent.lua_state_agent, 1.0);
    // if macros::is_excute(agent) {
    //     AttackModule::clear_all(agent.module_accessor);
    // }
    frame(agent.lua_state_agent, 13.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 22.0);
    frame(agent.lua_state_agent, 25.0);
    game_finalsub(agent);
    let scale = PostureModule::scale(agent.module_accessor);
    if scale >= 1.4 {
        if macros::is_excute(agent) {
            AttackModule::set_vec_target_pos(agent.module_accessor, 0, Hash40::new("top"), &Vector2f{x: 15.0, y: 12.0}, 15, false);
            AttackModule::set_vec_target_pos(agent.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 15.0, y: 9.0}, 15, false);
        }
    }
    else if scale <= 0.5{
        if macros::is_excute(agent) {
            AttackModule::set_vec_target_pos(agent.module_accessor, 0, Hash40::new("top"), &Vector2f{x: 37.0, y: 14.0}, 15, false);
            AttackModule::set_vec_target_pos(agent.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 35.0, y: 9.0}, 15, false);
        }
    }
    else {
        if macros::is_excute(agent) {
            AttackModule::set_vec_target_pos(agent.module_accessor, 0, Hash40::new("top"), &Vector2f{x: 19.0, y: 13.0}, 15, false);
            AttackModule::set_vec_target_pos(agent.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 19.0, y: 8.0}, 15, false);
        }
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        HitModule::set_whole(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(agent.lua_state_agent, 40.0);
    game_finalsub(agent);
    let scale = PostureModule::scale(agent.module_accessor);
    if scale >= 1.4 {
        if macros::is_excute(agent) {
            AttackModule::set_vec_target_pos(agent.module_accessor, 0, Hash40::new("top"), &Vector2f{x: 15.0, y: 12.0}, 15, false);
            AttackModule::set_vec_target_pos(agent.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 15.0, y: 9.0}, 15, false);
        }
    }
    else if scale <= 0.5{
        if macros::is_excute(agent) {
            AttackModule::set_vec_target_pos(agent.module_accessor, 0, Hash40::new("top"), &Vector2f{x: 37.0, y: 14.0}, 15, false);
            AttackModule::set_vec_target_pos(agent.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 35.0, y: 9.0}, 15, false);
        }
    }
    else {
        if macros::is_excute(agent) {
            AttackModule::set_vec_target_pos(agent.module_accessor, 0, Hash40::new("top"), &Vector2f{x: 19.0, y: 13.0}, 15, false);
            AttackModule::set_vec_target_pos(agent.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 19.0, y: 8.0}, 15, false);
        }
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 55.0);
    game_finalsub(agent);
    let scale = PostureModule::scale(agent.module_accessor);
    if scale >= 1.4 {
        if macros::is_excute(agent) {
            AttackModule::set_vec_target_pos(agent.module_accessor, 0, Hash40::new("top"), &Vector2f{x: 15.0, y: 12.0}, 15, false);
            AttackModule::set_vec_target_pos(agent.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 15.0, y: 9.0}, 15, false);
        }
    }
    else if scale <= 0.5{
        if macros::is_excute(agent) {
            AttackModule::set_vec_target_pos(agent.module_accessor, 0, Hash40::new("top"), &Vector2f{x: 46.0, y: 14.0}, 15, false);
            AttackModule::set_vec_target_pos(agent.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 46.0, y: 9.0}, 15, false);
        }
    }
    else {
        if macros::is_excute(agent) {
            AttackModule::set_vec_target_pos(agent.module_accessor, 0, Hash40::new("top"), &Vector2f{x: 22.0, y: 13.0}, 15, false);
            AttackModule::set_vec_target_pos(agent.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 22.0, y: 8.0}, 15, false);
        }
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 64.0);
    game_finalsub(agent);
    let scale = PostureModule::scale(agent.module_accessor);
    if scale >= 1.4 {
        if macros::is_excute(agent) {
            AttackModule::set_vec_target_pos(agent.module_accessor, 0, Hash40::new("top"), &Vector2f{x: 15.0, y: 12.0}, 15, false);
            AttackModule::set_vec_target_pos(agent.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 15.0, y: 9.0}, 15, false);
        }
    }
    else if scale <= 0.5 {
        if macros::is_excute(agent) {
            AttackModule::set_vec_target_pos(agent.module_accessor, 0, Hash40::new("top"), &Vector2f{x: 52.0, y: 14.0}, 15, false);
            AttackModule::set_vec_target_pos(agent.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 52.0, y: 10.0}, 15, false);
        }
    }
    else {
        if macros::is_excute(agent) {
            AttackModule::set_vec_target_pos(agent.module_accessor, 0, Hash40::new("top"), &Vector2f{x: 26.0, y: 15.0}, 15, false);
            AttackModule::set_vec_target_pos(agent.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 26.0, y: 8.0}, 15, false);
        }
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 76.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLAG_LOCK_ATTACK);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 50, 95, 40, 0, 11.0, 0.0, 8.0, 8.0, Some(0.0), Some(10.0), Some(8.0), 3.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_FINAL01, *ATTACK_REGION_KICK);
        AttackModule::set_no_dead_all(agent.module_accessor, true, false);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLAG_BRANCH_HIT);
        SlowModule::clear_whole(agent.module_accessor);
    }
}

unsafe extern "C" fn game_finalsub(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::CAM_ZOOM_OUT(agent);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.2, 45, 150, 40, 0, 9.0, 0.0, 11.0, 5.0, Some(0.0), Some(11.0), Some(10.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_FINAL01, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 3.2, 45, 150, 40, 0, 9.0, 0.0, 11.0, 5.0, Some(0.0), Some(11.0), Some(10.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_FINAL01, *ATTACK_REGION_KICK);
        AttackModule::set_no_dead_all(agent.module_accessor, true, false);
        AttackModule::set_optional_hit_effect(agent.module_accessor, 0, Hash40::new("ken_final_shippu_hit_rush"));
        AttackModule::set_optional_hit_effect(agent.module_accessor, 1, Hash40::new("ken_final_shippu_hit_rush"));
    }
}

unsafe extern "C" fn game_final2(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        macros::WHOLE_HIT(agent, *HIT_STATUS_XLU);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::CHECK_VALID_FINAL_START_CAMERA(agent, 0, 0, 20, 0, 0, 0);
        macros::SLOW_OPPONENT(agent, 400.0, 52.0);
    }
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) {
        frame(agent.lua_state_agent, 10.0);
        if macros::is_excute(agent) {
            macros::FT_START_CUTIN(agent);
            macros::FT_SET_FINAL_FEAR_FACE(agent, 40);
            macros::REQ_FINAL_START_CAMERA(agent, Hash40::new("d04final2.nuanmb"), true);
        }
    }
    else {
        if macros::is_excute(agent) {
            let scale = PostureModule::scale(agent.module_accessor);
            macros::CAM_ZOOM_IN_arg5(agent, 3.0, 0.0, 1.8 * scale, 0.0, 0.0);
            macros::FT_START_CUTIN(agent);
        }
        frame(agent.lua_state_agent, 28.0);
        if macros::is_excute(agent) {
            macros::CAM_ZOOM_OUT(agent);
        }
    }
    // let scale = PostureModule::scale(agent.module_accessor);
    // if scale >= 1.4 {
    //     if macros::is_excute(agent) {
    //         macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 366, 100, 140, 0, 35.0, 0.0, 5.0, -1.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    //     }
    // }
    // else if scale <= 0.5 {
    //     if macros::is_excute(agent) {
    //         macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 366, 100, 22, 0, 35.0, 0.0, 5.0, -1.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    //     }
    // }
    // else {
    //     if macros::is_excute(agent) {
    //         macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 366, 100, 70, 0, 35.0, 0.0, 5.0, -1.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    //     }
    // }
    // wait(agent.lua_state_agent, 10.0);
    // if macros::is_excute(agent) {
    //     AttackModule::clear(agent.module_accessor, 0, false);
    // }
    frame(agent.lua_state_agent, 48.0);
    if macros::is_excute(agent) {
        SlowModule::set_whole(agent.module_accessor, 2, 0);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        SlowModule::clear_whole(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 52.0);
    if macros::is_excute(agent) {
        macros::SA_SET(agent, *SITUATION_KIND_AIR);
        camera!(agent, *MA_MSC_CMD_CAMERA_CAM_RECT, 40, -40, 20, 0);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLAG_ADJUST_SHINRYUKEN_POS);
    }
    frame(agent.lua_state_agent, 90.0);
    if macros::is_excute(agent) {
        HitModule::set_whole(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_final", game_final, Priority::Low);

    agent.acmd("game_finalair", game_finalair, Priority::Low);

    agent.acmd("game_final2", game_final2, Priority::Low);

    agent.acmd("game_final2air", game_final2, Priority::Low);
}