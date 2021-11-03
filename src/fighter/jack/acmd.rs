use {
    smash::{
        lua2cpp::L2CAgentBase,
        phx::Hash40,
        app::{lua_bind::*, sv_animcmd::*, *},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*,
    crate::{
        common_funcs::*,
        vars::*
    }
};

#[acmd_script( agent = "jack", script = "game_specials1", category = ACMD_GAME, low_priority )]
unsafe fn jack_sspecial1(fighter: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(fighter, 14.0 / 9.0);
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    }
    frame(fighter.lua_state_agent, 9.0);
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        if macros::is_excute(fighter) {
            FEINT[entry_id(fighter.module_accessor)] = true;
        }
        macros::FT_MOTION_RATE(fighter, 5.0);
    }
    else {
        macros::FT_MOTION_RATE(fighter, 1.0);
    }
    frame(fighter.lua_state_agent, 13.0);
    if !FEINT[entry_id(fighter.module_accessor)] {
        macros::FT_MOTION_RATE(fighter, 1.0);
        if macros::is_excute(fighter) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 80, 20, 0, 80, 3.0, 0.0, 6.5, 5.0, Some(0.0), Some(6.5), Some(0.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    else {
        macros::FT_MOTION_RATE(fighter, 0.8);
        if macros::is_excute(fighter) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
            HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
            JostleModule::set_status(fighter.module_accessor, false);
        }
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        if !FEINT[entry_id(fighter.module_accessor)] {
            AttackModule::clear_all(fighter.module_accessor);
        }
        else {
            macros::FT_MOTION_RATE(fighter, 0.75);
            HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
            JostleModule::set_status(fighter.module_accessor, true);
        }
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    }
}

#[acmd_script( agent = "jack", script = "effect_specials1", category = ACMD_EFFECT, low_priority )]
unsafe fn jack_sspecial1eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        if !FEINT[entry_id(fighter.module_accessor)] {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.6, 0, 0, 0, 0, 0, 0, false);
            macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_jack_sword1"), Hash40::new("tex_jack_sword2"), 14, Hash40::new("knife"), 0.0, 0.25, 0.15, Hash40::new("knife"), 0.0, 5.8, 0.0, false, Hash40::new("none"), Hash40::new("none"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        }
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        if FEINT[entry_id(fighter.module_accessor)] {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.6, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        if !FEINT[entry_id(fighter.module_accessor)] {
            macros::AFTER_IMAGE_OFF(fighter, 0);
        }
    }
    frame(fighter.lua_state_agent, 20.0);
    for _ in 0..2 {
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_sliding_smoke"), Hash40::new("top"), 7, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
        }
        wait(fighter.lua_state_agent, 3.0);
    }
}

#[acmd_script( agent = "jack", script = "sound_specials1", category = ACMD_SOUND, low_priority )]
unsafe fn jack_sspecial1snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_jack_attackhard_l01"));
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_jack_rnd_attack_l"));
    }
}

#[acmd_script( agent = "jack", script = "expression_specials1", category = ACMD_EXPRESSION, low_priority )]
unsafe fn jack_sspecial1exp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        ItemModule::set_attach_item_visibility(fighter.module_accessor, false, 0);
        VisibilityModule::set_whole(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, true, 0);
        ItemModule::set_attach_item_visibility(fighter.module_accessor, true, 0);
        VisibilityModule::set_whole(fighter.module_accessor, true);
    }
}

#[acmd_script( agent = "jack", script = "game_specialairs1", category = ACMD_GAME, low_priority )]
unsafe fn jack_sspecial1air(fighter: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(fighter, 14.0 / 9.0);
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    }
    frame(fighter.lua_state_agent, 9.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 80, 20, 0, 70, 3.0, 0.0, 6.5, 5.0, Some(0.0), Some(6.5), Some(0.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_S_FLAG_ENABLE_CONTROL_ENERGY);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        macros::SET_SPEED_EX(fighter, 0.4, 0.2, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    frame(fighter.lua_state_agent, 34.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_S_FLAG_SET_FALL_NORMAL);
    }
    frame(fighter.lua_state_agent, 42.0);
    if macros::is_excute(fighter) {
        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL_SPECIAL, false);
    }
}

#[acmd_script( agent = "jack", script = "effect_specialairs1", category = ACMD_EFFECT, low_priority )]
unsafe fn jack_sspecial1aireff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
        macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_jack_sword1"), Hash40::new("tex_jack_sword2"), 14, Hash40::new("knife"), 0.0, 0.25, 0.15, Hash40::new("knife"), 0.0, 5.8, 0.0, false, Hash40::new("none"), Hash40::new("none"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::AFTER_IMAGE_OFF(fighter, 0);
    }
}

#[acmd_script( agent = "jack", script = "sound_specialairs1", category = ACMD_SOUND, low_priority )]
unsafe fn jack_sspecial1airsnd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_jack_attackhard_l01"));
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_jack_rnd_attack_l"));
    }
}

#[acmd_script( agent = "jack", script = "expression_specialairs1", category = ACMD_EXPRESSION, low_priority )]
unsafe fn jack_sspecial1airexp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        ItemModule::set_attach_item_visibility(fighter.module_accessor, false, 0);
        VisibilityModule::set_whole(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, true, 0);
        ItemModule::set_attach_item_visibility(fighter.module_accessor, true, 0);
        VisibilityModule::set_whole(fighter.module_accessor, true);
    }
}

#[acmd_script( agent = "jack", scripts = [ "game_speciallw", "game_specialairlw" ], category = ACMD_GAME, low_priority )]
unsafe fn jack_dspecial(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 2.0);
    frame(fighter.lua_state_agent, 3.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        smash_script::shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_SHIELD, 0, *FIGHTER_JACK_SHIELD_GROUP_KIND_SPECIAL_LW);
        smash_script::shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_JACK_REFLECTOR_KIND_SPECIAL_LW, *FIGHTER_REFLECTOR_GROUP_EXTEND);
    }
    frame(fighter.lua_state_agent, 32.0);
    if macros::is_excute(fighter) {
        smash_script::shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_SHIELD, 0, *FIGHTER_JACK_SHIELD_GROUP_KIND_SPECIAL_LW);
        smash_script::shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_JACK_REFLECTOR_KIND_SPECIAL_LW, *FIGHTER_REFLECTOR_GROUP_EXTEND);
    }
}

#[acmd_script( agent = "jack", scripts = [ "game_speciallwcounter", "game_specialairlwcounter" ], category = ACMD_GAME, low_priority )]
unsafe fn jack_dspecialcounter(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 361, 51, 0, 80, 11.0, 0.0, 15.0, 4.0, Some(0.0), Some(15.0), Some(19.0), 0.75, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 1, true, false);
    }
    frame(fighter.lua_state_agent, 32.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

pub fn install() {
    install_acmd_scripts!(
        jack_sspecial1,
        jack_sspecial1eff,
        jack_sspecial1snd,
        jack_sspecial1exp,
        jack_sspecial1air,
        jack_sspecial1aireff,
        jack_sspecial1airsnd,
        jack_sspecial1airexp,
        jack_dspecial,
        jack_dspecialcounter
    );
}