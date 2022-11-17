#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Iso3AlphaCurrencyCodeContentType {
    /// xml value == 'AED'
    Aed,
    /// xml value == 'AFN'
    Afn,
    /// xml value == 'ALL'
    All,
    /// xml value == 'AMD'
    Amd,
    /// xml value == 'ANG'
    Ang,
    /// xml value == 'AOA'
    Aoa,
    /// xml value == 'ARS'
    Ars,
    /// xml value == 'AUD'
    Aud,
    /// xml value == 'AWG'
    Awg,
    /// xml value == 'AZN'
    Azn,
    /// xml value == 'BAM'
    Bam,
    /// xml value == 'BBD'
    Bbd,
    /// xml value == 'BDT'
    Bdt,
    /// xml value == 'BGN'
    Bgn,
    /// xml value == 'BHD'
    Bhd,
    /// xml value == 'BIF'
    Bif,
    /// xml value == 'BMD'
    Bmd,
    /// xml value == 'BND'
    Bnd,
    /// xml value == 'BOB'
    Bob,
    /// xml value == 'BOV'
    Bov,
    /// xml value == 'BRL'
    Brl,
    /// xml value == 'BSD'
    Bsd,
    /// xml value == 'BTN'
    Btn,
    /// xml value == 'BWP'
    Bwp,
    /// xml value == 'BYR'
    Byr,
    /// xml value == 'BZD'
    Bzd,
    /// xml value == 'CAD'
    Cad,
    /// xml value == 'CDF'
    Cdf,
    /// xml value == 'CHE'
    Che,
    /// xml value == 'CHF'
    Chf,
    /// xml value == 'CHW'
    Chw,
    /// xml value == 'CLF'
    Clf,
    /// xml value == 'CLP'
    Clp,
    /// xml value == 'CNY'
    Cny,
    /// xml value == 'COP'
    Cop,
    /// xml value == 'COU'
    Cou,
    /// xml value == 'CRC'
    Crc,
    /// xml value == 'CUC'
    Cuc,
    /// xml value == 'CUP'
    Cup,
    /// xml value == 'CVE'
    Cve,
    /// xml value == 'CZK'
    Czk,
    /// xml value == 'DJF'
    Djf,
    /// xml value == 'DKK'
    Dkk,
    /// xml value == 'DOP'
    Dop,
    /// xml value == 'DZD'
    Dzd,
    /// xml value == 'EEK'
    Eek,
    /// xml value == 'EGP'
    Egp,
    /// xml value == 'ERN'
    Ern,
    /// xml value == 'ETB'
    Etb,
    /// xml value == 'EUR'
    Eur,
    /// xml value == 'FJD'
    Fjd,
    /// xml value == 'FKP'
    Fkp,
    /// xml value == 'GBP'
    Gbp,
    /// xml value == 'GEL'
    Gel,
    /// xml value == 'GHS'
    Ghs,
    /// xml value == 'GIP'
    Gip,
    /// xml value == 'GMD'
    Gmd,
    /// xml value == 'GNF'
    Gnf,
    /// xml value == 'GTQ'
    Gtq,
    /// xml value == 'GWP'
    Gwp,
    /// xml value == 'GYD'
    Gyd,
    /// xml value == 'HKD'
    Hkd,
    /// xml value == 'HNL'
    Hnl,
    /// xml value == 'HRK'
    Hrk,
    /// xml value == 'HTG'
    Htg,
    /// xml value == 'HUF'
    Huf,
    /// xml value == 'IDR'
    Idr,
    /// xml value == 'ILS'
    Ils,
    /// xml value == 'INR'
    Inr,
    /// xml value == 'IQD'
    Iqd,
    /// xml value == 'IRR'
    Irr,
    /// xml value == 'ISK'
    Isk,
    /// xml value == 'JMD'
    Jmd,
    /// xml value == 'JOD'
    Jod,
    /// xml value == 'JPY'
    Jpy,
    /// xml value == 'KES'
    Kes,
    /// xml value == 'KGS'
    Kgs,
    /// xml value == 'KHR'
    Khr,
    /// xml value == 'KMF'
    Kmf,
    /// xml value == 'KPW'
    Kpw,
    /// xml value == 'KRW'
    Krw,
    /// xml value == 'KWD'
    Kwd,
    /// xml value == 'KYD'
    Kyd,
    /// xml value == 'KZT'
    Kzt,
    /// xml value == 'LAK'
    Lak,
    /// xml value == 'LBP'
    Lbp,
    /// xml value == 'LKR'
    Lkr,
    /// xml value == 'LRD'
    Lrd,
    /// xml value == 'LSL'
    Lsl,
    /// xml value == 'LTL'
    Ltl,
    /// xml value == 'LVL'
    Lvl,
    /// xml value == 'LYD'
    Lyd,
    /// xml value == 'MAD'
    Mad,
    /// xml value == 'MDL'
    Mdl,
    /// xml value == 'MGA'
    Mga,
    /// xml value == 'MKD'
    Mkd,
    /// xml value == 'MMK'
    Mmk,
    /// xml value == 'MNT'
    Mnt,
    /// xml value == 'MOP'
    Mop,
    /// xml value == 'MRO'
    Mro,
    /// xml value == 'MUR'
    Mur,
    /// xml value == 'MVR'
    Mvr,
    /// xml value == 'MWK'
    Mwk,
    /// xml value == 'MXN'
    Mxn,
    /// xml value == 'MXV'
    Mxv,
    /// xml value == 'MYR'
    Myr,
    /// xml value == 'MZN'
    Mzn,
    /// xml value == 'NAD'
    Nad,
    /// xml value == 'NGN'
    Ngn,
    /// xml value == 'NIO'
    Nio,
    /// xml value == 'NOK'
    Nok,
    /// xml value == 'NPR'
    Npr,
    /// xml value == 'NZD'
    Nzd,
    /// xml value == 'OMR'
    Omr,
    /// xml value == 'PAB'
    Pab,
    /// xml value == 'PEN'
    Pen,
    /// xml value == 'PGK'
    Pgk,
    /// xml value == 'PHP'
    Php,
    /// xml value == 'PKR'
    Pkr,
    /// xml value == 'PLN'
    Pln,
    /// xml value == 'PYG'
    Pyg,
    /// xml value == 'QAR'
    Qar,
    /// xml value == 'RON'
    Ron,
    /// xml value == 'RSD'
    Rsd,
    /// xml value == 'RUB'
    Rub,
    /// xml value == 'RWF'
    Rwf,
    /// xml value == 'SAR'
    Sar,
    /// xml value == 'SBD'
    Sbd,
    /// xml value == 'SCR'
    Scr,
    /// xml value == 'SDG'
    Sdg,
    /// xml value == 'SEK'
    Sek,
    /// xml value == 'SGD'
    Sgd,
    /// xml value == 'SHP'
    Shp,
    /// xml value == 'SLL'
    Sll,
    /// xml value == 'SOS'
    Sos,
    /// xml value == 'SRD'
    Srd,
    /// xml value == 'STD'
    Std,
    /// xml value == 'SVC'
    Svc,
    /// xml value == 'SYP'
    Syp,
    /// xml value == 'SZL'
    Szl,
    /// xml value == 'THB'
    Thb,
    /// xml value == 'TJS'
    Tjs,
    /// xml value == 'TMT'
    Tmt,
    /// xml value == 'TND'
    Tnd,
    /// xml value == 'TOP'
    Top,
    /// xml value == 'TRY'
    Try,
    /// xml value == 'TTD'
    Ttd,
    /// xml value == 'TWD'
    Twd,
    /// xml value == 'TZS'
    Tzs,
    /// xml value == 'UAH'
    Uah,
    /// xml value == 'UGX'
    Ugx,
    /// xml value == 'USD'
    Usd,
    /// xml value == 'USN'
    Usn,
    /// xml value == 'USS'
    Uss,
    /// xml value == 'UYI'
    Uyi,
    /// xml value == 'UYU'
    Uyu,
    /// xml value == 'UZS'
    Uzs,
    /// xml value == 'VEF'
    Vef,
    /// xml value == 'VND'
    Vnd,
    /// xml value == 'VUV'
    Vuv,
    /// xml value == 'WST'
    Wst,
    /// xml value == 'XAF'
    Xaf,
    /// xml value == 'XAG'
    Xag,
    /// xml value == 'XAU'
    Xau,
    /// xml value == 'XBA'
    Xba,
    /// xml value == 'XBB'
    Xbb,
    /// xml value == 'XBC'
    Xbc,
    /// xml value == 'XBD'
    Xbd,
    /// xml value == 'XCD'
    Xcd,
    /// xml value == 'XDR'
    Xdr,
    /// xml value == 'XFU'
    Xfu,
    /// xml value == 'XOF'
    Xof,
    /// xml value == 'XPD'
    Xpd,
    /// xml value == 'XPF'
    Xpf,
    /// xml value == 'XPT'
    Xpt,
    /// xml value == 'XTS'
    Xts,
    /// xml value == 'XXX'
    Xxx,
    /// xml value == 'YER'
    Yer,
    /// xml value == 'ZAR'
    Zar,
    /// xml value == 'ZMK'
    Zmk,
    /// xml value == 'ZWL'
    Zwl,
}

