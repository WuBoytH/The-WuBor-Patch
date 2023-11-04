use crate::imports::status_imports::*;

#[status_script(agent = "ken", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn ken_special_s_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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
    let attr = if VarModule::is_flag(fighter.module_accessor, ken::instance::flag::QUICK_STEP_INHERIT) {
        0
    }
    else {
        *FIGHTER_STATUS_ATTR_START_TURN
    };
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S |
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
            *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON
        ) as u64,
        attr as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    0.into()
}

#[status_script(agent = "ken", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn ken_special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ken_special_s_main_inner(fighter)
}

#[status_script(agent = "ken", status = FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn ken_special_s_command_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ken_special_s_main_inner(fighter)
}

pub unsafe extern "C" fn ken_special_s_main_inner(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, ken::instance::flag::QUICK_STEP_INHERIT) {
        VarModule::on_flag(fighter.module_accessor, ken::status::flag::QUICK_STEP_INHERITED);
        VarModule::off_flag(fighter.module_accessor, ken::instance::flag::QUICK_STEP_INHERIT);
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(ken_special_s_substatus as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(ken_special_s_main_loop as *const () as _))
}

unsafe extern "C" fn ken_special_s_substatus(fighter: &mut L2CFighterCommon, param_2: L2CValue) -> L2CValue {
    if param_2.get_bool() {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_BUTTON_ON_TIMER);
    }
    0.into()
}

unsafe extern "C" fn ken_special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_changing(fighter.module_accessor)
    || StatusModule::is_situation_changed(fighter.module_accessor) {
        ken_special_s_mot_helper(fighter);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn ken_special_s_mot_helper(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if StatusModule::is_changing(fighter.module_accessor) {
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new("special_air_s_start"),
                0.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
        }
        if StatusModule::is_changing(fighter.module_accessor) {
            return;
        }
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("air_accel_y"));
        sv_kinetic_energy!(
            set_accel,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            -air_accel_y
        );
        let air_max_speed_y = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND) {
            1.0
        }
        else {
            WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("air_max_speed_y"))
        };
        sv_kinetic_energy!(
            set_limit_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            air_max_speed_y
        );
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        if StatusModule::is_changing(fighter.module_accessor) {
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new("special_s_start"),
                0.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
        }
        if StatusModule::is_changing(fighter.module_accessor) {
            return;
        }
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        sv_kinetic_energy!(
            set_brake,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            0.0,
            0.0
        );
    }
}

#[status_script(agent = "ken", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn ken_special_s_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    ken_special_s_end_inner(fighter)
}

#[status_script(agent = "ken", status = FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn ken_special_s_command_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    ken_special_s_end_inner(fighter)
}

pub unsafe extern "C" fn ken_special_s_end_inner(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP {
        EffectModule::kill_kind(
            fighter.module_accessor,
            Hash40::new("ken_syoryuken_fire"),
            false,
            true
        );
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        ken_special_s_pre,
        ken_special_s_main,
        ken_special_s_command_main,
        ken_special_s_end,
        ken_special_s_command_end
    );
}