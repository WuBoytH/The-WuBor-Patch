// use smash::phx::Hash40;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CAgentBase, L2CFighterCommon};
use smash::app::*;
use smash_script::*;
use smashline::*;
// use crate::IS_FUNNY;
// use crate::commonfuncs::*;

#[fighter_frame( agent = FIGHTER_KIND_REFLET )]
fn reflet_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;
        let boma = sv_system::battle_object_module_accessor(lua_state);

        if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ENTRY {
            WorkModule::set_int(boma, 8, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT);
        }

        if smashball::is_training_mode(){
            if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_APPEAL_HI) {
                WorkModule::set_int(boma, 8, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT);
            }
        }

        if (MotionModule::motion_kind(boma) == hash40("special_hi")
        || MotionModule::motion_kind(boma) == hash40("special_air_hi"))
        && MotionModule::frame(boma) >= 12.0 && MotionModule::frame(boma) < 46.0 {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL)
            || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
                WorkModule::on_flag(boma, *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_TRY_2ND);
            }
        }
    }
}

#[acmd_script( agent = "reflet", scripts = ["game_specialhi", "game_specialairhi"], category = ACMD_GAME, low_priority )]
unsafe fn reflet_uspecial1(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state, 8.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_REFLET_GENERATE_ARTICLE_ELWIND, false, 0);
        WorkModule::on_flag(boma, *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_JUMP);
    }
    sv_animcmd::frame(lua_state, 28.0);
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