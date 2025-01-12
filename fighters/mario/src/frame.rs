use super::*;

extern "C" {
    #[link_name = "common_fighter_frame"]
    pub fn common_fighter_frame(fighter: &mut L2CFighterCommon);
}

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
    || StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_CLIFF
    || fighter.global_table[STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_REBIRTH {
        VarModule::off_flag(fighter.module_accessor, vars::fighter::instance::flag::DISABLE_SPECIAL_LW);
    }
}

unsafe extern "C" fn on_main(fighter: &mut L2CFighterCommon) {
    common_fighter_frame(fighter);
    mario_attack_air_lw_bounce(fighter);
    mario_reset_special_lw_kind(fighter);
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, on_main);
}