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
            ken::helper::*,
            lucina::helper::*,
            *
        }
    },
    custom_var::*,
    wubor_utils::{wua_bind::*, vars},
    skyline::hooks::{
        getRegionAddress,
        Region
    }
};

#[skyline::hook(offset = vars::NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET)]
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
                VarModule::set_int(attacker_object, vars::commons::instance::int::TARGET_ID, defender_object_id as i32);
            }
            else {
                VarModule::set_int(attacker_object, vars::commons::instance::int::TARGET_ID, 0);
            }
        }
        if attacker_fighter_kind == *FIGHTER_KIND_LUCINA {
            if StatusModule::status_kind(attacker_boma) == *FIGHTER_STATUS_KIND_SPECIAL_LW {
                let slow_mul;
                let frames;
                if VarModule::is_flag(attacker_object, vars::yu::status::flag::SPECIAL_LW_ROMAN_MOVE) {
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
            if VarModule::is_flag(defender_object, vars::ryu::instance::flag::SEC_SEN_STATE) {
                let target_x;
                let target_y;
                if attacker_cat == *BATTLE_OBJECT_CATEGORY_FIGHTER
                || attacker_cat == *BATTLE_OBJECT_CATEGORY_ENEMY {
                    VarModule::set_int(defender_object, vars::commons::instance::int::TARGET_ID, attacker_object_id as i32);
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
                        VarModule::set_int(defender_object, vars::commons::instance::int::TARGET_ID, 0);
                    }
                    else {
                        target_x = PostureModule::pos_x(oboma);
                        target_y = PostureModule::pos_y(oboma);
                        VarModule::set_int(defender_object, vars::commons::instance::int::TARGET_ID, otarget_id as i32);
                        if utility::get_category(&mut *oboma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
                            JostleModule::set_status(&mut *oboma, false);
                        }
                    }
                }
                else {
                    target_x = PostureModule::pos_x(defender_boma);
                    target_y = PostureModule::pos_y(defender_boma);
                    VarModule::set_int(defender_object, vars::commons::instance::int::TARGET_ID, 0);
                }
                VarModule::set_float(defender_object, vars::ryu::instance::float::TARGET_X, target_x);
                VarModule::set_float(defender_object, vars::ryu::instance::float::TARGET_Y, target_y);
                VarModule::on_flag(defender_object, vars::ryu::instance::flag::SECRET_SENSATION);
            }
        }
        else if defender_fighter_kind == *FIGHTER_KIND_SHULK {
            if attacker_cat == *BATTLE_OBJECT_CATEGORY_FIGHTER
            || attacker_cat == *BATTLE_OBJECT_CATEGORY_ENEMY {
                VarModule::set_int(defender_object, vars::commons::instance::int::TARGET_ID, attacker_object_id as i32);
            }
            else if attacker_cat == *BATTLE_OBJECT_CATEGORY_WEAPON {
                let otarget_id = WorkModule::get_int(attacker_boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
                let oboma = sv_battle_object::module_accessor(otarget_id);
                if utility::get_category(&mut *oboma) != *BATTLE_OBJECT_CATEGORY_FIGHTER {
                    VarModule::set_int(defender_object, vars::commons::instance::int::TARGET_ID, 0)
                }
                else {
                    VarModule::set_int(defender_object, vars::commons::instance::int::TARGET_ID, otarget_id as i32);
                }
            }
            else {
                VarModule::set_int(defender_object, vars::commons::instance::int::TARGET_ID, 0)
            }
        }
    }
    if attacker_cat == *BATTLE_OBJECT_CATEGORY_WEAPON {
        if attacker_fighter_kind == *WEAPON_KIND_MARIO_FIREBALL {
            let object = MiscModule::get_battle_object_from_id((WorkModule::get_int(attacker_boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID)) as u32);
            VarModule::on_flag(object, vars::mario::status::flag::SPECIAL_N_FGC_CANCEL);
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
        handle_revenge(object, module_accessor, attacker_ids);
    }
    for x in 0..8 {
        if attacker_ids & (1 << x) == 0 {
            continue;
        }
        if let Some(object_id) = MiscModule::get_active_battle_object_id_from_entry_id(x) {
            let object = MiscModule::get_battle_object_from_id(object_id);
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
    let kind = utility::get_kind(boma);
    let ret = original!()(boma,term);
    let object_id = boma.battle_object_id;
    let object = MiscModule::get_battle_object_from_id(object_id);
    if utility::get_category(boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if kind == *FIGHTER_KIND_LUCINA { // Make this a custom command grab
            if VarModule::is_flag(object, vars::yu::instance::flag::HEROIC_GRAB)
            && term != *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT
            && term != *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_HI
            && term != *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3
            && term != *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN {
                return false;
            }
        }
        else if kind == *FIGHTER_KIND_RYU { // make secret sensation its own status
            if VarModule::is_flag(object, vars::ryu::instance::flag::SEC_SEN_CAMERA) {
                return false;
            }
        }
    }
    ret
}

#[skyline::hook(offset = vars::FLOAT_OFFSET)]
pub unsafe fn get_param_float_replace(module: u64, param_type: u64, param_hash: u64) -> f32 {
    let ret = original!()(module, param_type, param_hash);
    let module_accessor = &mut *(*((module as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let category = utility::get_category(&mut *module_accessor);
    let kind = utility::get_kind(&mut *module_accessor);
    if category == *BATTLE_OBJECT_CATEGORY_WEAPON {
        if kind == *WEAPON_KIND_KAMUI_RYUSENSYA {
            let otarget_id = WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
            if sv_battle_object::is_active(otarget_id)
            && sv_battle_object::kind(otarget_id) == *FIGHTER_KIND_KAMUI {
                let object = MiscModule::get_battle_object_from_id(otarget_id);
                if param_hash == hash40("speed_max") {
                    if VarModule::get_float(object, vars::kamui::instance::float::DRAGON_INSTALL) > 0.0 {
                        return 1.2;
                    }
                }
                else if param_hash == hash40("life_max") {
                    if VarModule::get_float(object, vars::kamui::instance::float::DRAGON_INSTALL) > 0.0 {
                        return 150.0;
                    }
                }
                else if param_hash == hash40("scale_max") {
                    if VarModule::get_float(object, vars::kamui::instance::float::DRAGON_INSTALL) > 0.0 {
                        return 1.7;
                    }
                }
            }
        }
    }
    ret
}

#[skyline::hook(replace = WorkModule::get_int64 )]
pub unsafe fn get_int64_replace(boma: &mut BattleObjectModuleAccessor, term: i32) -> u64 {
    let ret = original!()(boma,term);
    if utility::get_category(boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if utility::get_kind(boma) == *FIGHTER_KIND_LUCINA
        && term == *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND {
            let object_id = boma.battle_object_id;
            let object = MiscModule::get_battle_object_from_id(object_id);
            if VarModule::is_flag(object, vars::yu::instance::flag::HEROIC_GRAB) {
                return hash40("throw_hi");
            }
        }
    }
    ret
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
            if se.get_int() == hash40(&("vc_lucina_".to_owned() + vars::yu::YU_AUDIO[i])) {
                new_se = hash40(&("vc_shadow_".to_owned() + vars::yu::YU_AUDIO[i]));
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
            if seq.get_int() == hash40(&("seq_lucina_rnd_".to_owned() + vars::yu::YU_SEQ[i])) {
                new_seq = hash40(&("seq_shadow_rnd_".to_owned() + vars::yu::YU_SEQ[i]));
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
            if seq.get_int() == hash40(&("seq_lucina_rnd_".to_owned() + vars::yu::YU_SEQ[i])) {
                new_seq = hash40(&("seq_shadow_rnd_".to_owned() + vars::yu::YU_SEQ[i]));
            }
            if seq2.get_int() == hash40(&("seq_lucina_rnd_".to_owned() + vars::yu::YU_SEQ[i])) {
                new_seq2 = hash40(&("seq_shadow_rnd_".to_owned() + vars::yu::YU_SEQ[i]));
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
            if se.get_int() == hash40(&("vc_lucina_".to_owned() + vars::yu::YU_AUDIO[i])) {
                new_se = hash40(&("vc_shadow_".to_owned() + vars::yu::YU_AUDIO[i]));
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
            if se.get_int() == hash40(&("vc_lucina_".to_owned() + vars::yu::YU_AUDIO[i])) {
                new_se = hash40(&("vc_shadow_".to_owned() + vars::yu::YU_AUDIO[i]));
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
            if se.get_int() == hash40(&("vc_lucina_".to_owned() + vars::yu::YU_AUDIO[i])) {
                new_se = hash40(&("vc_shadow_".to_owned() + vars::yu::YU_AUDIO[i]));
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
            if se.get_int() == hash40(&("vc_lucina_".to_owned() + vars::yu::YU_AUDIO[i])) {
                new_se = hash40(&("vc_shadow_".to_owned() + vars::yu::YU_AUDIO[i]));
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

#[allow(non_snake_case)]
pub mod LinkEventThrow {
    use super::*;
    extern "C" {
        #[link_name = "\u{1}_ZN3app14LinkEventThrow13new_l2c_tableEv"]
        pub fn new_l2c_table() -> L2CValue;
    }
}

// #[allow(non_snake_case)]
// extern "C" {
//     #[link_name = "\u{1}_ZN3app8lua_bind52GroundModule__get_touch_normal_consider_gravity_implEPNS_26BattleObjectModuleAccessorEj"]
//     pub fn get_touch_normal_consider_gravity_2(
//         module_accessor: *mut BattleObjectModuleAccessor,
//         arg2: u32,
//     ) -> smash_rs::phx::Vector4f;
// }

#[skyline::hook( replace = StatusModule::init_settings )]
pub unsafe fn init_settings_replace(
    module_accessor: *mut BattleObjectModuleAccessor,
    situation: SituationKind,
    kinetic: i32,
    correct: u32,
    cliff_check: GroundCliffCheckKind,
    jostle: bool,
    keep_flag: i32,
    keep_int: i32,
    keep_float: i32,
    arg10: i32,
) {
    let mut mask = 0;
    if keep_flag != *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG {
        mask += VarModule::RESET_STATUS_FLAG;
    }
    if keep_int != *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT {
        mask += VarModule::RESET_STATUS_INT;
        mask += VarModule::RESET_STATUS_INT64;
    }
    if keep_float != *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT {
        mask += VarModule::RESET_STATUS_FLOAT;
    }
    let object_id = (*module_accessor).battle_object_id;
    let object = MiscModule::get_battle_object_from_id(object_id);
    VarModule::reset(object, mask);
    original!()(
        module_accessor,
        situation,
        kinetic,
        correct,
        cliff_check,
        jostle,
        keep_flag,
        keep_int,
        keep_float,
        arg10
    )
}

// #[skyline::from_offset(0x3f0830)]
// pub fn some_catch(catchmodule: *mut smash_rs::app::Module);

// #[skyline::hook(offset = 0x3a6650)]
// unsafe fn get_article_use_type_mask(weapon_kind: i32, entry_id: i32) -> u8 {
//     1
// }

pub fn install() {
    unsafe{
        let text_ptr = getRegionAddress(Region::Text) as *const u8;
        let text_size = (getRegionAddress(Region::Rodata) as usize) - (text_ptr as usize);
        let text = std::slice::from_raw_parts(text_ptr, text_size);
        if let Some(offset) = find_subsequence(text, vars::FLOAT_SEARCH_CODE) {
            vars::FLOAT_OFFSET = offset;
        }
        if let Some(offset) = find_subsequence(text, vars::INT_SEARCH_CODE) {
            vars::INT_OFFSET = offset;
        }
        if let Some(offset) = find_subsequence(text, vars::NOTIFY_LOG_EVENT_COLLISION_HIT_SEARCH_CODE) {
            vars::NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET = offset;
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
        init_settings_replace
    );
    // Removes Phantom Hits
    skyline::patching::Patch::in_text(0x3e6ce8).data(0x14000012u32);
}