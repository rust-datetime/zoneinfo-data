
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "Europe/Chisinau",
    transitions: &[
        Transition { occurs_at: None, utc_offset: 6920, dst_offset: 0, name: "LMT" },
        Transition { occurs_at: Some(-2840147720), utc_offset: 6900, dst_offset: 0, name: "CMT" },
        Transition { occurs_at: Some(-1637114100), utc_offset: 6264, dst_offset: 0, name: "BMT" },
        Transition { occurs_at: Some(-1213148664), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(-1187056800), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(-1175479200), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(-1159754400), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(-1144029600), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(-1127700000), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(-1111975200), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(-1096250400), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(-1080525600), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(-1064800800), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(-1049076000), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(-1033351200), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(-1017626400), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(-1001901600), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(-986176800), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(-970452000), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(-954727200), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(-927165600), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(-898138800), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(-857257200), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(-844556400), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(-828226800), utc_offset: 3600, dst_offset: 0, name: "CET" },
        Transition { occurs_at: Some(-812502000), utc_offset: 3600, dst_offset: 3600, name: "CEST" },
        Transition { occurs_at: Some(-800157600), utc_offset: 10800, dst_offset: 0, name: "MSK" },
        Transition { occurs_at: Some(354920400), utc_offset: 10800, dst_offset: 3600, name: "MSD" },
        Transition { occurs_at: Some(370728000), utc_offset: 10800, dst_offset: 0, name: "MSK" },
        Transition { occurs_at: Some(386456400), utc_offset: 10800, dst_offset: 3600, name: "MSD" },
        Transition { occurs_at: Some(402264000), utc_offset: 10800, dst_offset: 0, name: "MSK" },
        Transition { occurs_at: Some(417992400), utc_offset: 10800, dst_offset: 3600, name: "MSD" },
        Transition { occurs_at: Some(433800000), utc_offset: 10800, dst_offset: 0, name: "MSK" },
        Transition { occurs_at: Some(449614800), utc_offset: 10800, dst_offset: 3600, name: "MSD" },
        Transition { occurs_at: Some(465346800), utc_offset: 10800, dst_offset: 0, name: "MSK" },
        Transition { occurs_at: Some(481071600), utc_offset: 10800, dst_offset: 3600, name: "MSD" },
        Transition { occurs_at: Some(496796400), utc_offset: 10800, dst_offset: 0, name: "MSK" },
        Transition { occurs_at: Some(512521200), utc_offset: 10800, dst_offset: 3600, name: "MSD" },
        Transition { occurs_at: Some(528246000), utc_offset: 10800, dst_offset: 0, name: "MSK" },
        Transition { occurs_at: Some(543970800), utc_offset: 10800, dst_offset: 3600, name: "MSD" },
        Transition { occurs_at: Some(559695600), utc_offset: 10800, dst_offset: 0, name: "MSK" },
        Transition { occurs_at: Some(575420400), utc_offset: 10800, dst_offset: 3600, name: "MSD" },
        Transition { occurs_at: Some(591145200), utc_offset: 10800, dst_offset: 0, name: "MSK" },
        Transition { occurs_at: Some(606870000), utc_offset: 10800, dst_offset: 3600, name: "MSD" },
        Transition { occurs_at: Some(622594800), utc_offset: 10800, dst_offset: 0, name: "MSK" },
        Transition { occurs_at: Some(641941200), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(670377600), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(686102400), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(701820000), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(717541200), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(733269600), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(748990800), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(764719200), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(780440400), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(796168800), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(811890000), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(828223200), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(846363600), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(859680000), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(877824000), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(891129600), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(909273600), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(922579200), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(941328000), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(954028800), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(972777600), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(985478400), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1004227200), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1017532800), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1035676800), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1048982400), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1067126400), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1080432000), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1099180800), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1111881600), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1130630400), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1143331200), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1162080000), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1174780800), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1193529600), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1206835200), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1224979200), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1238284800), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1256428800), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1269734400), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1288483200), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1301184000), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1319932800), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1332633600), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1351382400), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1364688000), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1382832000), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1396137600), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1414281600), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1427587200), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1445731200), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1459036800), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1477785600), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1490486400), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1509235200), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1521936000), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1540684800), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1553990400), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1572134400), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1585440000), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1603584000), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1616889600), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1635638400), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1648339200), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1667088000), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1679788800), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1698537600), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1711843200), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1729987200), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1743292800), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1761436800), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1774742400), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1792886400), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1806192000), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1824940800), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1837641600), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1856390400), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1869091200), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1887840000), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1901145600), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1919289600), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1932595200), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1950739200), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1964044800), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1982793600), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1995494400), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2014243200), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2026944000), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2045692800), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2058393600), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2077142400), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2090448000), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2108592000), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2121897600), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2140041600), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2153347200), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2172096000), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2184796800), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2203545600), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2216246400), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2234995200), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2248300800), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2266444800), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2279750400), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2297894400), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2311200000), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2329344000), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2342649600), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2361398400), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2374099200), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2392848000), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2405548800), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2424297600), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2437603200), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2455747200), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2469052800), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2487196800), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2500502400), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2519251200), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2531952000), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2550700800), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2563401600), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2582150400), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2595456000), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2613600000), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2626905600), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2645049600), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2658355200), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2676499200), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2689804800), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2708553600), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2721254400), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2740003200), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2752704000), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2771452800), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2784758400), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2802902400), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2816208000), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2834352000), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2847657600), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2866406400), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2879107200), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2897856000), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2910556800), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2929305600), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2942006400), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2960755200), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2974060800), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2992204800), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3005510400), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3023654400), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3036960000), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3055708800), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3068409600), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3087158400), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3099859200), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3118608000), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3131913600), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3150057600), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3163363200), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3181507200), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3194812800), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3212956800), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3226262400), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3245011200), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3257712000), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3276460800), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3289161600), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3307910400), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3321216000), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3339360000), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3352665600), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3370809600), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3384115200), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3402864000), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3415564800), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3434313600), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3447014400), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3465763200), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3479068800), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3497212800), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3510518400), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3528662400), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3541968000), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3560112000), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3573417600), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3592166400), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3604867200), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3623616000), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3636316800), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3655065600), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3668371200), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3686515200), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3699820800), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3717964800), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3731270400), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3750019200), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3762720000), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3781468800), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3794169600), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3812918400), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3825619200), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3844368000), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3857673600), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3875817600), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3889123200), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3907267200), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3920572800), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3939321600), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3952022400), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3970771200), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3983472000), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(4002220800), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(4015526400), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(4033670400), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(4046976000), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(4065120000), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(4078425600), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(4096569600), utc_offset: 7200, dst_offset: 0, name: "EET" },
    ],
};


