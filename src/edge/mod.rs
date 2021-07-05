use smash::phx::Hash40;
use smash::lua2cpp::{L2CAgentBase, L2CFighterCommon};
// use smash::app::*;
use smash::app::sv_animcmd::*;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
// use smash::lib::L2CValue;
use smash_script::*;
use smashline::*;
use crate::system::IS_FUNNY;
// use crate::globals::*;
use crate::commonfuncs::*;

// static mut CAN_WING : [bool; 8] = [true; 8];
// pub static mut ONE_WING : [f32; 8] = [-1.0; 8];
static mut UP_SPECIAL_CANCEL : [bool; 8] = [false; 8];

#[fighter_frame( agent = FIGHTER_KIND_EDGE )]
fn edge_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        
        // Activate THE WING

        // if sv_information::is_ready_go() == false || StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_DEAD {
        //     CAN_WING[entry_id(fighter.module_accessor)] = true;
        //     ONE_WING[entry_id(fighter.module_accessor)] = -1.0;
        // }

        // if ONE_WING[entry_id(fighter.module_accessor)] >= 0.0 {
        //     if ONE_WING[entry_id(fighter.module_accessor)] == 0.0 {
        //         WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLOAT_ONE_WINGED_ACTIVATE_POINT);
        //     }
        //     ONE_WING[entry_id(fighter.module_accessor)] -= 1.0;
        // }

        // if CAN_WING[entry_id(fighter.module_accessor)] {
        //     if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
        //     && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        //         ONE_WING[entry_id(fighter.module_accessor)] = 900.0;
        //         WorkModule::set_float(fighter.module_accessor, 999.0, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLOAT_ONE_WINGED_ACTIVATE_POINT);
        //         // WorkModule::set_flag(fighter.module_accessor, true, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_ACTIVATED);
        //         // WorkModule::set_flag(fighter.module_accessor, true, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_JUMP_PLUS);
        //         // WorkModule::set_int(fighter.module_accessor, *FIGHTER_EDGE_ONE_WINGED_HAIR_ON, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ONE_WINGED_HAIR_STATE);
        //         WorkModule::set_flag(fighter.module_accessor, true, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_ACTIVATED);
        //         CAN_WING[entry_id(fighter.module_accessor)] = false;
        //     }
        // }

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

#[acmd_script( agent = "edge", script = "game_attackairb", category = ACMD_GAME, low_priority )]
unsafe fn edge_bair(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.8);
    frame(fighter.lua_state_agent, 6.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.5, 42, 91, 0, 35, 4.8, 0.0, 12.5, -6.0, Some(0.0), Some(12.5), Some(-11.3), 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 14.5, 42, 76, 0, 47, 6.0, 0.0, 13.0, -16.5, Some(0.0), Some(13.0), Some(-22.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 11.5, 42, 89, 0, 39, 3.7, 0.0, 12.5, -28.0, Some(0.0), Some(12.5), Some(-30.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 47.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        edge_frame
    );
    smashline::install_acmd_scripts!(
        edge_bair
    );
}