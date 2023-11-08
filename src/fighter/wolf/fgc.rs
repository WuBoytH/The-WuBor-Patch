use {
    smash::{
        phx::*,
        lib::lua_const::*
    },
    custom_cancel::*
};

pub fn install() {
    let agent = Hash40::new("fighter_kind_wolf");
    CustomCancelManager::initialize_agent(agent);
    CustomCancelManager::add_cancel_info(
        agent,
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        CancelInfo::new()
            .enable_normals([
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START
            ].to_vec())
            .enable_jump_cancel(CancelType::HIT)
    );
}
