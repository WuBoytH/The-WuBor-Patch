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
    wubor_utils::{wua_bind::*, vars::*}
};

fn set_normals(normals: Vec<i32>) -> CancelInfo {
    CancelInfo::new()
        .enable_normal_cancel(CancelType::HIT | CancelType::BLOCK)
        .enable_normals(normals.clone())
        .enable_special_cancel(CancelType::HIT | CancelType::BLOCK)
        .set_alt_info(
            AltInfo::new()
                .set_flag(vars::yu::instance::flag::SHADOW_FRENZY)
                .enable_normal_cancel(CancelType::HIT | CancelType::BLOCK)
                .enable_normals(normals)
                .enable_special_cancel(CancelType::HIT | CancelType::BLOCK)
                .enable_jump_cancel(CancelType::HIT | CancelType::BLOCK)
        )
}

fn set_aerials() -> CancelInfo {
    CancelInfo::new()
        .pre_function(lucina_attackair_set_cancels)
        .enable_special_cancel(CancelType::HIT | CancelType::BLOCK)
        .enable_aerials(CancelType::HIT | CancelType::BLOCK)
        .enable_jump_cancel(CancelType::HIT)
        .jump_cancel_require_flag()
        .set_alt_info(
            AltInfo::new()
                .set_flag(vars::yu::instance::flag::SHADOW_FRENZY)
                .enable_special_cancel(CancelType::HIT | CancelType::BLOCK)
                .enable_aerials(CancelType::HIT | CancelType::BLOCK)
                .enable_jump_cancel(CancelType::HIT | CancelType::BLOCK)
        )
}

fn set_specials(specials: Vec<i32>) -> CancelInfo {
    CancelInfo::new()
        .enable_special_cancel(CancelType::HIT | CancelType::BLOCK)
        .enable_specials([
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL
        ].to_vec())
        .set_alt_info(
            AltInfo::new()
                .set_flag(vars::yu::instance::flag::SHADOW_FRENZY)
                .enable_special_cancel(CancelType::HIT | CancelType::BLOCK)
                .enable_specials(specials)
        )
}

unsafe extern "C" fn lucina_attackair_set_cancels(fighter: &mut L2CFighterCommon) -> bool {
    let mot = MotionModule::motion_kind(fighter.module_accessor);
    let mut flags = ATTACK_AIR_N_MASK + ATTACK_AIR_F_MASK + ATTACK_AIR_B_MASK + ATTACK_AIR_HI_MASK + ATTACK_AIR_LW_MASK;
    VarModule::on_flag(fighter.module_accessor, vars::fighter::status::flag::ENABLE_AERIAL_STRING);
    if mot == hash40("attack_air_n") {
        FGCModule::disable_aerial(fighter, ATTACK_AIR_N_MASK);
    }
    else if mot == hash40("attack_air_f") {
        FGCModule::disable_aerial(fighter, ATTACK_AIR_F_MASK);
    }
    else if mot == hash40("attack_air_b") {
        FGCModule::disable_aerial(fighter, ATTACK_AIR_B_MASK);
    }
    else if mot == hash40("attack_air_hi") {
        FGCModule::disable_aerial(fighter, ATTACK_AIR_HI_MASK);
    }
    else {
        flags = 0b00000;
        FGCModule::disable_aerial(fighter, ATTACK_AIR_LW_MASK);
    }
    VarModule::set_int(fighter.module_accessor, vars::fighter::status::int::ENABLED_AERIALS, flags);
    false
}

pub fn install() {
    let agent = Hash40::new("fighter_kind_lucina");
    CustomCancelManager::initialize_agent(agent);
    CustomCancelManager::add_cancel_info(
        agent,
        *FIGHTER_STATUS_KIND_ATTACK,
        set_normals([
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3
        ].to_vec())
    );
    CustomCancelManager::add_cancel_info(
        agent,
        *FIGHTER_STATUS_KIND_ATTACK_DASH,
        set_specials([
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL
        ].to_vec())
    );
    CustomCancelManager::add_cancel_info(
        agent,
        *FIGHTER_STATUS_KIND_ATTACK_S3,
        set_normals([].to_vec())
    );
    CustomCancelManager::add_cancel_info(
        agent,
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
        set_normals([].to_vec())
    );
    CustomCancelManager::add_cancel_info(
        agent,
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        set_normals([].to_vec())
            .enable_jump_cancel(CancelType::HIT)
    );
    CustomCancelManager::add_cancel_info(
        agent,
        *FIGHTER_STATUS_KIND_ATTACK_S4,
        set_specials([
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL
        ].to_vec())
    );
    CustomCancelManager::add_cancel_info(
        agent,
        *FIGHTER_STATUS_KIND_ATTACK_HI4,
        set_specials([
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL
        ].to_vec())
    );
    CustomCancelManager::add_cancel_info(
        agent,
        *FIGHTER_STATUS_KIND_ATTACK_LW4,
        set_specials([
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL
        ].to_vec())
    );
    CustomCancelManager::add_cancel_info(
        agent,
        *FIGHTER_STATUS_KIND_ATTACK_AIR,
        set_aerials()
    );
    CustomCancelManager::add_cancel_info(
        agent,
        *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END,
        set_specials([
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL
        ].to_vec())
    );
    CustomCancelManager::add_cancel_info(
        agent,
        *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2,
        set_specials([
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL
        ].to_vec())
    );
    CustomCancelManager::add_cancel_info(
        agent,
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        set_specials([
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL
        ].to_vec())
    );
    CustomCancelManager::add_cancel_info(
        agent,
        *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S4,
        CancelInfo::new()
        .enable_special_cancel(CancelType::HIT | CancelType::BLOCK)
        .enable_specials([*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW].to_vec())
        .set_alt_info(
            AltInfo::new()
                .set_flag(vars::yu::instance::flag::SHADOW_FRENZY)
                .enable_special_cancel(CancelType::HIT | CancelType::BLOCK)
                .enable_specials([*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW].to_vec())
        )
    );
}
