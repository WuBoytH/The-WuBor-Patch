use crate::imports::status_imports::*;
use super::helper::*;

#[status_script(agent = "kirby", status = FIGHTER_KIRBY_STATUS_KIND_JACK_SPECIAL_N_JUMP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn kirby_jack_special_n_jump_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::reset_flick_y(fighter.module_accessor);
    ControlModule::reset_flick_sub_y(fighter.module_accessor);
    fighter.global_table[FLICK_Y].assign(&L2CValue::I32(0xFE));
    ControlModule::reset_trigger(fighter.module_accessor);
    FighterMotionModuleImpl::change_motion_kirby_copy(
        fighter.module_accessor,
        Hash40::new("special_n_jump"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    let x_stick_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("jump_barrage_accel_x_stick_mul"));
    sv_kinetic_energy!(
        controller_set_accel_x_mul,
        fighter,
        x_stick_mul
    );
    let speed_max_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("jump_barrage_speed_max_x"));
    sv_kinetic_energy!(
        set_stable_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        speed_max_x,
        0.0
    );
    if !StopModule::is_stop(fighter.module_accessor) {
        kirby_jack_special_n_jump_substatus(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(kirby_jack_special_n_jump_substatus as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(kirby_jack_special_n_jump_main_loop as *const () as _))
}

unsafe extern "C" fn kirby_jack_special_n_jump_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if !param_1.get_bool()
    && fighter.sub_is_dive().get_bool() {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_REQUEST_DIVE_EFFECT);
    }
    0.into()
}

unsafe extern "C" fn kirby_jack_special_n_jump_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PREV_SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        let landing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("jump_barrage_landing_frame"));
        kirby_jack_special_n_landing_handler(fighter, landing_frame.into());
        return 0.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
        || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL_RAW) {
            fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_JACK_SPECIAL_N.into(), true.into());
            return 0.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        kirby_jack_special_n_jump_main
    );
}