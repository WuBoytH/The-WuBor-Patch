use smash::lib::lua_const::*;
use smash::lua2cpp::{L2CAgentBase, L2CFighterCommon};
use smash::app::lua_bind::*;
use smash::hash40;
use smash::app::*;
use smash_script::*;
use smashline::*;
use smash::phx::Hash40;
use smash::phx::Vector3f;
use crate::{IS_FUNNY, _TIME_COUNTER};
use crate::commonfuncs::*;

pub static mut LUCINA_SPECIAL_AIR_S : [bool; 8] = [false; 8];
static mut SHADOW_FRENZY : [bool; 8] = [false; 8];
static mut AWAKENING : [bool; 8] = [false; 8];
static mut CAN_ONE_MORE : [bool; 8] = [false; 8];
static mut TRAINING_TOOLS : [bool; 8] = [false; 8];
static mut IS_EX : [bool; 8] = [false; 8];
static mut SP_GAUGE : [f32; 8] = [0.0; 8];
static mut SPENT_SP : [f32; 8] = [0.0; 8];
static mut SP_GAUGE_MAX : [f32; 8] = [100.0; 8];
static mut METER_GAIN : [f32; 8] = [0.0; 8];
static mut DAMAGE_TAKEN : [f32; 8] = [0.0; 8];
static mut DAMAGE_TAKEN_PREV : [f32; 8] = [0.0; 8];
static mut GFXCOORDS : Vector3f = Vector3f { x: 0.0, y: 0.0, z: 0.0 };

pub unsafe fn special_effect(module_accessor: &mut BattleObjectModuleAccessor) {
    let pos = Vector3f{x: 0.0, y: 13.0, z: 0.0};
    let rot = Vector3f{x: 0.0, y: 90.0, z: 0.0};
    let onemoreeff: u32 = EffectModule::req_follow(module_accessor, Hash40{hash: hash40("sys_counter_flash")}, Hash40{hash: hash40("top")}, &pos, &rot, 1.0, false, 0, 0, 0, 0, 0, false, false) as u32;
    if SHADOW_FRENZY[get_player_number(module_accessor)] == false {
        EffectModule::set_rgb(module_accessor, onemoreeff, 5.0, 5.0, 0.0);
    }
    else if SHADOW_FRENZY[get_player_number(module_accessor)] == true {
        EffectModule::set_rgb(module_accessor, onemoreeff, 2.0, 0.0, 5.0);
    }
}

pub unsafe fn spent_meter(module_accessor: &mut BattleObjectModuleAccessor, onemore: bool) -> bool {
    let mut spent = false;
    if SP_GAUGE[get_player_number(module_accessor)] > 0.0 {
        if SHADOW_FRENZY[get_player_number(module_accessor)] {
            if onemore {
                SPENT_SP[get_player_number(module_accessor)] = 12.5;
                spent = true;
            }
            else {
                SPENT_SP[get_player_number(module_accessor)] = 6.25;
                spent = true;
            }
        }
        else if SP_GAUGE[get_player_number(module_accessor)] >= 25.0 {
            SPENT_SP[get_player_number(module_accessor)] = 25.0;
            spent = true;
        }
    }
    return spent;
}

