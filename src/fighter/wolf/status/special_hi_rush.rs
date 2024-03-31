use crate::imports::*;

unsafe extern "C" fn wolf_special_hi_rush_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_FOX_FIRE_STATUS_WORK_ID_FLAG_CONTINUE);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_FOX_FIRE_STATUS_WORK_ID_INT_RUSH_FRAME);
    GroundModule::set_passable_check(fighter.module_accessor, true);
    GroundModule::select_cliff_hangdata(fighter.module_accessor, *FIGHTER_FOX_CLIFF_HANG_DATA_SPECIAL_HI as u32);
    if !StopModule::is_stop(fighter.module_accessor) {
        wolf_special_hi_rush_substatus(fighter);
    }
    fighter.global_table[SUB_STATUS2].assign(&L2CValue::Ptr(wolf_special_hi_rush_substatus as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(wolf_special_hi_rush_main_loop as *const () as _))
}

unsafe extern "C" fn wolf_special_hi_rush_substatus(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::inc_int(fighter.module_accessor, *FIGHTER_FOX_FIRE_STATUS_WORK_ID_INT_RUSH_FRAME);
    let rush_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_FOX_FIRE_STATUS_WORK_ID_INT_RUSH_FRAME);
    let fire_rush_xlu_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("fire_rush_xlu_frame"));
    if fire_rush_xlu_frame < rush_frame {
        // Excuse me????
        GroundModule::set_passable_check(fighter.module_accessor, false);
    }
    0.into()
}

unsafe extern "C" fn wolf_special_hi_rush_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    let rush_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_FOX_FIRE_STATUS_WORK_ID_INT_RUSH_FRAME);
    let fire_rush_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("fire_rush_frame"));
    if fire_rush_frame < rush_frame {
        fighter.change_status(FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_RUSH_END.into(), false.into());
        return 0.into();
    }
    let situation = fighter.global_table[SITUATION_KIND].get_i32();
    if StatusModule::is_changing(fighter.module_accessor)
    || (
        StatusModule::is_situation_changed(fighter.module_accessor)
        && situation == *SITUATION_KIND_AIR
    ) {
        if situation != *SITUATION_KIND_GROUND {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_FOX_FIRE_STATUS_WORK_ID_FLAG_AIR);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_BRAKE);
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FOX_FIRE_STATUS_WORK_ID_FLAG_CONTINUE) {
                MotionModule::change_motion(
                    fighter.module_accessor,
                    Hash40::new("special_hi"),
                    0.0,
                    1.0,
                    false,
                    0.0,
                    false,
                    false
                );
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_FOX_FIRE_STATUS_WORK_ID_FLAG_CONTINUE);
            }
            else {
                MotionModule::change_motion_inherit_frame(
                    fighter.module_accessor,
                    Hash40::new("special_hi"),
                    -1.0,
                    1.0,
                    0.0,
                    false,
                    false
                );
            }
            fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES.into()); // Was ALWAYS_BOTH_SIDES
        }
        else {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_FOX_FIRE_STATUS_WORK_ID_FLAG_AIR);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_BRAKE);
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FOX_FIRE_STATUS_WORK_ID_FLAG_CONTINUE) {
                MotionModule::change_motion(
                    fighter.module_accessor,
                    Hash40::new("special_hi"),
                    0.0,
                    1.0,
                    false,
                    0.0,
                    false,
                    false
                );
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_FOX_FIRE_STATUS_WORK_ID_FLAG_CONTINUE);
            }
            else {
                MotionModule::change_motion_inherit_frame(
                    fighter.module_accessor,
                    Hash40::new("special_hi"),
                    -1.0,
                    1.0,
                    0.0,
                    false,
                    false
                );
            }
            fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_NONE.into());
        }
    }
    if !fighter.global_table[IS_STOP].get_bool() {
        wolf_special_hi_rush_handle_bound(fighter);
    }
    0.into()
}

unsafe extern "C" fn wolf_special_hi_rush_handle_bound(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() == *FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_RUSH
    && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FOX_FIRE_STATUS_WORK_ID_FLAG_AIR)
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        let normal_x = GroundModule::get_touch_normal_x(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
        let normal_y = GroundModule::get_touch_normal_y(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
        let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let angle = sv_math::vec2_angle(normal_x, normal_y, speed_x, speed_y);
        let fire_crush_angle = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("fire_crush_angle"));
        let rad = (fire_crush_angle + 90.0).to_radians();
        if rad < angle {
            fighter.change_status(FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_BOUND.into(), false.into());
        }
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Main, *FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_RUSH, wolf_special_hi_rush_main);
}