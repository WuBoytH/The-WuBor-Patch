use {
    smash::{
        phx::*,
        lib::lua_const::*
    },
    custom_cancel::*,
    crate::fighter::common::fgc::*
};

pub fn install() {
    let agent = Hash40::new("fighter_kind_richter");
    CustomCancelManager::add_hp_value(agent, 198.0);
    CustomCancelManager::add_cancel_info(
        agent,
        *FIGHTER_STATUS_KIND_ATTACK,
        CancelInfo::new()
            .enable_normals([
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3
            ].to_vec())
    );
    CustomCancelManager::add_cancel_info(
        agent,
        *FIGHTER_STATUS_KIND_ATTACK_AIR,
        CancelInfo::new()
    );
    CustomCancelManager::add_cancel_info(
        agent,
        *FIGHTER_STATUS_KIND_ATTACK_100,
        CancelInfo::new()
            .enable_normals([
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3
            ].to_vec())
            .normal_cancel_require_flag()
            .special_cancel_require_flag()
    );
    for x in [
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        *FIGHTER_STATUS_KIND_ATTACK_DASH
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
            .exception_function(ftilt_dash_attack)
    );
    for x in [
        *FIGHTER_STATUS_KIND_ATTACK_S4,
        *FIGHTER_STATUS_KIND_ATTACK_LW4
    ].iter() {
        CustomCancelManager::add_cancel_info(
            agent,
            *x,
            CancelInfo::new()
        );
    }
    CustomCancelManager::add_cancel_info(
        agent,
        *FIGHTER_STATUS_KIND_ATTACK_HI4,
        CancelInfo::new()
            .enable_jump_cancel(CancelType::HIT)
    );
}
