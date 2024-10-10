use super::*;

unsafe extern "C" fn ken_final2_landing_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    // HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);

    let end_frame = MotionModule::end_frame_from_hash(fighter.module_accessor, Hash40::new("final2_landing"));
    let final2_landing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_private"), hash40("final2_landing_frame")) as f32;
    let rate = end_frame / final2_landing_frame;

    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("final2_landing"),
        0.0,
        rate,
        false,
        0.0,
        false,
        false
    );

    let main_loop = smashline::api::get_target_function("lua2cpp_ken.nrs", 0x3de90).unwrap();
    fighter.sub_shift_status_main(L2CValue::Ptr(main_loop as *const () as _))
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_RYU_STATUS_KIND_FINAL2_LANDING, ken_final2_landing_main);
}