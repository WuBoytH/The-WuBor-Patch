use {
    smash::{
        lua2cpp::L2CAgentBase,
        phx::Hash40,
        app::{lua_bind::*, sv_animcmd::*},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*,
    super::super::vl,
    wubor_utils::vars::*
};

#[acmd_script( agent = "kirby", scripts = [ "game_appealsl", "game_appealsr" ], category = ACMD_GAME, low_priority )]
unsafe fn kirby_appeals(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L | *CONTROL_PAD_BUTTON_APPEAL_S_R)
    && !WorkModule::is_flag(fighter.module_accessor, FIGHTER_KIRBY_STATUS_APPEAL_WORK_FLAG_APPEAL_S_HOLD) {
        if macros::is_excute(fighter) {
            WorkModule::set_int(fighter.module_accessor, 10, FIGHTER_STATUS_APPEAL_WORK_INT_APPEAL_RESTART_FRAME);
            WorkModule::on_flag(fighter.module_accessor, FIGHTER_KIRBY_STATUS_APPEAL_WORK_FLAG_APPEAL_S_HOLD);
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new("appeal_s_loop"),
                0.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
        }
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        let damage;
        let effect;
        let sound_level;
        let sound;
        let loops = WorkModule::get_int(fighter.module_accessor, FIGHTER_KIRBY_STATUS_APPEAL_WORK_INT_APPEAL_S_LOOP_COUNT);
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
        macros::ATTACK(fighter, 0, 0, Hash40::new("footl"), damage, 60, 50, 0, 72, 3.3, 1.0, -4.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, effect, sound_level, sound, *ATTACK_REGION_KICK);
        if loops > 10 {
            AttackModule::set_attack_level(fighter.module_accessor, 0, *FIGHTER_RYU_SAVING_LV_3 as u8);
        }
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "kirby", scripts = [ "effect_appealsl", "effect_appealsr" ], category = ACMD_EFFECT, low_priority )]
unsafe fn kirby_appeals_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.65, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 1, 0, 0, 0, 180, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 48.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script( agent = "kirby", scripts = [ "sound_appealsl", "sound_appealsr" ], category = ACMD_SOUND, low_priority )]
unsafe fn kirby_appeals_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_swing_02"));
    }
    wait(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_swing_02"));
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_kirby_dash_stop"));
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_kirby_004"));
    }
    frame(fighter.lua_state_agent, 49.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_kirby_landing02"));
    }
}

#[acmd_script( agent = "kirby", scripts = [ "expression_appealsl", "expression_appealsr" ], category = ACMD_EXPRESSION, low_priority )]
unsafe fn kirby_appeals_exp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        slope!(fighter, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(
            fighter.module_accessor,
            Hash40::new("rbkind_nohits"),
            6,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(
            fighter.module_accessor,
            Hash40::new("rbkind_nohits"),
            10,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(
            fighter.module_accessor,
            Hash40::new("rbkind_nohitm"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
}

#[acmd_script( agent = "kirby", script = "effect_appealsloop", category = ACMD_EFFECT, low_priority )]
unsafe fn kirby_appealsloop_eff(fighter: &mut L2CAgentBase) {
    for x in 0..i32::MAX {
        if macros::is_excute(fighter) {
            WorkModule::set_int(fighter.module_accessor, x + 1, FIGHTER_KIRBY_STATUS_APPEAL_WORK_INT_APPEAL_S_LOOP_COUNT);
        }
        frame(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.65, 0, 0, 0, 0, 0, 0, false);
        }
        fighter.clear_lua_stack();
        wait_loop_sync_mot(fighter.lua_state_agent);
        fighter.pop_lua_stack(1);
    }
}

#[acmd_script( agent = "kirby", script = "sound_appealsloop", category = ACMD_SOUND, low_priority )]
unsafe fn kirby_appealsloop_snd(fighter: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        frame(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_common_swing_02"));
        }
        fighter.clear_lua_stack();
        wait_loop_sync_mot(fighter.lua_state_agent);
        fighter.pop_lua_stack(1);
    }
}

#[acmd_script( agent = "kirby", script = "expression_appealsloop", category = ACMD_EXPRESSION, low_priority )]
unsafe fn kirby_appealsloop_exp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        slope!(fighter, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
    }
    for _ in 0..i32::MAX {
        let loops = WorkModule::get_int(fighter.module_accessor, FIGHTER_KIRBY_STATUS_APPEAL_WORK_INT_APPEAL_S_LOOP_COUNT);
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
        if loops % 5 == 0
        && quake != -1 {
            if macros::is_excute(fighter) {
                macros::QUAKE(fighter, quake);
            }
        }
        frame(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            ControlModule::set_rumble(
                fighter.module_accessor,
                rbkind,
                6,
                false,
                *BATTLE_OBJECT_ID_INVALID as u32
            );
        }
        fighter.clear_lua_stack();
        wait_loop_sync_mot(fighter.lua_state_agent);
        fighter.pop_lua_stack(1);
    }
}

pub fn install() {
    install_acmd_scripts!(
        kirby_appeals, kirby_appeals_eff, kirby_appeals_snd, kirby_appeals_exp,
        kirby_appealsloop_eff, kirby_appealsloop_snd, kirby_appealsloop_exp
    );
}