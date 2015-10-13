
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "Africa/Algiers",
    timespans: &[
        Timespan {
            offset: 732,
            format: "LMT",
            saving: Saving::NoSaving,
            start_time: None,
            end_time:   Some(-2486678340),
        },
        Timespan {
            offset: 561,
            format: "PMT",
            saving: Saving::NoSaving,
            start_time: Some(-2486678340),
            end_time:   Some(-1855958400),
        },
        Timespan {
            offset: 0,
            format: "WE%sT",
            saving: Saving::Multiple(&rulesets::Algeria),
            start_time: Some(-1855958400),
            end_time:   Some(-942012000),
        },
        Timespan {
            offset: 3600,
            format: "CE%sT",
            saving: Saving::Multiple(&rulesets::Algeria),
            start_time: Some(-942012000),
            end_time:   Some(-733276800),
        },
        Timespan {
            offset: 0,
            format: "WET",
            saving: Saving::NoSaving,
            start_time: Some(-733276800),
            end_time:   Some(-439430400),
        },
        Timespan {
            offset: 3600,
            format: "CET",
            saving: Saving::NoSaving,
            start_time: Some(-439430400),
            end_time:   Some(-212025600),
        },
        Timespan {
            offset: 0,
            format: "WE%sT",
            saving: Saving::Multiple(&rulesets::Algeria),
            start_time: Some(-212025600),
            end_time:   Some(246240000),
        },
        Timespan {
            offset: 3600,
            format: "CE%sT",
            saving: Saving::Multiple(&rulesets::Algeria),
            start_time: Some(246240000),
            end_time:   Some(309744000),
        },
        Timespan {
            offset: 0,
            format: "WE%sT",
            saving: Saving::Multiple(&rulesets::Algeria),
            start_time: Some(309744000),
            end_time:   Some(357523200),
        },
        Timespan {
            offset: 3600,
            format: "CET",
            saving: Saving::NoSaving,
            start_time: Some(357523200),
            end_time:   None,
        },
    ],
};


