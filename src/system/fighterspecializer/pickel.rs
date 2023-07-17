#[skyline::hook( offset = 0xf10660 )]
pub unsafe fn is_mining_material_table_normal() -> bool {
    false
}

pub fn install() {
    skyline::install_hooks!(
        is_mining_material_table_normal
    );
    
}