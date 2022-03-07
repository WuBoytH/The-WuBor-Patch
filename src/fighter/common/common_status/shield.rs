#![allow(unused_must_use)]

use {
    smash::{
        lua2cpp::{L2CFighterCommon, *},
        hash40,
        phx::{Hash40, Vector3f},
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smash_script::*,
    super::super::common_param::*,
    wubor_utils::{
        vars::*,
        table_const::*
    }
};

#[skyline::hook(replace = L2CFighterCommon_sub_ftStatusUniqProcessGuardOn_initStatus_common)]
unsafe fn sub_ftstatusuniqprocessguardon_initstatus_common(fighter: &mut L2CFighterCommon) {
    // Original
    ShieldModule::set_status(fighter.module_accessor, *FIGHTER_SHIELD_KIND_GUARD, ShieldStatus(*SHIELD_STATUS_NORMAL), 0);
    // Additions
    if FighterUtil::is_valid_just_shield(fighter.module_accessor) {
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

#[skyline::hook(replace = L2CFighterCommon_sub_status_guard_on_common)]
unsafe fn sub_status_guard_on_common(fighter: &mut L2CFighterCommon) {
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
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_guard_on_uniq as *const () as _));
}

#[skyline::hook(replace = L2CFighterCommon_sub_guard_cont_pre)]
unsafe fn sub_guard_cont_pre(fighter: &mut L2CFighterCommon) {
    // WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_GUARD);
    if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() == *FIGHTER_STATUS_KIND_GUARD_ON
    && fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_RUN {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH);
        let catch_dash_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("catch_dash_frame"));
        WorkModule::set_int(fighter.module_accessor, catch_dash_frame, *FIGHTER_STATUS_GUARD_ON_WORK_INT_CATCH_FRAME);
    }
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
    // WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B);
    if GroundModule::is_passable_ground(fighter.module_accessor) {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_PASS);
    }
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
}

#[skyline::hook(replace = L2CFighterCommon_sub_guard_on_uniq)]
unsafe fn sub_guard_on_uniq(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
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
            WorkModule::dec_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_INT_CATCH_FRAME);
            let catch_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_INT_CATCH_FRAME);
            if catch_frame < 0 {
                WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN);
                WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH);
            }
        }
    }
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_guard_cont)]
unsafe fn sub_guard_cont(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[GUARD_CONT_PRE].get_bool() != false && {
        let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[GUARD_CONT_PRE].get_ptr());
        callable(fighter).get_bool()
    } {
        return true.into();
    }
    let check_guard_hold = fighter.check_guard_hold().get_bool();
    if check_guard_hold == false {
        if fighter.sub_transition_group_check_ground_jump_mini_attack().get_bool() {
            return true.into();
        }
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
    if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() == *FIGHTER_STATUS_KIND_GUARD_ON
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
    if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() == *FIGHTER_STATUS_KIND_GUARD_ON
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

#[skyline::hook(replace = L2CFighterCommon_sub_status_end_guard_on_common)]
unsafe fn sub_status_end_guard_on_common(fighter: &mut L2CFighterCommon, param_1: L2CValue) {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if status != *FIGHTER_STATUS_KIND_GUARD
    && (status != *FIGHTER_STATUS_KIND_GUARD_DAMAGE
    || (status == *FIGHTER_STATUS_KIND_GUARD_DAMAGE
    && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD))) {
        effect!(fighter, MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND, Hash40::new_raw(0xafae75f05), true, true);
        effect!(fighter, MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND, Hash40::new_raw(0x10da0b43c8), true, true);
    }
    else if param_1.get_bool() == false {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x262a7a102d));
    }
}

