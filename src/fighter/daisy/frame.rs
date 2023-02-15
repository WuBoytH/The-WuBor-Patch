use {
    smash::{
        lua2cpp::*,
        app::lua_bind::*,
        lib::{lua_const::*, *}
    },
    smashline::*,
    custom_var::*,
    wubor_utils::{
        wua_bind::*,
        vars::*
    }
};

#[fighter_frame( agent = FIGHTER_KIND_DAISY, main )]
fn daisy_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        if VarModule::is_flag(fighter.battle_object, fighter::instance::flag::DISABLE_SPECIAL_S)
        && (StatusModule::situation_kind(fighter.module_accessor) != *SITUATION_KIND_AIR
        || MiscModule::is_damage_check(fighter.module_accessor, false)) {
            VarModule::off_flag(fighter.battle_object, fighter::instance::flag::DISABLE_SPECIAL_S);
        }
    }
}

pub fn install() {
    install_agent_frames!(
        daisy_frame
    );
}