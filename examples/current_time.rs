extern crate datetime;
use datetime::{Instant, LocalDateTime, TimeZone};
use datetime::format::DateFormat;

extern crate zoneinfo_data;
use zoneinfo_data::ZoneinfoData;

extern crate locale;


fn main() {
    let now = Instant::now();
    println!("The current Unix timestamp is {}.", now.seconds());

    let format = DateFormat::parse("{2>:D} {:M} {:Y}, {:h}:{02>:m}:{02>:s}").unwrap();
    let datetime = LocalDateTime::from_instant(now);
    println!("This corresponds to {} in UTC.", format.format(&datetime, &locale::Time::english()));

    if let Some(timezone) = TimeZone::system() {
        println!("\nHowever, your current timezone is {}.", timezone.zone_name());

        let offset = timezone.offset(datetime);
        println!("This currently has an offset of {} (called {}).", offset, timezone.name(datetime));

        let new_datetime = timezone.to_zoned(datetime);
        println!("So the actual time is {}.", format.format(&new_datetime, &locale::Time::english()));
    }
    else {
        println!("And your current timezone couldn't be found!");
    }
}
