#![allow(unused_must_use)]

use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        phx::Hash40,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smash_script::*,
    crate::{
        vars::*,
        table_const::*
    }
};

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_ftStatusUniqProcessGuardOn_initStatus_common)]
pub unsafe fn sub_ftstatusuniqprocessguardon_initstatus_common(fighter: &mut L2CFighterCommon) {
    // Original
    ShieldModule::set_status(fighter.module_accessor, *FIGHTER_SHIELD_KIND_GUARD, ShieldStatus(*SHIELD_STATUS_NORMAL), 0);
    // Additions
    let guard_hold_frame = WorkModule::get_int(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_GUARD_HOLD_FRAME);
    // println!("Guard Hold Frame: {}", guard_hold_frame);
    if FighterUtil::is_valid_just_shield(fighter.module_accessor)
    && (guard_hold_frame >= 5
    || ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)) {
        let shield_just_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("shield_just_frame")) as f32;
        let just_shield_check_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("just_shield_check_frame"), 0);
        let just_frame = (shield_just_frame * just_shield_check_frame + 0.5) as i32;
        WorkModule::set_int(fighter.module_accessor, just_frame, *FIGHTER_STATUS_GUARD_ON_WORK_INT_JUST_FRAME);
        ShieldModule::set_shield_type(fighter.module_accessor, ShieldType(*SHIELD_TYPE_JUST_SHIELD), *FIGHTER_SHIELD_KIND_GUARD, 0);
        if FighterUtil::is_valid_just_shield_reflector(fighter.module_accessor) {
            ReflectorModule::set_status(fighter.module_accessor, 0, ShieldStatus(*SHIELD_STATUS_NORMAL), *FIGHTER_REFLECTOR_GROUP_JUST_SHIELD);
        }
        fighter.FighterStatusGuard__set_just_shield_scale();
    }
    // Also Original, but moved down
    let hit_stop_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), 0x20d241cd64u64);
    ShieldModule::set_hit_stop_mul(fighter.module_accessor, hit_stop_mul);
    let guard_off_disable_shield_recovery = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("guard_off_disable_shield_recovery"));
    WorkModule::set_int(fighter.module_accessor, guard_off_disable_shield_recovery, *FIGHTER_INSTANCE_WORK_ID_INT_DISABLE_SHIELD_RECOVERY_FRAME);
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_status_guard_on_common)]
pub unsafe fn sub_status_guard_on_common(fighter: &mut L2CFighterCommon) {
    let shield_min_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("shield_min_frame"));
    WorkModule::set_int(fighter.module_accessor, shield_min_frame, *FIGHTER_STATUS_GUARD_ON_WORK_INT_MIN_FRAME);
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("guard_on"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_IGNORE_2ND_MOTION) {
        MotionModule::add_motion_2nd(
            fighter.module_accessor,
            Hash40::new("guard"),
            0.0,
            1.0,
            false,
            1.0
        );
        MotionModule::set_rate_2nd(fighter.module_accessor, 0.0);
        fighter.sub_ftStatusUniqProcessGuardFunc_updateShield(true.into());
    }
    fighter.sub_guard_cont_pre();
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_guard_on_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(smash::lua2cpp::L2CFighterCommon_bind_address_call_sub_guard_on_uniq as *const () as _));
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_guard_cont_pre)]
pub unsafe fn sub_guard_cont_pre(fighter: &mut L2CFighterCommon) {
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_GUARD);
    if fighter.global_table[STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_GUARD_ON
    && fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_RUN {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH);
        let catch_dash_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("catch_dash_frame"));
        WorkModule::set_int(fighter.module_accessor, catch_dash_frame, *FIGHTER_STATUS_GUARD_ON_WORK_INT_CATCH_FRAME);
    }
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B);
    if GroundModule::is_passable_ground(fighter.module_accessor) {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_PASS);
    }
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_guard_on_uniq)]
pub unsafe fn sub_guard_on_uniq(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() == false {
        fighter.FighterStatusGuard__landing_effect_control();
    }
    else {
        if 0 < WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_INT_JUST_FRAME) {
            WorkModule::dec_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_INT_JUST_FRAME);
            let just_guard_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_INT_JUST_FRAME);
            if just_guard_frame == 0 {
                ShieldModule::set_status(fighter.module_accessor, *FIGHTER_SHIELD_KIND_GUARD, ShieldStatus(*SHIELD_STATUS_NORMAL), 0);
                let guard_type;
                if FighterUtil::get_shield_type_of_guard(fighter.global_table[FIGHTER_KIND].get_i32()) {
                    guard_type = *SHIELD_TYPE_GUARD;
                }
                else {
                    guard_type = *SHIELD_TYPE_UNDEFINED;
                }
                ShieldModule::set_shield_type(fighter.module_accessor, ShieldType(guard_type), *FIGHTER_SHIELD_KIND_GUARD, 0);
                if FighterUtil::is_valid_just_shield_reflector(fighter.module_accessor) {
                    ReflectorModule::set_status(fighter.module_accessor, 0, ShieldStatus(*SHIELD_STATUS_NORMAL), *FIGHTER_REFLECTOR_GROUP_JUST_SHIELD);
                }
            }
        }
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SHIELD_LOCK) {
            let shield_dec1 : f32;
            if !WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FGC) {
                shield_dec1 = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("shield_dec1"));
            }
            else {
                shield_dec1 = 0.0;
            }
            let shield_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("shield_frame"), 0);
            let decrease = shield_dec1 / shield_frame;
            WorkModule::sub_float(fighter.module_accessor, decrease, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);
        }
        let shield_health = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);
        let shield_health_min = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD_MIN);
        if shield_health < shield_health_min {
            WorkModule::set_float(fighter.module_accessor, shield_health_min, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);
        }
        let shield_min_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_INT_MIN_FRAME);
        if 0 < shield_min_frame {
            WorkModule::dec_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_INT_MIN_FRAME);
        }
        let catch_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_INT_CATCH_FRAME);
        if catch_frame < 0 {
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH);
        }
    }
    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_guard_cont)]