pub unsafe fn upper_invuln(boma: &mut BattleObjectModuleAccessor, is_invuln: bool) {
    if is_invuln {
        HitModule::set_status_joint(boma, Hash40::new("waist"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(boma, Hash40::new("hip"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(boma, Hash40::new("head"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(boma, Hash40::new("shoulderr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(boma, Hash40::new("shoulderl"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(boma, Hash40::new("armr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(boma, Hash40::new("arml"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
    }
    else {
        HitModule::set_status_joint(boma, Hash40::new("waist"), HitStatus(*HIT_STATUS_NORMAL), 0);
        HitModule::set_status_joint(boma, Hash40::new("hip"), HitStatus(*HIT_STATUS_NORMAL), 0);
        HitModule::set_status_joint(boma, Hash40::new("head"), HitStatus(*HIT_STATUS_NORMAL), 0);
        HitModule::set_status_joint(boma, Hash40::new("shoulderr"), HitStatus(*HIT_STATUS_NORMAL), 0);
        HitModule::set_status_joint(boma, Hash40::new("shoulderl"), HitStatus(*HIT_STATUS_NORMAL), 0);
        HitModule::set_status_joint(boma, Hash40::new("armr"), HitStatus(*HIT_STATUS_NORMAL), 0);
        HitModule::set_status_joint(boma, Hash40::new("arml"), HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

pub unsafe fn full_invuln(boma: &mut BattleObjectModuleAccessor, is_invuln: bool) {
    if is_invuln {
        HitModule::set_whole(boma, HitStatus(*HIT_STATUS_XLU), 0);
    }
    else {
        HitModule::set_whole(boma, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

pub unsafe fn shadow_id(module_accessor: &mut BattleObjectModuleAccessor) -> bool {
    if WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 6
    || WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7 {
        return true;
    }
    else {
        return false;
    }
}

// This is left here for the standalone release of Yu Narukami
// #[skyline::hook(replace=lua_bind::WorkModule::is_enable_transition_term)]
// pub unsafe fn lucina_is_enable_transition_term_replace(module_accessor: &mut BattleObjectModuleAccessor, term: i32) -> bool {
//     let fighter_kind = utility::get_kind(module_accessor);
//     let ret = original!()(module_accessor,term);
//     let get_player_number(boma) = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_get_player_number(boma)) as usize;
//     if fighter_kind == *FIGHTER_KIND_LUCINA {
//         if LUCINA_SPECIAL_AIR_S[get_player_number(boma)] {
//             if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S {
//                 return false;
//             }
//             else {
//                 return ret;
//             }
//         }
//         if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW {
//             return false;
//         }
//         else {
//             return ret;
//         }
//     }
//     else {
//         return ret;
//     }
// }

#[fighter_frame( agent = FIGHTER_KIND_LUCINA )]
fn lucina_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = sv_system::battle_object_module_accessor(fighter.lua_state_agent);

        if get_player_number(boma) < 8 {

            // Reset Vars
            
            if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_REBIRTH {
                LUCINA_SPECIAL_AIR_S[get_player_number(boma)] = false;
                _TIME_COUNTER[get_player_number(boma)] = 0;
                if shadow_id(boma) {
                    if SHADOW_FRENZY[get_player_number(boma)] {
                        SP_GAUGE[get_player_number(boma)] = SP_GAUGE[get_player_number(boma)] / 2.0;
                        SHADOW_FRENZY[get_player_number(boma)] = false;
                    }
                }
                else {
                    SP_GAUGE_MAX[get_player_number(boma)] = 100.0;
                    AttackModule::set_power_up(boma, 1.0);
                    DamageModule::set_damage_mul(boma, 1.0);
                    SP_GAUGE[get_player_number(boma)] = 0.0;
                    AWAKENING[get_player_number(boma)] = false;
                }
            }
            if sv_information::is_ready_go() == false {
                DamageModule::set_damage_mul(boma, 1.0);
                LUCINA_SPECIAL_AIR_S[get_player_number(boma)] = false;
                SHADOW_FRENZY[get_player_number(boma)] = false;
                _TIME_COUNTER[get_player_number(boma)] = 0;
                if !(smashball::is_training_mode() && TRAINING_TOOLS[get_player_number(boma)]) {
                    SP_GAUGE[get_player_number(boma)] = 0.0;
                    AWAKENING[get_player_number(boma)] = false;
                    TRAINING_TOOLS[get_player_number(boma)] = false;
                    SP_GAUGE_MAX[get_player_number(boma)] = 100.0;
                    DAMAGE_TAKEN[get_player_number(boma)] = 0.0;
                    DAMAGE_TAKEN_PREV[get_player_number(boma)] = 0.0;
                }
            }

            // Meter Controller (Move to hook?)

            DAMAGE_TAKEN[get_player_number(boma)] = DamageModule::damage(boma, 0);
            if DAMAGE_TAKEN[get_player_number(boma)] > DAMAGE_TAKEN_PREV[get_player_number(boma)]
            && SHADOW_FRENZY[get_player_number(boma)] == false {
                SP_GAUGE[get_player_number(boma)] += (DAMAGE_TAKEN[get_player_number(boma)] - DAMAGE_TAKEN_PREV[get_player_number(boma)]) * (1.0/6.0);
                if SP_GAUGE[get_player_number(boma)] > SP_GAUGE_MAX[get_player_number(boma)] {
                    SP_GAUGE[get_player_number(boma)] = SP_GAUGE_MAX[get_player_number(boma)];
                }
            }
            DAMAGE_TAKEN_PREV[get_player_number(boma)] = DAMAGE_TAKEN[get_player_number(boma)];
            if AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) && SHADOW_FRENZY[get_player_number(boma)] == false {
                if MotionModule::motion_kind(boma) != hash40("attack_dash")
                && MotionModule::motion_kind(boma) != hash40("special_s1")
                && MotionModule::motion_kind(boma) != hash40("special_air_s2_hi")
                && MotionModule::motion_kind(boma) != hash40("special_air_s2_lw")
                && MotionModule::motion_kind(boma) != hash40("special_hi")
                && MotionModule::motion_kind(boma) != hash40("special_air_hi") {
                    IS_EX[get_player_number(boma)] = false;
                }
                if IS_EX[get_player_number(boma)] == false {
                    METER_GAIN[get_player_number(boma)] = AttackModule::get_power(boma, 0, false, 1.0, false);
                    if shadow_id(boma) == false {
                        METER_GAIN[get_player_number(boma)] *= 0.75;
                    }
                    if IS_FUNNY[get_player_number(boma)] {
                        METER_GAIN[get_player_number(boma)] *= 3.0;
                    }
                    SP_GAUGE[get_player_number(boma)] += METER_GAIN[get_player_number(boma)];
                    if SP_GAUGE[get_player_number(boma)] > SP_GAUGE_MAX[get_player_number(boma)] {
                        SP_GAUGE[get_player_number(boma)] = SP_GAUGE_MAX[get_player_number(boma)];
                    }
                }
            }

            // Normal vs Shadow Effects

            if shadow_id(boma) == true {
                DamageModule::set_damage_mul(boma, 0.92);
                AttackModule::set_power_up(boma, 0.8);
                if SHADOW_FRENZY[get_player_number(boma)] == true {
                    if !TRAINING_TOOLS[get_player_number(boma)] {
                        if IS_FUNNY[get_player_number(boma)] {
                            SP_GAUGE[get_player_number(boma)] -= 1.0/64.0;
                        }
                        else {
                            SP_GAUGE[get_player_number(boma)] -= 1.0/16.0;
                        }
                    }
                }
            }
            else {
                AttackModule::set_power_up(boma, 1.0);
                if DamageModule::damage(boma, 0) > 100.0 {
                    if AWAKENING[get_player_number(boma)] == false
                    && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND
                    && (!is_damage_check(boma)
                    || IS_FUNNY[get_player_number(boma)])
                    && sv_information::is_ready_go() == true {
                        SP_GAUGE[get_player_number(boma)] += 50.0;
                        AWAKENING[get_player_number(boma)] = true;
                        macros::FT_START_CUTIN(fighter);
                    }
                }
            }
            if AWAKENING[get_player_number(boma)] == true {
                DamageModule::set_damage_mul(boma, 0.8);
                SP_GAUGE_MAX[get_player_number(boma)] = 150.0;
            }

            if SP_GAUGE[get_player_number(boma)] <= 0.0{
                SP_GAUGE[get_player_number(boma)] = 0.0;
                SHADOW_FRENZY[get_player_number(boma)] = false;
            }

            if SP_GAUGE[get_player_number(boma)] > SP_GAUGE_MAX[get_player_number(boma)] {
                SP_GAUGE[get_player_number(boma)] = SP_GAUGE_MAX[get_player_number(boma)];
            }

            // Special Lw Check

            if StatusModule::prev_status_kind(boma, 0) != StatusModule::status_kind(boma) {
                CAN_ONE_MORE[get_player_number(boma)] = false;
            }

            if (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD))
            && MotionModule::motion_kind(boma) != hash40("catch_attack")
            && !((MotionModule::motion_kind(boma) == hash40("special_hi") || MotionModule::motion_kind(boma) == hash40("special_air_hi")) && IS_EX[get_player_number(boma)]) {
                CAN_ONE_MORE[get_player_number(boma)] = true;
            }

            if CAN_ONE_MORE[get_player_number(boma)] == true {
                if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK_RAW) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW) {
                    if spent_meter(boma, true) {
                        fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), false.into());
                    }
                }
                else if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_HI)
                && SP_GAUGE[get_player_number(boma)] == 100.0
                && SHADOW_FRENZY[get_player_number(boma)] == false
                && shadow_id(boma) {
                    fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT.into(), false.into());
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT, true);
                }
            }
            else if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_THROW {
                let throwframe : f32;
                if MotionModule::motion_kind(boma) == hash40("throw_f") {
                    throwframe = 18.0;
                }
                else if MotionModule::motion_kind(boma) == hash40("throw_b") {
                    throwframe = 19.0;
                }
                else if MotionModule::motion_kind(boma) == hash40("throw_hi") {
                    throwframe = 13.0;
                }
                else if MotionModule::motion_kind(boma) == hash40("throw_lw") {
                    throwframe = 20.0;
                }
                else{
                    throwframe = 20.0;
                }
                if MotionModule::frame(boma) > throwframe && CAN_ONE_MORE[get_player_number(boma)] == false {
                    CAN_ONE_MORE[get_player_number(boma)] = true;
                }
                if CAN_ONE_MORE[get_player_number(boma)] == true {
                    if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK_RAW)  && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW) {
                        if spent_meter(boma, true) {
                            fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), false.into());
                        }
                    }
                    else if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_HI)
                    && SP_GAUGE[get_player_number(boma)] == 100.0
                    && SHADOW_FRENZY[get_player_number(boma)] == false
                    && shadow_id(boma) {
                        fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT.into(), false.into());
                    }
                }
            }

            // Meter Effects

            if (SP_GAUGE[get_player_number(boma)] >= 25.0 && SHADOW_FRENZY[get_player_number(boma)] == false)
            || SHADOW_FRENZY[get_player_number(boma)] == true {
                if _TIME_COUNTER[get_player_number(boma)] == 0 {
                    let onemoreeff: u32 = EffectModule::req_follow(boma, Hash40::new("sys_hit_aura"), smash::phx::Hash40::new("haver"), &GFXCOORDS, &GFXCOORDS, 0.06, true, 0, 0, 0, 0, 0, true, true) as u32;
                    let onemoreeff2: u32 = EffectModule::req_follow(boma, Hash40::new("sys_hit_aura"), smash::phx::Hash40::new("havel"), &GFXCOORDS, &GFXCOORDS, 0.06, true, 0, 0, 0, 0, 0, true, true) as u32;
                    if SHADOW_FRENZY[get_player_number(boma)] || (SP_GAUGE[get_player_number(boma)] >= 125.0 && SP_GAUGE[get_player_number(boma)] < 150.0) {
                        EffectModule::set_rgb(boma, onemoreeff, 2.0, 0.0, 5.0);
                        EffectModule::set_rgb(boma, onemoreeff2, 2.0, 0.0, 5.0);
                    }
                    else if SP_GAUGE[get_player_number(boma)] >= 50.0 && SP_GAUGE[get_player_number(boma)] < 75.0 {
                        EffectModule::set_rgb(boma, onemoreeff, 0.0, 0.0, 5.0);
                        EffectModule::set_rgb(boma, onemoreeff2, 0.0, 0.0, 5.0);
                    }
                    else if SP_GAUGE[get_player_number(boma)] >= 75.0 && SP_GAUGE[get_player_number(boma)] < 100.0 {
                        EffectModule::set_rgb(boma, onemoreeff, 5.0, 5.0, 0.0);
                        EffectModule::set_rgb(boma, onemoreeff2, 5.0, 5.0, 0.0);
                    }
                    else if SP_GAUGE[get_player_number(boma)] >= 100.0 && SP_GAUGE[get_player_number(boma)] < 125.0 {
                        EffectModule::set_rgb(boma, onemoreeff, 5.0, 0.0, 0.0);
                        EffectModule::set_rgb(boma, onemoreeff2, 5.0, 0.0, 0.0);
                    }
                    else if SP_GAUGE[get_player_number(boma)] >= 25.0 && SP_GAUGE[get_player_number(boma)] < 50.0 {
                        EffectModule::set_rgb(boma, onemoreeff, 0.0, 5.0, 5.0);
                        EffectModule::set_rgb(boma, onemoreeff2, 0.0, 5.0, 5.0);
                    }
                    else{
                        EffectModule::set_rgb(boma, onemoreeff, 5.0, 5.0, 5.0);
                        EffectModule::set_rgb(boma, onemoreeff2, 5.0, 5.0, 5.0);
                    }
                    _TIME_COUNTER[get_player_number(boma)] = 12;
                }
                _TIME_COUNTER[get_player_number(boma)] -= 1;
            }

            // Special S Air Check

            if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_SPECIAL_S && StatusModule::status_kind(boma) != *FIGHTER_STATUS_KIND_LANDING {
                LUCINA_SPECIAL_AIR_S[get_player_number(boma)] = true;
            }
            else if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
                LUCINA_SPECIAL_AIR_S[get_player_number(boma)] = false;
            }

            // Shadow Frenzy Check

            if MotionModule::motion_kind(boma) == hash40("appeal_hi_l") || MotionModule::motion_kind(boma) == hash40("appeal_hi_r") {
                if SP_GAUGE[get_player_number(boma)] == 100.0 && shadow_id(boma) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                    SHADOW_FRENZY[get_player_number(boma)] = true;
                }
            }
            else if SP_GAUGE[get_player_number(boma)] == 0.0 {
                SHADOW_FRENZY[get_player_number(boma)] = false;
            }

            // Move Effects

            if MotionModule::motion_kind(boma) == hash40("special_s1") {
                if MotionModule::frame(boma) > 6.0 && MotionModule::frame(boma) < 18.0 {
                    macros::SET_SPEED_EX(fighter, 2.8, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                }
                if MotionModule::frame(boma) >= 18.0 {
                    macros::SET_SPEED_EX(fighter, 0, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                }
            }

            if MotionModule::motion_kind(boma) == hash40("special_s2_lw") || MotionModule::motion_kind(boma) == hash40("special_s2_hi") {
                if MotionModule::frame(boma) > 0.0 {
                    AttackModule::clear_all(boma);
                }
            }

            // Jump Cancels

            if MotionModule::motion_kind(boma) == hash40("attack_hi3")
            || MotionModule::motion_kind(boma) == hash40("attack_12")
            || MotionModule::motion_kind(boma) == hash40("attack_air_n")
            || MotionModule::motion_kind(boma) == hash40("attack_air_hi")
            || MotionModule::motion_kind(boma) == hash40("attack_air_lw")
            || (MotionModule::motion_kind(boma) == hash40("attack_dash") && IS_EX[get_player_number(boma)]) {
                if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                    if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_JUMP)
                    && (WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < 2 || IS_FUNNY[get_player_number(boma)]) {
                        CancelModule::enable_cancel(boma);
                    }
                }
            }

            // Training Mode Tools

            if smashball::is_training_mode(){
                if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_APPEAL_S_L) {
                    if SP_GAUGE[get_player_number(boma)] > 25.0 {
                        SP_GAUGE[get_player_number(boma)] -= 25.0
                    }
                    else {
                        SP_GAUGE[get_player_number(boma)] = 0.0;
                    }
                }
                if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
                    if SP_GAUGE[get_player_number(boma)] < SP_GAUGE_MAX[get_player_number(boma)] - 25.0 {
                        SP_GAUGE[get_player_number(boma)] += 25.0
                    }
                    else {
                        SP_GAUGE[get_player_number(boma)] = SP_GAUGE_MAX[get_player_number(boma)];
                    }
                }
                if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_APPEAL_LW) {
                    if TRAINING_TOOLS[get_player_number(boma)] {
                        TRAINING_TOOLS[get_player_number(boma)] = false;
                        let pos = Vector3f{x: 0.0, y: 13.0, z: 0.0};
                        let rot = Vector3f{x: 0.0, y: 90.0, z: 0.0};
                        let onemoreeff: u32 = EffectModule::req_follow(boma, smash::phx::Hash40{hash: hash40("sys_counter_flash")}, smash::phx::Hash40{hash: hash40("top")}, &pos, &rot, 1.0, false, 0, 0, 0, 0, 0, false, false) as u32;
                        EffectModule::set_rgb(boma, onemoreeff, 0.0, 0.0, 5.0);
                    }
                    else {
                        TRAINING_TOOLS[get_player_number(boma)] = true;
                        let pos = Vector3f{x: 0.0, y: 13.0, z: 0.0};
                        let rot = Vector3f{x: 0.0, y: 90.0, z: 0.0};
                        let onemoreeff: u32 = EffectModule::req_follow(boma, smash::phx::Hash40{hash: hash40("sys_counter_flash")}, smash::phx::Hash40{hash: hash40("top")}, &pos, &rot, 1.0, false, 0, 0, 0, 0, 0, false, false) as u32;
                        EffectModule::set_rgb(boma, onemoreeff, 5.0, 0.0, 0.0);
                    }
                }
            }
        }
    }
}

