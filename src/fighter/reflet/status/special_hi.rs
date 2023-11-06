use crate::imports::status_imports::*;

unsafe extern "C" fn reflet_special_hi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_REFLET_MAGIC_KIND_EL_WIND, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_LAST_USED_MAGIC_KIND);
    let correct;
    let kinetic;
    let mot;
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        correct = *GROUND_CORRECT_KIND_AIR;
        kinetic = *FIGHTER_KINETIC_TYPE_AIR_STOP;
        mot = hash40("special_air_hi");
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_AIR);
    }
    else {
        correct = *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP;
        kinetic = *FIGHTER_KINETIC_TYPE_GROUND_STOP;
        mot = hash40("special_hi");
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_AIR);
    }
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(correct));
    KineticModule::change_kinetic(fighter.module_accessor, kinetic);
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
    WorkModule::dec_int(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT);
    let current_point = WorkModule::get_int(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT);
    if current_point <= 0 {
        let fightermoduleaccessor = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut FighterModuleAccessor;
        FighterSpecializer_Reflet::set_flag_to_table(
            fightermoduleaccessor,
            *FIGHTER_REFLET_MAGIC_KIND_EL_WIND,
            true,
            *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THROWAWAY_TABLE
        );
    }
    let special_hi_landing_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_landing_frame"));
    WorkModule::set_float(fighter.module_accessor, special_hi_landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    fighter.sub_shift_status_main(L2CValue::Ptr(reflet_special_hi_main_loop as *const () as _))
}

unsafe extern "C" fn reflet_special_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if StatusModule::is_situation_changed(fighter.module_accessor)
        && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
            return 1.into();
        }
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into()); // Was FALL_SPECIAL
            return 1.into();
        }
    }
    reflet_special_hi_check_jump(fighter);
    reflet_special_hi_try_2nd(fighter);
    0.into()
}

unsafe extern "C" fn reflet_special_hi_check_jump(fighter: &mut L2CFighterCommon) {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_JUMP) {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        let fightermoduleaccessor = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut FighterModuleAccessor;
        let speed = FighterSpecializer_Reflet::get_special_hi_jump_speed(fightermoduleaccessor);
        let grav_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) as *mut smash::app::KineticEnergy;
        KineticEnergy::reset_energy(
            grav_energy,
            *ENERGY_GRAVITY_RESET_TYPE_GRAVITY,
            &Vector2f{x: 0.0, y: speed.y},
            &Vector3f{x: 0.0, y: 0.0, z: 0.0},
            fighter.module_accessor
        );
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        let stop_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP) as *mut smash::app::KineticEnergy;
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            0.0,
            0.0
        );
        KineticEnergy::unable(stop_energy);
        let control_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticEnergy::reset_energy(
            control_energy as *mut smash::app::KineticEnergy,
            *ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST,
            &Vector2f{x: 0.0, y: 0.0},
            &Vector3f{x: 0.0, y: 0.0, z: 0.0},
            fighter.module_accessor
        );
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            speed.x,
            0.0
        );
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        let control_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), 0x2c13759450);
        let air_accel_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_mul"), 0);
        let air_accel_x_add = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_add"), 0);
        let air_speed_x_limit = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("air_speed_x_limit"));
        FighterKineticEnergyController::set_accel_x_mul(control_energy as *mut smash::app::FighterKineticEnergyController, air_accel_x_mul * control_mul);
        FighterKineticEnergyController::set_accel_x_add(control_energy as *mut smash::app::FighterKineticEnergyController, air_accel_x_add * control_mul);
        KineticEnergyNormal::set_limit_speed(control_energy as *mut smash::app::KineticEnergyNormal, &Vector2f{x: air_speed_x_limit * control_mul, y: 0.0});
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_JUMP);
        VarModule::on_flag(fighter.module_accessor, fighter::instance::flag::DISABLE_SPECIAL_HI);
    }
}

unsafe extern "C" fn reflet_special_hi_try_2nd(fighter: &mut L2CFighterCommon) {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_TRY_2ND)
    && (
        ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
        || WarkModule::is_operation_cpu(fighter.module_accessor)
    ) {
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT) > 0 {
            StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_HI_2, false);
        }
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_TRY_2ND);
    }
}

pub fn install(agent : &mut smashline::Agent) {
    agent.status(smashline::Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, reflet_special_hi_main);
}