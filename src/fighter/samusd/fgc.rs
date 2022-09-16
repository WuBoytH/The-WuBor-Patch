use {
    smash::{
        phx::*,
        lib::lua_const::*
    },
    custom_cancel::*
};

pub fn install() {
    let agent = Hash40::new("fighter_kind_samusd");
    CustomCancelManager::add_hp_value(agent, 190.0);
    for x in [
        *FIGHTER_STATUS_KIND_ATTACK,
        *FIGHTER_STATUS_KIND_ATTACK_DASH
    ].iter() {
        CustomCancelManager::add_cancel_info(
            agent,
            *x,
            CancelInfo::new()
                .enable_normals([
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3,
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3,
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3
                ].to_vec())
        );
    }
    for x in [
        *FIGHTER_STATUS_KIND_ATTACK_S3,
        *FIGHTER_STATUS_KIND_ATTACK_LW3
    ].iter() {
        CustomCancelManager::add_cancel_info(
            agent,
            *x,
            CancelInfo::new()
                .enable_normals([
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START
                ].to_vec())
        );
    }
    CustomCancelManager::add_cancel_info(
        agent,
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        CancelInfo::new()
            .enable_jump_cancel(CancelType::HIT)
            .enable_normals([
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START
            ].to_vec())
    );
    CustomCancelManager::add_cancel_info(
        agent,
        *FIGHTER_STATUS_KIND_ATTACK_LW4,
        CancelInfo::new()
            .enable_specials([].to_vec())
            .enable_jump_cancel(CancelType::HIT)
    );
    CustomCancelManager::add_cancel_info(
        agent,
        *FIGHTER_STATUS_KIND_ATTACK_AIR,
        CancelInfo::new()
            .enable_jump_cancel(CancelType::HIT)
            .jump_cancel_require_flag()
    );
    CustomCancelManager::add_cancel_info(
        agent,
        *FIGHTER_STATUS_KIND_ATTACK_S4,
        CancelInfo::new()
            .enable_dash_cancel(CancelType::HIT)
            .set_dash_cancel_direction(DashCancelDir::FORWARD)
    );
    CustomCancelManager::add_cancel_info(
        agent,
        *FIGHTER_STATUS_KIND_ATTACK_HI4,
        CancelInfo::new()
            .enable_dash_cancel(CancelType::HIT)
            .set_dash_cancel_direction(DashCancelDir::BACKWARD)
    );
}
