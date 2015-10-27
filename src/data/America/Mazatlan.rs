
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "America/Mazatlan",
    transitions: &[
        Transition { occurs_at: None, utc_offset: -24860, dst_offset: 0, name: "LMT" },
        Transition { occurs_at: Some(-1514740280), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(-1343066400), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(-1234807200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(-1220292000), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(-1207159200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(-1191344400), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(-873828000), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(-661539600), utc_offset: -28800, dst_offset: 0, name: "PST" },
        Transition { occurs_at: Some(28800), utc_offset: -25200, dst_offset: 0, name: "MST" },
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
        Transition { occurs_at: Some(989139600), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1001836800), utc_offset: -25200, dst_offset: 0, name: "MST" },
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
        Transition { occurs_at: Some(1175418000), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1193558400), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1207472400), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1225008000), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1238922000), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1256457600), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1270371600), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1288512000), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1301821200), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1319961600), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1333270800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1351411200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1365325200), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1382860800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1396774800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1414310400), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1428224400), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1445760000), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1459674000), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1477814400), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1491123600), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1509264000), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1522573200), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1540713600), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1554627600), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1572163200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1586077200), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1603612800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1617526800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1635667200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1648976400), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1667116800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1680426000), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1698566400), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1712480400), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1730016000), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1743930000), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1761465600), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1775379600), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1792915200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1806829200), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1824969600), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1838278800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1856419200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1869728400), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1887868800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1901782800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1919318400), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1933232400), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1950768000), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1964682000), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1982822400), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1996131600), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2014272000), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2027581200), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2045721600), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2059030800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2077171200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2091085200), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2108620800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2122534800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2140070400), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2153984400), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2172124800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2185434000), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2203574400), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2216883600), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2235024000), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2248938000), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2266473600), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2280387600), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2297923200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2311837200), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2329372800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2343286800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2361427200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2374736400), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2392876800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2406186000), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2424326400), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2438240400), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2455776000), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2469690000), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2487225600), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2501139600), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2519280000), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2532589200), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2550729600), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2564038800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2582179200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2596093200), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2613628800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2627542800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2645078400), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2658992400), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2676528000), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2690442000), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2708582400), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2721891600), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2740032000), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2753341200), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2771481600), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2785395600), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2802931200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2816845200), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2834380800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2848294800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2866435200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2879744400), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2897884800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2911194000), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2929334400), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2942643600), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2960784000), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2974698000), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2992233600), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3006147600), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3023683200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3037597200), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3055737600), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3069046800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3087187200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3100496400), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3118636800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3132550800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3150086400), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3164000400), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3181536000), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3195450000), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3212985600), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3226899600), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3245040000), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3258349200), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3276489600), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3289798800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3307939200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3321853200), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3339388800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3353302800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3370838400), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3384752400), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3402892800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3416202000), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3434342400), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3447651600), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3465792000), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3479706000), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3497241600), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3511155600), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3528691200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3542605200), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3560140800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3574054800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3592195200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3605504400), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3623644800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3636954000), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3655094400), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3669008400), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3686544000), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3700458000), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3717993600), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3731907600), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3750048000), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3763357200), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3781497600), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3794806800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3812947200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3826256400), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3844396800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3858310800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3875846400), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3889760400), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3907296000), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3921210000), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3939350400), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3952659600), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3970800000), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3984109200), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(4002249600), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(4016163600), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(4033699200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(4047613200), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(4065148800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(4079062800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(4096598400), utc_offset: -25200, dst_offset: 0, name: "MST" },
    ],
};


