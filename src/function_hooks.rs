use {
    smash::{
        hash40,
        app::{lua_bind::*, FighterManager, *},
        lib::{lua_const::*, L2CValue, L2CAgent}
    },
    crate::{
        fighter::{
            lucina::helper::shadow_id,
            ken::helper::add_vgauge,
            *
        }
    },
    std::ffi::CStr,
    wubor_utils::{
        wua_bind::*,
        vars::*
    },
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
    let defender_boma = sv_battle_object::module_accessor(defender_object_id);
    let attacker_fighter_kind = sv_battle_object::kind(attacker_object_id);
    let defender_fighter_kind = sv_battle_object::kind(defender_object_id);
    let a_entry_id = WorkModule::get_int(attacker_boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let attacker_cat = utility::get_category(&mut *attacker_boma);
    let defender_cat = utility::get_category(&mut *defender_boma);
    if attacker_cat == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if attacker_fighter_kind == *FIGHTER_KIND_KEN {
            if defender_cat == *BATTLE_OBJECT_CATEGORY_FIGHTER {
                WorkModule::set_int64(attacker_boma, defender_object_id as i64, FIGHTER_INSTANCE_WORK_ID_INT_TARGET_ID);
            }
            else {
                WorkModule::set_int64(attacker_boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_TARGET_ID);
            }
            if MotionModule::motion_kind(attacker_boma) != hash40("special_lw")
            && !WorkModule::is_flag(attacker_boma, FIGHTER_KEN_INSTANCE_WORK_ID_FLAG_V_TRIGGER) {
                if MotionModule::motion_kind(attacker_boma) == hash40("attack_s3_s_w")
                && StatusModule::status_kind(attacker_boma) == *FIGHTER_STATUS_KIND_SPECIAL_LW {
                    add_vgauge(attacker_boma, 100.0);
                }
                else {
                    let amount = AttackModule::get_power(attacker_boma, 0, false, 1.0, false) * 5.0;
                    add_vgauge(attacker_boma, amount);
                }
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
        if a_entry_id < 8
        && WorkModule::is_flag(attacker_boma, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HITSTUN) {
            WorkModule::on_flag(defender_boma, FIGHTER_INSTANCE_WORK_ID_FLAG_HIT_BY_SPECIAL_HITSTUN);
        }
        if defender_fighter_kind == *FIGHTER_KIND_RYU {
            if WorkModule::is_flag(defender_boma, FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_SEC_SEN_STATE) {
                let target_x;
                let target_y;
                if attacker_cat == *BATTLE_OBJECT_CATEGORY_FIGHTER
                || attacker_cat == *BATTLE_OBJECT_CATEGORY_ENEMY {
                    WorkModule::set_int64(defender_boma, attacker_object_id as i64, FIGHTER_INSTANCE_WORK_ID_INT_TARGET_ID);
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
                        WorkModule::set_int64(defender_boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_TARGET_ID);
                    }
                    else {
                        target_x = PostureModule::pos_x(oboma);
                        target_y = PostureModule::pos_y(oboma);
                        WorkModule::set_int64(defender_boma, otarget_id as i64, FIGHTER_INSTANCE_WORK_ID_INT_TARGET_ID);
                        if utility::get_category(&mut *oboma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
                            JostleModule::set_status(&mut *oboma, false);
                        }
                    }
                }
                else {
                    target_x = PostureModule::pos_x(defender_boma);
                    target_y = PostureModule::pos_y(defender_boma);
                    WorkModule::set_int64(defender_boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_TARGET_ID);
                }
                WorkModule::set_float(defender_boma, target_x, FIGHTER_RYU_INSTANCE_WORK_ID_FLOAT_TARGET_X);
                WorkModule::set_float(defender_boma, target_y, FIGHTER_RYU_INSTANCE_WORK_ID_FLOAT_TARGET_Y);
                WorkModule::on_flag(defender_boma, FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_SECRET_SENSATION);
            }
        }
        // else if defender_fighter_kind == *FIGHTER_KIND_KEN {
        //     if MotionModule::motion_kind(defender_boma) == hash40("special_lw_step_b")
        //     && MotionModule::frame(defender_boma) <= 8.75 {
        //         WorkModule::on_flag(defender_boma, FIGHTER_KEN_STATUS_GUARD_FLAG_V_SHIFT);
        //     }
        // }
        else if defender_fighter_kind == *FIGHTER_KIND_GAOGAEN {
            if (MotionModule::motion_kind(defender_boma) == hash40("special_lw_start")
            || MotionModule::motion_kind(defender_boma) == hash40("special_air_lw_start"))
            && WorkModule::get_int(defender_boma, FIGHTER_GAOGAEN_INSTANCE_WORK_ID_INT_REVENGE) == 1 {
                WorkModule::set_int(defender_boma, 2, FIGHTER_GAOGAEN_INSTANCE_WORK_ID_INT_REVENGE);
                if PostureModule::pos_x(defender_boma) < PostureModule::pos_x(attacker_boma)
                && PostureModule::lr(defender_boma) == 1.0 {
                    PostureModule::reverse_lr(defender_boma);
                }
                else if PostureModule::pos_x(defender_boma) > PostureModule::pos_x(attacker_boma)
                && PostureModule::lr(defender_boma) == -1.0 {
                    PostureModule::reverse_lr(defender_boma);
                }
            }
        }
        else if defender_fighter_kind == *FIGHTER_KIND_SHULK {
            if attacker_cat == *BATTLE_OBJECT_CATEGORY_FIGHTER
            || attacker_cat == *BATTLE_OBJECT_CATEGORY_ENEMY {
                WorkModule::set_int64(defender_boma, attacker_object_id as i64, FIGHTER_INSTANCE_WORK_ID_INT_TARGET_ID);
            }
            else if attacker_cat == *BATTLE_OBJECT_CATEGORY_WEAPON {
                let otarget_id = WorkModule::get_int(attacker_boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
                let oboma = sv_battle_object::module_accessor(otarget_id);
                if utility::get_category(&mut *oboma) != *BATTLE_OBJECT_CATEGORY_FIGHTER {
                    WorkModule::set_int64(defender_boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_TARGET_ID);
                }
                else {
                    WorkModule::set_int64(defender_boma, otarget_id as i64, FIGHTER_INSTANCE_WORK_ID_INT_TARGET_ID);
                }
            }
            else {
                WorkModule::set_int64(defender_boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_TARGET_ID);
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

#[skyline::hook(replace = WorkModule::is_enable_transition_term )]
pub unsafe fn is_enable_transition_term_replace(boma: &mut BattleObjectModuleAccessor, term: i32) -> bool {
    let fighter_kind = utility::get_kind(boma);
    let ret = original!()(boma,term);
    
    if utility::get_category(boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {

        if [
            *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_FB,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_CEIL,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL_JUMP
        ].contains(&term) {
            if WorkModule::is_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FGC) {
                if !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_GROUND)
                && !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_AIR)
                && !WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR)
                && StatusModule::status_kind(boma) != *FIGHTER_STATUS_KIND_ESCAPE_AIR {
                    return false;
                }
            }
        }

        else if [
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_AIR_LASSO,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_JUMP_START,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON
        ].contains(&term) {
            if WorkModule::is_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FGC) {
                if MiscModule::is_damage_check(boma, false) {
                    return false;
                }
            }
        }

        if fighter_kind == *FIGHTER_KIND_LUCINA {
            if WorkModule::is_flag(boma, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_HEROIC_GRAB)
            && term != *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT
            && term != *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_HI
            && term != *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3
            && term != *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN {
                return false;
            }
        }
        else if fighter_kind == *FIGHTER_KIND_RYU {
            if WorkModule::is_flag(boma, FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_SEC_SEN_CAMERA) {
                return false;
            }
        }
    }
    return ret;
}

#[skyline::hook(offset = FLOAT_OFFSET)]
pub unsafe fn get_param_float_replace(module_accessor: u64, param_type: u64, param_hash: u64) -> f32 {
    let boma = &mut *(*((module_accessor as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
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
        else if fighter_kind == *FIGHTER_KIND_WARIO {
            if param_hash == hash40("gass_middle_time") {
                if WorkModule::get_int(boma, FIGHTER_WARIO_INSTANCE_WORK_ID_INT_FINISH_SIGN) > 0 {
                    return 1.0;
                }
            }
            else if param_hash == hash40("gass_large_time") {
                if WorkModule::get_int(boma, FIGHTER_WARIO_INSTANCE_WORK_ID_INT_FINISH_SIGN) > 6 {
                    return 1.0;
                }
            }
            else if param_hash == hash40("gass_max_time") {
                if WorkModule::get_int(boma, FIGHTER_WARIO_INSTANCE_WORK_ID_INT_FINISH_SIGN) >= 14 {
                    return 1.0;
                }
            }
        }
        else if fighter_kind == *FIGHTER_KIND_LUCARIO {
            if param_hash == 0x189cd804c5 {
                if WorkModule::is_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FGC) {
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

#[skyline::hook(replace = WorkModule::set_float )]
pub unsafe fn set_float_replace(boma: &mut BattleObjectModuleAccessor, mut val: f32, term: i32) {
    if utility::get_category(boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if StatusModule::status_kind(boma) - 0x48 < 7
        && term == *FIGHTER_STATUS_DAMAGE_WORK_FLOAT_REACTION_FRAME {
            if WorkModule::is_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FGC)
            && !WorkModule::is_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_HIT_BY_SPECIAL_HITSTUN) {
                val = val * WorkModule::get_float(boma, FIGHTER_INSTANCE_WORK_ID_FLOAT_FGC_HITSTUN_MUL);
                if WorkModule::get_float(boma, FIGHTER_INSTANCE_WORK_ID_FLOAT_FGC_HITSTUN_MUL) > 0.5 {
                    WarkModule::add_f32(boma, FIGHTER_INSTANCE_WORK_ID_FLOAT_FGC_HITSTUN_MUL, -0.05);
                }
                WorkModule::off_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_HIT_BY_SPECIAL_HITSTUN);
            }
        }
    }
    original!()(boma, val, term);
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

#[skyline::hook(offset = DEFINE_LUA_CONSTANT_OFFSET)]
unsafe fn declare_const_hook(unk: u64, constant: *const u8, mut value: u32) {
    let str = CStr::from_ptr(constant as _).to_str().unwrap();
    if str.contains("FIGHTER_MARTH_STATUS_KIND_NUM") {
        value += 0xD;
    }
    original!()(unk, constant, value)
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
        is_enable_transition_term_replace,
        get_param_float_replace,
        set_float_replace,
        get_int64_replace,
        play_se_replace,
        play_fly_voice_replace,
        play_sequence_replace,
        play_status_replace,
        play_down_se_replace,
        play_se_remain_replace,
        play_se_no_3d_replace,
        // crit_zoom,
        declare_const_hook
    );
}