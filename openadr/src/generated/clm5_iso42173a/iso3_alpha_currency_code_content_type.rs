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
    pub(crate) fn from_str(s: &str) -> Result<Self, xsd_api::ReadError> {
        match s {
            "AED" => Ok(Self::Aed),
            "AFN" => Ok(Self::Afn),
            "ALL" => Ok(Self::All),
            "AMD" => Ok(Self::Amd),
            "ANG" => Ok(Self::Ang),
            "AOA" => Ok(Self::Aoa),
            "ARS" => Ok(Self::Ars),
            "AUD" => Ok(Self::Aud),
            "AWG" => Ok(Self::Awg),
            "AZN" => Ok(Self::Azn),
            "BAM" => Ok(Self::Bam),
            "BBD" => Ok(Self::Bbd),
            "BDT" => Ok(Self::Bdt),
            "BGN" => Ok(Self::Bgn),
            "BHD" => Ok(Self::Bhd),
            "BIF" => Ok(Self::Bif),
            "BMD" => Ok(Self::Bmd),
            "BND" => Ok(Self::Bnd),
            "BOB" => Ok(Self::Bob),
            "BOV" => Ok(Self::Bov),
            "BRL" => Ok(Self::Brl),
            "BSD" => Ok(Self::Bsd),
            "BTN" => Ok(Self::Btn),
            "BWP" => Ok(Self::Bwp),
            "BYR" => Ok(Self::Byr),
            "BZD" => Ok(Self::Bzd),
            "CAD" => Ok(Self::Cad),
            "CDF" => Ok(Self::Cdf),
            "CHE" => Ok(Self::Che),
            "CHF" => Ok(Self::Chf),
            "CHW" => Ok(Self::Chw),
            "CLF" => Ok(Self::Clf),
            "CLP" => Ok(Self::Clp),
            "CNY" => Ok(Self::Cny),
            "COP" => Ok(Self::Cop),
            "COU" => Ok(Self::Cou),
            "CRC" => Ok(Self::Crc),
            "CUC" => Ok(Self::Cuc),
            "CUP" => Ok(Self::Cup),
            "CVE" => Ok(Self::Cve),
            "CZK" => Ok(Self::Czk),
            "DJF" => Ok(Self::Djf),
            "DKK" => Ok(Self::Dkk),
            "DOP" => Ok(Self::Dop),
            "DZD" => Ok(Self::Dzd),
            "EEK" => Ok(Self::Eek),
            "EGP" => Ok(Self::Egp),
            "ERN" => Ok(Self::Ern),
            "ETB" => Ok(Self::Etb),
            "EUR" => Ok(Self::Eur),
            "FJD" => Ok(Self::Fjd),
            "FKP" => Ok(Self::Fkp),
            "GBP" => Ok(Self::Gbp),
            "GEL" => Ok(Self::Gel),
            "GHS" => Ok(Self::Ghs),
            "GIP" => Ok(Self::Gip),
            "GMD" => Ok(Self::Gmd),
            "GNF" => Ok(Self::Gnf),
            "GTQ" => Ok(Self::Gtq),
            "GWP" => Ok(Self::Gwp),
            "GYD" => Ok(Self::Gyd),
            "HKD" => Ok(Self::Hkd),
            "HNL" => Ok(Self::Hnl),
            "HRK" => Ok(Self::Hrk),
            "HTG" => Ok(Self::Htg),
            "HUF" => Ok(Self::Huf),
            "IDR" => Ok(Self::Idr),
            "ILS" => Ok(Self::Ils),
            "INR" => Ok(Self::Inr),
            "IQD" => Ok(Self::Iqd),
            "IRR" => Ok(Self::Irr),
            "ISK" => Ok(Self::Isk),
            "JMD" => Ok(Self::Jmd),
            "JOD" => Ok(Self::Jod),
            "JPY" => Ok(Self::Jpy),
            "KES" => Ok(Self::Kes),
            "KGS" => Ok(Self::Kgs),
            "KHR" => Ok(Self::Khr),
            "KMF" => Ok(Self::Kmf),
            "KPW" => Ok(Self::Kpw),
            "KRW" => Ok(Self::Krw),
            "KWD" => Ok(Self::Kwd),
            "KYD" => Ok(Self::Kyd),
            "KZT" => Ok(Self::Kzt),
            "LAK" => Ok(Self::Lak),
            "LBP" => Ok(Self::Lbp),
            "LKR" => Ok(Self::Lkr),
            "LRD" => Ok(Self::Lrd),
            "LSL" => Ok(Self::Lsl),
            "LTL" => Ok(Self::Ltl),
            "LVL" => Ok(Self::Lvl),
            "LYD" => Ok(Self::Lyd),
            "MAD" => Ok(Self::Mad),
            "MDL" => Ok(Self::Mdl),
            "MGA" => Ok(Self::Mga),
            "MKD" => Ok(Self::Mkd),
            "MMK" => Ok(Self::Mmk),
            "MNT" => Ok(Self::Mnt),
            "MOP" => Ok(Self::Mop),
            "MRO" => Ok(Self::Mro),
            "MUR" => Ok(Self::Mur),
            "MVR" => Ok(Self::Mvr),
            "MWK" => Ok(Self::Mwk),
            "MXN" => Ok(Self::Mxn),
            "MXV" => Ok(Self::Mxv),
            "MYR" => Ok(Self::Myr),
            "MZN" => Ok(Self::Mzn),
            "NAD" => Ok(Self::Nad),
            "NGN" => Ok(Self::Ngn),
            "NIO" => Ok(Self::Nio),
            "NOK" => Ok(Self::Nok),
            "NPR" => Ok(Self::Npr),
            "NZD" => Ok(Self::Nzd),
            "OMR" => Ok(Self::Omr),
            "PAB" => Ok(Self::Pab),
            "PEN" => Ok(Self::Pen),
            "PGK" => Ok(Self::Pgk),
            "PHP" => Ok(Self::Php),
            "PKR" => Ok(Self::Pkr),
            "PLN" => Ok(Self::Pln),
            "PYG" => Ok(Self::Pyg),
            "QAR" => Ok(Self::Qar),
            "RON" => Ok(Self::Ron),
            "RSD" => Ok(Self::Rsd),
            "RUB" => Ok(Self::Rub),
            "RWF" => Ok(Self::Rwf),
            "SAR" => Ok(Self::Sar),
            "SBD" => Ok(Self::Sbd),
            "SCR" => Ok(Self::Scr),
            "SDG" => Ok(Self::Sdg),
            "SEK" => Ok(Self::Sek),
            "SGD" => Ok(Self::Sgd),
            "SHP" => Ok(Self::Shp),
            "SLL" => Ok(Self::Sll),
            "SOS" => Ok(Self::Sos),
            "SRD" => Ok(Self::Srd),
            "STD" => Ok(Self::Std),
            "SVC" => Ok(Self::Svc),
            "SYP" => Ok(Self::Syp),
            "SZL" => Ok(Self::Szl),
            "THB" => Ok(Self::Thb),
            "TJS" => Ok(Self::Tjs),
            "TMT" => Ok(Self::Tmt),
            "TND" => Ok(Self::Tnd),
            "TOP" => Ok(Self::Top),
            "TRY" => Ok(Self::Try),
            "TTD" => Ok(Self::Ttd),
            "TWD" => Ok(Self::Twd),
            "TZS" => Ok(Self::Tzs),
            "UAH" => Ok(Self::Uah),
            "UGX" => Ok(Self::Ugx),
            "USD" => Ok(Self::Usd),
            "USN" => Ok(Self::Usn),
            "USS" => Ok(Self::Uss),
            "UYI" => Ok(Self::Uyi),
            "UYU" => Ok(Self::Uyu),
            "UZS" => Ok(Self::Uzs),
            "VEF" => Ok(Self::Vef),
            "VND" => Ok(Self::Vnd),
            "VUV" => Ok(Self::Vuv),
            "WST" => Ok(Self::Wst),
            "XAF" => Ok(Self::Xaf),
            "XAG" => Ok(Self::Xag),
            "XAU" => Ok(Self::Xau),
            "XBA" => Ok(Self::Xba),
            "XBB" => Ok(Self::Xbb),
            "XBC" => Ok(Self::Xbc),
            "XBD" => Ok(Self::Xbd),
            "XCD" => Ok(Self::Xcd),
            "XDR" => Ok(Self::Xdr),
            "XFU" => Ok(Self::Xfu),
            "XOF" => Ok(Self::Xof),
            "XPD" => Ok(Self::Xpd),
            "XPF" => Ok(Self::Xpf),
            "XPT" => Ok(Self::Xpt),
            "XTS" => Ok(Self::Xts),
            "XXX" => Ok(Self::Xxx),
            "YER" => Ok(Self::Yer),
            "ZAR" => Ok(Self::Zar),
            "ZMK" => Ok(Self::Zmk),
            "ZWL" => Ok(Self::Zwl),
            _ => Err(xsd_api::ReadError::UnknownEnumVariant),
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