#[skyline::hook(replace = L2CFighterCommon_sub_ftStatusUniqProcessGuardDamage_initStatus_Inner)]
unsafe fn sub_ftstatusuniqprocessguarddamage_initstatus_inner(fighter: &mut L2CFighterCommon) {
    let shield_power = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLOAT_SHIELD_POWER);
    // println!("shield power: {}", shield_power);
    let shield_setoff_mul_status = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLOAT_SHIELD_SETOFF_MUL);
    // println!("shield_setoff_mul_status: {}", shield_setoff_mul_status);
    let mut shield_stiff_frame = shield_power * shield_setoff_mul_status;
    // println!("shield_stiff_frame: {}", shield_stiff_frame);
    shield_stiff_frame *= WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("shield_setoff_mul"));
    // println!("shield_stiff_frame * shield_setoff_mul: {}", shield_stiff_frame);
    let object_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_DAMAGE_WORK_INT_OBJECT_ID);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
        shield_stiff_frame *= WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("just_shield_setoff_mul"));
        // println!("now with just shield mul: {}", shield_stiff_frame);
    }
    shield_stiff_frame += WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("shield_setoff_add"));
    // println!("plus 3.0 for good measure: {}", shield_stiff_frame);
    let shield_stiff_frame_max = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("shield_stiff_frame_max"));
    if shield_stiff_frame_max < shield_stiff_frame {
        shield_stiff_frame = shield_stiff_frame_max;
    }
    // println!("final shield_stiff_frame: {}", shield_stiff_frame as i32);
    if object_id != *BATTLE_OBJECT_ID_INVALID {
        capture!(fighter, MA_MSC_CMD_CAPTURE_SET_IGNORE_OBJECT_ID, object_id);
        fighter.pop_lua_stack(1);
        let mut invalid_capture_frame = shield_stiff_frame;
        // println!("invalid_capture_frame: {}", invalid_capture_frame);
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
            invalid_capture_frame += WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("guard_off_cancel_frame")) as f32;
            // println!("invalid_capture_frame with cancel frame: {}", invalid_capture_frame);
        }
        // invalid_capture_frame *= WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("shield_ignore_capture_rate"));
        invalid_capture_frame += guard_off_invalid_capture_frame_add as f32;
        // println!("invalid_capture_frame: {}", invalid_capture_frame);
        WorkModule::set_int(fighter.module_accessor, invalid_capture_frame as i32, *FIGHTER_INSTANCE_WORK_ID_INT_GUARD_INVALID_CAPTURE_FRAME);
    }
    WorkModule::set_int(fighter.module_accessor, shield_stiff_frame as i32, *FIGHTER_STATUS_GUARD_DAMAGE_WORK_INT_STIFF_FRAME);
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
        let shield_setoff_catch_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("shield_setoff_catch_frame"));
        if shield_setoff_catch_frame > 0 {
            WorkModule::set_int(fighter.module_accessor, shield_setoff_catch_frame, *FIGHTER_INSTANCE_WORK_ID_INT_INVALID_CATCH_FRAME);
        }
    }
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
        fighter.clear_lua_stack();
        let mot = hash40("guard_damage");
        lua_args!(fighter, mot);
        let motion_rate = sv_fighter_util::get_guard_damage_motion_rate(fighter.lua_state_agent, Hash40::new_raw(mot));
        let weight = MotionModule::weight(fighter.module_accessor);
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new_raw(mot),
            0.0,
            motion_rate,
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
            MotionModule::set_weight(fighter.module_accessor, weight, true);
            let prev_x = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLOAT_PREV_X);
            let prev_y = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLOAT_PREV_Y);
            fighter.FighterStatusGuard__set_guard_blend_motion_angle(prev_x.into(), prev_y.into());
        }
    }
    else {
        let mut cancel_frame = FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor, Hash40::new("just_shield_off"), true);
        if cancel_frame == 0.0 {
            cancel_frame = MotionModule::end_frame_from_hash(fighter.module_accessor, Hash40::new("just_shield_off"));
        }
        let motion_rate = cancel_frame / shield_stiff_frame;
        let just_shield_motion = WorkModule::get_param_int(fighter.module_accessor, hash40("param_motion"), hash40("just_shield_motion"));
        if just_shield_motion == 0 {
            let frame = MotionModule::end_frame_from_hash(fighter.module_accessor, Hash40::new("just_shield_off"));
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new("just_shield_off"),
                frame,
                motion_rate,
                false,
                0.0,
                false,
                false
            );
        }
        else {
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new("just_shield_off"),
                0.0,
                motion_rate,
                false,
                0.0,
                false,
                false
            );
        }
        MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EXPRESSION, Hash40::new_raw(0x1a29f56bfb), -1);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_INT_JUST_SHEILD_COUNT);
        if fighter.FighterStatusGuard__is_continue_just_shield_count().get_bool() == false {
            CancelModule::enable_cancel(fighter.module_accessor);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_DISABLE_HIT_STOP_DELAY_STICK);
        }
        else {
            ShieldModule::set_status(fighter.module_accessor, *FIGHTER_SHIELD_KIND_GUARD, ShieldStatus(*SHIELD_STATUS_NORMAL), 0);
            ShieldModule::set_shield_type(fighter.module_accessor, ShieldType(*SHIELD_TYPE_JUST_SHIELD), *FIGHTER_SHIELD_KIND_GUARD, 0);
            let boma = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut BattleObjectModuleAccessor;
            if FighterUtil::is_valid_just_shield_reflector(boma) {
                ReflectorModule::set_status(fighter.module_accessor, 0, ShieldStatus(*SHIELD_STATUS_NORMAL), *FIGHTER_REFLECTOR_GROUP_JUST_SHIELD);
            }
        }
    }
    else {
        ShieldModule::set_status(fighter.module_accessor, *FIGHTER_SHIELD_KIND_GUARD, ShieldStatus(*SHIELD_STATUS_NORMAL), 0);
        ControlModule::clear_command(fighter.module_accessor, false);
    }
    let shield_setoff_speed_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("shield_setoff_speed_mul"));
    let mut setoff_speed = shield_setoff_speed_mul * shield_stiff_frame;
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
        setoff_speed *= WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("just_shield_speed_rate"));
    }
    let shield_setoff_speed_max = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("shield_setoff_speed_max"));
    if shield_setoff_speed_max < setoff_speed {
        setoff_speed = shield_setoff_speed_max;
    }
    let shield_lr = -WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLOAT_SHIELD_LR);
    setoff_speed *= shield_lr;
    // println!("setoff_speed: {}", setoff_speed);
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_DAMAGE, ENERGY_STOP_RESET_TYPE_GUARD_DAMAGE, setoff_speed, 0.0, 0.0, 0.0, 0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
    let mut hit_stop_frame = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLOAT_HIT_STOP_FRAME);
    hit_stop_frame *= WorkModule::get_param_float(fighter.module_accessor, hash40("common"), 0x2434ca61df);
    WorkModule::set_int(fighter.module_accessor, hit_stop_frame as i32, *FIGHTER_STATUS_GUARD_DAMAGE_WORK_INT_PREV_SHIELD_SCALE_FRAME);
    let hit_stop_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), 0x20d241cd64);
    ShieldModule::set_hit_stop_mul(fighter.module_accessor, hit_stop_mul);
}

