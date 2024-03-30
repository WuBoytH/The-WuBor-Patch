use crate::imports::*;
use super::super::helper::*;

unsafe extern "C" fn lucina_attack11(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 361, 16, 0, 20, 2.0, 0.0, 9.4, 6.2, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 361, 16, 0, 20, 5.0, 0.0, 9.4, 8.8, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART);
        AttackModule::clear_all(agent.module_accessor);
    }
    //frame(agent.lua_state_agent, 13.0);
    //macros::FT_MOTION_RATE(agent, 2.0);
    //frame(agent.lua_state_agent, 16.0);
    //macros::FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn lucina_attack11_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 18, -5, 0, 330, 90, 0.95, true);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), -1, 10, 10, 0, 0, 0, 1.15, 0, 0, 0, 0, 0, 360, true);
    }
}

unsafe extern "C" fn lucina_attack11_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 5, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
    }
}

unsafe extern "C" fn lucina_attack12(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("kneer"), 4.0, 361, 40, 0, 35, 4.2, 5.0, -1.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneer"), 4.0, 361, 40, 0, 35, 3.8, -1.0, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn lucina_attack12_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_line"), Hash40::new("top"), -5, 12, 2, 0, 0, 0, 0.95, true);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), -6, 12, 17, 0, 0, 0, 1.15, 0, 0, 0, 0, 0, 360, true);
    }
}

unsafe extern "C" fn lucina_attack12_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 5, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
    }
}

unsafe extern "C" fn lucina_attackdash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("kneer"), 1.0, 0, 40, 10, 40, 3.6, 5.0, -1.0, 1.5, Some(1.5), Some(-1.0), Some(1.5), 0.8, 0.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, -0.7, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 0, 40, 10, 60, 2.5, 0.0, 2.5, -2.6, None, None, None, 0.8, 0.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, -0.7, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        macros::HIT_NODE(agent, Hash40::new("kneel"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("legl"), *HIT_STATUS_XLU);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 1, false);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        let ratio = get_damage_mul(agent.module_accessor);
        if VarModule::is_flag(agent.module_accessor, yu::status::flag::ATTACK_DASH_BIG_GAMBLE) {
            macros::ATTACK(agent, 0, 0, Hash40::new("kneer"), 4.0 * ratio, 0, 40, 10, 40, 3.6, 5.0, -1.0, 1.5, Some(1.5), Some(-1.0), Some(1.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_KICK);
        }
        else {
            macros::ATTACK(agent, 0, 0, Hash40::new("kneer"), 6.0 * ratio, 361, 70, 0, 60, 3.6, 5.0, -1.0, 1.5, Some(1.5), Some(-1.0), Some(1.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_KICK);
            AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 5.0, false);
        }
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        HitModule::set_whole(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(agent.module_accessor);
        VarModule::on_flag(agent.module_accessor, yu::status::flag::ATTACK_DASH_BIG_GAMBLE_TRANSITION)
    }
}
unsafe extern "C" fn lucina_attackdash_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 8, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 8, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 8, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
}
unsafe extern "C" fn lucina_attackdash_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_lucina_rnd_attack"));
        macros::PLAY_SE(agent, Hash40::new("se_lucina_swing_ll"));
    }
}

unsafe extern "C" fn lucina_attackdash_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 6);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

