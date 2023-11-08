use {
    smash::{
        lua2cpp::L2CAgentBase,
        hash40,
        phx::Hash40,
        app::{lua_bind::*, sv_animcmd::*, *},
        lib::lua_const::*
    },
    smash_script::*,
    custom_var::*,
    wubor_utils::{wua_bind::*, vars::*},
    // super::super::vars::*
};

unsafe extern "C" fn edge_appeals(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 39.0);
    let hold_button = VarModule::get_int(agent.module_accessor, appeal::int::HOLD_BUTTON);
    if ControlModule::check_button_on(agent.module_accessor, hold_button) {
        if macros::is_excute(agent) {
            MiscModule::set_appeal_loop(
                agent.module_accessor,
                false,
                hash40("appeal_s_loop"),
                48
            );
            VarModule::set_int64(agent.module_accessor, appeal::int64::ACTION_MOT, hash40("appeal_s_attack"));
            VarModule::set_int(agent.module_accessor, appeal::int::ACTION_BUTTON, *CONTROL_PAD_BUTTON_ATTACK);
            VarModule::on_flag(agent.module_accessor, appeal::flag::ENABLE_ACTION);
        }
    }
}

unsafe extern "C" fn edge_appealsloop_eff(agent: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        frame(agent.lua_state_agent, 120.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("edge_sword_light"), Hash40::new("swordl2"), 0, 0, 0, 0, 180, -90, 1, true);
        }
        if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
            frame(agent.lua_state_agent, 121.0);
            if macros::is_excute(agent) {
                macros::EFFECT(agent, Hash40::new("edge_sword_flash"), Hash40::new("swordl2"), 12, 0, -0.2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
                macros::LAST_EFFECT_SET_RATE(agent, 0.85);
            }
        }
        else {
            frame(agent.lua_state_agent, 124.0);
            if macros::is_excute(agent) {
                macros::EFFECT(agent, Hash40::new("edge_sword_flash"), Hash40::new("swordl2"), 12, 0, -0.2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            }
        }
        frame(agent.lua_state_agent, 135.0);
        if macros::is_excute(agent) {
            VarModule::set_int64(agent.module_accessor, appeal::int64::ACTION_MOT, hash40("appeal_s_attack_just"));
        }
        frame(agent.lua_state_agent, 138.0);
        if macros::is_excute(agent) {
            VarModule::set_int64(agent.module_accessor, appeal::int64::ACTION_MOT, hash40("appeal_s_attack"));
        }
        agent.clear_lua_stack();
        wait_loop_sync_mot(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
}

unsafe extern "C" fn edge_appealsloop_snd(agent: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        frame(agent.lua_state_agent, 116.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_edge_appeal_s02"));
        }
        agent.clear_lua_stack();
        wait_loop_sync_mot(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
}

unsafe extern "C" fn edge_appealsloop_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
    }
    for _ in 0..i32::MAX {
        frame(agent.lua_state_agent, 116.0);
        if macros::is_excute(agent) {
            ControlModule::set_rumble(
                agent.module_accessor,
                Hash40::new("rbkind_nohits"),
                0,
                false,
                *BATTLE_OBJECT_ID_INVALID as u32
            );
        }
        frame(agent.lua_state_agent, 125.0);
        if macros::is_excute(agent) {
            ControlModule::set_rumble(
                agent.module_accessor,
                Hash40::new("rbkind_nohitll"),
                0,
                false,
                *BATTLE_OBJECT_ID_INVALID as u32
            );
        }
        agent.clear_lua_stack();
        wait_loop_sync_mot(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
}

unsafe extern "C" fn edge_appealsattack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 8.0, 3.0);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("swordl1"), 20.0, 361, 78, 0, 54, 4.0, 0.0, -1.0, 4.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("swordl1"), 20.0, 361, 78, 0, 54, 4.0, 7.0, -1.0, 4.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("swordl1"), 20.0, 361, 78, 0, 54, 4.0, 15.0, -1.0, 4.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 4, 0, Hash40::new("swordl1"), 20.0, 361, 78, 0, 54, 5.8, 21.5, -1.0, 4.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 20.0, 361, 78, 0, 54, 5.8, 0.0, 8.5, 6.5, Some(0.0), Some(8.5), Some(6.5), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 5, 0, Hash40::new("top"), 20.0, 361, 78, 0, 54, 5.5, 0.0, 9.0, 16.5, Some(0.0), Some(9.0), Some(21.5), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 42.0);
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 4.0, 3.0);
    }
}

