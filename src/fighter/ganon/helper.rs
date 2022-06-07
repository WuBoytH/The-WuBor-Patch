use {
    smash::{
        lua2cpp::L2CFighterCommon,
        phx::{Hash40, Vector2f},
        app::{lua_bind::*, *},
        lib::lua_const::*
    },
    smash_script::*,
    custom_var::*,
    wubor_utils::{
        wua_bind::*,
        vars::*,
        table_const::*
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
    VarModule::set_vec2(fighter.battle_object, ganon::status::float::END_POS, Vector2f{x: tele_x, y: tele_y});
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        VarModule::on_flag(fighter.battle_object, ganon::status::flag::TELEPORT_FEINT);
    }
}

#[inline(always)]
pub unsafe fn deception_feint_handler(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[FIGHTER_KIND].get_i32() == *FIGHTER_KIND_KIRBY {
        macros::EFFECT(fighter, Hash40::new("ganon_entry"), Hash40::new("top"), 0, 6.0, -2.0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
    }
    else {
        macros::EFFECT(fighter, Hash40::new("ganon_entry"), Hash40::new("top"), 0, 12.0, -2.0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    let tele_y = VarModule::get_vec2(fighter.battle_object, ganon::status::float::END_POS).y;
    if VarModule::is_flag(fighter.battle_object, ganon::status::flag::TELEPORT_FEINT) {
        let og = VarModule::get_vec2(fighter.battle_object, ganon::status::float::START_POS);
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
