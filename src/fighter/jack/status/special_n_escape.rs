use crate::imports::status_imports::*;
use super::helper::*;

#[status_script(agent = "jack", status = FIGHTER_JACK_STATUS_KIND_SPECIAL_N_ESCAPE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe extern "C" fn jack_special_n_escape_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::reset_flick_x(fighter.module_accessor);
    ControlModule::reset_flick_sub_x(fighter.module_accessor);
    fighter.global_table[FLICK_X].assign(&L2CValue::I32(0xFE));
    let situation = fighter.global_table[SITUATION_KIND].get_i32();
    let is_normal = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_CUSTOMIZE_SPECIAL_N_NO) == 0;
    let escape_f = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_ESCAPE_F);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_ESCAPE_F);
    let mot;
    let xlu_frame_param;
    let normal_frame_param;
    if escape_f {
        if situation == *SITUATION_KIND_GROUND {
            xlu_frame_param = hash40("escape_f_hit_xlu_frame");
            normal_frame_param = hash40("escape_f_hit_normal_frame");
            if is_normal {
                mot = hash40("special_n_escape_f");
            }
            else {
                mot = hash40("special_n2_escape_f");
            }
        }
        else {
            xlu_frame_param = hash40("escape_air_hit_xlu_frame");
            normal_frame_param = hash40("escape_air_hit_normal_frame");
            if is_normal {
                mot = hash40("special_air_n_escape_f");
            }
            else {
                mot = hash40("special_air_n2_escape_f");
            }
        }
    }
    else {
        if situation == *SITUATION_KIND_GROUND {
            xlu_frame_param = hash40("escape_b_hit_xlu_frame");
            normal_frame_param = hash40("escape_b_hit_normal_frame");
            if is_normal {
                mot = hash40("special_n_escape_b");
            }
            else {
                mot = hash40("special_n2_escape_b");
            }
        }
        else {
            xlu_frame_param = hash40("escape_air_hit_xlu_frame");
            normal_frame_param = hash40("escape_air_hit_normal_frame");
            if is_normal {
                mot = hash40("special_air_n_escape_b");
            }
            else {
                mot = hash40("special_air_n2_escape_b");
            }
        }
    }
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new_raw(mot),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    let xlu_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), xlu_frame_param);
    let normal_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), normal_frame_param);
    if situation != *SITUATION_KIND_GROUND {
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            0.0,
            0.0
        );
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_MOTION,
            ENERGY_MOTION_RESET_TYPE_AIR_TRANS,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0
        );
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    }
    // Usually has dodge staling stuff in here but it's gone now
    let escape_invincible_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("escape_invincible_frame"), 0);
    WorkModule::set_int(fighter.module_accessor, (xlu_frame as f32 * escape_invincible_frame) as i32, *FIGHTER_JACK_STATUS_SPECIAL_N_INT_HIT_XLU_FRAME);
    WorkModule::set_int(fighter.module_accessor, normal_frame, *FIGHTER_JACK_STATUS_SPECIAL_N_INT_HIT_NORMAL_FRAME);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_JACK_STATUS_SPECIAL_N_INT_BUTTON_RAPID_COUNT);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BUTTON_RAPID);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_TRIGGER);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_BUTTON_ON);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_JUMP);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY);
    WorkModule::inc_int(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_INT_ESCAPE);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
    WorkModule::set_int(fighter.module_accessor, -1, *FIGHTER_JACK_STATUS_SPECIAL_N_INT_STICK_DIR);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_JACK_STATUS_SPECIAL_N_INT_STICK_FRAME);
    MotionModule::set_trans_move_speed_no_scale(fighter.module_accessor, false);
    if !StopModule::is_stop(fighter.module_accessor) {
        jack_special_n_escape_substatus(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(jack_special_n_escape_substatus as *const () as _));
    let additions = if is_normal {
        *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_07 - 1
    }
    else {
        *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_08 - 1
    };
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, additions);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x3a40337e2c), additions);
    fighter.sub_shift_status_main(L2CValue::Ptr(jack_special_n_escape_main_loop as *const () as _))
}

unsafe extern "C" fn jack_special_n_escape_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_FLAG_HIT_XLU) {
            if 0 < WorkModule::get_int(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_INT_HIT_NORMAL_FRAME) {
                if WorkModule::count_down_int(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_INT_HIT_NORMAL_FRAME, 0) {
                    HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
                    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_FLAG_HIT_XLU);
                }
            }
        }
        else {
            if 0 < WorkModule::get_int(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_INT_HIT_XLU_FRAME) {
                if WorkModule::count_down_int(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_INT_HIT_XLU_FRAME, 0) {
                    HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_FLAG_HIT_XLU);
                }
            }
        }
        jack_special_n_substatus_stick_check(fighter);
    }
    0.into()
}

unsafe extern "C" fn jack_special_n_escape_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    let prev_situation = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let situation = fighter.global_table[SITUATION_KIND].get_i32();
    if prev_situation != *SITUATION_KIND_GROUND
    && situation == *SITUATION_KIND_GROUND {
        let landing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("barrage_landing_frame"));
        jack_special_n_landing_handler(fighter, landing_frame.into());
        return 0.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if prev_situation == *SITUATION_KIND_GROUND
        && situation == *SITUATION_KIND_AIR {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
            || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL_RAW) {
                fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_N.into(), true.into());
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
            return 0.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        let status = if situation == *SITUATION_KIND_GROUND {
            FIGHTER_STATUS_KIND_WAIT
        }
        else {
            FIGHTER_STATUS_KIND_FALL
        };
        fighter.change_status(status.into(), false.into());
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if jack_special_n_check_next(fighter, L2CValue::Ptr(jack_special_n_escape_next_status as *const () as _), 1.into()).get_bool() {
            return 0.into();
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY) {
        let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST,
            sum_speed_x,
            0.0,
            0.0,
            0.0,
            0.0
        );
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY);
    }
    0.into()
}

unsafe extern "C" fn jack_special_n_escape_next_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_N.into(), true.into());
    0.into()
}

pub fn install() {
    install_status_scripts!(
        jack_special_n_escape_main
    );
}