#[acmd_script( agent = "lucina", script = "game_attack11", category = ACMD_GAME, low_priority )]
unsafe fn lucina_jab1(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state, 4.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.5, 361, 15, 0, 20, 2.0, 0.0, 9.4, 6.2, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.5, 361, 15, 0, 20, 2.0, 0.0, 9.4, 8.8, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        AttackModule::clear_all(boma);
    }
    sv_animcmd::frame(lua_state, 11.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART);
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "lucina", script = "effect_attack11", category = ACMD_EFFECT, low_priority )]
unsafe fn lucina_jab1eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    sv_animcmd::frame(lua_state, 3.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 18, -5, 0, 330, 90, 0.95, true);
    }
    sv_animcmd::frame(lua_state, 4.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), -1, 10, 10, 0, 0, 0, 1.15, 0, 0, 0, 0, 0, 360, true);
    }
}

#[acmd_script( agent = "lucina", script = "game_attack12", category = ACMD_GAME, low_priority )]
unsafe fn lucina_jab2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 4.5, 50, 100, 0, 50, 4.2, 5.0, -1.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 3.5, 50, 100, 0, 50, 3.8, -1.0, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    sv_animcmd::wait(lua_state, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "lucina", script = "effect_attack12", category = ACMD_EFFECT, low_priority )]
unsafe fn lucina_jab2eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    sv_animcmd::frame(lua_state, 5.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_line"), Hash40::new("top"), -5, 12, 2, 0, 0, 0, 0.95, true);
    }
    sv_animcmd::frame(lua_state, 6.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), -6, 12, 17, 0, 0, 0, 1.15, 0, 0, 0, 0, 0, 360, true);
    }
}

