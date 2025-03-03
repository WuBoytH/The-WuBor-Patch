use super::*;
use super::super::helper::*;

unsafe extern "C" fn game_specialn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_DECIDE_STRENGTH);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) != *FIGHTER_DOLLY_STRENGTH_W {
        if macros::is_excute(agent) {
            MotionModule::set_rate(agent.module_accessor, 10.0 / 18.0);
        }
    }
    else {
        if macros::is_excute(agent) {
            MotionModule::set_rate(agent.module_accessor, 10.0 / 14.0);
        }
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_N_WORK_FLAG_GENERATE);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        if WorkModule::get_int(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            MotionModule::set_rate(agent.module_accessor, 1.0);
        }
        else {
            MotionModule::set_rate(agent.module_accessor, 32.0 / 28.0);
        }
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
}

unsafe extern "C" fn sound_specialn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_dolly_special_n01"));
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::IS_GENERATABLE_ARTICLE(agent, *FIGHTER_DOLLY_GENERATE_ARTICLE_WAVE) {
        if macros::is_excute(agent) {
            if WorkModule::get_int(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) != *FIGHTER_DOLLY_STRENGTH_W {
                macros::PLAY_SE(agent, Hash40::new("vc_dolly_special_n03"));
            }
            else {
                macros::PLAY_SEQUENCE(agent, Hash40::new("seq_dolly_rnd_special_n"));
            }
        }
        frame(agent.lua_state_agent, 13.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_dolly_special_n02"));
        }
    }
    else {
        frame(agent.lua_state_agent, 13.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_dolly_special_n02"));
        }
        frame(agent.lua_state_agent, 17.0);
        if macros::IS_GENERATABLE_ARTICLE(agent, *FIGHTER_DOLLY_GENERATE_ARTICLE_WAVE) {
            if macros::is_excute(agent) {
                if WorkModule::get_int(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) != *FIGHTER_DOLLY_STRENGTH_W {
                    macros::PLAY_SE(agent, Hash40::new("vc_dolly_special_n03"));
                }
                else {
                    macros::PLAY_SEQUENCE(agent, Hash40::new("seq_dolly_rnd_special_n02"));
                }
            }
        }
    }
}

unsafe extern "C" fn game_specialairn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_DECIDE_STRENGTH);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) != *FIGHTER_DOLLY_STRENGTH_W {
        if macros::is_excute(agent) {
            MotionModule::set_rate(agent.module_accessor, 10.0 / 18.0);
        }
    }
    else {
        if macros::is_excute(agent) {
            MotionModule::set_rate(agent.module_accessor, 10.0 / 14.0);
        }
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_N_WORK_FLAG_GENERATE);
        if WorkModule::get_int(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            MotionModule::set_rate(agent.module_accessor, 1.0);
        }
        else {
            MotionModule::set_rate(agent.module_accessor, 32.0 / 28.0);
        }
    }
}

unsafe extern "C" fn game_specialsfstart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        MotionModule::set_rate(agent.module_accessor, 5.0 / 7.0);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        MotionModule::set_rate(agent.module_accessor, 1.0);
        VarModule::off_flag(agent.module_accessor, vars::dolly::status::flag::SPECIAL_F_CHECK_FEINT);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_DECIDE_STRENGTH);
    }
    if !VarModule::is_flag(agent.module_accessor, vars::dolly::status::flag::SPECIAL_F_FEINT) {
        if WorkModule::get_int(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
            if macros::is_excute(agent) {
                MotionModule::set_rate(agent.module_accessor, 1.3);
            }
        }
        else {
            if macros::is_excute(agent) {
                MotionModule::set_rate(agent.module_accessor, 0.75);
            }
        }
    }
}

