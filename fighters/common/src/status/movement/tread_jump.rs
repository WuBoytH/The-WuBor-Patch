use super::*;

#[skyline::hook(replace = L2CFighterCommon_status_TreadJump)]
unsafe extern "C" fn status_treadjump(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP)
    || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI)
    || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L)
    || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R)
    || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_TREAD_FLAG_BUTTON);
        ControlModule::reset_trigger(fighter.module_accessor);
    }
    else {
        ControlModule::reset_flick_y(fighter.module_accessor);
    }
    WorkModule::inc_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_TREAD_JUMP_COUNT);
    let tread_jump_disable_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("tread_jump_disable_frame"));
    WorkModule::set_int(fighter.module_accessor, tread_jump_disable_frame, *FIGHTER_INSTANCE_WORK_ID_INT_NO_TREAD_FRAME);
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FROM_TREAD, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_JUMP_FROM);
    fighter.sub_tread_jump_unique_process_init_inner();
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_tread_jump_uniq_check();
    }
    fighter.global_table[SUB_STATUS2].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_tread_jump_uniq_check as *const () as _));
    let mut tread_attack_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("tread_attack_frame"));
    if MotionModule::is_flag_start_1_frame(fighter.module_accessor) {
        tread_attack_frame -= 1;
    }
    WorkModule::set_float(fighter.module_accessor, tread_attack_frame as f32, *FIGHTER_STATUS_TREAD_WORK_FLOAT_ATTACK_FRAME);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ITEM_THROW);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_TreadJump_Main as *const () as _))
}

#[skyline::hook(replace = L2CFighterCommon_sub_tread_jump_uniq_check)]
unsafe extern "C" fn sub_tread_jump_uniq_check(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_TREAD_FLAG_NO_REACTION) {
        let jump_mini = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_TREAD_FLAG_BUTTON) {
            ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP)
            && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI)
            && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R)
            && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L)
            && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW)
        }
        else {
            let jump_neutral_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("jump_neutral_y"));
            fighter.global_table[STICK_Y].get_f32() < jump_neutral_y
        };
        if jump_mini {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
        }
    }
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_end_TreadJump)]
unsafe extern "C" fn status_end_treadjump(fighter: &mut L2CFighterCommon) -> L2CValue {
    let tread_jump_disable_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("tread_jump_disable_frame"));
    WorkModule::set_int(fighter.module_accessor, tread_jump_disable_frame, *FIGHTER_INSTANCE_WORK_ID_INT_NO_TREAD_FRAME);
    // if fighter.global_table[STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_JUMP {
    //     let tread_jump_after_xlu_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("tread_jump_after_xlu_frame"));
    //     HitModule::set_xlu_frame_global(fighter.module_accessor, tread_jump_after_xlu_frame, 0);
    // }
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_treadjump,
            sub_tread_jump_uniq_check,
            status_end_treadjump
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}