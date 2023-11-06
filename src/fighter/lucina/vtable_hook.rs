use crate::imports::status_imports::*;

#[skyline::hook(offset = 0xcd9880)]
pub unsafe extern "C" fn marth_lucina_init(vtable: u64, fighter: &mut Fighter) {
    original!()(vtable, fighter);
    if fighter.battle_object.kind == 0x16 {
        WorkModule::on_flag(fighter.battle_object.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_SPECIAL_COMMAND);
    }
}

pub fn install() {
    skyline::install_hooks!(
        marth_lucina_init
    );
}