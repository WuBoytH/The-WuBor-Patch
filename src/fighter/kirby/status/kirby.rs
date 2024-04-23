use crate::imports::*;
use super::super::vl;

unsafe extern "C" fn kirby_attackdash_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    sv_kinetic_energy!(
        set_speed_mul,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_MOTION,
        1.32
    );
    fighter.status_AttackDash()
}

unsafe extern "C" fn kirby_attacklw3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_AttackLw3_common();
    fighter.sub_shift_status_main(L2CValue::Ptr(kirby_attacklw3_main_loop as *const () as _))
}

unsafe extern "C" fn kirby_attacklw3_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !VarModule::is_flag(fighter.module_accessor, kirby::status::flag::ATTACK_LW3_BOUNCE) {
        if !AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL)
        && !fighter.global_table[IS_STOP].get_bool()
        && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            macros::EFFECT(fighter, Hash40::new("kirby_star"), Hash40::new("top"), 0, 3, 10, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), true);
            GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("jump_b"), 22.0, 31.0 / 20.0, false, 0.0, false, false);
            macros::SET_SPEED_EX(fighter, -0.5, 1.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            AttackModule::clear_all(fighter.module_accessor);
            FighterControlModuleImpl::update_attack_air_kind(fighter.module_accessor, true);
            VarModule::on_flag(fighter.module_accessor, kirby::status::flag::ATTACK_LW3_BOUNCE);
        }
        return fighter.status_AttackLw3_Main();
    }
    else {
        if CancelModule::is_enable_cancel(fighter.module_accessor)
        && fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
        else if MotionModule::frame(fighter.module_accessor) >= vl::param_special_lw::slide_bounce_cancel_frame {
            CancelModule::enable_cancel(fighter.module_accessor);
        }
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 1.into();
        }
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_DASH, kirby_attackdash_main);

    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_LW3, kirby_attacklw3_main);
}