use {
    smash::{
        lua2cpp::L2CAgentBase,
        phx::Hash40,
        app::{lua_bind::*, sv_animcmd::*},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*,
    wubor_utils::vars::*
};

#[acmd_script( agent = "wario", scripts = ["game_speciallwsr", "game_specialairlwsr"], category = ACMD_GAME, low_priority )]
unsafe fn wario_speciallwsr(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 361, 1, 0, 0, 9.0, 0.0, 4.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 1.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_slip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "wario", scripts = ["game_speciallwmr", "game_specialairlwmr"], category = ACMD_GAME, low_priority )]
unsafe fn wario_speciallwmr(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 85, 50, 0, 90, 10.0, 0.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 85, 50, 0, 80, 5.0, 0.0, 3.0, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 6.0, 85, 50, 0, 80, 5.0, 0.0, 3.0, -5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 16.0);
    if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
        if macros::is_excute(fighter) {
            CancelModule::enable_cancel(fighter.module_accessor);
        }
    }
}

#[acmd_script( agent = "wario", scripts = ["game_speciallwlr", "game_specialairlwlr"], category = ACMD_GAME, low_priority )]
unsafe fn wario_speciallwlr(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 1.6);
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 40, 40, 0, 100, 11.0, 0.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_SPECIAL_LW_FLAG_JUMP);
    }
    frame(fighter.lua_state_agent, 7.8);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    macros::FT_MOTION_RATE(fighter, 1.0);
}

#[acmd_script( agent = "wario", scripts = ["game_speciallwflyr", "game_specialairlwflyr"], category = ACMD_GAME, low_priority )]
unsafe fn wario_speciallwflyr(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 20.0, 35, 75, 0, 50, 11.0, 0.0, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_SPECIAL_LW_FLAG_JUMP);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_SPECIAL_LW_FLAG_CRITICAL_HIT);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_SPECIAL_LW_FLAG_CRITICAL_HIT);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 18.0, 80, 75, 0, 0, 7.0, 0.0, 9.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
    }
    wait(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "wario", scripts = [
    "game_appeallwl", "game_appeallwr", "game_appealsl", "game_appealsr", "game_appealhil", "game_appealhir"
    ], category = ACMD_GAME, low_priority )]
unsafe fn wario_appeal(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        WorkModule::inc_int(fighter.module_accessor, FIGHTER_WARIO_INSTANCE_WORK_ID_INT_FINISH_SIGN);
    }
}

pub fn install() {
    install_acmd_scripts!(
        wario_speciallwsr,
        wario_speciallwmr,
        wario_speciallwlr,
        wario_speciallwflyr,
        wario_appeal
    );
}