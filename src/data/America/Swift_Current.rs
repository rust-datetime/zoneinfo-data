
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "America/Swift_Current",
    timespans: &[
        Timespan {
            offset: -24520,
            format: "LMT",
            saving: Saving::NoSaving,
            start_time: None,
            end_time:   Some(-2030227200),
        },
        Timespan {
            offset: -25200,
            format: "M%sT",
            saving: Saving::Multiple(&rulesets::Canada),
            start_time: Some(-2030227200),
            end_time:   Some(-747266400),
        },
        Timespan {
            offset: -25200,
            format: "M%sT",
            saving: Saving::Multiple(&rulesets::Regina),
            start_time: Some(-747266400),
            end_time:   Some(-631152000),
        },
        Timespan {
            offset: -25200,
            format: "M%sT",
            saving: Saving::Multiple(&rulesets::Swift),
            start_time: Some(-631152000),
            end_time:   Some(73447200),
        },
        Timespan {
            offset: -21600,
            format: "CST",
            saving: Saving::NoSaving,
            start_time: Some(73447200),
            end_time:   None,
        },
    ],
};