#[skyline::hook(replace = L2CFighterCommon_status_GuardDamage_common)]
unsafe fn status_guarddamage_common(fighter: &mut L2CFighterCommon, param_1: L2CValue) {
    ControlModule::reset_flick_x(fighter.module_accessor);
    ControlModule::reset_flick_sub_x(fighter.module_accessor);
    fighter.global_table[STICK_X].assign(&0xFE.into());
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD);
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_IS_DONE_GUARD_DAMAGE_NUM) {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_GUARD_DAMAGE_NUM);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_IS_DONE_GUARD_DAMAGE_NUM);
        }
        if param_1.get_bool() {
            let prev_shield = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLOAT_PREV_SHIELD);
            let prev_shield_scale = fighter.FighterStatusGuard__calc_shield_scale(prev_shield.into()).get_f32();
            let shield = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);
            let shield_scale = fighter.FighterStatusGuard__calc_shield_scale(shield.into()).get_f32();
            EffectModule::req_follow(
                fighter.module_accessor,
                Hash40::new_raw(0x12c9377e3d),
                Hash40::new("throw"),
                &ZERO_VECTOR,
                &ZERO_VECTOR,
                0.1,
                false,
                *EFFECT_SUB_ATTRIBUTE_NONE as u32,
                0,
                -1,
                *EFFECT_FLIP_NONE,
                1,
                false,
                true
            );
            let boma = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut BattleObjectModuleAccessor;
            let team_color = FighterUtil::get_team_color(boma);
            let effect_team_color = FighterUtil::get_effect_team_color(EColorKind(team_color as i32), Hash40::new("shield_effect_color"));
            EffectModule::set_rgb_partial_last(fighter.module_accessor, effect_team_color.x, effect_team_color.y, effect_team_color.z);
            let handle = EffectModule::req_follow(
                fighter.module_accessor,
                Hash40::new_raw(0x12be304eab),
                Hash40::new("throw"),
                &ZERO_VECTOR,
                &ZERO_VECTOR,
                0.1,
                false,
                *EFFECT_SUB_ATTRIBUTE_NONE as u32,
                0,
                -1,
                *EFFECT_FLIP_NONE,
                1,
                false,
                true
            );
            EffectModule::set_rgb_partial_last(fighter.module_accessor, effect_team_color.x, effect_team_color.y, effect_team_color.z);
            WorkModule::set_int(fighter.module_accessor, handle as i32, *FIGHTER_STATUS_GUARD_ON_WORK_INT_SHIELD_DAMAGE2_EFFECT_HANDLE);
            let handle = EffectModule::req_follow(
                fighter.module_accessor,
                Hash40::new_raw(0x113434cb66),
                Hash40::new("throw"),
                &ZERO_VECTOR,
                &ZERO_VECTOR,
                1.0,
                false,
                *EFFECT_SUB_ATTRIBUTE_NONE as u32,
                0,
                -1,
                *EFFECT_FLIP_NONE,
                1,
                false,
                true
            );
            EffectModule::set_rgb_partial_last(fighter.module_accessor, effect_team_color.x, effect_team_color.y, effect_team_color.z);
            WorkModule::set_int(fighter.module_accessor, handle as i32, *FIGHTER_STATUS_GUARD_ON_WORK_INT_SHIELD_DAMAGE_EFFECT_HANDLE);
            if handle != 0 {
                let diff = (shield_scale / prev_shield_scale) * 0.1;
                EffectModule::set_scale(fighter.module_accessor, handle as u32, &Vector3f{x: diff, y: diff, z: diff});
            }
        }
    }
    else {
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_GUARD_ON_WORK_INT_MIN_FRAME);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_DISABLE_GUARD_FRAME);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_DISABLE_ESCAPE_FRAME);
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_HIT_XLU);
        let just_shield_precede_extension = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("just_shield_precede_extension"));
        ControlModule::set_command_life_extend(fighter.module_accessor, just_shield_precede_extension as u8);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_JUST_SHIELD);
        let boma = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut BattleObjectModuleAccessor;
        FighterUtil::flash_eye_info(boma);
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL) {
            ModelModule::enable_gold_eye(fighter.module_accessor);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLAG_GOLD_EYE);
        }
        EffectModule::req_on_joint(
            fighter.module_accessor,
            Hash40::new("sys_just_shield"),
            Hash40::new("throw"),
            &ZERO_VECTOR,
            &ZERO_VECTOR,
            1.0,
            &ZERO_VECTOR,
            &ZERO_VECTOR,
            false,
            *EFFECT_SUB_ATTRIBUTE_NONE as u32,
            *EFFECT_FLIP_NONE,
            1
        );
        let shield_lr = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLOAT_SHIELD_LR);
        ColorBlendModule::set_last_attack_direction(fighter.module_accessor, &Vector3f{x: -shield_lr, y: 0.0, z: 0.0});
        EffectModule::req_common(fighter.module_accessor, Hash40::new("just_shield"), 0.0);
        if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_GUARD_ON {
            EffectModule::req_screen(fighter.module_accessor, Hash40::new("just_shield_screen"), false, false, false);
        }
        let fighter_kind = fighter.global_table[FIGHTER_KIND].get_i32();
        let se = FighterUtil::get_just_shield_se(fighter_kind);
        SoundModule::play_se(fighter.module_accessor, se, true, false, false, false, enSEType(0));
    }
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_GuardDamageUniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_GuardDamageUniq as *const () as _));
}

