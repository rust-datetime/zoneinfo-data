
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "Asia/Hong_Kong",
    transitions: &[
        Transition { occurs_at: None, utc_offset: 27402, dst_offset: 0, name: "LMT" },
        Transition { occurs_at: Some(-2056693002), utc_offset: 28800, dst_offset: 0, name: "HKT" },
        Transition { occurs_at: Some(-907389000), utc_offset: 28800, dst_offset: 3600, name: "HKST" },
        Transition { occurs_at: Some(-891667800), utc_offset: 28800, dst_offset: 0, name: "HKT" },
        Transition { occurs_at: Some(-884246400), utc_offset: 32400, dst_offset: 0, name: "JST" },
        Transition { occurs_at: Some(-766746000), utc_offset: 28800, dst_offset: 0, name: "HKT" },
        Transition { occurs_at: Some(-747981000), utc_offset: 28800, dst_offset: 3600, name: "HKST" },
        Transition { occurs_at: Some(-728544600), utc_offset: 28800, dst_offset: 0, name: "HKT" },
        Transition { occurs_at: Some(-717049800), utc_offset: 28800, dst_offset: 3600, name: "HKST" },
        Transition { occurs_at: Some(-694503000), utc_offset: 28800, dst_offset: 0, name: "HKT" },
        Transition { occurs_at: Some(-683785800), utc_offset: 28800, dst_offset: 3600, name: "HKST" },
        Transition { occurs_at: Some(-668064600), utc_offset: 28800, dst_offset: 0, name: "HKT" },
        Transition { occurs_at: Some(-654755400), utc_offset: 28800, dst_offset: 3600, name: "HKST" },
        Transition { occurs_at: Some(-636615000), utc_offset: 28800, dst_offset: 0, name: "HKT" },
        Transition { occurs_at: Some(-623305800), utc_offset: 28800, dst_offset: 3600, name: "HKST" },
        Transition { occurs_at: Some(-605165400), utc_offset: 28800, dst_offset: 0, name: "HKT" },
        Transition { occurs_at: Some(-591856200), utc_offset: 28800, dst_offset: 3600, name: "HKST" },
        Transition { occurs_at: Some(-573715800), utc_offset: 28800, dst_offset: 0, name: "HKT" },
        Transition { occurs_at: Some(-559801800), utc_offset: 28800, dst_offset: 3600, name: "HKST" },
        Transition { occurs_at: Some(-542352600), utc_offset: 28800, dst_offset: 0, name: "HKT" },
        Transition { occurs_at: Some(-528352200), utc_offset: 28800, dst_offset: 3600, name: "HKST" },
        Transition { occurs_at: Some(-510211800), utc_offset: 28800, dst_offset: 0, name: "HKT" },
        Transition { occurs_at: Some(-498112200), utc_offset: 28800, dst_offset: 3600, name: "HKST" },
        Transition { occurs_at: Some(-478762200), utc_offset: 28800, dst_offset: 0, name: "HKT" },
        Transition { occurs_at: Some(-466662600), utc_offset: 28800, dst_offset: 3600, name: "HKST" },
        Transition { occurs_at: Some(-446707800), utc_offset: 28800, dst_offset: 0, name: "HKT" },
        Transition { occurs_at: Some(-435213000), utc_offset: 28800, dst_offset: 3600, name: "HKST" },
        Transition { occurs_at: Some(-415258200), utc_offset: 28800, dst_offset: 0, name: "HKT" },
        Transition { occurs_at: Some(-403158600), utc_offset: 28800, dst_offset: 3600, name: "HKST" },
        Transition { occurs_at: Some(-383808600), utc_offset: 28800, dst_offset: 0, name: "HKT" },
        Transition { occurs_at: Some(-371709000), utc_offset: 28800, dst_offset: 3600, name: "HKST" },
        Transition { occurs_at: Some(-352359000), utc_offset: 28800, dst_offset: 0, name: "HKT" },
        Transition { occurs_at: Some(-340259400), utc_offset: 28800, dst_offset: 3600, name: "HKST" },
        Transition { occurs_at: Some(-320909400), utc_offset: 28800, dst_offset: 0, name: "HKT" },
        Transition { occurs_at: Some(-308809800), utc_offset: 28800, dst_offset: 3600, name: "HKST" },
        Transition { occurs_at: Some(-288855000), utc_offset: 28800, dst_offset: 0, name: "HKT" },
        Transition { occurs_at: Some(-277360200), utc_offset: 28800, dst_offset: 3600, name: "HKST" },
        Transition { occurs_at: Some(-257405400), utc_offset: 28800, dst_offset: 0, name: "HKT" },
        Transition { occurs_at: Some(-245910600), utc_offset: 28800, dst_offset: 3600, name: "HKST" },
        Transition { occurs_at: Some(-225955800), utc_offset: 28800, dst_offset: 0, name: "HKT" },
        Transition { occurs_at: Some(-213856200), utc_offset: 28800, dst_offset: 3600, name: "HKST" },
        Transition { occurs_at: Some(-194506200), utc_offset: 28800, dst_offset: 0, name: "HKT" },
        Transition { occurs_at: Some(-182406600), utc_offset: 28800, dst_offset: 3600, name: "HKST" },
        Transition { occurs_at: Some(-163056600), utc_offset: 28800, dst_offset: 0, name: "HKT" },
        Transition { occurs_at: Some(-148537800), utc_offset: 28800, dst_offset: 3600, name: "HKST" },
        Transition { occurs_at: Some(-132816600), utc_offset: 28800, dst_offset: 0, name: "HKT" },
        Transition { occurs_at: Some(-117088200), utc_offset: 28800, dst_offset: 3600, name: "HKST" },
        Transition { occurs_at: Some(-101367000), utc_offset: 28800, dst_offset: 0, name: "HKT" },
        Transition { occurs_at: Some(-85638600), utc_offset: 28800, dst_offset: 3600, name: "HKST" },
        Transition { occurs_at: Some(-69312600), utc_offset: 28800, dst_offset: 0, name: "HKT" },
        Transition { occurs_at: Some(-53584200), utc_offset: 28800, dst_offset: 3600, name: "HKST" },
        Transition { occurs_at: Some(-37863000), utc_offset: 28800, dst_offset: 0, name: "HKT" },
        Transition { occurs_at: Some(-22134600), utc_offset: 28800, dst_offset: 3600, name: "HKST" },
        Transition { occurs_at: Some(-6413400), utc_offset: 28800, dst_offset: 0, name: "HKT" },
        Transition { occurs_at: Some(9315000), utc_offset: 28800, dst_offset: 3600, name: "HKST" },
        Transition { occurs_at: Some(25036200), utc_offset: 28800, dst_offset: 0, name: "HKT" },
        Transition { occurs_at: Some(40764600), utc_offset: 28800, dst_offset: 3600, name: "HKST" },
        Transition { occurs_at: Some(56485800), utc_offset: 28800, dst_offset: 0, name: "HKT" },
        Transition { occurs_at: Some(72214200), utc_offset: 28800, dst_offset: 3600, name: "HKST" },
        Transition { occurs_at: Some(88540200), utc_offset: 28800, dst_offset: 0, name: "HKT" },
        Transition { occurs_at: Some(104268600), utc_offset: 28800, dst_offset: 3600, name: "HKST" },
        Transition { occurs_at: Some(119989800), utc_offset: 28800, dst_offset: 0, name: "HKT" },
        Transition { occurs_at: Some(126041400), utc_offset: 28800, dst_offset: 3600, name: "HKST" },
        Transition { occurs_at: Some(151439400), utc_offset: 28800, dst_offset: 0, name: "HKT" },
        Transition { occurs_at: Some(167167800), utc_offset: 28800, dst_offset: 3600, name: "HKST" },
        Transition { occurs_at: Some(182889000), utc_offset: 28800, dst_offset: 0, name: "HKT" },
        Transition { occurs_at: Some(198617400), utc_offset: 28800, dst_offset: 3600, name: "HKST" },
        Transition { occurs_at: Some(214338600), utc_offset: 28800, dst_offset: 0, name: "HKT" },
        Transition { occurs_at: Some(295385400), utc_offset: 28800, dst_offset: 3600, name: "HKST" },
        Transition { occurs_at: Some(309292200), utc_offset: 28800, dst_offset: 0, name: "HKT" },
    ],
};