pub unsafe fn sub_guard_cont(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[GUARD_CONT_PRE].get_bool() != false && {
        let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[GUARD_CONT_PRE].get_ptr());
        callable(fighter).get_bool()
    } {
        return true.into();
    }
    let check_guard_hold = fighter.check_guard_hold().get_bool();
    if fighter.sub_transition_group_check_ground_jump_mini_attack().get_bool() {
        return true.into();
    }
    if check_guard_hold == false
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_GUARD)
    && ItemModule::is_have_item(fighter.module_accessor, 0) && {
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_NO_THROW);
        sv_module_access::item(fighter.lua_state_agent);
        fighter.pop_lua_stack(1).get_bool() == false
    } {
        if fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0
        || (fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER == 0
        && fighter.global_table[CMD_CAT3].get_i32() & (*FIGHTER_PAD_CMD_CAT3_ITEM_LIGHT_THROW_HI | *FIGHTER_PAD_CMD_CAT3_ITEM_LIGHT_THROW_HI4) != 0) {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), false.into());
                return true.into();
            }
        }
    }
    if fighter.global_table[STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_GUARD_ON
    && fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_RUN
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN) && {
        let stick_x = fighter.global_table[STICK_X].get_f32();
        let lr = PostureModule::lr(fighter.module_accessor);
        let turn_run_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("turn_run_stick_x"));
        stick_x * lr <= turn_run_stick_x
    } && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && ItemModule::is_have_item(fighter.module_accessor, 0) == false {
        fighter.change_status(FIGHTER_STATUS_KIND_CATCH_TURN.into(), true.into());
        return true.into();
    }
    if check_guard_hold == false {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE)
        && fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_STICK_ESCAPE != 0
        && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE.into(), true.into());
            return true.into();
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F)
        && fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_STICK_ESCAPE_F != 0
        && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_F.into(), true.into());
            return true.into();
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B)
        && fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_STICK_ESCAPE_B != 0
        && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_B.into(), true.into());
            return true.into();
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_PASS)
        && fighter.global_table[CMD_CAT2].get_i32() & (
            *FIGHTER_PAD_CMD_CAT2_FLAG_GUARD_TO_PASS | *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI |
            *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L | *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R |
            *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_LW
        ) != 0
        && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_PASS.into(), true.into());
            return true.into();
        }
    }
    if fighter.global_table[STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_GUARD_ON
    && fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_RUN
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH)
    && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && ItemModule::is_have_item(fighter.module_accessor, 0) == false {
        fighter.change_status(FIGHTER_STATUS_KIND_CATCH_DASH.into(), true.into());
        return true.into();
    }
    if fighter.check_guard_attack_special_hi(check_guard_hold.into()).get_bool() == false {
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_INVALID_CATCH_FRAME) == 0 {
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH)
            && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH != 0
            && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
            && ItemModule::is_have_item(fighter.module_accessor, 0) == false {
                fighter.change_status(FIGHTER_STATUS_KIND_CATCH.into(), true.into());
                return true.into();
            }
        }
        if check_guard_hold == false {
            if fighter.sub_transition_group_check_ground_jump().get_bool() {
                return true.into();
            }
        }
        return false.into();
    }
    else {
        return true.into();
    }
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_status_end_guard_on_common)]
pub unsafe fn sub_status_end_guard_on_common(fighter: &mut L2CFighterCommon, param_1: L2CValue) {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if status != *FIGHTER_STATUS_KIND_GUARD {
        if status != *FIGHTER_STATUS_KIND_GUARD_DAMAGE
        || (status == *FIGHTER_STATUS_KIND_GUARD_DAMAGE && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD)) {
            fighter.clear_lua_stack();
            lua_args!(fighter, MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND, 0xafae75f05u64, true, true);
            sv_module_access::effect(fighter.lua_state_agent);
            fighter.pop_lua_stack(1);
            fighter.clear_lua_stack();
            lua_args!(fighter, MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND, 0x10da0b43c8u64, true, true);
            sv_module_access::effect(fighter.lua_state_agent);
            fighter.pop_lua_stack(1);
        }
        else {
            sub_status_end_guard_on_common_thing(fighter, param_1);
        }
    }
    else {
        sub_status_end_guard_on_common_thing(fighter, param_1);
    }
}

