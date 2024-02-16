use {
    smash::{
        app::*,
        lib::lua_const::*
    },
    custom_var::*,
    wubor_utils::vars::*
};

// Used for when generic weapons hit something else.
#[skyline::hook(offset = 0x33a8260)]
unsafe extern "C" fn weapon_attack_callback(weapon: *mut BattleObject, arg: u64) {
    if (*weapon).kind == *WEAPON_KIND_LUCARIO_AURABALL as u32 {
        *(weapon as *mut bool).add(0x90) = VarModule::is_flag((*weapon).module_accessor, lucario_auraball::instance::flag::SPIRIT_BOMB);
    }
    call_original!(weapon, arg)
}

pub fn install() {
    skyline::install_hooks!(
        weapon_attack_callback
    );
}