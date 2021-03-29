use smash::phx::Hash40;
use smash::lua2cpp::L2CFighterCommon;
// use smash::app::sv_animcmd;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash_script::*;
use smash::phx::Vector3f;
use smash::phx::Vector2f;
use smash::app::lua_bind::EffectModule;
use crate::IS_FUNNY;

static mut DRAGON_INSTALL : [bool; 8] = [false; 8];
static mut DI_FLASH : [bool; 8] = [false; 8];
static mut FLASH_TIMER : [i32; 8] = [-1; 8];

#[fighter_frame( agent = FIGHTER_KIND_WIIFIT )]
unsafe fn wiifit_frame(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if entry_id < 8 {

        // Reset Vars

        if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_REBIRTH || smash::app::sv_information::is_ready_go() == false {
            DRAGON_INSTALL[entry_id] = false;
            DI_FLASH[entry_id] = false;
        }

        // DRAGON INSTALL

        if IS_FUNNY[entry_id] {
            if MotionModule::motion_kind(boma) == smash::hash40("speciallwsuccessr")
            || MotionModule::motion_kind(boma) == smash::hash40("specialairlwsuccessr")
            || MotionModule::motion_kind(boma) == smash::hash40("speciallwsuccessl")
            || MotionModule::motion_kind(boma) == smash::hash40("specialairlwsuccessl") {

            }
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
                macros::FLASH(fighter, 1, 0, 0, 1.75);
                FLASH_TIMER[entry_id] -= 1;
            }
        }
    }
}

pub fn install() {
    smash_script::replace_fighter_frames!(
        ryu_frame
    );
}