use {
    smash::{
        phx::*,
        lib::lua_const::*
    },
    custom_cancel::*
};

pub unsafe extern "C" fn install() {
    let agent = Hash40::new("fighter_kind_lucario");
    CustomCancelManager::add_cancel_info(
        agent,
        *FIGHTER_STATUS_KIND_ATTACK,
        CancelInfo::new()
            .enable_jump_cancel(CancelType::HIT)
            .jump_cancel_require_flag()
    );
}
