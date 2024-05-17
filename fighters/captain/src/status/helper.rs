use super::*;

pub unsafe extern "C" fn captain_set_ground(fighter: &mut L2CFighterCommon) {
    fighter.set_situation(SITUATION_KIND_GROUND.into());
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
}

pub unsafe extern "C" fn captain_set_air(fighter: &mut L2CFighterCommon) {
    fighter.set_situation(SITUATION_KIND_AIR.into());
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
}
