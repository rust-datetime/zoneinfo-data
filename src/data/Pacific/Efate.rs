
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "Pacific/Efate",
    timespans: &[
        Timespan {
            offset: 40396,
            format: "LMT",
            saving: Saving::NoSaving,
            start_time: None,
            end_time:   Some(-1829347200),
        },
        Timespan {
            offset: 39600,
            format: "VU%sT",
            saving: Saving::Multiple(&rulesets::Vanuatu),
            start_time: Some(-1829347200),
            end_time:   None,
        },
    ],
};

