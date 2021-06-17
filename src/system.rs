//use smash::app::utility::get_kind;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
// use smash::lua2cpp::L2CAgentBase;
use smash::lua2cpp::{L2CFighterCommon/*, L2CFighterBase*/};
// use smash_script::*;
use smashline::*;
use crate::commonfuncs::*;
use smash::app::*;
// use smash::lib::L2CValue;

pub static mut _TIME_COUNTER: [i32; 8] = [0; 8];
pub static mut IS_FUNNY : [bool; 8] = [false; 8];
pub static mut IS_FGC : [bool; 8] = [false; 8];
pub static mut COUNTER_HIT_STATE : [i32; 8] = [0; 8];
pub static mut OPPONENT_BOMA : [u64; 8] = [0; 8];
pub static mut DAMAGE_TAKEN : [f32; 8] = [0.0; 8];
pub static mut DAMAGE_TAKEN_PREV : [f32; 8] = [0.0; 8];
pub static mut DMG_RATIO : [f32; 8] = [0.8; 8];
static mut INPUT_TIMER : [i32; 8] = [0; 8];
pub static mut QCF : [i32; 8] = [0; 8];
pub static mut QCB : [i32; 8] = [0; 8];

// #[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_DamageAir_Main)]
// pub unsafe fn damage_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
//     ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
//     call_original!(fighter)
// }

