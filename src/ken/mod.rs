use smash::phx::Hash40;
use smash::lua2cpp::{L2CFighterCommon, L2CAgentBase};
// use smash::app::sv_animcmd;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash_script::*;
// use smash::phx::Vector3f;
// use smash::phx::Vector2f;
// use smash::app::lua_bind::EffectModule;
// use crate::IS_FUNNY;
use crate::commonfuncs::*;

pub static mut QUICK_STEP_STATE : [i32; 8] = [0; 8];
/*
State list:
0 = Can Quick Step
2 = Cannot Quick Step
1 = Used to show you're in the first 22 frames of Quick Step.
*/
static mut VT1_CANCEL : [bool; 8] = [false; 8];
static mut EX_FLASH : [bool; 8] = [false; 8];

#[fighter_frame( agent = FIGHTER_KIND_KEN )]
unsafe fn ken_frame(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    
    if get_player_number(boma) < 8 {

        // Reset Vars

        if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_REBIRTH || smash::app::sv_information::is_ready_go() == false {
            QUICK_STEP_STATE[get_player_number(boma)] = 0;
            VT1_CANCEL[get_player_number(boma)] = false;
            EX_FLASH[get_player_number(boma)] = false;
        }

        // V Skill 1
        if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ATTACK
        || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ATTACK_S3
        || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ATTACK_HI3
        || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ATTACK_LW3
        || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ATTACK_S4
        || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ATTACK_LW4
        || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ATTACK_HI4
        || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ATTACK_DASH
        || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ATTACK_AIR {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
            || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
                VT1_CANCEL[get_player_number(boma)] = true;
            }
            else {
                VT1_CANCEL[get_player_number(boma)] = false;
            }
        }
        else {
            VT1_CANCEL[get_player_number(boma)] = true;
        }

        if MotionModule::motion_kind(boma) == smash::hash40("special_lw_step_f")
        && MotionModule::frame(boma) == 1.0 {
            MotionModule::change_motion(boma, Hash40::new("run"), 0.0, 1.0, true, 0.0, false, false);
        }

        if MotionModule::motion_kind(boma) == smash::hash40("run")
        && QUICK_STEP_STATE[get_player_number(boma)] == 1 {
            if MotionModule::frame(boma) >= 26.0 && MotionModule::frame(boma) <= 27.0
            && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                MotionModule::change_motion(boma, Hash40::new("attack_s3_s_w"), 0.0, 1.0, false, 0.0, false, false);
            }
            if MotionModule::frame(boma) >= 36.0 {
                CancelModule::enable_cancel(boma);
            }
        }

        if MotionModule::motion_kind(boma) == smash::hash40("attack_s3_s_w") {
            if MotionModule::frame(boma) > 26.0 {
                CancelModule::enable_cancel(boma);
            }
        }

        if (StatusModule::status_kind(boma) != *FIGHTER_STATUS_KIND_RUN
        && StatusModule::status_kind(boma) != *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F)
        && MotionModule::motion_kind(boma) != smash::hash40("attack_s3_s_w")
        && QUICK_STEP_STATE[get_player_number(boma)] == 1 {
            QUICK_STEP_STATE[get_player_number(boma)] = 2;
        }

        if ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW != 0
        && VT1_CANCEL[get_player_number(boma)]
        && QUICK_STEP_STATE[get_player_number(boma)] == 0 {
            if MotionModule::motion_kind(boma) == smash::hash40("attack_air_b") {
                PostureModule::reverse_lr(boma);
            }
            fighter.change_status(FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F.into(), false.into());
            if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
                QUICK_STEP_STATE[get_player_number(boma)] = 1;
            }
            else {
                QUICK_STEP_STATE[get_player_number(boma)] = 2;
            }
        }

        if QUICK_STEP_STATE[get_player_number(boma)] == 2
        && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
            QUICK_STEP_STATE[get_player_number(boma)] = 0;
        }

        // V Shift

        if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_GUARD {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                let stick_x = ControlModule::get_stick_x(boma);
                if (stick_x < -0.5 && PostureModule::lr(boma) == 1.0)
                || (stick_x > 0.5 && PostureModule::lr(boma) == -1.0) {
                    fighter.change_status(FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_B.into(), false.into());
                }
            }
        }

        // EX Flash

        // if EX_FLASH[get_player_number(boma)] {
        //     if SEC_SEN_STATE[get_player_number(boma)] {
        //         if FLASH_TIMER[get_player_number(boma)] < 0 {
        //             FLASH_TIMER[get_player_number(boma)] = 8;
        //         }
        //         if FLASH_TIMER[get_player_number(boma)] <= 4 {
        //             macros::COL_NORMAL(fighter);
        //             FLASH_TIMER[get_player_number(boma)] -= 1;
        //         }
        //         if FLASH_TIMER[get_player_number(boma)] > 4 {
        //             macros::FLASH(fighter, 0, 0.55, 1, 1.75);
        //             FLASH_TIMER[get_player_number(boma)] -= 1;
        //         }
        //     }
        //     else {
        //         if FLASH_TIMER[get_player_number(boma)] < 0 {
        //             FLASH_TIMER[get_player_number(boma)] = 12;
        //         }
        //         else if FLASH_TIMER[get_player_number(boma)] == 0 {
        //             macros::COL_NORMAL(fighter);
        //             EX_FLASH[get_player_number(boma)] = false;
        //             FLASH_TIMER[get_player_number(boma)] -= 1;
        //         }
        //         else {
        //             macros::FLASH(fighter, 1, 1, 0.0, 1.5);
        //             FLASH_TIMER[get_player_number(boma)] -= 1;
        //         }
        //     }
        // }
    }
}

// Motion Rate the Run Animation

#[script( agent = "ken", script = "game_run", category = ACMD_GAME )]
unsafe fn ken_run(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if QUICK_STEP_STATE[get_player_number(boma)] == 1 {
        macros::FT_MOTION_RATE(fighter, 0.6);
    }
}

// Make Quick Step (non-prox light f tilt) have step kick properties

// #[script( agent = "ken", script = "game_attacks3w", category = ACMD_GAME )]
// unsafe fn ken_ftiltwnp(fighter: &mut L2CAgentBase) {
//     let lua_state = fighter.lua_state_agent;
//     let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
//     if macros::is_excute(fighter) {
//         WorkModule::on_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
//     }
//     sv_animcmd::frame(lua_state, 1.0);
//     macros::FT_MOTION_RATE(fighter, 1.0);
//     sv_animcmd::frame(lua_state, 4.0);
//     if macros::is_excute(fighter) {
//         macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_XLU);
//         macros::HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_XLU);
//     }
//     sv_animcmd::frame(lua_state, 8.0);
//     if macros::is_excute(fighter) {
//         macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.8, 55, 46, 0, 66, 3.8, 0.0, 11.0, 6.5, Some(0.0), Some(11.0), Some(5.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.5, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
//         macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 6.8, 72, 46, 0, 66, 3.8, 0.0, 11.0, 14.0, Some(0.0), Some(11.0), Some(5.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.5, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
//     }
//     sv_animcmd::frame(lua_state, 12.0);
//     if macros::is_excute(fighter) {
//         AttackModule::clear_all(boma);
//         WorkModule::off_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
//         HitModule::set_status_all(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
//     }
//     macros::FT_MOTION_RATE(fighter, 0.8);
// }

pub fn install() {
    smash_script::replace_fighter_frames!(
        ken_frame
    );
    smash_script::replace_scripts!(
        ken_run
    //     ken_ftiltwnp
    );
}