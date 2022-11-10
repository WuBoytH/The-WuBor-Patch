use {
    smash::{
        lua2cpp::L2CFighterCommon,
        phx::Vector2f,
        app::{lua_bind::*, *},
        lib::lua_const::*
    },
    custom_var::*,
    wubor_utils::{
        wua_bind::*,
        vars::*
    }
};

#[inline(always)]
pub unsafe fn deception_init(fighter: &mut L2CFighterCommon) {
    let dir = FGCModule::get_command_stick_direction(fighter, false);
    let tele_x = match dir {
        5 => 40.0 * PostureModule::lr(fighter.module_accessor),
        4 => -40.0,
        6 => 40.0,
        1 | 7 => -35.0, 
        3 | 9 => 35.0,
        2 => {
            if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
                40.0 * PostureModule::lr(fighter.module_accessor)
            } else {
                0.0
            }
        },
        _ => 0.0
    };
    let tele_y = match dir {
        8 => 40.0,
        1 | 3 => {
            if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
                0.0
            }
            else {
                -30.0
            }
        },
        2 => {
            if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
                0.0
            }
            else {
                -40.0
            }
        },
        7 | 9 => 30.0,
        _ => 0.0
    };
    VarModule::set_vec2(fighter.battle_object, ganon::status::float::TELEPORT_END_POS, Vector2f{x: tele_x, y: tele_y});
}

#[inline(always)]
pub unsafe fn deception_movement(fighter: &mut L2CFighterCommon) {
    let end_pos = VarModule::get_vec2(fighter.battle_object, ganon::status::float::TELEPORT_END_POS);
    PostureModule::add_pos_2d(fighter.module_accessor, &Vector2f{x: end_pos.x, y: end_pos.y});
    VarModule::set_int(fighter.battle_object, ganon::status::int::TELEPORT_STEP, ganon::TELEPORT_STEP_MOVE_DONE);
}

#[inline(always)]
pub unsafe fn deception_feint_handler(fighter: &mut L2CFighterCommon) {
    if VarModule::is_flag(fighter.battle_object, ganon::status::flag::TELEPORT_FEINT) {
        ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
        let og = VarModule::get_vec2(fighter.battle_object, ganon::status::float::TELEPORT_START_POS);
        GroundModule::set_passable_check(fighter.module_accessor, true);
        PostureModule::set_pos_2d(fighter.module_accessor, &Vector2f {x: og.x, y: og.y});
        if VarModule::is_flag(fighter.battle_object, ganon::status::flag::TELEPORT_START_GROUND) {
            GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        }
        else {
            GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }
    }
    else {
        let tele_y = VarModule::get_vec2(fighter.battle_object, ganon::status::float::TELEPORT_END_POS).y;
        if VarModule::is_flag(fighter.battle_object, ganon::status::flag::TELEPORT_START_GROUND)
        && tele_y == 0.0 {
            GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        }
        else {
            GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }
    }
    VarModule::set_int(fighter.battle_object, ganon::status::int::TELEPORT_STEP, ganon::TELEPORT_STEP_END);
}
