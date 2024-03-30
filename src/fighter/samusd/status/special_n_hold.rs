use crate::imports::*;

unsafe extern "C" fn samusd_special_n_hold_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_air_n_h"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_n_h"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
    }
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_FLY_BUTTON);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_FLY);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_FLY_NEXT);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR);
    ControlModule::set_add_jump_mini_button_life(fighter.module_accessor, 8);
    fighter.sub_shift_status_main(L2CValue::Ptr(samusd_special_n_hold_main_loop as *const () as _))
}

unsafe extern "C" fn samusd_special_n_hold_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let prev_sit = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let curr_sit = fighter.global_table[SITUATION_KIND].get_i32();
    if prev_sit != curr_sit {
        let mot;
        if curr_sit == *SITUATION_KIND_GROUND {
            mot = Hash40::new("special_n_h");
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        }
        else {
            mot = Hash40::new("special_air_n_h");
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        }
        MotionModule::change_motion_inherit_frame(
            fighter.module_accessor,
            mot,
            -1.0,
            1.0,
            0.0,
            false,
            false
        );
    }
    let fire_trigger = fighter.global_table[PAD_FLAG].get_i32() & (
        *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER | *FIGHTER_PAD_FLAG_ATTACK_TRIGGER) != 0;
    if !fire_trigger {
        let mut cont = false;
        let mut int_to_set = 0;
        if curr_sit == *SITUATION_KIND_GROUND {
            if fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_STICK_ESCAPE != 0
            && !cont {
                if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE) {
                    int_to_set = *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_GROUND_ESCAPE;
                }
                else {
                    int_to_set = *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_NONE;
                }
                cont = true;
            }
            else if fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_STICK_ESCAPE_F != 0
            && !cont {
                if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F) {
                    int_to_set = *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_GROUND_ESCAPE_F;
                }
                else {
                    int_to_set = *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_NONE;
                }
                cont = true;
            }
            else if fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_STICK_ESCAPE_B != 0
            && !cont {
                if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B) {
                    int_to_set = *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_GROUND_ESCAPE_B;
                }
                else {
                    int_to_set = *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_NONE;
                }
                cont = true;
            }
            else if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON != 0
            && !cont {
                if fighter.sub_check_button_jump().get_bool() {
                    int_to_set = *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_GROUND_JUMP;
                }
                else {
                    int_to_set = *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_NONE;
                }
                cont = true;
            }
            else if ControlModule::is_enable_flick_jump(fighter.module_accessor)
            && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP != 0
            && !cont {
                if fighter.sub_check_button_frick().get_bool() {
                    int_to_set = *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_GROUND_JUMP;
                }
                else {
                    int_to_set = *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_NONE;
                }
                cont = true;
            }
            else if fighter.sub_check_command_guard().get_bool()
            && !cont {
                if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON) {
                    int_to_set = *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_GROUND_GUARD;
                }
                else {
                    int_to_set = *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_NONE;
                }
                cont = true;
            }
            if cont {
                WorkModule::set_int(fighter.module_accessor, int_to_set, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
                fighter.change_status(FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_C.into(), true.into());
                return 1.into();
            }
        }
        let mut next_status = 0;
        if fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_GUARD_TRIGGER != 0
        && next_status == 0 {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR) {
                if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR) {
                    int_to_set = *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_AIR_ESCAPE_AIR;
                }
                else {
                    int_to_set = *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_NONE;
                }
                next_status = *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_C;
            }
        }
        else if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP != 0
        && next_status == 0 {
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT)
            < WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
                if ControlModule::is_enable_flick_jump(fighter.module_accessor) {
                    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL) {
                        int_to_set = *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_AIR_JUMP_AERIAL;
                    }
                    else {
                        int_to_set = *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_NONE;
                    }
                    next_status = *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_JUMP_CANCEL;
                }
            }
        }
        else if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON != 0
        && next_status == 0 {
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT)
            < WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
                if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON) {
                    int_to_set = *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_AIR_JUMP_AERIAL;
                }
                else {
                    int_to_set = *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_NONE;
                }
                next_status = *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_JUMP_CANCEL;
            }
        }
        if next_status != 0 {
            WorkModule::set_int(fighter.module_accessor, int_to_set, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
            fighter.change_status(next_status.into(), true.into());
            return 1.into();
        }
        if fighter.global_table[IS_STOP].get_bool() {
            let charge = WorkModule::get_int(fighter.module_accessor, *FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT);
            let charge_max = samusd_get_max_charge_frame(fighter).get_f32();
            let ratio = charge as f32 / charge_max;
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x26b38955ef), ratio);
            return 0.into();
        }
        else {
            WorkModule::inc_int(fighter.module_accessor, *FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT);
            let charge = WorkModule::get_int(fighter.module_accessor, *FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT);
            let charge_max = samusd_get_max_charge_frame(fighter).get_f32();
            if charge >= charge_max as i32 {
                WorkModule::set_int(fighter.module_accessor, charge_max as i32, *FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT);
                let mot = if curr_sit != *SITUATION_KIND_GROUND {
                    Hash40::new("special_air_n_f_max")
                }
                else {
                    Hash40::new("special_n_f_max")
                };
                MotionModule::change_motion(
                    fighter.module_accessor,
                    mot,
                    0.0,
                    1.0,
                    false,
                    0.0,
                    false,
                    false
                );
                fighter.change_status(FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F.into(), false.into());
                return 1.into();
            }
        }
        return 0.into();
    }
    else {
        let mot = if curr_sit != *SITUATION_KIND_GROUND {
            Hash40::new("special_air_n_f")
        }
        else {
            Hash40::new("special_n_f")
        };
        MotionModule::change_motion(
            fighter.module_accessor,
            mot,
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        fighter.change_status(FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F.into(), false.into());
    }
    1.into()
}

unsafe extern "C" fn samusd_special_n_hold_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if status_kind == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F
    || status_kind == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_E {
        return 0.into();
    }
    if ![
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_C,
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_JUMP_CANCEL,
        *FIGHTER_STATUS_KIND_GUARD_ON,
        *FIGHTER_STATUS_KIND_ESCAPE,
        *FIGHTER_STATUS_KIND_ESCAPE_F,
        *FIGHTER_STATUS_KIND_ESCAPE_B
    ].contains(&status_kind) {
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT);
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SAMUSD_GENERATE_ARTICLE_CSHOT, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        EffectModule::remove_common(fighter.module_accessor, Hash40::new("charge_max"));
        let charge_max_efh = WorkModule::get_int(fighter.module_accessor, *FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_EFH_CHARGE_MAX);
        effect!(fighter, MA_MSC_EFFECT_REMOVE, charge_max_efh);
    }
    else {
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT);
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SAMUSD_GENERATE_ARTICLE_CSHOT, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    0.into()
}

unsafe extern "C" fn samusd_get_max_charge_frame(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("cshot_charge_frame")).into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Main, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_H, samusd_special_n_hold_main);
    agent.status(smashline::Exit, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_H, samusd_special_n_hold_exit);
}