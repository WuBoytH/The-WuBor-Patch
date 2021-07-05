use smash::phx::Hash40;
use smash::lua2cpp::L2CAgentBase;
// use smash::app::*;
use smash::app::sv_animcmd::*;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash_script::*;
use smashline::*;

#[acmd_script( agent = "gamewatch", script = "game_specialhi", category = ACMD_GAME, low_priority )]
unsafe fn gamewatch_uspecial(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 83, 36, 0, 132, 6.0, 0.0, 6.0, 7.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 83, 36, 0, 132, 6.0, 0.0, 6.0, -7.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 10.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 10.0, false);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("waist"), 6.0, 361, 80, 0, 60, 5.5, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_HI_FLAG_DECIDE_DIRECTION);
    }
    frame(fighter.lua_state_agent, 10.0);
    macros::FT_MOTION_RATE(fighter, 0.8);
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_HI_FLAG_LANDING_ENABLE);
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 30.0);
    macros::FT_MOTION_RATE(fighter, 0.8);
    frame(fighter.lua_state_agent, 33.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_HI_FLAG_PARACHUTE_OPEN);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        gamewatch_uspecial
    );
}