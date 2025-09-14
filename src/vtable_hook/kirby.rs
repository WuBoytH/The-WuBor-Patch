use crate::imports::*;

extern "C" {
    #[link_name = "captain_set_lightweight"]
    pub fn captain_set_lightweight(module_accessor: *mut BattleObjectModuleAccessor);
}

#[skyline::hook(offset = 0xb96770)]
pub unsafe extern "C" fn kirby_lose_copy_ability(fighter: &mut Fighter, param_1: u32) {
    let module_accessor = fighter.battle_object.module_accessor;
    let copy_module = WorkModule::get_int64(module_accessor, 0x10000106); // *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_MODULE_ADDRESS
    if copy_module != 0 {
        let copy_kind = *(copy_module as *const u32).add(0x17398 / 0x4);
        if copy_kind == *FIGHTER_KIND_DOLLY as u32 {
            WorkModule::off_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_SPECIAL_COMMAND);
            ControlModule::reset_special_command(module_accessor, true);
        }
        if copy_kind == *FIGHTER_KIND_CAPTAIN as u32 {
            VarModule::set_float(module_accessor, captain::instance::float::BOOST_POWER, 0.0);
            captain_set_lightweight(module_accessor);
        }
    }
    original!()(fighter, param_1);
}

#[skyline::hook(offset = 0xb97c78, inline)]
unsafe extern "C" fn kirby_frame_branch_copy_vtable(ctx: &mut skyline::hooks::InlineCtx) {
    // println!("kirby_frame_branch_copy_vtable");
    let kind = ctx.registers[8].x() as i32;
    // println!("kind?: {:#x}", kind);
    if kind == *FIGHTER_KIND_CAPTAIN {
        ctx.registers[8].set_x(0xF);
    }
}

pub fn install() {
    skyline::install_hooks!(
        kirby_lose_copy_ability,
        kirby_frame_branch_copy_vtable
    );
}