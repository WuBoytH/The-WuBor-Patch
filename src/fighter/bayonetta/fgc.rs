use {
    smash::{
        lua2cpp::L2CFighterCommon,
        phx::*,
        app::lua_bind::*,
        lib::lua_const::*
    },
    custom_cancel::*
};

pub unsafe extern "C" fn disable_during_bullet_arts(fighter: &mut L2CFighterCommon) -> bool {
    WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION)
}

pub unsafe extern "C" fn install() {
    let agent = Hash40::new("fighter_kind_bayonetta");
    CustomCancelManager::add_cancel_info(
        agent,
        *FIGHTER_STATUS_KIND_ATTACK_100,
        CancelInfo::new()
            .enable_dash_cancel(CancelType::HIT)
            .set_dash_cancel_direction(DashCancelDir::FORWARD)
            .dash_cancel_require_flag()
            .pre_function(disable_during_bullet_arts)
    );
    CustomCancelManager::add_cancel_info(
        agent,
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
        CancelInfo::new()
            .enable_dash_cancel(CancelType::HIT)
            .set_dash_cancel_direction(DashCancelDir::FORWARD)
            .pre_function(disable_during_bullet_arts)
    );
    CustomCancelManager::add_cancel_info(
        agent,
        *FIGHTER_STATUS_KIND_ATTACK_AIR,
        CancelInfo::new()
            .enable_airdash_cancel(CancelType::HIT)
            .airdash_cancel_require_flag()
    );
}
