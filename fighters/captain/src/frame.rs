use super::*;

extern "C" {
    #[link_name = "common_fighter_frame"]
    pub fn common_fighter_frame(fighter: &mut L2CFighterCommon);
}

pub unsafe extern "C" fn boost_power_handler(fighter: &mut L2CFighterCommon) {
    if [
        *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_STANDBY,
        *FIGHTER_STATUS_KIND_REBIRTH,
    ].contains(&fighter.global_table[0xB].get_i32()) {
        VarModule::off_flag(fighter.module_accessor, vars::captain::instance::flag::HAS_BOOST_POWER);
        VarModule::set_float(fighter.module_accessor, vars::captain::instance::float::BOOST_POWER, 0.0);
    }

    if smashball::is_training_mode() {
        if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) {
            captain_check_boost_power(fighter.module_accessor, 50.0);
        }
    }

    if VarModule::is_flag(fighter.module_accessor, vars::captain::instance::flag::HAS_BOOST_POWER) {
        let eff_handle = VarModule::get_int(fighter.module_accessor, vars::captain::instance::int::BOOST_POWER_EFF) as u32;
        if !EffectModule::is_exist_effect(fighter.module_accessor, eff_handle) {
            EffectModule::req_follow(
                fighter.module_accessor,
                Hash40::new("captain_appeal_hi"),
                Hash40::new("hip"),
                &Vector3f{x: 0.0, y: 0.0, z: 0.0},
                &Vector3f{x: 0.0, y: 0.0, z: 0.0},
                0.85,
                false,
                0,
                0,
                0,
                0,
                0,
                false,
                false
            );
            let eff_handle = EffectModule::get_last_handle(fighter.module_accessor);
            VarModule::set_int(fighter.module_accessor, vars::captain::instance::int::BOOST_POWER_EFF, eff_handle as i32);
        }
    }
}

pub unsafe extern "C" fn captain_falcon_kick_bounce(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[0xb].get_i32() == *FIGHTER_STATUS_KIND_SPECIAL_LW
    && VarModule::is_flag(fighter.module_accessor, vars::captain::status::flag::USED_BOOST_POWER)
    && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD)
    && !fighter.global_table[0x8].get_bool() {
        fighter.change_status(FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_WALL_END.into(), false.into());
    }
}

unsafe extern "C" fn on_main(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    boost_power_handler(fighter);
    captain_falcon_kick_bounce(fighter);
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, on_main);
}