use super::*;

unsafe extern "C" fn special_hi_rush_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    let fall_x_mul = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_FALL_X_MAX_MUL);
    sv_kinetic_energy!(
        mul_x_speed_max,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        fall_x_mul
    );
    original_status(Main, fighter, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH_END)(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH_END, special_hi_rush_end_main);
}