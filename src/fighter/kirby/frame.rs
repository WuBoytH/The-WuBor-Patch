use {
    crate::imports::status_imports::*,
    crate::fighter::common::frame::common_fighter_frame
};

unsafe extern "C" fn kirby_gaogaen_lariat_jump_cancel(fighter: &mut L2CFighterCommon) {
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND
    && StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_KIRBY_STATUS_KIND_GAOGAEN_SPECIAL_N
    && VarModule::is_flag(fighter.module_accessor, fighter::status::flag::JUMP_CANCEL)
    && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
    && !AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL)
    && !fighter.global_table[IS_STOP].get_bool() {
        FGCModule::jump_cancel_check_exception(fighter);
    }
}

unsafe extern "C" fn kirby_ganon_special_n_reset(fighter: &mut L2CFighterCommon) {
    if VarModule::is_flag(fighter.module_accessor, fighter::instance::flag::DISABLE_SPECIAL_N)
    && (StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_CLIFF
    || StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND) {
        VarModule::off_flag(fighter.module_accessor, fighter::instance::flag::DISABLE_SPECIAL_N);
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

unsafe extern "C" fn kirby_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    kirby_gaogaen_lariat_jump_cancel(fighter);
    kirby_ganon_special_n_reset(fighter);
    kirby_taunt_movement(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, kirby_frame);
}