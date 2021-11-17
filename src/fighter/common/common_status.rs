#![allow(unused_must_use)]

use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        phx::{Hash40, Vector3f},
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smash_script::*,
    smashline::*,
    crate::{
        common_funcs::*,
        vars::*,
        table_const::*
    }
};

#[common_status_script(status = FIGHTER_STATUS_KIND_DAMAGE_FALL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn common_status_damagefall(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_DamageFall_common();
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_AIR) {
        KineticModule::add_speed(fighter.module_accessor, &Vector3f {x: 1.0, y: 0.0, z: 0.0});
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(common_status_damagefall_main as *const () as _))
}

unsafe extern "C" fn common_status_damagefall_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() == true
    || fighter.check_damage_fall_transition().get_bool() == true {
        return 0.into();
    }
    let tech : bool;
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_GROUND) == false {
        tech = ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD);
    }
    else {
        // let mut flame_choke_tech_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("ganon_special_s_passive_trigger_frame")) as f32;
        // let tech_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("passive_trigger_frame_mul"), 0 as u64);
        // flame_choke_tech_frame *= tech_mul;
        tech = fighter.sub_check_passive_button(L2CValue::I32(0x30)).get_bool();
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_FB)
    && FighterUtil::is_touch_passive_ground(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32)
    && WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("passive_fb_cont_value")) <= fighter.global_table[STICK_X].get_f32().abs()
    && tech {
        fighter.change_status(FIGHTER_STATUS_KIND_PASSIVE_FB.into(), true.into());
        return L2CValue::Bool(true);
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE) {
        if FighterUtil::is_touch_passive_ground(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
            if FighterStopModuleImpl::is_damage_stop(fighter.module_accessor) == false {
                if tech {
                    fighter.change_status(FIGHTER_STATUS_KIND_PASSIVE.into(), true.into());
                    return true.into();
                }
            }
        }
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DOWN)
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_DOWN.into(), true.into());
        return 0.into();
    }
    fighter.sub_damage_fall_uniq_process_exec_fix_pos();
    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_DashCommon)]
