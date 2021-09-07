use smash::{
    lua2cpp::{L2CFighterCommon, L2CAgentBase},
    hash40,
    phx::{Hash40, Vector3f, Vector2f},
    app::{lua_bind::*, sv_animcmd::*, *},
    lib::{lua_const::*, L2CValue}
};
use smash_script::*;
use smashline::*;
use crate::{
    commonfuncs::*,
    globals::*,
    gameplay::*,
    vars::*
};

// For article stuff
// if macros::is_excute(fighter) {
//     ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LUCINA_GENERATE_ARTICLE_MASK, false, 0);
//     ArticleModule::have(fighter.module_accessor, *FIGHTER_LUCINA_GENERATE_ARTICLE_MASK, Hash40::new("havel"), ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), 0, true);
//     ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LUCINA_GENERATE_ARTICLE_MASK, Hash40::new("appeal_lw"), false, -1.0);
// }

#[inline(always)]
pub unsafe fn spent_meter(module_accessor: *mut BattleObjectModuleAccessor, onemore: bool) -> bool {
    let mut spent = false;
    if SP_GAUGE[entry_id(module_accessor)] > 0.0 {
        if SHADOW_FRENZY[entry_id(module_accessor)] {
            if onemore {
                SPENT_SP[entry_id(module_accessor)] = 12.5;
                spent = true;
                SP_GAUGE_TIMER[entry_id(module_accessor)] = 600;
            }
            else {
                SPENT_SP[entry_id(module_accessor)] = 6.25;
                spent = true;
                SP_GAUGE_TIMER[entry_id(module_accessor)] = 600;
            }
        }
        else if SP_GAUGE[entry_id(module_accessor)] >= 25.0 {
            SPENT_SP[entry_id(module_accessor)] = 25.0;
            spent = true;
            SP_GAUGE_TIMER[entry_id(module_accessor)] = 300;
        }
    }
    if spent {
        METER_PENALTY[entry_id(module_accessor)] = 600.0;
    }
    return spent;
}

