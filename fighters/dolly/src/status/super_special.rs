use super::*;

unsafe extern "C" fn dolly_super_special_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, vars::dolly::instance::flag::SPECIAL_CANCEL) {
        VarModule::on_flag(fighter.module_accessor, vars::dolly::status::flag::IS_SPECIAL_CANCEL);
        VarModule::off_flag(fighter.module_accessor, vars::dolly::instance::flag::SPECIAL_CANCEL);
    }

    fighter.sub_set_special_start_common_kinetic_setting(hash40("param_super_special").into());

    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("super_special"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );

    dolly_super_special_set_kinetic(fighter, false.into(), 0.into(), 0.into());

    let func = smashline::api::get_target_function("lua2cpp_dolly.nrs", 0x12d50).unwrap();
    let dolly_map_col : fn(&mut L2CFighterCommon, L2CValue) = std::mem::transmute(func);
    dolly_map_col(fighter, hash40("param_super_special").into());

    if !StopModule::is_stop(fighter.module_accessor) {
        dolly_super_special_substatus(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(dolly_super_special_substatus as *const () as _));

    fighter.sub_shift_status_main(L2CValue::Ptr(dolly_super_special_main_loop as *const () as _))
}

unsafe extern "C" fn dolly_super_special_set_kinetic(
    fighter: &mut L2CFighterCommon,
    param_1: L2CValue,
    param_2: L2CValue,
    param_3: L2CValue
) {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        if param_1.get_bool() {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        }
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        if param_2.get_bool() {
            let func : fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(param_2.get_ptr());
            func(fighter);
        }
    }
    else {
        if param_1.get_bool() {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        }
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if param_3.get_bool() {
            let func : fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(param_3.get_ptr());
            func(fighter);
        }
    }
}

unsafe extern "C" fn dolly_super_special_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if !param_1.get_bool() {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL_RAW)
        && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK_RAW) {
            VarModule::on_flag(fighter.module_accessor, vars::dolly::status::flag::SUPER_SPECIAL_TRY_TRIPLE);
        }
    }
    let func = smashline::api::get_target_function("lua2cpp_dolly.nrs", 0x15440).unwrap();
    let dolly_super_special_substatus_inner : fn(&mut L2CValue, &mut L2CFighterCommon, L2CValue) -> L2CValue = std::mem::transmute(func);
    let ret = &mut L2CValue::I32(0);
    dolly_super_special_substatus_inner(ret, fighter, param_1);

    ret.clone()
}

unsafe extern "C" fn dolly_super_special_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }

    if VarModule::is_flag(fighter.module_accessor, vars::dolly::status::flag::SUPER_SPECIAL_CHECK_TRIPLE) {
        VarModule::off_flag(fighter.module_accessor, vars::dolly::status::flag::SUPER_SPECIAL_CHECK_TRIPLE);

        if VarModule::is_flag(fighter.module_accessor, vars::dolly::status::flag::SUPER_SPECIAL_TRY_TRIPLE)
        && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_ENABLE_SUPER_SPECIAL) {
            MotionModule::change_motion_inherit_frame(
                fighter.module_accessor,
                Hash40::new("super_special_triple"),
                -1.0,
                1.0,
                0.0,
                false,
                false
            );

            VarModule::on_flag(fighter.module_accessor, vars::fighter::instance::flag::BURNOUT);

            WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);
        }
    }

    if MotionModule::is_end(fighter.module_accessor) {
        let status = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            FIGHTER_STATUS_KIND_WAIT
        }
        else {
            FIGHTER_STATUS_KIND_FALL
        };
        fighter.change_status(status.into(), false.into());
        return 0.into();
    }

    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        dolly_super_special_set_kinetic(fighter, true.into(), 0.into(), 0.into());
    }

    0.into()
}

unsafe extern "C" fn dolly_super_special_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.module_accessor, vars::dolly::status::flag::IS_SPECIAL_CANCEL);
    dolly_super_special_end_helper(fighter, hash40("param_super_special").into());
    let eff_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SUPER_SPECIAL_WORK_INT_SCREEN_EFFECT_COUNT);
    if 0 < eff_count {
        MotionAnimcmdModule::call_script_single(
            fighter.module_accessor,
            *FIGHTER_ANIMCMD_EFFECT,
            Hash40::new("effect_super_specialcancelfillscreen"),
            -1
        );
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL, dolly_super_special_main);
    agent.status(End, *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL, dolly_super_special_end);
}