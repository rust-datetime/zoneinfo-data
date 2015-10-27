
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "Asia/Yakutsk",
    transitions: &[
        Transition { occurs_at: None, utc_offset: 31138, dst_offset: 0, name: "LMT" },
        Transition { occurs_at: Some(-1579423138), utc_offset: 28800, dst_offset: 0, name: "YAKT" },
        Transition { occurs_at: Some(-1247558400), utc_offset: 32400, dst_offset: 0, name: "YAKT" },
        Transition { occurs_at: Some(354898800), utc_offset: 32400, dst_offset: 3600, name: "YAKST" },
        Transition { occurs_at: Some(370706400), utc_offset: 32400, dst_offset: 0, name: "YAKT" },
        Transition { occurs_at: Some(386434800), utc_offset: 32400, dst_offset: 3600, name: "YAKST" },
        Transition { occurs_at: Some(402242400), utc_offset: 32400, dst_offset: 0, name: "YAKT" },
        Transition { occurs_at: Some(417970800), utc_offset: 32400, dst_offset: 3600, name: "YAKST" },
        Transition { occurs_at: Some(433778400), utc_offset: 32400, dst_offset: 0, name: "YAKT" },
        Transition { occurs_at: Some(449593200), utc_offset: 32400, dst_offset: 3600, name: "YAKST" },
        Transition { occurs_at: Some(465325200), utc_offset: 32400, dst_offset: 0, name: "YAKT" },
        Transition { occurs_at: Some(481050000), utc_offset: 32400, dst_offset: 3600, name: "YAKST" },
        Transition { occurs_at: Some(496774800), utc_offset: 32400, dst_offset: 0, name: "YAKT" },
        Transition { occurs_at: Some(512499600), utc_offset: 32400, dst_offset: 3600, name: "YAKST" },
        Transition { occurs_at: Some(528224400), utc_offset: 32400, dst_offset: 0, name: "YAKT" },
        Transition { occurs_at: Some(543949200), utc_offset: 32400, dst_offset: 3600, name: "YAKST" },
        Transition { occurs_at: Some(559674000), utc_offset: 32400, dst_offset: 0, name: "YAKT" },
        Transition { occurs_at: Some(575398800), utc_offset: 32400, dst_offset: 3600, name: "YAKST" },
        Transition { occurs_at: Some(591123600), utc_offset: 32400, dst_offset: 0, name: "YAKT" },
        Transition { occurs_at: Some(606848400), utc_offset: 32400, dst_offset: 3600, name: "YAKST" },
        Transition { occurs_at: Some(622573200), utc_offset: 32400, dst_offset: 0, name: "YAKT" },
        Transition { occurs_at: Some(638298000), utc_offset: 32400, dst_offset: 3600, name: "YAKST" },
        Transition { occurs_at: Some(654627600), utc_offset: 32400, dst_offset: 0, name: "YAKT" },
        Transition { occurs_at: Some(670352400), utc_offset: 28800, dst_offset: 3600, name: "YAKST" },
        Transition { occurs_at: Some(686080800), utc_offset: 28800, dst_offset: 0, name: "YAKT" },
        Transition { occurs_at: Some(695757600), utc_offset: 32400, dst_offset: 0, name: "YAKT" },
        Transition { occurs_at: Some(701791200), utc_offset: 32400, dst_offset: 3600, name: "YAKST" },
        Transition { occurs_at: Some(717512400), utc_offset: 32400, dst_offset: 0, name: "YAKT" },
        Transition { occurs_at: Some(733251600), utc_offset: 32400, dst_offset: 3600, name: "YAKST" },
        Transition { occurs_at: Some(748976400), utc_offset: 32400, dst_offset: 0, name: "YAKT" },
        Transition { occurs_at: Some(764701200), utc_offset: 32400, dst_offset: 3600, name: "YAKST" },
        Transition { occurs_at: Some(780426000), utc_offset: 32400, dst_offset: 0, name: "YAKT" },
        Transition { occurs_at: Some(796150800), utc_offset: 32400, dst_offset: 3600, name: "YAKST" },
        Transition { occurs_at: Some(811875600), utc_offset: 32400, dst_offset: 0, name: "YAKT" },
        Transition { occurs_at: Some(828205200), utc_offset: 32400, dst_offset: 3600, name: "YAKST" },
        Transition { occurs_at: Some(846349200), utc_offset: 32400, dst_offset: 0, name: "YAKT" },
        Transition { occurs_at: Some(859654800), utc_offset: 32400, dst_offset: 3600, name: "YAKST" },
        Transition { occurs_at: Some(877798800), utc_offset: 32400, dst_offset: 0, name: "YAKT" },
        Transition { occurs_at: Some(891104400), utc_offset: 32400, dst_offset: 3600, name: "YAKST" },
        Transition { occurs_at: Some(909248400), utc_offset: 32400, dst_offset: 0, name: "YAKT" },
        Transition { occurs_at: Some(922554000), utc_offset: 32400, dst_offset: 3600, name: "YAKST" },
        Transition { occurs_at: Some(941302800), utc_offset: 32400, dst_offset: 0, name: "YAKT" },
        Transition { occurs_at: Some(954003600), utc_offset: 32400, dst_offset: 3600, name: "YAKST" },
        Transition { occurs_at: Some(972752400), utc_offset: 32400, dst_offset: 0, name: "YAKT" },
        Transition { occurs_at: Some(985453200), utc_offset: 32400, dst_offset: 3600, name: "YAKST" },
        Transition { occurs_at: Some(1004202000), utc_offset: 32400, dst_offset: 0, name: "YAKT" },
        Transition { occurs_at: Some(1017507600), utc_offset: 32400, dst_offset: 3600, name: "YAKST" },
        Transition { occurs_at: Some(1035651600), utc_offset: 32400, dst_offset: 0, name: "YAKT" },
        Transition { occurs_at: Some(1048957200), utc_offset: 32400, dst_offset: 3600, name: "YAKST" },
        Transition { occurs_at: Some(1067101200), utc_offset: 32400, dst_offset: 0, name: "YAKT" },
        Transition { occurs_at: Some(1080406800), utc_offset: 32400, dst_offset: 3600, name: "YAKST" },
        Transition { occurs_at: Some(1099155600), utc_offset: 32400, dst_offset: 0, name: "YAKT" },
        Transition { occurs_at: Some(1111856400), utc_offset: 32400, dst_offset: 3600, name: "YAKST" },
        Transition { occurs_at: Some(1130605200), utc_offset: 32400, dst_offset: 0, name: "YAKT" },
        Transition { occurs_at: Some(1143306000), utc_offset: 32400, dst_offset: 3600, name: "YAKST" },
        Transition { occurs_at: Some(1162054800), utc_offset: 32400, dst_offset: 0, name: "YAKT" },
        Transition { occurs_at: Some(1174755600), utc_offset: 32400, dst_offset: 3600, name: "YAKST" },
        Transition { occurs_at: Some(1193504400), utc_offset: 32400, dst_offset: 0, name: "YAKT" },
        Transition { occurs_at: Some(1206810000), utc_offset: 32400, dst_offset: 3600, name: "YAKST" },
        Transition { occurs_at: Some(1224954000), utc_offset: 32400, dst_offset: 0, name: "YAKT" },
        Transition { occurs_at: Some(1238259600), utc_offset: 32400, dst_offset: 3600, name: "YAKST" },
        Transition { occurs_at: Some(1256403600), utc_offset: 32400, dst_offset: 0, name: "YAKT" },
        Transition { occurs_at: Some(1269709200), utc_offset: 32400, dst_offset: 3600, name: "YAKST" },
        Transition { occurs_at: Some(1288458000), utc_offset: 32400, dst_offset: 0, name: "YAKT" },
        Transition { occurs_at: Some(1301158800), utc_offset: 36000, dst_offset: 0, name: "YAKT" },
        Transition { occurs_at: Some(1414252800), utc_offset: 32400, dst_offset: 0, name: "YAKT" },
    ],
};