unsafe extern "C" fn game_specialsfattack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 0.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_S_WORK_FLAG_AIR_ATTACK) {
        if WorkModule::get_int64(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W as u64 {
            if macros::is_excute(agent) {
                MotionModule::set_rate(agent.module_accessor, 1.2);
            }
        }
    }
    else {
        if macros::is_excute(agent) {
            MotionModule::set_rate(agent.module_accessor, 1.5);
        }
    }
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
        if WorkModule::get_int64(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) != *FIGHTER_DOLLY_STRENGTH_W as u64 {
            if macros::is_excute(agent) {
                let output = dolly_calc_special_cancel(agent, 12.0, 65);
                macros::ATTACK(agent, 0, 0, Hash40::new("top"), output.dmg, 40, 74, 0, output.bkb, 4.3, 0.0, 11.0, 13.0, Some(0.0), Some(11.0), Some(8.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 20, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            }
        }
        else {
            if macros::is_excute(agent) {
                let output = dolly_calc_special_cancel(agent, 10.0, 67);
                macros::ATTACK(agent, 0, 0, Hash40::new("top"), output.dmg, 40, 74, 0, output.bkb, 4.3, 0.0, 11.0, 13.0, Some(0.0), Some(11.0), Some(8.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 20, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            }
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
            macros::HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_XLU);
        }
        if WorkModule::get_int64(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) != *FIGHTER_DOLLY_STRENGTH_W as u64 {
            if macros::is_excute(agent) {
                let output = dolly_calc_special_cancel(agent, 15.0, 65);
                macros::ATTACK(agent, 0, 0, Hash40::new("top"), output.dmg, 40, 74, 0, output.bkb, 4.8, 0.0, 11.0, 14.0, Some(0.0), Some(11.0), Some(8.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 20, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_CRITICAL, *ATTACK_REGION_PUNCH);
            }
        }
        else {
            if macros::is_excute(agent) {
                let output = dolly_calc_special_cancel(agent, 13.0, 67);
                macros::ATTACK(agent, 0, 0, Hash40::new("top"), output.dmg, 40, 74, 0, output.bkb, 4.8, 0.0, 11.0, 14.0, Some(0.0), Some(11.0), Some(8.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 20, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            }
        }
    }
    frame(agent.lua_state_agent, 7.0);
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
        if macros::is_excute(agent) {
            let output = dolly_calc_special_cancel(agent, 9.0, 50);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), output.dmg, 40, 78, 0, output.bkb, 4.3, 0.0, 11.0, 13.0, Some(0.0), Some(11.0), Some(8.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 20, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    frame(agent.lua_state_agent, 9.0);
    // if macros::is_excute(agent) {
    //     WorkModule::off_flag(agent.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    // }
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
        if macros::is_excute(agent) {
            macros::HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_NORMAL);
            let output = dolly_calc_special_cancel(agent, 12.0, 52);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), output.dmg, 40, 78, 0, output.bkb, 4.8, 0.0, 11.0, 14.0, Some(0.0), Some(11.0), Some(8.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 20, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
}

unsafe extern "C" fn game_specialffeint(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        MotionModule::set_rate(agent.module_accessor, 1.0);
        VarModule::off_flag(agent.module_accessor, vars::dolly::status::flag::SPECIAL_F_CHECK_FEINT);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_DECIDE_STRENGTH);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        MotionModule::set_rate(agent.module_accessor, 0.75);
    }
}

unsafe extern "C" fn effect_specialffeint(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 10, 13, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
            macros::LAST_EFFECT_SET_RATE(agent, 1.3);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("dolly_drive_flash"), Hash40::new("top"), 10, 12, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
            macros::LAST_EFFECT_SET_RATE(agent, 1.3);
        }
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 1.2);
    }
}

unsafe extern "C" fn sound_specialffeint(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_dolly_command_success"));
        }
    }
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_dolly_special_sf01"));
    }
}

