use crate::imports::status_imports::*;

#[status("demon", FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR)]
unsafe fn demon_landing_attack_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_LandingAttackAirSub();
    fighter.sub_shift_status_main(L2CValue::Ptr(demon_landing_attack_air_main_loop as *const () as _))
}

unsafe extern "C" fn demon_landing_attack_air_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor)
    && MotionModule::motion_kind(fighter.module_accessor) == hash40("landing_air_lw") {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_DOWN_FLAG_UP);
        ControlModule::reset_down_stand_fb_kind(fighter.module_accessor);
        fighter.change_status(FIGHTER_STATUS_KIND_DOWN_WAIT.into(), false.into());
        return 1.into();
    }
    fighter.status_LandingAttackAir_Main()
}

pub fn install() {
    demon_landing_attack_air_main::install();
}