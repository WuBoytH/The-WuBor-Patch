#![allow(unused_must_use)]

use {
    smash::{
        lua2cpp::{L2CFighterCommon, *},
        hash40,
        phx::Hash40,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smash_script::*,
    wubor_utils::table_const::*
};

#[skyline::hook(replace = L2CFighterCommon_status_pre_Dash)]
unsafe fn status_pre_dash(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_DashCommon();
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_DASH,
        *GROUND_CORRECT_KIND_GROUND as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        true,
        *FIGHTER_TREADED_KIND_ENABLE,
        true,
        false,
        false,
        0,
        (*FIGHTER_STATUS_ATTR_ENABLE_ROCKETBELT_EJECT | *FIGHTER_STATUS_ATTR_INTO_DOOR) as u32,
        0,
        0
    );
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_pre_TurnDash)]
unsafe fn status_pre_turndash(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_DashCommon();
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_MOTION,
        *GROUND_CORRECT_KIND_GROUND as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        true,
        *FIGHTER_TREADED_KIND_ENABLE,
        true,
        false,
        false,
        0,
        (*FIGHTER_STATUS_ATTR_ENABLE_ROCKETBELT_EJECT | *FIGHTER_STATUS_ATTR_INTO_DOOR) as u32,
        0,
        0
    );
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_DashCommon)]
unsafe fn status_dashcommon(fighter: &mut L2CFighterCommon) {
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
    let transitions = [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_DASH,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_DASH,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE_DASH,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_4,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SHOOT_S4,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_SLIP,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_DASH,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_LIGHT,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_HEAVY,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_COMMAND1,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_LIGHT_DASH,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_HEAVY_DASH,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_COMMAND_623NB,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_STAND,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_SQUAT,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON
    ];
    for val in transitions.iter() {
        WorkModule::enable_transition_term(fighter.module_accessor, *val);
    }
    let turn_dash_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("turn_dash_frame"));
    WorkModule::set_int(fighter.module_accessor, turn_dash_frame, *FIGHTER_STATUS_DASH_WORK_INT_TURN_DASH_FRAME);
    let retry_turn_dash_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("retry_turn_dash_frame"));
    WorkModule::set_int(fighter.module_accessor, retry_turn_dash_frame, *FIGHTER_STATUS_DASH_WORK_INT_RETRY_TURN_DASH_FRAME);
    let dash_enable_attack_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("dash_enable_attack_frame"));
    WorkModule::set_int(fighter.module_accessor, dash_enable_attack_frame, *FIGHTER_STATUS_DASH_WORK_INT_ENABLE_ATTACK_FRAME);
    if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_RUN_BRAKE {
        let run_brake_attack_escape_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("run_brake_attack_escape_frame"));
        WorkModule::set_int(fighter.module_accessor, run_brake_attack_escape_frame - fighter.global_table[0x25].get_i32(), *FIGHTER_STATUS_DASH_WORK_INT_INVALID_ATTACK_ESCAPE_FRAME);
        if 0 < WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_DASH_WORK_INT_INVALID_ATTACK_ESCAPE_FRAME) {
            let untransitions = [
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_DASH,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_DASH,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE_DASH,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_4,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SHOOT_S4,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_DASH,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_COMMAND1,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_COMMAND_623NB,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_STAND,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_SQUAT
            ];
            for val in untransitions.iter() {
                WorkModule::unable_transition_term(fighter.module_accessor, *val);
            }
        }
    }
}

