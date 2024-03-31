use crate::imports::*;
use crate::fighter::ike::status::special_n_end::*;

unsafe extern "C" fn kirby_ike_special_n_end_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    ike_special_n_end_init_inner(fighter)
}

unsafe extern "C" fn kirby_ike_special_n_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT);
    kirby_ike_special_n_end_mot_helper(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(kirby_ike_special_n_end_main_loop as *const () as _))
}

unsafe extern "C" fn kirby_ike_special_n_end_mot_helper(fighter: &mut L2CFighterCommon) {
    let mot;
    let kinetic;
    let correct;
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        mot = Hash40::new("special_air_n_end");
        kinetic = *FIGHTER_KINETIC_TYPE_AIR_STOP;
        correct = *GROUND_CORRECT_KIND_AIR;
    }
    else {
        mot = Hash40::new("special_n_end");
        kinetic = *FIGHTER_KINETIC_TYPE_GROUND_STOP;
        correct = *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP;
    }
    KineticModule::change_kinetic(fighter.module_accessor, kinetic);
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(correct));
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT) {
        FighterMotionModuleImpl::change_motion_kirby_copy(
            fighter.module_accessor,
            mot,
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT);
    }
    else {
        FighterMotionModuleImpl::change_motion_inherit_frame_kirby_copy(
            fighter.module_accessor,
            mot,
            -1.0,
            1.0,
            0.0,
            false,
            false
        );
    }
}

unsafe extern "C" fn kirby_ike_special_n_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        kirby_ike_special_n_end_mot_helper(fighter);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        let status = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            FIGHTER_STATUS_KIND_WAIT
        }
        else {
            FIGHTER_STATUS_KIND_FALL
        };
        fighter.change_status(status.into(), false.into());
    }
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Init, *FIGHTER_KIRBY_STATUS_KIND_IKE_SPECIAL_N_END, kirby_ike_special_n_end_init);
    agent.status(smashline::Main, *FIGHTER_KIRBY_STATUS_KIND_IKE_SPECIAL_N_END, kirby_ike_special_n_end_main);
}