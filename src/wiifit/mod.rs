// use smash::phx::Hash40;
use smash::lua2cpp::L2CFighterCommon;
// use smash::app::sv_animcmd;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash_script::*;
// use smash::phx::Vector3f;
// use smash::phx::Vector2f;
// use smash::app::lua_bind::EffectModule;
use crate::{IS_FUNNY, MUSIC_PARAM1, MUSIC_PARAM2, NUS3AUDIO_HASH};
use crate::music_function_replace;
use crate::commonfuncs::*;

static mut DRAGON_INSTALL : [bool; 8] = [true; 8];
pub static mut CAN_DRAGON_INSTALL : [bool; 8] = [false; 8];
static mut DI_FLASH : [bool; 8] = [false; 8];
static mut FLASH_TIMER : [i32; 8] = [0; 8];

#[fighter_frame( agent = FIGHTER_KIND_WIIFIT )]
unsafe fn wiifit_frame(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    if get_player_number(boma) < 8 {

        // Reset Vars

        if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_REBIRTH || smash::app::sv_information::is_ready_go() == false {
            DRAGON_INSTALL[get_player_number(boma)] = false;
            DI_FLASH[get_player_number(boma)] = false;
            CAN_DRAGON_INSTALL[get_player_number(boma)] = true;
        }

        // DRAGON INSTALL

        if IS_FUNNY[get_player_number(boma)] {
            if MotionModule::motion_kind(boma) == smash::hash40("special_lw_success_r")
            || MotionModule::motion_kind(boma) == smash::hash40("special_air_lw_success_r")
            || MotionModule::motion_kind(boma) == smash::hash40("special_lw_success_l")
            || MotionModule::motion_kind(boma) == smash::hash40("special_air_lw_success_l") {
                if MotionModule::frame(boma) == 1.0 {
                    DRAGON_INSTALL[get_player_number(boma)] = true;
                    CAN_DRAGON_INSTALL[get_player_number(boma)] = false;
                    DI_FLASH[get_player_number(boma)] = true;
                    FLASH_TIMER[get_player_number(boma)] = 0;
                    *NUS3AUDIO_HASH = 0x2faf78f2ffu64;
                    music_function_replace(MUSIC_PARAM1, MUSIC_PARAM2, 199195422212, NUS3AUDIO_HASH, 0);
                }
            }
        }

        if WorkModule::get_int(boma, *FIGHTER_WIIFIT_INSTANCE_WORK_ID_INT_SPECIAL_LW_WAZA_EFFECTIVE_FRAME) == 1 {
            DI_FLASH[get_player_number(boma)] = false;
        }

        if DRAGON_INSTALL[get_player_number(boma)]
        && DI_FLASH[get_player_number(boma)] == false
        && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_SAVING_DAMAGE.into(), false.into());
            MotionModule::set_rate(boma, 0.05);
            DRAGON_INSTALL[get_player_number(boma)] = false;
        }

        // Dragon Install Flash

        if DI_FLASH[get_player_number(boma)] {
            if FLASH_TIMER[get_player_number(boma)] < 0 {
                FLASH_TIMER[get_player_number(boma)] = 8;
            }
            if FLASH_TIMER[get_player_number(boma)] <= 4 {
                macros::COL_NORMAL(fighter);
                FLASH_TIMER[get_player_number(boma)] -= 1;
            }
            if FLASH_TIMER[get_player_number(boma)] > 4 {
                macros::FLASH(fighter, 1, 0, 0, 1.25);
                FLASH_TIMER[get_player_number(boma)] -= 1;
            }
        }
    }
}

pub fn install() {
    smash_script::replace_fighter_frames!(
        wiifit_frame
    );
}