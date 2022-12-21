use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::lua_bind::*,
        lib::lua_const::*
    },
    smashline::*,
    custom_var::*,
    wubor_utils::{
        wua_bind::*,
        vars::*
    }
};

#[fighter_frame( agent = FIGHTER_KIND_RICHTER )]
fn richter_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        if VarModule::is_flag(fighter.battle_object, fighter::instance::flag::DISABLE_SPECIAL_HI)
        && (StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH
        || StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND
        || MiscModule::is_damage_check(fighter.module_accessor, false)) {
            VarModule::off_flag(fighter.battle_object, fighter::instance::flag::DISABLE_SPECIAL_HI);
        }
    }
}

pub fn install() {
    install_agent_frames!(
        richter_frame
    );
}