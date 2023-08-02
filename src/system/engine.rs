use crate::imports::status_imports::*;

// Only extra elec hitlag for hit character
#[skyline::hook(offset = 0x406804, inline)]
unsafe fn change_elec_hitlag_for_attacker(ctx: &mut skyline::hooks::InlineCtx) {
  let is_attacker = *ctx.registers[4].w.as_ref() & 1 == 0;
  if *ctx.registers[8].x.as_ref() == hash40("collision_attr_elec") && is_attacker {
    *ctx.registers[8].x.as_mut() = hash40("collision_attr_normal");
  }
}

pub fn install() {
    skyline::install_hooks!(
        change_elec_hitlag_for_attacker
    );

    // Removes Phantom Hits
    skyline::patching::Patch::in_text(0x3e6ce8).data(0x14000012u32);
}