// Use this for general per-frame fighter-level hooks
#[fighter_frame_callback]
fn global_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {

        // The code to set up Funny Mode.

        if ItemModule::is_attach_item(fighter.module_accessor, ItemKind(*ITEM_KIND_USAGIHAT))
        && IS_FUNNY[entry_id(fighter.module_accessor)] == false {
            IS_FUNNY[entry_id(fighter.module_accessor)] = true;
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_RABBIT_CAP) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_RABBIT_CAP);
        }
        if (StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_DEAD || sv_information::is_ready_go() == false)
        && IS_FUNNY[entry_id(fighter.module_accessor)] {
            IS_FUNNY[entry_id(fighter.module_accessor)] = false;
        }

        if FighterUtil::is_hp_mode(fighter.module_accessor) {
            IS_FGC[entry_id(fighter.module_accessor)] = true;
        }
        else {
            IS_FGC[entry_id(fighter.module_accessor)] = false;
        }

        // Remove Special Fall in Funny Mode

        if IS_FUNNY[entry_id(fighter.module_accessor)] {
            if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_FALL_SPECIAL {
                StatusModule::change_status_request_from_script(fighter.module_accessor,*FIGHTER_STATUS_KIND_FALL_AERIAL,true);
            }
        }

        // Remove an OPPONENT_BOMA if the opponent is dead.

        if entry_id(fighter.module_accessor) < 8 {
            if OPPONENT_BOMA[entry_id(fighter.module_accessor)] != 0 {
                if StatusModule::status_kind(OPPONENT_BOMA[entry_id(fighter.module_accessor)] as *mut BattleObjectModuleAccessor) == *FIGHTER_STATUS_KIND_DEAD {
                    OPPONENT_BOMA[entry_id(fighter.module_accessor)] = 0;
                }
            }
        }
        
        // Platform Dropping while in shield.

        if (StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_GUARD
        || StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_GUARD_ON)
        && ControlModule::get_stick_y(fighter.module_accessor) < -0.8
        && GroundModule::is_passable_ground(fighter.module_accessor) {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_PASS, true);
        }
        
        // Command Inputs

        let dir = get_command_stick_direction(fighter.module_accessor, true);
        if INPUT_TIMER[entry_id(fighter.module_accessor)] <= 10 {
            INPUT_TIMER[entry_id(fighter.module_accessor)] += 1;
        }
        else {
            QCF[entry_id(fighter.module_accessor)] = 0;
            QCB[entry_id(fighter.module_accessor)] = 0;
        }

        // Quarter Circle Back

        if QCB[entry_id(fighter.module_accessor)] == 0 {
            if dir == 2 {
                QCB[entry_id(fighter.module_accessor)] = 1;
                INPUT_TIMER[entry_id(fighter.module_accessor)] = 0;
            }
        }
        else if QCB[entry_id(fighter.module_accessor)] == 1 {
            if dir == 1 {
                QCB[entry_id(fighter.module_accessor)] = 2;
            }
            else if dir != 4
            && dir != 1
            && dir != 2 {
                QCB[entry_id(fighter.module_accessor)] = 0;
            }
        }
        else if QCB[entry_id(fighter.module_accessor)] == 2 {
            if dir == 4 {
                QCB[entry_id(fighter.module_accessor)] = 3;
            }
            else if dir != 4
            && dir != 1
            && dir != 7 {
                QCB[entry_id(fighter.module_accessor)] = 0;
            }
        }
        else {
            if dir != 4
            && dir != 7 {
                QCB[entry_id(fighter.module_accessor)] = 0;
            }
        }

        // Quarter Circle Forward

        if QCF[entry_id(fighter.module_accessor)] == 0 {
            if dir == 2 {
                QCF[entry_id(fighter.module_accessor)] = 1;
                INPUT_TIMER[entry_id(fighter.module_accessor)] = 0;
            }
        }
        else if QCF[entry_id(fighter.module_accessor)] == 1 {
            if dir == 1 {
                QCF[entry_id(fighter.module_accessor)] = 2;
            }
            else if dir != 6
            && dir != 3
            && dir != 2 {
                QCF[entry_id(fighter.module_accessor)] = 0;
            }
        }
        else if QCF[entry_id(fighter.module_accessor)] == 2 {
            if dir == 6 {
                QCF[entry_id(fighter.module_accessor)] = 3;
            }
            else if dir != 6
            && dir != 3
            && dir != 9 {
                QCF[entry_id(fighter.module_accessor)] = 0;
            }
        }
        else {
            if dir != 6
            && dir != 9 {
                QCF[entry_id(fighter.module_accessor)] = 0;
            }
        }

        // The Counter-Hit Code (only applicable to Jabs, Tilts, and Smash Attacks)

        if [
            *FIGHTER_STATUS_KIND_ATTACK,
            *FIGHTER_STATUS_KIND_ATTACK_S3,
            *FIGHTER_STATUS_KIND_ATTACK_HI3,
            *FIGHTER_STATUS_KIND_ATTACK_LW3,
            *FIGHTER_STATUS_KIND_ATTACK_S4_START,
            *FIGHTER_STATUS_KIND_ATTACK_HI4_START,
            *FIGHTER_STATUS_KIND_ATTACK_LW4_START,
            *FIGHTER_STATUS_KIND_ATTACK_S4,
            *FIGHTER_STATUS_KIND_ATTACK_HI4,
            *FIGHTER_STATUS_KIND_ATTACK_LW4,
            *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD,
            *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD,
            *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD,
            *FIGHTER_STATUS_KIND_ATTACK_DASH,
            *FIGHTER_STATUS_KIND_ATTACK_AIR,
            *FIGHTER_STATUS_KIND_SPECIAL_N,
            *FIGHTER_STATUS_KIND_SPECIAL_S,
            *FIGHTER_STATUS_KIND_SPECIAL_LW,
            *FIGHTER_STATUS_KIND_SPECIAL_HI,
            *FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F,
            *FIGHTER_RYU_STATUS_KIND_ATTACK_NEAR,
            *FIGHTER_SIMON_STATUS_KIND_ATTACK_HOLD_START,
            *FIGHTER_SIMON_STATUS_KIND_ATTACK_HOLD,
            *FIGHTER_SIMON_STATUS_KIND_ATTACK_LW32,
            *FIGHTER_PICKEL_STATUS_KIND_ATTACK_FALL,
            *FIGHTER_PICKEL_STATUS_KIND_ATTACK_FALL_AERIAL,
            *FIGHTER_PICKEL_STATUS_KIND_ATTACK_JUMP,
            *FIGHTER_PICKEL_STATUS_KIND_ATTACK_WAIT,
            *FIGHTER_PICKEL_STATUS_KIND_ATTACK_WALK,
            *FIGHTER_PICKEL_STATUS_KIND_ATTACK_LANDING,
            *FIGHTER_PICKEL_STATUS_KIND_ATTACK_WALK_BACK,
            *FIGHTER_RYU_STATUS_KIND_ATTACK_COMMAND1,
            *FIGHTER_RYU_STATUS_KIND_ATTACK_COMMAND2,
            *FIGHTER_TANTAN_STATUS_KIND_ATTACK_COMBO,
            *FIGHTER_TANTAN_STATUS_KIND_ATTACK_WAIT,
            *FIGHTER_TANTAN_STATUS_KIND_ATTACK_WALK,
            *FIGHTER_TANTAN_STATUS_KIND_ATTACK_SQUAT,
            *FIGHTER_TANTAN_STATUS_KIND_ATTACK_SQUAT_RV,
            *FIGHTER_TANTAN_STATUS_KIND_ATTACK_LANDING,
            *FIGHTER_TANTAN_STATUS_KIND_ATTACK_LADDER,
            *FIGHTER_METAKNIGHT_STATUS_KIND_ATTACK_S3,
            *FIGHTER_METAKNIGHT_STATUS_KIND_ATTACK_LW3,
        ].contains(&StatusModule::status_kind(fighter.module_accessor)) {
            println!("Don't get hit, scrub.");
            COUNTER_HIT_STATE[entry_id(fighter.module_accessor)] = 1;
        }
        else {
            COUNTER_HIT_STATE[entry_id(fighter.module_accessor)] = 0;
        }
    }
}

// Use this for general per-frame weapon-level hooks
//pub fn once_per_weapon_frame(fighter_base : &mut L2CFighterBase) {
//    unsafe {
//        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter_base.lua_state_agent);
//        let frame = smash::app::lua_bind::MotionModule::frame(module_accessor) as i32;
//
//        if frame % 10 == 0 {
//            println!("[Weapon Hook] Frame : {}", frame);
//        }
//    }
//}

pub fn install() {
    // skyline::install_hook!(damage_air_main);
    smashline::install_agent_frame_callbacks!(
        global_fighter_frame
    );
//    acmd::add_custom_weapon_hooks!(once_per_weapon_frame);
}