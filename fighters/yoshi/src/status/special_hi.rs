use super::*;

unsafe extern "C" fn yoshi_special_hi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
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
        (
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI |
            *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON
        ) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn yoshi_special_hi_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        *SITUATION_KIND_AIR
    }
    else {
        *SITUATION_KIND_GROUND
    };
    VarModule::set_int(fighter.module_accessor, vars::yoshi::status::int::SPECIAL_HI_START_SITUATION, situation);
    fighter.set_situation(SITUATION_KIND_AIR.into());
    GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
    let y_acl;
    let start_speed_y;
    if situation == *SITUATION_KIND_AIR {
        y_acl = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
        start_speed_y = 0.0;
    }
    else {
        y_acl = 0.0;
        start_speed_y = 0.2;
    }
    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        start_speed_y
    );
    sv_kinetic_energy!(
        set_accel,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        -y_acl
    );
    sv_kinetic_energy!(
        set_stable_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        air_speed_x_stable * 0.8,
        -1.0
    );
    sv_kinetic_energy!(
        set_limit_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        air_speed_x_stable * 0.8,
        -1.0
    );
    sv_kinetic_energy!(
        controller_set_accel_x_add,
        fighter,
        0.01
    );
    0.into()
}

unsafe extern "C" fn yoshi_special_hi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::on_flag(fighter.module_accessor, vars::fighter::instance::flag::DISABLE_SPECIAL_HI);
    VarModule::on_flag(fighter.module_accessor, vars::yoshi::status::flag::SPECIAL_HI_SPECIAL_HOLD);
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_air_hi"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    if !StopModule::is_stop(fighter.module_accessor) {
        yoshi_special_hi_substatus(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(yoshi_special_hi_substatus as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(yoshi_special_hi_main_loop as *const () as _))
}

unsafe extern "C" fn yoshi_special_hi_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() {
        if !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            VarModule::off_flag(fighter.module_accessor, vars::yoshi::status::flag::SPECIAL_HI_SPECIAL_HOLD);
        }
    }
    0.into()
}

unsafe extern "C" fn yoshi_special_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }

    let cancel = if VarModule::get_int(fighter.module_accessor, vars::yoshi::status::int::SPECIAL_HI_START_SITUATION) == *SITUATION_KIND_AIR {
        VarModule::is_flag(fighter.module_accessor, vars::yoshi::status::flag::SPECIAL_HI_RISE_AIR_CANCEL)
    }
    else {
        VarModule::is_flag(fighter.module_accessor, vars::yoshi::status::flag::SPECIAL_HI_RISE_GROUND_CANCEL)
    };
    if MotionModule::is_end(fighter.module_accessor)
    || (
        cancel &&
        !VarModule::is_flag(fighter.module_accessor, vars::yoshi::status::flag::SPECIAL_HI_SPECIAL_HOLD)
    ) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }
    0.into()
}

unsafe extern "C" fn yoshi_special_hi_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, vars::yoshi::status::flag::SPECIAL_HI_RISE) {
        VarModule::off_flag(fighter.module_accessor, vars::yoshi::status::flag::SPECIAL_HI_RISE);
        let y_acl = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
        sv_kinetic_energy!(
            set_accel,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            y_acl
        );
    }
    if VarModule::is_flag(fighter.module_accessor, vars::yoshi::status::flag::SPECIAL_HI_RISE_CUT) {
        VarModule::off_flag(fighter.module_accessor, vars::yoshi::status::flag::SPECIAL_HI_RISE_CUT);
        let y_acl = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
        sv_kinetic_energy!(
            set_accel,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            -y_acl * 0.5
        );
    }
    0.into()
}

unsafe extern "C" fn yoshi_special_hi_end(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, yoshi_special_hi_pre);
    agent.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_HI, yoshi_special_hi_init);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, yoshi_special_hi_main);
    agent.status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_HI, yoshi_special_hi_exec);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_HI, yoshi_special_hi_end);
}