
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "America/Dawson",
    transitions: &[
        Transition { occurs_at: None, utc_offset: -31340, dst_offset: 0, name: "LMT" },
        Transition { occurs_at: Some(-2188999060), utc_offset: -32400, dst_offset: 0, name: "YST" },
        Transition { occurs_at: Some(-1632056400), utc_offset: -32400, dst_offset: 3600, name: "YDT" },
        Transition { occurs_at: Some(-1615125600), utc_offset: -32400, dst_offset: 0, name: "YST" },
        Transition { occurs_at: Some(-1596978000), utc_offset: -32400, dst_offset: 3600, name: "YDT" },
        Transition { occurs_at: Some(-1583164800), utc_offset: -32400, dst_offset: 0, name: "YST" },
        Transition { occurs_at: Some(-880203600), utc_offset: -32400, dst_offset: 3600, name: "YWT" },
        Transition { occurs_at: Some(-765381600), utc_offset: -32400, dst_offset: 0, name: "YST" },
        Transition { occurs_at: Some(-147884400), utc_offset: -32400, dst_offset: 7200, name: "YDDT" },
        Transition { occurs_at: Some(-131554800), utc_offset: -32400, dst_offset: 0, name: "YST" },
        Transition { occurs_at: Some(120646800), utc_offset: -28800, dst_offset: 0, name: "PST" },
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
        Transition { occurs_at: Some(1173607200), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(1194166800), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(1205056800), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(1225616400), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(1236506400), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(1257066000), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(1268560800), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(1289120400), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(1300010400), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(1320570000), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(1331460000), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(1352019600), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(1362909600), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(1383469200), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(1394359200), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(1414918800), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(1425808800), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(1446368400), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(1457863200), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(1478422800), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(1489312800), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(1509872400), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(1520762400), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(1541322000), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(1552212000), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(1572771600), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(1583661600), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(1604221200), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(1615716000), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(1636275600), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(1647165600), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(1667725200), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(1678615200), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(1699174800), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(1710064800), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(1730624400), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(1741514400), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(1762074000), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(1772964000), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(1793523600), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(1805018400), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(1825578000), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(1836468000), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(1857027600), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(1867917600), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(1888477200), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(1899367200), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(1919926800), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(1930816800), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(1951376400), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(1962871200), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(1983430800), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(1994320800), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2014880400), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2025770400), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2046330000), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2057220000), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2077779600), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2088669600), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2109229200), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2120119200), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2140678800), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2152173600), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2172733200), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2183623200), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2204182800), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2215072800), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2235632400), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2246522400), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2267082000), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2277972000), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2298531600), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2309421600), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2329981200), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2341476000), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2362035600), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2372925600), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2393485200), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2404375200), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2424934800), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2435824800), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2456384400), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2467274400), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2487834000), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2499328800), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2519888400), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2530778400), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2551338000), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2562228000), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2582787600), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2593677600), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2614237200), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2625127200), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2645686800), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2656576800), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2677136400), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2688631200), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2709190800), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2720080800), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2740640400), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2751530400), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2772090000), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2782980000), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2803539600), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2814429600), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2834989200), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2846484000), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2867043600), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2877933600), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2898493200), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2909383200), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2929942800), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2940832800), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2961392400), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(2972282400), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(2992842000), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3003732000), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3024291600), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3035786400), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3056346000), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3067236000), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3087795600), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3098685600), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3119245200), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3130135200), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3150694800), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3161584800), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3182144400), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3193034400), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3213594000), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3225088800), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3245648400), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3256538400), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3277098000), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3287988000), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3308547600), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3319437600), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3339997200), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3350887200), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3371446800), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3382941600), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3403501200), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3414391200), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3434950800), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3445840800), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3466400400), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3477290400), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3497850000), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3508740000), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3529299600), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3540189600), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3560749200), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3572244000), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3592803600), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3603693600), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3624253200), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3635143200), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3655702800), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3666592800), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3687152400), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3698042400), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3718602000), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3730096800), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3750656400), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3761546400), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3782106000), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3792996000), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3813555600), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3824445600), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3845005200), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3855895200), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3876454800), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3887344800), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3907904400), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3919399200), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3939958800), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3950848800), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(3971408400), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(3982298400), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(4002858000), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(4013748000), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(4034307600), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(4045197600), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(4065757200), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(4076647200), utc_offset: -28800, dst_offset: 3600, name: "PDT" },
        Transition { occurs_at: Some(4097206800), utc_offset: -28800, dst_offset: 0, name: "PST" },
    ],
};