#[inline(always)]
pub unsafe fn upper_invuln(module_accessor: *mut BattleObjectModuleAccessor, is_invuln: bool) {
    if is_invuln {
        HitModule::set_status_joint(module_accessor, Hash40::new("waist"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(module_accessor, Hash40::new("hip"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(module_accessor, Hash40::new("head"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(module_accessor, Hash40::new("shoulderr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(module_accessor, Hash40::new("shoulderl"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(module_accessor, Hash40::new("arml"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
    }
    else {
        HitModule::set_status_joint(module_accessor, Hash40::new("waist"), HitStatus(*HIT_STATUS_NORMAL), 0);
        HitModule::set_status_joint(module_accessor, Hash40::new("hip"), HitStatus(*HIT_STATUS_NORMAL), 0);
        HitModule::set_status_joint(module_accessor, Hash40::new("head"), HitStatus(*HIT_STATUS_NORMAL), 0);
        HitModule::set_status_joint(module_accessor, Hash40::new("shoulderr"), HitStatus(*HIT_STATUS_NORMAL), 0);
        HitModule::set_status_joint(module_accessor, Hash40::new("shoulderl"), HitStatus(*HIT_STATUS_NORMAL), 0);
        HitModule::set_status_joint(module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_NORMAL), 0);
        HitModule::set_status_joint(module_accessor, Hash40::new("arml"), HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

#[inline(always)]
pub unsafe fn full_invuln(module_accessor: *mut BattleObjectModuleAccessor, is_invuln: bool) {
    if is_invuln {
        HitModule::set_whole(module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
    }
    else {
        HitModule::set_whole(module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

#[inline(always)]
pub unsafe fn shadow_id(module_accessor: *mut BattleObjectModuleAccessor) -> bool {
    if WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 6
    || WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7 {
        return true;
    }
    else {
        return false;
    }
}

#[inline(always)]
pub unsafe fn sp_glow_handler(module_accessor: *mut BattleObjectModuleAccessor) {
    let onemoreeff: u32 = EffectModule::req_follow(module_accessor, Hash40::new("sys_damage_elec"), smash::phx::Hash40::new("handr"), &Vector3f {x: 1.0, y: 0.0, z: 0.0}, &ZERO_VECTOR, 0.3, true, 0, 0, 0, 0, 0, true, true) as u32;
    let onemoreeff2: u32 = EffectModule::req_follow(module_accessor, Hash40::new("sys_damage_elec"), smash::phx::Hash40::new("handl"), &Vector3f {x: 1.0, y: 0.0, z: 0.0}, &ZERO_VECTOR, 0.3, true, 0, 0, 0, 0, 0, true, true) as u32;
    EffectModule::set_rate(module_accessor, onemoreeff, 2.0);
    EffectModule::set_rate(module_accessor, onemoreeff2, 2.0);
    if SHADOW_FRENZY[entry_id(module_accessor)] {
        EffectModule::set_rgb(module_accessor, onemoreeff, 0.6, 0.0, 1.0);
        EffectModule::set_rgb(module_accessor, onemoreeff2, 0.6, 0.0, 1.0);
    }
    else {
        EffectModule::set_rgb(module_accessor, onemoreeff, 1.0, 0.8, 0.0);
        EffectModule::set_rgb(module_accessor, onemoreeff2, 1.0, 0.8, 0.0);
    }
}

#[inline(always)]
pub unsafe fn sp_gauge_handler(module_accessor: *mut BattleObjectModuleAccessor, remove: bool) {
    EffectModule::kill_kind(module_accessor, Hash40::new("sys_starrod_bullet"), false, true);
    if !remove {
        let mut level = SP_LEVEL[entry_id(module_accessor)];
        if SHADOW_FRENZY[entry_id(module_accessor)] {
            level += 1;
        }
        while level > 0 {
            let pos;
            match level {
                2 => pos = SP_2,
                3 => pos = SP_3,
                4 => pos = SP_4,
                5 => pos = SP_5,
                6 => pos = SP_6,
                _ => pos = SP_1,
            }
            EffectModule::req_follow(module_accessor, Hash40::new("sys_starrod_bullet"), Hash40::new("top"), &pos, &ZERO_VECTOR, 0.3, false, 0, 0, 0, 0, 0, false, false);
            level -= 1;
        }
    }
}

#[inline(always)]
pub unsafe fn sp_diff_checker(module_accessor: *mut BattleObjectModuleAccessor) {
    if SP_GAUGE[entry_id(module_accessor)] < 25.0 {
        SP_LEVEL[entry_id(module_accessor)] = 0;
    }
    else {
        while SP_LEVEL[entry_id(module_accessor)] < 6 {
            if SP_GAUGE[entry_id(module_accessor)] >= SP_LEVEL[entry_id(module_accessor)] as f32 * 25.0
            && SP_LEVEL[entry_id(module_accessor)] as f32 * 25.0 > SP_GAUGE[entry_id(module_accessor)] {
                break;
            }
            SP_LEVEL[entry_id(module_accessor)] += 1;
        }
    }
    if SHADOW_FRENZY[entry_id(module_accessor)] {
        SP_GAUGE_TIMER[entry_id(module_accessor)] = 600;
    }
    else {
        SP_GAUGE_TIMER[entry_id(module_accessor)] = 300;
    }
    SP_LEVEL[entry_id(module_accessor)] = (SP_GAUGE[entry_id(module_accessor)] / 25.0) as i32;
    if SP_LEVEL[entry_id(module_accessor)] == 0 && !SHADOW_FRENZY[entry_id(module_accessor)] {
        sp_gauge_handler(module_accessor, true);
    }
    else {
        sp_gauge_handler(module_accessor, false);
    }
}

#[fighter_frame( agent = FIGHTER_KIND_LUCINA )]
fn lucina_frame(fighter: &mut L2CFighterCommon) {
    unsafe {

        // Reset Vars
        
        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH {
            AIR_ACTION[entry_id(fighter.module_accessor)] = false;
            _TIME_COUNTER[entry_id(fighter.module_accessor)] = 0;
            SP_GAUGE_TIMER[entry_id(fighter.module_accessor)] = 0;
            if shadow_id(fighter.module_accessor) {
                if SHADOW_FRENZY[entry_id(fighter.module_accessor)] {
                    SP_GAUGE[entry_id(fighter.module_accessor)] = SP_GAUGE[entry_id(fighter.module_accessor)] / 2.0;
                    SHADOW_FRENZY[entry_id(fighter.module_accessor)] = false;
                }
            }
            else {
                SP_GAUGE_MAX[entry_id(fighter.module_accessor)] = 100.0;
                SP_GAUGE[entry_id(fighter.module_accessor)] = 0.0;
                AWAKENING[entry_id(fighter.module_accessor)] = false;
            }
        }
        if sv_information::is_ready_go() == false {
            DamageModule::set_damage_mul(fighter.module_accessor, 1.0);
        }

        // Meter Controller

        DAMAGE_TAKEN[entry_id(fighter.module_accessor)] = DamageModule::damage(fighter.module_accessor, 0);
        if DAMAGE_TAKEN[entry_id(fighter.module_accessor)] > DAMAGE_TAKEN_PREV[entry_id(fighter.module_accessor)]
        && SHADOW_FRENZY[entry_id(fighter.module_accessor)] == false {
            SP_GAUGE[entry_id(fighter.module_accessor)] += (DAMAGE_TAKEN[entry_id(fighter.module_accessor)] - DAMAGE_TAKEN_PREV[entry_id(fighter.module_accessor)]) * (1.0/6.0);
        }
        DAMAGE_TAKEN_PREV[entry_id(fighter.module_accessor)] = DAMAGE_TAKEN[entry_id(fighter.module_accessor)];

        if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
        && SHADOW_FRENZY[entry_id(fighter.module_accessor)] == false {
            if StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END_MAX
            && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_SPECIAL_S
            && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2
            && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_SPECIAL_HI {
                IS_EX[entry_id(fighter.module_accessor)] = false;
            }
            if IS_EX[entry_id(fighter.module_accessor)] == false {
                METER_GAIN[entry_id(fighter.module_accessor)] = AttackModule::get_power(fighter.module_accessor, 0, false, 1.0, false);
                if shadow_id(fighter.module_accessor) == false {
                    METER_GAIN[entry_id(fighter.module_accessor)] *= 0.75;
                }
                if IS_FUNNY[entry_id(fighter.module_accessor)] {
                    METER_GAIN[entry_id(fighter.module_accessor)] *= 3.0;
                }
                if METER_PENALTY[entry_id(fighter.module_accessor)] > 0.0 {
                    METER_GAIN[entry_id(fighter.module_accessor)] *= 0.1;
                }
                SP_GAUGE[entry_id(fighter.module_accessor)] += METER_GAIN[entry_id(fighter.module_accessor)];
            }
        }

        if SP_GAUGE[entry_id(fighter.module_accessor)] > SP_GAUGE_MAX[entry_id(fighter.module_accessor)] {
            SP_GAUGE[entry_id(fighter.module_accessor)] = SP_GAUGE_MAX[entry_id(fighter.module_accessor)];
        }

        if SP_GAUGE[entry_id(fighter.module_accessor)] >= 25.0
        || SHADOW_FRENZY[entry_id(fighter.module_accessor)] == true {
            if _TIME_COUNTER[entry_id(fighter.module_accessor)] == 0 {
                sp_glow_handler(fighter.module_accessor);
                _TIME_COUNTER[entry_id(fighter.module_accessor)] = 4;
            }
            _TIME_COUNTER[entry_id(fighter.module_accessor)] -= 1;
        }

        // Training Mode Tools

        if smashball::is_training_mode(){
            if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L) {
                if SP_GAUGE[entry_id(fighter.module_accessor)] > 25.0 {
                    SP_GAUGE[entry_id(fighter.module_accessor)] -= 25.0
                }
                else {
                    SP_GAUGE[entry_id(fighter.module_accessor)] = 0.0;
                }
            }
            if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
                if SP_GAUGE[entry_id(fighter.module_accessor)] < SP_GAUGE_MAX[entry_id(fighter.module_accessor)] - 25.0 {
                    SP_GAUGE[entry_id(fighter.module_accessor)] += 25.0
                }
                else {
                    SP_GAUGE[entry_id(fighter.module_accessor)] = SP_GAUGE_MAX[entry_id(fighter.module_accessor)];
                }
            }
            if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) {
                if TRAINING_TOOLS[entry_id(fighter.module_accessor)] {
                    TRAINING_TOOLS[entry_id(fighter.module_accessor)] = false;
                    let pos = Vector3f{x: 0.0, y: 13.0, z: 0.0};
                    let rot = Vector3f{x: 0.0, y: 90.0, z: 0.0};
                    let onemoreeff: u32 = EffectModule::req_follow(fighter.module_accessor, smash::phx::Hash40{hash: hash40("sys_counter_flash")}, smash::phx::Hash40{hash: hash40("top")}, &pos, &rot, 1.0, false, 0, 0, 0, 0, 0, false, false) as u32;
                    EffectModule::set_rgb(fighter.module_accessor, onemoreeff, 0.0, 0.0, 5.0);
                }
                else {
                    TRAINING_TOOLS[entry_id(fighter.module_accessor)] = true;
                    let pos = Vector3f{x: 0.0, y: 13.0, z: 0.0};
                    let rot = Vector3f{x: 0.0, y: 90.0, z: 0.0};
                    let onemoreeff: u32 = EffectModule::req_follow(fighter.module_accessor, smash::phx::Hash40{hash: hash40("sys_counter_flash")}, smash::phx::Hash40{hash: hash40("top")}, &pos, &rot, 1.0, false, 0, 0, 0, 0, 0, false, false) as u32;
                    EffectModule::set_rgb(fighter.module_accessor, onemoreeff, 5.0, 0.0, 0.0);
                }
            }
        }

        if SP_LEVEL[entry_id(fighter.module_accessor)] != (SP_GAUGE[entry_id(fighter.module_accessor)] / 25.0) as i32 {
            sp_diff_checker(fighter.module_accessor);
        }

        if METER_PENALTY[entry_id(fighter.module_accessor)] > 0.0 {
            METER_PENALTY[entry_id(fighter.module_accessor)] -= 1.0;
        }

        if SP_GAUGE_TIMER[entry_id(fighter.module_accessor)] > 0 {
            SP_GAUGE_TIMER[entry_id(fighter.module_accessor)] -= 1;
            if SP_GAUGE_TIMER[entry_id(fighter.module_accessor)] == 0 {
                sp_gauge_handler(fighter.module_accessor, true);
            }
        }

        // Normal vs Shadow Effects

        if shadow_id(fighter.module_accessor) == true {
            DamageModule::set_damage_mul(fighter.module_accessor, 0.92);
            DMG_RATIO[entry_id(fighter.module_accessor)] = 0.8;
            if SHADOW_FRENZY[entry_id(fighter.module_accessor)] == true {
                if !TRAINING_TOOLS[entry_id(fighter.module_accessor)] {
                    if IS_FUNNY[entry_id(fighter.module_accessor)] {
                        SP_GAUGE[entry_id(fighter.module_accessor)] -= 1.0/64.0;
                    }
                    else {
                        SP_GAUGE[entry_id(fighter.module_accessor)] -= 1.0/16.0;
                    }
                }
            }
            if SoundModule::is_playing(fighter.module_accessor, Hash40::new("vc_lucina_missfoot01")) {
                SoundModule::stop_se(fighter.module_accessor, Hash40::new("vc_lucina_missfoot01"), 0);
                macros::PLAY_SE(fighter, Hash40::new("vc_shadow_missfoot01"));
            }
            if SoundModule::is_playing(fighter.module_accessor, Hash40::new("vc_lucina_missfoot02")) {
                SoundModule::stop_se(fighter.module_accessor, Hash40::new("vc_lucina_missfoot02"), 0);
                macros::PLAY_SE(fighter, Hash40::new("vc_shadow_missfoot02"));
            }
            if SoundModule::is_playing(fighter.module_accessor, Hash40::new("vc_lucina_damage_twinkle")) {
                SoundModule::stop_se(fighter.module_accessor, Hash40::new("vc_lucina_damage_twinkle"), 0);
                macros::PLAY_SE(fighter, Hash40::new("vc_shadow_damage_twinkle"));
            }
            if SoundModule::is_playing(fighter.module_accessor, Hash40::new("vc_lucina_knockout")) {
                SoundModule::stop_se(fighter.module_accessor, Hash40::new("vc_lucina_knockout"), 0);
                macros::PLAY_SE(fighter, Hash40::new("vc_shadow_knockout"));
            }
        }
        else {
            DMG_RATIO[entry_id(fighter.module_accessor)] = 1.0;
            if DamageModule::damage(fighter.module_accessor, 0) >= 100.0 {
                if AWAKENING[entry_id(fighter.module_accessor)] == false
                && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND
                && (!is_damage_check(fighter.module_accessor)
                || IS_FUNNY[entry_id(fighter.module_accessor)])
                && sv_information::is_ready_go() {
                    DamageModule::set_damage_mul(fighter.module_accessor, 0.8);
                    SP_GAUGE_MAX[entry_id(fighter.module_accessor)] = 150.0;
                    SP_GAUGE[entry_id(fighter.module_accessor)] += 50.0;
                    AWAKENING[entry_id(fighter.module_accessor)] = true;
                    macros::FT_START_CUTIN(fighter);
                }
            }
        }

        if SP_GAUGE[entry_id(fighter.module_accessor)] <= 0.0{
            SP_GAUGE[entry_id(fighter.module_accessor)] = 0.0;
            SHADOW_FRENZY[entry_id(fighter.module_accessor)] = false;
        }

        // Special Lw Check

        if StatusModule::prev_status_kind(fighter.module_accessor, 0) != StatusModule::status_kind(fighter.module_accessor) {
            CAN_ONE_MORE[entry_id(fighter.module_accessor)] = false;
        }

        if StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_SPECIAL_LW {
            if (AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
            || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD))
            && MotionModule::motion_kind(fighter.module_accessor) != hash40("catch_attack")
            && (MotionModule::motion_kind(fighter.module_accessor) != hash40("special_hi") || IS_FUNNY[entry_id(fighter.module_accessor)]) {
                CAN_ONE_MORE[entry_id(fighter.module_accessor)] = true;
            }

            if CAN_ONE_MORE[entry_id(fighter.module_accessor)] == true {
                if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW != 0 {
                    if spent_meter(fighter.module_accessor, true) {
                        ROMAN_ON_HIT[entry_id(fighter.module_accessor)] = true;
                        fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), false.into());
                        sp_diff_checker(fighter.module_accessor);
                    }
                }
                else if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI)
                && SP_GAUGE[entry_id(fighter.module_accessor)] == 100.0
                && SHADOW_FRENZY[entry_id(fighter.module_accessor)] == false
                && shadow_id(fighter.module_accessor) {
                    fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT.into(), false.into());
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT, true);
                }
            }
            else if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_THROW {
                let throwframe : f32;
                if MotionModule::motion_kind(fighter.module_accessor) == hash40("throw_f") {
                    throwframe = 18.0;
                }
                else if MotionModule::motion_kind(fighter.module_accessor) == hash40("throw_b") {
                    throwframe = 19.0;
                }
                else if MotionModule::motion_kind(fighter.module_accessor) == hash40("throw_hi") {
                    throwframe = 13.0;
                }
                else if MotionModule::motion_kind(fighter.module_accessor) == hash40("throw_lw") {
                    throwframe = 20.0;
                }
                else{
                    throwframe = 20.0;
                }
                if MotionModule::frame(fighter.module_accessor) > throwframe
                && CAN_ONE_MORE[entry_id(fighter.module_accessor)] == false {
                    CAN_ONE_MORE[entry_id(fighter.module_accessor)] = true;
                }
                if CAN_ONE_MORE[entry_id(fighter.module_accessor)] == true {
                    if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW != 0 {
                        if spent_meter(fighter.module_accessor, true) {
                            ROMAN_ON_HIT[entry_id(fighter.module_accessor)] = true;
                            fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), false.into());
                            sp_diff_checker(fighter.module_accessor);
                        }
                    }
                    else if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI)
                    && SP_GAUGE[entry_id(fighter.module_accessor)] == 100.0
                    && SHADOW_FRENZY[entry_id(fighter.module_accessor)] == false
                    && shadow_id(fighter.module_accessor) {
                        fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT.into(), false.into());
                    }
                }
            }
            else if (!is_damage_check(fighter.module_accessor)
            && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_SPECIAL_HI)
            || IS_FUNNY[entry_id(fighter.module_accessor)] {
                if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW != 0 {
                    if spent_meter(fighter.module_accessor, true) {
                        ROMAN_ON_HIT[entry_id(fighter.module_accessor)] = false;
                        fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), false.into());
                        sp_diff_checker(fighter.module_accessor);
                    }
                }
            }
        }

        if SP_FLASH[entry_id(fighter.module_accessor)] > 0 {
            if SP_FLASH[entry_id(fighter.module_accessor)] % 10 == 0 {
                if SHADOW_FRENZY[entry_id(fighter.module_accessor)] {
                    macros::FLASH(fighter, 0.4, 0.0, 1.0, 1.0);
                }
                else {
                    macros::FLASH(fighter, 1.0, 1.0, 0.0, 0.75);
                }
            }
            else if SP_FLASH[entry_id(fighter.module_accessor)] % 5 == 0 {
                macros::COL_NORMAL(fighter);
            }
            SP_FLASH[entry_id(fighter.module_accessor)] -= 1;
        }

        // Air Action Reset

        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND
        || StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_CLIFF {
            AIR_ACTION[entry_id(fighter.module_accessor)] = false;
        }

        // Shadow Frenzy Check

        if MotionModule::motion_kind(fighter.module_accessor) == hash40("appeal_hi_l") || MotionModule::motion_kind(fighter.module_accessor) == hash40("appeal_hi_r") {
            if SP_GAUGE[entry_id(fighter.module_accessor)] == 100.0 && shadow_id(fighter.module_accessor) && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                SHADOW_FRENZY[entry_id(fighter.module_accessor)] = true;
            }
        }
        else if SP_GAUGE[entry_id(fighter.module_accessor)] == 0.0 {
            SHADOW_FRENZY[entry_id(fighter.module_accessor)] = false;
        }

        // Move Effects

        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_12") {
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
            || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP)
                && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
                && get_command_stick_direction(fighter.module_accessor, true) == 6 {
                    StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), true);
                    fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_S.into(), false.into());
                }
                if QCB[entry_id(fighter.module_accessor)] == 3
                && (ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
                || ControlModule::check_button_on_release(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)) {
                    fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_DASH.into(), false.into());
                }
                if QCB[entry_id(fighter.module_accessor)] != 3
                && (ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
                || ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)) {
                    CancelModule::enable_cancel(fighter.module_accessor);
                }
            }
        }

        // if IS_FGC[entry_id(fighter.module_accessor)] {
        //     if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW == 0 {
        //         if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
        //         || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
        //             if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_LW3 {
        //                 if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3 != 0 {
        //                     fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_HI3.into(), false.into());
        //                 }
        //             }
        //             if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_f") {
        //                 if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
        //                 || (ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
        //                 && get_command_stick_direction(fighter.module_accessor, true) != 6) {
        //                     CancelModule::enable_cancel(fighter.module_accessor);
        //                 }
        //             }
        //             if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_hi") {
        //                 if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
        //                 || (ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
        //                 && get_command_stick_direction(fighter.module_accessor, true) != 8) {
        //                     CancelModule::enable_cancel(fighter.module_accessor);
        //                 }
        //             }
        //             if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_b") {
        //                 if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
        //                 || (ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
        //                 && get_command_stick_direction(fighter.module_accessor, true) != 4) {
        //                     CancelModule::enable_cancel(fighter.module_accessor);
        //                 }
        //             }
        //             if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_lw") {
        //                 if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
        //                 || (ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
        //                 && get_command_stick_direction(fighter.module_accessor, true) != 2) {
        //                     CancelModule::enable_cancel(fighter.module_accessor);
        //                 }
        //             }
        //             if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_n") {
        //                 if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
        //                 || ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        //                     CancelModule::enable_cancel(fighter.module_accessor);
        //                 }
        //             }
        //         }
        //     }
        // }

        if HEROIC_GRAB[entry_id(fighter.module_accessor)] {
            if MotionModule::motion_kind(fighter.module_accessor) == hash40("catch_wait")
            || StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_CATCH_ATTACK {
                fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), true.into());
            }
            if MotionModule::motion_kind(fighter.module_accessor) == hash40("throw_hi")
            && MotionModule::frame(fighter.module_accessor) > 22.0 {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_S3, true);
            }
        }
        if StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_CATCH_WAIT
        && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_CATCH_DASH
        && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_CATCH_TURN
        && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_CATCH_PULL
        && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_CATCH_JUMP
        && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_CATCH_CUT
        && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_CATCH
        && CatchModule::is_catch(fighter.module_accessor) == false
        && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_THROW {
            HEROIC_GRAB[entry_id(fighter.module_accessor)] = false;
        }

        // Jump Cancels

        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_hi3")
        || (MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_dash")
        && IS_FUNNY[entry_id(fighter.module_accessor)])
        || MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_n")
        || MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_hi")
        || MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_lw") {
            jump_cancel_check_hit(fighter, false);
        }
    }
}

