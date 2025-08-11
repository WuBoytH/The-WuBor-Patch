use crate::imports::*;

#[skyline::hook(offset = 0xb103bc, inline)]
unsafe extern "C" fn inkling_rollerink_generate(ctx: &mut skyline::hooks::InlineCtx) {
    let module_accessor = *ctx.registers[20].x.as_mut() as *mut BattleObjectModuleAccessor;
    let status = StatusModule::status_kind(module_accessor);
    if ![
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_WALK,
        *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_WALK_TURN,
        *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_DASH,
        *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_RUN,
        *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_RUN_TURN,
        *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_STOP_WALL,
        *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_JUMP_END,
        *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_END,
    ].contains(&status) {
        let module = (module_accessor as *mut u64).add(0x58 / 0x8);
        let func = MiscModule::get_module_vtable_func(module_accessor, 0x58, 0x580);
        *ctx.registers[0].x.as_mut() = *module;
        *ctx.registers[8].x.as_mut() = func;
    }
}

pub fn install() {
    skyline::install_hooks!(
        inkling_rollerink_generate
    );
}