use {
    smash::{
        lua2cpp::L2CAgentBase,
        phx::*,
        app::{lua_bind::*, sv_animcmd::*},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*
};

#[acmd_script( agent = "simon", script = "game_throwhi", category = ACMD_GAME, low_priority )]
unsafe fn simon_throwhi(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 8.0, 90, 90, 0, 78, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 35.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 89, 100, 0, 55, 2.0, 0.0, 23.0, 0.0, Some(0.0), Some(50.0), Some(0.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
        macros::CHECK_FINISH_CAMERA(fighter, 3, 34);
        FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 2.0);
        FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 0.0, y: 12.0, z: 0.0});
    }
    frame(fighter.lua_state_agent, 36.0);
    if macros::is_excute(fighter) {
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        AttackModule::clear_all(fighter.module_accessor);
    }
}

pub fn install() {
    install_acmd_scripts!(
        simon_throwhi
    );
}