#[status_script(agent = "lucina", status = FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_LOOP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn lucina_specialnloop_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        AIR_ACTION[entry_id(fighter.module_accessor)] = true;
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_loop"), 1.0, 1.0, false, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_loop"), 1.0, 1.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(lucina_specialnloop_loop as *const () as _))
}

unsafe extern "C" fn lucina_specialnloop_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
    && spent_meter(fighter.module_accessor, false) {
        SP_GAUGE[entry_id(fighter.module_accessor)] -= SPENT_SP[entry_id(fighter.module_accessor)];
        SP_FLASH[entry_id(fighter.module_accessor)] = 40;
        IS_EX[entry_id(fighter.module_accessor)] = true;
        fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END_MAX.into(), false.into());
        sp_diff_checker(fighter.module_accessor);
    }
    else if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END.into(), false.into());
    }
    else {
        if MotionModule::is_end(fighter.module_accessor) {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
                fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END.into(), false.into());
            }
            else {
                HEROIC_GRAB[entry_id(fighter.module_accessor)] = true;
                fighter.change_status(FIGHTER_STATUS_KIND_CATCH_DASH.into(), false.into());
            }
        }
    }
    L2CValue::I32(0)
}

#[status_script(agent = "lucina", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn lucina_specials_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_CONTINUE_MOT);
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_RESET);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), true);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        START_SITUATION[entry_id(fighter.module_accessor)] = *SITUATION_KIND_AIR;
        AIR_ACTION[entry_id(fighter.module_accessor)] = true;
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s1"), 1.0, 1.0, false, 0.0, false, false);
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        START_SITUATION[entry_id(fighter.module_accessor)] = *SITUATION_KIND_GROUND;
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s1"), 1.0, 1.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(lucina_specials_loop as *const () as _))
}

