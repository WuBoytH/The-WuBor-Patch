use {
    crate::imports::status_imports::*,
    crate::fighter::common::frame::common_fighter_frame
};

unsafe extern "C" fn mario_attack_air_lw_bounce(fighter: &mut L2CFighterCommon) {
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_lw") {
        if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) {
            macros::PLAY_SE(fighter, Hash40::new("se_mario_attackair_l02"));
            let speedx = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) * PostureModule::lr(fighter.module_accessor);
            macros::SET_SPEED_EX(fighter, speedx, 0.75, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
    }
}

unsafe extern "C" fn mario_reset_special_lw_kind(fighter: &mut L2CFighterCommon) {
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND
    || StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_CLIFF {
        if ![
            *FIGHTER_STATUS_KIND_SPECIAL_LW,
            *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT,
            *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_CHARGE
        ].contains(&StatusModule::status_kind(fighter.module_accessor)) {
            VarModule::set_int(fighter.module_accessor, mario::instance::int::SPECIAL_LW_KIND, mario::SPECIAL_LW_KIND_LONG_JUMP);
        }
    }
}

unsafe extern "C" fn mario_frame(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    mario_attack_air_lw_bounce(fighter);
    mario_reset_special_lw_kind(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, mario_frame);
}