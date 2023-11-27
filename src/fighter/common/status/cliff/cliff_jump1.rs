use crate::imports::status_imports::*;
use wubor_utils::controls::*;

#[skyline::hook(replace = L2CFighterCommon_status_CliffJump1)]
unsafe extern "C" fn status_cliffjump1(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[FLICK_Y_DIR].get_i32() <= 0 {
        VarModule::on_flag(fighter.module_accessor, cliff::flag::CLIFF_JUMP_BUTTON);
        if ControlModule::is_jump_mini_button(fighter.module_accessor) {
            VarModule::on_flag(fighter.module_accessor, cliff::flag::CLIFF_JUMP_MINI);
        }
    }
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_STICK_JUMP_COMMAND_LIFE);

    ControlModule::clear_command(fighter.module_accessor, false);
    ControlModule::reset_trigger(fighter.module_accessor);
    ControlModule::reset_flick_y(fighter.module_accessor);
    ControlModule::reset_flick_sub_y(fighter.module_accessor);
    fighter.global_table[FLICK_Y].assign(&L2CValue::I32(0xfe));
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CLIFF_JUMP);
    fighter.set_cliff_xlu_frame(hash40("cliff_jump_quick1").into());
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("cliff_jump_quick1"),
        0.0,
        13.0 / 18.0,
        false,
        0.0,
        false,
        false
    );
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SUB_FIGHTER) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_CLIFF_JUMP_NUM);
    }
    FighterUtil::set_pickelblock_mode_ignoreandattack(fighter.module_accessor);
    GroundModule::set_ignore_boss(fighter.module_accessor, true);
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_CliffJump1_Main as *const () as _))
}

pub unsafe extern "C" fn sub_cliff_jump1_uniq_process_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !VarModule::is_flag(fighter.module_accessor, cliff::flag::CLIFF_JUMP_BUTTON) {
        let stick_y = if Buttons::from_bits_unchecked(ControlModule::get_button(fighter.module_accessor)).intersects(Buttons::CStickOverride) {
            ControlModule::get_sub_stick_y(fighter.module_accessor)
        }
        else {
            ControlModule::get_stick_y(fighter.module_accessor)
        };
        let jump_neutral_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("jump_neutral_y"));
        if stick_y < jump_neutral_y {
            VarModule::on_flag(fighter.module_accessor, cliff::flag::CLIFF_JUMP_MINI);
        }
    }
    else {
        if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP)
        || ControlModule::is_jump_mini_button(fighter.module_accessor) {
            VarModule::on_flag(fighter.module_accessor, cliff::flag::CLIFF_JUMP_MINI);
        }
    }
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_cliffjump1
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}