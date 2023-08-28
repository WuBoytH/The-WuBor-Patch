use crate::imports::status_imports::*;
use crate::fighter::common::status::attack::attack::*;

#[skyline::hook(replace = L2CFighterCommon_sub_transition_group_check_ground_jump_mini_attack)]
unsafe fn sub_transition_group_check_ground_jump_mini_attack(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        if fighter.global_table[CHECK_GROUND_JUMP_MINI_ATTACK].get_bool() {
            let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[CHECK_GROUND_JUMP_MINI_ATTACK].get_ptr());
            return callable(fighter);
        }
        let cat1 = fighter.global_table[CMD_CAT1].get_i32();
        let check_attack_input = if
            [
                *FIGHTER_STATUS_KIND_GUARD_ON,
                *FIGHTER_STATUS_KIND_GUARD,
                *FIGHTER_STATUS_KIND_GUARD_OFF,
                *FIGHTER_STATUS_KIND_GUARD_DAMAGE,
            ].contains(&fighter.global_table[STATUS_KIND_INTERRUPT].get_i32()) {
            cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH != 0 || cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0
        }
        else {
            cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH == 0 && cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0
        };
        if check_attack_input
        && fighter.sub_check_button_jump().get_bool() {
            fighter.change_status_jump_mini_attack(false.into());
            return true.into();
        }
    }
    false.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_transition_group_check_ground_guard)]
unsafe fn sub_transition_group_check_ground_guard(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        if fighter.global_table[CHECK_GROUND_GUARD_UNIQ].get_bool() {
            let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[CHECK_GROUND_GUARD_UNIQ].get_ptr());
            if callable(fighter).get_bool() {
                return true.into();
            }
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON)
        && fighter.sub_check_command_guard().get_bool() {
            let guard_trigger = ControlModule::get_trigger_count(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD as u8) & 0xFF;
            VarModule::set_int(fighter.battle_object, fighter::instance::int::GUARD_TRIGGER, guard_trigger);
            let clear_buffer = fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH == 0;
            fighter.change_status(FIGHTER_STATUS_KIND_GUARD_ON.into(), clear_buffer.into());
            return true.into();
        }
    }
    false.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_transition_group_check_ground_catch)]
unsafe fn sub_transition_group_check_ground_catch(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[CHECK_GROUND_CATCH_UNIQ].get_bool() {
        let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[CHECK_GROUND_CATCH_UNIQ].get_ptr());
        if callable(fighter).get_bool() {
            return true.into();
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        let invalid_catch_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_INVALID_CATCH_FRAME);
        if invalid_catch_frame == 0 {
            let cat1 = fighter.global_table[CMD_CAT1].get_i32();
            if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH != 0 {
                if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH) {
                    if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH != 0 {
                        fighter.change_status(FIGHTER_STATUS_KIND_CATCH_DASH.into(), true.into());
                        return true.into();
                    }
                    if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH != 0 {
                        fighter.change_status(FIGHTER_STATUS_KIND_CATCH_TURN.into(), true.into());
                        return true.into();
                    }
                    fighter.change_status(FIGHTER_STATUS_KIND_CATCH.into(), true.into());
                    return true.into();
                }
            }
        }
    }
    false.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_transition_group_check_ground_item)]
unsafe fn sub_transition_group_check_ground_item(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH != 0
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE)
        && ItemModule::is_have_item(fighter.module_accessor, 0)
        && {
            fighter.clear_lua_stack();
            lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_NO_THROW);
            sv_module_access::item(fighter.lua_state_agent);
            !fighter.pop_lua_stack(1).get_bool()
        } {
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), false.into());
            return true.into();
        }
        if fighter.global_table[CMD_CAT3].get_i32() & *FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_ALL != 0
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW)
        && {
            fighter.clear_lua_stack();
            lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_THROW);
            sv_module_access::item(fighter.lua_state_agent);
            fighter.pop_lua_stack(1).get_bool()
        }
        || {
            fighter.clear_lua_stack();
            lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_SHOOT);
            sv_module_access::item(fighter.lua_state_agent);
            if fighter.pop_lua_stack(1).get_bool() {
                ItemModule::get_shoot_item_bullet(fighter.module_accessor, 0) <= 0
            }
            else {
                false
            }
        } {
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), false.into());
            return true.into();
        }
    }
    false.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_transition_group_check_ground_attack)]