unsafe extern "C" fn lucina_attacks3(agent: &mut L2CAgentBase) {
    if VarModule::is_flag(agent.module_accessor, yu::instance::flag::HEROIC_GRAB) {
        macros::FT_MOTION_RATE(agent, 0.5);
        VarModule::off_flag(agent.module_accessor, yu::instance::flag::HEROIC_GRAB);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        let ratio = get_damage_mul(agent.module_accessor);
        macros::ATTACK(agent, 0, 0, Hash40::new("sword1"), 8.0 * ratio, 361, 85, 0, 50, 3.5, 1.0, 0.0, 2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("armr"), 8.0 * ratio, 361, 85, 0, 50, 3.0, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("sword1"), 8.0 * ratio, 361, 85, 0, 50, 3.5, 1.0, 0.0, 7.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    macros::FT_MOTION_RATE(agent, 0.75);
    wait(agent.lua_state_agent, 12.0);
    macros::FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn lucina_attackhi3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 1.5);
    frame(agent.lua_state_agent, 5.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("sword1"), 1.5, 100, 60, 40, 50, 2.0, 0.0, -1.0, 1.0, Some(1.0), Some(1.0), Some(12.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("sword1"), 1.5, 100, 60, 40, 50, 2.0, 0.0, -1.0, 1.0, Some(1.0), Some(1.0), Some(12.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 9.0);
    let ratio = get_damage_mul(agent.module_accessor);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        macros::ATTACK(agent, 0, 0, Hash40::new("sword1"), 5.5 * ratio, 75, 60, 0, 65, 2.0, 0.0, -1.0, 1.0, Some(1.0), Some(0.0), Some(12.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("sword1"), 5.5 * ratio, 75, 60, 0, 65, 2.0, 0.0, -1.0, 1.0, Some(1.0), Some(-1.0), Some(12.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn lucina_attackhi3_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_lucina_sword1"), Hash40::new("tex_lucina_sword2"), 8, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("lucina_sword"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
        macros::FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 1);
    }
}

unsafe extern "C" fn lucina_attackhi3_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_lucina_rnd_attack"));
        macros::PLAY_SE(agent, Hash40::new("se_lucina_swing_l"));
    }
}

unsafe extern "C" fn lucina_attackhi3_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(
            agent.module_accessor,
            Hash40::new("haver"),
            AttackDirectionAxis(*ATTACK_DIRECTION_Z),
            AttackDirectionAxis(*ATTACK_DIRECTION_Y),
            AttackDirectionAxis(*ATTACK_DIRECTION_X)
        );
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashs"), 0);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
    }
}

unsafe extern "C" fn lucina_attacklw3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    macros::FT_MOTION_RATE(agent, 2.0);
    frame(agent.lua_state_agent, 4.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 5.0);
    macros::FT_MOTION_RATE(agent, 2.0);
    frame(agent.lua_state_agent, 6.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        let ratio = get_damage_mul(agent.module_accessor);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0 * ratio, 71, 44, 0, 57, 2.7, 0.0, 2.7, 16.7, Some(0.0), Some(4.3), Some(9.2), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.2, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("sword1"), 6.0 * ratio, 71, 44, 0, 57, 2.7, 0.0, 0.0, 8.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.2, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 19.0);
    macros::FT_MOTION_RATE(agent, 0.75);
    frame(agent.lua_state_agent, 23.0);
    macros::FT_MOTION_RATE(agent, 1.0);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_attack11", lucina_attack11);
    agent.acmd("effect_attack11", lucina_attack11_eff);
    agent.acmd("expression_attack11", lucina_attack11_exp);

    agent.acmd("game_attack12", lucina_attack12);
    agent.acmd("effect_attack12", lucina_attack12_eff);
    agent.acmd("expression_attack12", lucina_attack12_exp);

    agent.acmd("game_attackdash", lucina_attackdash);
    agent.acmd("effect_attackdash", lucina_attackdash_eff);
    agent.acmd("sound_attackdash", lucina_attackdash_snd);
    agent.acmd("expression_attackdash", lucina_attackdash_exp);

    agent.acmd("game_attacks3", lucina_attacks3);

    agent.acmd("game_attackhi3", lucina_attackhi3);
    agent.acmd("effect_attackhi3", lucina_attackhi3_eff);
    agent.acmd("sound_attackhi3", lucina_attackhi3_snd);
    agent.acmd("expression_attackhi3", lucina_attackhi3_exp);

    agent.acmd("game_attacklw3", lucina_attacklw3);
}