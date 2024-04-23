use crate::imports::*;
use super::helper::*;

#[allow(non_snake_case)]
extern "C" {
    #[link_name = "\u{1}_ZN3app8lua_bind29KineticEnergy__get_speed_implEPNS_13KineticEnergyE"]
    pub fn get_speed(
        energy: *mut smash::app::KineticEnergy,
    ) -> smash_rs::phx::Vector2f;
}

unsafe extern "C" fn edge_special_hi_landing_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let charged_rush = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLAG_CHARGED_RUSH);
    let speed_mul = if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLAG_USE_LANDING_SPEED_MUL) {
        edge_special_hi_param_float_helper(fighter, hash40("ground_speed_x_mul").into(), charged_rush.into()).get_f32()
    }
    else {
        edge_special_hi_param_float_helper(fighter, hash40("landing_speed_x_mul").into(), charged_rush.into()).get_f32()
    };
    let landing_brake_x = edge_special_hi_param_float_helper(fighter, hash40("landing_brake_x").into(), charged_rush.into()).get_f32();
    let stop_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    let speed = get_speed(stop_energy as *mut smash::app::KineticEnergy);
    lua_bind::KineticEnergyNormal::set_speed(
        stop_energy as *mut smash::app::KineticEnergyNormal,
        &Vector2f{x: speed.x * speed_mul, y: 0.0}
    );
    lua_bind::KineticEnergyNormal::set_brake(
        stop_energy as *mut smash::app::KineticEnergyNormal,
        &Vector2f{x: landing_brake_x, y: 0.0}
    );
    let mot = if !charged_rush {
        Hash40::new("special_hi1_end")
    }
    else {
        Hash40::new("special_hi2_end")
    };
    MotionModule::change_motion(
        fighter.module_accessor,
        mot,
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(edge_special_hi_landing_main_loop as *const () as _))
}

unsafe extern "C" fn edge_special_hi_landing_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        VarModule::off_flag(fighter.module_accessor, edge::status::flag::SPECIAL_HI_CANCEL);
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    // <WuBor>
    if edge_special_hi_cancel(fighter).get_bool() {
        return 1.into();
    }
    // </WuBor>
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    else {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn edge_special_hi_landing_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("edge_octaslash_line"), true, true);
    if !VarModule::is_flag(fighter.module_accessor, edge::status::flag::SPECIAL_HI_CANCEL) {
        VarModule::set_int(fighter.module_accessor, edge::instance::int::SPECIAL_HI_CANCEL_COUNT, 0);
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_LANDING, edge_special_hi_landing_main);
    agent.status(End, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_LANDING, edge_special_hi_landing_end);
}