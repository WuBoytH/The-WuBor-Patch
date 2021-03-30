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

pub static mut DRAGON_INSTALL : [bool; 8] = [true; 8];
pub static mut CAN_DRAGON_INSTALL : [bool; 8] = [false; 8];
static mut DI_FLASH : [bool; 8] = [false; 8];
static mut FLASH_TIMER : [i32; 8] = [0; 8];

#[fighter_frame( agent = FIGHTER_KIND_WIIFIT )]
unsafe fn wiifit_frame(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if entry_id < 8 {

        // Reset Vars

        if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_REBIRTH || smash::app::sv_information::is_ready_go() == false {
            DRAGON_INSTALL[entry_id] = false;
            DI_FLASH[entry_id] = false;
            CAN_DRAGON_INSTALL[entry_id] = true;
        }

        // DRAGON INSTALL

        if IS_FUNNY[entry_id] {
            if MotionModule::motion_kind(boma) == smash::hash40("special_lw_success_r")
            || MotionModule::motion_kind(boma) == smash::hash40("special_air_lw_success_r")
            || MotionModule::motion_kind(boma) == smash::hash40("special_lw_success_l")
            || MotionModule::motion_kind(boma) == smash::hash40("special_air_lw_success_l") {
                if MotionModule::frame(boma) == 1.0 {
                    DRAGON_INSTALL[entry_id] = true;
                    CAN_DRAGON_INSTALL[entry_id] = false;
                    DI_FLASH[entry_id] = true;
                    FLASH_TIMER[entry_id] = 0;
                    music_function_replace(MUSIC_PARAM1, MUSIC_PARAM2, 199195422212, NUS3AUDIO_HASH, 0);
                }
            }
        }

        if WorkModule::get_int(boma, *FIGHTER_WIIFIT_INSTANCE_WORK_ID_INT_SPECIAL_LW_WAZA_EFFECTIVE_FRAME) == 1 {
            DI_FLASH[entry_id] = false;
        }

        if DRAGON_INSTALL[entry_id]
        && DI_FLASH[entry_id] == false
        && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_SAVING_DAMAGE.into(), false.into());
            DRAGON_INSTALL[entry_id] = false;
        }

        // Dragon Install Flash

        if DI_FLASH[entry_id] {
            if FLASH_TIMER[entry_id] < 0 {
                FLASH_TIMER[entry_id] = 8;
            }
            if FLASH_TIMER[entry_id] <= 4 {
                macros::COL_NORMAL(fighter);
                FLASH_TIMER[entry_id] -= 1;
            }
            if FLASH_TIMER[entry_id] > 4 {
                macros::FLASH(fighter, 1, 0, 0, 1.25);
                FLASH_TIMER[entry_id] -= 1;
            }
        }
    }
}

pub fn install() {
    smash_script::replace_fighter_frames!(
        wiifit_frame
    );
}