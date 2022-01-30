use {
    smash::{
        lua2cpp::L2CAgentBase,
        phx::Hash40,
        app::lua_bind::WorkModule,
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*,
    wubor_utils::vars::*
};

#[acmd_script( agent = "purin", script = "game_shieldbreakfly", category = ACMD_GAME, low_priority )]
unsafe fn purin_shieldbreakfly(fighter: &mut L2CAgentBase) {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FUNNY) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 361, 1000, 500, 1000, 50.0, 0.0, 4.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BODY);
        }
    }
}

pub fn install() {
    install_acmd_scripts!(
        purin_shieldbreakfly
    );
}