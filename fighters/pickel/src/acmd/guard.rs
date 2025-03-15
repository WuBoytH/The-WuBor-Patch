use super::*;

unsafe extern "C" fn game_guardcancelattack(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
        WorkModule::set_int(agent.module_accessor, *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_SWORD, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_REQUEST_HAVE_CRAFT_WEAPON_KIND);
    }
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 9.0 / 10.0);
    frame(agent.lua_state_agent, 11.0);
    let material = WorkModule::get_int(agent.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
    let tool_params = match material {
        6 => Some((12.0, 14.0)), // Diamond
        4 => Some((6.0, 18.0)), // Gold
        3 => Some((10.0, 14.0)), // Iron
        2 => Some((8.0, 14.0)), // Stone
        1 => Some((6.0, 9.0)), // Wood
        _ => None
    };
    if let Some((damage, durability)) = tool_params {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), damage, 361, 0, 0, 80, 3.8, 0.0, 8.0, 6.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), damage, 361, 0, 0, 80, 4.4, 0.0, 8.0, 10.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), damage, 361, 0, 0, 80, 4.6, 0.0, 8.0, 14.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            WorkModule::set_float(agent.module_accessor, durability, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 0, 0, 80, 3.8, 0.0, 8.0, 6.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 361, 0, 0, 80, 4.4, 0.0, 8.0, 10.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_guardcancelattack(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_flash"), Hash40::new("top"), 0.0, 12.0, 6.0, 0.0, 0.0, 0.0, 1.4, true);
    }
    
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 6, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    let material = WorkModule::get_int(agent.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
    let effects = match material {
        6 => Some(("pickel_sword_flare_diamond", "pickel_atks4_slash_diamond")), // Diamond
        4 => Some(("pickel_sword_flare_gold", "pickel_atks4_slash_gold")), // Gold
        3 => Some(("pickel_sword_flare_iron", "pickel_atks4_slash_iron")), // Iron
        2 => Some(("pickel_sword_flare_stone", "pickel_atks4_slash_stone")), // Stone
        1 => Some(("pickel_sword_flare_wood", "pickel_atks4_slash_wood")), // Wood
        _ => None
    };
    if let Some((flare, slash)) = effects {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new(flare), Hash40::new("weaponr"), 0, 0, 0, 0, 180, 0, 1.25, true);
        }
        frame(agent.lua_state_agent, 10.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new(slash), Hash40::new(slash), Hash40::new("top"), 0, 11, 3, 6, 7, 32, 1.15, true, *EF_FLIP_YZ);
            macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("pickel_sweep"), Hash40::new("pickel_sweep"), Hash40::new("top"), -1, 10, 17.3, 10, 175, 40, 1.2, true, *EF_FLIP_YZ);
        }
    }
    else {
        frame(agent.lua_state_agent, 11.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FLIP(agent, Hash40::new("sys_attack_impact"), Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 8, 9.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        }
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("pickel_sword_flare_diamond"), false, true);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("pickel_sword_flare_gold"), false, true);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("pickel_sword_flare_iron"), false, true);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("pickel_sword_flare_stone"), false, true);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("pickel_sword_flare_wood"), false, true);
    }
}

unsafe extern "C" fn sound_guardcancelattack(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_guard_cancel_attack"));
    }
    let material = WorkModule::get_int(agent.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
    let is_sword = [
        *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND,
        *FIGHTER_PICKEL_MATERIAL_KIND_GOLD,
        *FIGHTER_PICKEL_MATERIAL_KIND_IRON,
        *FIGHTER_PICKEL_MATERIAL_KIND_STONE,
        *FIGHTER_PICKEL_MATERIAL_KIND_WOOD,
    ].contains(&material);
    if is_sword {
        frame(agent.lua_state_agent, 11.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_pickel_smash_s01"));
        }
    }
    else {
        frame(agent.lua_state_agent, 9.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_pickel_swing_l"));
        }
    }
}

unsafe extern "C" fn expression_guardcancelattack(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 3);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_77_nohitm"), 7, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    let material = WorkModule::get_int(agent.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
    let (rumble, length) = match material {
        6 => ("rbkind_slash_critical", 22), // Diamond
        4 => ("rbkind_slashl", 12), // Gold
        3 => ("rbkind_slashll", 9), // Iron
        2 => ("rbkind_77_attacks", 0), // Stone
        1 => ("rbkind_77_attacks", 0), // Wood
        _ => ("rbkind_77_attackm", 0)
    };
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new(rumble), length);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_guardcancelattack", game_guardcancelattack, Priority::Low);
    agent.acmd("effect_guardcancelattack", effect_guardcancelattack, Priority::Low);
    agent.acmd("sound_guardcancelattack", sound_guardcancelattack, Priority::Low);
    agent.acmd("expression_guardcancelattack", expression_guardcancelattack, Priority::Low);
}