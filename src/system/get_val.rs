use {
    smash::{
        hash40,
        app::{lua_bind::*, *},
        lib::lua_const::*
    },
    custom_var::*,
    wubor_utils::vars::*
};

#[skyline::hook(replace = WorkModule::get_int64 )]
pub unsafe fn get_int64_replace(module_accessor: &mut BattleObjectModuleAccessor, term: i32) -> u64 {
    let ret = original!()(module_accessor, term);
    if utility::get_category(module_accessor) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if utility::get_kind(module_accessor) == *FIGHTER_KIND_LUCINA
        && term == *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND {
            if VarModule::is_flag(module_accessor, yu::instance::flag::HEROIC_GRAB) {
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