use {
    smash::{
        lua2cpp::*,
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

#[line("daisy", main)]
unsafe fn daisy_frame(fighter: &mut L2CFighterCommon) {
    if VarModule::is_flag(fighter.battle_object, fighter::instance::flag::DISABLE_SPECIAL_S)
    && (StatusModule::situation_kind(fighter.module_accessor) != *SITUATION_KIND_AIR
    || MiscModule::is_damage_check(fighter.module_accessor, false)) {
        VarModule::off_flag(fighter.battle_object, fighter::instance::flag::DISABLE_SPECIAL_S);
    }
}

pub fn install() {
    daisy_frame::install();
}