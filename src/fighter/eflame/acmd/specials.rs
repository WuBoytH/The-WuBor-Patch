use {
    smash::{
        lua2cpp::L2CAgentBase,
        hash40,
        phx::*,
        app::{lua_bind::*, sv_animcmd::*, *},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*
};

#[acmd_script( agent = "eflame", scripts = [ "game_specials", "game_specialairs", "game_specialsflick", "game_specialairsflick" ], category = ACMD_GAME, low_priority )]
unsafe fn eflame_specials(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD) {
        if macros::is_excute(fighter) {
            ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, *WEAPON_EFLAME_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 10.0, 10.0, false, false, 0.0, false, true, false);
        }
    }
    if MotionModule::is_changing(fighter.module_accessor) {
        if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
        }
    }
    if macros::is_excute(fighter) {
        WorkModule::set_int64(fighter.module_accessor, hash40("special_s") as i64, *FIGHTER_EFLAME_INSTANCE_WORK_ID_INT_ESWORD_INHERIT_OPEN_MOTION_KIND);
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        ArticleModule::shoot(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_ESWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        // No hitbox
    }
    // frame 16 clears the old hitbox
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
}

#[acmd_script( agent = "eflame_esword", scripts = [ "game_flyl", "game_flyr" ], category = ACMD_GAME, low_priority )]
unsafe fn eflame_esword_fly(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 0.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.45);
    }
    // there's usually a hitbox here but uh no more
}

#[acmd_script( agent = "eflame_esword", scripts = [ "game_flyflickl", "game_flyflickr" ], category = ACMD_GAME, low_priority )]
unsafe fn eflame_esword_flyflick(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 0.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.3);
    }
    // there's usually a hitbox here but uh no more
}

pub fn install() {
    install_acmd_scripts!(
        eflame_specials,
        eflame_esword_fly,
        eflame_esword_flyflick
    );
}