unsafe extern "C" fn sub_status_end_guard_on_common_thing(fighter: &mut L2CFighterCommon, param_1: L2CValue) {
    if param_1.get_bool() == false {
        notify_event_msc_cmd!(fighter, 0x262a7a102du64);
    }
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_GuardDamage_Main)]
pub unsafe fn status_guarddamage_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.status_guard_damage_main_common_air().get_bool() {
        return 0.into();
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
            // if fighter.FighterStatusGuard__is_continue_just_shield_count().get_bool() {
            //     fighter.status_guard_damage_main_common();
            //     return 0.into();
            // }
            let is_hit = StopModule::is_hit(fighter.module_accessor);
            if is_hit {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_TRANSITION_STATUS_STOP);
            }
            if CancelModule::is_enable_cancel(fighter.module_accessor) {
                if fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_COMMON_GUARD != 0 {
                    let common_guard_hold = ControlModule::get_command_life(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY2, 0x18) as i32;
                    WorkModule::set_int(fighter.module_accessor, common_guard_hold, FIGHTER_INSTANCE_WORK_ID_INT_GUARD_HOLD_FRAME);
                    fighter.change_status(FIGHTER_STATUS_KIND_GUARD_ON.into(), true.into());
                    return 0.into();
                }
                if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
                    if is_hit {
                        StopModule::cancel_hit_stop(fighter.module_accessor);
                        return 0.into();
                    }
                }
            }
            if is_hit {
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_TRANSITION_STATUS_STOP);
            }
        }
    }
    fighter.status_guard_damage_main_common();
    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_ftStatusUniqProcessGuardOff_initStatus)]
