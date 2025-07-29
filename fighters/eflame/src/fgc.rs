use {
    smash::{
        phx::*,
        lib::lua_const::*
    },
    custom_cancel::*
};

pub fn install() {
    let agent = Hash40::new("fighter_kind_eflame");
    CustomCancelManager::initialize_agent(agent);
    CustomCancelManager::add_cancel_info(
        agent,
        *FIGHTER_STATUS_KIND_ATTACK_AIR,
        CancelInfo::new()
            .enable_airdash_cancel(CancelType::HIT)
            .airdash_cancel_require_flag()
    );
}
