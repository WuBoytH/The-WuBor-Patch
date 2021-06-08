// use smash::phx::Hash40;
use smash::lua2cpp::{/*L2CAgentBase, */L2CFighterCommon};
// use smash::app::*;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
// use smash::lib::L2CValue;
// use smash_script::*;
use smashline::*;
use crate::system::IS_FUNNY;
// use crate::globals::*;
use crate::commonfuncs::*;

static mut UP_SPECIAL_CANCEL : [bool; 8] = [false; 8];

#[fighter_frame( agent = FIGHTER_KIND_EDGE )]
fn edge_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        
        // Cancel Frames

        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            UP_SPECIAL_CANCEL[entry_id(fighter.module_accessor)] = false;
        }
        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH {
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
            || IS_FUNNY[entry_id(fighter.module_accessor)] {
                UP_SPECIAL_CANCEL[entry_id(fighter.module_accessor)] = true;
            }
        }
        if (StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH
        || StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_END
        || StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_LANDING)
        && UP_SPECIAL_CANCEL[entry_id(fighter.module_accessor)] == true {
            CancelModule::enable_cancel(fighter.module_accessor);
        }
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        edge_frame
    );
}