unsafe extern "C" fn lucina_specials_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if START_SITUATION[entry_id(fighter.module_accessor)] == *SITUATION_KIND_AIR
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
    }
    else if START_SITUATION[entry_id(fighter.module_accessor)] == *SITUATION_KIND_GROUND
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    else if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        if CancelModule::is_enable_cancel(fighter.module_accessor) {
            fighter.sub_wait_ground_check_common(L2CValue::I32(0));
        }
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    else if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        if CancelModule::is_enable_cancel(fighter.module_accessor) {
            fighter.sub_air_check_fall_common();
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE) {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL_RAW)
            || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK_RAW) {
                fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2.into(), false.into());
            }
        }
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    L2CValue::I32(0)
}

#[status_script(agent = "lucina", status = FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn lucina_specials2_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_CONTINUE_MOT);
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK_RAW) {
        START_SITUATION[entry_id(fighter.module_accessor)] = 1;
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s2_lw"), 1.0, 1.0, false, 0.0, false, false);
    }
    else {
        START_SITUATION[entry_id(fighter.module_accessor)] = 0;
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s2_hi"), 1.0, 1.0, false, 0.0, false, false);
    }   
    fighter.sub_shift_status_main(L2CValue::Ptr(lucina_specials2_loop as *const () as _))
}

unsafe extern "C" fn lucina_specials2_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        fighter.sub_wait_ground_check_common(L2CValue::I32(0));
        fighter.sub_air_check_fall_common();
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            if START_SITUATION[entry_id(fighter.module_accessor)] == 0 {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s2_hi"), 1.0, 1.0, false, 0.0, false, false);
            }
            if START_SITUATION[entry_id(fighter.module_accessor)] == 1 {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s2_lw"), 1.0, 1.0, false, 0.0, false, false);
            }
        }
    }
    else {
        if MotionModule::is_end(fighter.module_accessor) {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR
            && START_SITUATION[entry_id(fighter.module_accessor)] == 1 {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
            else if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                if START_SITUATION[entry_id(fighter.module_accessor)] == 1 {
                    fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
                }
                else {
                    fighter.change_status(FIGHTER_STATUS_KIND_SQUAT_WAIT.into(), false.into());
                }
            }
        }
    }
    L2CValue::I32(0)
}

#[status_script(agent = "lucina", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn lucina_speciallw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    IS_ROMAN_MOVE[entry_id(fighter.module_accessor)] = false;
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw"), 0.0, 1.0, false, 0.0, false, false);
    }   
    fighter.sub_shift_status_main(L2CValue::Ptr(lucina_speciallw_loop as *const () as _))
}

unsafe extern "C" fn lucina_speciallw_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        fighter.sub_wait_ground_check_common(L2CValue::I32(0));
        fighter.sub_air_check_fall_common();
    }
    if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
    || AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
        macros::SLOW_OPPONENT(fighter, 10.0, 20.0);
        AttackModule::clear_all(fighter.module_accessor);
    }
    if IS_ROMAN_MOVE[entry_id(fighter.module_accessor)] {
        PostureModule::add_pos_2d(fighter.module_accessor, &Vector2f{
            x: ROMAN_MOVE[entry_id(fighter.module_accessor)],
            y: 0.0
        });
        ROMAN_MOVE[entry_id(fighter.module_accessor)] *= 0.9;
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    L2CValue::I32(0)
}

#[acmd_script( agent = "lucina", script = "game_attack11", category = ACMD_GAME, low_priority )]
unsafe fn lucina_jab1(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.5, 361, 15, 0, 20, 2.0, 0.0, 9.4, 6.2, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.5, 361, 15, 0, 20, 5.0, 0.0, 9.4, 8.8, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART);
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "lucina", script = "effect_attack11", category = ACMD_EFFECT, low_priority )]
unsafe fn lucina_jab1eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 18, -5, 0, 330, 90, 0.95, true);
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), -1, 10, 10, 0, 0, 0, 1.15, 0, 0, 0, 0, 0, 360, true);
    }
}

#[acmd_script( agent = "lucina", script = "game_attack12", category = ACMD_GAME, low_priority )]
unsafe fn lucina_jab2(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 4.5 * DMG_RATIO[entry_id(fighter.module_accessor)], 361, 88, 0, 30, 4.2, 5.0, -1.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 4.5 * DMG_RATIO[entry_id(fighter.module_accessor)], 361, 88, 0, 30, 3.8, -1.0, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "lucina", script = "effect_attack12", category = ACMD_EFFECT, low_priority )]
unsafe fn lucina_jab2eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_line"), Hash40::new("top"), -5, 12, 2, 0, 0, 0, 0.95, true);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), -6, 12, 17, 0, 0, 0, 1.15, 0, 0, 0, 0, 0, 360, true);
    }
}

