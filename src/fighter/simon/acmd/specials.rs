use {
    smash::{
        lua2cpp::L2CAgentBase,
        phx::Hash40,
        app::{lua_bind::*, sv_animcmd::*, *},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*,
    custom_var::*,
    wubor_utils::{wua_bind::*, vars::*}
};

#[acmd_script( agent = "simon", scripts = ["game_specialn", "game_specialairn"], category = ACMD_GAME )]
unsafe fn simon_specialn(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        VarModule::on_flag(fighter.battle_object, simon::status::flag::SPECIAL_N_SHOOT);
    }
}

#[acmd_script( agent = "simon", scripts = ["expression_specialn", "expression_specialairn"], category = ACMD_EXPRESSION )]
unsafe fn simon_specialn_exp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        slope!(fighter, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(
            fighter.module_accessor,
            Hash40::new("rbkind_lightthrow4item"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
}

#[acmd_script( agent = "simon_axe", script = "game_fly", category = ACMD_GAME )]
unsafe fn simon_axe_fly(weapon: &mut L2CAgentBase) {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    if sv_battle_object::is_active(owner_id)
    && sv_battle_object::kind(owner_id) == *FIGHTER_KIND_SIMON {
        let object = MiscModule::get_battle_object_from_id(owner_id);
        VarModule::set_int(object, simon::instance::int::AXE_ID, weapon.battle_object_id as i32);
    }
    if macros::is_excute(weapon) {
        macros::ATTACK(weapon, 0, 0, Hash40::new("axe"), 15.0, 74, 75, 0, 50, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 8, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        macros::ATK_SET_SHIELD_SETOFF_MUL(weapon, 0, 1.1);
    }
}

#[acmd_script( agent = "simon", scripts = ["game_speciallw", "game_specialairlw"], category = ACMD_GAME )]
unsafe fn simon_speciallw(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SIMON_STATUS_SPECIAL_LW_FLAG_GENERATE_HOLYWATER);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SIMON_STATUS_SPECIAL_LW_FLAG_SHOOT_HOLYWATER);
    }
}

#[acmd_script( agent = "simon", script = "effect_speciallw", category = ACMD_EFFECT )]
unsafe fn simon_speciallw_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("simon_bottle_appear"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.4, true);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("simon_bottle_release"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script( agent = "simon", script = "effect_specialairlw", category = ACMD_EFFECT )]
unsafe fn simon_specialairlw_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("simon_bottle_appear"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.4, true);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("simon_bottle_release"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script( agent = "simon", scripts = ["sound_speciallw", "sound_specialairlw"], category = ACMD_SOUND )]
unsafe fn simon_speciallw_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_simon_special_l01"));
        macros::PLAY_SE(fighter, Hash40::new("se_simon_special_l01"));
    }
}

#[acmd_script( agent = "simon", scripts = ["expression_speciallw", "expression_specialairlw"], category = ACMD_EXPRESSION )]
unsafe fn simon_speciallw_exp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        slope(*MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(
            fighter.module_accessor,
            Hash40::new("rbkind_lightthrowitem"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
}

pub fn install() {
    install_acmd_scripts!(
        simon_specialn,
        simon_specialn_exp,
        simon_axe_fly,
        simon_speciallw, simon_speciallw_eff, simon_specialairlw_eff, simon_speciallw_snd, simon_speciallw_exp
    );
}