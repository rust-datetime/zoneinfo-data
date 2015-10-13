
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "America/Ojinaga",
    timespans: &[
        Timespan {
            offset: -18140,
            format: "LMT",
            saving: Saving::NoSaving,
            start_time: None,
            end_time:   Some(-1514764660),
        },
        Timespan {
            offset: -25200,
            format: "MST",
            saving: Saving::NoSaving,
            start_time: Some(-1514764660),
            end_time:   Some(-1343091600),
        },
        Timespan {
            offset: -21600,
            format: "CST",
            saving: Saving::NoSaving,
            start_time: Some(-1343091600),
            end_time:   Some(-1234828800),
        },
        Timespan {
            offset: -25200,
            format: "MST",
            saving: Saving::NoSaving,
            start_time: Some(-1234828800),
            end_time:   Some(-1220317200),
        },
        Timespan {
            offset: -21600,
            format: "CST",
            saving: Saving::NoSaving,
            start_time: Some(-1220317200),
            end_time:   Some(-1207180800),
        },
        Timespan {
            offset: -25200,
            format: "MST",
            saving: Saving::NoSaving,
            start_time: Some(-1207180800),
            end_time:   Some(-1191369600),
        },
        Timespan {
            offset: -21600,
            format: "CST",
            saving: Saving::NoSaving,
            start_time: Some(-1191369600),
            end_time:   Some(820454400),
        },
        Timespan {
            offset: -21600,
            format: "C%sT",
            saving: Saving::Multiple(&rulesets::Mexico),
            start_time: Some(820454400),
            end_time:   Some(883612800),
        },
        Timespan {
            offset: -21600,
            format: "CST",
            saving: Saving::NoSaving,
            start_time: Some(883612800),
            end_time:   Some(891745200),
        },
        Timespan {
            offset: -25200,
            format: "M%sT",
            saving: Saving::Multiple(&rulesets::Mexico),
            start_time: Some(891745200),
            end_time:   Some(1262304000),
        },
        Timespan {
            offset: -25200,
            format: "M%sT",
            saving: Saving::Multiple(&rulesets::US),
            start_time: Some(1262304000),
            end_time:   None,
        },
    ],
};

