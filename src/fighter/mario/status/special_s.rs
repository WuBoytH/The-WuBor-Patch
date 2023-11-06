use crate::imports::status_imports::*;

unsafe extern "C" fn mario_special_s_init(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn mario_special_s_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
        let air_accel_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y_stable"), 0);
        let air_accel_x = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_mul"), 0);
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_S_HOP) {
            sv_kinetic_energy!(
                set_accel,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                -air_accel_y
            );
            sv_kinetic_energy!(
                set_accel_x_mul,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                air_accel_x * 0.5
            );
            sv_kinetic_energy!(
                set_limit_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                air_accel_y_stable
            );
        }
        else {
            sv_kinetic_energy!(
                set_accel,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                -air_accel_y / 1.5
            );
            sv_kinetic_energy!(
                set_accel_x_mul,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                air_accel_x * 0.2
            );
            sv_kinetic_energy!(
                set_limit_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                air_accel_y_stable / 2.0
            );
        }
    }
    0.into()
}

pub fn install(agent : &mut smashline::Agent) {
    agent.status(smashline::Init, *FIGHTER_STATUS_KIND_SPECIAL_S, mario_special_s_init);
    agent.status(smashline::Exec, *FIGHTER_STATUS_KIND_SPECIAL_S, mario_special_s_exec);
}