use smash::phx::Hash40;
use smash::lua2cpp::L2CAgentBase;
// use smash::app::*;
use smash::lib::lua_const::*;
// use smash::app::lua_bind::*;
use smash_script::*;
use smashline::*;
use crate::IS_FUNNY;
use crate::commonfuncs::*;

#[acmd_script( agent = "purin", script = "game_shieldbreakfly", category = ACMD_GAME, low_priority )]
unsafe fn purin_shieldbreak(fighter: &mut L2CAgentBase) {
    if IS_FUNNY[entry_id(fighter.module_accessor)] {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 361, 1000, 500, 1000, 50.0, 0.0, 4.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BODY);
        }
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        purin_shieldbreak
    );
}