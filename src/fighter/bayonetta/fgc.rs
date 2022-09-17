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
    wubor_utils::{
        wua_bind::*,
        vars::*
    }
};

pub unsafe extern "C" fn disable_during_bullet_arts(fighter: &mut L2CFighterCommon) -> bool {
    WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION)
}

pub unsafe extern "C" fn bayo_disable_aerials(fighter: &mut L2CFighterCommon) -> bool {
    let mot = MotionModule::motion_kind(fighter.module_accessor);
    let flags = ATTACK_AIR_N_MASK + ATTACK_AIR_F_MASK + ATTACK_AIR_B_MASK + ATTACK_AIR_HI_MASK + ATTACK_AIR_LW_MASK;
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
    else if mot == hash40("attack_air_lw") {
        FGCModule::disable_aerial(fighter, ATTACK_AIR_LW_MASK);
    }
    VarModule::set_int(fighter.battle_object, commons::status::int::ENABLED_AERIALS, flags);
    disable_during_bullet_arts(fighter)
}

pub unsafe extern "C" fn install() {
    let agent = Hash40::new("fighter_kind_bayonetta");
    CustomCancelManager::add_hp_value(agent, 168.0);
    CustomCancelManager::add_cancel_info(
        agent,
        *FIGHTER_STATUS_KIND_ATTACK,
        CancelInfo::new()
            .enable_normals([
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3
            ].to_vec())
            .pre_function(disable_during_bullet_arts)
    );
    CustomCancelManager::add_cancel_info(
        agent,
        *FIGHTER_STATUS_KIND_ATTACK_100,
        CancelInfo::new()
            .enable_normals([
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START
            ].to_vec())
            .enable_dash_cancel(CancelType::HIT)
            .set_dash_cancel_direction(DashCancelDir::FORWARD)
            .dash_cancel_require_flag()
            .set_fgc_flags(FGCFlags::AERIAL | FGCFlags::AIRDASH | FGCFlags::NORMAL | FGCFlags::SPECIAL | FGCFlags::JUMP)
            .pre_function(disable_during_bullet_arts)
    );
    CustomCancelManager::add_cancel_info(
        agent,
        *FIGHTER_STATUS_KIND_ATTACK_S3,
        CancelInfo::new()
            .enable_normals([
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START
            ].to_vec())
            .enable_jump_cancel(CancelType::HIT)
            .jump_cancel_require_flag()
            .set_fgc_flags(FGCFlags::AERIAL | FGCFlags::AIRDASH | FGCFlags::NORMAL | FGCFlags::SPECIAL | FGCFlags::DASH)
            .pre_function(disable_during_bullet_arts)
    );
    CustomCancelManager::add_cancel_info(
        agent,
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        CancelInfo::new()
            .enable_normals([
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START
            ].to_vec())
            .pre_function(disable_during_bullet_arts)
    );
    CustomCancelManager::add_cancel_info(
        agent,
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
        CancelInfo::new()
            .enable_normals([
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START
            ].to_vec())
            .enable_dash_cancel(CancelType::HIT)
            .set_dash_cancel_direction(DashCancelDir::FORWARD)
            .set_fgc_flags(FGCFlags::AERIAL | FGCFlags::AIRDASH | FGCFlags::NORMAL | FGCFlags::SPECIAL | FGCFlags::JUMP)
            .pre_function(disable_during_bullet_arts)
    );
    CustomCancelManager::add_cancel_info(
        agent,
        *FIGHTER_STATUS_KIND_ATTACK_AIR,
        CancelInfo::new()
            .enable_aerials(CancelType::HIT | CancelType::BLOCK)
            .enable_jump_cancel(CancelType::HIT)
            .pre_function(bayo_disable_aerials)
    );
    CustomCancelManager::add_cancel_info(
        agent,
        *FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F,
        CancelInfo::new()
            .enable_aerials(CancelType::HIT | CancelType::BLOCK)
            .enable_jump_cancel(CancelType::HIT)
            .pre_function(bayo_disable_aerials)
    );
    // if [
    //     *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_U
    // ].contains(&status)
    // && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION) {
    //     if FGCModule::air_dash_cancel_check(fighter, false, false).get_bool() {
    //         return;
    //     }
    // }
    // if [
    //     *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_HI_JUMP
    // ].contains(&status)
    // && MotionModule::frame(fighter.module_accessor) > 31.0 {
    //     if FGCModule::air_dash_cancel_check(fighter, false, false).get_bool() {
    //         return;
    //     }
    // }
}