unsafe fn sub_transition_group_check_ground_attack(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[CHECK_GROUND_ATTACK_UNIQ].get_bool() {
        let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[CHECK_GROUND_ATTACK_UNIQ].get_ptr());
        if callable(fighter).get_bool() {
            return true.into();
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        if !CancelModule::is_enable_cancel(fighter.module_accessor) {
            FGCModule::set_used_ground_normal_transition_terms(fighter);
        }
        if fighter.sub_transition_specialflag_hoist().get_bool() {
            return true.into();
        }
        if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0 {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPECIALFLAG_HOIST) {
                let item = ItemModule::get_have_item_kind(fighter.module_accessor, 0);
                if item == *ITEM_KIND_SPECIALFLAG || item == *ITEM_KIND_BOMBER {
                    fighter.change_status(FIGHTER_STATUS_KIND_ITEM_SPECIALFLAG_HOIST.into(), true.into());
                }
            }
        }
        if fighter.global_table[CHECK_ATTACK_S4_UNIQ].get_bool() {
            let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[CHECK_ATTACK_S4_UNIQ].get_ptr());
            if callable(fighter).get_bool() {
                return true.into();
            }
        }
        if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4 != 0 {
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_4) {
                fighter.clear_lua_stack();
                lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_SWING);
                sv_module_access::item(fighter.lua_state_agent);
                if fighter.pop_lua_stack(1).get_bool() {
                    fighter.change_status(FIGHTER_STATUS_KIND_ITEM_SWING_S4_START.into(), true.into());
                    return true.into();
                }
            }
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SHOOT_S4) {
                fighter.clear_lua_stack();
                lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_SHOOT);
                sv_module_access::item(fighter.lua_state_agent);
                if fighter.pop_lua_stack(1).get_bool() {
                    fighter.change_status(FIGHTER_STATUS_KIND_ITEM_SHOOT_WAIT.into(), true.into());
                    return true.into();
                }
            }
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START) {
                fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_S4_START.into(), true.into());
                return true.into();
            }
        }
        if fighter.global_table[CHECK_ATTACK_HI4_UNIQ].get_bool() {
            let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[CHECK_ATTACK_HI4_UNIQ].get_ptr());
            if callable(fighter).get_bool() {
                return true.into();
            }
        }
        if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 != 0
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START) {
            fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_HI4_START.into(), true.into());
            return true.into();
        }
        if fighter.global_table[CHECK_ATTACK_LW4_UNIQ].get_bool() {
            let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[CHECK_ATTACK_LW4_UNIQ].get_ptr());
            if callable(fighter).get_bool() {
                return true.into();
            }
        }
        if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4 != 0
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START) {
            fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_LW4_START.into(), true.into());
            return true.into();
        }
        if fighter.global_table[CHECK_ATTACK_3_UNIQ].get_bool() {
            let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[CHECK_ATTACK_3_UNIQ].get_ptr());
            if callable(fighter).get_bool() {
                return true.into();
            }
        }
        if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3 != 0
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3) {
            fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_HI3.into(), true.into());
            return true.into();
        }
        if FighterUtil::is_valid_auto_catch_item(fighter.module_accessor, true)
        || fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_HEAVY) {
            fighter.clear_lua_stack();
            lua_args!(fighter, MA_MSC_ITEM_IS_PICKABLE_ITEM_HEAVY);
            sv_module_access::item(fighter.lua_state_agent);
            if fighter.pop_lua_stack(1).get_bool()
            && !ItemModule::is_have_item(fighter.module_accessor, 0) {
                fighter.change_status(FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP.into(), true.into());
                return true.into();
            }
        }
        if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3 != 0
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_3) {
            fighter.clear_lua_stack();
            lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_SWING);
            sv_module_access::item(fighter.lua_state_agent);
            if fighter.pop_lua_stack(1).get_bool() {
                fighter.change_status(FIGHTER_STATUS_KIND_ITEM_SWING_S3.into(), true.into());
                return true.into();
            }
        }
        if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3 != 0
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3) {
            fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_LW3.into(), true.into());
            return true.into();
        }
        if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3 != 0 {
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SHOOT_S3) {
                fighter.clear_lua_stack();
                lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_SHOOT);
                sv_module_access::item(fighter.lua_state_agent);
                if fighter.pop_lua_stack(1).get_bool() {
                    fighter.change_status(FIGHTER_STATUS_KIND_ITEM_SHOOT_WAIT.into(), true.into());
                    return true.into();
                }
            }
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3) {
                fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_S3.into(), true.into());
                return true.into();
            }
        }
        if fighter.global_table[CHECK_ATTACK_N_UNIQ].get_bool() {
            let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[CHECK_ATTACK_N_UNIQ].get_ptr());
            if callable(fighter).get_bool() {
                return true.into();
            }
        }
        if FighterUtil::is_valid_auto_catch_item(fighter.module_accessor, true)
        || fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0 {
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_LIGHT)
            && ItemModule::get_pickable_item_size(fighter.module_accessor) == *ITEM_SIZE_LIGHT as u64 {
                fighter.clear_lua_stack();
                lua_args!(fighter, MA_MSC_CMD_ITEM_IS_GET_PICKABLE_ITEM);
                sv_module_access::item(fighter.lua_state_agent);
                if fighter.pop_lua_stack(1).get_bool() {
                    fighter.change_status(FIGHTER_STATUS_KIND_ITEM_LIGHT_PICKUP.into(), true.into());
                    return true.into();
                }
            }
        }
        if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0
        && only_jabs(fighter) {
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING) {
                fighter.clear_lua_stack();
                lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_SWING);
                sv_module_access::item(fighter.lua_state_agent);
                if fighter.pop_lua_stack(1).get_bool() {
                    fighter.change_status(FIGHTER_STATUS_KIND_ITEM_SWING.into(), true.into());
                    return true.into();
                }
            }
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SHOOT) {
                fighter.clear_lua_stack();
                lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_SHOOT);
                sv_module_access::item(fighter.lua_state_agent);
                if fighter.pop_lua_stack(1).get_bool() {
                    fighter.change_status(FIGHTER_STATUS_KIND_ITEM_SHOOT_WAIT.into(), true.into());
                    return true.into();
                }
            }
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_100) {
                let attack_100_type = WorkModule::get_param_int(fighter.module_accessor, hash40("attack_combo_type"), 0);
                if attack_100_type == *FIGHTER_COMBO_TYPE_UNIQ_DANCE {
                    fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_100.into(), true.into());
                    return true.into();
                }
            }
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK) {
                fighter.change_status(FIGHTER_STATUS_KIND_ATTACK.into(), true.into());
                return true.into();
            }
        }
    }
    false.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_transition_group_check_ground)]