pub unsafe fn dash_common(fighter: &mut L2CFighterCommon) {
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_DASH);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_DASH);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE_DASH);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_4);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SHOOT_S4);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_SLIP);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_DASH);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_LIGHT);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_HEAVY);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_COMMAND1);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_LIGHT_DASH);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_HEAVY_DASH);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_COMMAND_623NB);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_STAND);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_SQUAT);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON);
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
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_DASH);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_DASH);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE_DASH);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_4);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SHOOT_S4);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_DASH);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_COMMAND1);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_COMMAND_623NB);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_STAND);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_SQUAT);
        }
    }
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_Dash_Main_common)]
pub unsafe fn dash_main_common(fighter: &mut L2CFighterCommon, param_1 : L2CValue) -> L2CValue {
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
        let common_guard_hold = ControlModule::get_command_life(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY2, 0x18) as i32;
        WorkModule::set_int(fighter.module_accessor, common_guard_hold, FIGHTER_INSTANCE_WORK_ID_INT_GUARD_HOLD_FRAME);
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
        let common_guard_hold = ControlModule::get_command_life(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY2, 0x18) as i32;
        WorkModule::set_int(fighter.module_accessor, common_guard_hold, FIGHTER_INSTANCE_WORK_ID_INT_GUARD_HOLD_FRAME);
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
            // this part *shouldn't* matter because it's all the same value constant, but just to be safe...
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

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_ftStatusUniqProcessGuardOn_initStatus_common)]
pub unsafe fn sub_ftstatusuniqprocessguardon_initstatus_common(fighter: &mut L2CFighterCommon) {
    // Original
    ShieldModule::set_status(fighter.module_accessor, *FIGHTER_SHIELD_KIND_GUARD, ShieldStatus(*SHIELD_STATUS_NORMAL), 0);
    let guard_off_disable_shield_recovery = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("guard_off_disable_shield_recovery"));
    WorkModule::set_int(fighter.module_accessor, guard_off_disable_shield_recovery, *FIGHTER_INSTANCE_WORK_ID_INT_DISABLE_SHIELD_RECOVERY_FRAME);
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
        ShieldModule::set_status(fighter.module_accessor, *FIGHTER_SHIELD_KIND_GUARD, ShieldStatus(*SHIELD_STATUS_NORMAL), 0);
        ShieldModule::set_shield_type(fighter.module_accessor, ShieldType(*SHIELD_TYPE_JUST_SHIELD), *FIGHTER_SHIELD_KIND_GUARD, 0);
        if FighterUtil::is_valid_just_shield_reflector(fighter.module_accessor) {
            ReflectorModule::set_status(fighter.module_accessor, 0, ShieldStatus(*SHIELD_STATUS_NORMAL), *FIGHTER_REFLECTOR_GROUP_JUST_SHIELD);
        }
        fighter.FighterStatusGuard__set_just_shield_scale();
    }
    // Also Original, but moved down
    let hit_stop_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), 0x20d241cd64u64);
    ShieldModule::set_hit_stop_mul(fighter.module_accessor, hit_stop_mul);
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
            if !IS_FGC[entry_id(fighter.module_accessor)] {
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
        let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[DASH_COMMON_PRE].get_ptr());
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
    let guard_off_cancel_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("guard_off_cancel_frame"));
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

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_attack_air_common)]
pub unsafe fn sub_attack_air_common(fighter: &mut L2CFighterCommon, param_1: L2CValue) {
    AIR_WHIFF[entry_id(fighter.module_accessor)] = false;
    ControlModule::reset_trigger(fighter.module_accessor);
    ControlModule::reset_flick_y(fighter.module_accessor);
    ControlModule::reset_flick_sub_y(fighter.module_accessor);
    fighter.global_table[FLICK_Y].assign(&0xfe.into());
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.attack_air_uniq(false.into());
    }
    let bind_address_call_attack_air_uniq = smash::lua2cpp::L2CFighterCommon_bind_address_call_attack_air_uniq;
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(bind_address_call_attack_air_uniq as *const () as _));
    if param_1.get_bool() == true {
        fighter.sub_attack_air_kind();
    }
    ControlModule::reset_attack_air_kind(fighter.module_accessor);
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_AttackAir_Main_common)]
pub unsafe fn status_attackair_main_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
    || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
        AIR_WHIFF[entry_id(fighter.module_accessor)] = false;
    }
    else if AttackModule::is_attack(fighter.module_accessor, 0, false) {
        AIR_WHIFF[entry_id(fighter.module_accessor)] = true;
    }
    if fighter.attack_air_common_strans().get_bool() == false {
        if CancelModule::is_enable_cancel(fighter.module_accessor) == false {
            if MotionModule::is_end(fighter.module_accessor) == false {
                return false.into();
            }
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            if fighter.sub_wait_ground_check_common(false.into()).get_bool() == false {
                if fighter.sub_air_check_fall_common().get_bool() {
                    return true.into();
                }
                if MotionModule::is_end(fighter.module_accessor) == false {
                    return false.into();
                }
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
        }
    }
    true.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_landing_attack_air_init)]
pub unsafe fn sub_landing_attack_air_init(fighter: &mut L2CFighterCommon, param_1: L2CValue, param_2: L2CValue, param_3: L2CValue) {
    let mot = param_1.get_int();
    let mut landing_lag = WorkModule::get_param_float(fighter.module_accessor, param_2.get_int(), 0) + param_3.get_f32();
    if AIR_WHIFF[entry_id(fighter.module_accessor)] {
        landing_lag += 4.0;
    }
    let mut motion_rate = 1.0;
    if landing_lag != 0.0 {
        motion_rate = fighter.sub_get_landing_motion_rate(mot.into(), landing_lag.into()).get_f32();
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_LANDING_CANCEL);
    }
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
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_DamageAir_Main)]
pub unsafe fn damage_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !IS_FGC[entry_id(fighter.module_accessor)] {
        ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
    }
    call_original!(fighter)
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_transition_group_check_ground_guard)]
pub unsafe fn sub_transition_group_check_ground_guard(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        if fighter.global_table[CHECK_GROUND_GUARD_PRE].get_bool() {
            let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[CHECK_GROUND_GUARD_PRE].get_ptr());
            if callable(fighter).get_bool() {
                return true.into();
            }
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON) {
            if fighter.sub_check_command_guard().get_bool() {
                let common_guard_hold = ControlModule::get_command_life(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY2, 0x18) as i32;
                WorkModule::set_int(fighter.module_accessor, common_guard_hold, FIGHTER_INSTANCE_WORK_ID_INT_GUARD_HOLD_FRAME);
                fighter.change_status(FIGHTER_STATUS_KIND_GUARD_ON.into(), true.into());
                return true.into();
            }
        }
    }
    false.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_transition_group_check_air_tread_jump)]
