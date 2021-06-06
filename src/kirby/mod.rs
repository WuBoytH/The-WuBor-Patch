use smash::phx::Hash40;
use smash::hash40;
// use smash::phx::Vector3f;
use smash::phx::Vector2f;
use smash::lua2cpp::{L2CAgentBase, L2CFighterCommon};
use smash::app::*;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash_script::*;
use smashline::*;
use crate::ganon::*;
use crate::commonfuncs::*;
use crate::system::IS_FUNNY;

pub static mut SLIDE_BOUNCE : [bool; 8] = [false; 8];

#[fighter_frame( agent = FIGHTER_KIND_KIRBY )]
fn kirby_frame(fighter: &mut L2CFighterCommon) {
    unsafe {

        // Incin Darkest Lariat Jump Cancel

        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND
        && StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_KIRBY_STATUS_KIND_GAOGAEN_SPECIAL_N
        && MotionModule::frame(fighter.module_accessor) > 19.0
        && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
        && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
        }

        // Down Tilt Bounce

        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_lw3") {
            if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                macros::EFFECT(fighter, Hash40::new_raw(0x0ab6e0ea34), Hash40::new("top"), 0, 3, 10, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
                SLIDE_BOUNCE[entry_id(fighter.module_accessor)] = true;
                StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), true);
                GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_JUMP);
                macros::SET_SPEED_EX(fighter, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            }
        }
        if SLIDE_BOUNCE[entry_id(fighter.module_accessor)]
        && MotionModule::motion_kind(fighter.module_accessor) != hash40("jump_b") {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("jump_b"), 22.0, 1.0, false, 0.0, false, false);
            macros::SET_SPEED_EX(fighter, -1.0, 1.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        else if (MotionModule::motion_kind(fighter.module_accessor) == hash40("jump_b")
        && MotionModule::frame(fighter.module_accessor) >= 30.0)
        || MotionModule::motion_kind(fighter.module_accessor) != hash40("jump_b") {
            SLIDE_BOUNCE[entry_id(fighter.module_accessor)] = false;
        }

        // Ganon Teleport

        if entry_id(fighter.module_accessor) < 8 {
            if TELEPORT[entry_id(fighter.module_accessor)] == 1 || TELEPORT[entry_id(fighter.module_accessor)] == 5 {
                let dir = get_command_stick_direction(fighter.module_accessor, false);
                if dir == 5 || dir == 2 || dir == 8 {
                    TELE_X[entry_id(fighter.module_accessor)] = 0.0;
                }
                else if dir == 3 || dir == 9 {
                    TELE_X[entry_id(fighter.module_accessor)] = 35.0;
                }
                else if dir == 6 {
                    TELE_X[entry_id(fighter.module_accessor)] = 40.0;
                }
                else if dir == 1 || dir == 7 {
                    TELE_X[entry_id(fighter.module_accessor)] = -35.0;
                }
                else if dir == 4 {
                    TELE_X[entry_id(fighter.module_accessor)] = -40.0;
                }
                if dir == 5
                || dir == 4
                || dir == 6
                || (dir == 2 && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND)
                || (dir == 1 && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND)
                || (dir == 3 && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND) {
                    TELE_Y[entry_id(fighter.module_accessor)] = 0.0;
                }
                else if (dir == 1 && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR)
                || (dir == 3 && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR) {
                    TELE_Y[entry_id(fighter.module_accessor)] = -30.0;
                }
                else if dir == 2
                && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
                    TELE_Y[entry_id(fighter.module_accessor)] = -40.0;
                }
                else if dir == 7
                || dir == 9 {
                    TELE_Y[entry_id(fighter.module_accessor)] = 30.0;
                }
                else if dir == 8 {
                    TELE_Y[entry_id(fighter.module_accessor)] = 40.0;
                }
            }
            if TELEPORT[entry_id(fighter.module_accessor)] == 3 || TELEPORT[entry_id(fighter.module_accessor)] == 7 {
                macros::EFFECT(fighter, Hash40::new_raw(0x0b7a7552cf), Hash40::new("top"), 0, 8.0, -2.0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
                if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
                    if TELE_Y[entry_id(fighter.module_accessor)] != 0.0 {
                        StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), true);
                    }
                    else {
                        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                    }
                }
                PostureModule::add_pos_2d(fighter.module_accessor, &Vector2f {x: TELE_X[entry_id(fighter.module_accessor)], y: TELE_Y[entry_id(fighter.module_accessor)]});
                if TELE_X[entry_id(fighter.module_accessor)] == 0.0 && TELE_Y[entry_id(fighter.module_accessor)] == 0.0 {
                    macros::EFFECT(fighter, Hash40::new_raw(0x0b7a7552cf), Hash40::new("top"), 0, 8.0, 38.0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
                }
                else {
                    macros::EFFECT(fighter, Hash40::new_raw(0x0b7a7552cf), Hash40::new("top"), 0, 8.0, -2.0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
                }
                TELEPORT[entry_id(fighter.module_accessor)] += 1;
            }

            if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_CLIFF
            || StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND
            || IS_FUNNY[entry_id(fighter.module_accessor)] {
                CAN_TELEPORT[entry_id(fighter.module_accessor)] = true;
            }

            if TELE_STOP[entry_id(fighter.module_accessor)] {
                KineticModule::unable_energy_all(fighter.module_accessor);
            }
        }
    }
}

