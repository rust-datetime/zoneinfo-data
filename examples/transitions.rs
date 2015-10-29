extern crate zoneinfo_data;

fn main() {
    let zone = zoneinfo_data::Africa::Tripoli;
    for transition in zone.transitions {
        println!("    Transition {{ occurs_at: {:?},   utc_offset: {}, std_offset: {} }},",
                 transition.occurs_at, transition.utc_offset, transition.dst_offset);
    }
}