#[skyline::hook(replace = L2CFighterCommon_status_GuardDamage_Main)]
unsafe fn status_guarddamage_main(fighter: &mut L2CFighterCommon) -> L2CValue {
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

#[skyline::hook(replace = L2CFighterCommon_sub_ftStatusUniqProcessGuardOff_initStatus)]
unsafe fn sub_ftstatusuniqprocessguardoff_initstatus(_fighter: &mut L2CFighterCommon) -> L2CValue {
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

#[skyline::hook(replace = L2CFighterCommon_status_GuardOff_Common)]
unsafe fn status_guardoff_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    let enabled_terms = [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_GUARD,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
        // Updated transition terms
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_4,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SHOOT_S4,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_100,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_STAND,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_COMMAND1,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL
    ];
    for val in enabled_terms.iter() {
        WorkModule::enable_transition_term(fighter.module_accessor, *val);
    }
    // Original Parry stuff
    // let shield_just_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("shield_just_frame")) as f32;
    // let just_shield_check_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("just_shield_check_frame"), 0);
    // let just_frame = (shield_just_frame * just_shield_check_frame + 0.5) as i32;
    // WorkModule::set_int(fighter.module_accessor, just_frame, *FIGHTER_STATUS_GUARD_ON_WORK_INT_JUST_FRAME);
    let guard_off_cancel_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("guard_off_cancel_frame"));
    WorkModule::set_int(fighter.module_accessor, guard_off_cancel_frame, *FIGHTER_STATUS_GUARD_OFF_WORK_INT_CANCEL_FRAME);
    WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_STATUS_GUARD_OFF_WORK_INT_ATTACK_CANCEL_FRAME);
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
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_guard_off_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_guard_off_uniq as *const () as _));
    motion_rate.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_guard_off_uniq)]
