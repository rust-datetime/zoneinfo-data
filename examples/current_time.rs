extern crate datetime;
extern crate locale;
extern crate zoneinfo_data;
use datetime::instant::Instant;
use datetime::format::DateFormat;
use datetime::local::LocalDateTime;
use datetime::zoned::TimeZone;
use datetime::zoned::factory::current_timezone;
use zoneinfo_data::lookup;


fn main() {
    let now = Instant::now();
    println!("The current Unix timestamp is {}.", now.seconds());

    let format = DateFormat::parse("{2>:D} {:M} {:Y}, {:h}:{02>:m}:{02>:s}").unwrap();
    let datetime = LocalDateTime::from_instant(now);
    println!("This corresponds to {} in UTC.", format.format(&datetime, &locale::Time::english()));

    if let Some(timezone) = current_timezone() {
        println!("\nHowever, your current timezone is {}.", timezone);

        if let Some(zone) = lookup(&*timezone) {
            let offset = zone.offset(datetime);
            println!("This currently has an offset of {} (called {}).", offset, zone.name(datetime));

            let new_datetime = zone.to_zoned(datetime);
            println!("So the actual time is {}.", format.format(&new_datetime, &locale::Time::english()));
        }
        else {
            println!("Which isn't a known timezone! Weird.");
        }
    }
    else {
        println!("And your current timezone couldn't be found!");
    }
}
