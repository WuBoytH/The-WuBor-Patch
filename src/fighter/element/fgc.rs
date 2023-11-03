use {
    smash::{
        lua2cpp::*,
        hash40,
        phx::*,
        app::lua_bind::*,
        lib::lua_const::*
    },
    custom_var::*,
    custom_cancel::*,
    wubor_utils::vars::*
};

unsafe extern "C" fn elight_attackair_set_cancels(fighter: &mut L2CFighterCommon) -> bool {
    let mot = MotionModule::motion_kind(fighter.module_accessor);
    if mot == hash40("attack_air_n") {
        VarModule::on_flag(fighter.module_accessor, fighter::status::flag::ENABLE_AERIAL_STRING);
        VarModule::set_int(fighter.module_accessor, fighter::status::int::ENABLED_AERIALS, ATTACK_AIR_N_MASK);
    }
    false
}

pub unsafe extern "C" fn element_fgc(agent: Hash40) {
    if agent.hash == hash40("fighter_kind_elight") {
        CustomCancelManager::add_cancel_info(
            agent,
            *FIGHTER_STATUS_KIND_ATTACK_AIR,
            CancelInfo::new()
                .enable_aerials(CancelType::HIT | CancelType::BLOCK)
                .aerial_cancel_require_flag()
                .pre_function(elight_attackair_set_cancels)
        );
    }
}
