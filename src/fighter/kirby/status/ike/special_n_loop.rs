use crate::imports::*;

unsafe extern "C" fn kirby_ike_special_n_loop_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT);
    if !StopModule::is_stop(fighter.module_accessor) {
        kirby_ike_special_n_loop_substatus(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(kirby_ike_special_n_loop_substatus as *const () as _));
    kirby_ike_special_n_loop_mot_helper(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(kirby_ike_special_n_loop_main_loop as *const () as _))
}

unsafe extern "C" fn kirby_ike_special_n_loop_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() {
        let charge_speed_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("charge_speed_mul"));
        WorkModule::add_float(fighter.module_accessor, charge_speed_mul, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_FLOAT_CHARGE_COUNT);
    }
    0.into()
}

unsafe extern "C" fn kirby_ike_special_n_loop_mot_helper(fighter: &mut L2CFighterCommon) {
    let charge_speed_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("charge_speed_mul"));
    let mot;
    let kinetic;
    let correct;
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        mot = Hash40::new("special_air_n_loop");
        kinetic = *FIGHTER_KINETIC_TYPE_AIR_STOP;
        correct = *GROUND_CORRECT_KIND_AIR;
    }
    else {
        mot = Hash40::new("special_n_loop");
        kinetic = *FIGHTER_KINETIC_TYPE_GROUND_STOP;
        correct = *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP;
    }
    KineticModule::change_kinetic(fighter.module_accessor, kinetic);
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(correct));
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT) {
        FighterMotionModuleImpl::change_motion_kirby_copy(
            fighter.module_accessor,
            mot,
            0.0,
            charge_speed_mul,
            false,
            0.0,
            false,
            false
        );
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT);
    }
    else {
        FighterMotionModuleImpl::change_motion_inherit_frame_keep_rate_kirby_copy(
            fighter.module_accessor,
            mot,
            -1.0,
            1.0,
            0.0
        );
    }
}

unsafe extern "C" fn kirby_ike_special_n_loop_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let charge_count = WorkModule::get_float(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_N_WORK_FLOAT_CHARGE_COUNT);
    let special_n_charge_count_max = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("special_n_charge_count_max"));
    let special_n_charge_count_mdl = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("special_n_charge_count_mdl"));
    if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        // let max_count = (special_n_charge_count_max - 1) as f32 * 30.0;
        // let mdl_count = special_n_charge_count_mdl as f32 * 30.0;
        // let status = if charge_count < mdl_count {
        //     FIGHTER_KIRBY_STATUS_KIND_IKE_SPECIAL_N_END
        // }
        // else if charge_count < max_count {
        //     FIGHTER_KIRBY_STATUS_KIND_IKE_SPECIAL_N_END_MDL
        // }
        // else {
        //     FIGHTER_KIRBY_STATUS_KIND_IKE_SPECIAL_N_END_MAX
        // };
        // fighter.change_status(status.into(), false.into());
        // return 0.into();
        fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_IKE_SPECIAL_N_END.into(), false.into());
        return 0.into();
    }
    if (special_n_charge_count_max as f32 * 30.0) <= charge_count {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_N_FLAG_CHARGE_MAX);
        // fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_IKE_SPECIAL_N_END_MAX.into(), false.into());
        fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_IKE_SPECIAL_N_END.into(), false.into());
        return 0.into();
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_N_FLAG_RUMBLE_MIDDLE) {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_N_FLAG_RUMBLE_MAX) {
            if (special_n_charge_count_max as f32 * 30.0) <= charge_count {
                ControlModule::stop_rumble_kind(
                    fighter.module_accessor,
                    Hash40::new("rbkind_32_hold_lv2"),
                    *BATTLE_OBJECT_ID_INVALID as u32
                );
                ControlModule::set_rumble(
                    fighter.module_accessor,
                    Hash40::new("rbkind_32_hold_lv3"),
                    0,
                    true,
                    *BATTLE_OBJECT_ID_INVALID as u32
                );
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_N_FLAG_RUMBLE_MAX);
            }
        }
    }
    else {
        if (special_n_charge_count_mdl as f32 * 30.0) <= charge_count {
            ControlModule::set_rumble(
                fighter.module_accessor,
                Hash40::new("rbkind_32_hold_lv2"),
                0,
                true,
                *BATTLE_OBJECT_ID_INVALID as u32
            );
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_N_FLAG_RUMBLE_MIDDLE);
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        kirby_ike_special_n_loop_mot_helper(fighter);
    }
    0.into()
}
pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Main, *FIGHTER_KIRBY_STATUS_KIND_IKE_SPECIAL_N_LOOP, kirby_ike_special_n_loop_main);
}