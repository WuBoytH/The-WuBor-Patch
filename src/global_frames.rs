#![allow(unused_must_use)]

use smash::{
    lua2cpp::L2CFighterCommon,
    hash40,
    phx::Vector3f,
    app::{lua_bind::*/*, sv_animcmd::**/, *},
    lib::lua_const::*
};
use smashline::*;
use crate::{
    vars::*,
    common_funcs::*
};
#[inline(always)]
pub unsafe fn global_fgc(fighter: &mut L2CFighterCommon) {
    let status = StatusModule::status_kind(fighter.module_accessor);
    if !is_damage_check(fighter.module_accessor, false) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ESCAPE_XLU_START_1F);
        FGC_HITSTUN_MUL[entry_id(fighter.module_accessor)] = 1.2;
    }
    else  {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ESCAPE_XLU_START_1F);
    }
    if status == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {
            let frame : f32;
            if !is_damage_check(fighter.module_accessor, true) {
                frame = 7.0;
            }
            else {
                frame = 30.0;
            }
            if MotionModule::frame(fighter.module_accessor) >= frame {
                CancelModule::enable_cancel(fighter.module_accessor);
            }
        }
    }
}

// Use this for general per-frame fighter-level hooks
#[fighter_frame_callback]
fn global_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let status = StatusModule::status_kind(fighter.module_accessor);

        if utility::get_kind(&mut *fighter.module_accessor) == *FIGHTER_KIND_DONKEY {
            IS_DK[entry_id(fighter.module_accessor)] = true;
        }

        // The code to set up Funny Mode.

        if ItemModule::is_attach_item(fighter.module_accessor, ItemKind(*ITEM_KIND_USAGIHAT))
        && IS_FUNNY[entry_id(fighter.module_accessor)] == false {
            IS_FUNNY[entry_id(fighter.module_accessor)] = true;
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_RABBIT_CAP) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_RABBIT_CAP);
        }
        if (status == *FIGHTER_STATUS_KIND_DEAD || sv_information::is_ready_go() == false)
        && IS_FUNNY[entry_id(fighter.module_accessor)] {
            IS_FUNNY[entry_id(fighter.module_accessor)] = false;
        }

        // The code to set up FGC Mode.

        if smashball::is_training_mode() {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
            && ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) {
                FGC_TRAINING = !FGC_TRAINING;
                if !FGC_TRAINING {
                    let pos = Vector3f{x: 0.0, y: 13.0, z: 0.0};
                    let rot = Vector3f{x: 0.0, y: 90.0, z: 0.0};
                    let onemoreeff: u32 = EffectModule::req_follow(fighter.module_accessor, smash::phx::Hash40{hash: hash40("sys_flame")}, smash::phx::Hash40{hash: hash40("top")}, &pos, &rot, 1.0, false, 0, 0, 0, 0, 0, false, false) as u32;
                    EffectModule::set_rgb(fighter.module_accessor, onemoreeff, 0.0, 0.0, 5.0);
                }
                else {
                    let pos = Vector3f{x: 0.0, y: 13.0, z: 0.0};
                    let rot = Vector3f{x: 0.0, y: 90.0, z: 0.0};
                    let onemoreeff: u32 = EffectModule::req_follow(fighter.module_accessor, smash::phx::Hash40{hash: hash40("sys_flame")}, smash::phx::Hash40{hash: hash40("top")}, &pos, &rot, 1.0, false, 0, 0, 0, 0, 0, false, false) as u32;
                    EffectModule::set_rgb(fighter.module_accessor, onemoreeff, 5.0, 0.0, 0.0);
                }
            }
        }
        if FighterUtil::is_hp_mode(fighter.module_accessor)
        && !smashball::is_training_mode() {
            IS_FGC[entry_id(fighter.module_accessor)] = true;
            FGC_TRAINING = false;
        }
        else if FGC_TRAINING {
            IS_FGC[entry_id(fighter.module_accessor)] = true;
        }
        else {
            IS_FGC[entry_id(fighter.module_accessor)] = false;
        }

        if IS_FGC[entry_id(fighter.module_accessor)] {
            global_fgc(fighter);
        }

        // Checks what frame you hit the opponent.

        if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            HIT_FRAME[entry_id(fighter.module_accessor)] = MotionModule::frame(fighter.module_accessor);
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

        if (status == *FIGHTER_STATUS_KIND_GUARD
        || status == *FIGHTER_STATUS_KIND_GUARD_ON)
        && ControlModule::get_stick_y(fighter.module_accessor) < -0.8
        && GroundModule::is_passable_ground(fighter.module_accessor) {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_PASS, true);
        }

        // Whifflag???

        if status == *FIGHTER_STATUS_KIND_ATTACK_AIR
        || (utility::get_kind(&mut *fighter.module_accessor) == *FIGHTER_KIND_BAYONETTA
        && status == *FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F)
        || (utility::get_kind(&mut *fighter.module_accessor) == *FIGHTER_KIND_TRAIL
        && [
            *FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_F,
            *FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_F
        ].contains(&status)) {
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
            || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
                AIR_WHIFF[entry_id(fighter.module_accessor)] = false;
            }
            else if AttackModule::is_attack(fighter.module_accessor, 0, false) {
                AIR_WHIFF[entry_id(fighter.module_accessor)] = true;
            }
        }
        else {
            AIR_WHIFF[entry_id(fighter.module_accessor)] = false;
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
        ].contains(&status) {
            COUNTER_HIT_STATE[entry_id(fighter.module_accessor)] = 1;
        }
        else {
            COUNTER_HIT_STATE[entry_id(fighter.module_accessor)] = 0;
        }
    }
}

// Use this for general per-frame weapon-level hooks
// #[weapon_frame_callback]
// pub fn once_per_weapon_frame(fighter_base : &mut L2CFighterBase) {
//    unsafe {
//        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter_base.lua_state_agent);
//        let frame = smash::app::lua_bind::MotionModule::frame(module_accessor) as i32;
//        if frame % 10 == 0 {
//            println!("[Weapon Hook] Frame : {}", frame);
//        }
//    }
// }

pub fn install() {
    install_agent_frame_callbacks!(
        global_fighter_frame,
        // once_per_weapon_frame
    );
}