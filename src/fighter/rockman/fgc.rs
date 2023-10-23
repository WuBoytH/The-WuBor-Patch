use {
    smash::{
        phx::*,
        lib::lua_const::*
    },
    custom_cancel::*
};

pub fn install() {
    let agent = Hash40::new("fighter_kind_rockman");
    CustomCancelManager::initialize_agent(agent);
    CustomCancelManager::add_cancel_info(
        agent,
        *FIGHTER_STATUS_KIND_ATTACK,
        CancelInfo::new()
            .enable_jump_cancel(CancelType::HIT)
            .jump_cancel_require_flag()
    );
}
