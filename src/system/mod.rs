//use smash::app::utility::get_kind;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
// use smash::lua2cpp::L2CAgentBase;
use smash::lua2cpp::{L2CFighterCommon/*, L2CFighterBase*/};
// use smash_script::*;
use smashline::*;
use crate::FIGHTER_CUTIN_MANAGER_ADDR;
use crate::{IS_FUNNY, /*IS_FGC,*/ COUNTER_HIT_STATE, COUNTER_HIT_HELPER, OPPONENT_BOMA};
use crate::commonfuncs::*;
use skyline::nn::ro::LookupSymbol;
use smash::app::*;

pub static mut DMG_RATIO : [f32; 8] = [0.8; 8];
pub static mut QCF : [i32; 8] = [0; 8];
pub static mut QCB : [i32; 8] = [0; 8];

// Use this for general per-frame fighter-level hooks
#[fighter_frame_callback]
fn global_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;
        let boma = sv_system::battle_object_module_accessor(lua_state);
        let status_kind = StatusModule::status_kind(boma);

        LookupSymbol(
            &mut FIGHTER_CUTIN_MANAGER_ADDR,
            "_ZN3lib9SingletonIN3app19FighterCutInManagerEE9instance_E\u{0}"
                .as_bytes()
                .as_ptr(),
        );

        // The code to set up Funny Mode.

        if ItemModule::is_attach_item(boma, ItemKind(*ITEM_KIND_USAGIHAT))
        && IS_FUNNY[get_player_number(boma)] == false {
            IS_FUNNY[get_player_number(boma)] = true;
        }
        if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_RABBIT_CAP) {
            WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_RABBIT_CAP);
        }
        if (StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_DEAD || sv_information::is_ready_go() == false)
        && IS_FUNNY[get_player_number(boma)] {
            IS_FUNNY[get_player_number(boma)] = false;
        }

        // if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_HIT_REFLECTOR)
        // && IS_FGC[get_player_number(boma)] == false {
        //     IS_FGC[get_player_number(boma)] = true;
        //     println!("FGC is on!");
        // }
        // if IS_FGC[get_player_number(boma)] {
        //     if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_HIT_REFLECTOR) {
        //         WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_HIT_REFLECTOR);
        //         println!("Disabled Badge Reflector!");
        //     }
        // }
        // if !ItemModule::is_attach_item(boma, ItemKind(*ITEM_KIND_BADGE))
        // && IS_FGC[get_player_number(boma)] {
        //     IS_FGC[get_player_number(boma)] = false;
        //     println!("FGC is off!");
        // }

        // Remove Special Fall in Funny Mode

        if IS_FUNNY[get_player_number(boma)] {
            if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_FALL_SPECIAL {
                StatusModule::change_status_request_from_script(boma,*FIGHTER_STATUS_KIND_FALL_AERIAL,true);
            }
        }

        // Remove an OPPONENT_BOMA if the opponent is dead.

        if get_player_number(boma) < 8 {
            if OPPONENT_BOMA[get_player_number(boma)] != 0 {
                if StatusModule::status_kind(OPPONENT_BOMA[get_player_number(boma)] as *mut BattleObjectModuleAccessor) == *FIGHTER_STATUS_KIND_DEAD {
                    OPPONENT_BOMA[get_player_number(boma)] = 0;
                }
            }
        }
        
        // Platform Dropping while in shield.

        if (StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_GUARD
        || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_GUARD_ON)
        && ControlModule::get_stick_y(boma) < -0.8
        && GroundModule::is_passable_ground(boma) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_PASS, true);
        }
        
        // Command Inputs

        let dir = get_command_stick_direction(boma, true);

        // Quarter Circle Back

        if QCB[get_player_number(boma)] == 0 {
            if dir == 2 {
                QCB[get_player_number(boma)] = 1;
            }
        }
        else if QCB[get_player_number(boma)] == 1 {
            if dir == 1 {
                QCB[get_player_number(boma)] = 2;
            }
            else if dir != 4
            && dir != 1
            && dir != 2 {
                QCB[get_player_number(boma)] = 0;
            }
        }
        else if QCB[get_player_number(boma)] == 2 {
            if dir == 4 {
                QCB[get_player_number(boma)] = 3;
            }
            else if dir != 4
            && dir != 1
            && dir != 7 {
                QCB[get_player_number(boma)] = 0;
            }
        }
        else {
            if dir != 4
            && dir != 7 {
                QCB[get_player_number(boma)] = 0;
            }
        }

        //Quarter Circle Forward

        if QCF[get_player_number(boma)] == 0 {
            if dir == 2 {
                QCF[get_player_number(boma)] = 1;
            }
        }
        else if QCF[get_player_number(boma)] == 1 {
            if dir == 1 {
                QCF[get_player_number(boma)] = 3;
            }
            else if dir != 6
            && dir != 3
            && dir != 2 {
                QCF[get_player_number(boma)] = 0;
            }
        }
        else if QCF[get_player_number(boma)] == 2 {
            if dir == 6 {
                QCF[get_player_number(boma)] = 3;
            }
            else if dir != 6
            && dir != 3
            && dir != 9 {
                QCF[get_player_number(boma)] = 0;
            }
        }
        else {
            if dir != 6
            && dir != 9 {
                QCF[get_player_number(boma)] = 0;
            }
        }

        // The Counter-Hit Code (only applicable to Jabs, Tilts, and Smash Attacks)

        if status_kind == *FIGHTER_STATUS_KIND_ATTACK
        || status_kind == *FIGHTER_STATUS_KIND_ATTACK_S3
        || status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3
        || status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW3
        || status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4_START
        || status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4_START
        || status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW4_START
        || status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4
        || status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4
        || status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW4
        || status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD
        || status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD
        || status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD
        || status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH
        || status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
            if MotionModule::frame(boma) <= 1.0 {
                COUNTER_HIT_HELPER[get_player_number(boma)] = 0.0;
                COUNTER_HIT_STATE[get_player_number(boma)] = 1;
            }
            if AttackModule::is_attack(boma, 0, false)
            && COUNTER_HIT_HELPER[get_player_number(boma)] == 0.0 {
                COUNTER_HIT_HELPER[get_player_number(boma)] = MotionModule::frame(boma);
            }
            if !AttackModule::is_attack(boma, 0, false)
            && COUNTER_HIT_HELPER[get_player_number(boma)] != 0.0 {
                COUNTER_HIT_STATE[get_player_number(boma)] = 0;
            }
        }
        else {
            COUNTER_HIT_STATE[get_player_number(boma)] = 0;
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
    smashline::install_agent_frame_callbacks!(
        global_fighter_frame
    );
//    acmd::add_custom_weapon_hooks!(once_per_weapon_frame);
}