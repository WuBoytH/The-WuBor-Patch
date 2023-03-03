use crate::imports::status_imports::*;

#[skyline::hook(replace = L2CFighterCommon_status_pre_AttackS3_common)]
unsafe fn status_pre_attacks3_common(fighter: &mut L2CFighterCommon, param_1: L2CValue) {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_MOTION_RUN_STOP,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP as u64,
        param_1.get_u32(),
        *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_3 as u32,
        0
    );
}

#[skyline::hook(replace = L2CFighterCommon_status_AttackS3_Main_param)]
unsafe fn status_attacks3_main_param(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        return 0.into();
    }
    if CustomCancelManager::execute_cancel(fighter) {
        return 1.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        let combo = ComboModule::count(fighter.module_accessor) as i32;
        let s3_combo_max = WorkModule::get_param_int(fighter.module_accessor, hash40("s3_combo_max"), 0);
        if combo < s3_combo_max
        && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO_PRECEDE)
        && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) {
            fighter.attack_s3_mtrans_param(param_1);
        }
    }
    else {
        fighter.attack_s3_mtrans_param(param_1);
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }
    let jump_attack_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
    if 0 < jump_attack_frame {
        if !StopModule::is_stop(fighter.module_accessor)
        && fighter.sub_check_button_jump().get_bool() {
            let log = fighter.status_attack();
            let info = log[0x10f40d7b92u64].get_i64();
            let mot = MotionModule::motion_kind(fighter.module_accessor);
            MotionAnimcmdModule::call_script_single(
                fighter.module_accessor,
                *FIGHTER_ANIMCMD_EXPRESSION,
                Hash40::new_raw(mot),
                -1
            );
            WorkModule::set_int64(fighter.module_accessor, info, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
            fighter.change_status_jump_mini_attack(true.into());
            return 1.into();
        }
    }
    if 1 == jump_attack_frame {
        if fighter.global_table[IS_STOP].get_bool()
        && WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND) > 0 {
            let log = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
            FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, log);
            WorkModule::set_int64(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    }
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_end_AttackS3)]
unsafe fn status_end_attacks3(fighter: &mut L2CFighterCommon) -> L2CValue {
    FGCModule::reset_used_ground_normals(fighter, false);
    let attack_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    if 0 < attack_kind {
        FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, attack_kind);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    }
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_AttackHi3_Main)]
unsafe fn status_attackhi3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        return 0.into();
    }
    if CustomCancelManager::execute_cancel(fighter) {
        return 1.into();
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO_PRECEDE) {
        if !StatusModule::is_changing(fighter.module_accessor) {
            let combo = ComboModule::count(fighter.module_accessor) as i32;
            let s3_combo_max = WorkModule::get_param_int(fighter.module_accessor, hash40("s3_combo_max"), 0);
            if combo < s3_combo_max
            || (WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO_PRECEDE)
            && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)) {
                fighter.attack_s3_mtrans_param(FIGHTER_COMBO_KIND_S3.into());
            }
        }
        else {
            fighter.attack_s3_mtrans_param(FIGHTER_COMBO_KIND_S3.into());
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }
    if fighter.status_AttackHi3_Main_minjump().get_bool() {
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    }
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_end_AttackHi3)]
unsafe fn status_end_attackhi3(fighter: &mut L2CFighterCommon) -> L2CValue {
    FGCModule::reset_used_ground_normals(fighter, false);
    let attack_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    if 0 < attack_kind {
        FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, attack_kind);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    }
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_AttackLw3_Main)]
unsafe fn status_attacklw3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        return 0.into();
    }
    if CustomCancelManager::execute_cancel(fighter) {
        return 1.into();
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO_PRECEDE) {
        if !StatusModule::is_changing(fighter.module_accessor) {
            let combo = ComboModule::count(fighter.module_accessor) as i32;
            let s3_combo_max = WorkModule::get_param_int(fighter.module_accessor, hash40("s3_combo_max"), 0);
            if combo < s3_combo_max
            || (WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO_PRECEDE)
            && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)) {
                fighter.attack_s3_mtrans_param(FIGHTER_COMBO_KIND_S3.into());
            }
        }
        else {
            fighter.attack_s3_mtrans_param(FIGHTER_COMBO_KIND_S3.into());
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }
    let jump_attack_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
    if 0 < jump_attack_frame {
        if !StopModule::is_stop(fighter.module_accessor)
        && fighter.sub_check_button_jump().get_bool() {
            let log = fighter.status_attack();
            let info = log[0x10f40d7b92u64].get_i64();
            let mot = MotionModule::motion_kind(fighter.module_accessor);
            MotionAnimcmdModule::call_script_single(
                fighter.module_accessor,
                *FIGHTER_ANIMCMD_EXPRESSION,
                Hash40::new_raw(mot),
                -1
            );
            WorkModule::set_int64(fighter.module_accessor, info, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
            fighter.change_status_jump_mini_attack(true.into());
            return 1.into();
        }
    }
    if 1 == jump_attack_frame {
        if fighter.global_table[IS_STOP].get_bool()
        && WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND) > 0 {
            let log = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
            FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, log);
            WorkModule::set_int64(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_SQUAT_WAIT.into(), false.into());
    }
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_end_AttackLw3)]
unsafe fn status_end_attacklw3(fighter: &mut L2CFighterCommon) -> L2CValue {
    FGCModule::reset_used_ground_normals(fighter, false);
    let attack_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    if 0 < attack_kind {
        FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, attack_kind);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    }
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pre_attacks3_common, status_attacks3_main_param, status_end_attacks3,
            status_attackhi3_main, status_end_attackhi3,
            status_attacklw3_main, status_end_attacklw3
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}