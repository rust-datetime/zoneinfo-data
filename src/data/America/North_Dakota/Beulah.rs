
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "America/North_Dakota/Beulah",
    transitions: &[
        Transition { occurs_at: None, utc_offset: -18773, dst_offset: 0, name: "LMT" },
        Transition { occurs_at: Some(-2717649254), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(-1633273200), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(-1615132800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(-1601823600), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(-1583683200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(-880210800), utc_offset: -25200, dst_offset: 3600, name: "MWT" },
        Transition { occurs_at: Some(-765388800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(-84380400), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(-68659200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(-52930800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(-37209600), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(-21481200), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(-5760000), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(9968400), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(25689600), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(41418000), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(57744000), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(73472400), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(89193600), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(104922000), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(120643200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(126694800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(152092800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(162378000), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(183542400), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(199270800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(215596800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(230720400), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(247046400), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(262774800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(278496000), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(294224400), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(309945600), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(325674000), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(341395200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(357123600), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(372844800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(388573200), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(404899200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(420022800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(436348800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(452077200), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(467798400), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(483526800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(499248000), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(514976400), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(530697600), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(544611600), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(562147200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(576061200), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(594201600), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(607510800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(625651200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(638960400), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(657100800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(671014800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(688550400), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(702464400), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(720000000), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(733914000), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(752054400), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(765363600), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(783504000), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(796813200), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(814953600), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(828867600), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(846403200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(860317200), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(877852800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(891766800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(909302400), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(923216400), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(941356800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(954666000), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(972806400), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(986115600), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1004256000), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1018170000), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1035705600), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1049619600), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1067155200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1081069200), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1099209600), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1112518800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1130659200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1143968400), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1162108800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1173603600), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1194163200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1205053200), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1225612800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1236502800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1257062400), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1268557200), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1289116800), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(1300003200), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(1320562800), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(1331452800), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(1352012400), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(1362902400), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(1383462000), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(1394352000), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(1414911600), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(1425801600), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(1446361200), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(1457856000), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(1478415600), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(1489305600), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(1509865200), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(1520755200), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(1541314800), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(1552204800), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(1572764400), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(1583654400), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(1604214000), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(1615708800), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(1636268400), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(1647158400), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(1667718000), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(1678608000), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(1699167600), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(1710057600), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(1730617200), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(1741507200), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(1762066800), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(1772956800), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(1793516400), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(1805011200), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(1825570800), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(1836460800), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(1857020400), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(1867910400), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(1888470000), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(1899360000), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(1919919600), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(1930809600), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(1951369200), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(1962864000), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(1983423600), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(1994313600), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2014873200), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2025763200), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2046322800), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2057212800), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2077772400), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2088662400), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2109222000), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2120112000), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2140671600), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2152166400), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2172726000), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2183616000), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2204175600), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2215065600), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2235625200), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2246515200), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2267074800), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2277964800), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2298524400), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2309414400), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2329974000), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2341468800), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2362028400), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2372918400), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2393478000), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2404368000), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2424927600), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2435817600), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2456377200), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2467267200), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2487826800), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2499321600), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2519881200), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2530771200), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2551330800), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2562220800), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2582780400), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2593670400), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2614230000), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2625120000), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2645679600), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2656569600), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2677129200), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2688624000), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2709183600), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2720073600), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2740633200), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2751523200), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2772082800), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2782972800), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2803532400), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2814422400), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2834982000), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2846476800), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2867036400), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2877926400), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2898486000), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2909376000), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2929935600), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2940825600), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2961385200), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2972275200), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2992834800), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3003724800), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3024284400), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3035779200), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3056338800), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3067228800), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3087788400), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3098678400), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3119238000), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3130128000), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3150687600), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3161577600), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3182137200), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3193027200), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3213586800), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3225081600), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3245641200), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3256531200), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3277090800), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3287980800), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3308540400), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3319430400), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3339990000), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3350880000), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3371439600), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3382934400), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3403494000), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3414384000), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3434943600), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3445833600), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3466393200), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3477283200), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3497842800), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3508732800), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3529292400), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3540182400), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3560742000), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3572236800), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3592796400), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3603686400), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3624246000), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3635136000), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3655695600), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3666585600), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3687145200), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3698035200), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3718594800), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3730089600), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3750649200), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3761539200), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3782098800), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3792988800), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3813548400), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3824438400), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3844998000), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3855888000), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3876447600), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3887337600), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3907897200), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3919392000), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3939951600), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3950841600), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3971401200), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3982291200), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(4002850800), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(4013740800), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(4034300400), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(4045190400), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(4065750000), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(4076640000), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(4097199600), utc_offset: -21600, dst_offset: 0, name: "CST" },
    ],
};


