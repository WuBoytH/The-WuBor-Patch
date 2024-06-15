#[allow(unused_imports)]
use {
    smash::{
        lua2cpp::*,
        hash40,
        phx::*,
        app::{lua_bind::*, sv_animcmd::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    smash_script::*,
    custom_var::*,
    custom_cancel::*,
    wubor_utils::{app::*, cancels::*, wua_bind::*, vars, table_const::*}
};

mod acmd;
mod frame;
mod status;
mod agent_init;
pub mod vl;

// static RYU_COPY_MOTIONS : [u64; 6] = [
//     0xd5899ddb0, 0x915c5de42,
//     0x11787641d2, 0xd483c0ed2,
//     0xed1ec945c, 0xa82125b6f,
//     0x1272707bb4, 0xe724031fb,
//     // Pad with the beginning of the next table
//     0xdefce9270, 0x915c5de42
// ];

pub fn install() {
    let agent = &mut Agent::new("kirby");
    acmd::install(agent);
    frame::install(agent);
    status::install(agent);
    agent_init::install(agent);
    agent.install();

    // MiscModule::patch_vtable_function(0x52c1950, RYU_COPY_MOTIONS.as_ptr() as u64);
    // let _ = skyline::patching::Patch::in_text(0x413698).data(0x321A03E1u32);
    // let _ = skyline::patching::Patch::in_text(0x4136d8).data(0x321A03E1u32);

    // let _ = skyline::patching::Patch::in_text(0x4136f8).data(0x91010008u32);
    // unsafe {
    //     let text = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64 + 0x52c1950;
    //     let ryu_copy_table = *(text as *mut *mut u64);
    //     println!("table: {:#?}", ryu_copy_table);
    //     for index in 0..16 {
    //         println!("{:#x}", *ryu_copy_table.add(index));
    //     }
    //     // let mut ryu_copy_motion = RYU_COPY_MOTIONS.as_ptr();
    //     // println!("motions: {:#?}", ryu_copy_motion);
    //     // while *ryu_copy_motion != 0 {
    //     //     println!("{:#x}", *ryu_copy_motion);
    //     //     ryu_copy_motion = ryu_copy_motion.add(1);
    //     // }
    // }
}