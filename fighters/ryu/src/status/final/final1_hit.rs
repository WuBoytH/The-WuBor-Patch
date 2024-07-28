use super::*;

unsafe extern "C" fn ryu_final_hit_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);

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

    let mot = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        "final_hit"
    }
    else {
        "final_air_hit"
    };
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new(mot),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );

    let main_loop = smashline::api::get_target_function("lua2cpp_ryu.nrs", 0x430a0).unwrap();
    fighter.sub_shift_status_main(L2CValue::Ptr(main_loop as *const () as _))
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_RYU_STATUS_KIND_FINAL_HIT, ryu_final_hit_main);
}