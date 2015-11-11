
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zone::TimeZone;

pub mod Africa;
pub mod America;
pub mod Antarctica;
pub mod Asia;
pub mod Atlantic;
pub mod Australia;
pub mod Europe;
pub mod Indian;
pub mod Pacific;



mod CET;
pub use self::CET::ZONE as CET;

mod CST6CDT;
pub use self::CST6CDT::ZONE as CST6CDT;

mod EET;
pub use self::EET::ZONE as EET;

mod EST;
pub use self::EST::ZONE as EST;

mod EST5EDT;
pub use self::EST5EDT::ZONE as EST5EDT;

mod HST;
pub use self::HST::ZONE as HST;

mod MET;
pub use self::MET::ZONE as MET;

mod MST;
pub use self::MST::ZONE as MST;

mod MST7MDT;
pub use self::MST7MDT::ZONE as MST7MDT;

mod PST8PDT;
pub use self::PST8PDT::ZONE as PST8PDT;

mod WET;
pub use self::WET::ZONE as WET;




pub fn lookup(input: &str) -> Option<TimeZone> {
    if input == "Africa/Abidjan" {
        return Some(Africa::Abidjan);
    }
    if input == "Africa/Accra" {
        return Some(Africa::Accra);
    }
    if input == "Africa/Algiers" {
        return Some(Africa::Algiers);
    }
    if input == "Africa/Bissau" {
        return Some(Africa::Bissau);
    }
    if input == "Africa/Cairo" {
        return Some(Africa::Cairo);
    }
    if input == "Africa/Casablanca" {
        return Some(Africa::Casablanca);
    }
    if input == "Africa/Ceuta" {
        return Some(Africa::Ceuta);
    }
    if input == "Africa/El_Aaiun" {
        return Some(Africa::El_Aaiun);
    }
    if input == "Africa/Johannesburg" {
        return Some(Africa::Johannesburg);
    }
    if input == "Africa/Khartoum" {
        return Some(Africa::Khartoum);
    }
    if input == "Africa/Lagos" {
        return Some(Africa::Lagos);
    }
    if input == "Africa/Maputo" {
        return Some(Africa::Maputo);
    }
    if input == "Africa/Monrovia" {
        return Some(Africa::Monrovia);
    }
    if input == "Africa/Nairobi" {
        return Some(Africa::Nairobi);
    }
    if input == "Africa/Ndjamena" {
        return Some(Africa::Ndjamena);
    }
    if input == "Africa/Tripoli" {
        return Some(Africa::Tripoli);
    }
    if input == "Africa/Tunis" {
        return Some(Africa::Tunis);
    }
    if input == "Africa/Windhoek" {
        return Some(Africa::Windhoek);
    }
    if input == "America/Adak" {
        return Some(America::Adak);
    }
    if input == "America/Anchorage" {
        return Some(America::Anchorage);
    }
    if input == "America/Araguaina" {
        return Some(America::Araguaina);
    }
    if input == "America/Argentina/Buenos_Aires" {
        return Some(America::Argentina::Buenos_Aires);
    }
    if input == "America/Argentina/Catamarca" {
        return Some(America::Argentina::Catamarca);
    }
    if input == "America/Argentina/Cordoba" {
        return Some(America::Argentina::Cordoba);
    }
    if input == "America/Argentina/Jujuy" {
        return Some(America::Argentina::Jujuy);
    }
    if input == "America/Argentina/La_Rioja" {
        return Some(America::Argentina::La_Rioja);
    }
    if input == "America/Argentina/Mendoza" {
        return Some(America::Argentina::Mendoza);
    }
    if input == "America/Argentina/Rio_Gallegos" {
        return Some(America::Argentina::Rio_Gallegos);
    }
    if input == "America/Argentina/Salta" {
        return Some(America::Argentina::Salta);
    }
    if input == "America/Argentina/San_Juan" {
        return Some(America::Argentina::San_Juan);
    }
    if input == "America/Argentina/San_Luis" {
        return Some(America::Argentina::San_Luis);
    }
    if input == "America/Argentina/Tucuman" {
        return Some(America::Argentina::Tucuman);
    }
    if input == "America/Argentina/Ushuaia" {
        return Some(America::Argentina::Ushuaia);
    }
    if input == "America/Asuncion" {
        return Some(America::Asuncion);
    }
    if input == "America/Atikokan" {
        return Some(America::Atikokan);
    }
    if input == "America/Bahia" {
        return Some(America::Bahia);
    }
    if input == "America/Bahia_Banderas" {
        return Some(America::Bahia_Banderas);
    }
    if input == "America/Barbados" {
        return Some(America::Barbados);
    }
    if input == "America/Belem" {
        return Some(America::Belem);
    }
    if input == "America/Belize" {
        return Some(America::Belize);
    }
    if input == "America/Blanc-Sablon" {
        return Some(America::Blanc_Sablon);
    }
    if input == "America/Boa_Vista" {
        return Some(America::Boa_Vista);
    }
    if input == "America/Bogota" {
        return Some(America::Bogota);
    }
    if input == "America/Boise" {
        return Some(America::Boise);
    }
    if input == "America/Cambridge_Bay" {
        return Some(America::Cambridge_Bay);
    }
    if input == "America/Campo_Grande" {
        return Some(America::Campo_Grande);
    }
    if input == "America/Cancun" {
        return Some(America::Cancun);
    }
    if input == "America/Caracas" {
        return Some(America::Caracas);
    }
    if input == "America/Cayenne" {
        return Some(America::Cayenne);
    }
    if input == "America/Cayman" {
        return Some(America::Cayman);
    }
    if input == "America/Chicago" {
        return Some(America::Chicago);
    }
    if input == "America/Chihuahua" {
        return Some(America::Chihuahua);
    }
    if input == "America/Costa_Rica" {
        return Some(America::Costa_Rica);
    }
    if input == "America/Creston" {
        return Some(America::Creston);
    }
    if input == "America/Cuiaba" {
        return Some(America::Cuiaba);
    }
    if input == "America/Curacao" {
        return Some(America::Curacao);
    }
    if input == "America/Danmarkshavn" {
        return Some(America::Danmarkshavn);
    }
    if input == "America/Dawson" {
        return Some(America::Dawson);
    }
    if input == "America/Dawson_Creek" {
        return Some(America::Dawson_Creek);
    }
    if input == "America/Denver" {
        return Some(America::Denver);
    }
    if input == "America/Detroit" {
        return Some(America::Detroit);
    }
    if input == "America/Edmonton" {
        return Some(America::Edmonton);
    }
    if input == "America/Eirunepe" {
        return Some(America::Eirunepe);
    }
    if input == "America/El_Salvador" {
        return Some(America::El_Salvador);
    }
    if input == "America/Fort_Nelson" {
        return Some(America::Fort_Nelson);
    }
    if input == "America/Fortaleza" {
        return Some(America::Fortaleza);
    }
    if input == "America/Glace_Bay" {
        return Some(America::Glace_Bay);
    }
    if input == "America/Godthab" {
        return Some(America::Godthab);
    }
    if input == "America/Goose_Bay" {
        return Some(America::Goose_Bay);
    }
    if input == "America/Grand_Turk" {
        return Some(America::Grand_Turk);
    }
    if input == "America/Guatemala" {
        return Some(America::Guatemala);
    }
    if input == "America/Guayaquil" {
        return Some(America::Guayaquil);
    }
    if input == "America/Guyana" {
        return Some(America::Guyana);
    }
    if input == "America/Halifax" {
        return Some(America::Halifax);
    }
    if input == "America/Havana" {
        return Some(America::Havana);
    }
    if input == "America/Hermosillo" {
        return Some(America::Hermosillo);
    }
    if input == "America/Indiana/Indianapolis" {
        return Some(America::Indiana::Indianapolis);
    }
    if input == "America/Indiana/Knox" {
        return Some(America::Indiana::Knox);
    }
    if input == "America/Indiana/Marengo" {
        return Some(America::Indiana::Marengo);
    }
    if input == "America/Indiana/Petersburg" {
        return Some(America::Indiana::Petersburg);
    }
    if input == "America/Indiana/Tell_City" {
        return Some(America::Indiana::Tell_City);
    }
    if input == "America/Indiana/Vevay" {
        return Some(America::Indiana::Vevay);
    }
    if input == "America/Indiana/Vincennes" {
        return Some(America::Indiana::Vincennes);
    }
    if input == "America/Indiana/Winamac" {
        return Some(America::Indiana::Winamac);
    }
    if input == "America/Inuvik" {
        return Some(America::Inuvik);
    }
    if input == "America/Iqaluit" {
        return Some(America::Iqaluit);
    }
    if input == "America/Jamaica" {
        return Some(America::Jamaica);
    }
    if input == "America/Juneau" {
        return Some(America::Juneau);
    }
    if input == "America/Kentucky/Louisville" {
        return Some(America::Kentucky::Louisville);
    }
    if input == "America/Kentucky/Monticello" {
        return Some(America::Kentucky::Monticello);
    }
    if input == "America/La_Paz" {
        return Some(America::La_Paz);
    }
    if input == "America/Lima" {
        return Some(America::Lima);
    }
    if input == "America/Los_Angeles" {
        return Some(America::Los_Angeles);
    }
    if input == "America/Maceio" {
        return Some(America::Maceio);
    }
    if input == "America/Managua" {
        return Some(America::Managua);
    }
    if input == "America/Manaus" {
        return Some(America::Manaus);
    }
    if input == "America/Martinique" {
        return Some(America::Martinique);
    }
    if input == "America/Matamoros" {
        return Some(America::Matamoros);
    }
    if input == "America/Mazatlan" {
        return Some(America::Mazatlan);
    }
    if input == "America/Menominee" {
        return Some(America::Menominee);
    }
    if input == "America/Merida" {
        return Some(America::Merida);
    }
    if input == "America/Metlakatla" {
        return Some(America::Metlakatla);
    }
    if input == "America/Mexico_City" {
        return Some(America::Mexico_City);
    }
    if input == "America/Miquelon" {
        return Some(America::Miquelon);
    }
    if input == "America/Moncton" {
        return Some(America::Moncton);
    }
    if input == "America/Monterrey" {
        return Some(America::Monterrey);
    }
    if input == "America/Montevideo" {
        return Some(America::Montevideo);
    }
    if input == "America/Nassau" {
        return Some(America::Nassau);
    }
    if input == "America/New_York" {
        return Some(America::New_York);
    }
    if input == "America/Nipigon" {
        return Some(America::Nipigon);
    }
    if input == "America/Nome" {
        return Some(America::Nome);
    }
    if input == "America/Noronha" {
        return Some(America::Noronha);
    }
    if input == "America/North_Dakota/Beulah" {
        return Some(America::North_Dakota::Beulah);
    }
    if input == "America/North_Dakota/Center" {
        return Some(America::North_Dakota::Center);
    }
    if input == "America/North_Dakota/New_Salem" {
        return Some(America::North_Dakota::New_Salem);
    }
    if input == "America/Ojinaga" {
        return Some(America::Ojinaga);
    }
    if input == "America/Panama" {
        return Some(America::Panama);
    }
    if input == "America/Pangnirtung" {
        return Some(America::Pangnirtung);
    }
    if input == "America/Paramaribo" {
        return Some(America::Paramaribo);
    }
    if input == "America/Phoenix" {
        return Some(America::Phoenix);
    }
    if input == "America/Port-au-Prince" {
        return Some(America::Port_au_Prince);
    }
    if input == "America/Port_of_Spain" {
        return Some(America::Port_of_Spain);
    }
    if input == "America/Porto_Velho" {
        return Some(America::Porto_Velho);
    }
    if input == "America/Puerto_Rico" {
        return Some(America::Puerto_Rico);
    }
    if input == "America/Rainy_River" {
        return Some(America::Rainy_River);
    }
    if input == "America/Rankin_Inlet" {
        return Some(America::Rankin_Inlet);
    }
    if input == "America/Recife" {
        return Some(America::Recife);
    }
    if input == "America/Regina" {
        return Some(America::Regina);
    }
    if input == "America/Resolute" {
        return Some(America::Resolute);
    }
    if input == "America/Rio_Branco" {
        return Some(America::Rio_Branco);
    }
    if input == "America/Santa_Isabel" {
        return Some(America::Santa_Isabel);
    }
    if input == "America/Santarem" {
        return Some(America::Santarem);
    }
    if input == "America/Santiago" {
        return Some(America::Santiago);
    }
    if input == "America/Santo_Domingo" {
        return Some(America::Santo_Domingo);
    }
    if input == "America/Sao_Paulo" {
        return Some(America::Sao_Paulo);
    }
    if input == "America/Scoresbysund" {
        return Some(America::Scoresbysund);
    }
    if input == "America/Sitka" {
        return Some(America::Sitka);
    }
    if input == "America/St_Johns" {
        return Some(America::St_Johns);
    }
    if input == "America/Swift_Current" {
        return Some(America::Swift_Current);
    }
    if input == "America/Tegucigalpa" {
        return Some(America::Tegucigalpa);
    }
    if input == "America/Thule" {
        return Some(America::Thule);
    }
    if input == "America/Thunder_Bay" {
        return Some(America::Thunder_Bay);
    }
    if input == "America/Tijuana" {
        return Some(America::Tijuana);
    }
    if input == "America/Toronto" {
        return Some(America::Toronto);
    }
    if input == "America/Vancouver" {
        return Some(America::Vancouver);
    }
    if input == "America/Whitehorse" {
        return Some(America::Whitehorse);
    }
    if input == "America/Winnipeg" {
        return Some(America::Winnipeg);
    }
    if input == "America/Yakutat" {
        return Some(America::Yakutat);
    }
    if input == "America/Yellowknife" {
        return Some(America::Yellowknife);
    }
    if input == "Antarctica/Casey" {
        return Some(Antarctica::Casey);
    }
    if input == "Antarctica/Davis" {
        return Some(Antarctica::Davis);
    }
    if input == "Antarctica/DumontDUrville" {
        return Some(Antarctica::DumontDUrville);
    }
    if input == "Antarctica/Macquarie" {
        return Some(Antarctica::Macquarie);
    }
    if input == "Antarctica/Mawson" {
        return Some(Antarctica::Mawson);
    }
    if input == "Antarctica/Palmer" {
        return Some(Antarctica::Palmer);
    }
    if input == "Antarctica/Rothera" {
        return Some(Antarctica::Rothera);
    }
    if input == "Antarctica/Syowa" {
        return Some(Antarctica::Syowa);
    }
    if input == "Antarctica/Troll" {
        return Some(Antarctica::Troll);
    }
    if input == "Antarctica/Vostok" {
        return Some(Antarctica::Vostok);
    }
    if input == "Asia/Almaty" {
        return Some(Asia::Almaty);
    }
    if input == "Asia/Amman" {
        return Some(Asia::Amman);
    }
    if input == "Asia/Anadyr" {
        return Some(Asia::Anadyr);
    }
    if input == "Asia/Aqtau" {
        return Some(Asia::Aqtau);
    }
    if input == "Asia/Aqtobe" {
        return Some(Asia::Aqtobe);
    }
    if input == "Asia/Ashgabat" {
        return Some(Asia::Ashgabat);
    }
    if input == "Asia/Baghdad" {
        return Some(Asia::Baghdad);
    }
    if input == "Asia/Baku" {
        return Some(Asia::Baku);
    }
    if input == "Asia/Bangkok" {
        return Some(Asia::Bangkok);
    }
    if input == "Asia/Beirut" {
        return Some(Asia::Beirut);
    }
    if input == "Asia/Bishkek" {
        return Some(Asia::Bishkek);
    }
    if input == "Asia/Brunei" {
        return Some(Asia::Brunei);
    }
    if input == "Asia/Chita" {
        return Some(Asia::Chita);
    }
    if input == "Asia/Choibalsan" {
        return Some(Asia::Choibalsan);
    }
    if input == "Asia/Colombo" {
        return Some(Asia::Colombo);
    }
    if input == "Asia/Damascus" {
        return Some(Asia::Damascus);
    }
    if input == "Asia/Dhaka" {
        return Some(Asia::Dhaka);
    }
    if input == "Asia/Dili" {
        return Some(Asia::Dili);
    }
    if input == "Asia/Dubai" {
        return Some(Asia::Dubai);
    }
    if input == "Asia/Dushanbe" {
        return Some(Asia::Dushanbe);
    }
    if input == "Asia/Gaza" {
        return Some(Asia::Gaza);
    }
    if input == "Asia/Hebron" {
        return Some(Asia::Hebron);
    }
    if input == "Asia/Ho_Chi_Minh" {
        return Some(Asia::Ho_Chi_Minh);
    }
    if input == "Asia/Hong_Kong" {
        return Some(Asia::Hong_Kong);
    }
    if input == "Asia/Hovd" {
        return Some(Asia::Hovd);
    }
    if input == "Asia/Irkutsk" {
        return Some(Asia::Irkutsk);
    }
    if input == "Asia/Jakarta" {
        return Some(Asia::Jakarta);
    }
    if input == "Asia/Jayapura" {
        return Some(Asia::Jayapura);
    }
    if input == "Asia/Jerusalem" {
        return Some(Asia::Jerusalem);
    }
    if input == "Asia/Kabul" {
        return Some(Asia::Kabul);
    }
    if input == "Asia/Kamchatka" {
        return Some(Asia::Kamchatka);
    }
    if input == "Asia/Karachi" {
        return Some(Asia::Karachi);
    }
    if input == "Asia/Kathmandu" {
        return Some(Asia::Kathmandu);
    }
    if input == "Asia/Khandyga" {
        return Some(Asia::Khandyga);
    }
    if input == "Asia/Kolkata" {
        return Some(Asia::Kolkata);
    }
    if input == "Asia/Krasnoyarsk" {
        return Some(Asia::Krasnoyarsk);
    }
    if input == "Asia/Kuala_Lumpur" {
        return Some(Asia::Kuala_Lumpur);
    }
    if input == "Asia/Kuching" {
        return Some(Asia::Kuching);
    }
    if input == "Asia/Macau" {
        return Some(Asia::Macau);
    }
    if input == "Asia/Magadan" {
        return Some(Asia::Magadan);
    }
    if input == "Asia/Makassar" {
        return Some(Asia::Makassar);
    }
    if input == "Asia/Manila" {
        return Some(Asia::Manila);
    }
    if input == "Asia/Nicosia" {
        return Some(Asia::Nicosia);
    }
    if input == "Asia/Novokuznetsk" {
        return Some(Asia::Novokuznetsk);
    }
    if input == "Asia/Novosibirsk" {
        return Some(Asia::Novosibirsk);
    }
    if input == "Asia/Omsk" {
        return Some(Asia::Omsk);
    }
    if input == "Asia/Oral" {
        return Some(Asia::Oral);
    }
    if input == "Asia/Pontianak" {
        return Some(Asia::Pontianak);
    }
    if input == "Asia/Pyongyang" {
        return Some(Asia::Pyongyang);
    }
    if input == "Asia/Qatar" {
        return Some(Asia::Qatar);
    }
    if input == "Asia/Qyzylorda" {
        return Some(Asia::Qyzylorda);
    }
    if input == "Asia/Rangoon" {
        return Some(Asia::Rangoon);
    }
    if input == "Asia/Riyadh" {
        return Some(Asia::Riyadh);
    }
    if input == "Asia/Sakhalin" {
        return Some(Asia::Sakhalin);
    }
    if input == "Asia/Samarkand" {
        return Some(Asia::Samarkand);
    }
    if input == "Asia/Seoul" {
        return Some(Asia::Seoul);
    }
    if input == "Asia/Shanghai" {
        return Some(Asia::Shanghai);
    }
    if input == "Asia/Singapore" {
        return Some(Asia::Singapore);
    }
    if input == "Asia/Srednekolymsk" {
        return Some(Asia::Srednekolymsk);
    }
    if input == "Asia/Taipei" {
        return Some(Asia::Taipei);
    }
    if input == "Asia/Tashkent" {
        return Some(Asia::Tashkent);
    }
    if input == "Asia/Tbilisi" {
        return Some(Asia::Tbilisi);
    }
    if input == "Asia/Tehran" {
        return Some(Asia::Tehran);
    }
    if input == "Asia/Thimphu" {
        return Some(Asia::Thimphu);
    }
    if input == "Asia/Tokyo" {
        return Some(Asia::Tokyo);
    }
    if input == "Asia/Ulaanbaatar" {
        return Some(Asia::Ulaanbaatar);
    }
    if input == "Asia/Urumqi" {
        return Some(Asia::Urumqi);
    }
    if input == "Asia/Ust-Nera" {
        return Some(Asia::Ust_Nera);
    }
    if input == "Asia/Vladivostok" {
        return Some(Asia::Vladivostok);
    }
    if input == "Asia/Yakutsk" {
        return Some(Asia::Yakutsk);
    }
    if input == "Asia/Yekaterinburg" {
        return Some(Asia::Yekaterinburg);
    }
    if input == "Asia/Yerevan" {
        return Some(Asia::Yerevan);
    }
    if input == "Atlantic/Azores" {
        return Some(Atlantic::Azores);
    }
    if input == "Atlantic/Bermuda" {
        return Some(Atlantic::Bermuda);
    }
    if input == "Atlantic/Canary" {
        return Some(Atlantic::Canary);
    }
    if input == "Atlantic/Cape_Verde" {
        return Some(Atlantic::Cape_Verde);
    }
    if input == "Atlantic/Faroe" {
        return Some(Atlantic::Faroe);
    }
    if input == "Atlantic/Madeira" {
        return Some(Atlantic::Madeira);
    }
    if input == "Atlantic/Reykjavik" {
        return Some(Atlantic::Reykjavik);
    }
    if input == "Atlantic/South_Georgia" {
        return Some(Atlantic::South_Georgia);
    }
    if input == "Atlantic/Stanley" {
        return Some(Atlantic::Stanley);
    }
    if input == "Australia/Adelaide" {
        return Some(Australia::Adelaide);
    }
    if input == "Australia/Brisbane" {
        return Some(Australia::Brisbane);
    }
    if input == "Australia/Broken_Hill" {
        return Some(Australia::Broken_Hill);
    }
    if input == "Australia/Currie" {
        return Some(Australia::Currie);
    }
    if input == "Australia/Darwin" {
        return Some(Australia::Darwin);
    }
    if input == "Australia/Eucla" {
        return Some(Australia::Eucla);
    }
    if input == "Australia/Hobart" {
        return Some(Australia::Hobart);
    }
    if input == "Australia/Lindeman" {
        return Some(Australia::Lindeman);
    }
    if input == "Australia/Lord_Howe" {
        return Some(Australia::Lord_Howe);
    }
    if input == "Australia/Melbourne" {
        return Some(Australia::Melbourne);
    }
    if input == "Australia/Perth" {
        return Some(Australia::Perth);
    }
    if input == "Australia/Sydney" {
        return Some(Australia::Sydney);
    }
    if input == "CET" {
        return Some(CET);
    }
    if input == "CST6CDT" {
        return Some(CST6CDT);
    }
    if input == "EET" {
        return Some(EET);
    }
    if input == "EST" {
        return Some(EST);
    }
    if input == "EST5EDT" {
        return Some(EST5EDT);
    }
    if input == "Europe/Amsterdam" {
        return Some(Europe::Amsterdam);
    }
    if input == "Europe/Andorra" {
        return Some(Europe::Andorra);
    }
    if input == "Europe/Athens" {
        return Some(Europe::Athens);
    }
    if input == "Europe/Belgrade" {
        return Some(Europe::Belgrade);
    }
    if input == "Europe/Berlin" {
        return Some(Europe::Berlin);
    }
    if input == "Europe/Brussels" {
        return Some(Europe::Brussels);
    }
    if input == "Europe/Bucharest" {
        return Some(Europe::Bucharest);
    }
    if input == "Europe/Budapest" {
        return Some(Europe::Budapest);
    }
    if input == "Europe/Chisinau" {
        return Some(Europe::Chisinau);
    }
    if input == "Europe/Copenhagen" {
        return Some(Europe::Copenhagen);
    }
    if input == "Europe/Dublin" {
        return Some(Europe::Dublin);
    }
    if input == "Europe/Gibraltar" {
        return Some(Europe::Gibraltar);
    }
    if input == "Europe/Helsinki" {
        return Some(Europe::Helsinki);
    }
    if input == "Europe/Istanbul" {
        return Some(Europe::Istanbul);
    }
    if input == "Europe/Kaliningrad" {
        return Some(Europe::Kaliningrad);
    }
    if input == "Europe/Kiev" {
        return Some(Europe::Kiev);
    }
    if input == "Europe/Lisbon" {
        return Some(Europe::Lisbon);
    }
    if input == "Europe/London" {
        return Some(Europe::London);
    }
    if input == "Europe/Luxembourg" {
        return Some(Europe::Luxembourg);
    }
    if input == "Europe/Madrid" {
        return Some(Europe::Madrid);
    }
    if input == "Europe/Malta" {
        return Some(Europe::Malta);
    }
    if input == "Europe/Minsk" {
        return Some(Europe::Minsk);
    }
    if input == "Europe/Monaco" {
        return Some(Europe::Monaco);
    }
    if input == "Europe/Moscow" {
        return Some(Europe::Moscow);
    }
    if input == "Europe/Oslo" {
        return Some(Europe::Oslo);
    }
    if input == "Europe/Paris" {
        return Some(Europe::Paris);
    }
    if input == "Europe/Prague" {
        return Some(Europe::Prague);
    }
    if input == "Europe/Riga" {
        return Some(Europe::Riga);
    }
    if input == "Europe/Rome" {
        return Some(Europe::Rome);
    }
    if input == "Europe/Samara" {
        return Some(Europe::Samara);
    }
    if input == "Europe/Simferopol" {
        return Some(Europe::Simferopol);
    }
    if input == "Europe/Sofia" {
        return Some(Europe::Sofia);
    }
    if input == "Europe/Stockholm" {
        return Some(Europe::Stockholm);
    }
    if input == "Europe/Tallinn" {
        return Some(Europe::Tallinn);
    }
    if input == "Europe/Tirane" {
        return Some(Europe::Tirane);
    }
    if input == "Europe/Uzhgorod" {
        return Some(Europe::Uzhgorod);
    }
    if input == "Europe/Vienna" {
        return Some(Europe::Vienna);
    }
    if input == "Europe/Vilnius" {
        return Some(Europe::Vilnius);
    }
    if input == "Europe/Volgograd" {
        return Some(Europe::Volgograd);
    }
    if input == "Europe/Warsaw" {
        return Some(Europe::Warsaw);
    }
    if input == "Europe/Zaporozhye" {
        return Some(Europe::Zaporozhye);
    }
    if input == "Europe/Zurich" {
        return Some(Europe::Zurich);
    }
    if input == "HST" {
        return Some(HST);
    }
    if input == "Indian/Chagos" {
        return Some(Indian::Chagos);
    }
    if input == "Indian/Christmas" {
        return Some(Indian::Christmas);
    }
    if input == "Indian/Cocos" {
        return Some(Indian::Cocos);
    }
    if input == "Indian/Kerguelen" {
        return Some(Indian::Kerguelen);
    }
    if input == "Indian/Mahe" {
        return Some(Indian::Mahe);
    }
    if input == "Indian/Maldives" {
        return Some(Indian::Maldives);
    }
    if input == "Indian/Mauritius" {
        return Some(Indian::Mauritius);
    }
    if input == "Indian/Reunion" {
        return Some(Indian::Reunion);
    }
    if input == "MET" {
        return Some(MET);
    }
    if input == "MST" {
        return Some(MST);
    }
    if input == "MST7MDT" {
        return Some(MST7MDT);
    }
    if input == "PST8PDT" {
        return Some(PST8PDT);
    }
    if input == "Pacific/Apia" {
        return Some(Pacific::Apia);
    }
    if input == "Pacific/Auckland" {
        return Some(Pacific::Auckland);
    }
    if input == "Pacific/Bougainville" {
        return Some(Pacific::Bougainville);
    }
    if input == "Pacific/Chatham" {
        return Some(Pacific::Chatham);
    }
    if input == "Pacific/Chuuk" {
        return Some(Pacific::Chuuk);
    }
    if input == "Pacific/Easter" {
        return Some(Pacific::Easter);
    }
    if input == "Pacific/Efate" {
        return Some(Pacific::Efate);
    }
    if input == "Pacific/Enderbury" {
        return Some(Pacific::Enderbury);
    }
    if input == "Pacific/Fakaofo" {
        return Some(Pacific::Fakaofo);
    }
    if input == "Pacific/Fiji" {
        return Some(Pacific::Fiji);
    }
    if input == "Pacific/Funafuti" {
        return Some(Pacific::Funafuti);
    }
    if input == "Pacific/Galapagos" {
        return Some(Pacific::Galapagos);
    }
    if input == "Pacific/Gambier" {
        return Some(Pacific::Gambier);
    }
    if input == "Pacific/Guadalcanal" {
        return Some(Pacific::Guadalcanal);
    }
    if input == "Pacific/Guam" {
        return Some(Pacific::Guam);
    }
    if input == "Pacific/Honolulu" {
        return Some(Pacific::Honolulu);
    }
    if input == "Pacific/Kiritimati" {
        return Some(Pacific::Kiritimati);
    }
    if input == "Pacific/Kosrae" {
        return Some(Pacific::Kosrae);
    }
    if input == "Pacific/Kwajalein" {
        return Some(Pacific::Kwajalein);
    }
    if input == "Pacific/Majuro" {
        return Some(Pacific::Majuro);
    }
    if input == "Pacific/Marquesas" {
        return Some(Pacific::Marquesas);
    }
    if input == "Pacific/Nauru" {
        return Some(Pacific::Nauru);
    }
    if input == "Pacific/Niue" {
        return Some(Pacific::Niue);
    }
    if input == "Pacific/Norfolk" {
        return Some(Pacific::Norfolk);
    }
    if input == "Pacific/Noumea" {
        return Some(Pacific::Noumea);
    }
    if input == "Pacific/Pago_Pago" {
        return Some(Pacific::Pago_Pago);
    }
    if input == "Pacific/Palau" {
        return Some(Pacific::Palau);
    }
    if input == "Pacific/Pitcairn" {
        return Some(Pacific::Pitcairn);
    }
    if input == "Pacific/Pohnpei" {
        return Some(Pacific::Pohnpei);
    }
    if input == "Pacific/Port_Moresby" {
        return Some(Pacific::Port_Moresby);
    }
    if input == "Pacific/Rarotonga" {
        return Some(Pacific::Rarotonga);
    }
    if input == "Pacific/Tahiti" {
        return Some(Pacific::Tahiti);
    }
    if input == "Pacific/Tarawa" {
        return Some(Pacific::Tarawa);
    }
    if input == "Pacific/Tongatapu" {
        return Some(Pacific::Tongatapu);
    }
    if input == "Pacific/Wake" {
        return Some(Pacific::Wake);
    }
    if input == "Pacific/Wallis" {
        return Some(Pacific::Wallis);
    }
    if input == "WET" {
        return Some(WET);
    }
    return None;
}
