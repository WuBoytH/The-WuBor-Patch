use skyline::hooks::*;
use smash::hash40;

#[repr(C)]
#[derive(Clone, Debug)]
struct CommonEffectPair {
    pub eff_name: u64,
    pub function_hash: u64,
    pub padding: u64,
}

// static mut EFFECT_MAP_POINTER: u64 = 0;

// #[skyline::hook(offset = 0x41a588, inline)]
// unsafe extern "C" fn effect_map_hook(ctx: &mut skyline::hooks::InlineCtx) {
//     println!("sup");
//     let original_table = *ctx.registers[0].x.as_ref() as *mut CommonEffectPair;
//     *(original_table.add(15)) = CommonEffectPair{eff_name: hash40("guard_crush"), function_hash: hash40("effect_guardcrush"), padding: 0};
//     *(original_table.add(16)) = CommonEffectPair{eff_name: hash40("burnout"), function_hash: hash40("effect_burnout"), padding: 0};
// }

pub fn install() {
    // // Patch common effect hashmap allocate size
    // let _ = skyline::patching::Patch::in_text(0x41a454).data(0x52803301u32);
    // let _ = skyline::patching::Patch::in_text(0x41a498).data(0x52803301u32);

    // // Patch common effect hashmap ending offset
    // let _ = skyline::patching::Patch::in_text(0x41a4c0).data(0x91066008u32);

    // skyline::install_hooks!(
    //     effect_map_hook
    // );

    unsafe {
        let original = (skyline::hooks::getRegionAddress(Region::Text) as *mut u8)
            .add(0x45412c8)
            .cast::<*mut CommonEffectPair>();
        let original_table = std::slice::from_raw_parts(*original, 15);
        let new_table = &mut Vec::new();
        new_table.extend_from_slice(original_table);
        new_table.push(CommonEffectPair {
            eff_name: hash40("guard_crush"),
            function_hash: hash40("effect_guardcrush"),
            padding: 0,
        });
        new_table.push(CommonEffectPair {
            eff_name: hash40("burnout"),
            function_hash: hash40("effect_burnout"),
            padding: 0,
        });
        *original = new_table.as_mut_ptr();
        *(original).add(1) = *original.add(17);
        *(original).add(2) = *original.add(17);
    }
}
