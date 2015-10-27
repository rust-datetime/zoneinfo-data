
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "America/Santa_Isabel",
    transitions: &[
        Transition { occurs_at: None, utc_offset: -22832, dst_offset: 0, name: "LMT" },
        Transition { occurs_at: Some(-1514740736), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(-1451667600), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(-1343062800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(-1234803600), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(-1222963200), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(-1207242000), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(-873820800), utc_offset: -28800, dst_offset: 3600, name: "PWT" },
        Transition { occurs_at: Some(-761677200), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(-686073600), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(-661539600), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(-495036000), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(-481734000), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(-463586400), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(-450284400), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(-431532000), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(-418230000), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(-400082400), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(-386780400), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(-368632800), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(-355330800), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(-337183200), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(-323881200), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(-305733600), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(-292431600), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(199274400), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(215600400), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(230724000), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(247050000), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(262778400), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(278499600), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(294228000), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(309949200), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(325677600), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(341398800), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(357127200), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(372848400), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(388576800), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(404902800), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(420026400), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(436352400), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(452080800), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(467802000), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(483530400), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(499251600), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(514980000), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(530701200), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(544615200), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(562150800), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(576064800), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(594205200), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(607514400), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(625654800), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(638964000), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(657104400), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(671018400), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(688554000), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(702468000), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(720003600), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(733917600), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(752058000), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(765367200), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(783507600), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(796816800), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(814957200), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(828871200), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(846406800), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(860320800), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(877856400), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(891770400), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(909306000), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(923220000), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(941360400), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(954669600), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(972810000), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(986119200), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(1004259600), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(1018173600), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(1035709200), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(1049623200), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(1067158800), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(1081072800), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(1099213200), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(1112522400), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(1130662800), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(1143972000), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(1162112400), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(1175421600), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(1193562000), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(1207476000), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(1225011600), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(1238925600), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(1256461200), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(1270375200), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(1288515600), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(1301824800), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(1319965200), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(1333274400), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(1351414800), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(1365328800), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(1382864400), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(1396778400), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(1414314000), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(1428228000), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(1445763600), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(1459677600), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(1477818000), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(1491127200), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(1509267600), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(1522576800), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(1540717200), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(1554631200), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(1572166800), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(1586080800), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(1603616400), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(1617530400), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(1635670800), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(1648980000), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(1667120400), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(1680429600), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(1698570000), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(1712484000), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(1730019600), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(1743933600), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(1761469200), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(1775383200), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(1792918800), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(1806832800), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(1824973200), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(1838282400), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(1856422800), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(1869732000), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(1887872400), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(1901786400), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(1919322000), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(1933236000), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(1950771600), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(1964685600), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(1982826000), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(1996135200), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2014275600), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2027584800), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2045725200), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2059034400), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2077174800), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2091088800), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2108624400), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2122538400), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2140074000), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2153988000), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2172128400), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2185437600), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2203578000), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2216887200), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2235027600), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2248941600), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2266477200), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2280391200), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2297926800), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2311840800), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2329376400), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2343290400), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2361430800), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2374740000), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2392880400), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2406189600), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2424330000), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2438244000), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2455779600), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2469693600), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2487229200), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2501143200), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2519283600), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2532592800), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2550733200), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2564042400), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2582182800), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2596096800), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2613632400), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2627546400), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2645082000), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2658996000), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2676531600), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2690445600), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2708586000), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2721895200), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2740035600), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2753344800), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2771485200), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2785399200), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2802934800), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2816848800), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2834384400), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2848298400), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2866438800), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2879748000), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2897888400), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2911197600), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2929338000), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2942647200), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2960787600), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2974701600), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2992237200), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3006151200), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3023686800), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3037600800), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3055741200), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3069050400), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3087190800), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3100500000), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3118640400), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3132554400), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3150090000), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3164004000), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3181539600), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3195453600), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3212989200), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3226903200), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3245043600), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3258352800), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3276493200), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3289802400), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3307942800), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3321856800), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3339392400), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3353306400), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3370842000), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3384756000), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3402896400), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3416205600), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3434346000), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3447655200), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3465795600), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3479709600), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3497245200), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3511159200), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3528694800), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3542608800), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3560144400), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3574058400), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3592198800), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3605508000), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3623648400), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3636957600), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3655098000), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3669012000), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3686547600), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3700461600), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3717997200), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3731911200), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3750051600), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3763360800), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3781501200), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3794810400), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3812950800), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3826260000), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3844400400), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3858314400), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3875850000), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3889764000), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3907299600), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3921213600), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3939354000), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3952663200), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3970803600), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3984112800), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(4002253200), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(4016167200), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(4033702800), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(4047616800), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(4065152400), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(4079066400), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(4096602000), utc_offset: -28800, dst_offset: 0, name: "PST" },
    ],
};


