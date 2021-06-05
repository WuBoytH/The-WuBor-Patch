// use smash::phx::Hash40;
use smash::lua2cpp::L2CFighterCommon;
use smash::app::*;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash_script::*;
use smashline::*;
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
fn wiifit_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        if entry_id(boma) < 8 {

            // Reset Vars

            if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_REBIRTH || sv_information::is_ready_go() == false {
                DRAGON_INSTALL[entry_id(boma)] = false;
                DI_FLASH[entry_id(boma)] = false;
                CAN_DRAGON_INSTALL[entry_id(boma)] = true;
            }

            // DRAGON INSTALL

            if IS_FUNNY[entry_id(boma)] {
                if MotionModule::motion_kind(boma) == smash::hash40("special_lw_success_r")
                || MotionModule::motion_kind(boma) == smash::hash40("special_air_lw_success_r")
                || MotionModule::motion_kind(boma) == smash::hash40("special_lw_success_l")
                || MotionModule::motion_kind(boma) == smash::hash40("special_air_lw_success_l") {
                    if MotionModule::frame(boma) == 1.0 {
                        DRAGON_INSTALL[entry_id(boma)] = true;
                        CAN_DRAGON_INSTALL[entry_id(boma)] = false;
                        DI_FLASH[entry_id(boma)] = true;
                        FLASH_TIMER[entry_id(boma)] = 0;
                        *NUS3AUDIO_HASH = 0x2faf78f2ffu64;
                        music_function_replace(MUSIC_PARAM1, MUSIC_PARAM2, 199195422212, NUS3AUDIO_HASH, 0);
                    }
                }
            }

            if WorkModule::get_int(boma, *FIGHTER_WIIFIT_INSTANCE_WORK_ID_INT_SPECIAL_LW_WAZA_EFFECTIVE_FRAME) == 1 {
                DI_FLASH[entry_id(boma)] = false;
            }

            if DRAGON_INSTALL[entry_id(boma)]
            && DI_FLASH[entry_id(boma)] == false
            && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_SAVING_DAMAGE.into(), false.into());
                MotionModule::set_rate(boma, 0.05);
                DRAGON_INSTALL[entry_id(boma)] = false;
            }

            // Dragon Install Flash

            if DI_FLASH[entry_id(boma)] {
                if FLASH_TIMER[entry_id(boma)] < 0 {
                    FLASH_TIMER[entry_id(boma)] = 8;
                }
                if FLASH_TIMER[entry_id(boma)] <= 4 {
                    macros::COL_NORMAL(fighter);
                    FLASH_TIMER[entry_id(boma)] -= 1;
                }
                if FLASH_TIMER[entry_id(boma)] > 4 {
                    macros::FLASH(fighter, 1, 0, 0, 1.25);
                    FLASH_TIMER[entry_id(boma)] -= 1;
                }
            }
        }
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        wiifit_frame
    );
}