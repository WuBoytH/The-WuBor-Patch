use super::*;

unsafe extern "C" fn ken_final_jump_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    AttackModule::set_damage_shake_scale(fighter.module_accessor, 1.0);

    ken_final_set_area(fighter, false.into());

    // HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);

    let main_loop = smashline::api::get_target_function("lua2cpp_ken.nrs", 0x42100).unwrap();
    fighter.sub_shift_status_main(L2CValue::Ptr(main_loop as *const () as _))
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_RYU_STATUS_KIND_FINAL_JUMP, ken_final_jump_main);
}