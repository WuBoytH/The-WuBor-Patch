#[skyline::hook(offset = 0x1797ff8, inline)]
unsafe fn fix_chara_replace(ctx: &skyline::hooks::InlineCtx) {
    let ptr1 = *ctx.registers[0].x.as_ref() as *mut u64;
    let ptr2 = *ctx.registers[1].x.as_ref() as *mut u64;

    *ptr2.add(0x2) = *ptr1.add(0x2);
    *ptr2.add(0x3) = *ptr1.add(0x3);
    *ptr2.add(0x4) = *ptr1.add(0x4);
}

pub fn install() {
    skyline::install_hooks!(
        fix_chara_replace
    );
}