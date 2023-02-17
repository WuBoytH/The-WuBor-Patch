use {
    smash::{
        phx::*,
        lib::lua_const::*
    },
    custom_cancel::*,
    crate::fighter::common::fgc::*
};

pub fn install() {
    let agent = Hash40::new("fighter_kind_inkling");
    CustomCancelManager::add_cancel_info(
        agent,
        *FIGHTER_STATUS_KIND_ATTACK_AIR,
        CancelInfo::new()
            .enable_airdash_cancel(CancelType::HIT)
            .airdash_cancel_require_flag()
            .set_fgc_flags(FGCFlags::ALL - FGCFlags::AIRDASH)
    );
    generic_attack(agent);
    generic_attack3(agent);
    generic_attack4(agent);
}
