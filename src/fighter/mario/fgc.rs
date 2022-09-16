use {
    smash::{
        phx::*,
        lib::lua_const::*
    },
    custom_cancel::*,
    crate::fighter::common::common_fgc::*
};

pub fn install() {
    let agent = Hash40::new("fighter_kind_mario");
    CustomCancelManager::initialize_agent(agent);
    CustomCancelManager::add_hp_value(agent, 195.0);
    CustomCancelManager::add_cancel_info(
        agent,
        *FIGHTER_STATUS_KIND_ATTACK,
        CancelInfo::new()
            .enable_normals([
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START
            ].to_vec())
    );
    CustomCancelManager::add_cancel_info(
        agent,
        *FIGHTER_STATUS_KIND_ATTACK_AIR,
        CancelInfo::new()
            .enable_aerials(CancelType::HIT | CancelType::BLOCK)
            .aerial_cancel_require_flag()
            .enable_jump_cancel(CancelType::HIT)
            .jump_cancel_require_flag()
    );
    for x in [
        *FIGHTER_STATUS_KIND_ATTACK_S3,
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        *FIGHTER_STATUS_KIND_ATTACK_DASH
    ].iter() {
        let mut info = CancelInfo::new()
        .enable_normals([
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START
        ].to_vec());
        if *x == *FIGHTER_STATUS_KIND_ATTACK_S3 {
            info = info.exception_function(ftilt_dash_attack);
        }
        CustomCancelManager::add_cancel_info(
            agent,
            *x,
            info
        );
    }
    for x in [
        *FIGHTER_STATUS_KIND_ATTACK_S4,
        *FIGHTER_STATUS_KIND_ATTACK_LW4,
        *FIGHTER_STATUS_KIND_ATTACK_HI4
    ].iter() {
        let mut info = CancelInfo::new()
        .enable_normals([
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START
        ].to_vec());
        if *x == *FIGHTER_STATUS_KIND_ATTACK_HI4 {
            info = info.enable_jump_cancel(CancelType::HIT);
        }
        CustomCancelManager::add_cancel_info(
            agent,
            *x,
            info
        );
    }
}