pub unsafe fn sub_ftstatusuniqprocessguardoff_initstatus(_fighter: &mut L2CFighterCommon) -> L2CValue {
    // Original, except we're using NONE OF IT HAHAHAHAHHAHA
    // if FighterUtil::is_valid_just_shield(fighter.module_accessor) {
    //     ShieldModule::set_status(fighter.module_accessor, *FIGHTER_SHIELD_KIND_GUARD, ShieldStatus(*SHIELD_STATUS_NORMAL), 0);
    //     ShieldModule::set_shield_type(fighter.module_accessor, ShieldType(*SHIELD_TYPE_JUST_SHIELD), *FIGHTER_SHIELD_KIND_GUARD, 0);
    //     if FighterUtil::is_valid_just_shield_reflector(fighter.module_accessor) {
    //         ReflectorModule::set_status(fighter.module_accessor, 0, ShieldStatus(*SHIELD_STATUS_NORMAL), *FIGHTER_REFLECTOR_GROUP_JUST_SHIELD);
    //     }
    //     fighter.FighterStatusGuard__set_just_shield_scale();
    //     let hit_stop_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), 0x20d241cd64u64);
    //     ShieldModule::set_hit_stop_mul(fighter.module_accessor, hit_stop_mul);
    // }
    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_GuardOff_Common)]
pub unsafe fn status_guardoff_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_GUARD);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
    // Original Parry stuff
    // let shield_just_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("shield_just_frame")) as f32;
    // let just_shield_check_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("just_shield_check_frame"), 0);
    // let just_frame = (shield_just_frame * just_shield_check_frame + 0.5) as i32;
    // WorkModule::set_int(fighter.module_accessor, just_frame, *FIGHTER_STATUS_GUARD_ON_WORK_INT_JUST_FRAME);
    let guard_off_cancel_frame;
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FUNNY)
    || WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FGC) {
        guard_off_cancel_frame = 5;
    }
    else {
        guard_off_cancel_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("guard_off_cancel_frame"));
    }
    WorkModule::set_int(fighter.module_accessor, guard_off_cancel_frame, *FIGHTER_STATUS_GUARD_OFF_WORK_INT_CANCEL_FRAME);
    let anim_cancel_frame = FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor, Hash40::new("guard_off"), true) as f32;
    let mut motion_rate = 1.0;
    if 0.0 < guard_off_cancel_frame as f32
    && 0.0 < anim_cancel_frame {
        motion_rate = anim_cancel_frame / guard_off_cancel_frame as f32;
    }
    let guard_off_enable_shield_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("guard_off_enable_shield_frame"));
    let disable_guard_escape_frame = guard_off_cancel_frame + guard_off_enable_shield_frame;
    WorkModule::set_int(fighter.module_accessor, disable_guard_escape_frame, *FIGHTER_INSTANCE_WORK_ID_INT_DISABLE_GUARD_FRAME);
    WorkModule::set_int(fighter.module_accessor, disable_guard_escape_frame, *FIGHTER_INSTANCE_WORK_ID_INT_DISABLE_ESCAPE_FRAME);
    // I don't think we need this since we're moving Parry to GUARD_ON instead
    // if !StopModule::is_stop(fighter.module_accessor) {
    //     fighter.sub_guard_off_uniq(false.into());
    // }
    // fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(smash::lua2cpp::L2CFighterCommon_bind_address_call_sub_guard_off_uniq as *const () as _));
    L2CValue::new_num(motion_rate)
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_ftstatusuniqprocessguardon_initstatus_common,
            sub_guard_cont_pre,
            sub_guard_on_uniq,
            sub_guard_cont,
            sub_status_end_guard_on_common,
            status_guarddamage_main,
            sub_ftstatusuniqprocessguardoff_initstatus,
            sub_status_guard_on_common,
            status_guardoff_common
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}