#[acmd_script( agent = "lucina", script = "game_attacks3", category = ACMD_GAME, low_priority )]
unsafe fn lucina_ftilt(fighter: &mut L2CAgentBase) {
    if HEROIC_GRAB[entry_id(fighter.module_accessor)] {
        macros::FT_MOTION_RATE(fighter, 0.5);
        HEROIC_GRAB[entry_id(fighter.module_accessor)] = false;
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("sword1"), 10.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 361, 75, 0, 50, 3.5, 1.0, 0.0, 2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 10.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 361, 75, 0, 50, 3.0, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("sword1"), 10.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 361, 75, 0, 50, 3.5, 1.0, 0.0, 7.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    macros::FT_MOTION_RATE(fighter, 1.0);
}

#[acmd_script( agent = "lucina", script = "game_attackhi3", category = ACMD_GAME, low_priority )]
unsafe fn lucina_utilt(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("sword1"), 1.5, 100, 60, 40, 50, 2.0, 0.0, -1.0, 1.0, Some(1.0), Some(1.0), Some(12.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("sword1"), 1.5, 100, 60, 40, 50, 2.0, 0.0, -1.0, 1.0, Some(1.0), Some(1.0), Some(12.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        macros::ATTACK(fighter, 0, 0, Hash40::new("sword1"), 5.5 * DMG_RATIO[entry_id(fighter.module_accessor)], 75, 60, 0, 65, 2.0, 0.0, -1.0, 1.0, Some(1.0), Some(0.0), Some(12.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("sword1"), 5.5 * DMG_RATIO[entry_id(fighter.module_accessor)], 75, 60, 0, 65, 2.0, 0.0, -1.0, 1.0, Some(1.0), Some(-1.0), Some(12.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "lucina", script = "game_attacklw3", category = ACMD_GAME, low_priority )]
unsafe fn lucina_dtilt(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 71, 40, 0, 57, 2.7, 0.0, 2.7, 16.700001, Some(0.0), Some(4.3), Some(9.2), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.2, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("sword1"), 6.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 71, 40, 0, 57, 2.7, 0.0, 0.0, 8.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.2, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "lucina", script = "game_attackdash", category = ACMD_GAME, low_priority )]
unsafe fn lucina_dashattack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 7.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 65, 85, 0, 65, 3.6, 5.0, -1.0, 1.5, Some(1.5), Some(-1.0), Some(1.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.2, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 65, 85, 0, 65, 2.5, 0.0, 2.5, -2.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.2, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_XLU);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 1, false);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        KineticModule::resume_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
}

#[acmd_script( agent = "lucina", script = "sound_attackdash", category = ACMD_SOUND, low_priority )]
unsafe fn lucina_dashattacksound(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_lucina_rnd_attack"));
        macros::PLAY_SE(fighter, Hash40::new("se_lucina_swing_ll"));
    }
}

#[acmd_script( agent = "lucina", script = "game_attackairn", category = ACMD_GAME, low_priority )]
unsafe fn lucina_nair(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 7.0);
    macros::FT_MOTION_RATE(fighter, 0.0833333);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 4.5 * DMG_RATIO[entry_id(fighter.module_accessor)], 51, 70, 0, 40, 5.0, 3.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 4.5 * DMG_RATIO[entry_id(fighter.module_accessor)], 51, 70, 0, 40, 5.0, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 2, 0, Hash40::new("kneel"), 4.5 * DMG_RATIO[entry_id(fighter.module_accessor)], 51, 70, 0, 40, 3.0, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 31.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "lucina", script = "effect_attackairn", category = ACMD_EFFECT, low_priority )]
unsafe fn lucina_naireff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_line"), Hash40::new("top"), -2, 9.3, -5.5, 0, 10, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 11, 9.3, -2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, 0.7);
    }
}

#[acmd_script( agent = "lucina", script = "sound_attackairn", category = ACMD_SOUND, low_priority )]
unsafe fn lucina_nairsnd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_punch_kick_swing_m"));
    }
}

#[acmd_script( agent = "lucina", script = "game_attackairf", category = ACMD_GAME, low_priority )]
unsafe fn lucina_fair(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    macros::FT_MOTION_RATE(fighter, 2.0);
    frame(fighter.lua_state_agent, 5.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("sword1"), 10.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 361, 75, 0, 50, 3.0, 1.0, 0.0, 3.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 10.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 361, 75, 0, 50, 3.8, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("sword1"), 10.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 361, 75, 0, 50, 3.0, 1.0, 0.0, 7.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 36.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "lucina", script = "game_attackairb", category = ACMD_GAME, low_priority )]
unsafe fn lucina_bair(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::REVERSE_LR(fighter);
    }
    macros::FT_MOTION_RATE(fighter, 2.0);
    frame(fighter.lua_state_agent, 6.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("sword1"), 10.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 361, 75, 0, 50, 3.5, 0.0, 0.0, 3.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 10.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 361, 75, 0, 50, 3.5, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("sword1"), 10.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 361, 75, 0, 50, 3.5, 0.0, 0.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 32.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "lucina", script = "game_attackairhi", category = ACMD_GAME, low_priority )]
