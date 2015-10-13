
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "America/Rankin_Inlet",
    timespans: &[
        Timespan {
            offset: 0,
            format: "zzz",
            saving: Saving::NoSaving,
            start_time: None,
            end_time:   Some(-410227200),
        },
        Timespan {
            offset: -21600,
            format: "C%sT",
            saving: Saving::Multiple(&rulesets::NT_YK),
            start_time: Some(-410227200),
            end_time:   Some(972784800),
        },
        Timespan {
            offset: -18000,
            format: "EST",
            saving: Saving::NoSaving,
            start_time: Some(972784800),
            end_time:   Some(986094000),
        },
        Timespan {
            offset: -21600,
            format: "C%sT",
            saving: Saving::Multiple(&rulesets::Canada),
            start_time: Some(986094000),
            end_time:   None,
        },
    ],
};


