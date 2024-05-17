use super::*;

unsafe extern "C" fn pitb_special_n_charge_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_FLAG_DIR_S) {
        WorkModule::set_int64(fighter.module_accessor, hash40("special_n_hold_hi") as i64, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_INT_MOTION);
        WorkModule::set_int64(fighter.module_accessor, hash40("special_air_n_hold_hi") as i64, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_INT_MOTION_AIR);
        WorkModule::set_int64(fighter.module_accessor, 0x7cabbcbb5, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_INT_BOW_MOTION);
        WorkModule::set_int64(fighter.module_accessor, 0xbd2abc95c, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_INT_BOW_MOTION_AIR);
    }
    else {
        WorkModule::set_int64(fighter.module_accessor, hash40("special_n_hold_s") as i64, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_INT_MOTION);
        WorkModule::set_int64(fighter.module_accessor, hash40("special_air_n_hold_s") as i64, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_INT_MOTION_AIR);
        WorkModule::set_int64(fighter.module_accessor, 0x684068652, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_INT_BOW_MOTION);
        WorkModule::set_int64(fighter.module_accessor, 0xa23431885, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_INT_BOW_MOTION_AIR);
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(pitb_special_n_charge_charge as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(pitb_special_n_charge_loop as *const () as _))
}

unsafe extern "C" fn pitb_special_n_charge_charge(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::inc_int(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_INT_CHARGE);
    0.into()
}

unsafe extern "C" fn pitb_special_n_charge_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_PIT_SPECIAL_AIR_N);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_FLAG_FIRST) {
            let mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_INT_MOTION_AIR);
            MotionModule::change_motion_inherit_frame(
                fighter.module_accessor,
                Hash40::new_raw(mot),
                -1.0,
                1.0,
                0.0,
                false,
                false
            );
            let bowmot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_INT_BOW_MOTION_AIR);
            ArticleModule::change_motion(
                fighter.module_accessor,
                *FIGHTER_PIT_GENERATE_ARTICLE_BOW,
                Hash40::new_raw(bowmot),
                true,
                -1.0
            );
        }
        else {
            let mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_INT_MOTION_AIR);
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
            let bowmot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_INT_BOW_MOTION_AIR);
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PIT_GENERATE_ARTICLE_BOW, Hash40::new_raw(bowmot), false, -1.0);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_FLAG_FIRST);
        }
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_FLAG_FIRST) {
            let mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_INT_MOTION);
            MotionModule::change_motion_inherit_frame(
                fighter.module_accessor,
                Hash40::new_raw(mot),
                -1.0,
                1.0,
                0.0,
                false,
                false
            );
            let bowmot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_INT_BOW_MOTION);
            ArticleModule::change_motion(
                fighter.module_accessor,
                *FIGHTER_PIT_GENERATE_ARTICLE_BOW,
                Hash40::new_raw(bowmot),
                true,
                -1.0
            );
        }
        else {
            let mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_INT_MOTION);
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
            let bowmot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_INT_BOW_MOTION);
            ArticleModule::change_motion(
                fighter.module_accessor,
                *FIGHTER_PIT_GENERATE_ARTICLE_BOW,
                Hash40::new_raw(bowmot),
                false,
                -1.0
            );
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_FLAG_FIRST);
        }
    }
    let curr_charge = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_INT_CHARGE);
    let max_charge = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("charge_frame"));
    if !ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
    && curr_charge < max_charge {
        let sticky = fighter.global_table[STICK_Y].get_f32();
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_FLAG_DIR_S) {
            let upsticky = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("up_stick_y"));
            if sticky < upsticky {
                fighter.change_status(FIGHTER_PIT_STATUS_KIND_SPECIAL_N_DIR.into(), true.into());
            }
        }
        else {
            let upsticky = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("up_stick_y"));
            if sticky >= upsticky {
                fighter.change_status(FIGHTER_PIT_STATUS_KIND_SPECIAL_N_DIR.into(), true.into());
            }
        }
    }
    else {
        if curr_charge >= max_charge {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_FLAG_CHARGE_MAX);
            fighter.change_status(FIGHTER_PIT_STATUS_KIND_SPECIAL_N_SHOOT.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_PIT_STATUS_KIND_SPECIAL_N_SHOOT.into(), true.into());
        }
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_CHARGE, pitb_special_n_charge_main);
}