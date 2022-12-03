use {
    smash::{
        lua2cpp::*,
        hash40,
        phx::*,
        app::lua_bind::*,
        lib::lua_const::*
    },
    custom_var::*,
    custom_cancel::*,
    wubor_utils::vars::*,
    crate::fighter::common::common_fgc::*
};

unsafe extern "C" fn elight_attackair_set_cancels(fighter: &mut L2CFighterCommon) -> bool {
    let mot = MotionModule::motion_kind(fighter.module_accessor);
    if mot == hash40("attack_air_n") {
        VarModule::on_flag(fighter.battle_object, commons::status::flag::ENABLE_AERIAL_STRING);
        VarModule::set_int(fighter.battle_object, commons::status::int::ENABLED_AERIALS, ATTACK_AIR_N_MASK);
    }
    false
}

pub unsafe extern "C" fn element_fgc(agent: Hash40) {
    CustomCancelManager::add_hp_value(agent, 188.0);
    CustomCancelManager::add_cancel_info(
        agent,
        *FIGHTER_STATUS_KIND_ATTACK,
        CancelInfo::new()
            .enable_normals([
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3
            ].to_vec())
            .enable_specials([
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI
            ].to_vec())
    );
    if agent.hash == hash40("fighter_kind_elight") {
        CustomCancelManager::add_cancel_info(
            agent,
            *FIGHTER_STATUS_KIND_ATTACK_AIR,
            CancelInfo::new()
                .enable_jump_cancel(CancelType::HIT | CancelType::BLOCK)
                .enable_aerials(CancelType::HIT | CancelType::BLOCK)
                .aerial_cancel_require_flag()
                .set_fgc_flags(FGCFlags::AIRDASH | FGCFlags::DASH | FGCFlags::JUMP | FGCFlags::NORMAL | FGCFlags::SPECIAL)
                .pre_function(elight_attackair_set_cancels)
        );
    }
    else {
        CustomCancelManager::add_cancel_info(
            agent,
            *FIGHTER_STATUS_KIND_ATTACK_AIR,
            CancelInfo::new()
                .enable_jump_cancel(CancelType::HIT | CancelType::BLOCK)
        );
    }
    CustomCancelManager::add_cancel_info(
        agent,
        *FIGHTER_STATUS_KIND_ATTACK_S3,
        CancelInfo::new()
            .enable_normals([
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START
            ].to_vec())
            .enable_specials([
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI
            ].to_vec())
            .exception_function(ftilt_dash_attack)
    );
    for x in [
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
        *FIGHTER_STATUS_KIND_ATTACK_HI3
    ].iter() {
        CustomCancelManager::add_cancel_info(
            agent,
            *x,
            CancelInfo::new()
                .enable_normals([
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START,
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START,
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START
                ].to_vec())
                .enable_specials([
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N,
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S,
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI
                ].to_vec())
        );
    }
    for x in [
        *FIGHTER_STATUS_KIND_ATTACK_S4,
        *FIGHTER_STATUS_KIND_ATTACK_LW4,
        *FIGHTER_STATUS_KIND_ATTACK_HI4,
        *FIGHTER_STATUS_KIND_ATTACK_DASH
    ].iter() {
        CustomCancelManager::add_cancel_info(
            agent,
            *x,
            CancelInfo::new()
        );
    }
    CustomCancelManager::add_cancel_info(
        agent,
        *FIGHTER_STATUS_KIND_ATTACK_100,
        CancelInfo::new()
            .special_cancel_require_flag()
    );
}
