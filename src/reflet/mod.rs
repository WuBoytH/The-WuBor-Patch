// use smash::phx::Hash40;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CAgentBase, L2CFighterCommon};
use smash::app::*;
use smash::app::sv_animcmd::*;
use smash_script::*;
use smashline::*;
// use crate::commonfuncs::*;

#[fighter_frame( agent = FIGHTER_KIND_REFLET )]
fn reflet_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_ENTRY {
            WorkModule::set_int(fighter.module_accessor, 8, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT);
        }

        if smashball::is_training_mode(){
            if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) {
                WorkModule::set_int(fighter.module_accessor, 8, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT);
            }
        }

        if (MotionModule::motion_kind(fighter.module_accessor) == hash40("special_hi")
        || MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_hi"))
        && MotionModule::frame(fighter.module_accessor) >= 12.0 && MotionModule::frame(fighter.module_accessor) < 46.0 {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
            || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_TRY_2ND);
            }
        }
    }
}

#[acmd_script( agent = "reflet", scripts = ["game_specialhi", "game_specialairhi"], category = ACMD_GAME, low_priority )]
unsafe fn reflet_uspecial1(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_REFLET_GENERATE_ARTICLE_ELWIND, false, 0);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_JUMP);
    }
    frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        reflet_frame
    );
    smashline::install_acmd_scripts!(
        reflet_uspecial1
    );
}