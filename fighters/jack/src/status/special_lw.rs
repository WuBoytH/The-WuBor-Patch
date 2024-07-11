use super::*;

pub unsafe extern "C" fn jack_special_lw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let rebel_gauge = WorkModule::get_float(fighter.module_accessor, 0x4D);
    if rebel_gauge >= 20.0 // FIGHTER_JACK_INSTANCE_WORK_ID_FLOAT_REBEL_GAUGE
    || VarModule::is_flag(fighter.module_accessor, vars::jack::instance::flag::SPECIAL_LW_CANCEL) {
        VarModule::off_flag(fighter.module_accessor, vars::jack::instance::flag::SPECIAL_LW_CANCEL);
        WorkModule::on_flag(fighter.module_accessor, 0x200000E3); // FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE_SUMMON
        FighterSpecializer_Jack::check_doyle_summon_dispatch(fighter.module_accessor, true, false);
        StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_JACK_STATUS_KIND_SUMMON);
        return 1.into();
    }
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        0,
        0
    );
    0.into()
}

pub unsafe extern "C" fn jack_special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mot = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        Hash40::new("special_air_lw_start")
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        Hash40::new("special_lw_start")
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
    fighter.sub_shift_status_main(L2CValue::Ptr(jack_special_lw_main_loop as *const () as _))
}

unsafe extern "C" fn jack_special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    let is_start = [hash40("special_lw_start"), hash40("special_air_lw_start")].contains(&MotionModule::motion_kind(fighter.module_accessor));
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        let mot = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            if is_start {
                Hash40::new("special_air_lw_start")
            }
            else {
                Hash40::new("special_air_lw_end")
            }
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            if is_start {
                Hash40::new("special_lw_start")
            }
            else {
                Hash40::new("special_lw_end")
            }
        };
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
    if MotionModule::is_end(fighter.module_accessor) {
        if is_start {
            let mot = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
                Hash40::new("special_air_lw_end")
            }
            else {
                Hash40::new("special_lw_end")
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
        }
        else {
            let status = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
                FIGHTER_STATUS_KIND_FALL
            }
            else {
                FIGHTER_STATUS_KIND_WAIT
            };
            fighter.change_status(status.into(), false.into());
        }
    }
    0.into()
}

pub unsafe extern "C" fn jack_special_lw2_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, 0x200000E4); // FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE_END
    FighterSpecializer_Jack::check_doyle_summon_dispatch(fighter.module_accessor, true, false);
    StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_JACK_STATUS_KIND_DISPATCH);
    1.into()
}