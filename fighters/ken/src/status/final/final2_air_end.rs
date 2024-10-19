use super::*;

unsafe extern "C" fn ken_final2_air_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    // HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);

    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("final2_air_end"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );

    fighter.sub_shift_status_main(L2CValue::Ptr(ken_final2_air_end_main_loop as *const () as _))
}

unsafe extern "C" fn ken_final2_air_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }

    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_RYU_STATUS_KIND_FINAL2_LANDING.into(), false.into());
        return 0.into();
    }

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }

    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_RYU_STATUS_KIND_FINAL2_AIR_END, ken_final2_air_end_main);
}