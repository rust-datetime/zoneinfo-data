extern crate zoneinfo_data;

extern crate datetime;
use datetime::instant::Instant;
use datetime::local::{LocalDateTime, DatePiece, TimePiece};

fn main() {
    let zone = zoneinfo_data::Africa::Algiers;
    for transition in zone.forward_transitions(LocalDateTime::from_instant(Instant::at(0))).take(150) {
        let at = transition.occurs_at;
        println!("At {}\t({} {} {})\t({}:{}:{}) GMT offset {}\tDST offset {}",
            at.to_instant().seconds(),
            at.year(), &format!("{:?}", at.month())[0..3], at.day(),
            at.hour(), at.minute(), at.second(),
            transition.gmt_offset, transition.dst_offset
        );
    }
}
