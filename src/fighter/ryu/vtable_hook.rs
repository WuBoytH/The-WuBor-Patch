use crate::imports::status_imports::*;

#[skyline::hook(offset = 0x10d4550)]
unsafe extern "C" fn ryu_ken_init(vtable: u64, fighter: &mut Fighter) {
    original!()(vtable, fighter);
    let module_accessor = fighter.battle_object.module_accessor;
    if fighter.battle_object.kind != 0x3d {
        FGCModule::set_command_input_button(module_accessor, 1, 2);
    }
    FGCModule::set_command_input_button(module_accessor, 0, 2);
    FGCModule::set_command_input_button(module_accessor, 2, 2);
    FGCModule::set_command_input_button(module_accessor, 3, 2);
    FGCModule::set_command_input_button(module_accessor, 7, 2);
    FGCModule::set_command_input_button(module_accessor, 8, 2);
    FGCModule::set_command_input_button(module_accessor, 9, 2);
    FGCModule::set_command_input_button(module_accessor, 10, 2);
    FGCModule::set_command_input_button(module_accessor, 11, 2);
}

pub fn install() {
    // Patches out the removal of unused command input classes for Ryu and Ken
    skyline::patching::Patch::in_text(0x10d45a4).data(0x14000014u32);

    skyline::install_hooks!(
        ryu_ken_init
    );
}