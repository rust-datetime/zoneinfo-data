
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "Atlantic/Stanley",
    transitions: &[
        Transition { occurs_at: None, utc_offset: -7716, dst_offset: 0, name: "LMT" },
        Transition { occurs_at: Some(-1824241884), utc_offset: -14400, dst_offset: 0, name: "FKT" },
        Transition { occurs_at: Some(-1018209600), utc_offset: -14400, dst_offset: 3600, name: "FKST" },
        Transition { occurs_at: Some(-1003093200), utc_offset: -14400, dst_offset: 0, name: "FKT" },
        Transition { occurs_at: Some(-986760000), utc_offset: -14400, dst_offset: 3600, name: "FKST" },
        Transition { occurs_at: Some(-971643600), utc_offset: -14400, dst_offset: 0, name: "FKT" },
        Transition { occurs_at: Some(-954705600), utc_offset: -14400, dst_offset: 3600, name: "FKST" },
        Transition { occurs_at: Some(-939589200), utc_offset: -14400, dst_offset: 0, name: "FKT" },
        Transition { occurs_at: Some(-923256000), utc_offset: -14400, dst_offset: 3600, name: "FKST" },
        Transition { occurs_at: Some(-908139600), utc_offset: -14400, dst_offset: 0, name: "FKT" },
        Transition { occurs_at: Some(-891806400), utc_offset: -14400, dst_offset: 3600, name: "FKST" },
        Transition { occurs_at: Some(-876690000), utc_offset: -14400, dst_offset: 0, name: "FKT" },
        Transition { occurs_at: Some(-860356800), utc_offset: -14400, dst_offset: 3600, name: "FKST" },
        Transition { occurs_at: Some(-852066000), utc_offset: -14400, dst_offset: 0, name: "FKT" },
        Transition { occurs_at: Some(420609600), utc_offset: -10800, dst_offset: 0, name: "FKT" },
        Transition { occurs_at: Some(433306800), utc_offset: -10800, dst_offset: 3600, name: "FKST" },
        Transition { occurs_at: Some(452052000), utc_offset: -10800, dst_offset: 0, name: "FKT" },
        Transition { occurs_at: Some(464151600), utc_offset: -10800, dst_offset: 3600, name: "FKST" },
        Transition { occurs_at: Some(483501600), utc_offset: -10800, dst_offset: 0, name: "FKT" },
        Transition { occurs_at: Some(495601200), utc_offset: -14400, dst_offset: 3600, name: "FKST" },
        Transition { occurs_at: Some(514350000), utc_offset: -14400, dst_offset: 0, name: "FKT" },
        Transition { occurs_at: Some(527054400), utc_offset: -14400, dst_offset: 3600, name: "FKST" },
        Transition { occurs_at: Some(545799600), utc_offset: -14400, dst_offset: 0, name: "FKT" },
        Transition { occurs_at: Some(558504000), utc_offset: -14400, dst_offset: 3600, name: "FKST" },
        Transition { occurs_at: Some(577249200), utc_offset: -14400, dst_offset: 0, name: "FKT" },
        Transition { occurs_at: Some(589953600), utc_offset: -14400, dst_offset: 3600, name: "FKST" },
        Transition { occurs_at: Some(608698800), utc_offset: -14400, dst_offset: 0, name: "FKT" },
        Transition { occurs_at: Some(621403200), utc_offset: -14400, dst_offset: 3600, name: "FKST" },
        Transition { occurs_at: Some(640753200), utc_offset: -14400, dst_offset: 0, name: "FKT" },
        Transition { occurs_at: Some(652852800), utc_offset: -14400, dst_offset: 3600, name: "FKST" },
        Transition { occurs_at: Some(672202800), utc_offset: -14400, dst_offset: 0, name: "FKT" },
        Transition { occurs_at: Some(684907200), utc_offset: -14400, dst_offset: 3600, name: "FKST" },
        Transition { occurs_at: Some(703652400), utc_offset: -14400, dst_offset: 0, name: "FKT" },
        Transition { occurs_at: Some(716356800), utc_offset: -14400, dst_offset: 3600, name: "FKST" },
        Transition { occurs_at: Some(735102000), utc_offset: -14400, dst_offset: 0, name: "FKT" },
        Transition { occurs_at: Some(747806400), utc_offset: -14400, dst_offset: 3600, name: "FKST" },
        Transition { occurs_at: Some(766551600), utc_offset: -14400, dst_offset: 0, name: "FKT" },
        Transition { occurs_at: Some(779256000), utc_offset: -14400, dst_offset: 3600, name: "FKST" },
        Transition { occurs_at: Some(798001200), utc_offset: -14400, dst_offset: 0, name: "FKT" },
        Transition { occurs_at: Some(810705600), utc_offset: -14400, dst_offset: 3600, name: "FKST" },
        Transition { occurs_at: Some(830055600), utc_offset: -14400, dst_offset: 0, name: "FKT" },
        Transition { occurs_at: Some(842760000), utc_offset: -14400, dst_offset: 3600, name: "FKST" },
        Transition { occurs_at: Some(861505200), utc_offset: -14400, dst_offset: 0, name: "FKT" },
        Transition { occurs_at: Some(874209600), utc_offset: -14400, dst_offset: 3600, name: "FKST" },
        Transition { occurs_at: Some(892954800), utc_offset: -14400, dst_offset: 0, name: "FKT" },
        Transition { occurs_at: Some(905659200), utc_offset: -14400, dst_offset: 3600, name: "FKST" },
        Transition { occurs_at: Some(924404400), utc_offset: -14400, dst_offset: 0, name: "FKT" },
        Transition { occurs_at: Some(937108800), utc_offset: -14400, dst_offset: 3600, name: "FKST" },
        Transition { occurs_at: Some(955854000), utc_offset: -14400, dst_offset: 0, name: "FKT" },
        Transition { occurs_at: Some(968558400), utc_offset: -14400, dst_offset: 3600, name: "FKST" },
        Transition { occurs_at: Some(987310800), utc_offset: -14400, dst_offset: 0, name: "FKT" },
        Transition { occurs_at: Some(999410400), utc_offset: -14400, dst_offset: 3600, name: "FKST" },
        Transition { occurs_at: Some(1019365200), utc_offset: -14400, dst_offset: 0, name: "FKT" },
        Transition { occurs_at: Some(1030860000), utc_offset: -14400, dst_offset: 3600, name: "FKST" },
        Transition { occurs_at: Some(1050814800), utc_offset: -14400, dst_offset: 0, name: "FKT" },
        Transition { occurs_at: Some(1062914400), utc_offset: -14400, dst_offset: 3600, name: "FKST" },
        Transition { occurs_at: Some(1082264400), utc_offset: -14400, dst_offset: 0, name: "FKT" },
        Transition { occurs_at: Some(1094364000), utc_offset: -14400, dst_offset: 3600, name: "FKST" },
        Transition { occurs_at: Some(1113714000), utc_offset: -14400, dst_offset: 0, name: "FKT" },
        Transition { occurs_at: Some(1125813600), utc_offset: -14400, dst_offset: 3600, name: "FKST" },
        Transition { occurs_at: Some(1145163600), utc_offset: -14400, dst_offset: 0, name: "FKT" },
        Transition { occurs_at: Some(1157263200), utc_offset: -14400, dst_offset: 3600, name: "FKST" },
        Transition { occurs_at: Some(1176613200), utc_offset: -14400, dst_offset: 0, name: "FKT" },
        Transition { occurs_at: Some(1188712800), utc_offset: -14400, dst_offset: 3600, name: "FKST" },
        Transition { occurs_at: Some(1208667600), utc_offset: -14400, dst_offset: 0, name: "FKT" },
        Transition { occurs_at: Some(1220767200), utc_offset: -14400, dst_offset: 3600, name: "FKST" },
        Transition { occurs_at: Some(1240117200), utc_offset: -14400, dst_offset: 0, name: "FKT" },
        Transition { occurs_at: Some(1252216800), utc_offset: -14400, dst_offset: 3600, name: "FKST" },
        Transition { occurs_at: Some(1271566800), utc_offset: -14400, dst_offset: 0, name: "FKT" },
        Transition { occurs_at: Some(1283666400), utc_offset: -10800, dst_offset: 0, name: "FKST" },
    ],
};


