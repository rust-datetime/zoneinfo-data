
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "Asia/Hong_Kong",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 27402,  // UTC offset 27402, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-2056693002, FixedTimespan {  // 1904-10-29T16:23:18.000 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("HKT"),
        }),
        (-907389000, FixedTimespan {  // 1941-03-31T19:30:00.000 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("HKST"),
        }),
        (-891667800, FixedTimespan {  // 1941-09-29T18:30:00.000 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("HKT"),
        }),
        (-884246400, FixedTimespan {  // 1941-12-24T16:00:00.000 UTC
            offset: 32400,  // UTC offset 32400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("JST"),
        }),
        (-766746000, FixedTimespan {  // 1945-09-14T15:00:00.000 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("HKT"),
        }),
        (-747981000, FixedTimespan {  // 1946-04-19T19:30:00.000 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("HKST"),
        }),
        (-728544600, FixedTimespan {  // 1946-11-30T18:30:00.000 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("HKT"),
        }),
        (-717049800, FixedTimespan {  // 1947-04-12T19:30:00.000 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("HKST"),
        }),
        (-694503000, FixedTimespan {  // 1947-12-29T18:30:00.000 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("HKT"),
        }),
        (-683785800, FixedTimespan {  // 1948-05-01T19:30:00.000 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("HKST"),
        }),
        (-668064600, FixedTimespan {  // 1948-10-30T18:30:00.000 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("HKT"),
        }),
        (-654755400, FixedTimespan {  // 1949-04-02T19:30:00.000 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("HKST"),
        }),
        (-636615000, FixedTimespan {  // 1949-10-29T18:30:00.000 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("HKT"),
        }),
        (-623305800, FixedTimespan {  // 1950-04-01T19:30:00.000 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("HKST"),
        }),
        (-605165400, FixedTimespan {  // 1950-10-28T18:30:00.000 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("HKT"),
        }),
        (-591856200, FixedTimespan {  // 1951-03-31T19:30:00.000 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("HKST"),
        }),
        (-573715800, FixedTimespan {  // 1951-10-27T18:30:00.000 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("HKT"),
        }),
        (-559801800, FixedTimespan {  // 1952-04-05T19:30:00.000 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("HKST"),
        }),
        (-542352600, FixedTimespan {  // 1952-10-24T18:30:00.000 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("HKT"),
        }),
        (-528352200, FixedTimespan {  // 1953-04-04T19:30:00.000 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("HKST"),
        }),
        (-510211800, FixedTimespan {  // 1953-10-31T18:30:00.000 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("HKT"),
        }),
        (-498112200, FixedTimespan {  // 1954-03-20T19:30:00.000 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("HKST"),
        }),
        (-478762200, FixedTimespan {  // 1954-10-30T18:30:00.000 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("HKT"),
        }),
        (-466662600, FixedTimespan {  // 1955-03-19T19:30:00.000 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("HKST"),
        }),
        (-446707800, FixedTimespan {  // 1955-11-05T18:30:00.000 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("HKT"),
        }),
        (-435213000, FixedTimespan {  // 1956-03-17T19:30:00.000 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("HKST"),
        }),
        (-415258200, FixedTimespan {  // 1956-11-03T18:30:00.000 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("HKT"),
        }),
        (-403158600, FixedTimespan {  // 1957-03-23T19:30:00.000 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("HKST"),
        }),
        (-383808600, FixedTimespan {  // 1957-11-02T18:30:00.000 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("HKT"),
        }),
        (-371709000, FixedTimespan {  // 1958-03-22T19:30:00.000 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("HKST"),
        }),
        (-352359000, FixedTimespan {  // 1958-11-01T18:30:00.000 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("HKT"),
        }),
        (-340259400, FixedTimespan {  // 1959-03-21T19:30:00.000 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("HKST"),
        }),
        (-320909400, FixedTimespan {  // 1959-10-31T18:30:00.000 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("HKT"),
        }),
        (-308809800, FixedTimespan {  // 1960-03-19T19:30:00.000 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("HKST"),
        }),
        (-288855000, FixedTimespan {  // 1960-11-05T18:30:00.000 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("HKT"),
        }),
        (-277360200, FixedTimespan {  // 1961-03-18T19:30:00.000 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("HKST"),
        }),
        (-257405400, FixedTimespan {  // 1961-11-04T18:30:00.000 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("HKT"),
        }),
        (-245910600, FixedTimespan {  // 1962-03-17T19:30:00.000 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("HKST"),
        }),
        (-225955800, FixedTimespan {  // 1962-11-03T18:30:00.000 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("HKT"),
        }),
        (-213856200, FixedTimespan {  // 1963-03-23T19:30:00.000 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("HKST"),
        }),
        (-194506200, FixedTimespan {  // 1963-11-02T18:30:00.000 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("HKT"),
        }),
        (-182406600, FixedTimespan {  // 1964-03-21T19:30:00.000 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("HKST"),
        }),
        (-163056600, FixedTimespan {  // 1964-10-31T18:30:00.000 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("HKT"),
        }),
        (-148537800, FixedTimespan {  // 1965-04-17T19:30:00.000 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("HKST"),
        }),
        (-132816600, FixedTimespan {  // 1965-10-16T18:30:00.000 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("HKT"),
        }),
        (-117088200, FixedTimespan {  // 1966-04-16T19:30:00.000 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("HKST"),
        }),
        (-101367000, FixedTimespan {  // 1966-10-15T18:30:00.000 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("HKT"),
        }),
        (-85638600, FixedTimespan {  // 1967-04-15T19:30:00.000 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("HKST"),
        }),
        (-69312600, FixedTimespan {  // 1967-10-21T18:30:00.000 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("HKT"),
        }),
        (-53584200, FixedTimespan {  // 1968-04-20T19:30:00.000 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("HKST"),
        }),
        (-37863000, FixedTimespan {  // 1968-10-19T18:30:00.000 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("HKT"),
        }),
        (-22134600, FixedTimespan {  // 1969-04-19T19:30:00.000 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("HKST"),
        }),
        (-6413400, FixedTimespan {  // 1969-10-18T18:30:00.000 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("HKT"),
        }),
        (9315000, FixedTimespan {  // 1970-04-18T19:30:00.000 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("HKST"),
        }),
        (25036200, FixedTimespan {  // 1970-10-17T18:30:00.000 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("HKT"),
        }),
        (40764600, FixedTimespan {  // 1971-04-17T19:30:00.000 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("HKST"),
        }),
        (56485800, FixedTimespan {  // 1971-10-16T18:30:00.000 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("HKT"),
        }),
        (72214200, FixedTimespan {  // 1972-04-15T19:30:00.000 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("HKST"),
        }),
        (88540200, FixedTimespan {  // 1972-10-21T18:30:00.000 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("HKT"),
        }),
        (104268600, FixedTimespan {  // 1973-04-21T19:30:00.000 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("HKST"),
        }),
        (119989800, FixedTimespan {  // 1973-10-20T18:30:00.000 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("HKT"),
        }),
        (126041400, FixedTimespan {  // 1973-12-29T19:30:00.000 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("HKST"),
        }),
        (151439400, FixedTimespan {  // 1974-10-19T18:30:00.000 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("HKT"),
        }),
        (167167800, FixedTimespan {  // 1975-04-19T19:30:00.000 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("HKST"),
        }),
        (182889000, FixedTimespan {  // 1975-10-18T18:30:00.000 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("HKT"),
        }),
        (198617400, FixedTimespan {  // 1976-04-17T19:30:00.000 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("HKST"),
        }),
        (214338600, FixedTimespan {  // 1976-10-16T18:30:00.000 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("HKT"),
        }),
        (295385400, FixedTimespan {  // 1979-05-12T19:30:00.000 UTC
            offset: 32400,  // UTC offset 28800, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("HKST"),
        }),
        (309292200, FixedTimespan {  // 1979-10-20T18:30:00.000 UTC
            offset: 28800,  // UTC offset 28800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("HKT"),
        }),
    ]},
};