unsafe fn lucina_uair(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("sword1"), 8.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 59, 40, 0, 40, 3.5, 1.0, 0.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 8.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 59, 40, 0, 40, 3.0, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("colonells"), 8.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 59, 40, 0, 40, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("sword1"), 8.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 59, 40, 0, 40, 3.5, 1.0, 0.0, 6.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "lucina", script = "game_attackairlw", category = ACMD_GAME, low_priority )]
unsafe fn lucina_dair(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 8.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 73, 75, 0, 30, 3.0, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("sword1"), 8.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 73, 75, 0, 30, 3.5, 1.0, 0.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("sword1"), 8.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 73, 75, 0, 30, 3.5, 1.0, 0.0, 6.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 55.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "lucina", script = "game_attacks4", category = ACMD_GAME, low_priority )]
unsafe fn lucina_fsmash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 13.5 * DMG_RATIO[entry_id(fighter.module_accessor)], 361, 85, 0, 30, 4.0, 0.0, 8.5, 10.0, Some(0.0), Some(3.5), Some(22.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "lucina", script = "game_attackhi4", category = ACMD_GAME, low_priority )]
unsafe fn lucina_usmash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("sword1"), 12.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 89, 90, 0, 42, 5.8, 0.0, 0.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("sword1"), 12.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 89, 90, 0, 42, 4.6, 0.0, 0.0, 7.3, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 0, 0, Hash40::new("sword1"), 12.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 89, 90, 0, 42, 5.8, 0.0, 0.0, -3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 3.0, 125, 100, 155, 0, 5.5, 0.0, 5.0, 9.0, Some(0.0), Some(5.0), Some(-9.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 0, false);
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "lucina", script = "game_throwhi", category = ACMD_GAME, low_priority )]
unsafe fn lucina_uthrow(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        if HEROIC_GRAB[entry_id(fighter.module_accessor)] {
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 5.0, 60, 40, 20, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
        else {
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 5.0, 93, 102, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        if HEROIC_GRAB[entry_id(fighter.module_accessor)] {
            macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
        }
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::CHECK_FINISH_CAMERA(fighter, 1, 21);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        if HEROIC_GRAB[entry_id(fighter.module_accessor)] == false {
            macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
        }
    }
}

#[acmd_script( agent = "lucina", scripts = [ "game_specialnloop", "game_specialairnloop" ], category = ACMD_GAME, low_priority )]
unsafe fn lucina_nspecialloop(fighter: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(fighter, 1.4);
}

#[acmd_script( agent = "lucina", scripts = [ "game_specialnend", "game_specialnendhi", "game_specialnendlw", "game_specialairnend", "game_specialairnendhi", "game_specialairnendlw" ], category = ACMD_GAME, low_priority )]
unsafe fn lucina_nspecialend(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        frame(fighter.lua_state_agent, 8.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 9.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 361, 20, 0, 95, 3.0, 0.0, 5.0, 8.0, Some(0.0), Some(5.0), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 9.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 361, 20, 0, 95, 3.0, 0.0, 5.0, 25.0, Some(0.0), Some(5.0), Some(27.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 361, 20, 0, 95, 0.9, 0.0, 5.0, 17.0, Some(0.0), Some(5.0), Some(22.0), 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 361, 20, 0, 95, 0.9, 0.0, 5.0, 23.5, Some(0.0), Some(5.0), Some(28.9), 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 9.0);
        if macros::is_excute(fighter) {
            AttackModule::clear(fighter.module_accessor, 0, false);
            AttackModule::clear(fighter.module_accessor, 1, false);
        }
        frame(fighter.lua_state_agent, 10.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
    }
    else {
        frame(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            KineticModule::mul_speed(fighter.module_accessor, &Vector3f {x: 0.4, y: 0.0, z: 0.0}, *FIGHTER_KINETIC_TYPE_UNIQ);
        }
        macros::FT_MOTION_RATE(fighter, 0.125);
        frame(fighter.lua_state_agent, 9.0);
        macros::FT_MOTION_RATE(fighter, 1.0);
        frame(fighter.lua_state_agent, 13.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 361, 20, 0, 95, 3.3, 0.0, 7.7, 9.1, Some(0.0), Some(7.7), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("sword1"), 9.0, 361, 20, 0, 95, 4.0, 2.0, 1.0, 1.5, Some(14.0), Some(1.0), Some(1.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("sword1"), 9.0, 361, 20, 0, 95, 3.5, 2.0, 1.0, 7.5, Some(9.5), Some(1.0), Some(7.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 16.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
    }
}

#[acmd_script( agent = "lucina", scripts = [ "game_specialnendmax", "game_specialnendmaxhi", "game_specialnendmaxlw", "game_specialairnendmax", "game_specialairnendmaxhi", "game_specialairnendmaxlw" ], category = ACMD_GAME, low_priority )]
unsafe fn lucina_nspecialendmax(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        frame(fighter.lua_state_agent, 8.0);
        if macros::is_excute(fighter) {
            if PostureModule::lr(fighter.module_accessor) == 1.0 && ControlModule::get_stick_x(fighter.module_accessor) < -0.75 {
                PostureModule::reverse_lr(fighter.module_accessor);
            }
            else if PostureModule::lr(fighter.module_accessor) == -1.0 && ControlModule::get_stick_x(fighter.module_accessor) > 0.75 {
                PostureModule::reverse_lr(fighter.module_accessor);
            }
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 9.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 60, 80, 0, 30, 3.0, 0.0, 5.0, 8.0, Some(0.0), Some(5.0), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 9.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 60, 80, 0, 30, 3.0, 0.0, 5.0, 25.0, Some(0.0), Some(5.0), Some(27.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 60, 80, 0, 30, 0.9, 0.0, 5.0, 17.0, Some(0.0), Some(5.0), Some(22.0), 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 60, 80, 0, 30, 0.9, 0.0, 5.0, 23.5, Some(0.0), Some(5.0), Some(28.9), 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 9.0);
        if macros::is_excute(fighter) {
            AttackModule::clear(fighter.module_accessor, 0, false);
            AttackModule::clear(fighter.module_accessor, 1, false);
        }
        frame(fighter.lua_state_agent, 10.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
    }
    else{
        frame(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            KineticModule::mul_speed(fighter.module_accessor, &Vector3f {x: 0.4, y: 0.0, z: 0.0}, *FIGHTER_KINETIC_TYPE_UNIQ);
        }
        macros::FT_MOTION_RATE(fighter, 0.125);
        frame(fighter.lua_state_agent, 9.0);
        macros::FT_MOTION_RATE(fighter, 1.25);
        frame(fighter.lua_state_agent, 13.0);
        if macros::is_excute(fighter) {
            if PostureModule::lr(fighter.module_accessor) == 1.0 && ControlModule::get_stick_x(fighter.module_accessor) < -0.75 {
                PostureModule::reverse_lr(fighter.module_accessor);
            }
            else if PostureModule::lr(fighter.module_accessor) == -1.0 && ControlModule::get_stick_x(fighter.module_accessor) > 0.75 {
                PostureModule::reverse_lr(fighter.module_accessor);
            }
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 60, 80, 0, 30, 3.3, 0.0, 7.7, 9.1, Some(0.0), Some(7.7), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("sword1"), 9.0, 60, 80, 0, 30, 4.0, 2.0, 1.0, 1.5, Some(14.0), Some(1.0), Some(1.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("sword1"), 9.0, 60, 80, 0, 30, 3.5, 2.0, 1.0, 7.5, Some(9.5), Some(1.0), Some(7.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 16.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 17.0);
        macros::FT_MOTION_RATE(fighter, 1.0);
    }
}

#[acmd_script( agent = "lucina", script = "game_specials1", category = ACMD_GAME, low_priority )]
unsafe fn lucina_sspecial1(fighter: &mut L2CAgentBase) {
    let mut dmg : f32;
    let kbg : i32;
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 2.0/3.0);
    frame(fighter.lua_state_agent, 15.0);
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
    && spent_meter(fighter.module_accessor, false) {
        SP_GAUGE[entry_id(fighter.module_accessor)] -= SPENT_SP[entry_id(fighter.module_accessor)];
        SP_FLASH[entry_id(fighter.module_accessor)] = 40;
        IS_EX[entry_id(fighter.module_accessor)] = true;
        sp_diff_checker(fighter.module_accessor);
        macros::FT_MOTION_RATE(fighter, 0.2);
    }
    else {
        IS_EX[entry_id(fighter.module_accessor)] = false;
        macros::FT_MOTION_RATE(fighter, 1.0);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
    }
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.5, 367, 140, 50, 0, 5.0, 0.0, 7.0, 7.0, Some(0.0), Some(7.0), Some(1.0), 0.4, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.5, 367, 140, 50, 0, 5.0, 0.0, 7.0, 7.0, Some(0.0), Some(7.0), Some(-5.0), 0.4, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 44.0);
    if macros::is_excute(fighter) {
        if IS_EX[entry_id(fighter.module_accessor)] == true {
            dmg = 13.0;
            kbg = 45;
        }
        else {
            dmg = 9.0;
            kbg = 66;
        }
        dmg *= DMG_RATIO[entry_id(fighter.module_accessor)];
        AttackModule::clear_all(fighter.module_accessor);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), dmg, 45, kbg, 0, 60, 2.5, 0.0, 8.5, -5.0, Some(0.0), Some(8.5), Some(20.0), 1.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), dmg, 45, kbg, 0, 60, 2.5, 0.0, 8.5, 25.0, Some(0.0), Some(8.5), Some(27.0), 1.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 47.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "lucina", script = "effect_specials1", category = ACMD_EFFECT, low_priority )]
unsafe fn lucina_sspecial1eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("lucina_sword_purple"), Hash40::new("haver"), -0.0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 42.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("lucina_breaker_sting"), Hash40::new("top"), -0.0, 11.0, 12, 0, 0, 0, 1.2, true);
        macros::EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -0.0, 9.5, 6, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_COLOR(fighter, 0.264, 0.47, 1.3);
        macros::LAST_EFFECT_SET_RATE(fighter, 0.7);
    }
    frame(fighter.lua_state_agent, 44.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_sp_flash"), Hash40::new("sword1"), -0.0, -0.0, 10, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
	    macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(fighter.lua_state_agent, 46.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 18, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 51.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("lucina_sword_purple"), false, true);
    }
}

#[acmd_script( agent = "lucina", script = "sound_specials1", category = ACMD_SOUND, low_priority )]
unsafe fn lucina_sspecial1snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_lucina_jump02"));
    }
    frame(fighter.lua_state_agent, 44.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_lucina_special_n01"));
        macros::PLAY_SE(fighter, Hash40::new("se_lucina_special_n04"));
    }
}

#[acmd_script( agent = "lucina", script = "expression_specials1", category = ACMD_EXPRESSION, low_priority )]
unsafe fn lucina_sspecial1exp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        AttackModule::set_attack_reference_joint_id(fighter.module_accessor, Hash40::new("sword1"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 42.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohit11"), 0, false, 0);
    }
    frame(fighter.lua_state_agent, 44.0);
    if macros::is_excute(fighter) {
        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_L);
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_piercel"), 0);
    }
}

#[acmd_script( agent = "lucina", script = "game_specialairs1", category = ACMD_GAME, low_priority )]
unsafe fn lucina_sspecial1air(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
        KineticModule::resume_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    macros::FT_MOTION_RATE(fighter, 5.0/35.0);
    frame(fighter.lua_state_agent, 52.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
}

#[acmd_script( agent = "lucina", script = "effect_specialairs1", category = ACMD_EFFECT, low_priority )]
unsafe fn lucina_sspecial1aireff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script( agent = "lucina", script = "game_specialairs2lw", category = ACMD_GAME, low_priority )]
unsafe fn lucina_sspecial2lwair(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_LANDING_CLEAR_SPEED);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        macros::SET_SPEED_EX(fighter, 0, 0.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
    }
    frame(fighter.lua_state_agent, 2.0);
    macros::FT_MOTION_RATE(fighter, 0.1);
    frame(fighter.lua_state_agent, 12.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 13.0);
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
    && spent_meter(fighter.module_accessor, false) {
        SP_GAUGE[entry_id(fighter.module_accessor)] -= SPENT_SP[entry_id(fighter.module_accessor)];
        SP_FLASH[entry_id(fighter.module_accessor)] = 40;
        IS_EX[entry_id(fighter.module_accessor)] = true;
        sp_diff_checker(fighter.module_accessor);
    }
    else {
        IS_EX[entry_id(fighter.module_accessor)] = false;
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        let dmg : f32;
        let angle : u64;
        let velx : f32;
        let vely : f32;
        if IS_EX[entry_id(fighter.module_accessor)] == true {
            dmg = 12.0 * DMG_RATIO[entry_id(fighter.module_accessor)];
            angle = 290;
            velx = 1.75;
            vely = -3.5;
        }
        else {
            dmg = 8.0 * DMG_RATIO[entry_id(fighter.module_accessor)];
            angle = 40;
            velx = 1.5;
            vely = -3.0;
        }
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        macros::SET_SPEED_EX(fighter, velx, vely, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        macros::ATTACK(fighter, 0, 0, Hash40::new("footl"), dmg, angle, 90, 0, 60, 6.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneel"), dmg, angle, 90, 0, 60, 5.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 2, 0, Hash40::new("footl"), dmg, 40, 60, 0, 60, 6.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 3, 0, Hash40::new("kneel"), dmg, 40, 60, 0, 60, 5.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 50.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 61.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
        KineticModule::resume_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
}

#[acmd_script( agent = "lucina", script = "effect_specialairs2lw", category = ACMD_EFFECT, low_priority )]
unsafe fn lucina_sspecial2lwaireff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script( agent = "lucina", script = "game_specialairs2hi", category = ACMD_GAME, low_priority )]
unsafe fn lucina_sspecial2hiair(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
        && spent_meter(fighter.module_accessor, false) {
            SP_GAUGE[entry_id(fighter.module_accessor)] -= SPENT_SP[entry_id(fighter.module_accessor)];
            SP_FLASH[entry_id(fighter.module_accessor)] = 40;
            IS_EX[entry_id(fighter.module_accessor)] = true;
            sp_diff_checker(fighter.module_accessor);
        }
        else {
            IS_EX[entry_id(fighter.module_accessor)] = false;
        }
    }
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        let dmg : f32;
        let kbg : i32;
        let velx : f32;
        let vely : f32;
        if IS_EX[entry_id(fighter.module_accessor)] == true {
            dmg = 16.0 * DMG_RATIO[entry_id(fighter.module_accessor)];
            kbg = 90;
            velx = 3.0;
            vely = -3.0;
        }
        else {
            dmg = 12.0 * DMG_RATIO[entry_id(fighter.module_accessor)];
            kbg = 66;
            velx = 2.1;
            vely = -2.1;
        }
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        macros::SET_SPEED_EX(fighter, velx, vely, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), dmg, 361, kbg, 0, 45, 2.5, 0.0, 8.5, 4.0, Some(0.0), Some(4.0), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), dmg, 361, kbg, 0, 45, 2.5, 0.0, 2.5, 3.0, Some(0.0), Some(11.0), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), dmg, 361, kbg, 0, 45, 2.5, 0.0, 2.5, 3.0, Some(0.0), Some(2.5), Some(-2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
}

