
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "Asia/Ashgabat",
    timespans: &[
        Timespan {
            offset: 14012,
            format: "LMT",
            saving: Saving::NoSaving,
            start_time: None,
            end_time:   Some(-1441152000),
        },
        Timespan {
            offset: 14400,
            format: "ASHT",
            saving: Saving::NoSaving,
            start_time: Some(-1441152000),
            end_time:   Some(-1247529600),
        },
        Timespan {
            offset: 18000,
            format: "ASH%sT",
            saving: Saving::Multiple(&rulesets::RussiaAsia),
            start_time: Some(-1247529600),
            end_time:   Some(670384800),
        },
        Timespan {
            offset: 14400,
            format: "ASH%sT",
            saving: Saving::Multiple(&rulesets::RussiaAsia),
            start_time: Some(670384800),
            end_time:   Some(688521600),
        },
        Timespan {
            offset: 14400,
            format: "TM%sT",
            saving: Saving::Multiple(&rulesets::RussiaAsia),
            start_time: Some(688521600),
            end_time:   Some(695786400),
        },
        Timespan {
            offset: 18000,
            format: "TMT",
            saving: Saving::NoSaving,
            start_time: Some(695786400),
            end_time:   None,
        },
    ],
};

