use wubor_utils::offsets;

extern "C" {
    #[link_name = "inkling_rollerink_generate_inline"]
    fn inkling_rollerink_generate_inline(ctx: &mut skyline::hooks::InlineCtx);
}

#[skyline::hook(offset = offsets::weapon::inkling_rollerink::GENERATE, inline)]
unsafe extern "C" fn inkling_rollerink_generate(ctx: &mut skyline::hooks::InlineCtx) {
    inkling_rollerink_generate_inline(ctx);
}

pub fn install() {
    skyline::install_hooks!(
        inkling_rollerink_generate
    );
}