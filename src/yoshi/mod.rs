use smash::phx::Hash40;
use smash::lua2cpp::L2CAgentBase;
// use smash::app::*;
use smash::app::sv_animcmd::*;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash_script::*;
use smashline::*;
// use crate::system::IS_FUNNY;
// use crate::commonfuncs::*;

#[acmd_script( agent = "yoshi", script = "game_attack11", category = ACMD_GAME, low_priority )]
unsafe fn yoshi_jab1(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 80, 50, 0, 70, 3.5, 0.0, 5.6, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 32, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        JostleModule::set_status(fighter.module_accessor, false);
    }
}

#[acmd_script( agent = "yoshi", script = "game_specialsloop", category = ACMD_GAME, low_priority )]
unsafe fn yoshi_sspecialloop(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 80, 50, 0, 70, 3.5, 0.0, 5.6, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 32, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        JostleModule::set_status(fighter.module_accessor, false);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        yoshi_jab1,
        yoshi_sspecialloop
    );
}