unsafe extern "C" fn edge_appealsattack_eff(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 9, 13, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("edge_sword_smash_flash"), Hash40::new("swordl2"), 0, 0, 0, 0, 180, -90, 1, true);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_edge_sword1"), Hash40::new("tex_edge_sword2"), 6, Hash40::new("swordl2"), -4.0, 0.0, -0.7, Hash40::new("swordl2"), 29.2, 0.0, 1.5, true, Hash40::new("null"), Hash40::new("swordl2"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        macros::EFFECT_FOLLOW(agent, Hash40::new("edge_sword_flare"), Hash40::new("swordl2"), 0, 0, 0, 0, 180, -90, 1, true);
        macros::LAST_EFFECT_SET_RATE(agent, 3.0);
        macros::EFFECT_FOLLOW(agent, Hash40::new("edge_sword_light3"), Hash40::new("swordl2"), 0, 0, 0, 0, 180, -90, 1, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.2);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.9, 10, 0, 4, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 1.2);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("edge_sword_flare"), false, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("edge_sword"), Hash40::new("top"), -1, 13.2, 3.5, 0, -50, 24, 0.95, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.1);
        macros::EFFECT_FOLLOW(agent, Hash40::new("edge_slash_light"), Hash40::new("top"), -1, 13.2, 3.5, 0, 40, 24, 1.05, true);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 2);
    }
}

unsafe extern "C" fn edge_appealsattack_snd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_edge_smash_s01"));
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_edge_smash_s02"));
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_edge_rnd_attack_smash"));
    }
    frame(agent.lua_state_agent, 18.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_ACTIVATED) {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_edge_winged_smash_s01"));
        }
    }
    frame(agent.lua_state_agent, 43.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_edge_smash_s03"));
    }
    frame(agent.lua_state_agent, 58.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_edge_step_right_s"));
    }
    frame(agent.lua_state_agent, 69.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_edge_step_left_s"));
    }
}

unsafe extern "C" fn edge_appealsattack_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(
            agent.module_accessor,
            Hash40::new("top"),
            AttackDirectionAxis(*ATTACK_DIRECTION_Z),
            AttackDirectionAxis(*ATTACK_DIRECTION_X_MINUS),
            AttackDirectionAxis(*ATTACK_DIRECTION_Y)
        );
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_78_smash"), 30);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohitl_l"),
            9,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
}

unsafe extern "C" fn edge_appealsattackjust(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        damage!(agent, MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        macros::SLOW_OPPONENT(agent, 20.0, 30.0);
    }
    macros::FT_MOTION_RATE(agent, 30.0 / 22.0);
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 8.0, 3.0);
    }
    frame(agent.lua_state_agent, 22.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        damage!(agent, MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        macros::ATTACK(agent, 0, 0, Hash40::new("swordl1"), 777.7, 361, 78, 0, 54, 4.0, 0.0, -1.0, 4.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("swordl1"), 777.7, 361, 78, 0, 54, 4.0, 7.0, -1.0, 4.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("swordl1"), 777.7, 361, 78, 0, 54, 4.0, 15.0, -1.0, 4.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 4, 0, Hash40::new("swordl1"), 777.7, 361, 78, 0, 54, 5.8, 21.5, -1.0, 4.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 777.7, 361, 78, 0, 54, 5.8, 0.0, 8.5, 6.5, Some(0.0), Some(8.5), Some(6.5), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 5, 0, Hash40::new("top"), 777.7, 361, 78, 0, 54, 5.5, 0.0, 9.0, 16.5, Some(0.0), Some(9.0), Some(21.5), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 42.0);
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 4.0, 3.0);
    }
}

unsafe extern "C" fn edge_appealsattackjust_eff(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 9, 13, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("edge_aura"), Hash40::new("hip"), -2, -2, 0, 80, 90, 0, 1, true);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("edge_sword_smash_flash"), Hash40::new("swordl2"), 0, 0, 0, 0, 180, -90, 1, true);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_edge_sword1"), Hash40::new("tex_edge_sword2"), 6, Hash40::new("swordl2"), -4.0, 0.0, -0.7, Hash40::new("swordl2"), 29.2, 0.0, 1.5, true, Hash40::new("null"), Hash40::new("swordl2"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        macros::EFFECT_FOLLOW(agent, Hash40::new("edge_sword_flare"), Hash40::new("swordl2"), 0, 0, 0, 0, 180, -90, 1, true);
        macros::LAST_EFFECT_SET_RATE(agent, 3.0);
        macros::EFFECT_FOLLOW(agent, Hash40::new("edge_sword_light3"), Hash40::new("swordl2"), 0, 0, 0, 0, 180, -90, 1, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.2);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.9, 10, 0, 4, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 1.2);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("edge_sword_flare"), false, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("edge_sword"), Hash40::new("top"), -1, 13.2, 3.5, 0, -50, 24, 0.95, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.1);
        macros::EFFECT_FOLLOW(agent, Hash40::new("edge_slash_light"), Hash40::new("top"), -1, 13.2, 3.5, 0, 40, 24, 1.05, true);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 2);
    }
}

