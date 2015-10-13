
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "Asia/Almaty",
    timespans: &[
        Timespan {
            offset: 18468,
            format: "LMT",
            saving: Saving::NoSaving,
            start_time: None,
            end_time:   Some(-1441152000),
        },
        Timespan {
            offset: 18000,
            format: "ALMT",
            saving: Saving::NoSaving,
            start_time: Some(-1441152000),
            end_time:   Some(-1247529600),
        },
        Timespan {
            offset: 21600,
            format: "ALM%sT",
            saving: Saving::Multiple(&rulesets::RussiaAsia),
            start_time: Some(-1247529600),
            end_time:   Some(662688000),
        },
        Timespan {
            offset: 21600,
            format: "ALMT",
            saving: Saving::NoSaving,
            start_time: Some(662688000),
            end_time:   Some(694224000),
        },
        Timespan {
            offset: 21600,
            format: "ALM%sT",
            saving: Saving::Multiple(&rulesets::RussiaAsia),
            start_time: Some(694224000),
            end_time:   Some(1110844800),
        },
        Timespan {
            offset: 21600,
            format: "ALMT",
            saving: Saving::NoSaving,
            start_time: Some(1110844800),
            end_time:   None,
        },
    ],
};


