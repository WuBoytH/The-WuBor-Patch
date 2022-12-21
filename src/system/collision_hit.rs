use {
    smash::{
        app::{lua_bind::*, FighterManager, *},
        lib::lua_const::*
    },
    crate::{
        fighter::lucina::helper::*
    },
    custom_var::*,
    wubor_utils::{wua_bind::*, vars::*}
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
    let attacker_object = MiscModule::get_battle_object_from_id(attacker_object_id);
    let defender_boma = sv_battle_object::module_accessor(defender_object_id);
    let defender_object = MiscModule::get_battle_object_from_id(defender_object_id);
    let attacker_fighter_kind = sv_battle_object::kind(attacker_object_id);
    let defender_fighter_kind = sv_battle_object::kind(defender_object_id);
    // let a_entry_id = WorkModule::get_int(attacker_boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let attacker_cat = utility::get_category(&mut *attacker_boma);
    let defender_cat = utility::get_category(&mut *defender_boma);
    if attacker_cat == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if attacker_fighter_kind == *FIGHTER_KIND_KEN {
            if defender_cat == *BATTLE_OBJECT_CATEGORY_FIGHTER {
                VarModule::set_int(attacker_object, fighter::instance::int::TARGET_ID, defender_object_id as i32);
            }
            else {
                VarModule::set_int(attacker_object, fighter::instance::int::TARGET_ID, 0);
            }
        }
        if attacker_fighter_kind == *FIGHTER_KIND_LUCINA {
            if StatusModule::status_kind(attacker_boma) == *FIGHTER_STATUS_KIND_SPECIAL_LW {
                handle_slow(attacker_object, defender_boma);
            }
        }
    }
    if defender_cat == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if defender_fighter_kind == *FIGHTER_KIND_RYU {
            if VarModule::is_flag(defender_object, ryu::instance::flag::SEC_SEN_STATE) {
                let target_x;
                let target_y;
                if attacker_cat == *BATTLE_OBJECT_CATEGORY_FIGHTER
                || attacker_cat == *BATTLE_OBJECT_CATEGORY_ENEMY {
                    VarModule::set_int(defender_object, fighter::instance::int::TARGET_ID, attacker_object_id as i32);
                    target_x = PostureModule::pos_x(attacker_boma);
                    target_y = PostureModule::pos_y(attacker_boma);
                    if utility::get_category(&mut *attacker_boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
                        JostleModule::set_status(&mut *attacker_boma, false);
                    }
                }
                else if attacker_cat == *BATTLE_OBJECT_CATEGORY_WEAPON {
                    let otarget_id = WorkModule::get_int(attacker_boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
                    let oboma = sv_battle_object::module_accessor(otarget_id);
                    if utility::get_category(&mut *oboma) != *BATTLE_OBJECT_CATEGORY_FIGHTER {
                        target_x = PostureModule::pos_x(defender_boma);
                        target_y = PostureModule::pos_y(defender_boma);
                        VarModule::set_int(defender_object, fighter::instance::int::TARGET_ID, 0);
                    }
                    else {
                        target_x = PostureModule::pos_x(oboma);
                        target_y = PostureModule::pos_y(oboma);
                        VarModule::set_int(defender_object, fighter::instance::int::TARGET_ID, otarget_id as i32);
                        if utility::get_category(&mut *oboma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
                            JostleModule::set_status(&mut *oboma, false);
                        }
                    }
                }
                else {
                    target_x = PostureModule::pos_x(defender_boma);
                    target_y = PostureModule::pos_y(defender_boma);
                    VarModule::set_int(defender_object, fighter::instance::int::TARGET_ID, 0);
                }
                VarModule::set_float(defender_object, ryu::instance::float::TARGET_X, target_x);
                VarModule::set_float(defender_object, ryu::instance::float::TARGET_Y, target_y);
                VarModule::on_flag(defender_object, ryu::instance::flag::SECRET_SENSATION);
            }
        }
        else if defender_fighter_kind == *FIGHTER_KIND_SHULK {
            if attacker_cat == *BATTLE_OBJECT_CATEGORY_FIGHTER
            || attacker_cat == *BATTLE_OBJECT_CATEGORY_ENEMY {
                VarModule::set_int(defender_object, fighter::instance::int::TARGET_ID, attacker_object_id as i32);
            }
            else if attacker_cat == *BATTLE_OBJECT_CATEGORY_WEAPON {
                let otarget_id = WorkModule::get_int(attacker_boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
                let oboma = sv_battle_object::module_accessor(otarget_id);
                if utility::get_category(&mut *oboma) != *BATTLE_OBJECT_CATEGORY_FIGHTER {
                    VarModule::set_int(defender_object, fighter::instance::int::TARGET_ID, 0)
                }
                else {
                    VarModule::set_int(defender_object, fighter::instance::int::TARGET_ID, otarget_id as i32);
                }
            }
            else {
                VarModule::set_int(defender_object, fighter::instance::int::TARGET_ID, 0)
            }
        }
    }
    if attacker_cat == *BATTLE_OBJECT_CATEGORY_WEAPON {
        if attacker_fighter_kind == *WEAPON_KIND_MARIO_FIREBALL {
            let object = MiscModule::get_battle_object_from_id((WorkModule::get_int(attacker_boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID)) as u32);
            VarModule::on_flag(object, mario::status::flag::SPECIAL_N_FGC_CANCEL);
        }
    }
    original!()(fighter_manager, attacker_object_id, defender_object_id, move_type, arg5, move_type_again)
}

pub fn install() {
    skyline::install_hooks!(
        notify_log_event_collision_hit_replace
    );
    
}