
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "Australia/Broken_Hill",
    transitions: &[
        Transition { occurs_at: None, utc_offset: 33948, dst_offset: 0, name: "LMT" },
        Transition { occurs_at: Some(-2364110748), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(-2314951200), utc_offset: 32400, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(-2230189200), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(-1672565340), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(-1665390600), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(-883639800), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(-876126600), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(-860398200), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(-844677000), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(-828343800), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(-813227400), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(57688200), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(67969800), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(89137800), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(100024200), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(120587400), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(131473800), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(152037000), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(162923400), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(183486600), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(194977800), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(215541000), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(226427400), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(246990600), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(257877000), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(278440200), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(289326600), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(309889800), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(320776200), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(341339400), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(352225800), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(372789000), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(386699400), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(404843400), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(415729800), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(436293000), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(447179400), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(467742600), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(478629000), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(499192200), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(511288200), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(530037000), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(542737800), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(562091400), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(574792200), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(594145800), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(606241800), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(625595400), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(636481800), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(657045000), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(667931400), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(688494600), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(699467400), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(719944200), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(731435400), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(751998600), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(762885000), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(783448200), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(794334600), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(814897800), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(828203400), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(846347400), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(859653000), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(877797000), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(891102600), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(909246600), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(922552200), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(941301000), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(954001800), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(972750600), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(985451400), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(1004200200), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(1017505800), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(1035649800), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(1048955400), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(1067099400), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(1080405000), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(1099153800), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(1111854600), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(1130603400), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(1143909000), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(1162053000), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(1174753800), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(1193502600), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(1207413000), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(1223137800), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(1238862600), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(1254587400), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(1270312200), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(1286037000), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(1301761800), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(1317486600), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(1333211400), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(1349541000), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(1365265800), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(1380990600), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(1396715400), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(1412440200), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(1428165000), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(1443889800), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(1459614600), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(1475339400), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(1491064200), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(1506789000), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(1522513800), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(1538843400), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(1554568200), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(1570293000), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(1586017800), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(1601742600), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(1617467400), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(1633192200), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(1648917000), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(1664641800), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(1680366600), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(1696091400), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(1712421000), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(1728145800), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(1743870600), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(1759595400), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(1775320200), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(1791045000), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(1806769800), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(1822494600), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(1838219400), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(1853944200), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(1869669000), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(1885998600), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(1901723400), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(1917448200), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(1933173000), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(1948897800), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(1964622600), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(1980347400), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(1996072200), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(2011797000), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(2027521800), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(2043246600), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(2058971400), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(2075301000), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(2091025800), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(2106750600), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(2122475400), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(2138200200), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(2153925000), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(2169649800), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(2185374600), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(2201099400), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(2216824200), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(2233153800), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(2248878600), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(2264603400), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(2280328200), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(2296053000), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(2311777800), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(2327502600), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(2343227400), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(2358952200), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(2374677000), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(2390401800), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(2406126600), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(2422456200), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(2438181000), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(2453905800), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(2469630600), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(2485355400), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(2501080200), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(2516805000), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(2532529800), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(2548254600), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(2563979400), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(2579704200), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(2596033800), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(2611758600), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(2627483400), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(2643208200), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(2658933000), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(2674657800), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(2690382600), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(2706107400), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(2721832200), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(2737557000), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(2753281800), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(2769611400), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(2785336200), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(2801061000), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(2816785800), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(2832510600), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(2848235400), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(2863960200), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(2879685000), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(2895409800), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(2911134600), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(2926859400), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(2942584200), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(2958913800), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(2974638600), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(2990363400), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(3006088200), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(3021813000), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(3037537800), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(3053262600), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(3068987400), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(3084712200), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(3100437000), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(3116766600), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(3132491400), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(3148216200), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(3163941000), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(3179665800), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(3195390600), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(3211115400), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(3226840200), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(3242565000), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(3258289800), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(3274014600), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(3289739400), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(3306069000), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(3321793800), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(3337518600), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(3353243400), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(3368968200), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(3384693000), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(3400417800), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(3416142600), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(3431867400), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(3447592200), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(3463317000), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(3479646600), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(3495371400), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(3511096200), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(3526821000), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(3542545800), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(3558270600), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(3573995400), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(3589720200), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(3605445000), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(3621169800), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(3636894600), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(3653224200), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(3668949000), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(3684673800), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(3700398600), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(3716123400), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(3731848200), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(3747573000), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(3763297800), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(3779022600), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(3794747400), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(3810472200), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(3826197000), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(3842526600), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(3858251400), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(3873976200), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(3889701000), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(3905425800), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(3921150600), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(3936875400), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(3952600200), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(3968325000), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(3984049800), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(4000379400), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(4016104200), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(4031829000), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(4047553800), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(4063278600), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
        Transition { occurs_at: Some(4079003400), utc_offset: 34200, dst_offset: 0, name: "ACST" },
        Transition { occurs_at: Some(4094728200), utc_offset: 34200, dst_offset: 3600, name: "ACDT" },
    ],
};


