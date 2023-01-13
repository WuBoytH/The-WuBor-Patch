use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{lua_bind::*, *},
        lib::lua_const::*
    },
    custom_var::*,
    smashline::*,
    wubor_utils::vars::*
};

#[weapon_frame( agent = WEAPON_KIND_EFLAME_ESWORD )]
fn eflame_esword_frame(weapon: &mut L2CFighterBase) {
    unsafe {
        if StatusModule::status_kind(weapon.module_accessor) == *WEAPON_EFLAME_ESWORD_STATUS_KIND_SPECIAL_S_FLY
        && VarModule::is_flag(weapon.battle_object, eflame_esword::status::flag::ENABLE_EARLY_SPIN)
        && ControlModule::check_button_on(weapon.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            WeaponSpecializer_EFlameEsword::energy_motion_set_speed_mul_2nd(weapon.module_accessor, &Vector2f{x: 0.1, y: 0.1});
            MotionModule::set_frame(weapon.module_accessor, 20.0, false);
            VarModule::off_flag(weapon.battle_object, eflame_esword::status::flag::ENABLE_EARLY_SPIN);
        }
    }
}

pub fn install() {
    install_agent_frames!(
        eflame_esword_frame
    );
}