#[skyline::hook(replace = L2CFighterCommon_status_Dash_Main_common)]
unsafe fn status_dash_main_common(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if fighter.global_table[DASH_COMMON_PRE].get_bool() != false && {
        let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[DASH_COMMON_PRE].get_ptr());
        callable(fighter).get_bool()
    } {
        return 1.into();
    }

    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }

    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        return 1.into();
    }

    if fighter.sub_transition_group_check_ground_jump_mini_attack().get_bool() {
        return 1.into();
    }

    let mut can_s4 = true;
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DASH_FLAG_NO_S4) {
        can_s4 = fighter.global_table[STICK_X].get_f32() * PostureModule::lr(fighter.module_accessor) < WorkModule::get_param_float(fighter.module_accessor, hash40("common"), 0x206138766c)
    }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW)
    && can_s4 {
        let mut throw = false;
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_THROW);
        sv_module_access::item(fighter.lua_state_agent);
        if fighter.pop_lua_stack(1).get_bool() {
            throw = fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0;
        }
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_SHOOT);
        sv_module_access::item(fighter.lua_state_agent);
        if fighter.pop_lua_stack(1).get_bool() {
            let item_bullet = ItemModule::get_shoot_item_bullet(fighter.module_accessor, 0);
            if item_bullet <= 0 {
                throw = true;
            }
            else {
                throw = false;
            }
        }
        if throw {
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), false.into());
            return 1.into();
        }
    }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_DASH) && {
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_THROW);
        sv_module_access::item(fighter.lua_state_agent);
        fighter.pop_lua_stack(1).get_bool() 
    } && fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0 {
        fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW_DASH.into(), false.into());
        return 1.into();
    }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE)
    && ItemModule::is_have_item(fighter.module_accessor, 0)
    && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH != 0 && {
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_NO_THROW);
        sv_module_access::item(fighter.lua_state_agent);
        fighter.pop_lua_stack(1).get_bool() == false
     } {
        fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), false.into());
        return 1.into();
    }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE_DASH)
    && ItemModule::is_have_item(fighter.module_accessor, 0)
    && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH != 0 && {
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_NO_THROW);
        sv_module_access::item(fighter.lua_state_agent);
        fighter.pop_lua_stack(1).get_bool() == false
    } {
        fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW_DASH.into(), false.into());
        return 1.into();
    }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN) && {
        let stick_x = fighter.global_table[STICK_X].get_f32();
        let lr = PostureModule::lr(fighter.module_accessor);
        let turn_run_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("turn_run_stick_x"));
        stick_x * lr <= turn_run_stick_x
    } && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH != 0
    && !ItemModule::is_have_item(fighter.module_accessor, 0) {
        fighter.change_status(FIGHTER_STATUS_KIND_CATCH_TURN.into(), true.into());
        return 1.into();
    }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH)
    && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH != 0
    && !ItemModule::is_have_item(fighter.module_accessor, 0) {
        fighter.change_status(FIGHTER_STATUS_KIND_CATCH_DASH.into(), true.into());
        return 1.into();
    }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE)
    && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        fighter.change_status(FIGHTER_STATUS_KIND_GUARD_ON.into(), true.into());
        return 1.into();
    }

    if fighter.sub_transition_group_check_special_command().get_bool() {
        return true.into();
    }

    if fighter.sub_transition_group_check_ground_special().get_bool() {
        return true.into();
    }

    if fighter.sub_transition_specialflag_hoist().get_bool() {
        return true.into();
    }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_4) && {
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_SWING);
        sv_module_access::item(fighter.lua_state_agent);
        fighter.pop_lua_stack(1).get_bool()
    } && fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_DASH_ATTACK_S4 != 0
    && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DASH_FLAG_NO_S4) {
        fighter.change_status(FIGHTER_STATUS_KIND_ITEM_SWING_S4_START.into(), true.into());
        return 1.into();
    }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SHOOT_S4) && {
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_SHOOT);
        sv_module_access::item(fighter.lua_state_agent);
        fighter.pop_lua_stack(1).get_bool()
    } && fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0
    && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DASH_FLAG_NO_S4) {
        if ItemModule::get_shoot_item_bullet(fighter.module_accessor, 0) > 0 {
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_SHOOT_WAIT.into(), true.into());
            return 1.into();
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), true.into());
            return 1.into();
        }
    }

    if fighter.global_table[ATTACK_S4_PRE].get_bool() && {
        let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[ATTACK_S4_PRE].get_ptr());
        callable(fighter).get_bool()
    } {
        return 1.into();
    }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START)
    && fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_DASH_ATTACK_S4 != 0
    && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DASH_FLAG_NO_S4) == false {
        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_S4_START.into(), true.into());
        return 1.into();
    }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_DASH) && {
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_SWING);
        sv_module_access::item(fighter.lua_state_agent);
        fighter.pop_lua_stack(1).get_bool()
    } && fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0 {
        fighter.change_status(FIGHTER_STATUS_KIND_ITEM_SWING_DASH.into(), true.into());
        return 1.into();
    }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_DASH)
    && fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0 {
        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_DASH.into(), true.into());
        return 1.into();
    }

    if fighter.sub_transition_group_check_ground_attack().get_bool() {
        return true.into();
    }

    if 0 < WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_DASH_WORK_INT_ENABLE_ATTACK_FRAME)
    && (fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0
    || FighterUtil::is_valid_auto_catch_item(fighter.module_accessor, false)) {
        if fighter.global_table[CMD_CAT1].get_i32() & (
            *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4 |
            *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3 |
            *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3
        ) != 0 && {
            fighter.clear_lua_stack();
            lua_args!(fighter, MA_MSC_ITEM_IS_PICKABLE_ITEM_HEAVY);
            sv_module_access::item(fighter.lua_state_agent);
            fighter.pop_lua_stack(1).get_bool()
        }
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_HEAVY_DASH)
        && !ItemModule::is_have_item(fighter.module_accessor, 0) {
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP.into(), true.into());
            return true.into();
        }
        if ItemModule::get_pickable_item_size(fighter.module_accessor) == *ITEM_SIZE_LIGHT as u64
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_LIGHT_DASH)
        && {
            fighter.clear_lua_stack();
            lua_args!(fighter, MA_MSC_CMD_ITEM_IS_GET_PICKABLE_ITEM);
            sv_module_access::item(fighter.lua_state_agent);
            fighter.pop_lua_stack(1).get_bool()
        } {
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_LIGHT_PICKUP.into(), true.into());
            return true.into();
        }
    }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH) 
    && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH != 0 {
        fighter.change_status(FIGHTER_STATUS_KIND_TURN_DASH.into(), true.into());
        return 1.into();
    }

    if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH != 0 && {
        let frame = MotionModule::frame(fighter.module_accessor);
        let re_dash_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("re_dash_frame")) as f32;
        re_dash_frame <= frame
    } {
        fighter.change_status(FIGHTER_STATUS_KIND_DASH.into(), true.into());
        return 1.into();
    }

    if fighter.sub_transition_group_check_ground_jump().get_bool() == false {
        if param_1.get_bool() {
            let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(param_1.get_ptr());
            if callable(fighter).get_bool() {
                return 1.into();
            }
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN) && {
            let stick_x = fighter.global_table[STICK_X].get_f32();
            let lr = PostureModule::lr(fighter.module_accessor);
            let run_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("run_stick_x"));
            run_stick_x <= stick_x * lr
        } {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_RUN) {
                fighter.change_status(FIGHTER_STATUS_KIND_WALK.into(), true.into());
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_RUN.into(), true.into());
            }
            return 1.into();
        }
        if GroundModule::get_down_friction(fighter.module_accessor) < 1.0
        && FighterMotionModuleImpl::is_valid_cancel_frame(fighter.module_accessor, -1, true) {
            fighter.change_status(FIGHTER_STATUS_KIND_WALK_BRAKE.into(), false.into());
            return 1.into();
        }
        if !MotionModule::is_end(fighter.module_accessor) {
            if fighter.sub_ground_check_stop_wall().get_bool() {
                return 1.into();
            }
            else {
                return 0.into();
            }
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            return 1.into();
        }
    }
    else {
        return 1.into();
    }
}

