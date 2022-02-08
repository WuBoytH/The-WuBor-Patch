use {
    smash::{
        lua2cpp::L2CAgentBase,
        hash40,
        app::{lua_bind::*, sv_animcmd::*, *},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*,
    wubor_utils::vars::*
};

#[acmd_script( agent = "diddy", scripts = ["game_speciallw", "game_specialairlw"], category = ACMD_GAME, low_priority )]
unsafe fn diddy_speciallw(fighter: &mut L2CAgentBase) {
    let rng : i32;
    let randitem : i32;
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FUNNY) {
        rng = sv_math::rand(hash40("fighter"), 100);
        match rng {
            21..=45 => randitem = *ITEM_KIND_SENSORBOMB,
            46..=60 => randitem = *ITEM_KIND_UNIRA,
            61..=74 => randitem = *ITEM_KIND_HAMMERHEAD,
            75..=86 => randitem = *ITEM_KIND_BUMPER,
            87..=96 => randitem = *ITEM_KIND_POWBLOCK,
            97..=98 => randitem = *ITEM_KIND_SMASHBOMB,
            99..=100 => randitem = *ITEM_KIND_SMASHBALL,
            _ => randitem = *ITEM_KIND_BOMBHEI,
        }
    }
    else {
        rng = 101;
        randitem = 0;
    }
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        if rng == 101 {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_DIDDY_GENERATE_ARTICLE_ITEM_BANANA, false, 0);
        }
        else {
            ItemModule::have_item(fighter.module_accessor, ItemKind(randitem), 0, 0, false, false);
        }
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        if rng == 101 {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DIDDY_STATUS_SPECIAL_LW_FLAG_ITEM_THROW);
        }
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        if rng != 101 {
            let angle : f32;
            let power : f32;
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                angle = 40.0;
                power = 2.0;
            }
            else {
                angle = 75.0;
                power = 2.2;
            }
            ItemModule::throw_item(fighter.module_accessor, angle, power, 1.0, 0, true, 0.0);
        }
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        if rng == 101 {
            ArticleModule::shoot(fighter.module_accessor, *FIGHTER_DIDDY_GENERATE_ARTICLE_ITEM_BANANA, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        }
    }
}

pub fn install() {
    install_acmd_scripts!(
        diddy_speciallw
    );
}