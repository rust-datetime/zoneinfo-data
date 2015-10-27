
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "America/Thule",
    transitions: &[
        Transition { occurs_at: None, utc_offset: -12292, dst_offset: 0, name: "LMT" },
        Transition { occurs_at: Some(-1686083708), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(670399200), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(686120400), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(701848800), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(717570000), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(733903200), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(752043600), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(765352800), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(783493200), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(796802400), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(814942800), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(828856800), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(846392400), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(860306400), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(877842000), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(891756000), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(909291600), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(923205600), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(941346000), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(954655200), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(972795600), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(986104800), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(1004245200), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(1018159200), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(1035694800), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(1049608800), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(1067144400), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(1081058400), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(1099198800), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(1112508000), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(1130648400), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(1143957600), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(1162098000), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(1173592800), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(1194152400), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(1205042400), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(1225602000), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(1236492000), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(1257051600), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(1268546400), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(1289106000), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(1299996000), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(1320555600), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(1331445600), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(1352005200), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(1362895200), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(1383454800), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(1394344800), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(1414904400), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(1425794400), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(1446354000), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(1457848800), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(1478408400), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(1489298400), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(1509858000), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(1520748000), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(1541307600), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(1552197600), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(1572757200), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(1583647200), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(1604206800), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(1615701600), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(1636261200), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(1647151200), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(1667710800), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(1678600800), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(1699160400), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(1710050400), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(1730610000), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(1741500000), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(1762059600), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(1772949600), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(1793509200), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(1805004000), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(1825563600), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(1836453600), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(1857013200), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(1867903200), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(1888462800), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(1899352800), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(1919912400), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(1930802400), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(1951362000), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(1962856800), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(1983416400), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(1994306400), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(2014866000), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(2025756000), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(2046315600), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(2057205600), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(2077765200), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(2088655200), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(2109214800), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(2120104800), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(2140664400), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(2152159200), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(2172718800), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(2183608800), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(2204168400), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(2215058400), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(2235618000), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(2246508000), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(2267067600), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(2277957600), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(2298517200), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(2309407200), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(2329966800), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(2341461600), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(2362021200), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(2372911200), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(2393470800), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(2404360800), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(2424920400), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(2435810400), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(2456370000), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(2467260000), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(2487819600), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(2499314400), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(2519874000), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(2530764000), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(2551323600), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(2562213600), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(2582773200), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(2593663200), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(2614222800), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(2625112800), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(2645672400), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(2656562400), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(2677122000), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(2688616800), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(2709176400), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(2720066400), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(2740626000), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(2751516000), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(2772075600), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(2782965600), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(2803525200), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(2814415200), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(2834974800), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(2846469600), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(2867029200), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(2877919200), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(2898478800), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(2909368800), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(2929928400), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(2940818400), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(2961378000), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(2972268000), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(2992827600), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(3003717600), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(3024277200), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(3035772000), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(3056331600), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(3067221600), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(3087781200), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(3098671200), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(3119230800), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(3130120800), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(3150680400), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(3161570400), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(3182130000), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(3193020000), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(3213579600), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(3225074400), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(3245634000), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(3256524000), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(3277083600), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(3287973600), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(3308533200), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(3319423200), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(3339982800), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(3350872800), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(3371432400), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(3382927200), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(3403486800), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(3414376800), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(3434936400), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(3445826400), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(3466386000), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(3477276000), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(3497835600), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(3508725600), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(3529285200), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(3540175200), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(3560734800), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(3572229600), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(3592789200), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(3603679200), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(3624238800), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(3635128800), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(3655688400), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(3666578400), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(3687138000), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(3698028000), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(3718587600), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(3730082400), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(3750642000), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(3761532000), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(3782091600), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(3792981600), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(3813541200), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(3824431200), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(3844990800), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(3855880800), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(3876440400), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(3887330400), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(3907890000), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(3919384800), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(3939944400), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(3950834400), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(3971394000), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(3982284000), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(4002843600), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(4013733600), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(4034293200), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(4045183200), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(4065742800), utc_offset: -14400, dst_offset: 0, name: "AST" },
        Transition { occurs_at: Some(4076632800), utc_offset: -14400, dst_offset: 3600, name: "ADT" },
        Transition { occurs_at: Some(4097192400), utc_offset: -14400, dst_offset: 0, name: "AST" },
    ],
};