impl Iso3AlphaCurrencyCodeContentType {
    pub(crate) fn from_str(s: &str) -> Option<Self> {
        match s {
            "AED" => Some(Self::Aed),
            "AFN" => Some(Self::Afn),
            "ALL" => Some(Self::All),
            "AMD" => Some(Self::Amd),
            "ANG" => Some(Self::Ang),
            "AOA" => Some(Self::Aoa),
            "ARS" => Some(Self::Ars),
            "AUD" => Some(Self::Aud),
            "AWG" => Some(Self::Awg),
            "AZN" => Some(Self::Azn),
            "BAM" => Some(Self::Bam),
            "BBD" => Some(Self::Bbd),
            "BDT" => Some(Self::Bdt),
            "BGN" => Some(Self::Bgn),
            "BHD" => Some(Self::Bhd),
            "BIF" => Some(Self::Bif),
            "BMD" => Some(Self::Bmd),
            "BND" => Some(Self::Bnd),
            "BOB" => Some(Self::Bob),
            "BOV" => Some(Self::Bov),
            "BRL" => Some(Self::Brl),
            "BSD" => Some(Self::Bsd),
            "BTN" => Some(Self::Btn),
            "BWP" => Some(Self::Bwp),
            "BYR" => Some(Self::Byr),
            "BZD" => Some(Self::Bzd),
            "CAD" => Some(Self::Cad),
            "CDF" => Some(Self::Cdf),
            "CHE" => Some(Self::Che),
            "CHF" => Some(Self::Chf),
            "CHW" => Some(Self::Chw),
            "CLF" => Some(Self::Clf),
            "CLP" => Some(Self::Clp),
            "CNY" => Some(Self::Cny),
            "COP" => Some(Self::Cop),
            "COU" => Some(Self::Cou),
            "CRC" => Some(Self::Crc),
            "CUC" => Some(Self::Cuc),
            "CUP" => Some(Self::Cup),
            "CVE" => Some(Self::Cve),
            "CZK" => Some(Self::Czk),
            "DJF" => Some(Self::Djf),
            "DKK" => Some(Self::Dkk),
            "DOP" => Some(Self::Dop),
            "DZD" => Some(Self::Dzd),
            "EEK" => Some(Self::Eek),
            "EGP" => Some(Self::Egp),
            "ERN" => Some(Self::Ern),
            "ETB" => Some(Self::Etb),
            "EUR" => Some(Self::Eur),
            "FJD" => Some(Self::Fjd),
            "FKP" => Some(Self::Fkp),
            "GBP" => Some(Self::Gbp),
            "GEL" => Some(Self::Gel),
            "GHS" => Some(Self::Ghs),
            "GIP" => Some(Self::Gip),
            "GMD" => Some(Self::Gmd),
            "GNF" => Some(Self::Gnf),
            "GTQ" => Some(Self::Gtq),
            "GWP" => Some(Self::Gwp),
            "GYD" => Some(Self::Gyd),
            "HKD" => Some(Self::Hkd),
            "HNL" => Some(Self::Hnl),
            "HRK" => Some(Self::Hrk),
            "HTG" => Some(Self::Htg),
            "HUF" => Some(Self::Huf),
            "IDR" => Some(Self::Idr),
            "ILS" => Some(Self::Ils),
            "INR" => Some(Self::Inr),
            "IQD" => Some(Self::Iqd),
            "IRR" => Some(Self::Irr),
            "ISK" => Some(Self::Isk),
            "JMD" => Some(Self::Jmd),
            "JOD" => Some(Self::Jod),
            "JPY" => Some(Self::Jpy),
            "KES" => Some(Self::Kes),
            "KGS" => Some(Self::Kgs),
            "KHR" => Some(Self::Khr),
            "KMF" => Some(Self::Kmf),
            "KPW" => Some(Self::Kpw),
            "KRW" => Some(Self::Krw),
            "KWD" => Some(Self::Kwd),
            "KYD" => Some(Self::Kyd),
            "KZT" => Some(Self::Kzt),
            "LAK" => Some(Self::Lak),
            "LBP" => Some(Self::Lbp),
            "LKR" => Some(Self::Lkr),
            "LRD" => Some(Self::Lrd),
            "LSL" => Some(Self::Lsl),
            "LTL" => Some(Self::Ltl),
            "LVL" => Some(Self::Lvl),
            "LYD" => Some(Self::Lyd),
            "MAD" => Some(Self::Mad),
            "MDL" => Some(Self::Mdl),
            "MGA" => Some(Self::Mga),
            "MKD" => Some(Self::Mkd),
            "MMK" => Some(Self::Mmk),
            "MNT" => Some(Self::Mnt),
            "MOP" => Some(Self::Mop),
            "MRO" => Some(Self::Mro),
            "MUR" => Some(Self::Mur),
            "MVR" => Some(Self::Mvr),
            "MWK" => Some(Self::Mwk),
            "MXN" => Some(Self::Mxn),
            "MXV" => Some(Self::Mxv),
            "MYR" => Some(Self::Myr),
            "MZN" => Some(Self::Mzn),
            "NAD" => Some(Self::Nad),
            "NGN" => Some(Self::Ngn),
            "NIO" => Some(Self::Nio),
            "NOK" => Some(Self::Nok),
            "NPR" => Some(Self::Npr),
            "NZD" => Some(Self::Nzd),
            "OMR" => Some(Self::Omr),
            "PAB" => Some(Self::Pab),
            "PEN" => Some(Self::Pen),
            "PGK" => Some(Self::Pgk),
            "PHP" => Some(Self::Php),
            "PKR" => Some(Self::Pkr),
            "PLN" => Some(Self::Pln),
            "PYG" => Some(Self::Pyg),
            "QAR" => Some(Self::Qar),
            "RON" => Some(Self::Ron),
            "RSD" => Some(Self::Rsd),
            "RUB" => Some(Self::Rub),
            "RWF" => Some(Self::Rwf),
            "SAR" => Some(Self::Sar),
            "SBD" => Some(Self::Sbd),
            "SCR" => Some(Self::Scr),
            "SDG" => Some(Self::Sdg),
            "SEK" => Some(Self::Sek),
            "SGD" => Some(Self::Sgd),
            "SHP" => Some(Self::Shp),
            "SLL" => Some(Self::Sll),
            "SOS" => Some(Self::Sos),
            "SRD" => Some(Self::Srd),
            "STD" => Some(Self::Std),
            "SVC" => Some(Self::Svc),
            "SYP" => Some(Self::Syp),
            "SZL" => Some(Self::Szl),
            "THB" => Some(Self::Thb),
            "TJS" => Some(Self::Tjs),
            "TMT" => Some(Self::Tmt),
            "TND" => Some(Self::Tnd),
            "TOP" => Some(Self::Top),
            "TRY" => Some(Self::Try),
            "TTD" => Some(Self::Ttd),
            "TWD" => Some(Self::Twd),
            "TZS" => Some(Self::Tzs),
            "UAH" => Some(Self::Uah),
            "UGX" => Some(Self::Ugx),
            "USD" => Some(Self::Usd),
            "USN" => Some(Self::Usn),
            "USS" => Some(Self::Uss),
            "UYI" => Some(Self::Uyi),
            "UYU" => Some(Self::Uyu),
            "UZS" => Some(Self::Uzs),
            "VEF" => Some(Self::Vef),
            "VND" => Some(Self::Vnd),
            "VUV" => Some(Self::Vuv),
            "WST" => Some(Self::Wst),
            "XAF" => Some(Self::Xaf),
            "XAG" => Some(Self::Xag),
            "XAU" => Some(Self::Xau),
            "XBA" => Some(Self::Xba),
            "XBB" => Some(Self::Xbb),
            "XBC" => Some(Self::Xbc),
            "XBD" => Some(Self::Xbd),
            "XCD" => Some(Self::Xcd),
            "XDR" => Some(Self::Xdr),
            "XFU" => Some(Self::Xfu),
            "XOF" => Some(Self::Xof),
            "XPD" => Some(Self::Xpd),
            "XPF" => Some(Self::Xpf),
            "XPT" => Some(Self::Xpt),
            "XTS" => Some(Self::Xts),
            "XXX" => Some(Self::Xxx),
            "YER" => Some(Self::Yer),
            "ZAR" => Some(Self::Zar),
            "ZMK" => Some(Self::Zmk),
            "ZWL" => Some(Self::Zwl),
            _ => None,
        }
    }

