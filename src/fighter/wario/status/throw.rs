use {
    crate::imports::*,
    super::super::vl
};

unsafe extern "C" fn wario_throw_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, wario::status::flag::THROW_B_MOVE) {
        // KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        // KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        if !VarModule::is_flag(fighter.module_accessor, wario::status::flag::THROW_B_CONTROL_RESET) {
            sv_kinetic_energy!(
                reset_energy,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                ENERGY_CONTROLLER_RESET_TYPE_FREE,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0
            );
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            VarModule::on_flag(fighter.module_accessor, wario::status::flag::THROW_B_CONTROL_RESET);
        }
        sv_kinetic_energy!(
            set_stable_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            vl::param_private::throw_b_speed_max,
            0.0
        );
        sv_kinetic_energy!(
            set_limit_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            vl::param_private::throw_b_speed_max,
            0.0
        );
        sv_kinetic_energy!(
            controller_set_accel_x_add,
            fighter,
            vl::param_private::throw_b_accel
        );
    }
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Exec, *FIGHTER_STATUS_KIND_THROW, wario_throw_exec);
}