unsafe fn sub_transition_group_check_ground(fighter: &mut L2CFighterCommon, to_squat_wait: L2CValue) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        let cat2 = fighter.global_table[CMD_CAT2].get_i32();
        if cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI != 0
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_U)
        && {
            fighter.clear_lua_stack();
            lua_args!(fighter, Hash40::new_raw(0x1daca540be));
            sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
            fighter.pop_lua_stack(1).get_bool()
        } {
            fighter.change_status(FIGHTER_STATUS_KIND_APPEAL.into(), false.into());
            return true.into();
        }
        if cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_LW != 0
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_LW)
        && {
            fighter.clear_lua_stack();
            lua_args!(fighter, Hash40::new_raw(0x1daca540be));
            sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
            fighter.pop_lua_stack(1).get_bool()
        } {
            fighter.change_status(FIGHTER_STATUS_KIND_APPEAL.into(), false.into());
            return true.into();
        }
        if cat2 & (*FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L | *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R) != 0
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_S)
        && {
            fighter.clear_lua_stack();
            lua_args!(fighter, Hash40::new_raw(0x1daca540be));
            sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
            fighter.pop_lua_stack(1).get_bool()
        } {
            fighter.change_status(FIGHTER_STATUS_KIND_APPEAL.into(), false.into());
            return true.into();
        }
        let cat1 = fighter.global_table[CMD_CAT1].get_i32();
        if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH != 0
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH) {
            fighter.change_status(FIGHTER_STATUS_KIND_TURN_DASH.into(), true.into());
            return true.into();
        }
        if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH != 0
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH) {
            fighter.change_status(FIGHTER_STATUS_KIND_DASH.into(), true.into());
            return true.into();
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT)
        && fighter.sub_check_command_squat().get_bool() {
            let status = if to_squat_wait.get_bool() {
                FIGHTER_STATUS_KIND_SQUAT_WAIT
            }
            else {
                FIGHTER_STATUS_KIND_SQUAT
            };
            fighter.change_status(status.into(), true.into());
            return true.into();
        }
        if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN != 0
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN) {
            fighter.change_status(FIGHTER_STATUS_KIND_TURN.into(), true.into());
            return true.into();
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_WALK)
        && fighter.sub_check_command_walk().get_bool() {
            fighter.change_status(FIGHTER_STATUS_KIND_WALK.into(), true.into());
            return true.into();
        }
    }
    false.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_transition_group_check_air_attack)]
