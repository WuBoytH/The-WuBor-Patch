use super::*;
use wubor_utils::offsets;

#[skyline::from_offset(offsets::fighter::belmont::ON_INIT_DEATH)]
unsafe extern "C" fn richter_on_death_inner(vtable: u64, fighter: &mut Fighter);

unsafe extern "C" fn richter_on_death(vtable: u64, fighter: &mut Fighter){
    richter_on_death_inner(vtable, fighter);
    let module_accessor = fighter.battle_object.module_accessor;
    VarModule::off_flag(module_accessor, vars::fighter::instance::flag::DISABLE_SPECIAL_S);
    VarModule::off_flag(module_accessor, vars::fighter::instance::flag::DISABLE_SPECIAL_HI);
}

unsafe extern "C" fn richter_second_situation_callback(_vtable: u64, fighter: &mut Fighter, log: *const u64) {
    if *(log as *const u8).add(0xC) != 2 {
        let module_accessor = fighter.battle_object.module_accessor;
        VarModule::off_flag(module_accessor, vars::fighter::instance::flag::DISABLE_SPECIAL_S);
        VarModule::off_flag(module_accessor, vars::fighter::instance::flag::DISABLE_SPECIAL_HI);
    }
}

unsafe extern "C" fn richter_on_damage(_vtable: u64, fighter: &mut Fighter, log: *const u64) {
    if *(log as *const u8).add(0x18) != 0 {
        let module_accessor = fighter.battle_object.module_accessor;
        WorkModule::on_flag(module_accessor, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LASSO_DAMAGE);
        VarModule::off_flag(module_accessor, vars::fighter::instance::flag::DISABLE_SPECIAL_S);
        VarModule::off_flag(module_accessor, vars::fighter::instance::flag::DISABLE_SPECIAL_HI);
    }
}

pub fn install() {
    replace_fighter_vtable(
        *FIGHTER_KIND_RICHTER,
        FighterVtableSlot::OnDeath,
        richter_on_death as *const () as _
    );
    replace_fighter_vtable(
        *FIGHTER_KIND_RICHTER,
        FighterVtableSlot::SecondSituationCallback,
        richter_second_situation_callback as *const () as _
    );
    replace_fighter_vtable(
        *FIGHTER_KIND_RICHTER,
        FighterVtableSlot::OnDamage,
        richter_on_damage as *const () as _
    );
}
