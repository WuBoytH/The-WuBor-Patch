use crate::imports::status_imports::*;

unsafe extern "C" fn ryu_special_lw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, ryu::instance::flag::DENJIN_CHARGE) {
        StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F);
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
        (
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW
        ) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn ryu_special_lw_init(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn ryu_special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ryu_special_lw_mot_helper(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(ryu_special_lw_main_loop as *const () as _))
}

unsafe extern "C" fn ryu_special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        ryu_special_lw_mot_helper(fighter);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        let status = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            FIGHTER_STATUS_KIND_WAIT
        }
        else {
            FIGHTER_STATUS_KIND_FALL
        };
        fighter.change_status(status.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn ryu_special_lw_mot_helper(fighter: &mut L2CFighterCommon) {
    let mut speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let mut speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let start_speed_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("start_speed_x_mul"));
    speed_x *= start_speed_x_mul;
    let situation = fighter.global_table[SITUATION_KIND].get_i32();
    let mot;
    let kinetic;
    let correct;
    if situation == *SITUATION_KIND_GROUND {
        mot = hash40("special_lw");
        kinetic = *FIGHTER_KINETIC_TYPE_GROUND_STOP;
        correct = *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP;
    }
    else {
        mot = hash40("special_air_lw");
        kinetic = *FIGHTER_KINETIC_TYPE_AIR_STOP;
        correct = *GROUND_CORRECT_KIND_AIR;
    }
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(correct));
    KineticModule::change_kinetic(fighter.module_accessor, kinetic);
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_MOTION_FIRST) {
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
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_MOTION_FIRST);
    }
    else {
        MotionModule::change_motion_inherit_frame(
            fighter.module_accessor,
            Hash40::new_raw(mot),
            -1.0,
            1.0,
            0.0,
            false,
            false
        );
    }
    if situation != *SITUATION_KIND_GROUND {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_SPECIAL_AIR_LW) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_FLAG_AIR_FIRST);
            let start_speed_y_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("start_speed_y_mul"));
            speed_y *= start_speed_y_mul;
        }
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            ENERGY_STOP_RESET_TYPE_AIR,
            speed_x,
            0.0,
            0.0,
            0.0,
            0.0
        );
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
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
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            ENERGY_GRAVITY_RESET_TYPE_GRAVITY,
            0.0,
            speed_y,
            0.0,
            0.0,
            0.0
        );
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_LW_FLAG_AIR_FIRST) {
            let accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("accel_y"));
            sv_kinetic_energy!(
                set_accel,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                -accel_y
            );
            let max_speed_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("max_speed_y"));
            sv_kinetic_energy!(
                set_limit_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                max_speed_y
            );
        }
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_SPECIAL_AIR_LW);
    }
    else {
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            ENERGY_STOP_RESET_TYPE_GROUND,
            speed_x,
            0.0,
            0.0,
            0.0,
            0.0
        );
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_MOTION,
            ENERGY_MOTION_RESET_TYPE_GROUND_TRANS,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0
        );
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, fighter.module_accessor);
    }
}

unsafe extern "C" fn ryu_special_lw_exec(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn ryu_special_lw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !VarModule::is_flag(fighter.module_accessor, ryu::instance::flag::DENJIN_CHARGE) {
        let eff_handle = VarModule::get_int(fighter.module_accessor, ryu::instance::int::DENJIN_EFF_HANDLE) as u32;
        if EffectModule::is_exist_effect(fighter.module_accessor, eff_handle) {
            EffectModule::kill(fighter.module_accessor, eff_handle, true, true);
        }
    }
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, ryu_special_lw_pre);
    agent.status(smashline::Init, *FIGHTER_STATUS_KIND_SPECIAL_LW, ryu_special_lw_init);
    agent.status(smashline::Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, ryu_special_lw_main);
    agent.status(smashline::Exec, *FIGHTER_STATUS_KIND_SPECIAL_LW, ryu_special_lw_exec);
    agent.status(smashline::End, *FIGHTER_STATUS_KIND_SPECIAL_LW, ryu_special_lw_end);
}