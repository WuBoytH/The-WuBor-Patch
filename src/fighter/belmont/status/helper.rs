use crate::imports::status_imports::*;

pub unsafe extern "C" fn belmont_mot_kinetic_helper(
    fighter: &mut L2CFighterCommon,
    some_bool: L2CValue,
    mot_g: L2CValue,
    mot_a: L2CValue,
    kinetic_g: L2CValue,
    kinetic_a: L2CValue,
    correct_g: L2CValue,
    correct_a: L2CValue
) -> L2CValue {
    if !some_bool.get_bool()
    && !StatusModule::is_situation_changed(fighter.module_accessor) {
        return false.into();
    }
    let mot;
    let kinetic;
    let correct;
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        mot = mot_g.get_u64();
        kinetic = kinetic_g.get_i32();
        correct = correct_g.get_i32();
    }
    else {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        mot = mot_a.get_u64();
        kinetic = kinetic_a.get_i32();
        correct = correct_a.get_i32();
    }
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(correct));
    if kinetic != FIGHTER_KINETIC_TYPE_NONE {
        KineticModule::change_kinetic(fighter.module_accessor, kinetic);
        if some_bool.get_bool() {
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new_raw(mot),
                0.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
        }
        else {
            MotionModule::change_motion_inherit_frame(
                fighter.module_accessor,
                Hash40::new_raw(mot),
                -1.0,
                1.0,
                0.0,
                false,
                false
            );
        }
    }
    true.into()
}