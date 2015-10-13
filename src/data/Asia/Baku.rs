
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "Asia/Baku",
    timespans: &[
        Timespan {
            offset: 11964,
            format: "LMT",
            saving: Saving::NoSaving,
            start_time: None,
            end_time:   Some(-1441152000),
        },
        Timespan {
            offset: 10800,
            format: "BAKT",
            saving: Saving::NoSaving,
            start_time: Some(-1441152000),
            end_time:   Some(-405129600),
        },
        Timespan {
            offset: 14400,
            format: "BAK%sT",
            saving: Saving::Multiple(&rulesets::RussiaAsia),
            start_time: Some(-405129600),
            end_time:   Some(670384800),
        },
        Timespan {
            offset: 10800,
            format: "BAKST",
            saving: Saving::OneOff(3600),
            start_time: Some(670384800),
            end_time:   Some(683510400),
        },
        Timespan {
            offset: 10800,
            format: "AZ%sT",
            saving: Saving::Multiple(&rulesets::RussiaAsia),
            start_time: Some(683510400),
            end_time:   Some(717548400),
        },
        Timespan {
            offset: 14400,
            format: "AZT",
            saving: Saving::NoSaving,
            start_time: Some(717548400),
            end_time:   Some(820454400),
        },
        Timespan {
            offset: 14400,
            format: "AZ%sT",
            saving: Saving::Multiple(&rulesets::EUAsia),
            start_time: Some(820454400),
            end_time:   Some(852076800),
        },
        Timespan {
            offset: 14400,
            format: "AZ%sT",
            saving: Saving::Multiple(&rulesets::Azer),
            start_time: Some(852076800),
            end_time:   None,
        },
    ],
};


