use smash::phx::Hash40;
use smash::lua2cpp::L2CAgentBase;
// use smash::app::*;
// use smash::app::sv_animcmd::*;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash_script::*;
use smashline::*;
use crate::system::IS_FUNNY;
use crate::commonfuncs::*;

#[acmd_script( agent = "mario_pumpwater", script = "game_regular", category = ACMD_GAME, low_priority )]
unsafe fn mario_pumpwater_regular(weapon: &mut L2CAgentBase) {
    if macros::is_excute(weapon) {
        let mut angle : u64 = 55;
        if IS_FUNNY[entry_id(weapon.module_accessor)] {
            angle = 125;
        }
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 0.0, angle, 100, 70, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MARIO_WATER_PUMP, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(weapon.module_accessor);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        mario_pumpwater_regular
    );
}