#[acmd_script( agent = "lucina", script = "effect_specialairs2hi", category = ACMD_EFFECT, low_priority )]
unsafe fn lucina_sspecial2hiaireff(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("lucina_sword_purple"), Hash40::new("haver"), -0.0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("lucina_breaker_sting"), Hash40::new("top"), -0.0, 6, 8, 40, 0, 0, 0.9, true);
        macros::EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -0.0, 7.5, 6, 40, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_COLOR(fighter, 0.264, 0.47, 1.3);
        macros::LAST_EFFECT_SET_RATE(fighter, 0.7);
    }
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_sp_flash"), Hash40::new("sword1"), -0.0, -0.0, 10, 0, 0, 0, 0.6, true);
	    macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
}

#[acmd_script( agent = "lucina", script = "sound_specialairs2hi", category = ACMD_SOUND, low_priority )]
unsafe fn lucina_sspecial2hiairsnd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_lucina_special_n03"));
    }
}

#[acmd_script( agent = "lucina", script = "game_specials2lw", category = ACMD_GAME, low_priority )]
unsafe fn lucina_sspecial2lw(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "lucina", script = "effect_specials2lw", category = ACMD_EFFECT, low_priority )]
unsafe fn lucina_sspecial2lweff(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script( agent = "lucina", script = "sound_specials2lw", category = ACMD_SOUND, low_priority )]
unsafe fn lucina_sspecial2lwsnd(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::PLAY_LANDING_SE(fighter, Hash40::new("se_lucina_landing02"));
    }
}

#[acmd_script( agent = "lucina", script = "expression_specials2lw", category = ACMD_EXPRESSION, low_priority )]
unsafe fn lucina_sspecial2lwexp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
	    macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
    }
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_impact"), 0, false, 0);
    }
}

#[acmd_script( agent = "lucina", script = "game_specials2hi", category = ACMD_GAME, low_priority )]
unsafe fn lucina_sspecial2hi(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "lucina", script = "effect_specials2hi", category = ACMD_EFFECT, low_priority )]
unsafe fn lucina_sspecial2hieff(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script( agent = "lucina", script = "sound_specials2hi", category = ACMD_SOUND, low_priority )]
unsafe fn lucina_sspecial2hisnd(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::PLAY_LANDING_SE(fighter, Hash40::new("se_lucina_landing02"));
    }
}

#[acmd_script( agent = "lucina", script = "expression_specials2hi", category = ACMD_EXPRESSION, low_priority )]
unsafe fn lucina_sspecial2hiexp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
	    macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
    }
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_impact"), 0, false, 0);
    }
}

#[acmd_script( agent = "lucina", script = "game_specialhi", category = ACMD_GAME, low_priority )]
unsafe fn lucina_uspecial(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        upper_invuln(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 2.0);
    frame(fighter.lua_state_agent, 3.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
    && spent_meter(fighter.module_accessor, false) {
        SP_GAUGE[entry_id(fighter.module_accessor)] -= SPENT_SP[entry_id(fighter.module_accessor)];
        SP_FLASH[entry_id(fighter.module_accessor)] = 60;
        IS_EX[entry_id(fighter.module_accessor)] = true;
        sp_diff_checker(fighter.module_accessor);
    }
    else {
        IS_EX[entry_id(fighter.module_accessor)] = false;
    }
    if IS_EX[entry_id(fighter.module_accessor)] {
        upper_invuln(fighter.module_accessor, false);
        full_invuln(fighter.module_accessor, true);
        frame(fighter.lua_state_agent, 5.0);
        macros::FT_MOTION_RATE(fighter, 6.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.5, 87, 100, 160, 0, 5.1, 0.0, 11.0, 10.0, Some(0.0), Some(7.0), Some(10.0), 1.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_SPECIAL_HI_SET_LR);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE);
        }
        wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.1, 367, 100, 85, 0, 6.0, 0.0, 20.0, 10.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.1, 367, 100, 85, 0, 5.3, 0.0, 0.0, 10.0, Some(0.0), Some(17.0), Some(10.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
        }
        frame(fighter.lua_state_agent, 8.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 40, 85, 0, 75, 6.5, 0.0, 12.0, 10.0, Some(0.0), Some(21.0), Some(10.0), 2.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        }
        frame(fighter.lua_state_agent, 9.0);
        if macros::is_excute(fighter) {
            full_invuln(fighter.module_accessor, false);
        }
        macros::FT_MOTION_RATE(fighter, 1.0);
        frame(fighter.lua_state_agent, 11.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
    }
    else{
        frame(fighter.lua_state_agent, 5.0);
        macros::FT_MOTION_RATE(fighter, 3.5);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 10.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 79, 60, 0, 70, 4.0, 0.0, 8.0, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 10.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 79, 60, 0, 70, 4.0, 0.0, 8.0, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_SPECIAL_HI_SET_LR);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE);
        }
        frame(fighter.lua_state_agent, 6.0);
        if macros::is_excute(fighter) {
            upper_invuln(fighter.module_accessor, false);
            macros::ATTACK(fighter, 0, 0, Hash40::new("sword1"), 7.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 79, 50, 0, 50, 5.0, 0.0, 0.0, 4.5, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("sword1"), 7.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 79, 50, 0, 50, 5.0, 0.0, 0.0, -1.5, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 7.0);
        macros::FT_MOTION_RATE(fighter, 1.0);
        if macros::is_excute(fighter) {
            AttackModule::clear(fighter.module_accessor, 2, false);
            AttackModule::clear(fighter.module_accessor, 3, false);
            smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        }
        frame(fighter.lua_state_agent, 12.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
    }
}

