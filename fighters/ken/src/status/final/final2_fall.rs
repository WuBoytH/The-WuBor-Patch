use super::*;

unsafe extern "C" fn ken_final2_fall_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    // HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);

    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("final2_fall"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );

    let final2_fall_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_private"), hash40("final2_fall_frame"));
    WorkModule::set_int(fighter.module_accessor, final2_fall_frame, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_INT_COUNTER);

    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(ken_final2_fall_substatus as *const () as _));

    let gravity = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);

    let final2_fall_brake_y_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_private"), hash40("final2_fall_brake_y_mul"));
    lua_bind::FighterKineticEnergyGravity::set_brake(gravity as *mut smash::app::FighterKineticEnergyGravity, final2_fall_brake_y_mul);

    let air_speed_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
    let air_speed_y_limit = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("air_speed_y_limit"));
    let final2_air_speed_y_stable_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_private"), hash40("final2_air_speed_y_stable_mul"));
    lua_bind::FighterKineticEnergyGravity::set_stable_speed(
        gravity as *mut smash::app::FighterKineticEnergyGravity,
        air_speed_y_stable * final2_air_speed_y_stable_mul
    );
    lua_bind::FighterKineticEnergyGravity::set_limit_speed(
        gravity as *mut smash::app::FighterKineticEnergyGravity,
        air_speed_y_limit * final2_air_speed_y_stable_mul
    );

    ken_final_set_area(fighter, false.into());

    let main_loop = smashline::api::get_target_function("lua2cpp_ken.nrs", 0x3e1f0).unwrap();
    fighter.sub_shift_status_main(L2CValue::Ptr(main_loop as *const () as _))
}

unsafe extern "C" fn ken_final2_fall_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() {
        WorkModule::dec_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_INT_COUNTER);
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_RYU_STATUS_KIND_FINAL2_FALL, ken_final2_fall_main);
}