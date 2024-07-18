use super::*;

unsafe extern "C" fn ryu_final_fall_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    // HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);

    let mot = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_FLAG_HIT) {
        "final_hit_fall"
    }
    else {
        "final_fall"
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

    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(ryu_final_fall_substatus as *const () as _));

    let main_loop = smashline::api::get_target_function("lua2cpp_ryu.nrs", 0x43b30).unwrap();
    fighter.sub_shift_status_main(L2CValue::Ptr(main_loop as *const () as _))
}

unsafe extern "C" fn ryu_final_fall_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() {
        WorkModule::dec_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_FINAL_INT_COUNTER);
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_RYU_STATUS_KIND_FINAL_FALL, ryu_final_fall_main);
}