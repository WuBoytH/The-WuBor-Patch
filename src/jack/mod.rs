use smash::{
    lua2cpp::L2CAgentBase,
    phx::Hash40,
    app::{lua_bind::*, sv_animcmd::*/*, **/},
    lib::lua_const::*
};
use smash_script::*;
use smashline::*;

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
    smashline::install_acmd_scripts!(
        jack_dspecial,
        jack_dspecialcounter
    );
}