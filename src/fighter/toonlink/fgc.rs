use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        phx::*,
        app::lua_bind::*,
        lib::lua_const::*
    },
    custom_var::*,
    custom_cancel::*,
    wubor_utils::vars::*
};

unsafe extern "C" fn toonlink_attackair_pre(fighter: &mut L2CFighterCommon) -> bool {
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_lw") {
        VarModule::set_int(fighter.battle_object, commons::status::int::ENABLED_AERIALS, 0b01111);
        VarModule::on_flag(fighter.battle_object, commons::status::flag::ENABLE_AERIAL_STRING);
    }
    false
}

pub unsafe extern "C" fn install() {
    let agent = Hash40::new("fighter_kind_toonlink");
    CustomCancelManager::add_hp_value(agent, 182.0);
    CustomCancelManager::add_cancel_info(
        agent,
        *FIGHTER_STATUS_KIND_ATTACK,
        CancelInfo::new()
            .enable_normals([
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3
            ].to_vec())
    );
    for x in [
        *FIGHTER_STATUS_KIND_ATTACK_S3,
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        *FIGHTER_STATUS_KIND_ATTACK_DASH
    ].iter() {
        CustomCancelManager::add_cancel_info(
            agent,
            *x,
            CancelInfo::new()
                .enable_normals([
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START,
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START,
                    *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START
                ].to_vec())
        );
    }
    for x in [
        *FIGHTER_STATUS_KIND_ATTACK_S4,
        *FIGHTER_STATUS_KIND_ATTACK_LW4,
    ].iter() {
        CustomCancelManager::add_cancel_info(
            agent,
            *x,
            CancelInfo::new()
        );
    }
    CustomCancelManager::add_cancel_info(
        agent,
        *FIGHTER_STATUS_KIND_ATTACK_HI4,
        CancelInfo::new()
            .enable_jump_cancel(CancelType::HIT)
    );
    CustomCancelManager::add_cancel_info(
        agent,
        *FIGHTER_STATUS_KIND_ATTACK_AIR,
        CancelInfo::new()
            .enable_jump_cancel(CancelType::HIT)
            .jump_cancel_require_flag()
            .enable_aerials(CancelType::HIT)
            .aerial_cancel_require_flag()
            .pre_function(toonlink_attackair_pre)
    );
}
