use super::*;

unsafe extern "C" fn ryu_final_landing_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    // HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);

    let mot = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLAG_HIT) {
        "final_hit_landing"
    }
    else {
        "final_landing"
    };
    let end_frame = MotionModule::end_frame_from_hash(fighter.module_accessor, Hash40::new(mot));
    let final2_landing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_private"), hash40("final_landing_frame")) as f32;
    let rate = end_frame / final2_landing_frame;

    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new(mot),
        0.0,
        rate,
        false,
        0.0,
        false,
        false
    );

    let main_loop = smashline::api::get_target_function("lua2cpp_ryu.nrs", 0x43770).unwrap();
    fighter.sub_shift_status_main(L2CValue::Ptr(main_loop as *const () as _))
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_RYU_STATUS_KIND_FINAL_LANDING, ryu_final_landing_main);
}