unsafe extern "C" fn edge_appealsattackjust_snd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_edge_smash_s01"));
        macros::PLAY_SE(agent, Hash40::new("vc_edge_win03"));
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_edge_smash_s02"));
    }
    frame(agent.lua_state_agent, 18.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_ACTIVATED) {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_edge_winged_smash_s01"));
        }
    }
    frame(agent.lua_state_agent, 43.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_edge_smash_s03"));
    }
    frame(agent.lua_state_agent, 58.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_edge_step_right_s"));
    }
    frame(agent.lua_state_agent, 69.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_edge_step_left_s"));
    }
}

unsafe extern "C" fn edge_appealsattackjust_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(
            agent.module_accessor,
            Hash40::new("top"),
            AttackDirectionAxis(*ATTACK_DIRECTION_Z),
            AttackDirectionAxis(*ATTACK_DIRECTION_X_MINUS),
            AttackDirectionAxis(*ATTACK_DIRECTION_Y)
        );
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_78_smash"), 30);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohitl_l"),
            9,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
}

unsafe extern "C" fn edge_appeallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 39.0);
    let hold_button = VarModule::get_int(agent.module_accessor, appeal::int::HOLD_BUTTON);
    if ControlModule::check_button_on(agent.module_accessor, hold_button) {
        if macros::is_excute(agent) {
            MiscModule::set_appeal_loop(
                agent.module_accessor,
                false,
                hash40("appeal_lw_loop"),
                60
            );
        }
    }
}

unsafe extern "C" fn edge_appeallwloop_eff(agent: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        frame(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("edge_aura"), Hash40::new("hip"), -2, -2, 0, 80, 90, 0, 1, true);
        }
        frame(agent.lua_state_agent, 60.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("edge_aura"), Hash40::new("hip"), -2, -2, 0, 80, 90, 0, 1, true);
        }
        agent.clear_lua_stack();
        wait_loop_sync_mot(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
}

unsafe extern "C" fn edge_appeallwloop_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
    }
    for _ in 0..i32::MAX {
        frame(agent.lua_state_agent, 70.0);
        if macros::is_excute(agent) {
            ControlModule::set_rumble(
                agent.module_accessor,
                Hash40::new("rbkind_elecattack"),
                100,
                true,
                *BATTLE_OBJECT_ID_INVALID as u32
            );
        }
        agent.clear_lua_stack();
        wait_loop_sync_mot(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.game_acmd("game_appealsl", edge_appeals);
    agent.game_acmd("game_appealsr", edge_appeals);

    agent.effect_acmd("effect_appealsloop", edge_appealsloop_eff);
    agent.sound_acmd("sound_appealsloop", edge_appealsloop_snd);
    agent.expression_acmd("expression_appealsloop", edge_appealsloop_exp);

    agent.game_acmd("game_appealsattack", edge_appealsattack);
    agent.effect_acmd("effect_appealsattack", edge_appealsattack_eff);
    agent.sound_acmd("sound_appealsattack", edge_appealsattack_snd);
    agent.expression_acmd("expression_appealsattack", edge_appealsattack_exp);

    agent.game_acmd("game_appealsattackjust", edge_appealsattackjust);
    agent.effect_acmd("effect_appealsattackjust", edge_appealsattackjust_eff);
    agent.sound_acmd("sound_appealsattackjust", edge_appealsattackjust_snd);
    agent.expression_acmd("expression_appealsattackjust", edge_appealsattackjust_exp);

    agent.game_acmd("game_appeallwl", edge_appeallw);
    agent.game_acmd("game_appeallwr", edge_appeallw);

    agent.effect_acmd("effect_appeallwloop", edge_appeallwloop_eff);
    agent.expression_acmd("expression_appeallwloop", edge_appeallwloop_exp);
}