pub unsafe fn sub_transition_group_check_air_tread_jump(fighter: &mut L2CFighterCommon) -> L2CValue {
    let cont;
    if fighter.global_table[CHECK_AIR_TREAD_JUMP_PRE].get_bool() == false {
        cont = false;
    }
    else {
        let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[CHECK_AIR_TREAD_JUMP_PRE].get_ptr());
        cont = callable(fighter).get_bool();
    }
    if cont == false {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP != 0 {
                if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TREAD_JUMP)
                && ControlModule::is_enable_flick_jump(fighter.module_accessor) {
                    let do_footstool;
                    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_NO_TREAD_FRAME) != 0 {
                        do_footstool = false;
                    }
                    else {
                        let tread_speed_y = fighter.FL_sub_fighter_float_next_tread_speed_y().get_f32();
                        let tread_jump_speed_limit = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("tread_jump_speed_limit"));
                        if !(tread_jump_speed_limit <= tread_speed_y) {
                            do_footstool = false;
                        }
                        else {
                            fighter.clear_lua_stack();
                            lua_args!(fighter, 0x21bfbd3f83u64);
                            smash::app::sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
                            do_footstool = fighter.pop_lua_stack(1).get_bool();
                        }
                    }
                    if do_footstool {
                        fighter.change_status(FIGHTER_STATUS_KIND_TREAD_JUMP.into(), true.into());
                        return true.into();
                    }
                }
            }
            if fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_JUMP_TRIGGER != 0
            || fighter.global_table[CMD_CAT2].get_i32() & (
                *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI |
                *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L |
                *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R |
                *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_LW
            ) != 0 /* this is the addition */ {
                if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TREAD_JUMP_BUTTON) {
                    let do_footstool;
                    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_NO_TREAD_FRAME) != 0 {
                        do_footstool = false;
                    }
                    else {
                        let tread_speed_y = fighter.FL_sub_fighter_float_next_tread_speed_y().get_f32();
                        let tread_jump_speed_limit = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("tread_jump_speed_limit"));
                        if !(tread_jump_speed_limit <= tread_speed_y) {
                            do_footstool = false;
                        }
                        else {
                            fighter.clear_lua_stack();
                            lua_args!(fighter, 0x21bfbd3f83u64);
                            smash::app::sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
                            do_footstool = fighter.pop_lua_stack(1).get_bool();
                        }
                    }
                    if do_footstool {
                        fighter.change_status(FIGHTER_STATUS_KIND_TREAD_JUMP.into(), true.into());
                        return true.into();
                    }
                }
            }
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TREAD_JUMP_NO_TRIGGER) {
                fighter.clear_lua_stack();
                lua_args!(fighter, 0x21bfbd3f83u64, true);
                smash::app::sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
                if fighter.pop_lua_stack(1).get_bool() {
                    fighter.change_status(FIGHTER_STATUS_KIND_TREAD_JUMP.into(), false.into());
                    return true.into();
                }
            }
        }
    }
    else {
        return true.into();
    }
    false.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_transition_group_check_special_command)]
pub unsafe fn sub_transition_group_check_special_command(fighter: &mut L2CFighterCommon) -> L2CValue {
    // if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_SPECIAL_COMMAND) {
        if fighter.global_table[CHECK_SPECIAL_COMMAND].get_bool() == false {
            return false.into();
        }
        else {
            let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[CHECK_SPECIAL_COMMAND].get_ptr());
            if callable(fighter).get_bool() {
                return true.into();
            }
        }
    // }
    false.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            dash_common,
            dash_main_common,
            sub_ftstatusuniqprocessguardon_initstatus_common,
            sub_guard_cont_pre,
            sub_guard_on_uniq,
            sub_guard_cont,
            sub_status_end_guard_on_common,
            sub_ftstatusuniqprocessguardoff_initstatus,
            sub_status_guard_on_common,
            status_guardoff_common,
            sub_attack_air_common,
            status_attackair_main_common,
            sub_landing_attack_air_init,
            damage_air_main,
            sub_transition_group_check_ground_guard,
            sub_transition_group_check_air_tread_jump,
            sub_transition_group_check_special_command
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
    install_status_scripts!(
        common_status_damagefall
    );
}