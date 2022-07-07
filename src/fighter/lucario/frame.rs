use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::{lua_bind::*, *},
        lib::lua_const::*
    },
    smashline::*,
    custom_var::*,
    wubor_utils::vars::*,
    super::vl
};

unsafe fn lucario_training_tools(fighter: &mut L2CFighterCommon) {
    if smashball::is_training_mode()
    && [
        *FIGHTER_STATUS_KIND_GUARD_ON,
        *FIGHTER_STATUS_KIND_GUARD
    ].contains(&StatusModule::status_kind(fighter.module_accessor)) {
        let charges = VarModule::get_int(fighter.battle_object, lucario::instance::int::AURA_LEVEL);
        let charge_max = vl::private::AURA_CHARGE_MAX;
        if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L)
        && charges > 0 {
            VarModule::dec_int(fighter.battle_object, lucario::instance::int::AURA_LEVEL);
        }
        if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R)
        && charges < charge_max {
            VarModule::inc_int(fighter.battle_object, lucario::instance::int::AURA_LEVEL);
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_LUCARIO )]
fn lucario_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        lucario_training_tools(fighter);
    }
}

pub fn install() {
    install_agent_frames!(
        lucario_frame
    );
}