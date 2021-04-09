use smash::phx::Hash40;
use smash::lua2cpp::L2CFighterCommon;
// use smash::app::sv_animcmd;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash_script::*;
// use smash::phx::Vector3f;
// use smash::phx::Vector2f;
// use smash::app::lua_bind::EffectModule;
// use crate::IS_FUNNY;
use crate::commonfuncs::*;

static mut QUICK_STEP_STATE : [i32; 8] = [0; 8];
static mut CANCEL : [bool; 8] = [false; 8];
static mut EX_FLASH : [bool; 8] = [false; 8];

#[fighter_frame( agent = FIGHTER_KIND_KEN )]
unsafe fn ken_frame(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    
    if get_player_number(boma) < 8 {

        // Reset Vars

        if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_REBIRTH || smash::app::sv_information::is_ready_go() == false {
            QUICK_STEP_STATE[get_player_number(boma)] = 0;
            CANCEL[get_player_number(boma)] = false;
            EX_FLASH[get_player_number(boma)] = false;
        }

        // V Skill 1
        if (StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ATTACK
        || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ATTACK_S3
        || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ATTACK_HI3
        || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ATTACK_LW3
        || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ATTACK_S4
        || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ATTACK_LW4
        || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ATTACK_HI4
        || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ATTACK_DASH
        || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ATTACK_AIR)
        && (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
        || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD)) {
            CANCEL[get_player_number(boma)] = true;
        }
        else {
            CANCEL[get_player_number(boma)] = false;
        }

        if ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW != 0
        && CANCEL[get_player_number(boma)] {
            if MotionModule::motion_kind(boma) == smash::hash40("attack_air_b") {
                PostureModule::reverse_lr(boma);
            }
            if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_RUN.into(), false.into());
                QUICK_STEP_STATE[get_player_number(boma)] = 2;
            }
            else {
                fighter.change_status(FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F.into(), false.into());
            }
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

pub fn install() {
    smash_script::replace_fighter_frames!(
        ken_frame
    );
}