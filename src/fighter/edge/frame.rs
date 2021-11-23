use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::lua_bind::*,
        lib::lua_const::*
    },
    smashline::*,
    crate::{
        common_funcs::*,
        vars::*
    }
};

#[fighter_frame( agent = FIGHTER_KIND_EDGE )]
fn edge_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        
        // Activate THE WING

        // if sv_information::is_ready_go() == false || StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_DEAD {
        //     CAN_WING[entry_id(fighter.module_accessor)] = true;
        //     ONE_WING[entry_id(fighter.module_accessor)] = -1.0;
        // }

        // if ONE_WING[entry_id(fighter.module_accessor)] >= 0.0 {
        //     if ONE_WING[entry_id(fighter.module_accessor)] == 0.0 {
        //         WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLOAT_ONE_WINGED_ACTIVATE_POINT);
        //     }
        //     ONE_WING[entry_id(fighter.module_accessor)] -= 1.0;
        // }

        // if CAN_WING[entry_id(fighter.module_accessor)] {
        //     if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
        //     && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        //         ONE_WING[entry_id(fighter.module_accessor)] = 900.0;
        //         WorkModule::set_float(fighter.module_accessor, 999.0, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLOAT_ONE_WINGED_ACTIVATE_POINT);
        //         // WorkModule::set_flag(fighter.module_accessor, true, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_ACTIVATED);
        //         // WorkModule::set_flag(fighter.module_accessor, true, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_JUMP_PLUS);
        //         // WorkModule::set_int(fighter.module_accessor, *FIGHTER_EDGE_ONE_WINGED_HAIR_ON, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ONE_WINGED_HAIR_STATE);
        //         WorkModule::set_flag(fighter.module_accessor, true, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_ACTIVATED);
        //         CAN_WING[entry_id(fighter.module_accessor)] = false;
        //     }
        // }

        // Cancel Frames

        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            CANCEL[entry_id(fighter.module_accessor)] = false;
        }
        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH {
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
            || WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_FUNNY) {
                CANCEL[entry_id(fighter.module_accessor)] = true;
            }
        }
        if (StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH
        || StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_END
        || StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_LANDING)
        && CANCEL[entry_id(fighter.module_accessor)] == true {
            CancelModule::enable_cancel(fighter.module_accessor);
        }
    }
}

pub fn install() {
    install_agent_frames!(
        edge_frame
    );
}