// Preserved to remind myself I made reverse beat in Smash

use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        phx::*,
        app::lua_bind::*,
        lib::lua_const::*
    },
    custom_var::*,
    custom_cancel::*,
    wubor_utils::{wua_bind::*, vars::*}
};

unsafe extern "C" fn miifighter_attack_pre(fighter: &mut L2CFighterCommon) -> bool {
    let status = StatusModule::status_kind(fighter.module_accessor);
    if status == *FIGHTER_STATUS_KIND_ATTACK
    || status == *FIGHTER_STATUS_KIND_ITEM_SWING {
        FGCModule::disable_ground_normal(fighter, ATTACK_N_MASK);
    }
    else if status == *FIGHTER_STATUS_KIND_ATTACK_S3
    || status == *FIGHTER_STATUS_KIND_ITEM_SWING_S3 {
        FGCModule::disable_ground_normal(fighter, ATTACK_S3_MASK);
    }
    else if status == *FIGHTER_STATUS_KIND_ATTACK_HI3 {
        FGCModule::disable_ground_normal(fighter, ATTACK_HI3_MASK);
    }
    else if status == *FIGHTER_STATUS_KIND_ATTACK_LW3 {
        FGCModule::disable_ground_normal(fighter, ATTACK_LW3_MASK);
    }
    else if status == *FIGHTER_STATUS_KIND_ATTACK_S4_START
    || status == *FIGHTER_STATUS_KIND_ATTACK_S4
    || status == *FIGHTER_STATUS_KIND_ITEM_SWING_S4_START {
        FGCModule::disable_ground_normal(fighter, ATTACK_S4_MASK);
    }
    else if status == *FIGHTER_STATUS_KIND_ATTACK_HI4_START
    || status == *FIGHTER_STATUS_KIND_ATTACK_HI4 {
        FGCModule::disable_ground_normal(fighter, ATTACK_HI4_MASK);
    }
    else if status == *FIGHTER_STATUS_KIND_ATTACK_LW4_START
    || status == *FIGHTER_STATUS_KIND_ATTACK_LW4 {
        FGCModule::disable_ground_normal(fighter, ATTACK_LW4_MASK);
    }
    false
}

unsafe extern "C" fn miifighter_attackair_pre(fighter: &mut L2CFighterCommon) -> bool {
    let mot = MotionModule::motion_kind(fighter.module_accessor);
    let flags = ATTACK_AIR_N_MASK + ATTACK_AIR_F_MASK + ATTACK_AIR_B_MASK + ATTACK_AIR_HI_MASK + ATTACK_AIR_LW_MASK;
    VarModule::on_flag(fighter.battle_object, fighter::status::flag::ENABLE_AERIAL_STRING);
    if mot == hash40("attack_air_n") {
        FGCModule::disable_aerial(fighter, ATTACK_AIR_N_MASK);
    }
    else if mot == hash40("attack_air_f") {
        FGCModule::disable_aerial(fighter, ATTACK_AIR_F_MASK);
    }
    else if mot == hash40("attack_air_b") {
        FGCModule::disable_aerial(fighter, ATTACK_AIR_B_MASK);
    }
    else if mot == hash40("attack_air_hi") {
        FGCModule::disable_aerial(fighter, ATTACK_AIR_HI_MASK);
    }
    else {
        FGCModule::disable_aerial(fighter, ATTACK_AIR_LW_MASK);
    }
    VarModule::set_int(fighter.battle_object, fighter::status::int::ENABLED_AERIALS, flags);
    false
}

pub fn install() {
    let agent = Hash40::new("fighter_kind_miifighter");
    for x in [
        *FIGHTER_STATUS_KIND_ATTACK,
        *FIGHTER_STATUS_KIND_ATTACK_S3,
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
        *FIGHTER_STATUS_KIND_ATTACK_S4,
        *FIGHTER_STATUS_KIND_ATTACK_HI4,
        *FIGHTER_STATUS_KIND_ATTACK_LW4
    ].iter() {
        let mut info = CancelInfo::new()
            .enable_normals([
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START
            ].to_vec())
            .pre_function(miifighter_attack_pre);
        if *x == *FIGHTER_STATUS_KIND_ATTACK_HI4 {
            info = info.enable_jump_cancel(CancelType::HIT);
        }
        CustomCancelManager::add_cancel_info(
            agent,
            *x,
            info
        );
    }
    CustomCancelManager::add_cancel_info(
        agent,
        *FIGHTER_STATUS_KIND_ATTACK_AIR,
        CancelInfo::new()
            .enable_aerials(CancelType::HIT | CancelType::BLOCK)
            .pre_function(miifighter_attackair_pre)
    );
}
