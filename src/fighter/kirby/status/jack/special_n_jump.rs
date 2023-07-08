use crate::imports::status_imports::*;
use super::helper::*;

#[status_script(agent = "kirby", status = FIGHTER_KIRBY_STATUS_KIND_JACK_SPECIAL_N_JUMP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn kirby_jack_special_n_jump_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::reset_flick_y(fighter.module_accessor);
    ControlModule::reset_flick_sub_y(fighter.module_accessor);
    fighter.global_table[FLICK_Y].assign(&L2CValue::I32(0xFE));
    ControlModule::reset_trigger(fighter.module_accessor);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_JACK_STATUS_SPECIAL_N_INT_BUTTON_RAPID_COUNT);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BUTTON_RAPID);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_TRIGGER);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_BUTTON_ON);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_JUMP);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY);
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
    if !StatusModule::is_changing(fighter.module_accessor) {
        if kirby_jack_special_n_jump_check_next(fighter, 1.into()).get_bool() {
            return 0.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    0.into()
}

pub unsafe fn kirby_jack_special_n_jump_check_next(
    fighter: &mut L2CFighterCommon,
    stick_frame: L2CValue
) -> L2CValue {
    let attack_hi4_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("attack_hi4_stick_y"));
    let attack_lw4_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("attack_lw4_stick_y"));
    let escape_fb_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("escape_fb_stick_x"));
    let stick_x = fighter.global_table[STICK_X].get_f32();
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    if escape_fb_stick_x <= stick_x.abs() {
        let lr = PostureModule::lr(fighter.module_accessor);
        if 0.0 < stick_x * lr {
            WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_JACK_STATUS_SPECIAL_N_INT_STICK_DIR);
            WorkModule::set_int(fighter.module_accessor, stick_frame.get_i32(), *FIGHTER_JACK_STATUS_SPECIAL_N_INT_STICK_FRAME);
        }
        else {
            WorkModule::set_int(fighter.module_accessor, 1, *FIGHTER_JACK_STATUS_SPECIAL_N_INT_STICK_DIR);
            WorkModule::set_int(fighter.module_accessor, stick_frame.get_i32(), *FIGHTER_JACK_STATUS_SPECIAL_N_INT_STICK_FRAME);
        }
    }
    else {
        if attack_hi4_stick_y <= stick_y {
            WorkModule::set_int(fighter.module_accessor, 2, *FIGHTER_JACK_STATUS_SPECIAL_N_INT_STICK_DIR);
            WorkModule::set_int(fighter.module_accessor, stick_frame.get_i32(), *FIGHTER_JACK_STATUS_SPECIAL_N_INT_STICK_FRAME);
        }
        if stick_y <= attack_lw4_stick_y {
            WorkModule::set_int(fighter.module_accessor, 3, *FIGHTER_JACK_STATUS_SPECIAL_N_INT_STICK_DIR);
            WorkModule::set_int(fighter.module_accessor, stick_frame.get_i32(), *FIGHTER_JACK_STATUS_SPECIAL_N_INT_STICK_FRAME);
        }
    }
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BUTTON_RAPID);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_TRIGGER);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_BUTTON_ON);
    }
    else {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BUTTON_RAPID) {
            let escape_num_max = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("escape_num_max"));
            let escape_num = WorkModule::get_int(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_INT_ESCAPE);
            if escape_num < escape_num_max {
                if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N) {
                    let cat1 = fighter.global_table[CMD_CAT1].get_i32();
                    if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S != 0 {
                        let lr = PostureModule::lr(fighter.module_accessor);
                        let special_s_turn = ControlModule::special_s_turn(fighter.module_accessor) as i32;
                        if (lr == 1.0 && special_s_turn == *FIGHTER_COMMAND_TURN_LR_RIGHT)
                        || (lr != 1.0 && special_s_turn == *FIGHTER_COMMAND_TURN_LR_LEFT) {
                            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_ESCAPE_F);
                        }
                        fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_JACK_SPECIAL_N_ESCAPE.into(), true.into());
                        return true.into();
                    }
                }
            }
        }
        if (ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
        || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL_RAW))
        && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N) {
            let stick_dir = WorkModule::get_int(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_INT_STICK_DIR);
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_BUTTON_ON) {
                if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
                    if stick_dir == 2
                    || stick_dir == 3 {
                        if stick_dir == 3 {
                            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_BARRAGE_LW);
                        }
                        fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_JACK_SPECIAL_N_BARRAGE_START.into(), true.into());
                        return true.into();
                    }
                }
            }
        }
    }
    false.into()
}

pub fn install() {
    install_status_scripts!(
        kirby_jack_special_n_jump_main
    );
}