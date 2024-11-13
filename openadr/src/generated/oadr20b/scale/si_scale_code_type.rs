#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
/// Scale based on representations of SI scale as expressed in the unit multipliers
pub enum SiScaleCodeType {
    /// Pico 10**-12 (xml value == 'p')
    P,
    /// Nano 10**-9 (xml value == 'n')
    N,
    /// Micro 10**-6 (xml value == 'micro')
    Micro,
    /// Milli 10**-3 (xml value == 'm')
    M,
    /// Centi 10**-2 (xml value == 'c')
    C,
    /// Deci 10**-1 (xml value == 'd')
    D,
    /// Kilo 10**3 (xml value == 'k')
    K,
    /// Mega 10**6 (xml value == 'M')
    M2,
    /// Giga 10**9 (xml value == 'G')
    G,
    /// Tera 10**12 (xml value == 'T')
    T,
    /// Native Scale (xml value == 'none')
    None,
}

impl xsd_util::StringEnumeration for SiScaleCodeType {
    fn find(s: &str) -> Option<Self> {
        match s {
            "p" => Some(Self::P),
            "n" => Some(Self::N),
            "micro" => Some(Self::Micro),
            "m" => Some(Self::M),
            "c" => Some(Self::C),
            "d" => Some(Self::D),
            "k" => Some(Self::K),
            "M" => Some(Self::M2),
            "G" => Some(Self::G),
            "T" => Some(Self::T),
            "none" => Some(Self::None),
            _ => None,
        }
    }

    fn to_str(self) -> &'static str {
        match self {
            Self::P => "p",
            Self::N => "n",
            Self::Micro => "micro",
            Self::M => "m",
            Self::C => "c",
            Self::D => "d",
            Self::K => "k",
            Self::M2 => "M",
            Self::G => "G",
            Self::T => "T",
            Self::None => "none",
        }
    }
}
