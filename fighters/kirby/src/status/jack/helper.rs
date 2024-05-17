use super::*;

pub unsafe extern "C" fn kirby_jack_special_n_substatus_stick_check(fighter: &mut L2CFighterCommon) {
    let stick_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_INT_STICK_FRAME);
    if 0 < stick_frame {
        if stick_frame - 1 == 0 {
            WorkModule::set_int(fighter.module_accessor, -1, *FIGHTER_JACK_STATUS_SPECIAL_N_INT_STICK_DIR);
        }
        WorkModule::set_int(fighter.module_accessor, stick_frame - 1, *FIGHTER_JACK_STATUS_SPECIAL_N_INT_STICK_FRAME);
    }
}

pub unsafe extern "C" fn kirby_jack_special_n_check_next(
    fighter: &mut L2CFighterCommon,
    change_status_func: L2CValue,
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
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_JUMP)
                && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N)
                && (
                    fighter.sub_check_button_jump().get_bool()
                    || fighter.sub_check_button_frick().get_bool()
                ) {
                    fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_JACK_SPECIAL_N_JUMP.into(), true.into());
                    return true.into();
                }
            }
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_TRIGGER) {
            if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
                if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N) {
                    let cat1 = fighter.global_table[CMD_CAT1].get_i32();
                    if cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI | *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 {
                        if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW != 0 {
                            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_BARRAGE_LW);
                        }
                        fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_JACK_SPECIAL_N_BARRAGE_START.into(), true.into());
                        return true.into();
                    }
                }
            }
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BUTTON_RAPID) {
            if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() != *FIGHTER_KIRBY_STATUS_KIND_JACK_SPECIAL_N_ESCAPE {
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
                        let rapid_count_curr = WorkModule::get_int(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_INT_BUTTON_RAPID_COUNT);
                        let rapid_count = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("rapid_count"));
                        if rapid_count <= rapid_count_curr {
                            if escape_fb_stick_x <= stick_x.abs() {
                                let lr = PostureModule::lr(fighter.module_accessor);
                                if 0.0 < stick_x * lr {
                                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_ESCAPE_F);
                                }
                                fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_JACK_SPECIAL_N_ESCAPE.into(), true.into());
                                return true.into();
                            }
                        }
                    }
                }
            }
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_BUTTON_ON) {
            if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() == *FIGHTER_KIRBY_STATUS_KIND_JACK_SPECIAL_N_ESCAPE {
                if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_JUMP)
                    && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N)
                    && (
                        fighter.sub_check_button_jump().get_bool()
                        || fighter.sub_check_button_frick().get_bool()
                    ) {
                        fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_JACK_SPECIAL_N_JUMP.into(), true.into());
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
                if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() != *FIGHTER_KIRBY_STATUS_KIND_JACK_SPECIAL_N_ESCAPE {
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
                else {
                    let escape_num_max = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("escape_num_max"));
                    let escape_num = WorkModule::get_int(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_INT_ESCAPE);
                    if escape_num < escape_num_max
                    && (stick_dir == 0
                    || stick_dir == 1) {
                        if stick_dir == 0 {
                            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_ESCAPE_F);
                        }
                        fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_JACK_SPECIAL_N_ESCAPE.into(), true.into());
                        return true.into();
                    }
                }
            }
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS) {
                if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() != *FIGHTER_KIRBY_STATUS_KIND_JACK_SPECIAL_N_ESCAPE {
                    let escape_num_max = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("escape_num_max"));
                    let escape_num = WorkModule::get_int(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_INT_ESCAPE);
                    if escape_num < escape_num_max
                    && (stick_dir == 0
                    || stick_dir == 1) {
                        if stick_dir == 0 {
                            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_ESCAPE_F);
                        }
                        fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_JACK_SPECIAL_N_ESCAPE.into(), true.into());
                        return true.into();
                    }
                }
                let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(change_status_func.get_ptr());
                callable(fighter);
                return true.into();
            }
        }
    }
    false.into()
}

pub unsafe extern "C" fn kirby_jack_special_n_landing_handler(fighter: &mut L2CFighterCommon, landing_frame: L2CValue) {
    if !CancelModule::is_enable_cancel(fighter.module_accessor) {
        WorkModule::set_float(fighter.module_accessor, landing_frame.get_f32(), *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_LANDING_CANCEL);
        fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_JACK_SPECIAL_N_LANDING.into(), false.into());
    }
    else {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
    }
}