unsafe extern "C" fn expression_specialffeint(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 4, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_specialsbattackw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        let output = dolly_calc_special_cancel(agent, 2.0, 75);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), output.dmg, 368, 100, 0, 0, 5.0, 0.0, 14.0, 5.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), output.dmg, 368, 100, 0, 0, 5.0, 0.0, 5.0, 1.0, Some(0.0), Some(8.0), Some(1.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_vec_target_pos(agent.module_accessor, 2, Hash40::new("top"), &Vector2f{x: 10.0, y: 3.0}, 2, false);
        AttackModule::set_vec_target_pos(agent.module_accessor, 3, Hash40::new("top"), &Vector2f{x: 10.0, y: 3.0}, 2, false);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        let output = dolly_calc_special_cancel(agent, 2.0, 75);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), output.dmg, 368, 20, 0, output.bkb, 5.0, 0.0, 9.0, 8.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_vec_target_pos(agent.module_accessor, 2, Hash40::new("top"), &Vector2f{x: 10.0, y: 3.0}, 2, false);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        let output1 = dolly_calc_special_cancel(agent, 8.0, 95);
        let output2 = dolly_calc_special_cancel(agent, 6.0, 75);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), output1.dmg, 270, 10, 0, 95, 5.0, 0.0, 6.0, 8.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), output1.dmg, 270, 10, 0, 95, 5.0, 0.0, 5.0, 1.0, Some(0.0), Some(8.0), Some(1.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), output2.dmg, 78, 20, 0, output2.bkb, 5.0, 0.0, 6.0, 8.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), output2.dmg, 78, 20, 0, output2.bkb, 5.0, 0.0, 5.0, 1.0, Some(0.0), Some(8.0), Some(1.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn game_specialsbattack(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        let output = dolly_calc_special_cancel(agent, 2.0, 95);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), output.dmg, 368, 100, 0, 0, 5.0, 0.0, 16.0, 3.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), output.dmg, 368, 100, 0, 0, 5.0, 0.0, 16.0, 3.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_vec_target_pos(agent.module_accessor, 2, Hash40::new("top"), &Vector2f{x: 10.0, y: 3.0}, 2, false);
        AttackModule::set_vec_target_pos(agent.module_accessor, 3, Hash40::new("top"), &Vector2f{x: 10.0, y: 3.0}, 2, false);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        let output = dolly_calc_special_cancel(agent, 2.0, 95);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), output.dmg, 368, 100, 0, 0, 5.0, 0.0, 15.0, 5.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), output.dmg, 368, 100, 0, 0, 5.0, 0.0, 15.0, 5.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_vec_target_pos(agent.module_accessor, 2, Hash40::new("top"), &Vector2f{x: 10.0, y: 3.0}, 2, false);
        AttackModule::set_vec_target_pos(agent.module_accessor, 3, Hash40::new("top"), &Vector2f{x: 10.0, y: 3.0}, 2, false);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        let output = dolly_calc_special_cancel(agent, 2.0, 75);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), output.dmg, 368, 100, 0, 0, 5.0, 0.0, 14.0, 5.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), output.dmg, 368, 100, 0, 0, 5.0, 0.0, 5.0, 1.0, Some(0.0), Some(8.0), Some(1.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_vec_target_pos(agent.module_accessor, 2, Hash40::new("top"), &Vector2f{x: 10.0, y: 3.0}, 2, false);
        AttackModule::set_vec_target_pos(agent.module_accessor, 3, Hash40::new("top"), &Vector2f{x: 10.0, y: 3.0}, 2, false);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        let output = dolly_calc_special_cancel(agent, 2.0, 75);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), output.dmg, 368, 100, 0, 0, 5.0, 0.0, 11.0, 8.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_vec_target_pos(agent.module_accessor, 2, Hash40::new("top"), &Vector2f{x: 10.0, y: 3.0}, 2, false);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        let output1 = dolly_calc_special_cancel(agent, 10.0, 95);
        let output2 = dolly_calc_special_cancel(agent, 6.0, 75);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), output1.dmg, 270, 10, 0, 95, 5.0, 0.0, 9.0, 8.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), output1.dmg, 270, 10, 0, 95, 5.0, 0.0, 5.0, 1.0, Some(0.0), Some(8.0), Some(1.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), output2.dmg, 78, 20, 0, output2.bkb, 5.0, 0.0, 9.0, 8.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), output2.dmg, 78, 20, 0, output2.bkb, 5.0, 0.0, 5.0, 1.0, Some(0.0), Some(8.0), Some(1.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        let output1 = dolly_calc_special_cancel(agent, 10.0, 95);
        let output2 = dolly_calc_special_cancel(agent, 6.0, 75);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), output1.dmg, 270, 10, 0, 95, 5.0, 0.0, 7.0, 8.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), output2.dmg, 78, 20, 0, output2.bkb, 5.0, 0.0, 7.0, 8.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_specialsbattack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    let alpha: f32 = if WorkModule::is_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("dolly_kick_arc_s_command"), Hash40::new("dolly_kick_arc_s_command"), Hash40::new("rot"), -1, -6, 0, -90, -90, 0, 1, true, *EF_FLIP_YZ);
            EffectModule::set_disable_render_offset_last(agent.module_accessor);
        }
        1.3
    }
    else {
        0.9
    };
    let color = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let eff = match color {
        0 => hash40("dolly_kick_arc_s01"),
        1 => hash40("dolly_kick_arc_s02"),
        2 => hash40("dolly_kick_arc_s03"),
        3 => hash40("dolly_kick_arc_s04"),
        4 => hash40("dolly_kick_arc_s05"),
        5 => hash40("dolly_kick_arc_s06"),
        6 => hash40("dolly_kick_arc_s07"),
        _ => hash40("dolly_kick_arc_s08")
    };
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new_raw(eff), Hash40::new_raw(eff), Hash40::new("rot"), -1, -6, 0, -90, -90, 0, 1, true, *EF_FLIP_YZ, alpha)
    }
}

unsafe extern "C" fn sound_specialsbattack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
        if macros::is_excute(agent) {
            macros::PLAY_STATUS(agent, Hash40::new("se_dolly_special_sb03_command"));
            macros::PLAY_SEQUENCE(agent, Hash40::new("seq_dolly_rnd_special_b02"));
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::PLAY_STATUS(agent, Hash40::new("se_dolly_special_sb03"));
            macros::PLAY_SEQUENCE(agent, Hash40::new("seq_dolly_rnd_special_b02"));
        }
    }
}

unsafe extern "C" fn expression_specialsbattack(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohitm"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
}