unsafe fn sub_transition_group_check_air_attack(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[CHECK_AIR_ATTACK_UNIQ].get_bool() {
        let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[CHECK_AIR_ATTACK_UNIQ].get_ptr());
        if callable(fighter).get_bool() {
            return true.into();
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR
    && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0 {
        if fighter.sub_transition_group_check_air_jump_attack().get_bool() {
            return true.into();
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SHOOT_AIR)
        && fighter.sub_is_item_shoot_air().get_bool() {
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_SHOOT_AIR.into(), true.into());
            return true.into();
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR)
        && FGCModule::check_enabled_aerial(fighter) {
            fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_AIR.into(), true.into());
            return true.into();
        }
    }
    false.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_transition_group_check_air_escape)]
unsafe fn sub_transition_group_check_air_escape(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[CHECK_AIR_ESCAPE_UNIQ].get_bool() {
        let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[CHECK_AIR_ESCAPE_UNIQ].get_ptr());
        if callable(fighter).get_bool() {
            return true.into();
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR
    && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR)
    && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE != 0
    && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR) {
        let stick_x = fighter.global_table[STICK_X].get_f32();
        let stick_y = fighter.global_table[STICK_Y].get_f32();
        let length = sv_math::vec2_length(stick_x, stick_y);
        let escape_air_slide_stick = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("escape_air_slide_stick"));
        let cancel = !CancelModule::is_enable_cancel(fighter.module_accessor) && VarModule::is_flag(fighter.battle_object, fighter::status::flag::FORCE_ESCAPE_AIR_SLIDE_IN_STATUS);
        let status = if escape_air_slide_stick <= length
        || cancel {
            FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE
        }
        else {
            FIGHTER_STATUS_KIND_ESCAPE_AIR
        };
        fighter.change_status(status.into(), true.into());
        return true.into();
    }
    false.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_transition_group_check_air_tread_jump)]
