use {
    smash::{
        lua2cpp::*,
        app::{lua_bind::*, *},
        lib::lua_const::*
    },
    smashline::*,
    custom_var::*,
    wubor_utils::{vars::*, table_const::*}
};

unsafe fn hit_cancel_frame_set(fighter: &mut L2CFighterCommon) {
    let frame = fighter.global_table[STATUS_FRAME].get_f32();
    let hit_frame = VarModule::get_float(fighter.battle_object, fighter::status::float::HIT_FRAME);

    if frame < hit_frame {
        VarModule::set_float(fighter.battle_object, fighter::status::float::HIT_FRAME, 0.0);
    }

    if hit_frame == 0.0 {
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
        || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
            VarModule::set_float(fighter.battle_object, fighter::status::float::HIT_FRAME, frame);
        }
    }
    
    if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
    || AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD)
    && frame != hit_frame {
        VarModule::set_float(fighter.battle_object, fighter::status::float::HIT_FRAME, frame);
    }
}

unsafe fn special_jump_stick_flick(fighter: &mut L2CFighterCommon) {
    if VarModule::get_float(fighter.battle_object, fighter::instance::float::FLICK_DOWN) > 0.0
    && !fighter.global_table[IS_STOP].get_bool() {
        VarModule::sub_float(fighter.battle_object, fighter::instance::float::FLICK_DOWN, 1.0);
    }

    if fighter.global_table[STICK_Y].get_f32() < -0.8
    && fighter.global_table[FLICK_Y].get_i32() < 4
    && fighter.global_table[FLICK_Y_DIR].get_f32() < 0.0 {
        VarModule::set_float(fighter.battle_object, fighter::instance::float::FLICK_DOWN, 5.0);
    }
}

// Use this for general per-frame fighter-level hooks
#[line(main)]
unsafe fn common_fighter_frame(fighter: &mut L2CFighterCommon) {
    if utility::get_category(&mut *fighter.module_accessor) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        hit_cancel_frame_set(fighter);
        special_jump_stick_flick(fighter);
    }
}

pub fn install() {
    common_fighter_frame::install();
}