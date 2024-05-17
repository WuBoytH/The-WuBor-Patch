use super::*;

unsafe extern "C" fn sheik_special_hi_move_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let move_time = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("move_time"));
    WorkModule::set_int(fighter.module_accessor, move_time, *FT_SHEIK_STATUS_SPECIAL_HI_WORK_INT_FRAME);
    
    let move_xlu = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("move_xlu"));
    WorkModule::set_int(fighter.module_accessor, move_xlu, *FT_SHEIK_STATUS_SPECIAL_HI_WORK_INT_XLU_FRAME);

    ArticleModule::generate_article(
        fighter.module_accessor,
        *FIGHTER_SHEIK_GENERATE_ARTICLE_FUSIN,
        false,
        -1
    );

    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        GroundModule::set_passable_check(fighter.module_accessor, true);
    }

    VisibilityModule::set_whole(fighter.module_accessor, false);

    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f20a9d549), false);

    WorkModule::set_flag(fighter.module_accessor, false, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);

    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_SHEIK_SPECIAL_HI_MOVE);

    // I'M REWRITING THIS ENTIRE STATUS JUST TO GET RID OF THIS ONE LINE RAAAAAAAAAAAAGH
    // fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES.into());

    if !StopModule::is_stop(fighter.module_accessor) {
        sheik_special_hi_move_substatus(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(sheik_special_hi_move_substatus as *const () as _));

    fighter.sub_shift_status_main(L2CValue::Ptr(sheik_special_hi_move_main_loop as *const () as _))
}

unsafe extern "C" fn sheik_special_hi_move_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if !param_1.get_bool() {
        let xlu_frame = WorkModule::get_int(fighter.module_accessor, *FT_SHEIK_STATUS_SPECIAL_HI_WORK_INT_XLU_FRAME);
        if xlu_frame <= 0 {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
                GroundModule::set_passable_check(fighter.module_accessor, false);
            }
        }
    }
    else {
        WorkModule::dec_int(fighter.module_accessor, *FT_SHEIK_STATUS_SPECIAL_HI_WORK_INT_FRAME);
        WorkModule::dec_int(fighter.module_accessor, *FT_SHEIK_STATUS_SPECIAL_HI_WORK_INT_XLU_FRAME);
    }
    0.into()
}

unsafe extern "C" fn sheik_special_hi_move_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }

    let frame = WorkModule::get_int(fighter.module_accessor, *FT_SHEIK_STATUS_SPECIAL_HI_WORK_INT_FRAME);
    if frame <= 0 {
        fighter.change_status(FIGHTER_SHEIK_STATUS_KIND_SPECIAL_HI_END.into(), false.into());
        return 1.into();
    }

    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && GroundModule::is_touch(fighter.module_accessor, (*GROUND_TOUCH_FLAG_LEFT | *GROUND_TOUCH_FLAG_RIGHT) as u32) {
        fighter.change_status(FIGHTER_SHEIK_STATUS_KIND_SPECIAL_HI_END.into(), false.into());
        return 1.into();
    }

    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_HI_MOVE, sheik_special_hi_move_main);
}