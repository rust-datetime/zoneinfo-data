
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "Europe/Minsk",
    timespans: &[
        Timespan {
            offset: 6616,
            format: "LMT",
            saving: Saving::NoSaving,
            start_time: None,
            end_time:   Some(-2840140800),
        },
        Timespan {
            offset: 6600,
            format: "MMT",
            saving: Saving::NoSaving,
            start_time: Some(-2840140800),
            end_time:   Some(-1441152000),
        },
        Timespan {
            offset: 7200,
            format: "EET",
            saving: Saving::NoSaving,
            start_time: Some(-1441152000),
            end_time:   Some(-1247529600),
        },
        Timespan {
            offset: 10800,
            format: "MSK",
            saving: Saving::NoSaving,
            start_time: Some(-1247529600),
            end_time:   Some(-899769600),
        },
        Timespan {
            offset: 3600,
            format: "CE%sT",
            saving: Saving::Multiple(&rulesets::C_Eur),
            start_time: Some(-899769600),
            end_time:   Some(-804643200),
        },
        Timespan {
            offset: 10800,
            format: "MSK/MSD",
            saving: Saving::Multiple(&rulesets::Russia),
            start_time: Some(-804643200),
            end_time:   Some(631152000),
        },
        Timespan {
            offset: 10800,
            format: "MSK",
            saving: Saving::NoSaving,
            start_time: Some(631152000),
            end_time:   Some(670384800),
        },
        Timespan {
            offset: 7200,
            format: "EEST",
            saving: Saving::OneOff(3600),
            start_time: Some(670384800),
            end_time:   Some(686109600),
        },
        Timespan {
            offset: 7200,
            format: "EET",
            saving: Saving::NoSaving,
            start_time: Some(686109600),
            end_time:   Some(701827200),
        },
        Timespan {
            offset: 7200,
            format: "EEST",
            saving: Saving::OneOff(3600),
            start_time: Some(701827200),
            end_time:   Some(717552000),
        },
        Timespan {
            offset: 7200,
            format: "EE%sT",
            saving: Saving::Multiple(&rulesets::Russia),
            start_time: Some(717552000),
            end_time:   Some(1301191200),
        },
        Timespan {
            offset: 10800,
            format: "FET",
            saving: Saving::NoSaving,
            start_time: Some(1301191200),
            end_time:   Some(1414285200),
        },
        Timespan {
            offset: 10800,
            format: "MSK",
            saving: Saving::NoSaving,
            start_time: Some(1414285200),
            end_time:   None,
        },
    ],
};


