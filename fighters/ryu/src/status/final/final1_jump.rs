use super::*;

unsafe extern "C" fn ryu_final_jump_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    // HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);

    let main_loop = smashline::api::get_target_function("lua2cpp_ryu.nrs", 0x43e90).unwrap();
    fighter.sub_shift_status_main(L2CValue::Ptr(main_loop as *const () as _))
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_RYU_STATUS_KIND_FINAL_JUMP, ryu_final_jump_main);
}