use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        phx::Hash40,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smash_script::*,
    smashline::*,
    wubor_utils::{
        vars::*,
        table_const::*
    }
};

#[status_script(agent = "luigi", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn luigi_specials_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    luigi_remove_thunderhand_eff(fighter);
    0.into()
}

#[status_script(agent = "luigi", status = FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_CHARGE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn luigi_specials_charge_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_DISCHARGE);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_FLASHING);
    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_WORK_FLOAT_CHARGE);
    if !StopModule::is_stop(fighter.module_accessor) {
        luigi_specials_charge_substatus(fighter);
    }
    fighter.global_table[SUB_STATUS2].assign(&L2CValue::Ptr(luigi_specials_charge_substatus as *const () as _));
    luigi_specials_charge_mot_helper(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(luigi_specials_charge_main_loop as *const () as _))
}

unsafe extern "C" fn luigi_specials_charge_substatus(fighter: &mut L2CFighterCommon) -> L2CValue {
    let charge_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("charge_speed_mul"));
    WorkModule::add_float(fighter.module_accessor, charge_speed, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_WORK_FLOAT_CHARGE);
    0.into()
}

unsafe extern "C" fn luigi_specials_charge_mot_helper(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_FIRST) == false {
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new("special_air_s_hold"),
                1.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_FIRST);
        }
        else {
            MotionModule::change_motion_inherit_frame(
                fighter.module_accessor,
                Hash40::new("special_air_s_hold"),
                -1.0,
                1.0,
                0.0,
                false,
                false
            );
        }
        WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_GROUND, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_INT_MTRANS);
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_FIRST) == false {
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new("special_s_hold"),
                1.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_FIRST);
        }
        else {
            MotionModule::change_motion_inherit_frame(
                fighter.module_accessor,
                Hash40::new("special_s_hold"),
                -1.0,
                1.0,
                0.0,
                false,
                false
            );
        }
        WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_AIR, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_INT_MTRANS);
    }
    return
}

unsafe extern "C" fn luigi_specials_charge_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) == false {
        let charge = WorkModule::get_float(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_WORK_FLOAT_CHARGE);
        let charge_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("charge_frame"));
        if charge_frame <= charge {
            fighter.change_status(FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_END.into(), false.into());
        }
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            luigi_specials_charge_mot_helper(fighter);
        }
    }
    else {
        fighter.change_status(FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_END.into(), false.into());
    }
    0.into()
}

#[status_script(agent = "luigi", status = FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_CHARGE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn luigi_specials_charge_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    luigi_remove_thunderhand_eff(fighter);
    0.into()
}

#[status_script(agent = "luigi", status = FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn luigi_specials_end_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    luigi_remove_thunderhand_eff(fighter);
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HITSTUN);
    0.into()
}

unsafe extern "C" fn luigi_remove_thunderhand_eff(fighter: &mut L2CFighterCommon) {
    if ![
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_CHARGE,
        *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_END
    ].contains(&fighter.global_table[STATUS_KIND].get_i32()) {
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_thunder"), false, true);
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("luigi_rocket_hold"), false, true);
    }
}

#[status_script(agent = "luigi", status = FIGHTER_LUIGI_STATUS_KIND_SPECIAL_HI_DROP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn luigi_specialhi_drop_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let fall_max_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("fall_max_x"));
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    let stable_speed_y = sv_kinetic_energy::get_stable_speed_y(fighter.lua_state_agent);
    sv_kinetic_energy!(
        set_stable_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        fall_max_x,
        stable_speed_y
    );
    let fall_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("fall_x_mul"));
    sv_kinetic_energy!(
        controller_set_accel_x_mul,
        fighter,
        fall_x_mul
    );
    sv_kinetic_energy!(
        controller_set_accel_x_add,
        fighter,
        0
    );
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_fall_common_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(luigi_specialhi_drop_substatus as *const () as _));
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_hi_drop"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_CANCEL)
    || WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FUNNY) {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(luigi_specialhi_drop_main_loop as *const () as _))
}

unsafe extern "C" fn luigi_specialhi_drop_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    fighter.sub_fall_common_uniq(param_1.clone())
}

unsafe extern "C" fn luigi_specialhi_drop_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 0.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if !WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_FALL_SPECIAL) {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_LUIGI_STATUS_KIND_SPECIAL_HI_FALL.into(), false.into());
        }
    }
    else {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_LUIGI_STATUS_KIND_SPECIAL_HI_LANDING_FALL.into(), false.into());
        }
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        luigi_specials_end,
        luigi_specials_charge_main,
        luigi_specials_charge_end,
        luigi_specials_end_end,
        luigi_specialhi_drop_main
    );
}