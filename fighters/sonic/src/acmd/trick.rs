use super::*;

unsafe extern "C" fn game_trickhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 8.0 / 13.0);
    frame(agent.lua_state_agent, 14.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, vars::sonic::status::flag::TRICK_ENABLE_MOVEMENT);
        VarModule::on_flag(agent.module_accessor, vars::sonic::status::flag::TRICK_ENABLE_CONTROL);
    }
    MiscModule::calc_motion_rate_from_cancel_frame(agent, 14.0, 15.0);
}

unsafe extern "C" fn sound_trickhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_swing_01"));
    }
}

unsafe extern "C" fn expression_trickhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohitm"),
            7,
            true,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohitm"),
            7,
            true,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
}

unsafe extern "C" fn game_trickf(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 14.0 / 18.0);
    frame(agent.lua_state_agent, 19.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, vars::sonic::status::flag::TRICK_ENABLE_MOVEMENT);
        macros::ATTACK(agent, 0, 0, Hash40::new("legr"), 10.0, 25, 75, 0, 60, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneer"), 10.0, 25, 75, 0, 60, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("footr"), 10.0, 25, 75, 0, 60, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    macros::FT_MOTION_RATE(agent, 1.5);
    frame(agent.lua_state_agent, 43.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 60.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, vars::sonic::status::flag::TRICK_ENABLE_CONTROL);
    }
}

unsafe extern "C" fn effect_trickf(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 19.0);
    for _ in 0..3 {
        if macros::is_excute(agent) {
            if sv_animcmd::get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
                macros::EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 6.0, 0, 0, 0, -20, 1.3, true);
            }
            else {
                macros::EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 6.0, 0, 180, 0, 20, 1.3, true);
            }
            macros::LAST_EFFECT_SET_COLOR(agent, 0.4, 0.4, 1.0);
            macros::LAST_EFFECT_SET_RATE(agent, 2.0 / 3.0);
        }
        wait(agent.lua_state_agent, 8.0);
    }
}

unsafe extern "C" fn sound_trickf(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 19.0);
    for _ in 0..3 {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_common_swing_02"));
        }
        wait(agent.lua_state_agent, 8.0);
    }
}

unsafe extern "C" fn expression_trickf(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohitm"),
            7,
            true,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
    for _ in 0..3 {
        if macros::is_excute(agent) {
            ControlModule::set_rumble(
                agent.module_accessor,
                Hash40::new("rbkind_nohitl"),
                7,
                true,
                *BATTLE_OBJECT_ID_INVALID as u32
            );
        }
        wait(agent.lua_state_agent, 8.0);
    }
}

unsafe extern "C" fn game_trickb(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, vars::sonic::status::flag::TRICK_ENABLE_MOVEMENT);
        VarModule::on_flag(agent.module_accessor, vars::sonic::status::flag::TRICK_ENABLE_CONTROL);
    }
    MiscModule::calc_motion_rate_from_cancel_frame(agent, 6.0, 5.0);
}

unsafe extern "C" fn sound_trickb(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_swing_01"));
    }
}

unsafe extern "C" fn expression_trickb(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohitm"),
            7,
            true,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohits"),
            7,
            true,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_trickhi", game_trickhi, Priority::Low);
    agent.acmd("sound_trickhi", sound_trickhi, Priority::Low);
    agent.acmd("expression_trickhi", expression_trickhi, Priority::Low);

    agent.acmd("game_trickf", game_trickf, Priority::Low);
    agent.acmd("effect_trickf", effect_trickf, Priority::Low);
    agent.acmd("sound_trickf", sound_trickf, Priority::Low);
    agent.acmd("expression_trickf", expression_trickf, Priority::Low);

    agent.acmd("game_trickb", game_trickb, Priority::Low);
    agent.acmd("sound_trickb", sound_trickb, Priority::Low);
    agent.acmd("expression_trickb", expression_trickb, Priority::Low);
}