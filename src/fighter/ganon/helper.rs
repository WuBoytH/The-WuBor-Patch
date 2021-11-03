use smash::{
    phx::Vector2f,
    app::{lua_bind::*, *},
    lib::lua_const::*
};
use crate::{
    common_funcs::*,
    vars::*
};

#[inline(always)]
pub unsafe fn deception_init(module_accessor: *mut BattleObjectModuleAccessor) {
    let dir = get_command_stick_direction(module_accessor, false);
    if dir == 5 {
        TELE_X[entry_id(module_accessor)] = 40.0 * PostureModule::lr(module_accessor);
    }
    if dir == 4 {
        TELE_X[entry_id(module_accessor)] = -40.0;
    }
    else if dir == 6 {
        TELE_X[entry_id(module_accessor)] = 40.0;
    }
    else if dir == 3 || dir == 9 {
        TELE_X[entry_id(module_accessor)] = 35.0;
    }
    else if dir == 1 || dir == 7 {
        TELE_X[entry_id(module_accessor)] = -35.0;
    }
    else if dir == 2 {
        if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND {
            TELE_X[entry_id(module_accessor)] = 40.0 * PostureModule::lr(module_accessor);
        }
        else {
            TELE_X[entry_id(module_accessor)] = 0.0;
        }
    }
    else if dir == 8 {
        TELE_X[entry_id(module_accessor)] = 0.0;
    }
    if dir == 5
    || dir == 4
    || dir == 6 {
        TELE_Y[entry_id(module_accessor)] = 0.0;
    }
    else if dir == 1 || dir == 3 {
        if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND {
            TELE_Y[entry_id(module_accessor)] = 0.0;
        }
        else {
            TELE_Y[entry_id(module_accessor)] = -30.0;
        }
    }
    else if dir == 2 {
        if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND {
            TELE_Y[entry_id(module_accessor)] = 0.0;
        }
        else {
            TELE_Y[entry_id(module_accessor)] = -40.0;
        }
    }
    else if dir == 7
    || dir == 9 {
        TELE_Y[entry_id(module_accessor)] = 30.0;
    }
    else if dir == 8 {
        TELE_Y[entry_id(module_accessor)] = 40.0;
    }
    if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        FEINT[entry_id(module_accessor)] = true;
    }
}

#[inline(always)]
pub unsafe fn deception_feint_handler(module_accessor: *mut BattleObjectModuleAccessor) {
    if FEINT[entry_id(module_accessor)] {
        if TELE_Y[entry_id(module_accessor)] != 0.0 {
            StatusModule::set_situation_kind(module_accessor, SituationKind(*SITUATION_KIND_AIR), true);
            GroundModule::correct(module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }
        let ogx = OG_X[entry_id(module_accessor)];
        let ogy = OG_Y[entry_id(module_accessor)];
        GroundModule::set_passable_check(module_accessor, true);
        PostureModule::set_pos_2d(module_accessor, &Vector2f {x: ogx, y: ogy});
        if TELE_Y[entry_id(module_accessor)] == 0.0 {
            GroundModule::correct(module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        }
    }
    OG_X[entry_id(module_accessor)] = 0.0;
    OG_Y[entry_id(module_accessor)] = 0.0;
    TELEPORT[entry_id(module_accessor)] += 1;
}
