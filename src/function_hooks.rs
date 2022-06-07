use {
    smash::{
        hash40,
        app::{lua_bind::*, FighterManager, *},
        lib::{lua_const::*, L2CValue, L2CAgent}
    },
    crate::{
        fighter::{
            dolly::helper::*,
            gaogaen::helper::*,
            kamui::vars::*,
            ken::{helper::*, vars::*},
            lucina::{helper::*, vars::*},
            mario::vars::*,
            *
        }
    },
    custom_var::*,
    wubor_utils::vars::*,
    skyline::hooks::{
        getRegionAddress,
        Region
    }
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
    let attacker_object = sv_system::battle_object(attacker_object_id as u64);
    let defender_boma = sv_battle_object::module_accessor(defender_object_id);
    let defender_object = sv_system::battle_object(defender_object_id as u64);
    let attacker_fighter_kind = sv_battle_object::kind(attacker_object_id);
    let defender_fighter_kind = sv_battle_object::kind(defender_object_id);
    // let a_entry_id = WorkModule::get_int(attacker_boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let attacker_cat = utility::get_category(&mut *attacker_boma);
    let defender_cat = utility::get_category(&mut *defender_boma);
    if attacker_cat == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if attacker_fighter_kind == *FIGHTER_KIND_KEN {
            if defender_cat == *BATTLE_OBJECT_CATEGORY_FIGHTER {
                VarModule::set_int(attacker_object, commons::instance::int::TARGET_ID, defender_object_id as i32);
            }
            else {
                VarModule::set_int(attacker_object, commons::instance::int::TARGET_ID, 0);
            }
        }
        if attacker_fighter_kind == *FIGHTER_KIND_LUCINA {
            if StatusModule::status_kind(attacker_boma) == *FIGHTER_STATUS_KIND_SPECIAL_LW {
                let slow_mul;
                let frames;
                if WorkModule::is_flag(attacker_boma, FIGHTER_YU_STATUS_SPECIAL_LW_FLAG_ROMAN_MOVE) {
                    slow_mul = lucina::vl::param_special_lw::onemore_slowdown_mul;
                    frames = lucina::vl::param_special_lw::onemore_slowdown_frame;
                    SlowModule::set(defender_boma, 0, slow_mul, frames, false, *BATTLE_OBJECT_ID_INVALID as u32);
                }
                else {
                    slow_mul = lucina::vl::param_special_lw::onemore_slowdown_mul_on_hit;
                    frames = lucina::vl::param_special_lw::onemore_slowdown_frame_on_hit;
                    SlowModule::set(defender_boma, 0, slow_mul, frames, false, *BATTLE_OBJECT_ID_INVALID as u32);
                }
            }
        }
    }
    if defender_cat == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if defender_fighter_kind == *FIGHTER_KIND_RYU {
            if VarModule::is_flag(defender_object, wubor_utils::vars::ryu::instance::flag::SEC_SEN_STATE) {
                let target_x;
                let target_y;
                if attacker_cat == *BATTLE_OBJECT_CATEGORY_FIGHTER
                || attacker_cat == *BATTLE_OBJECT_CATEGORY_ENEMY {
                    VarModule::set_int(defender_object, commons::instance::int::TARGET_ID, attacker_object_id as i32);
                    target_x = PostureModule::pos_x(attacker_boma);
                    target_y = PostureModule::pos_y(attacker_boma);
                    if utility::get_category(&mut *attacker_boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
                        JostleModule::set_status(&mut *attacker_boma, false);
                    }
                }
                else if attacker_cat == *BATTLE_OBJECT_CATEGORY_WEAPON {
                    let otarget_id = WorkModule::get_int(attacker_boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
                    let oboma = sv_battle_object::module_accessor(otarget_id);
                    if utility::get_category(&mut *oboma) != *BATTLE_OBJECT_CATEGORY_FIGHTER {
                        target_x = PostureModule::pos_x(defender_boma);
                        target_y = PostureModule::pos_y(defender_boma);
                        VarModule::set_int(defender_object, commons::instance::int::TARGET_ID, 0);
                    }
                    else {
                        target_x = PostureModule::pos_x(oboma);
                        target_y = PostureModule::pos_y(oboma);
                        VarModule::set_int(defender_object, commons::instance::int::TARGET_ID, otarget_id as i32);
                        if utility::get_category(&mut *oboma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
                            JostleModule::set_status(&mut *oboma, false);
                        }
                    }
                }
                else {
                    target_x = PostureModule::pos_x(defender_boma);
                    target_y = PostureModule::pos_y(defender_boma);
                    VarModule::set_int(defender_object, commons::instance::int::TARGET_ID, 0);
                }
                VarModule::set_float(defender_object, wubor_utils::vars::ryu::instance::float::TARGET_X, target_x);
                VarModule::set_float(defender_object, wubor_utils::vars::ryu::instance::float::TARGET_Y, target_y);
                VarModule::on_flag(defender_object, wubor_utils::vars::ryu::instance::flag::SECRET_SENSATION);
            }
        }
        // else if defender_fighter_kind == *FIGHTER_KIND_KEN {
        //     if MotionModule::motion_kind(defender_boma) == hash40("special_lw_step_b")
        //     && MotionModule::frame(defender_boma) <= 8.75 {
        //         WorkModule::on_flag(defender_boma, FIGHTER_KEN_STATUS_GUARD_FLAG_V_SHIFT);
        //     }
        // }
        else if defender_fighter_kind == *FIGHTER_KIND_SHULK {
            if attacker_cat == *BATTLE_OBJECT_CATEGORY_FIGHTER
            || attacker_cat == *BATTLE_OBJECT_CATEGORY_ENEMY {
                VarModule::set_int(defender_object, commons::instance::int::TARGET_ID, attacker_object_id as i32);
            }
            else if attacker_cat == *BATTLE_OBJECT_CATEGORY_WEAPON {
                let otarget_id = WorkModule::get_int(attacker_boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
                let oboma = sv_battle_object::module_accessor(otarget_id);
                if utility::get_category(&mut *oboma) != *BATTLE_OBJECT_CATEGORY_FIGHTER {
                    VarModule::set_int(defender_object, commons::instance::int::TARGET_ID, 0)
                }
                else {
                    VarModule::set_int(defender_object, commons::instance::int::TARGET_ID, otarget_id as i32);
                }
            }
            else {
                VarModule::set_int(defender_object, commons::instance::int::TARGET_ID, 0)
            }
        }
    }
    if attacker_cat == *BATTLE_OBJECT_CATEGORY_WEAPON {
        if attacker_fighter_kind == *WEAPON_KIND_MARIO_FIREBALL {
            let oboma = sv_battle_object::module_accessor((WorkModule::get_int(attacker_boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
            WorkModule::on_flag(oboma, FIGHTER_MARIO_STATUS_SPECIAL_N_FLAG_FGC_CANCEL);
        }
    }
    original!()(fighter_manager, attacker_object_id, defender_object_id, move_type, arg5, move_type_again)
}

#[skyline::hook(offset = 0x6310a0)]
unsafe fn fighter_handle_damage_hook(object: *mut BattleObject, arg: *const u8) {
    let module_accessor = (*object).module_accessor;
    // let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    let damage_received = WorkModule::get_float(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_SUCCEED_HIT_DAMAGE);
    // let hitstun = WorkModule::get_float(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME);
    // println!("histun remaining: {}", hitstun);
    call_original!(object, arg);
    let damage_received = WorkModule::get_float(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_SUCCEED_HIT_DAMAGE) - damage_received;
    let attacker_ids = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_SUCCEED_ATTACKER_ENTRY_ID);
    // println!("attacker ids: {}", attacker_ids);
    let fighter_kind = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND);
    if fighter_kind == *FIGHTER_KIND_GAOGAEN {
        handle_revenge(module_accessor, attacker_ids);
    }
    for x in 0..8 {
        if attacker_ids & (1 << x) == 0 {
            continue;
        }
        if let Some(object_id) = get_active_battle_object_id_from_entry_id(x) {
            let object = get_battle_object_from_id(object_id);
            let module_accessor = (*object).module_accessor;
            let kind = utility::get_kind(&mut *module_accessor);
            if kind == *FIGHTER_KIND_LUCINA {
                add_sp(object, module_accessor, damage_received);
            }
            else if kind == *FIGHTER_KIND_KEN {
                add_vgauge(object, module_accessor, damage_received);
            }
            else if kind == *FIGHTER_KIND_DOLLY {
                add_go(object, module_accessor, damage_received);
            }
        }
    }
}

#[skyline::hook(replace = WorkModule::is_enable_transition_term )]
pub unsafe fn is_enable_transition_term_replace(boma: &mut BattleObjectModuleAccessor, term: i32) -> bool {
    let fighter_kind = utility::get_kind(boma);
    let ret = original!()(boma,term);
    let object_id = boma.battle_object_id;
    let object = sv_system::battle_object(object_id as u64);
    if utility::get_category(boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if fighter_kind == *FIGHTER_KIND_LUCINA { // Make this a custom command grab
            if WorkModule::is_flag(boma, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_HEROIC_GRAB)
            && term != *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT
            && term != *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_HI
            && term != *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3
            && term != *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN {
                return false;
            }
        }
        else if fighter_kind == *FIGHTER_KIND_RYU { // make secret sensation its own status
            if VarModule::is_flag(object, wubor_utils::vars::ryu::instance::flag::SEC_SEN_CAMERA) {
                return false;
            }
        }
    }
    return ret;
}

#[skyline::hook(offset = FLOAT_OFFSET)]
pub unsafe fn get_param_float_replace(module_accessor: u64, param_type: u64, param_hash: u64) -> f32 {
    let boma = &mut *(*((module_accessor as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let object_id = (*boma).battle_object_id;
    let object = sv_system::battle_object(object_id as u64);
    let ret = original!()(module_accessor, param_type, param_hash);
    let fighter_kind = utility::get_kind(boma);
    
    if utility::get_category(boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {

        if fighter_kind == *FIGHTER_KIND_KEN {
            if param_hash == hash40("speed_x_mul_s") {
                if WorkModule::get_int(boma, FIGHTER_KEN_INSTANCE_WORK_ID_INT_SHORYUREPPA) == 1 {
                    return 0.15;
                }
            }
            else if param_hash == hash40("speed_y_mul_s") {
                if WorkModule::is_flag(boma, FIGHTER_KEN_INSTANCE_WORK_ID_FLAG_V_TRIGGER)
                && WorkModule::get_int(boma, FIGHTER_KEN_INSTANCE_WORK_ID_INT_SHORYUREPPA) == 1 {
                    return 0.1;
                }
            }
        }
        else if fighter_kind == *FIGHTER_KIND_LUCARIO {
            if param_hash == 0x189cd804c5 {
                if VarModule::is_flag(object, commons::instance::flag::IS_FGC) {
                    return 1.0;
                }
            }
        }
    }
    else if utility::get_category(boma) == *BATTLE_OBJECT_CATEGORY_WEAPON {
        if fighter_kind == *WEAPON_KIND_KAMUI_RYUSENSYA {
            if param_hash == hash40("speed_max") {
                let otarget_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
                let oboma = sv_battle_object::module_accessor(otarget_id);
                if WorkModule::get_float(oboma, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_DRAGON_INSTALL) > 0.0 {
                    return 1.2;
                }
            }
            else if param_hash == hash40("life_max") {
                let otarget_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
                let oboma = sv_battle_object::module_accessor(otarget_id);
                if WorkModule::get_float(oboma, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_DRAGON_INSTALL) > 0.0 {
                    return 150.0;
                }
            }
            else if param_hash == hash40("scale_max") {
                let otarget_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
                let oboma = sv_battle_object::module_accessor(otarget_id);
                if WorkModule::get_float(oboma, FIGHTER_KAMUI_INSTANCE_WORK_ID_FLOAT_DRAGON_INSTALL) > 0.0 {
                    return 1.7;
                }
            }
        }
    }
    return ret;
}

#[skyline::hook(replace = WorkModule::get_int64 )]
pub unsafe fn get_int64_replace(boma: &mut BattleObjectModuleAccessor, term: i32) -> u64 {
    let ret = original!()(boma,term);
    if utility::get_category(boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if utility::get_kind(boma) == *FIGHTER_KIND_LUCINA
        && term == *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND {
            if WorkModule::is_flag(boma, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_HEROIC_GRAB) {
                return hash40("throw_hi");
            }
        }
    }
    return ret;
}

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

#[skyline::hook(replace = sv_animcmd::PLAY_SE)]
unsafe fn play_se_replace(lua_state: u64) {
    let boma = sv_system::battle_object_module_accessor(lua_state);
    let fighter_kind = utility::get_kind(boma);
    if fighter_kind == *FIGHTER_KIND_LUCINA
    && shadow_id(boma) {
        let mut l2c_agent = L2CAgent::new(lua_state);
        let se = l2c_agent.pop_lua_stack(1);
        l2c_agent.clear_lua_stack();
        let mut new_se = se.get_int();
        for i in 0..36 {
            if se.get_int() == hash40(&("vc_lucina_".to_owned() + YU_AUDIO[i])) {
                new_se = hash40(&("vc_shadow_".to_owned() + YU_AUDIO[i]));
                break;
            }
        }
        l2c_agent.push_lua_stack(&mut L2CValue::new_int(new_se));
    }
    original!()(lua_state);
}

#[skyline::hook(replace = sv_animcmd::PLAY_SEQUENCE)]
pub unsafe fn play_sequence_replace(lua_state: u64) -> u64 {
    let mut l2c_agent = L2CAgent::new(lua_state);
    let boma = sv_system::battle_object_module_accessor(lua_state);
    let fighter_kind = utility::get_kind(boma);
    if fighter_kind == *FIGHTER_KIND_LUCINA
    && shadow_id(boma) {
        let seq = l2c_agent.pop_lua_stack(1);
        l2c_agent.clear_lua_stack();
        let mut new_seq = seq.get_int();
        for i in 0..8 {
            if seq.get_int() == hash40(&("seq_lucina_rnd_".to_owned() + YU_SEQ[i])) {
                new_seq = hash40(&("seq_shadow_rnd_".to_owned() + YU_SEQ[i]));
                break;
            }
        }
        l2c_agent.push_lua_stack(&mut L2CValue::new_int(new_seq));
    }
    original!()(lua_state)
}

#[skyline::hook(replace = sv_animcmd::PLAY_FLY_VOICE)]
pub unsafe fn play_fly_voice_replace(lua_state: u64) -> u64 {
    let mut l2c_agent = L2CAgent::new(lua_state);
    let boma = sv_system::battle_object_module_accessor(lua_state);
    let fighter_kind = utility::get_kind(boma);
    if fighter_kind == *FIGHTER_KIND_LUCINA
    && shadow_id(boma) {
        let seq = l2c_agent.pop_lua_stack(1);
        let seq2 = l2c_agent.pop_lua_stack(1);
        l2c_agent.clear_lua_stack();
        let mut new_seq = seq.get_int();
        let mut new_seq2 = seq2.get_int();
        for i in 0..8 {
            if seq.get_int() == hash40(&("seq_lucina_rnd_".to_owned() + YU_SEQ[i])) {
                new_seq = hash40(&("seq_shadow_rnd_".to_owned() + YU_SEQ[i]));
            }
            if seq2.get_int() == hash40(&("seq_lucina_rnd_".to_owned() + YU_SEQ[i])) {
                new_seq2 = hash40(&("seq_shadow_rnd_".to_owned() + YU_SEQ[i]));
            }
        }
        l2c_agent.push_lua_stack(&mut L2CValue::new_int(new_seq2));
        l2c_agent.push_lua_stack(&mut L2CValue::new_int(new_seq));
    }
    original!()(lua_state)
}

#[skyline::hook(replace = sv_animcmd::PLAY_STATUS)]
unsafe fn play_status_replace(lua_state: u64) {
    let boma = sv_system::battle_object_module_accessor(lua_state);
    let fighter_kind = utility::get_kind(boma);
    if utility::get_category(boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER
    && fighter_kind == *FIGHTER_KIND_LUCINA
    && shadow_id(boma) {
        let mut l2c_agent = L2CAgent::new(lua_state);
        let se = l2c_agent.pop_lua_stack(1);
        l2c_agent.clear_lua_stack();
        let mut new_se = se.get_int();
        for i in 0..36 {
            if se.get_int() == hash40(&("vc_lucina_".to_owned() + YU_AUDIO[i])) {
                new_se = hash40(&("vc_shadow_".to_owned() + YU_AUDIO[i]));
                break;
            }
        }
        l2c_agent.push_lua_stack(&mut L2CValue::new_int(new_se));
    }
    original!()(lua_state);
}

#[skyline::hook(replace = sv_animcmd::PLAY_DOWN_SE)]
unsafe fn play_down_se_replace(lua_state: u64) {
    let boma = sv_system::battle_object_module_accessor(lua_state);
    let fighter_kind = utility::get_kind(boma);
    if fighter_kind == *FIGHTER_KIND_LUCINA
    && shadow_id(boma) {
        let mut l2c_agent = L2CAgent::new(lua_state);
        let se = l2c_agent.pop_lua_stack(1);
        l2c_agent.clear_lua_stack();
        let mut new_se = se.get_int();
        for i in 0..36 {
            if se.get_int() == hash40(&("vc_lucina_".to_owned() + YU_AUDIO[i])) {
                new_se = hash40(&("vc_shadow_".to_owned() + YU_AUDIO[i]));
                break;
            }
        }
        l2c_agent.push_lua_stack(&mut L2CValue::new_int(new_se));
    }
    original!()(lua_state);
}

#[skyline::hook(replace = sv_animcmd::PLAY_SE_REMAIN)]
unsafe fn play_se_remain_replace(lua_state: u64) {
    let boma = sv_system::battle_object_module_accessor(lua_state);
    let fighter_kind = utility::get_kind(boma);
    if fighter_kind == *FIGHTER_KIND_LUCINA
    && shadow_id(boma) {
        let mut l2c_agent = L2CAgent::new(lua_state);
        let se = l2c_agent.pop_lua_stack(1);
        l2c_agent.clear_lua_stack();
        let mut new_se = se.get_int();
        for i in 0..36 {
            if se.get_int() == hash40(&("vc_lucina_".to_owned() + YU_AUDIO[i])) {
                new_se = hash40(&("vc_shadow_".to_owned() + YU_AUDIO[i]));
                break;
            }
        }
        l2c_agent.push_lua_stack(&mut L2CValue::new_int(new_se));
    }
    original!()(lua_state);
}

#[skyline::hook(replace = sv_animcmd::PLAY_SE_NO_3D)]
unsafe fn play_se_no_3d_replace(lua_state: u64) {
    let boma = sv_system::battle_object_module_accessor(lua_state);
    let fighter_kind = utility::get_kind(boma);
    if fighter_kind == *FIGHTER_KIND_LUCINA
    && shadow_id(boma) {
        let mut l2c_agent = L2CAgent::new(lua_state);
        let se = l2c_agent.pop_lua_stack(1);
        l2c_agent.clear_lua_stack();
        let mut new_se = se.get_int();
        for i in 0..36 {
            if se.get_int() == hash40(&("vc_lucina_".to_owned() + YU_AUDIO[i])) {
                new_se = hash40(&("vc_shadow_".to_owned() + YU_AUDIO[i]));
                break;
            }
        }
        l2c_agent.push_lua_stack(&mut L2CValue::new_int(new_se));
    }
    original!()(lua_state);
}

// #[skyline::hook(offset = 0x696700)]
// pub unsafe extern "C" fn crit_zoom(
//     module_accessor: *mut BattleObjectModuleAccessor,
//     param_2: u64,
//     param_3: u64,
//     param_4: u64,
//     param_5: u64,
//     param_6: u64,
//     param_7: u64,
//     param_8: u64,
//     param_9: u64
// ) -> bool {
//     println!("param_2: {}", param_2);
//     println!("param_3: {}", param_3);
//     println!("param_4: {}", param_4);
//     println!("param_5: {}", param_5);
//     println!("param_6: {}", param_6);
//     println!("param_7: {}", param_7);
//     println!("param_8: {}", param_8);
//     println!("param_9: {}", param_9);
//     original!()(
//         module_accessor,
//         param_2,
//         param_3,
//         param_4,
//         param_5,
//         param_6,
//         param_7,
//         param_8,
//         param_9
//     )
// }

pub fn get_active_battle_object_id_from_entry_id(entry_id: u32) -> Option<u32> {
    use smash::lib::lua_const::*;
    use smash::app::lua_bind::*;
    let object = get_battle_object_from_entry_id(entry_id)?;
    if object.is_null() { return None; }
    let object = unsafe { &mut *object };
    let kind = object.kind as i32;
    let status = unsafe {
        StatusModule::status_kind(object.module_accessor)
    };
    if status != *FIGHTER_STATUS_KIND_NONE && status != *FIGHTER_STATUS_KIND_STANDBY {
        return Some(object.battle_object_id);
    }
    if kind == *FIGHTER_KIND_ELIGHT || kind == *FIGHTER_KIND_EFLAME {
        Some(object.battle_object_id + 0x10000)
    } else if kind == *FIGHTER_KIND_PZENIGAME || kind == *FIGHTER_KIND_PFUSHIGISOU || kind == *FIGHTER_KIND_PLIZARDON {
        let next_id = object.battle_object_id + 0x10000;
        let next_object = unsafe { &mut *get_battle_object_from_id(next_id) };
        let next_status = unsafe {
            StatusModule::status_kind(next_object.module_accessor)
        };
        if next_status != *FIGHTER_STATUS_KIND_NONE && next_status != *FIGHTER_STATUS_KIND_STANDBY {
            Some(next_id)
        } else {
            Some(next_id + 0x10000)
        }
    } else {
        Some(object.battle_object_id)
    }
}

pub fn get_battle_object_from_entry_id(entry_id: u32) -> Option<*mut BattleObject> {
    unsafe {
        let entry = get_fighter_entry(singletons::FighterManager(), entry_id);
        if entry.is_null() {
            None
        } else {
            Some(*(entry.add(0x4160) as *mut *mut BattleObject))
        }
    }
}

#[skyline::from_offset(0x3ac540)]
pub fn get_battle_object_from_id(id: u32) -> *mut BattleObject;

extern "C" {
    #[link_name = "\u{1}_ZN3app8lua_bind38FighterManager__get_fighter_entry_implEPNS_14FighterManagerENS_14FighterEntryIDE"]
    fn get_fighter_entry(manager: *mut smash::app::FighterManager, entry_id: u32) -> *mut u8;
}

pub fn install() {
    unsafe{
        let text_ptr = getRegionAddress(Region::Text) as *const u8;
        let text_size = (getRegionAddress(Region::Rodata) as usize) - (text_ptr as usize);
        let text = std::slice::from_raw_parts(text_ptr, text_size);
        if let Some(offset) = find_subsequence(text, FLOAT_SEARCH_CODE) {
            FLOAT_OFFSET = offset;
        }
        if let Some(offset) = find_subsequence(text, INT_SEARCH_CODE) {
            INT_OFFSET = offset;
        }
        if let Some(offset) = find_subsequence(text, NOTIFY_LOG_EVENT_COLLISION_HIT_SEARCH_CODE) {
            NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET = offset;
        }
    }
    skyline::install_hooks!(
        notify_log_event_collision_hit_replace,
        fighter_handle_damage_hook,
        is_enable_transition_term_replace,
        get_param_float_replace,
        get_int64_replace,
        play_se_replace,
        play_fly_voice_replace,
        play_sequence_replace,
        play_status_replace,
        play_down_se_replace,
        play_se_remain_replace,
        play_se_no_3d_replace,
        // crit_zoom,
        set_work_keep_hook
    );
}