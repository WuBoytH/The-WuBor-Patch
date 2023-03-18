use {
    wubor_utils::vars::*,
    skyline::hooks::{
        getRegionAddress,
        Region
    }
};

mod fgc_reset;
mod init_settings;
mod get_val;
mod get_param;
mod transition;
mod collision_hit;
mod handle_damage;
mod sound;
pub mod func_links;
mod fighter_util;
mod menu;
mod music;

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

pub fn install() {
    unsafe{
        let text_ptr = getRegionAddress(Region::Text) as *const u8;
        let text_size = (getRegionAddress(Region::Rodata) as usize) - (text_ptr as usize);
        let text = std::slice::from_raw_parts(text_ptr, text_size);
        if let Some(offset) = find_subsequence(text, FLOAT_SEARCH_CODE) {
            FLOAT_OFFSET = offset;
        }
        if let Some(offset) = find_subsequence(text, INT_SEARCH_CODE) {
            INT_OFFSET = offset;
        }
        if let Some(offset) = find_subsequence(text, NOTIFY_LOG_EVENT_COLLISION_HIT_SEARCH_CODE) {
            NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET = offset;
        }
    }
    fgc_reset::install();
    init_settings::install();
    get_val::install();
    get_param::install();
    transition::install();
    collision_hit::install();
    handle_damage::install();
    sound::install();
    fighter_util::install();
    menu::install();
    music::install();
    
    // Removes Phantom Hits
    skyline::patching::Patch::in_text(0x3e6ce8).data(0x14000012u32);
}