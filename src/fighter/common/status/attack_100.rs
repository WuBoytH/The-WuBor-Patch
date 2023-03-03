use crate::imports::status_imports::*;

#[skyline::hook(replace = L2CFighterCommon_status_Attack100_Main_uniq_func)]
unsafe fn status_attack100_main_uniq_func(fighter: &mut L2CFighterCommon, param_1: L2CValue) {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return;
        }
    }
    if CustomCancelManager::execute_cancel(fighter) {
        return;
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return;
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
            return;
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
    let step = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_WORK_INT_100_STEP);
    if step == *FIGHTER_STATUS_ATTACK_100_STEP_START {
        if !StopModule::is_stop(fighter.module_accessor) {
            fighter.attack_100_start_uniq_chk(false.into());
        }
        fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_attack_100_start_uniq_chk as *const () as _));
        let motion = MotionModule::motion_kind(fighter.module_accessor);
        if motion == hash40("attack_100_start")
        && !MotionModule::is_end(fighter.module_accessor) {
            return;
        }
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_INPUT);
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("attack_100"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_100_STEP_LOOP, *FIGHTER_STATUS_ATTACK_WORK_INT_100_STEP);
    }
    else if step == *FIGHTER_STATUS_ATTACK_100_STEP_LOOP {
        let callable: extern "C" fn(&mut L2CFighterCommon, L2CValue) -> L2CValue = std::mem::transmute(param_1.get_ptr());
        if !StopModule::is_stop(fighter.module_accessor) {
            callable(fighter, false.into());
        }
        fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(callable as *const () as _));
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE) {
            return;
        }
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("attack_100_end"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_ATTACK_WORK_INT_100_HIT_NEAR_COUNT);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_100_STEP_END, *FIGHTER_STATUS_ATTACK_WORK_INT_100_STEP);
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_LOG_ATTACK_SUB_KIND_UNIQ, *FIGHTER_INSTANCE_WORK_ID_INT_TRICK_SUB);
    }
    else {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_attack100_main_uniq_func
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}