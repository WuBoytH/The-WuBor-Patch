use crate::imports::status_imports::*;

#[skyline::hook(replace = L2CFighterCommon_status_pre_GuardOff)]
unsafe fn status_pre_guardoff(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_MOTION,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_GUARD_OFF_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_GUARD_OFF_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_GUARD_OFF_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        0,
        *FIGHTER_STATUS_ATTR_DISABLE_SHIELD_RECOVERY as u32,
        0,
        0
    );
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
    // let anim_cancel_frame = FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor, Hash40::new("guard_off"), true);
    let motion_rate = 1.0;
    // if 0.0 < guard_off_cancel_frame as f32
    // && 0.0 < anim_cancel_frame {
    //     motion_rate = anim_cancel_frame / guard_off_cancel_frame as f32;
    // }
    if VarModule::is_flag(fighter.battle_object, guard::flag::ADD_BUFFER) {
        ControlModule::set_command_life_extend(fighter.module_accessor, guard_off_cancel_frame as u8);
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
    false.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_status_guard_off_main_common_control)]
unsafe fn sub_status_guard_off_main_common_control(_fighter: &mut L2CFighterCommon) -> L2CValue {
    // if fighter.sub_transition_group_check_ground_jump().get_bool() {
    //     return true.into();
    // }
    false.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_end_GuardOff)]
unsafe fn status_end_guardoff(fighter: &mut L2CFighterCommon) -> L2CValue {
    ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("throw"), &Vector3f{x: 1.0, y: 1.0, z: 1.0});
    // WorkModule::unable_transition_term_forbid(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON);
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pre_guardoff,
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