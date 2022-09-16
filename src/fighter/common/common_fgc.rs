use {
    smash::{
        lua2cpp::L2CFighterCommon,
        phx::*,
        app::lua_bind::*,
        lib::lua_const::*
    },
    smashline::*,
    custom_var::*,
    custom_cancel::*,
    wubor_utils::{
        wua_bind::*,
        vars::*,
    }
};

pub unsafe fn fgc_frame(fighter: &mut L2CFighterCommon) {
    if VarModule::is_flag(fighter.battle_object, commons::instance::flag::IS_FGC) {
        if !MiscModule::is_damage_check(fighter.module_accessor, false) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ESCAPE_XLU_START_1F);
        }
        else {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ESCAPE_XLU_START_1F);
        }
        let hp = CustomCancelManager::get_hp_value((*fighter.battle_object).agent_kind_hash);
        if hp != 0.0 {
            MiscModule::set_hp(fighter, hp);
        }
        else {
            MiscModule::set_hp(fighter, 190.0);
        }
    }
    // if fighter.global_table["fgc_func"].get_bool() {
    //     let callable: extern "C" fn(&mut L2CFighterCommon) = std::mem::transmute(fighter.global_table["fgc_func"].get_ptr());
    //     callable(fighter);
    // }
}

pub unsafe extern "C" fn ftilt_dash_attack(fighter: &mut L2CFighterCommon) -> bool {
    FGCModule::cancel_exceptions(fighter, *FIGHTER_STATUS_KIND_ATTACK_DASH, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3, true).get_bool()
}

pub unsafe fn generic_attack(agent: Hash40) {
    CustomCancelManager::add_cancel_info(
        agent,
        *FIGHTER_STATUS_KIND_ATTACK,
        CancelInfo::new()
            .enable_normals([
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START
            ].to_vec())
    );
}

pub unsafe fn generic_attackair(agent: Hash40) {
    CustomCancelManager::add_cancel_info(
        agent,
        *FIGHTER_STATUS_KIND_ATTACK_AIR,
        CancelInfo::new()
    );
}

pub unsafe fn generic_attack3(agent: Hash40) {
    for x in [
        *FIGHTER_STATUS_KIND_ATTACK_S3,
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        *FIGHTER_STATUS_KIND_ATTACK_DASH
    ].iter() {
        let mut info = CancelInfo::new()
        .enable_normals([
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START
        ].to_vec());
        if *x == *FIGHTER_STATUS_KIND_ATTACK_S3 {
            info = info.exception_function(ftilt_dash_attack);
        }
        if *x == *FIGHTER_STATUS_KIND_ATTACK_HI3 {
            info = info.enable_jump_cancel(CancelType::HIT);
        }
        CustomCancelManager::add_cancel_info(
            agent,
            *x,
            info
        );
    }
}

pub unsafe fn generic_attack4(agent: Hash40) {
    for x in [
        *FIGHTER_STATUS_KIND_ATTACK_S4,
        *FIGHTER_STATUS_KIND_ATTACK_LW4,
        *FIGHTER_STATUS_KIND_ATTACK_HI4
    ].iter() {
        let mut info = CancelInfo::new()
        .enable_normals([
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START
        ].to_vec());
        if *x == *FIGHTER_STATUS_KIND_ATTACK_HI4 {
            info = info.enable_jump_cancel(CancelType::HIT);
        }
        CustomCancelManager::add_cancel_info(
            agent,
            *x,
            info
        );
    }
}

#[fighter_init]
fn agent_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        if CustomCancelManager::initialize_agent((*fighter.battle_object).agent_kind_hash) {
            let agent = (*fighter.battle_object).agent_kind_hash;
            generic_attack(agent);
            generic_attackair(agent);
            generic_attack3(agent);
            generic_attack4(agent);
        }
    }
}

pub fn install() {
    install_agent_init_callbacks!(
        agent_init
    );
}
