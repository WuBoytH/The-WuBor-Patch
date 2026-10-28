use super::*;

// Inline hooked in the main plugin (src/vtable_hook/kirby.rs).
#[no_mangle]
pub unsafe extern "C" fn kirby_frame_branch_copy_vtable_inline(ctx: &mut skyline::hooks::InlineCtx) {
    // println!("kirby_frame_branch_copy_vtable");
    let kind = ctx.registers[8].x() as i32;
    // println!("kind?: {:#x}", kind);
    if kind == *FIGHTER_KIND_CAPTAIN {
        ctx.registers[8].set_x(0xF);
    }
}
