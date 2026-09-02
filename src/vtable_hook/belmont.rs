use crate::imports::*;

pub const SIMON_VTABLE_START : usize = 0x5041690;

#[skyline::hook(offset = 0x11944e0)]
unsafe extern "C" fn belmont_on_init_death(vtable: u64, fighter: &mut Fighter) {
    original!()(vtable, fighter);
    let module_accessor = fighter.battle_object.module_accessor;
    VarModule::off_flag(module_accessor, fighter::instance::flag::DISABLE_SPECIAL_S);
    VarModule::off_flag(module_accessor, fighter::instance::flag::DISABLE_SPECIAL_HI);
}

unsafe extern "C" fn simon_on_situation_change(_vtable: u64, fighter: &mut Fighter, log: *const u64) {
    if *(log as *const u8).add(0xC) != 2 {
        let module_accessor = fighter.battle_object.module_accessor;
        VarModule::off_flag(module_accessor, fighter::instance::flag::DISABLE_SPECIAL_S);
        VarModule::off_flag(module_accessor, fighter::instance::flag::DISABLE_SPECIAL_HI);
    }
}

#[skyline::hook(offset = 0x1195890)]
unsafe extern "C" fn simon_on_damage(_vtable: u64, fighter: &mut Fighter, log: *const u64) {
    if *(log as *const u8).add(0x18) != 0 {
        let module_accessor = fighter.battle_object.module_accessor;
        WorkModule::on_flag(module_accessor, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LASSO_DAMAGE);
        VarModule::off_flag(module_accessor, fighter::instance::flag::DISABLE_SPECIAL_S);
        VarModule::off_flag(module_accessor, fighter::instance::flag::DISABLE_SPECIAL_HI);
    }
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(SIMON_VTABLE_START + (0x8 * 45)).data(simon_on_situation_change as *const () as u64);

    skyline::install_hooks!(
        belmont_on_init_death,
        simon_on_damage
    );
}
