
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "Pacific/Fiji",
    transitions: &[
        Transition { occurs_at: None, utc_offset: 42944, dst_offset: 0, name: "LMT" },
        Transition { occurs_at: Some(-1709985344), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(909842400), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(920124000), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(941896800), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(951573600), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(1259416800), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(1269698400), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(1287842400), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(1299333600), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(1319292000), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(1327154400), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(1350741600), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(1358604000), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(1382796000), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(1390050000), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(1414850400), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(1421503200), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(1446300000), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(1452952800), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(1478354400), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(1484402400), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(1509804000), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(1516456800), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(1541253600), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(1547906400), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(1572703200), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(1579356000), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(1604152800), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(1610805600), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(1636207200), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(1642255200), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(1667656800), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(1673704800), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(1699106400), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(1705759200), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(1730556000), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(1737208800), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(1762005600), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(1768658400), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(1793455200), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(1800108000), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(1825509600), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(1831557600), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(1856959200), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(1863612000), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(1888408800), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(1895061600), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(1919858400), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(1926511200), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(1951308000), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(1957960800), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(1983362400), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(1989410400), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(2014812000), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(2020860000), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(2046261600), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(2052914400), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(2077711200), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(2084364000), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(2109160800), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(2115813600), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(2140610400), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(2147263200), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(2172664800), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(2178712800), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(2204114400), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(2210162400), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(2235564000), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(2242216800), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(2267013600), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(2273666400), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(2298463200), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(2305116000), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(2329912800), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(2336565600), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(2361967200), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(2368015200), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(2393416800), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(2400069600), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(2424866400), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(2431519200), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(2456316000), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(2462968800), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(2487765600), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(2494418400), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(2519820000), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(2525868000), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(2551269600), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(2557317600), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(2582719200), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(2589372000), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(2614168800), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(2620821600), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(2645618400), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(2652271200), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(2677068000), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(2683720800), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(2709122400), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(2715170400), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(2740572000), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(2747224800), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(2772021600), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(2778674400), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(2803471200), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(2810124000), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(2834920800), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(2841573600), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(2866975200), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(2873023200), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(2898424800), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(2904472800), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(2929874400), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(2936527200), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(2961324000), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(2967976800), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(2992773600), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(2999426400), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(3024223200), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(3030876000), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(3056277600), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(3062325600), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(3087727200), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(3093775200), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(3119176800), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(3125829600), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(3150626400), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(3157279200), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(3182076000), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(3188728800), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(3213525600), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(3220178400), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(3245580000), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(3251628000), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(3277029600), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(3283682400), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(3308479200), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(3315132000), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(3339928800), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(3346581600), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(3371378400), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(3378031200), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(3403432800), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(3409480800), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(3434882400), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(3440930400), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(3466332000), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(3472984800), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(3497781600), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(3504434400), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(3529231200), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(3535884000), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(3560680800), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(3567333600), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(3592735200), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(3598783200), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(3624184800), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(3630837600), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(3655634400), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(3662287200), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(3687084000), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(3693736800), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(3718533600), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(3725186400), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(3750588000), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(3756636000), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(3782037600), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(3788085600), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(3813487200), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(3820140000), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(3844936800), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(3851589600), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(3876386400), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(3883039200), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(3907836000), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(3914488800), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(3939890400), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(3945938400), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(3971340000), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(3977388000), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(4002789600), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(4009442400), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(4034239200), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(4040892000), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(4065688800), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
        Transition { occurs_at: Some(4072341600), utc_offset: 43200, dst_offset: 0, name: "FJT" },
        Transition { occurs_at: Some(4097138400), utc_offset: 43200, dst_offset: 3600, name: "FJST" },
    ],
};


