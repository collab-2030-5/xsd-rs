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

impl SiScaleCodeType {
    pub(crate) fn from_str(s: &str) -> Result<Self, crate::ReadError> {
        match s {
            "p" => Ok(Self::P),
            "n" => Ok(Self::N),
            "micro" => Ok(Self::Micro),
            "m" => Ok(Self::M),
            "c" => Ok(Self::C),
            "d" => Ok(Self::D),
            "k" => Ok(Self::K),
            "M" => Ok(Self::M2),
            "G" => Ok(Self::G),
            "T" => Ok(Self::T),
            "none" => Ok(Self::None),
            _ => Err(crate::ReadError::UnknownEnumVariant),
        }
    }

    pub(crate) fn to_str(self) -> &'static str {
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
