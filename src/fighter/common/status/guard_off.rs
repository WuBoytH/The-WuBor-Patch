use crate::imports::status_imports::*;
use super::super::param;

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
    VarModule::set_int(fighter.battle_object, guard_off::int::ATTACK_CANCEL_FRAME, 0);
    let anim_cancel_frame = FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor, Hash40::new("guard_off"), true);
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
        if VarModule::get_int(fighter.battle_object, guard_off::int::ATTACK_CANCEL_FRAME) < param::shield::guard_off_attack_cancel_frame {
            VarModule::inc_int(fighter.battle_object, guard_off::int::ATTACK_CANCEL_FRAME);
            if VarModule::get_int(fighter.battle_object, guard_off::int::ATTACK_CANCEL_FRAME) == param::shield::guard_off_attack_cancel_frame {
                VarModule::on_flag(fighter.battle_object, fighter::instance::flag::GUARD_OFF_ATTACK_CANCEL);
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
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return true.into();
        }
    }
    else {
        if fighter.sub_transition_group_check_ground_jump_mini_attack().get_bool() {
            return true.into();
        }
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
                    if !cont
                    && fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0
                    && fighter.global_table[CMD_CAT3].get_i32() & (
                        *FIGHTER_PAD_CMD_CAT3_ITEM_LIGHT_THROW_HI |
                        *FIGHTER_PAD_CMD_CAT3_ITEM_LIGHT_THROW_HI4
                    ) != 0 {
                        fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), false.into());
                        return true.into();
                    }
                    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
                    && fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER == 0
                    && fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0
                    && fighter.global_table[CMD_CAT3].get_i32() & (
                        *FIGHTER_PAD_CMD_CAT3_ITEM_LIGHT_THROW_HI |
                        *FIGHTER_PAD_CMD_CAT3_ITEM_LIGHT_THROW_HI4
                    ) != 0 {
                        fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), false.into());
                        return true.into();
                    }
                }
            }
        }
        if VarModule::is_flag(fighter.battle_object, fighter::instance::flag::GUARD_OFF_ATTACK_CANCEL) {
            if fighter.sub_transition_group_check_ground_item().get_bool()
            || fighter.sub_transition_group_check_ground_catch().get_bool()
            || fighter.sub_transition_group_check_ground_special().get_bool()
            || fighter.sub_transition_group_check_ground_attack().get_bool() {
                return true.into();
            }
        }
    }
    false.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_status_guard_off_main_common_control)]
unsafe fn sub_status_guard_off_main_common_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.sub_transition_group_check_ground_jump().get_bool() {
        return false.into();
    }
    true.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_end_GuardOff)]
unsafe fn status_end_guardoff(fighter: &mut L2CFighterCommon) -> L2CValue {
    ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("throw"), &Vector3f{x: 1.0, y: 1.0, z: 1.0});
    WorkModule::unable_transition_term_forbid(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON);
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_STATUS_KIND_JUMP_SQUAT {
        VarModule::off_flag(fighter.battle_object, fighter::instance::flag::GUARD_OFF_ATTACK_CANCEL);
    }
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_ftstatusuniqprocessguardoff_initstatus,
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