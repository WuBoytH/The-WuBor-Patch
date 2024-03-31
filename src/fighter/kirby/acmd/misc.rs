use crate::imports::*;
use super::super::vl;

unsafe extern "C" fn game_appeals(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    let hold_button = VarModule::get_int(agent.module_accessor, appeal::int::HOLD_BUTTON);
    if ControlModule::check_button_on(agent.module_accessor, game_button)
    && !VarModule::is_flag(agent.module_accessor, appeal::flag::LOOP) {
        if macros::is_excute(agent) {
            MiscModule::set_appeal_loop(
                agent.module_accessor,
                true,
                hash40("appeal_s_loop"),
                9
            );
        }
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        let damage;
        let effect;
        let sound_level;
        let sound;
        let loops = VarModule::get_int(agent.module_accessor, kirby::status::int::APPEAL_S_LOOP_COUNT);
        if loops > vl::param_appeal_hi::appeal_hi_spin_level_3 {
            damage = 20.0 + loops as f32 * 2.0;
            effect = Hash40::new("collision_attr_normal");
            sound_level = *ATTACK_SOUND_LEVEL_L;
            sound = *COLLISION_SOUND_ATTR_HEAVY;
        }
        else if loops > vl::param_appeal_hi::appeal_hi_spin_level_2 {
            damage = 20.0;
            effect = Hash40::new("collision_attr_saving");
            sound_level = *ATTACK_SOUND_LEVEL_M;
            sound = *COLLISION_SOUND_ATTR_HEAVY;
        }
        else {
            damage = 8.0;
            effect = Hash40::new("collision_attr_normal");
            sound_level = *ATTACK_SOUND_LEVEL_M;
            sound = *COLLISION_SOUND_ATTR_KICK;
        }
        macros::ATTACK(agent, 0, 0, Hash40::new("footl"), damage, 60, 50, 0, 72, 3.3, 1.0, -4.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, effect, sound_level, sound, *ATTACK_REGION_KICK);
        if loops > 10 {
            AttackModule::set_attack_level(agent.module_accessor, 0, *FIGHTER_RYU_SAVING_LV_3 as u8);
        }
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_appeals(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.65, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 1, 0, 0, 0, 180, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 48.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_appeals(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_swing_02"));
    }
    wait(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_swing_02"));
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_kirby_dash_stop"));
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_kirby_004"));
    }
    frame(agent.lua_state_agent, 49.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_kirby_landing02"));
    }
}

unsafe extern "C" fn expression_appeals(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohits"),
            6,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohits"),
            10,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohitm"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
}

unsafe extern "C" fn effect_appealsloop(agent: &mut L2CAgentBase) {
    loop {
        if macros::is_excute(agent) {
            VarModule::inc_int(agent.module_accessor, kirby::status::int::APPEAL_S_LOOP_COUNT);
        }
        frame(agent.lua_state_agent, 5.0);
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.65, 0, 0, 0, 0, 0, 0, false);
        }
        agent.clear_lua_stack();
        wait_loop_sync_mot(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
}

unsafe extern "C" fn sound_appealsloop(agent: &mut L2CAgentBase) {
    loop {
        frame(agent.lua_state_agent, 4.0);
        let loops = VarModule::get_int(agent.module_accessor, kirby::status::int::APPEAL_S_LOOP_COUNT);
        let sound;
        if loops > vl::param_appeal_hi::appeal_hi_spin_level_3 + 20 {
            sound = Hash40::new("se_common_swing_06");
        }
        else if loops > vl::param_appeal_hi::appeal_hi_spin_level_3 {
            sound = Hash40::new("se_common_swing_04");
        }
        else {
            sound = Hash40::new("se_common_swing_02");
        }
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, sound);
        }
        agent.clear_lua_stack();
        wait_loop_sync_mot(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
}

unsafe extern "C" fn expression_appealsloop(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
    }
    loop {
        let loops = VarModule::get_int(agent.module_accessor, kirby::status::int::APPEAL_S_LOOP_COUNT);
        let quake;
        let rbkind;
        if loops > vl::param_appeal_hi::appeal_hi_spin_level_3 + 20 {
            quake = *CAMERA_QUAKE_KIND_L;
            rbkind = Hash40::new("rbkind_nohitl");
        }
        else if loops > vl::param_appeal_hi::appeal_hi_spin_level_3 {
            quake = *CAMERA_QUAKE_KIND_M;
            rbkind = Hash40::new("rbkind_nohitm");
        }
        else if loops > vl::param_appeal_hi::appeal_hi_spin_level_2 {
            quake = *CAMERA_QUAKE_KIND_S;
            rbkind = Hash40::new("rbkind_nohits");
        }
        else {
            quake = -1;
            rbkind = Hash40::new("rbkind_nohits");
        }
        if loops % 4 == 0
        && quake != -1 {
            if macros::is_excute(agent) {
                macros::QUAKE(agent, quake);
            }
        }
        frame(agent.lua_state_agent, 5.0);
        if macros::is_excute(agent) {
            ControlModule::set_rumble(
                agent.module_accessor,
                rbkind,
                6,
                false,
                *BATTLE_OBJECT_ID_INVALID as u32
            );
        }
        agent.clear_lua_stack();
        wait_loop_sync_mot(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_appealsl", game_appeals);
    agent.acmd("effect_appealsl", effect_appeals);
    agent.acmd("sound_appealsl", sound_appeals);
    agent.acmd("expression_appealsl", expression_appeals);

    agent.acmd("game_appealsr", game_appeals);
    agent.acmd("effect_appealsr", effect_appeals);
    agent.acmd("sound_appealsr", sound_appeals);
    agent.acmd("expression_appealsr", expression_appeals);

    agent.acmd("effect_appealsloop", effect_appealsloop);
    agent.acmd("sound_appealsloop", sound_appealsloop);
    agent.acmd("expression_appealsloop", expression_appealsloop);
}