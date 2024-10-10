use super::*;

unsafe extern "C" fn ken_final_hit_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let camera_func : fn(fighter: &mut L2CFighterCommon, cam_const: L2CValue) = std::mem::transmute(smashline::api::get_target_function("lua2cpp_ken.nrs", 0x40590).unwrap());
    camera_func(fighter, FIGHTER_RYU_FINAL_CAMERA_OFFSET_1.into());

    let final_shake_scale2 = WorkModule::get_param_float(fighter.module_accessor, hash40("param_private"), hash40("final_shake_scale2"));
    AttackModule::set_damage_shake_scale(fighter.module_accessor, final_shake_scale2);

    MotionModule::set_trans_move_speed_no_scale(fighter.module_accessor, false);

    ken_final_set_area(fighter, false.into());

    // HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);

    let mut zoom = 1.1;
    if PostureModule::scale(fighter.module_accessor) > 1.0 {
        zoom *= PostureModule::scale(fighter.module_accessor);
    }
    CameraModule::zoom_in(
        fighter.module_accessor,
        5,
        0,
        zoom,
        &Vector2f{x: 0.0, y: 0.0},
        true
    );

    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);

    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("final_hit"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );

    let main_loop = smashline::api::get_target_function("lua2cpp_ken.nrs", 0x40b30).unwrap();
    fighter.sub_shift_status_main(L2CValue::Ptr(main_loop as *const () as _))
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_RYU_STATUS_KIND_FINAL_HIT, ken_final_hit_main);
}