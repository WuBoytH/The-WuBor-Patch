use {
    smash::{
        app::{lua_bind::*, *},
        lib::lua_const::*
    }
};

#[skyline::hook(offset = 0x1171c90)]
pub unsafe extern "C" fn shulk_shield_art_hit_decrease(vtable: u64, fighter: *mut u64, something: u64) {
    let boma = *fighter.add(4) as *mut BattleObjectModuleAccessor;
    if utility::get_category(&mut *boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER
    && utility::get_kind(&mut *boma) == *FIGHTER_KIND_KIRBY {
        original!()(vtable, fighter, something)
    }
}

pub fn install() {
    skyline::install_hooks!(
        shulk_shield_art_hit_decrease
    );
}