unsafe fn sub_transition_group_check_air_tread_jump(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[CHECK_AIR_TREAD_JUMP_UNIQ].get_bool() {
        let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[CHECK_AIR_TREAD_JUMP_UNIQ].get_ptr());
        if callable(fighter).get_bool() {
            return true.into();
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP != 0
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TREAD_JUMP)
        && ControlModule::is_enable_flick_jump(fighter.module_accessor) {
            let do_footstool;
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_NO_TREAD_FRAME) != 0 {
                do_footstool = false;
            }
            else {
                let tread_speed_y = fighter.FL_sub_fighter_float_next_tread_speed_y().get_f32();
                let tread_jump_speed_limit = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("tread_jump_speed_limit"));
                if tread_speed_y < tread_jump_speed_limit {
                    do_footstool = false;
                }
                else {
                    fighter.clear_lua_stack();
                    lua_args!(fighter, Hash40::new_raw(0x21bfbd3f83));
                    sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
                    do_footstool = fighter.pop_lua_stack(1).get_bool();
                }
            }
            if do_footstool {
                fighter.change_status(FIGHTER_STATUS_KIND_TREAD_JUMP.into(), true.into());
                return true.into();
            }
        }
        if fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_JUMP_TRIGGER != 0
        || fighter.global_table[CMD_CAT2].get_i32() & (
            *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI |
            *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L |
            *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R |
            *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_LW
        ) != 0 /* this is the addition */
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TREAD_JUMP_BUTTON) {
            let do_footstool;
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_NO_TREAD_FRAME) != 0 {
                do_footstool = false;
            }
            else {
                let tread_speed_y = fighter.FL_sub_fighter_float_next_tread_speed_y().get_f32();
                let tread_jump_speed_limit = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("tread_jump_speed_limit"));
                if tread_speed_y < tread_jump_speed_limit {
                    do_footstool = false;
                }
                else {
                    fighter.clear_lua_stack();
                    lua_args!(fighter, Hash40::new_raw(0x21bfbd3f83));
                    sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
                    do_footstool = fighter.pop_lua_stack(1).get_bool();
                }
            }
            if do_footstool {
                fighter.change_status(FIGHTER_STATUS_KIND_TREAD_JUMP.into(), true.into());
                return true.into();
            }
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TREAD_JUMP_NO_TRIGGER) {
            fighter.clear_lua_stack();
            lua_args!(fighter, Hash40::new_raw(0x21bfbd3f83), true);
            sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
            if fighter.pop_lua_stack(1).get_bool() {
                fighter.change_status(FIGHTER_STATUS_KIND_TREAD_JUMP.into(), false.into());
                return true.into();
            }
        }
    }
    false.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_transition_group_check_air_cliff)]
unsafe fn sub_transition_group_check_air_cliff(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut early_end = false;
    if fighter.global_table[CHECK_AIR_CLIFF_LASSO_UNIQ].get_bool() {
        let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[CHECK_AIR_CLIFF_LASSO_UNIQ].get_ptr());
        early_end = callable(fighter).get_bool()
    }
    if !early_end {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            // println!("air!");
            if GroundModule::can_entry_cliff(fighter.module_accessor) != 0 {
                // println!("can entry cliff");
                let stick_y = fighter.global_table[STICK_Y].get_f32();
                let cliff_catch_cancel_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("cliff_catch_cancel_stick_y"));
                if cliff_catch_cancel_stick_y < stick_y {
                    // println!("not canceling!");
                    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CLIFF_CATCH) {
                        // println!("transition term");
                        if GroundModule::is_status_cliff(fighter.module_accessor)
                        || VarModule::is_flag(fighter.battle_object, fighter::status::flag::SKIP_IS_STATUS_CLIFF_CHECK) {
                            // println!("is status cliff");
                            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_NO_CATCH_FRAME) == 0 {
                                // println!("cliff frame passed");
                                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_CLIFF_CATCH_MOVE);
                                fighter.change_status(FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE.into(), true.into());
                                return true.into();
                            }
                        }
                    }
                }
            }
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_CLIFF_CATCH_MOVE) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_CLIFF_CATCH_MOVE);
            StatusModule::delete_status_request_from_script(fighter.module_accessor);
        }
    }
    false.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_transition_group_check_ground_jump_mini_attack,
            sub_transition_group_check_ground_guard,
            sub_transition_group_check_ground_catch,
            sub_transition_group_check_ground_item,
            sub_transition_group_check_ground_attack,
            sub_transition_group_check_ground,
            sub_transition_group_check_air_attack,
            sub_transition_group_check_air_escape,
            sub_transition_group_check_air_tread_jump,
            sub_transition_group_check_air_cliff
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}