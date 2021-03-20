use smash::phx::Hash40;
use smash::lua2cpp::L2CAgentBase;
// use smash::app::sv_animcmd;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash_script::*;
use smash_script::macros;
use crate::IS_FUNNY;

#[script( agent = "purin", script = "game_shieldbreakfly", category = ACMD_GAME )]
unsafe fn purin_shieldbreak(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if IS_FUNNY[entry_id] {
        if macros::is_excute(fighter) {
            smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 361, 1000, 500, 1000, 50.0, 0.0, 4.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BODY);
        }
    }
}

pub fn install() {
    smash_script::replace_scripts!(
        purin_shieldbreak
    );
}