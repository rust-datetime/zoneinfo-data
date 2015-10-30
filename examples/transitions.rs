extern crate zoneinfo_data;

fn main() {
    let zone = zoneinfo_data::Africa::Tripoli;
    for transition in zone.transitions {
        println!("    Transition {{ occurs_at: {:?},   offset: {} }},",
                 transition.occurs_at, transition.offset);
    }
}
