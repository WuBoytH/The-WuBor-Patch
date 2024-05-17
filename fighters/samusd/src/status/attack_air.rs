use {
    crate::imports::*,
    super::super::vl,
};

unsafe extern "C" fn samusd_attack_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air_common(true.into());
    if !StopModule::is_stop(fighter.module_accessor) {
        samusd_attack_air_substatus2(fighter);
    }
    fighter.global_table[SUB_STATUS2].assign(&L2CValue::Ptr(samusd_attack_air_substatus2 as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_status_AttackAir_Main as *const () as _))
}

unsafe extern "C" fn samusd_attack_air_substatus2(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_n")
    && VarModule::is_flag(fighter.module_accessor, vars::samusd::status::flag::ATTACK_AIR_N_START_FLOAT) {
        VarModule::on_flag(fighter.module_accessor, vars::samusd::instance::flag::ATTACK_AIR_N_FLOAT);
        let sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
        let mul = if sum_speed_y <= 0.0 {
            vl::param_private::attack_air_n_gravity_mul
        }
        else {
            1.0
        };
        sv_kinetic_energy!(
            set_accel,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            -air_accel_y * mul
        );
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_AIR, samusd_attack_air_main);
}