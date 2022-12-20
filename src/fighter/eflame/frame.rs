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
            weapon.change_status(WEAPON_EFLAME_ESWORD_STATUS_KIND_SPECIAL_S_ROTATE.into(), false.into());
        }
    }
}

pub fn install() {
    install_agent_frames!(
        eflame_esword_frame
    );
}