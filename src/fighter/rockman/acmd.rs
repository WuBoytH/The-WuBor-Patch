use {
    smash::{
        lua2cpp::L2CAgentBase,
        phx::Hash40,
        app::sv_animcmd::*,
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*,
    crate::{
        common_funcs::*,
        vars::*
    }
};

#[acmd_script( agent = "rockman_rockbuster", script = "game_regular", category = ACMD_GAME, low_priority )]
unsafe fn rockman_rockbuster_shoot(weapon: &mut L2CAgentBase) {
    if macros::is_excute(weapon) {
        if !IS_FUNNY[entry_id(weapon.module_accessor)] {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 2.0, 361, 30, 0, 22, 3.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, false, -1, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_ENERGY);
        }
        else {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 3.0, 361, 0, 0, 0, 3.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, false, -1, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_ENERGY);
        }
    }
    frame(weapon.lua_state_agent, 4.0);
    if macros::is_excute(weapon) {
        if !IS_FUNNY[entry_id(weapon.module_accessor)] {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 2.0, 361, 10, 0, 12, 2.2, 0.0, 0.0, 0.0, None, None, None, 0.75, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, false, -1, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_ENERGY);
        }
    }
    frame(weapon.lua_state_agent, 8.0);
    if macros::is_excute(weapon) {
        if !IS_FUNNY[entry_id(weapon.module_accessor)] {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 2.0, 361, 10, 0, 8, 1.8, 0.0, 0.0, 0.0, None, None, None, 0.1, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, false, -1, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_ENERGY);
        }
    }
}

pub fn install() {
    install_acmd_scripts!(
        rockman_rockbuster_shoot
    );
}