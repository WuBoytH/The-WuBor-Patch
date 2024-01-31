use {
    smash::{
        app::{lua_bind::*, FighterManager, *},
        lib::lua_const::*
    },
    crate::fighter::lucina::helper::*,
    custom_var::*,
    wubor_utils::vars::*
};

#[skyline::hook(offset = NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET)]
pub unsafe fn notify_log_event_collision_hit_replace(
fighter_manager: &mut FighterManager,
attacker_object_id: u32,
defender_object_id: u32, 
move_type: f32,
arg5: i32,
move_type_again: bool) -> u64 {
    let attacker_boma = sv_battle_object::module_accessor(attacker_object_id);
    let defender_boma = sv_battle_object::module_accessor(defender_object_id);
    let attacker_fighter_kind = sv_battle_object::kind(attacker_object_id);
    let defender_fighter_kind = sv_battle_object::kind(defender_object_id);
    // let a_entry_id = WorkModule::get_int(attacker_boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let attacker_cat = utility::get_category(&mut *attacker_boma);
    let defender_cat = utility::get_category(&mut *defender_boma);
    if attacker_cat == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if attacker_fighter_kind == *FIGHTER_KIND_LUCINA {
            if StatusModule::status_kind(attacker_boma) == *FIGHTER_STATUS_KIND_SPECIAL_LW {
                handle_slow(attacker_boma, defender_boma);
            }
        }
    }
    original!()(fighter_manager, attacker_object_id, defender_object_id, move_type, arg5, move_type_again)
}

pub fn install() {
    skyline::install_hooks!(
        notify_log_event_collision_hit_replace
    );
    
}