unsafe fn sub_guard_off_uniq(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() {
        if WorkModule::get_int(fighter.module_accessor, FIGHTER_STATUS_GUARD_OFF_WORK_INT_ATTACK_CANCEL_FRAME) < guard_off_attack_cancel_frame {
            WorkModule::inc_int(fighter.module_accessor, FIGHTER_STATUS_GUARD_OFF_WORK_INT_ATTACK_CANCEL_FRAME);
            if WorkModule::get_int(fighter.module_accessor, FIGHTER_STATUS_GUARD_OFF_WORK_INT_ATTACK_CANCEL_FRAME) == guard_off_attack_cancel_frame {
                WorkModule::on_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_GUARD_OFF_ATTACK_CANCEL);
            }
        }
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_OFF_WORK_INT_CANCEL_FRAME) > 0 {
            WorkModule::dec_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_OFF_WORK_INT_CANCEL_FRAME);
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_OFF_WORK_INT_CANCEL_FRAME) == 0 {
                CancelModule::enable_cancel(fighter.module_accessor);
            }
        }
    }
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_status_guard_off_main_common_cancel)]
unsafe fn sub_status_guard_off_main_common_cancel(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !CancelModule::is_enable_cancel(fighter.module_accessor) {
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_GUARD_OFF_ATTACK_CANCEL) {
            if fighter.sub_transition_group_check_ground_jump_mini_attack().get_bool() == false {
                if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_GUARD) {
                    let mut cont = ItemModule::is_have_item(fighter.module_accessor, 0);
                    if cont {
                        fighter.clear_lua_stack();
                        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_THROW);
                        sv_module_access::item(fighter.lua_state_agent);
                        cont = fighter.pop_lua_stack(1).get_bool();
                        if !cont {
                            fighter.clear_lua_stack();
                            lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_SHOOT);
                            sv_module_access::item(fighter.lua_state_agent);
                            cont = fighter.pop_lua_stack(1).get_bool();
                            if cont {
                                cont = ItemModule::get_shoot_item_bullet(fighter.module_accessor, 0) <= 0;
                            }
                        }
                        if cont {
                            fighter.clear_lua_stack();
                            lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_THROW);
                            sv_module_access::item(fighter.lua_state_agent);
                            cont = fighter.pop_lua_stack(1).get_bool();
                            if !cont {
                                if fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0 {
                                    if fighter.global_table[CMD_CAT3].get_i32() & (
                                        *FIGHTER_PAD_CMD_CAT3_ITEM_LIGHT_THROW_HI |
                                        *FIGHTER_PAD_CMD_CAT3_ITEM_LIGHT_THROW_HI4
                                    ) != 0 {
                                        fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), false.into());
                                        return true.into();
                                    }
                                }
                            }
                            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                                if fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER == 0 {
                                    if fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0 {
                                        if fighter.global_table[CMD_CAT3].get_i32() & (
                                            *FIGHTER_PAD_CMD_CAT3_ITEM_LIGHT_THROW_HI |
                                            *FIGHTER_PAD_CMD_CAT3_ITEM_LIGHT_THROW_HI4
                                        ) != 0 {
                                            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), false.into());
                                            return true.into();
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                if fighter.sub_transition_group_check_ground_item().get_bool() == false
                && fighter.sub_transition_group_check_ground_catch().get_bool() == false
                && fighter.sub_transition_group_check_ground_special().get_bool() == false
                && fighter.sub_transition_group_check_ground_attack().get_bool() == false {
                    return false.into();
                }
            }
        }
        return false.into();
    }
    else {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() == false {
            return false.into();
        }
    }
    true.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_status_guard_off_main_common_control)]
unsafe fn sub_status_guard_off_main_common_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_ground_jump().get_bool() == false {
        return false.into();
    }
    true.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_end_GuardOff)]
unsafe fn status_end_guardoff(fighter: &mut L2CFighterCommon) -> L2CValue {
    ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("throw"), &Vector3f{x: 1.0, y: 1.0, z: 1.0});
    WorkModule::unable_transition_term_forbid(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON);
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_STATUS_KIND_JUMP_SQUAT {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_GUARD_OFF_ATTACK_CANCEL);
    }
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_ftstatusuniqprocessguardon_initstatus_common,
            sub_guard_cont_pre,
            sub_guard_on_uniq,
            sub_guard_cont,
            sub_status_end_guard_on_common,
            sub_ftstatusuniqprocessguarddamage_initstatus_inner,
            status_guarddamage_common,
            status_guarddamage_main,
            sub_ftstatusuniqprocessguardoff_initstatus,
            sub_status_guard_on_common,
            status_guardoff_common,
            sub_guard_off_uniq,
            sub_status_guard_off_main_common_cancel,
            sub_status_guard_off_main_common_control,
            status_end_guardoff
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}