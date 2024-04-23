use crate::imports::*;

unsafe extern "C" fn on_main(weapon: &mut L2CFighterBase) {
    if StatusModule::status_kind(weapon.module_accessor) == *WEAPON_EFLAME_ESWORD_STATUS_KIND_SPECIAL_S_FLY
    && VarModule::is_flag(weapon.module_accessor, eflame_esword::status::flag::ENABLE_EARLY_SPIN)
    && ControlModule::check_button_trigger(weapon.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        WeaponSpecializer_EFlameEsword::energy_motion_set_speed_mul_2nd(weapon.module_accessor, &Vector2f{x: 0.1, y: 0.1});
        MotionModule::set_frame(weapon.module_accessor, 20.0, false);
        VarModule::off_flag(weapon.module_accessor, eflame_esword::status::flag::ENABLE_EARLY_SPIN);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, on_main);
}