    pub(crate) fn to_str(self) -> &'static str {
        match self {
            Self::Aed => "AED",
            Self::Afn => "AFN",
            Self::All => "ALL",
            Self::Amd => "AMD",
            Self::Ang => "ANG",
            Self::Aoa => "AOA",
            Self::Ars => "ARS",
            Self::Aud => "AUD",
            Self::Awg => "AWG",
            Self::Azn => "AZN",
            Self::Bam => "BAM",
            Self::Bbd => "BBD",
            Self::Bdt => "BDT",
            Self::Bgn => "BGN",
            Self::Bhd => "BHD",
            Self::Bif => "BIF",
            Self::Bmd => "BMD",
            Self::Bnd => "BND",
            Self::Bob => "BOB",
            Self::Bov => "BOV",
            Self::Brl => "BRL",
            Self::Bsd => "BSD",
            Self::Btn => "BTN",
            Self::Bwp => "BWP",
            Self::Byr => "BYR",
            Self::Bzd => "BZD",
            Self::Cad => "CAD",
            Self::Cdf => "CDF",
            Self::Che => "CHE",
            Self::Chf => "CHF",
            Self::Chw => "CHW",
            Self::Clf => "CLF",
            Self::Clp => "CLP",
            Self::Cny => "CNY",
            Self::Cop => "COP",
            Self::Cou => "COU",
            Self::Crc => "CRC",
            Self::Cuc => "CUC",
            Self::Cup => "CUP",
            Self::Cve => "CVE",
            Self::Czk => "CZK",
            Self::Djf => "DJF",
            Self::Dkk => "DKK",
            Self::Dop => "DOP",
            Self::Dzd => "DZD",
            Self::Eek => "EEK",
            Self::Egp => "EGP",
            Self::Ern => "ERN",
            Self::Etb => "ETB",
            Self::Eur => "EUR",
            Self::Fjd => "FJD",
            Self::Fkp => "FKP",
            Self::Gbp => "GBP",
            Self::Gel => "GEL",
            Self::Ghs => "GHS",
            Self::Gip => "GIP",
            Self::Gmd => "GMD",
            Self::Gnf => "GNF",
            Self::Gtq => "GTQ",
            Self::Gwp => "GWP",
            Self::Gyd => "GYD",
            Self::Hkd => "HKD",
            Self::Hnl => "HNL",
            Self::Hrk => "HRK",
            Self::Htg => "HTG",
            Self::Huf => "HUF",
            Self::Idr => "IDR",
            Self::Ils => "ILS",
            Self::Inr => "INR",
            Self::Iqd => "IQD",
            Self::Irr => "IRR",
            Self::Isk => "ISK",
            Self::Jmd => "JMD",
            Self::Jod => "JOD",
            Self::Jpy => "JPY",
            Self::Kes => "KES",
            Self::Kgs => "KGS",
            Self::Khr => "KHR",
            Self::Kmf => "KMF",
            Self::Kpw => "KPW",
            Self::Krw => "KRW",
            Self::Kwd => "KWD",
            Self::Kyd => "KYD",
            Self::Kzt => "KZT",
            Self::Lak => "LAK",
            Self::Lbp => "LBP",
            Self::Lkr => "LKR",
            Self::Lrd => "LRD",
            Self::Lsl => "LSL",
            Self::Ltl => "LTL",
            Self::Lvl => "LVL",
            Self::Lyd => "LYD",
            Self::Mad => "MAD",
            Self::Mdl => "MDL",
            Self::Mga => "MGA",
            Self::Mkd => "MKD",
            Self::Mmk => "MMK",
            Self::Mnt => "MNT",
            Self::Mop => "MOP",
            Self::Mro => "MRO",
            Self::Mur => "MUR",
            Self::Mvr => "MVR",
            Self::Mwk => "MWK",
            Self::Mxn => "MXN",
            Self::Mxv => "MXV",
            Self::Myr => "MYR",
            Self::Mzn => "MZN",
            Self::Nad => "NAD",
            Self::Ngn => "NGN",
            Self::Nio => "NIO",
            Self::Nok => "NOK",
            Self::Npr => "NPR",
            Self::Nzd => "NZD",
            Self::Omr => "OMR",
            Self::Pab => "PAB",
            Self::Pen => "PEN",
            Self::Pgk => "PGK",
            Self::Php => "PHP",
            Self::Pkr => "PKR",
            Self::Pln => "PLN",
            Self::Pyg => "PYG",
            Self::Qar => "QAR",
            Self::Ron => "RON",
            Self::Rsd => "RSD",
            Self::Rub => "RUB",
            Self::Rwf => "RWF",
            Self::Sar => "SAR",
            Self::Sbd => "SBD",
            Self::Scr => "SCR",
            Self::Sdg => "SDG",
            Self::Sek => "SEK",
            Self::Sgd => "SGD",
            Self::Shp => "SHP",
            Self::Sll => "SLL",
            Self::Sos => "SOS",
            Self::Srd => "SRD",
            Self::Std => "STD",
            Self::Svc => "SVC",
            Self::Syp => "SYP",
            Self::Szl => "SZL",
            Self::Thb => "THB",
            Self::Tjs => "TJS",
            Self::Tmt => "TMT",
            Self::Tnd => "TND",
            Self::Top => "TOP",
            Self::Try => "TRY",
            Self::Ttd => "TTD",
            Self::Twd => "TWD",
            Self::Tzs => "TZS",
            Self::Uah => "UAH",
            Self::Ugx => "UGX",
            Self::Usd => "USD",
            Self::Usn => "USN",
            Self::Uss => "USS",
            Self::Uyi => "UYI",
            Self::Uyu => "UYU",
            Self::Uzs => "UZS",
            Self::Vef => "VEF",
            Self::Vnd => "VND",
            Self::Vuv => "VUV",
            Self::Wst => "WST",
            Self::Xaf => "XAF",
            Self::Xag => "XAG",
            Self::Xau => "XAU",
            Self::Xba => "XBA",
            Self::Xbb => "XBB",
            Self::Xbc => "XBC",
            Self::Xbd => "XBD",
            Self::Xcd => "XCD",
            Self::Xdr => "XDR",
            Self::Xfu => "XFU",
            Self::Xof => "XOF",
            Self::Xpd => "XPD",
            Self::Xpf => "XPF",
            Self::Xpt => "XPT",
            Self::Xts => "XTS",
            Self::Xxx => "XXX",
            Self::Yer => "YER",
            Self::Zar => "ZAR",
            Self::Zmk => "ZMK",
            Self::Zwl => "ZWL",
        }
    }
}
