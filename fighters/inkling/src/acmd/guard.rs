use super::*;

unsafe extern "C" fn game_guardcancelattack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 6.0 / 2.0);
    frame(agent.lua_state_agent, 3.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 6.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_INK_SUCCESS) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 361, 0, 0, 80, 5.0, 0.0, 8.0, 12.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_WATER);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 361, 0, 0, 80, 5.0, 0.0, 8.0, 16.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_WATER);
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 8.0, 361, 0, 0, 80, 5.0, 0.0, 8.0, 21.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_WATER);
            AttackModule::set_ink_value(agent.module_accessor, 0, 10.0);
            AttackModule::set_ink_value(agent.module_accessor, 1, 10.0);
            AttackModule::set_ink_value(agent.module_accessor, 2, 10.0);
        }
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_guardcancelattack(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_flash"), Hash40::new("top"), 0.0, 12.0, 6.0, 0.0, 0.0, 0.0, 1.4, true);
    }
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_INK_SUCCESS) {
        frame(agent.lua_state_agent, 5.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("inkling_muzzle_shot2"), Hash40::new("muzzle"), -1, 0, 0, 0, 180, 0, 1.6, true);
        }
        let r = WorkModule::get_float(agent.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_R);
        let g = WorkModule::get_float(agent.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_G);
        let b = WorkModule::get_float(agent.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_B);
        if macros::is_excute(agent) {
            macros::LAST_PARTICLE_SET_COLOR(agent, r, g, b);
        }
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(agent.lua_state_agent, 7.0);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("inkling_muzzle_shot3"), Hash40::new("muzzle"), -1, 0, 0, 0, 180, 0, 1, true);
        }
        let r = WorkModule::get_float(agent.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_R);
        let g = WorkModule::get_float(agent.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_G);
        let b = WorkModule::get_float(agent.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_B);
        if macros::is_excute(agent) {
            macros::LAST_PARTICLE_SET_COLOR(agent, r, g, b);
        }
        frame(agent.lua_state_agent, 7.0);
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 1, 0, 0, 0, 180, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

unsafe extern "C" fn sound_guardcancelattack(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_guard_cancel_attack"));
    }
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_INK_SUCCESS) {
        frame(agent.lua_state_agent, 6.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_inkling_attack100_02"));
        }
    }
    else {
        frame(agent.lua_state_agent, 0.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_inkling_airshot"));
        }
    }
}

unsafe extern "C" fn expression_guardcancelattack(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 6.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_INK_SUCCESS) {
        if macros::is_excute(agent) {
            macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohit_attacks"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
    }
    else {
        if macros::is_excute(agent) {
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
    }
    frame(agent.lua_state_agent, 47.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_BOY) {
        if macros::is_excute(agent) {
            agent.clear_lua_stack();
            FT_MOTION_INTP_WAIT(agent.lua_state_agent);
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_guardcancelattack", game_guardcancelattack, Priority::Low);
    agent.acmd("effect_guardcancelattack", effect_guardcancelattack, Priority::Low);
    agent.acmd("sound_guardcancelattack", sound_guardcancelattack, Priority::Low);
    agent.acmd("expression_guardcancelattack", expression_guardcancelattack, Priority::Low);
}