#[acmd_script( agent = "lucina", script = "game_attackhi3", category = ACMD_GAME, low_priority )]
unsafe fn lucina_utilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state, 5.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("sword1"), 6.0, 71, 60, 0, 65, 2.0, 0.0, -1.0, 1.0, Some(1.0), Some(1.0), Some(12.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(lua_state, 7.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("sword1"), 6.0, 71, 60, 0, 65, 2.0, 0.0, -1.0, 1.0, Some(1.0), Some(1.0), Some(12.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(lua_state, 8.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("sword1"), 6.0, 71, 60, 0, 65, 2.0, 0.0, -1.0, 1.0, Some(1.0), Some(0.0), Some(12.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(lua_state, 9.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("sword1"), 6.0, 71, 60, 0, 65, 2.0, 0.0, -1.0, 1.0, Some(1.0), Some(-1.0), Some(12.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::wait(lua_state, 12.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "lucina", script = "game_attacklw3", category = ACMD_GAME, low_priority )]
unsafe fn lucina_dtilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state, 7.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 75, 40, 0, 57, 2.7, 0.0, 2.7, 16.700001, Some(0.0), Some(4.3), Some(9.2), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.2, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("sword1"), 6.0, 75, 40, 0, 57, 2.7, 0.0, 0.0, 8.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.2, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(boma, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    sv_animcmd::wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "lucina", script = "game_attackdash", category = ACMD_GAME, low_priority )]
unsafe fn lucina_dashattack(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state, 5.0);
    if macros::is_excute(fighter) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK)
        && spent_meter(boma, false) {
            SP_GAUGE[get_player_number(boma)] -= SPENT_SP[get_player_number(boma)];
            special_effect(boma);
            IS_EX[get_player_number(boma)] = true;
        }
        else{
            IS_EX[get_player_number(boma)] = false;
        }
    }
    sv_animcmd::frame(lua_state, 7.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 7.0, 65, 85, 0, 65, 3.6, 5.0, -1.0, 1.5, Some(1.5), Some(-1.0), Some(1.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.2, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 65, 85, 0, 65, 2.5, 0.0, 2.5, -2.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.2, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(boma, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_XLU);
    }
    sv_animcmd::wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(boma, 1, false);
    }
    sv_animcmd::frame(lua_state, 18.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
        KineticModule::resume_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
}

#[acmd_script( agent = "lucina", script = "sound_attackdash", category = ACMD_SOUND, low_priority )]
unsafe fn lucina_dashattacksound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    sv_animcmd::frame(lua_state, 7.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_lucina_rnd_attack"));
        macros::PLAY_SE(fighter, Hash40::new("se_lucina_swing_ll"));
    }
}

#[acmd_script( agent = "lucina", script = "game_attackairn", category = ACMD_GAME, low_priority )]
unsafe fn lucina_nair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state, 4.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    sv_animcmd::frame(lua_state, 7.0);
    macros::FT_MOTION_RATE(fighter, 0.0833333);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 4.5, 361, 100, 0, 40, 5.0, 3.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 4.5, 361, 100, 0, 40, 5.0, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 2, 0, Hash40::new("kneel"), 4.5, 361, 100, 0, 40, 3.0, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    sv_animcmd::frame(lua_state, 31.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    sv_animcmd::wait(lua_state, 4.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "lucina", script = "game_attackairf", category = ACMD_GAME, low_priority )]
unsafe fn lucina_fair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    macros::FT_MOTION_RATE(fighter, 1.6);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    sv_animcmd::frame(lua_state, 6.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("sword1"), 10.0, 361, 80, 0, 40, 3.0, 1.0, 0.0, 3.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 10.0, 361, 80, 0, 40, 3.8, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("sword1"), 10.0, 361, 80, 0, 40, 3.0, 1.0, 0.0, 7.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::wait(lua_state, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    sv_animcmd::frame(lua_state, 36.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "lucina", script = "game_attackairb", category = ACMD_GAME, low_priority )]
unsafe fn lucina_bair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    macros::FT_MOTION_RATE(fighter, 1.5);
    sv_animcmd::frame(lua_state, 3.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::REVERSE_LR(fighter);
    }
    sv_animcmd::frame(lua_state, 6.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    sv_animcmd::frame(lua_state, 7.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("sword1"), 10.0, 361, 80, 0, 40, 3.5, 0.0, 0.0, 3.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 10.0, 361, 80, 0, 40, 3.5, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("sword1"), 10.0, 361, 80, 0, 40, 3.5, 0.0, 0.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::wait(lua_state, 5.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    sv_animcmd::frame(lua_state, 32.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "lucina", script = "game_attackairhi", category = ACMD_GAME, low_priority )]
unsafe fn lucina_uair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state, 3.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    sv_animcmd::frame(lua_state, 5.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("sword1"), 9.0, 55, 40, 0, 40, 3.5, 1.0, 0.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 9.0, 55, 40, 0, 40, 3.0, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("colonells"), 9.0, 55, 40, 0, 40, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("sword1"), 9.0, 55, 40, 0, 40, 3.5, 1.0, 0.0, 6.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(lua_state, 13.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    sv_animcmd::frame(lua_state, 31.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "lucina", script = "game_attackairlw", category = ACMD_GAME, low_priority )]
unsafe fn lucina_dair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state, 3.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    sv_animcmd::frame(lua_state, 9.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 9.0, 80, 75, 0, 30, 3.0, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("sword1"), 9.0, 80, 75, 0, 30, 3.5, 1.0, 0.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("sword1"), 9.0, 80, 75, 0, 30, 3.5, 1.0, 0.0, 6.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(lua_state, 14.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    sv_animcmd::frame(lua_state, 55.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "lucina", script = "game_attacks4", category = ACMD_GAME, low_priority )]
unsafe fn lucina_fsmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state, 7.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    sv_animcmd::frame(lua_state, 13.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 13.5, 361, 85, 0, 30, 4.0, 0.0, 8.5, 10.0, Some(0.0), Some(3.5), Some(22.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "lucina", script = "game_attackhi4", category = ACMD_GAME, low_priority )]
unsafe fn lucina_usmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    sv_animcmd::frame(lua_state, 13.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("sword1"), 12.0, 89, 90, 0, 42, 5.8, 0.0, 0.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("sword1"), 12.0, 89, 90, 0, 42, 4.6, 0.0, 0.0, 7.3, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("sword1"), 12.0, 89, 90, 0, 42, 5.8, 0.0, 0.0, -3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 125, 100, 155, 0, 5.5, 0.0, 5.0, 9.0, Some(0.0), Some(5.0), Some(-9.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(boma, 0, false);
    }
    sv_animcmd::wait(lua_state, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "lucina", scripts = [ "game_specialnend", "game_specialnendhi", "game_specialnendlw", "game_specialairnend", "game_specialairnendhi", "game_specialairnendlw" ], category = ACMD_GAME, low_priority )]
unsafe fn lucina_nspecialend(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        sv_animcmd::frame(lua_state, 8.0);
        if PostureModule::lr(boma) == 1.0 && ControlModule::get_stick_x(boma) < -0.75 {
            PostureModule::reverse_lr(boma);
        }
        else if PostureModule::lr(boma) == -1.0 && ControlModule::get_stick_x(boma) > 0.75 {
            PostureModule::reverse_lr(boma);
        }
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 9.0, 361, 60, 0, 40, 3.0, 0.0, 5.0, 8.0, Some(0.0), Some(5.0), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 9.0, 361, 60, 0, 40, 3.0, 0.0, 5.0, 25.0, Some(0.0), Some(5.0), Some(27.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 361, 60, 0, 40, 0.9, 0.0, 5.0, 17.0, Some(0.0), Some(5.0), Some(22.0), 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 361, 60, 0, 40, 0.9, 0.0, 5.0, 23.5, Some(0.0), Some(5.0), Some(28.9), 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        sv_animcmd::frame(lua_state, 9.0);
        if macros::is_excute(fighter) {
            AttackModule::clear(boma, 0, false);
            AttackModule::clear(boma, 1, false);
        }
        sv_animcmd::frame(lua_state, 10.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
    }
    else {
        sv_animcmd::frame(lua_state, 1.0);
        macros::FT_MOTION_RATE(fighter, 0.125);
        sv_animcmd::frame(lua_state, 9.0);
        macros::FT_MOTION_RATE(fighter, 2.0);
        sv_animcmd::frame(lua_state, 13.0);
        if PostureModule::lr(boma) == 1.0 && ControlModule::get_stick_x(boma) < -0.75 {
            PostureModule::reverse_lr(boma);
        }
        else if PostureModule::lr(boma) == -1.0 && ControlModule::get_stick_x(boma) > 0.75 {
            PostureModule::reverse_lr(boma);
        }
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 361, 60, 0, 40, 3.3, 0.0, 7.7, 9.1, Some(0.0), Some(7.7), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("sword1"), 9.0, 361, 60, 0, 40, 4.0, 2.0, 1.0, 1.5, Some(14.0), Some(1.0), Some(1.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("sword1"), 9.0, 361, 60, 0, 40, 3.5, 2.0, 1.0, 7.5, Some(9.5), Some(1.0), Some(7.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        sv_animcmd::frame(lua_state, 16.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        sv_animcmd::frame(lua_state, 17.0);
        macros::FT_MOTION_RATE(fighter, 1.0);
    }
}

#[acmd_script( agent = "lucina", scripts = [ "game_specialnendmax", "game_specialnendmaxhi", "game_specialnendmaxlw", "game_specialairnendmax", "game_specialairnendmaxhi", "game_specialairnendmaxlw" ], category = ACMD_GAME, low_priority )]
unsafe fn lucina_nspecialendmax(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        sv_animcmd::frame(lua_state, 8.0);
        if macros::is_excute(fighter) {
            if PostureModule::lr(boma) == 1.0 && ControlModule::get_stick_x(boma) < -0.75 {
                PostureModule::reverse_lr(boma);
            }
            else if PostureModule::lr(boma) == -1.0 && ControlModule::get_stick_x(boma) > 0.75 {
                PostureModule::reverse_lr(boma);
            }
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 17.0, 60, 100, 0, 0, 3.0, 0.0, 5.0, 8.0, Some(0.0), Some(5.0), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 17.0, 60, 100, 0, 0, 3.0, 0.0, 5.0, 25.0, Some(0.0), Some(5.0), Some(27.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 17.0, 60, 100, 0, 0, 0.9, 0.0, 5.0, 17.0, Some(0.0), Some(5.0), Some(22.0), 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 17.0, 60, 100, 0, 0, 0.9, 0.0, 5.0, 23.5, Some(0.0), Some(5.0), Some(28.9), 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
        sv_animcmd::frame(lua_state, 9.0);
        if macros::is_excute(fighter) {
            AttackModule::clear(boma, 0, false);
            AttackModule::clear(boma, 1, false);
        }
        sv_animcmd::frame(lua_state, 10.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
    }
    else{
        sv_animcmd::frame(lua_state, 1.0);
        macros::FT_MOTION_RATE(fighter, 0.125);
        sv_animcmd::frame(lua_state, 9.0);
        macros::FT_MOTION_RATE(fighter, 2.0);
        sv_animcmd::frame(lua_state, 13.0);
        if macros::is_excute(fighter) {
            if PostureModule::lr(boma) == 1.0 && ControlModule::get_stick_x(boma) < -0.75 {
                PostureModule::reverse_lr(boma);
            }
            else if PostureModule::lr(boma) == -1.0 && ControlModule::get_stick_x(boma) > 0.75 {
                PostureModule::reverse_lr(boma);
            }
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 17.0, 60, 100, 0, 0, 3.3, 0.0, 7.7, 9.1, Some(0.0), Some(7.7), Some(7.0), 2.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("sword1"), 17.0, 60, 100, 0, 0, 4.0, 2.0, 1.0, 1.5, Some(14.0), Some(1.0), Some(1.5), 2.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("sword1"), 17.0, 60, 100, 0, 0, 3.5, 2.0, 1.0, 7.5, Some(9.5), Some(1.0), Some(7.5), 2.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
        sv_animcmd::frame(lua_state, 16.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        sv_animcmd::frame(lua_state, 17.0);
        macros::FT_MOTION_RATE(fighter, 1.0);
    }
}

#[acmd_script( agent = "lucina", script = "game_specials1", category = ACMD_GAME, low_priority )]
unsafe fn lucina_sspecial1(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    let mut dmg : f32;
    let mut kbg : i32;
    sv_animcmd::frame(lua_state, 1.0);
    macros::FT_MOTION_RATE(fighter, 4.0);
    sv_animcmd::frame(lua_state, 2.0);
    if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL)
    && spent_meter(boma, false) {
        SP_GAUGE[get_player_number(boma)] -= SPENT_SP[get_player_number(boma)];
        special_effect(boma);
        IS_EX[get_player_number(boma)] = true;
        macros::FT_MOTION_RATE(fighter, 0.333);
    }
    else {
        IS_EX[get_player_number(boma)] = false;
    }
    sv_animcmd::frame(lua_state, 5.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    sv_animcmd::frame(lua_state, 8.0);
    if macros::is_excute(fighter) {
        if IS_EX[get_player_number(boma)] == true {
            dmg = 18.0;
            kbg = 45;
        }
        else {
            dmg = 14.0;
            kbg = 66;
        }
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), dmg, 45, kbg, 0, 45, 2.5, 0.0, 8.5, 8.0, Some(0.0), Some(8.5), Some(20.0), 2.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), dmg, 45, kbg, 0, 45, 2.5, 0.0, 8.5, 25.0, Some(0.0), Some(8.5), Some(27.0), 2.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), dmg, 45, kbg, 0, 45, 0.7, 0.0, 8.5, 17.0, Some(0.0), Some(8.5), Some(22.0), 2.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), dmg, 45, kbg, 0, 45, 0.7, 0.0, 8.5, 23.5, Some(0.0), Some(8.5), Some(28.9), 2.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(lua_state, 13.0);
    if macros::is_excute(fighter) {
        if IS_EX[get_player_number(boma)] == true {
            dmg = 13.0;
            kbg = 45;
        }
        else {
            dmg = 9.0;
            kbg = 66;
        }
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), dmg, 45, kbg, 0, 45, 2.5, 0.0, 8.5, 8.0, Some(0.0), Some(8.5), Some(20.0), 1.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), dmg, 45, kbg, 0, 45, 2.5, 0.0, 8.5, 25.0, Some(0.0), Some(8.5), Some(27.0), 1.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), dmg, 45, kbg, 0, 45, 0.7, 0.0, 8.5, 17.0, Some(0.0), Some(8.5), Some(22.0), 1.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), dmg, 45, kbg, 0, 45, 0.7, 0.0, 8.5, 23.5, Some(0.0), Some(8.5), Some(28.9), 1.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(lua_state, 22.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "lucina", script = "game_specialairs1", category = ACMD_GAME, low_priority )]
unsafe fn lucina_sspecial1air(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_LANDING_CLEAR_SPEED);
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        macros::SET_SPEED_EX(fighter, 1.362, 2, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        KineticModule::suspend_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
    sv_animcmd::frame(lua_state, 13.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
    }
    sv_animcmd::frame(lua_state, 46.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
}

#[acmd_script( agent = "lucina", script = "game_specialairs2lw", category = ACMD_GAME, low_priority )]
unsafe fn lucina_sspecial2lwair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_LANDING_CLEAR_SPEED);
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        macros::SET_SPEED_EX(fighter, 0, 0.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        KineticModule::suspend_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
    }
    sv_animcmd::frame(lua_state, 2.0);
    macros::FT_MOTION_RATE(fighter, 0.1);
    sv_animcmd::frame(lua_state, 12.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    sv_animcmd::frame(lua_state, 13.0);
    if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL)
    && spent_meter(boma, false) {
        SP_GAUGE[get_player_number(boma)] -= SPENT_SP[get_player_number(boma)];
        special_effect(boma);
        IS_EX[get_player_number(boma)] = true;
    }
    else {
        IS_EX[get_player_number(boma)] = false;
    }
    sv_animcmd::frame(lua_state, 14.0);
    if macros::is_excute(fighter) {
        let dmg : f32;
        let velx : f32;
        let vely : f32;
        if IS_EX[get_player_number(boma)] == true {
            dmg = 12.0;
            velx = 1.75;
            vely = -3.5;
        }
        else {
            dmg = 8.0;
            velx = 1.5;
            vely = -3.0;
        }
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        macros::SET_SPEED_EX(fighter, velx, vely, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        macros::ATTACK(fighter, 0, 0, Hash40::new("footl"), dmg, 40, 90, 0, 60, 6.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneel"), dmg, 40, 90, 0, 60, 5.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 2, 0, Hash40::new("footl"), dmg, 40, 60, 0, 60, 6.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 3, 0, Hash40::new("kneel"), dmg, 40, 60, 0, 60, 5.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    sv_animcmd::frame(lua_state, 50.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    sv_animcmd::frame(lua_state, 61.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
        KineticModule::resume_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
}

#[acmd_script( agent = "lucina", script = "game_specialairs2hi", category = ACMD_GAME, low_priority )]
unsafe fn lucina_sspecial2hiair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_LANDING_CLEAR_SPEED);
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        macros::SET_SPEED_EX(fighter, 1.5, -2, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        KineticModule::suspend_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
    }
    sv_animcmd::frame(lua_state, 4.0);
    if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL)
    && spent_meter(boma, false) {
        SP_GAUGE[get_player_number(boma)] -= SPENT_SP[get_player_number(boma)];
        special_effect(boma);
        IS_EX[get_player_number(boma)] = true;
    }
    else {
        IS_EX[get_player_number(boma)] = false;
    }
    sv_animcmd::frame(lua_state, 8.0);
    if macros::is_excute(fighter) {
        let dmg : f32;
        let velx : f32;
        let vely : f32;
        if IS_EX[get_player_number(boma)] == true {
            dmg = 16.0;
            velx = 2.0;
            vely = -1.5;
        }
        else {
            dmg = 12.0;
            velx = 1.5;
            vely = -3.0;
        }
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        macros::SET_SPEED_EX(fighter, velx, vely, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), dmg, 361, 66, 0, 45, 2.5, 0.0, 6.5, 8.0, Some(0.0), Some(4.0), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), dmg, 361, 66, 0, 45, 2.5, 0.0, 3.0, 25.0, Some(0.0), Some(2.7), Some(27.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), dmg, 361, 66, 0, 45, 0.7, 0.0, 5.6, 17.0, Some(0.0), Some(3.5), Some(22.0), 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), dmg, 361, 66, 0, 45, 0.7, 0.0, 3.0, 23.5, Some(0.0), Some(1.7), Some(28.9), 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(boma, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    sv_animcmd::frame(lua_state, 17.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(boma, 0, false);
        AttackModule::clear(boma, 1, false);
    }
    sv_animcmd::frame(lua_state, 18.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "lucina", scripts = [ "game_speciallw", "game_specialairlw" ], category = ACMD_GAME, low_priority )]
unsafe fn lucina_dspecial(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    SP_GAUGE[get_player_number(boma)] -= SPENT_SP[get_player_number(boma)];
}

#[acmd_script( agent = "lucina", scripts = [ "game_speciallwhit", "game_specialairlwhit" ], category = ACMD_GAME, low_priority )]
unsafe fn lucina_dspecialhit(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    if shadow_id(boma) {
        macros::FT_START_CUTIN(fighter);
        macros::SLOW_OPPONENT(fighter, 20.0, 8.0);
        SHADOW_FRENZY[get_player_number(boma)] = true;
    }
    sv_animcmd::frame(lua_state, 5.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("sword1"), 3.4, 90, 0, 60, 40, 5.0, 1.0, 0.0, 1.5, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 3.4, 90, 0, 60, 40, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("colonells"), 3.4, 90, 0, 60, 40, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("sword1"), 3.4, 90, 0, 60, 40, 5.0, 1.0, 0.0, 7.0, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        AttackModule::set_force_reaction(boma, 0, true, false);
        AttackModule::set_force_reaction(boma, 1, true, false);
        AttackModule::set_force_reaction(boma, 2, true, false);
        AttackModule::set_force_reaction(boma, 3, true, false);
        if WorkModule::is_flag(boma, *FIGHTER_MARTH_STATUS_SPECIAL_LW_FLAG_SPECIAL_EFFECT) {
            AttackModule::set_optional_hit_effect(boma, 0, Hash40::new("se_lucina_criticalhit"));
            AttackModule::set_optional_hit_effect(boma, 1, Hash40::new("se_lucina_criticalhit"));
            AttackModule::set_optional_hit_effect(boma, 2, Hash40::new("se_lucina_criticalhit"));
            AttackModule::set_optional_hit_effect(boma, 3, Hash40::new("se_lucina_criticalhit"));
        }
    }
    sv_animcmd::frame(lua_state, 8.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "lucina", script = "game_specialhi", category = ACMD_GAME, low_priority )]
unsafe fn lucina_uspecial(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    if macros::is_excute(fighter) {
        upper_invuln(boma, true);
    }
    sv_animcmd::frame(lua_state, 1.0);
    macros::FT_MOTION_RATE(fighter, 2.0);
    sv_animcmd::frame(lua_state, 3.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL)
    && spent_meter(boma, false) {
        SP_GAUGE[get_player_number(boma)] -= SPENT_SP[get_player_number(boma)];
        special_effect(boma);
        IS_EX[get_player_number(boma)] = true;
    }
    else {
        IS_EX[get_player_number(boma)] = false;
    }
    if IS_EX[get_player_number(boma)] {
        upper_invuln(boma, false);
        full_invuln(boma, true);
        sv_animcmd::frame(lua_state, 5.0);
        macros::FT_MOTION_RATE(fighter, 6.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.5, 87, 100, 160, 0, 5.1, 0.0, 11.0, 10.0, Some(0.0), Some(7.0), Some(10.0), 1.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
            WorkModule::on_flag(boma, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_SPECIAL_HI_SET_LR);
            WorkModule::on_flag(boma, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE);
        }
        sv_animcmd::wait(lua_state, 1.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        sv_animcmd::wait(lua_state, 1.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.1, 367, 100, 85, 0, 6.0, 0.0, 20.0, 10.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.1, 367, 100, 85, 0, 5.3, 0.0, 0.0, 10.0, Some(0.0), Some(17.0), Some(10.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
        }
        sv_animcmd::frame(lua_state, 8.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(boma);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 80, 75, 0, 75, 6.5, 0.0, 12.0, 10.0, Some(0.0), Some(21.0), Some(10.0), 2.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        }
        sv_animcmd::frame(lua_state, 9.0);
        if macros::is_excute(fighter) {
            full_invuln(boma, false);
        }
        macros::FT_MOTION_RATE(fighter, 1.0);
        sv_animcmd::frame(lua_state, 11.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
    }
    else{
        sv_animcmd::frame(lua_state, 5.0);
        macros::FT_MOTION_RATE(fighter, 3.5);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 7.0, 79, 90, 0, 70, 4.0, 0.0, 8.0, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 7.0, 79, 90, 0, 70, 4.0, 0.0, 8.0, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            WorkModule::on_flag(boma, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_SPECIAL_HI_SET_LR);
            WorkModule::on_flag(boma, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE);
        }
        sv_animcmd::frame(lua_state, 6.0);
        if macros::is_excute(fighter) {
            upper_invuln(boma, false);
            macros::ATTACK(fighter, 0, 0, Hash40::new("sword1"), 7.0, 79, 90, 0, 20, 5.0, 0.0, 0.0, 4.5, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("sword1"), 7.0, 79, 90, 0, 20, 5.0, 0.0, 0.0, -1.5, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        sv_animcmd::frame(lua_state, 7.0);
        macros::FT_MOTION_RATE(fighter, 1.0);
        if macros::is_excute(fighter) {
            AttackModule::clear(boma, 2, false);
            AttackModule::clear(boma, 3, false);
            smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        }
        sv_animcmd::frame(lua_state, 12.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
    }
}

#[acmd_script( agent = "lucina", script = "game_specialairhi", category = ACMD_GAME, low_priority )]
unsafe fn lucina_uspecialair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state, 1.0);
    macros::FT_MOTION_RATE(fighter, 2.0);
    sv_animcmd::frame(lua_state, 3.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL)
    && spent_meter(boma, false) {
        SP_GAUGE[get_player_number(boma)] -= SPENT_SP[get_player_number(boma)];
        special_effect(boma);
        IS_EX[get_player_number(boma)] = true;
    }
    else {
        IS_EX[get_player_number(boma)] = false;
    }
    if IS_EX[get_player_number(boma)] {
        full_invuln(boma, true);
        sv_animcmd::frame(lua_state, 5.0);
        macros::FT_MOTION_RATE(fighter, 6.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.5, 87, 100, 140, 0, 5.1, 0.0, 11.0, 10.0, Some(0.0), Some(7.0), Some(10.0), 1.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
            WorkModule::on_flag(boma, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_SPECIAL_HI_SET_LR);
            WorkModule::on_flag(boma, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE);
        }
        sv_animcmd::wait(lua_state, 1.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        sv_animcmd::wait(lua_state, 1.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.1, 367, 50, 10, 0, 6.0, 0.0, 20.0, 10.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.1, 367, 50, 10, 0, 5.3, 0.0, 0.0, 10.0, Some(0.0), Some(17.0), Some(10.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
        }
        sv_animcmd::frame(lua_state, 8.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(boma);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 80, 75, 0, 75, 6.5, 0.0, 12.0, 10.0, Some(0.0), Some(21.0), Some(10.0), 2.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        }
        sv_animcmd::frame(lua_state, 9.0);
        if macros::is_excute(fighter) {
            full_invuln(boma, false);
        }
        macros::FT_MOTION_RATE(fighter, 1.0);
        sv_animcmd::frame(lua_state, 11.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
    }
    else{
        upper_invuln(boma, true);
        sv_animcmd::frame(lua_state, 5.0);
        macros::FT_MOTION_RATE(fighter, 3.5);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 7.0, 79, 90, 0, 70, 4.0, 0.0, 8.0, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 7.0, 79, 90, 0, 70, 4.0, 0.0, 8.0, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            WorkModule::on_flag(boma, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_SPECIAL_HI_SET_LR);
            WorkModule::on_flag(boma, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE);
        }
        sv_animcmd::frame(lua_state, 6.0);
        if macros::is_excute(fighter) {
            upper_invuln(boma, false);
            macros::ATTACK(fighter, 0, 0, Hash40::new("sword1"), 7.0, 79, 90, 0, 20, 5.0, 0.0, 0.0, 4.5, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("sword1"), 7.0, 79, 90, 0, 20, 5.0, 0.0, 0.0, -1.5, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        sv_animcmd::frame(lua_state, 7.0);
        macros::FT_MOTION_RATE(fighter, 1.0);
        if macros::is_excute(fighter) {
            AttackModule::clear(boma, 2, false);
            AttackModule::clear(boma, 3, false);
            smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        }
        sv_animcmd::frame(lua_state, 12.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
    }
}

// #[acmd_script( agent = "lucina", script = "effect_run", category = ACMD_EFFECT, low_priority )]
// unsafe fn lucina_runeffect(fighter: &mut L2CAgentBase) {
//     let lua_state = fighter.lua_state_agent;
//     macros::wait_loop_sync_mot(fighter);
//     sv_animcmd::frame(lua_state, 4.0);
//     if macros::is_excute(fighter) {
//         macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
//     }
//     sv_animcmd::frame(lua_state, 21.0);
//     if macros::is_excute(fighter) {
//         macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
//     }
//     sv_animcmd::frame(lua_state, 37.0);
//     if macros::is_excute(fighter) {
//         macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
//     }
//     sv_animcmd::frame(lua_state, 56.0);
// }

// #[acmd_script( agent = "lucina", script = "sound_run", category = ACMD_SOUND, low_priority )]
// unsafe fn lucina_runsound(fighter: &mut L2CAgentBase) {
//     let lua_state = fighter.lua_state_agent;
//     macros::wait_loop_sync_mot(fighter);
//     sv_animcmd::frame(lua_state, 5.0);
//     if macros::is_excute(fighter) {
//         macros::PLAY_STEP(fighter, Hash40::new("se_lucina_step_right_l"));
//     }
//     sv_animcmd::wait(lua_state, 17.0);
//     if macros::is_excute(fighter) {
//         macros::PLAY_STEP(fighter, Hash40::new("se_lucina_step_right_l"));
//     }
//     sv_animcmd::wait(lua_state, 17.0);
//     if macros::is_excute(fighter) {
//         macros::PLAY_STEP(fighter, Hash40::new("se_lucina_step_right_l"));
//     }
//     sv_animcmd::wait(lua_state, 17.0);
// }

pub fn install() {
    smashline::install_agent_frames!(
        lucina_frame
    );
    smashline::install_acmd_scripts!(
        lucina_jab1,
        lucina_jab1eff,
        lucina_jab2,
        lucina_jab2eff,
        lucina_utilt,
        lucina_dtilt,
        lucina_dashattack,
        lucina_dashattacksound,
        lucina_nair,
        lucina_fair,
        lucina_bair,
        lucina_uair,
        lucina_dair,
        lucina_fsmash,
        lucina_usmash,
        lucina_nspecialend,
        lucina_nspecialendmax,
        lucina_sspecial1,
        lucina_sspecial1air,
        lucina_sspecial2lwair,
        lucina_sspecial2hiair,
        lucina_dspecial,
        lucina_dspecialhit,
        lucina_uspecial,
        lucina_uspecialair,
        // lucina_runeffect,
        // lucina_runsound
    );
    // skyline::install_hook!(lucina_is_enable_transition_term_replace);
}