#[acmd_script( agent = "kirby", script = "game_attacklw3", category = ACMD_GAME, low_priority )]
unsafe fn kirby_dtilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    sv_animcmd::frame(lua_state, 7.0);
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 48, 50, 0, 72, 4.0, 0.0, 2.0, 7.0, Some(0.0), Some(2.0), Some(3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    sv_animcmd::frame(lua_state, 20.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 60, 50, 0, 72, 4.0, 0.0, 2.0, 7.0, Some(0.0), Some(2.0), Some(3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    sv_animcmd::wait(lua_state, 7.0);
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, true);
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "kirby", script = "effect_attacklw3", category = ACMD_EFFECT, low_priority )]
unsafe fn kirby_dtilteff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    sv_animcmd::frame(lua_state, 6.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 2, 2, 0, 0, 0, 0.8, true);
	    macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 2, 2.5, 2, 0, 0, 0, 0.75, true, *EF_FLIP_YZ);
    }
    sv_animcmd::frame(lua_state, 7.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.55, 0, 0, 0, 0, 0, 0, false);
    }
    sv_animcmd::frame(lua_state, 9.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_sliding_smoke"), Hash40::new("top"), -1, 0, 8, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
	    macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 2, 3, 0, 0, 0, 0.8, true);
    }
}

#[acmd_script( agent = "kirby", script = "game_attackairhi", category = ACMD_GAME, low_priority )]
unsafe fn kirby_uair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    sv_animcmd::frame(lua_state, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.375);
    sv_animcmd::frame(lua_state, 9.0);
    macros::FT_MOTION_RATE(fighter, 1);
    sv_animcmd::frame(lua_state, 10.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(fighter, 0, 0, Hash40::new("footr"), 10.0, 70, 115, 0, 20, 4.0, 0.0, -5.6, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("footr"), 10.0, 70, 115, 0, 20, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    sv_animcmd::wait(lua_state, 6.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::wait(lua_state, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "kirby", script = "game_specialsstart", category = ACMD_GAME, low_priority )]
unsafe fn kirby_sspecialstart(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_HAMMER, false, 0);
    }
    sv_animcmd::frame(lua_state, 3.0);
    macros::FT_MOTION_RATE(fighter, 0.5);
    sv_animcmd::frame(lua_state, 15.0);
    macros::FT_MOTION_RATE(fighter, 1);
}

#[acmd_script( agent = "kirby", script = "game_specialairsstart", category = ACMD_GAME, low_priority )]
unsafe fn kirby_sspecialstartair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_HAMMER, false, 0);
    }
    sv_animcmd::frame(lua_state, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.667);
    sv_animcmd::frame(lua_state, 25.0);
    macros::FT_MOTION_RATE(fighter, 1);
}

#[acmd_script( agent = "kirby", scripts = [ "game_specialhi", "game_specialairhi" ], category = ACMD_GAME, low_priority )]
unsafe fn kirby_uspecial(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, false, 0);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, Hash40::new("special_hi"), false, 0.0);
    }
    sv_animcmd::frame(lua_state, 5.0);
    macros::FT_MOTION_RATE(fighter, 0.5);
    sv_animcmd::frame(lua_state, 19.0);
    macros::FT_MOTION_RATE(fighter, 1);
}

#[acmd_script( agent = "kirby", scripts = ["effect_ganonspecialn", "effect_ganonspecialairn"], category = ACMD_EFFECT, low_priority )]
unsafe fn kirby_ganonspecialeff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new_raw(0x14020f4ff6), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    sv_animcmd::frame(lua_state, 30.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new_raw(0x0b7a7552cf), Hash40::new("top"), 0, 8, -2, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        macros::FLASH(fighter, 1, 0, 1, 1.0);
    }
    sv_animcmd::frame(lua_state, 34.0);
    if macros::is_excute(fighter) {
        VisibilityModule::set_whole(fighter.module_accessor, false);
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        ItemModule::set_attach_item_visibility(fighter.module_accessor, false, 0);
    }
    sv_animcmd::frame(lua_state, 60.0);
    if macros::is_excute(fighter) {
        VisibilityModule::set_whole(fighter.module_accessor, true);
        ItemModule::set_have_item_visibility(fighter.module_accessor, true, 0);
        ItemModule::set_attach_item_visibility(fighter.module_accessor, true, 0);
    }
    sv_animcmd::frame(lua_state, 64.0);
    if macros::is_excute(fighter) {
        macros::COL_NORMAL(fighter);
    }
}

#[acmd_script( agent = "kirby", scripts = ["sound_ganonspecialn", "sound_ganonspecialairn"], category = ACMD_SOUND, low_priority )]
unsafe fn kirby_ganonspecialsnd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    sv_animcmd::frame(lua_state, 12.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_ganon_special_n01"));
        macros::PLAY_SE(fighter, Hash40::new("vc_ganon_appeal_h01"));
    }
    sv_animcmd::frame(lua_state, 30.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_ganon_appear01"));
    }
    sv_animcmd::frame(lua_state, 50.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_ganon_appear01"));
    }
    sv_animcmd::frame(lua_state, 60.0);
    if macros::is_excute(fighter) {
        VisibilityModule::set_model_visible(fighter.module_accessor, true);
        ItemModule::set_have_item_visibility(fighter.module_accessor, true, 0);
        ItemModule::set_attach_item_visibility(fighter.module_accessor, true, 0);
    }
    sv_animcmd::frame(lua_state, 64.0);
    if macros::is_excute(fighter) {
        macros::COL_NORMAL(fighter);
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        kirby_frame
    );
    smashline::install_acmd_scripts!(
        kirby_dtilt,
        kirby_dtilteff,
        kirby_uair,
        kirby_sspecialstart,
        kirby_sspecialstartair,
        kirby_uspecial,
        kirby_ganonspecialeff,
        kirby_ganonspecialsnd
    );
}