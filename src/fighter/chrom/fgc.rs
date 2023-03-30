use {
    smash::{
        phx::*,
        lib::lua_const::*
    },
    custom_cancel::*,
};

pub fn install() {
    let agent = Hash40::new("fighter_kind_chrom");
    for x in [
        *FIGHTER_STATUS_KIND_ATTACK_S3,
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
    ].iter() {
        CustomCancelManager::add_cancel_info(
            agent,
            *x,
            CancelInfo::new()
                .enable_normal_cancel(CancelType::HIT | CancelType::BLOCK)
                .enable_normals([
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START,
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START,
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START
                ].to_vec())
        );
    }
    CustomCancelManager::add_cancel_info(
        agent,
        *FIGHTER_STATUS_KIND_ATTACK_AIR,
        CancelInfo::new()
            .enable_special_cancel(CancelType::HIT | CancelType::BLOCK)
            .enable_specials([].to_vec())
            .enable_jump_cancel(CancelType::HIT)
            .jump_cancel_require_flag()
    );
}