unsafe extern "C" fn game_specialsbend(agent: &mut L2CAgentBase) {
    MiscModule::calc_motion_rate_from_cancel_frame(agent, 0.0, -5.0);
}

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_HI_WORK_FLAG_REVERSE_LR);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_DECIDE_STRENGTH);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        let output = dolly_calc_special_cancel(agent, 4.0, 30);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), output.dmg, 80, 100, 30, 30, 4.0, 0.0, 10.0, 6.5, Some(0.0), Some(10.0), Some(2.0), 1.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        AttackModule::set_no_finish_camera(agent.module_accessor, 0, true, false);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("kneel"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("legr"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("legl"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("hip"), *HIT_STATUS_XLU);
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_HI_WORK_FLAG_JUMP);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        GroundModule::select_cliff_hangdata(agent.module_accessor, 1);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.3, 98, 110, 110, 30, 3.5, 0.0, 10.0, 6.5, Some(0.0), Some(10.0), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.3, 75, 110, 110, 30, 3.5, 0.0, 10.0, -2.0, Some(0.0), Some(10.0), Some(-3.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.5, 0.0, 4.5, 0.0, Some(0.0), Some(4.5), Some(6.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.3, 70, 30, 30, 30, 3.5, 0.0, 17.0, 1.0, Some(0.0), Some(19.5), Some(1.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 35, 30, 10, 4.5, 0.0, 10.0, 3.0, Some(0.0), Some(10.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 45, 40, 10, 4.5, 0.0, 10.0, 3.0, Some(0.0), Some(10.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.0, 0.0, 4.5, -4.0, Some(0.0), Some(4.5), Some(-6.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.5, 0.0, 3.5, 6.0, Some(0.0), Some(3.5), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.3, 70, 30, 30, 30, 3.5, 0.0, 16.0, 1.0, Some(0.0), Some(18.5), Some(1.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 35, 30, 10, 4.5, 0.0, 9.0, 3.0, Some(0.0), Some(9.0), Some(1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 40, 40, 10, 4.5, 0.0, 9.0, 3.0, Some(0.0), Some(9.0), Some(1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.0, 0.0, 3.5, -4.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.5, 0.0, 3.5, 0.0, Some(0.0), Some(3.5), Some(6.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.3, 70, 30, 30, 30, 3.5, 0.0, 16.0, 1.0, Some(0.0), Some(18.5), Some(1.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 35, 30, 10, 4.5, 0.0, 9.0, 3.0, Some(0.0), Some(9.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 40, 40, 10, 4.5, 0.0, 9.0, 3.0, Some(0.0), Some(9.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.0, 0.0, 3.5, -4.0, Some(0.0), Some(3.5), Some(-6.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("kneel"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("legr"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("legl"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("hip"), *HIT_STATUS_NORMAL);
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 25.0);
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
        if macros::is_excute(agent) {
            let output = dolly_calc_special_cancel(agent, 7.0, 90);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), output.dmg, 83, 65, 0, output.bkb, 7.0, 0.0, 7.0, 2.0, Some(0.0), Some(14.0), Some(2.0), 2.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    else {
        if macros::is_excute(agent) {
            let output = dolly_calc_special_cancel(agent, 9.0, 90);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), output.dmg, 83, 55, 0, output.bkb, 7.0, 0.0, 7.0, 2.0, Some(0.0), Some(14.0), Some(2.0), 2.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        MotionModule::set_rate(agent.module_accessor, 1.5);
    }
}

unsafe extern "C" fn expression_specialhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 2);
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 1);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 4, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 1);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 9, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 18.0);
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
}

unsafe extern "C" fn game_specialhicommand(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::WHOLE_HIT(agent, *HIT_STATUS_XLU);
    }
    if VarModule::is_flag(agent.module_accessor, vars::dolly::instance::flag::RISING_FORCE) {
        if macros::is_excute(agent) {
            VarModule::on_flag(agent.module_accessor, vars::dolly::status::flag::DISABLE_METER_GAIN);
        }
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_HI_WORK_FLAG_REVERSE_LR);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_DECIDE_STRENGTH);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        let output = dolly_calc_special_cancel(agent, 4.5, 30);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), output.dmg, 80, 100, 30, 30, 4.5, 0.0, 10.0, 6.5, Some(0.0), Some(10.0), Some(2.0), 1.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        AttackModule::set_no_finish_camera(agent.module_accessor, 0, true, false);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("kneel"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("legr"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("legl"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("hip"), *HIT_STATUS_XLU);
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_HI_WORK_FLAG_JUMP);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        GroundModule::select_cliff_hangdata(agent.module_accessor, 1);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.3, 98, 110, 110, 30, 3.5, 0.0, 10.0, 6.5, Some(0.0), Some(10.0), Some(2.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.3, 75, 110, 110, 30, 3.5, 0.0, 10.0, -2.0, Some(0.0), Some(10.0), Some(-3.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.5, 0.0, 7.5, 0.0, Some(0.0), Some(7.5), Some(6.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.3, 70, 50, 5, 20, 3.5, 0.0, 20.0, 1.0, Some(0.0), Some(22.5), Some(1.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 40, 40, 20, 4.5, 0.0, 13.0, 3.0, Some(0.0), Some(13.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 45, 40, 20, 4.5, 0.0, 13.0, 3.0, Some(0.0), Some(13.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.0, 0.0, 7.5, -4.0, Some(0.0), Some(7.5), Some(-7.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.5, 0.0, 6.5, 7.5, Some(0.0), Some(6.5), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.3, 70, 50, 5, 20, 3.5, 0.0, 19.0, 1.0, Some(0.0), Some(21.5), Some(1.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 35, 30, 20, 4.5, 0.0, 12.0, 3.0, Some(0.0), Some(12.0), Some(1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 45, 40, 20, 4.5, 0.0, 12.0, 3.0, Some(0.0), Some(12.0), Some(1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.0, 0.0, 6.5, -4.0, Some(0.0), Some(6.5), Some(-4.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.5, 0.0, 5.5, 0.0, Some(0.0), Some(5.5), Some(6.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.3, 70, 50, 5, 20, 3.5, 0.0, 18.0, 1.0, Some(0.0), Some(20.5), Some(1.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 35, 30, 20, 4.5, 0.0, 11.0, 3.0, Some(0.0), Some(11.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 45, 40, 20, 4.5, 0.0, 11.0, 3.0, Some(0.0), Some(11.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.0, 0.0, 5.5, -4.0, Some(0.0), Some(5.5), Some(-7.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.5, 0.0, 5.5, 7.5, Some(0.0), Some(5.5), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.3, 70, 50, 5, 20, 3.5, 0.0, 18.0, 1.0, Some(0.0), Some(20.5), Some(1.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 35, 30, 20, 4.5, 0.0, 11.0, 3.0, Some(0.0), Some(11.0), Some(1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 45, 40, 20, 4.5, 0.0, 11.0, 3.0, Some(0.0), Some(11.0), Some(1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.0, 0.0, 5.5, -4.0, Some(0.0), Some(5.5), Some(-4.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.5, 0.0, 5.5, 0.0, Some(0.0), Some(5.5), Some(6.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.3, 70, 50, 5, 20, 3.5, 0.0, 18.0, 1.0, Some(0.0), Some(20.5), Some(1.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 35, 30, 20, 4.5, 0.0, 11.0, 3.0, Some(0.0), Some(11.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 45, 40, 20, 4.5, 0.0, 11.0, 3.0, Some(0.0), Some(11.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.0, 0.0, 5.5, -4.0, Some(0.0), Some(5.5), Some(-7.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.5, 0.0, 4.5, 7.5, Some(0.0), Some(4.5), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.3, 70, 50, 5, 20, 3.5, 0.0, 17.0, 1.0, Some(0.0), Some(19.5), Some(1.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 35, 30, 20, 4.5, 0.0, 10.0, 3.0, Some(0.0), Some(10.0), Some(1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 45, 40, 20, 4.5, 0.0, 10.0, 3.0, Some(0.0), Some(10.0), Some(1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.0, 0.0, 4.5, -4.0, Some(0.0), Some(4.5), Some(-4.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("kneel"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("legr"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("legl"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("hip"), *HIT_STATUS_NORMAL);
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.3, 90, 30, 50, 30, 3.5, 0.0, 3.5, 5.0, Some(0.0), Some(3.5), Some(-7.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.3, 70, 50, 5, 20, 3.5, 0.0, 16.0, 1.0, Some(0.0), Some(18.5), Some(1.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 40, 45, 20, 4.5, 0.0, 10.0, 3.0, Some(0.0), Some(10.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 50, 45, 20, 4.5, 0.0, 10.0, 3.0, Some(0.0), Some(10.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 34.0);
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
        if macros::is_excute(agent) {
            let output = dolly_calc_special_cancel(agent, 8.0, 90);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), output.dmg, 83, 73, 0, output.bkb, 7.0, 0.0, 8.0, 2.0, Some(0.0), Some(15.0), Some(2.0), 2.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    else {
        if macros::is_excute(agent) {
            let output = dolly_calc_special_cancel(agent, 10.0, 90);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), output.dmg, 83, 62, 0, output.bkb, 7.0, 0.0, 8.0, 2.0, Some(0.0), Some(15.0), Some(2.0), 2.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        MotionModule::set_rate(agent.module_accessor, 1.5);
    }
}

unsafe extern "C" fn expression_specialhicommand(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 2);
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 1);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 4, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 1);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 9, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 18.0);
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 8, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
}

unsafe extern "C" fn game_speciallwstart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.5, 72, 30, 30, 73, 5.0, 0.0, 10.0, 7.5, Some(0.0), Some(6.0), Some(7.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_BODY);
    }
}

unsafe extern "C" fn effect_speciallwstart(agent: &mut L2CAgentBase) {
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 10, 8, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
            macros::LAST_EFFECT_SET_RATE(agent, 1.3);
        }
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("dolly_down_start"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_attack_impact"), Hash40::new("kneer"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn effect_specialairlwstart(agent: &mut L2CAgentBase) {
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 10, 8, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
            macros::LAST_EFFECT_SET_RATE(agent, 1.3);
        }
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_attack_impact"), Hash40::new("kneer"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn expression_speciallwstart(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_rush"),
            8,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
        if macros::is_excute(agent) {
            macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        }
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

unsafe extern "C" fn game_specialairlwrisew(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        KineticModule::clear_speed_all(agent.module_accessor);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
    }
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
        if macros::is_excute(agent) {
            macros::WHOLE_HIT(agent, *HIT_STATUS_XLU);
        }
    }
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 63, 100, 10, 50, 5.0, 0.0, 10.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 63, 80, 10, 50, 5.0, 0.0, 10.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 63, 60, 10, 50, 5.0, 0.0, 10.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 63, 40, 10, 50, 5.0, 0.0, 10.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        macros::WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        KineticModule::add_speed(agent.module_accessor, &Vector3f{x: 5.0, y: -1.5, z: 0.0});
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
    }
}

unsafe extern "C" fn game_specialairlwrise(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        KineticModule::clear_speed_all(agent.module_accessor);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
    }
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
        if macros::is_excute(agent) {
            macros::WHOLE_HIT(agent, *HIT_STATUS_XLU);
        }
    }
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 63, 100, 30, 50, 5.0, 0.0, 10.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 63, 80, 30, 50, 5.0, 0.0, 10.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 63, 60, 30, 50, 5.0, 0.0, 10.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 63, 40, 30, 50, 5.0, 0.0, 10.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        macros::WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        KineticModule::add_speed(agent.module_accessor, &Vector3f{x: 0.3, y: -1.5, z: 0.0});
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
    }
}

unsafe extern "C" fn game_specialairlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 0.0);
    if WorkModule::get_int64(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W as u64 {
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
            KineticModule::add_speed(agent.module_accessor, &Vector3f { x: 0.3, y: -1.0, z: 0.0 });
            WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        }
    }
    else {
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
            KineticModule::add_speed(agent.module_accessor, &Vector3f { x: 1.3, y: -1.5, z: 0.0 });
            WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        }
    }
    frame(agent.lua_state_agent, 1.0);
    if WorkModule::get_int64(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W as u64 {
        if macros::is_excute(agent) {
            KineticModule::add_speed(agent.module_accessor, &Vector3f { x: 0.0, y: -0.3, z: 0.0 });
        }
    }
    else {
        if macros::is_excute(agent) {
            KineticModule::add_speed(agent.module_accessor, &Vector3f { x: 0.0, y: -1.0, z: 0.0 });
        }
    }
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
        if WorkModule::get_int64(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W as u64 {
            if macros::is_excute(agent) {
                let output = dolly_calc_special_cancel(agent, 11.0, 80);
                macros::ATTACK(agent, 0, 0, Hash40::new("top"), output.dmg, 50, 70, 0, output.bkb, 7.0, 0.0, 8.0, 5.5, Some(0.0), Some(4.0), Some(3.5), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            }
        }
        else {
            if macros::is_excute(agent) {
                let output = dolly_calc_special_cancel(agent, 12.0, 80);
                macros::ATTACK(agent, 0, 0, Hash40::new("top"), output.dmg, 50, 65, 0, output.bkb, 7.0, 0.0, 8.0, 5.5, Some(0.0), Some(4.0), Some(3.5), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            }
        }
    }
    else {
        if WorkModule::get_int64(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W as u64 {
            if macros::is_excute(agent) {
                let output = dolly_calc_special_cancel(agent, 11.0, 80);
                macros::ATTACK(agent, 0, 0, Hash40::new("top"), output.dmg, 50, 75, 0, output.bkb, 7.0, 0.0, 8.0, 5.5, Some(0.0), Some(4.0), Some(3.5), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_CRITICAL, *ATTACK_REGION_PUNCH);
            }
        }
        else {
            if macros::is_excute(agent) {
                let output = dolly_calc_special_cancel(agent, 12.0, 80);
                macros::ATTACK(agent, 0, 0, Hash40::new("top"), output.dmg, 50, 70, 0, output.bkb, 7.0, 0.0, 8.0, 5.5, Some(0.0), Some(4.0), Some(3.5), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            }
        }
    }
    frame(agent.lua_state_agent, 2.0);
    if WorkModule::get_int64(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W as u64 {
        if macros::is_excute(agent) {
            KineticModule::add_speed(agent.module_accessor, &vars::ZERO_VECTOR);
        }
    }
    else {
        if macros::is_excute(agent) {
            KineticModule::add_speed(agent.module_accessor, &Vector3f { x: 0.0, y: -0.5, z: 0.0 });
        }
    }
    frame(agent.lua_state_agent, 4.0);
    if WorkModule::get_int64(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W as u64 {
        if macros::is_excute(agent) {
            KineticModule::add_speed(agent.module_accessor, &Vector3f { x: 0.0, y: 0.05, z: 0.0 });
        }
    }
    else {
        if macros::is_excute(agent) {
            KineticModule::add_speed(agent.module_accessor, &Vector3f { x: 0.0, y: 0.2, z: 0.0 });
        }
    }
    frame(agent.lua_state_agent, 5.0);
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
        if WorkModule::get_int64(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W as u64 {
            if macros::is_excute(agent) {
                let output = dolly_calc_special_cancel(agent, 14.0, 60);
                macros::ATTACK(agent, 0, 0, Hash40::new("top"), output.dmg, 50, 80, 0, output.bkb, 7.0, 0.0, 8.0, 5.5, Some(0.0), Some(4.0), Some(3.5), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            }
        }
        else {
            if macros::is_excute(agent) {
                let output = dolly_calc_special_cancel(agent, 14.0, 60);
                macros::ATTACK(agent, 0, 0, Hash40::new("top"), output.dmg, 50, 80, 0, output.bkb, 7.0, 0.0, 8.0, 5.5, Some(0.0), Some(4.0), Some(3.5), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            }
        }
    }
    else {
        if WorkModule::get_int64(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) != *FIGHTER_DOLLY_STRENGTH_W as u64 {
            if macros::is_excute(agent) {
                let output = dolly_calc_special_cancel(agent, 14.0, 30);
                macros::ATTACK(agent, 0, 0, Hash40::new("top"), output.dmg, 310, 95, 0, output.bkb, 7.0, 0.0, 8.0, 5.5, Some(0.0), Some(4.0), Some(3.5), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_CRITICAL, *ATTACK_REGION_PUNCH);
            }
        }
        else {
            if macros::is_excute(agent) {
                let output = dolly_calc_special_cancel(agent, 14.0, 60);
                macros::ATTACK(agent, 0, 0, Hash40::new("top"), output.dmg, 50, 80, 0, output.bkb, 7.0, 0.0, 8.0, 5.5, Some(0.0), Some(4.0), Some(3.5), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            }
        }
    }
    if WorkModule::get_int64(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W as u64 {
        if macros::is_excute(agent) {
            MotionModule::set_rate(agent.module_accessor, 1.2);
            KineticModule::add_speed(agent.module_accessor, &Vector3f { x: 0.0, y: 0.05, z: 0.0 });
        }
    }
    else {
        if macros::is_excute(agent) {
            MotionModule::set_rate(agent.module_accessor, 1.0);
            KineticModule::add_speed(agent.module_accessor, &Vector3f { x: 0.0, y: 0.2, z: 0.0 });
        }
    }
    frame(agent.lua_state_agent, 6.0);
    if WorkModule::get_int64(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W as u64 {
        if macros::is_excute(agent) {
            KineticModule::add_speed(agent.module_accessor, &Vector3f { x: 0.0, y: 0.05, z: 0.0 });
        }
    }
    else {
        if macros::is_excute(agent) {
            KineticModule::add_speed(agent.module_accessor, &Vector3f { x: 0.0, y: 0.2, z: 0.0 });
        }
    }
    frame(agent.lua_state_agent, 7.0);
    if WorkModule::get_int64(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W as u64 {
        if macros::is_excute(agent) {
            KineticModule::add_speed(agent.module_accessor, &Vector3f { x: 0.0, y: 0.05, z: 0.0 });
        }
    }
    else {
        if macros::is_excute(agent) {
            MotionModule::set_rate(agent.module_accessor, 1.0);
            KineticModule::add_speed(agent.module_accessor, &Vector3f { x: 0.0, y: 0.2, z: 0.0 });
        }
    }
    frame(agent.lua_state_agent, 8.0);
    if WorkModule::get_int64(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W as u64 {
        if macros::is_excute(agent) {
            KineticModule::add_speed(agent.module_accessor, &Vector3f { x: 0.0, y: 0.05, z: 0.0 });
        }
    }
    else {
        if macros::is_excute(agent) {
            MotionModule::set_rate(agent.module_accessor, 1.0);
            KineticModule::add_speed(agent.module_accessor, &Vector3f { x: 0.0, y: 0.2, z: 0.0 });
        }
    }
    frame(agent.lua_state_agent, 9.0);
    if WorkModule::get_int64(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W as u64 {
        if macros::is_excute(agent) {
            KineticModule::add_speed(agent.module_accessor, &Vector3f { x: 0.0, y: 0.05, z: 0.0 });
        }
    }
    else {
        if macros::is_excute(agent) {
            MotionModule::set_rate(agent.module_accessor, 1.0);
            KineticModule::add_speed(agent.module_accessor, &Vector3f { x: 0.0, y: 0.2, z: 0.0 });
        }
    }
    frame(agent.lua_state_agent, 10.0);
    if WorkModule::get_int64(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W as u64 {
        if macros::is_excute(agent) {
            KineticModule::add_speed(agent.module_accessor, &Vector3f { x: 0.0, y: 0.05, z: 0.0 });
        }
    }
    else {
        if macros::is_excute(agent) {
            MotionModule::set_rate(agent.module_accessor, 1.0);
            KineticModule::add_speed(agent.module_accessor, &Vector3f { x: 0.0, y: 0.2, z: 0.0 });
        }
    }
    frame(agent.lua_state_agent, 11.0);
    if WorkModule::get_int64(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W as u64 {
        if macros::is_excute(agent) {
            KineticModule::add_speed(agent.module_accessor, &Vector3f { x: 0.0, y: 0.05, z: 0.0 });
        }
    }
    else {
        if macros::is_excute(agent) {
            MotionModule::set_rate(agent.module_accessor, 1.0);
            KineticModule::add_speed(agent.module_accessor, &Vector3f { x: 0.0, y: 0.2, z: 0.0 });
        }
    }
    frame(agent.lua_state_agent, 12.0);
    if WorkModule::get_int64(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W as u64 {
        if macros::is_excute(agent) {
            KineticModule::add_speed(agent.module_accessor, &vars::ZERO_VECTOR);
        }
    }
    else {
        if macros::is_excute(agent) {
            MotionModule::set_rate(agent.module_accessor, 1.0);
            KineticModule::add_speed(agent.module_accessor, &Vector3f { x: 0.0, y: 0.2, z: 0.0 });
        }
    }
    frame(agent.lua_state_agent, 13.0);
    if WorkModule::get_int64(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W as u64 {
        if macros::is_excute(agent) {
            KineticModule::add_speed(agent.module_accessor, &vars::ZERO_VECTOR);
        }
    }
    else {
        if macros::is_excute(agent) {
            MotionModule::set_rate(agent.module_accessor, 1.0);
            KineticModule::add_speed(agent.module_accessor, &Vector3f { x: 0.0, y: 0.2, z: 0.0 });
        }
    }
    frame(agent.lua_state_agent, 14.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
        if WorkModule::get_int64(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) != *FIGHTER_DOLLY_STRENGTH_W as u64 {
            if macros::is_excute(agent) {
                let output = dolly_calc_special_cancel(agent, 14.0, 60);
                macros::ATTACK(agent, 0, 0, Hash40::new("top"), output.dmg, 50, 80, 0, output.bkb, 7.0, 0.0, 8.0, 5.5, Some(0.0), Some(4.0), Some(3.5), 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            }
        }
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        MotionModule::set_rate(agent.module_accessor, 10.0);
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_DOLLY_SPECIAL_LW_FALL);
        MotionModule::set_rate(agent.module_accessor, 1.0);
    }
    frame(agent.lua_state_agent, 37.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_LW_WORK_FLAG_LANDING_HEAVY);
    }
}

unsafe extern "C" fn game_speciallwbreaking(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::WHOLE_HIT(agent, *HIT_STATUS_XLU);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
    }
}

unsafe extern "C" fn effect_speciallwbreaking(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_COLOR(agent, 0.3, 0.3, 0.8);
    }
    for _ in 0..5 {
        if macros::is_excute(agent) {
            macros::FLASH(agent, 0.1, 0.1, 1, 0.5);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::FLASH_FRM(agent, 2, 0.1, 0.1, 0.9, 0);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::COL_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 1.0);
    }
}

unsafe extern "C" fn sound_speciallwbreaking(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        let rand = sv_math::rand(hash40("fighter"), 10);
        if rand < 1 {
            macros::PLAY_SE(agent, Hash40::new("vc_dolly_special_l03"));
        }
        else {
            macros::PLAY_SE(agent, Hash40::new("vc_dolly_ottotto"));
        }
    }
}

unsafe extern "C" fn expression_speciallwbreaking(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_rush"),
            8,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialn", game_specialn, Priority::Low);
    agent.acmd("sound_specialn", sound_specialn, Priority::Low);

    agent.acmd("game_specialairn", game_specialairn, Priority::Low);
    agent.acmd("sound_specialairn", sound_specialn, Priority::Low);

    agent.acmd("game_specialsfstart", game_specialsfstart, Priority::Low);

    agent.acmd("game_specialairsfstart", game_specialsfstart, Priority::Low);

    agent.acmd("game_specialsfattack", game_specialsfattack, Priority::Low);

    agent.acmd("game_specialffeint", game_specialffeint, Priority::Low);
    agent.acmd("effect_specialffeint", effect_specialffeint, Priority::Low);
    agent.acmd("sound_specialffeint", sound_specialffeint, Priority::Low);
    agent.acmd("expression_specialffeint", expression_specialffeint, Priority::Low);

    agent.acmd("game_specialsbattackw", game_specialsbattackw, Priority::Low);

    agent.acmd("game_specialsbattack", game_specialsbattack, Priority::Low);
    agent.acmd("effect_specialsbattack", effect_specialsbattack, Priority::Low);
    agent.acmd("sound_specialsbattack", sound_specialsbattack, Priority::Low);
    agent.acmd("expression_specialsbattack", expression_specialsbattack, Priority::Low);

    agent.acmd("game_specialsbend", game_specialsbend, Priority::Low);

    agent.acmd("game_specialhi1", game_specialhi, Priority::Low);
    agent.acmd("expression_specialhi1", expression_specialhi, Priority::Low);

    agent.acmd("game_specialairhi1", game_specialhi, Priority::Low);
    agent.acmd("expression_specialairhi1", expression_specialhi, Priority::Low);

    agent.acmd("game_specialhicommand", game_specialhicommand, Priority::Low);
    agent.acmd("expression_specialhicommand", expression_specialhicommand, Priority::Low);

    agent.acmd("game_specialairhicommand", game_specialhicommand, Priority::Low);
    agent.acmd("expression_specialairhicommand", expression_specialhicommand, Priority::Low);

    agent.acmd("game_speciallwstart", game_speciallwstart, Priority::Low);
    agent.acmd("effect_speciallwstart", effect_speciallwstart, Priority::Low);
    agent.acmd("expression_speciallwstart", expression_speciallwstart, Priority::Low);

    agent.acmd("game_specialairlwstart", game_speciallwstart, Priority::Low);
    agent.acmd("effect_specialairlwstart", effect_specialairlwstart, Priority::Low);
    agent.acmd("expression_specialairlwstart", expression_speciallwstart, Priority::Low);

    agent.acmd("game_specialairlwrisew", game_specialairlwrisew, Priority::Low);

    agent.acmd("game_specialairlwrise", game_specialairlwrise, Priority::Low);

    agent.acmd("game_specialairlw", game_specialairlw, Priority::Low);

    agent.acmd("game_speciallwbreaking", game_speciallwbreaking, Priority::Low);
    agent.acmd("effect_speciallwbreaking", effect_speciallwbreaking, Priority::Low);
    agent.acmd("sound_speciallwbreaking", sound_speciallwbreaking, Priority::Low);
    agent.acmd("expression_speciallwbreaking", expression_speciallwbreaking, Priority::Low);
}