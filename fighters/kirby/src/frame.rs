use super::*;

extern "C" {
    #[link_name = "common_fighter_frame"]
    pub fn common_fighter_frame(fighter: &mut L2CFighterCommon);
}

unsafe extern "C" fn kirby_gaogaen_lariat_jump_cancel(fighter: &mut L2CFighterCommon) {
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND
    && StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_KIRBY_STATUS_KIND_GAOGAEN_SPECIAL_N
    && VarModule::is_flag(fighter.module_accessor, vars::fighter::status::flag::JUMP_CANCEL)
    && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
    && !AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL)
    && !fighter.global_table[IS_STOP].get_bool() {
        FGCModule::jump_cancel_check_exception(fighter);
    }
}

unsafe extern "C" fn kirby_ganon_special_n_reset(fighter: &mut L2CFighterCommon) {
    if VarModule::is_flag(fighter.module_accessor, vars::fighter::instance::flag::DISABLE_SPECIAL_N)
    && (StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_CLIFF
    || StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND) {
        VarModule::off_flag(fighter.module_accessor, vars::fighter::instance::flag::DISABLE_SPECIAL_N);
    }
}

unsafe extern "C" fn kirby_taunt_movement(fighter: &mut L2CFighterCommon) {
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("appeal_s_loop") {
        let stickx = ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor);
        let mut spin = 0.5 * stickx;
        if spin.abs() > 0.5 {
            if spin < 0.0 {
                spin = -0.5;
            }
            else {
                spin = 0.5;
            }
        }
        macros::SET_SPEED_EX(fighter, spin, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
}

pub unsafe extern "C" fn kirby_boost_power_handler(fighter: &mut L2CFighterCommon) {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_COPY) {
        return;
    }
    if [
        *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_STANDBY,
        *FIGHTER_STATUS_KIND_REBIRTH,
    ].contains(&fighter.global_table[0xB].get_i32()) {
        VarModule::off_flag(fighter.module_accessor, vars::captain::instance::flag::HAS_BOOST_POWER);
        VarModule::set_float(fighter.module_accessor, vars::captain::instance::float::BOOST_POWER, 0.0);
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

unsafe extern "C" fn on_main(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    kirby_gaogaen_lariat_jump_cancel(fighter);
    kirby_ganon_special_n_reset(fighter);
    kirby_taunt_movement(fighter);
    kirby_boost_power_handler(fighter);
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, on_main);
}