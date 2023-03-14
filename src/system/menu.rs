// Shamelessly taken from HDR

#[skyline::hook(offset = 0x235be30, inline)]
unsafe fn main_menu_quick(ctx: &skyline::hooks::InlineCtx) {
    let sp = (ctx as *const skyline::hooks::InlineCtx as *mut u8).add(0x100);
    *(sp.add(0x60) as *mut u64) = 0x1100000000;
    let slice = std::slice::from_raw_parts_mut(sp.add(0x68), 18);
    slice.copy_from_slice(b"MenuSequenceScene\0");
    // let mode = (skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64 + 0x53030f0) as *const u64;
    // if we are in the controls menu mode, there is no ui overlay, so dont update the hud
    // println!("{:#x}", *mode);
}

pub fn install() {
    skyline::install_hooks!(
        main_menu_quick
    );
}