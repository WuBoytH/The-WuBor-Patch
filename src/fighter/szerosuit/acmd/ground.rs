use {
    smash::{
        lua2cpp::L2CAgentBase,
        phx::Hash40,
        app::{lua_bind::*, sv_animcmd::*, *},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*
};

#[acmd_script( agent = "szerosuit", script = "game_attack11", category = ACMD_GAME, low_priority )]
unsafe fn szerosuit_attack11(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 361, 50, 0, 20, 2.0, 0.0, 10.0, 5.5, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.5, 361, 33, 0, 15, 2.3, 0.0, 10.0, 8.5, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 1.5, 180, 20, 0, 10, 2.5, 0.0, 10.0, 11.5, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 1.5, 361, 20, 0, 10, 2.5, 0.0, 10.0, 11.5, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 10.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 6.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 2.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 3, 2.0, false);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART);
    }
}

#[acmd_script( agent = "szerosuit", script = "game_attacklw3", category = ACMD_GAME, low_priority )]
unsafe fn szerosuit_attacklw3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("kneel"), 8.0, 70, 85, 0, 60, 3.8, 5.2, 0.0, 0.0, Some(-2.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

pub fn install() {
    install_acmd_scripts!(
        szerosuit_attack11,
        szerosuit_attacklw3
    );
}