#[acmd_script( agent = "lucina", script = "game_specialairhi", category = ACMD_GAME, low_priority )]
unsafe fn lucina_uspecialair(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 2.0);
    frame(fighter.lua_state_agent, 3.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
    && spent_meter(fighter.module_accessor, false) {
        SP_GAUGE[entry_id(fighter.module_accessor)] -= SPENT_SP[entry_id(fighter.module_accessor)];
        SP_FLASH[entry_id(fighter.module_accessor)] = 60;
        IS_EX[entry_id(fighter.module_accessor)] = true;
        sp_diff_checker(fighter.module_accessor);
    }
    else {
        IS_EX[entry_id(fighter.module_accessor)] = false;
    }
    if IS_EX[entry_id(fighter.module_accessor)] {
        full_invuln(fighter.module_accessor, true);
        frame(fighter.lua_state_agent, 5.0);
        macros::FT_MOTION_RATE(fighter, 6.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.5, 87, 100, 140, 0, 5.1, 0.0, 11.0, 10.0, Some(0.0), Some(7.0), Some(10.0), 1.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_SPECIAL_HI_SET_LR);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE);
        }
        wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.1, 367, 50, 10, 0, 6.0, 0.0, 20.0, 10.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.1, 367, 50, 10, 0, 5.3, 0.0, 0.0, 10.0, Some(0.0), Some(17.0), Some(10.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
        }
        frame(fighter.lua_state_agent, 8.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 40, 85, 0, 75, 6.5, 0.0, 12.0, 10.0, Some(0.0), Some(21.0), Some(10.0), 2.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        }
        frame(fighter.lua_state_agent, 9.0);
        if macros::is_excute(fighter) {
            full_invuln(fighter.module_accessor, false);
        }
        macros::FT_MOTION_RATE(fighter, 1.0);
        frame(fighter.lua_state_agent, 11.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
    }
    else{
        upper_invuln(fighter.module_accessor, true);
        frame(fighter.lua_state_agent, 5.0);
        macros::FT_MOTION_RATE(fighter, 3.5);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 10.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 79, 60, 0, 70, 4.0, 0.0, 8.0, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 10.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 79, 60, 0, 70, 4.0, 0.0, 8.0, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_SPECIAL_HI_SET_LR);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE);
        }
        frame(fighter.lua_state_agent, 6.0);
        if macros::is_excute(fighter) {
            upper_invuln(fighter.module_accessor, false);
            macros::ATTACK(fighter, 0, 0, Hash40::new("sword1"), 7.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 79, 50, 0, 50, 5.0, 0.0, 0.0, 4.5, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("sword1"), 7.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 79, 50, 0, 50, 5.0, 0.0, 0.0, -1.5, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        frame(fighter.lua_state_agent, 7.0);
        macros::FT_MOTION_RATE(fighter, 1.0);
        if macros::is_excute(fighter) {
            AttackModule::clear(fighter.module_accessor, 2, false);
            AttackModule::clear(fighter.module_accessor, 3, false);
            smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        }
        frame(fighter.lua_state_agent, 12.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
    }
}

#[acmd_script( agent = "lucina", scripts = [ "game_speciallw", "game_specialairlw" ], category = ACMD_GAME, low_priority )]
unsafe fn lucina_dspecial(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        SP_GAUGE[entry_id(fighter.module_accessor)] -= SPENT_SP[entry_id(fighter.module_accessor)];
        JostleModule::set_status(fighter.module_accessor, false);
        KineticModule::unable_energy_all(fighter.module_accessor);
        if ROMAN_ON_HIT[entry_id(fighter.module_accessor)] {
            AIR_ACTION[entry_id(fighter.module_accessor)] = false;
            macros::SLOW_OPPONENT(fighter, 50.0, 19.0);
            full_invuln(fighter.module_accessor, true);
        }
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        let dir = get_command_stick_direction(fighter.module_accessor, false);
        if dir == 5
        || dir == 8
        || dir == 2 {
            ROMAN_MOVE[entry_id(fighter.module_accessor)] = 0.0;
        }
        else if dir == 4
        || dir == 7
        || dir == 1 {
            ROMAN_MOVE[entry_id(fighter.module_accessor)] = -2.0;
        }
        else {
            ROMAN_MOVE[entry_id(fighter.module_accessor)] = 2.0;
        }
    }
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 40.0, 0.0, 10.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 1, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        IS_ROMAN_MOVE[entry_id(fighter.module_accessor)] = true;
    }
    macros::FT_MOTION_RATE(fighter, 5.0);
    frame(fighter.lua_state_agent, 21.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        if ROMAN_ON_HIT[entry_id(fighter.module_accessor)] {
            IS_ROMAN_MOVE[entry_id(fighter.module_accessor)] = false;
            full_invuln(fighter.module_accessor, false);
        }
    }
}

#[acmd_script( agent = "lucina", scripts = [ "effect_speciallw", "effect_specialairlw" ], category = ACMD_EFFECT, low_priority )]
unsafe fn lucina_dspecialeff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("lucina_counter_flash"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 0.3);
    }
}

#[acmd_script( agent = "lucina", scripts = [ "sound_speciallw", "sound_specialairlw" ], category = ACMD_SOUND, low_priority )]
unsafe fn lucina_dspecialsnd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_lucina_special_l01"));
    }
}

#[acmd_script( agent = "lucina", scripts = [ "game_speciallwhit", "game_specialairlwhit" ], category = ACMD_GAME, low_priority )]
unsafe fn lucina_dspecialhit(fighter: &mut L2CAgentBase) {
    if shadow_id(fighter.module_accessor) {
        macros::FT_START_CUTIN(fighter);
        macros::SLOW_OPPONENT(fighter, 20.0, 8.0);
        SHADOW_FRENZY[entry_id(fighter.module_accessor)] = true;
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("sword1"), 3.4, 90, 0, 60, 40, 5.0, 1.0, 0.0, 1.5, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 3.4, 90, 0, 60, 40, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("colonells"), 3.4, 90, 0, 60, 40, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("sword1"), 3.4, 90, 0, 60, 40, 5.0, 1.0, 0.0, 7.0, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 1, true, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 2, true, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 3, true, false);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_LW_FLAG_SPECIAL_EFFECT) {
            AttackModule::set_optional_hit_effect(fighter.module_accessor, 0, Hash40::new("se_lucina_criticalhit"));
            AttackModule::set_optional_hit_effect(fighter.module_accessor, 1, Hash40::new("se_lucina_criticalhit"));
            AttackModule::set_optional_hit_effect(fighter.module_accessor, 2, Hash40::new("se_lucina_criticalhit"));
            AttackModule::set_optional_hit_effect(fighter.module_accessor, 3, Hash40::new("se_lucina_criticalhit"));
        }
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

// #[acmd_script( agent = "lucina", script = "effect_run", category = ACMD_EFFECT, low_priority )]
// unsafe fn lucina_runeff(fighter: &mut L2CAgentBase) {
//     while {
//         frame(fighter.lua_state_agent, 4.0);
//         if macros::is_excute(fighter) {
//             macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
//         }
//         frame(fighter.lua_state_agent, 21.0);
//         if macros::is_excute(fighter) {
//             macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
//         }
//         frame(fighter.lua_state_agent, 37.0);
//         if macros::is_excute(fighter) {
//             macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
//         }
//         frame(fighter.lua_state_agent, 56.0);
//         fighter.clear_lua_stack();
//         wait_loop_sync_mot(fighter.lua_state_agent);
//         fighter.pop_lua_stack(1);
//         true
//     } {}
// }

pub fn install() {
    install_agent_frames!(
        lucina_frame
    );
    install_status_scripts!(
        lucina_specialnloop_main,
        lucina_specials_main,
        lucina_specials2_main,
        lucina_speciallw_main
    );
    install_acmd_scripts!(
        lucina_jab1,
        lucina_jab1eff,
        lucina_jab2,
        lucina_jab2eff,
        lucina_ftilt,
        lucina_utilt,
        lucina_dtilt,
        lucina_dashattack,
        lucina_dashattacksound,
        lucina_nair,
        lucina_naireff,
        lucina_nairsnd,
        lucina_fair,
        lucina_bair,
        lucina_uair,
        lucina_dair,
        lucina_fsmash,
        lucina_usmash,
        lucina_uthrow,
        lucina_nspecialloop,
        lucina_nspecialend,
        lucina_nspecialendmax,
        lucina_sspecial1,
        lucina_sspecial1eff,
        lucina_sspecial1snd,
        lucina_sspecial1exp,
        lucina_sspecial1air,
        lucina_sspecial1aireff,
        lucina_sspecial2lwair,
        lucina_sspecial2lwaireff,
        lucina_sspecial2hiair,
        lucina_sspecial2hiaireff,
        lucina_sspecial2hiairsnd,
        lucina_sspecial2lw,
        lucina_sspecial2lweff,
        lucina_sspecial2lwsnd,
        lucina_sspecial2lwexp,
        lucina_sspecial2hi,
        lucina_sspecial2hieff,
        lucina_sspecial2hisnd,
        lucina_sspecial2hiexp,
        lucina_uspecial,
        lucina_uspecialair,
        lucina_dspecial,
        lucina_dspecialeff,
        lucina_dspecialsnd,
        lucina_dspecialhit,
        // lucina_runeff
    );
    // skyline::install_hook!(lucina_is_enable_transition_term_replace);
}