pub unsafe fn fgc_dashback_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_DashCommon();
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_DASH_BACK,
        *GROUND_CORRECT_KIND_GROUND as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        true,
        *FIGHTER_TREADED_KIND_ENABLE,
        true,
        false,
        false,
        0,
        (*FIGHTER_STATUS_ATTR_ENABLE_ROCKETBELT_EJECT | *FIGHTER_STATUS_ATTR_INTO_DOOR) as u32,
        0,
        0
    );
    0.into()
}

pub unsafe fn fgc_dashback_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("dash_b"), 0.0, 1.0, false, 0.0, false, false);
    fighter.status_DashCommon();
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_DASH);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_DASH);
    GroundModule::set_reverse_direction(fighter.module_accessor, true, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(fgc_dashback_main_loop as *const () as _))
}

unsafe extern "C" fn fgc_dashback_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), 1.into());
        return 1.into();
    }

    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        return 1.into();
    }

    if fighter.sub_transition_group_check_ground_jump_mini_attack().get_bool() {
        return 1.into();
    }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW) {
        let mut throw = false;
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_THROW);
        sv_module_access::item(fighter.lua_state_agent);
        if fighter.pop_lua_stack(1).get_bool() {
            throw = fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0;
        }
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_SHOOT);
        sv_module_access::item(fighter.lua_state_agent);
        if fighter.pop_lua_stack(1).get_bool() {
            let item_bullet = ItemModule::get_shoot_item_bullet(fighter.module_accessor, 0);
            if item_bullet <= 0 {
                throw = true;
            }
            else {
                throw = false;
            }
        }
        if throw {
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), false.into());
            return 1.into();
        }
    }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE)
    && ItemModule::is_have_item(fighter.module_accessor, 0)
    && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH != 0 && {
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_NO_THROW);
        sv_module_access::item(fighter.lua_state_agent);
        fighter.pop_lua_stack(1).get_bool() == false
    } {
        fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), false.into());
        return 1.into();
    }

    // original
    // if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE) {
    //     if WorkMoudle::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH)
    //     && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE_F != 0 {
    //         fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_F.into(), true.into());
    //         return 1.into();
    //     }
    //     if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
    //         if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DASH_FLAG_NO_ESCAPE) {
    //             fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_B.into(), true.into());
    //             return 1.into();
    //         }
    //     }
    // }

    // new
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE)
    && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        fighter.change_status(FIGHTER_STATUS_KIND_GUARD_ON.into(), true.into());
        return 1.into();
    }

    if fighter.sub_transition_group_check_special_command().get_bool() {
        return 1.into();
    }

    if fighter.sub_transition_group_check_ground_special().get_bool() {
        return true.into();
    }

    if fighter.sub_transition_specialflag_hoist().get_bool() {
        return true.into();
    }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_4) && {
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_SWING);
        sv_module_access::item(fighter.lua_state_agent);
        fighter.pop_lua_stack(1).get_bool()
    } && fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_DASH_ATTACK_S4 != 0
    && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DASH_FLAG_NO_S4) {
        fighter.change_status(FIGHTER_STATUS_KIND_ITEM_SWING_S4_START.into(), true.into());
        return 1.into();
    }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SHOOT_S4) && {
        fighter.clear_lua_stack();
        lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_SHOOT);
        sv_module_access::item(fighter.lua_state_agent);
        fighter.pop_lua_stack(1).get_bool()
    } && fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0
    && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DASH_FLAG_NO_S4) {
        if ItemModule::get_shoot_item_bullet(fighter.module_accessor, 0) <= 0 {
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_SHOOT_WAIT.into(), true.into());
            return 1.into();
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), true.into());
            return 1.into();
        }
    }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START)
    && fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_DASH_ATTACK_S4 != 0
    && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DASH_FLAG_NO_S4) {
        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_S4_START.into(), true.into());
        return 1.into();
    }

    if fighter.sub_transition_group_check_ground_attack().get_bool() {
        return true.into();
    }

    if 0 < WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_DASH_WORK_INT_ENABLE_ATTACK_FRAME)
    && (fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0
    || FighterUtil::is_valid_auto_catch_item(fighter.module_accessor, false)) {
        if fighter.global_table[CMD_CAT1].get_i32() & (
            *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4 |
            *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3 |
            *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3
        ) != 0 && {
            fighter.clear_lua_stack();
            lua_args!(fighter, MA_MSC_ITEM_IS_PICKABLE_ITEM_HEAVY);
            sv_module_access::item(fighter.lua_state_agent);
            fighter.pop_lua_stack(1).get_bool()
        } && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_HEAVY_DASH)
        && !ItemModule::is_have_item(fighter.module_accessor, 0) {
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP.into(), true.into());
            return true.into();
        }

        if ItemModule::get_pickable_item_size(fighter.module_accessor) == *ITEM_SIZE_LIGHT as u64
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_LIGHT_DASH)
        && {
            fighter.clear_lua_stack();
            lua_args!(fighter, MA_MSC_CMD_ITEM_IS_GET_PICKABLE_ITEM);
            sv_module_access::item(fighter.lua_state_agent);
            fighter.pop_lua_stack(1).get_bool()
        } {
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_LIGHT_PICKUP.into(), true.into());
            return true.into();
        }
    }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH)
    && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH != 0 {
        fighter.change_status(FIGHTER_STATUS_KIND_DASH.into(), true.into());
    }
    
    if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH != 0 && {
        let frame = MotionModule::frame(fighter.module_accessor);
        let re_dash_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("re_dash_frame")) as f32;
        re_dash_frame <= frame
    } {
        fighter.change_status(FIGHTER_STATUS_KIND_TURN_DASH.into(), true.into());
        return 1.into();
    }

    if fighter.sub_transition_group_check_ground_jump().get_bool() == false {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN) && {
            let stick_x = fighter.global_table[STICK_X].get_f32();
            let lr = PostureModule::lr(fighter.module_accessor);
            let run_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("run_stick_x"));
            run_stick_x <= stick_x * lr * -1.0
        } && fighter.global_table[CMD_CAT1].get_i32() & (
            *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4 |
            *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3 |
            *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3 |
            *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N | *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N |
            *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S | *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI |
            *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW
        ) == 0 {
            let kind;
            if fighter.global_table[FIGHTER_KIND].get_i32() == *FIGHTER_KIND_DOLLY {
                kind = FIGHTER_DOLLY_STATUS_KIND_TURN_RUN_BACK;
            }
            else if fighter.global_table[FIGHTER_KIND].get_i32() == *FIGHTER_KIND_DEMON {
                kind = FIGHTER_DEMON_STATUS_KIND_TURN_RUN_BACK;
            }
            else  {
                kind = FIGHTER_RYU_STATUS_KIND_TURN_RUN_BACK;
            }
            fighter.change_status(kind.into(), false.into());
            return 1.into();
        }
        if MotionModule::is_end(fighter.module_accessor) {
            if GroundModule::get_down_friction(fighter.module_accessor) < 1.0 {
                fighter.change_status(FIGHTER_STATUS_KIND_RUN_BRAKE.into(), false.into());
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            }
            return 1.into();
        }
        else {
            return 0.into();
        }
    }
    else {
        return 1.into();
    }
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pre_dash,
            status_pre_turndash,
            status_dashcommon,
            status_dash_main_common
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}