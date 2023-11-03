use {
    smash::{
        app::{lua_bind::*, *},
        lib::lua_const::*
    },
    custom_var::*,
    wubor_utils::vars
};

#[skyline::hook(replace = WorkModule::is_enable_transition_term )]
pub unsafe fn is_enable_transition_term_replace(module_accessor: &mut BattleObjectModuleAccessor, term: i32) -> bool {
    let ret = original!()(module_accessor, term);
    let kind = utility::get_kind(module_accessor);
    if utility::get_category(module_accessor) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if kind == *FIGHTER_KIND_LUCINA { // Make this a custom command grab
            if VarModule::is_flag(module_accessor, vars::yu::instance::flag::HEROIC_GRAB)
            && term != *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT
            && term != *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_HI
            && term != *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3
            && term != *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN {
                return false;
            }
        }
        else if kind == *FIGHTER_KIND_RYU { // make secret sensation its own status
            if VarModule::is_flag(module_accessor, vars::ryu::instance::flag::SEC_SEN_CAMERA) {
                return false;
            }
        }
    }
    ret
}

pub fn install() {
    skyline::install_hooks!(
        is_enable_transition_term_replace
    );
    
}