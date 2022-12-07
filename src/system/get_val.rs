use {
    smash::{
        hash40,
        app::{lua_bind::*, *},
        lib::lua_const::*
    },
    custom_var::*,
    wubor_utils::{wua_bind::*, vars::*}
};

#[skyline::hook(replace = WorkModule::get_int64 )]
pub unsafe fn get_int64_replace(boma: &mut BattleObjectModuleAccessor, term: i32) -> u64 {
    let ret = original!()(boma,term);
    if utility::get_category(boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if utility::get_kind(boma) == *FIGHTER_KIND_LUCINA
        && term == *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND {
            let object_id = boma.battle_object_id;
            let object = MiscModule::get_battle_object_from_id(object_id);
            if VarModule::is_flag(object, yu::instance::flag::HEROIC_GRAB) {
                return hash40("throw_hi");
            }
        }
    }
    ret
}

pub fn install() {
    skyline::install_hooks!(
        get_int64_replace
    );
    
}