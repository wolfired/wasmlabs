use std::fmt::Display;

#[derive(Copy, Debug, Clone)]
pub enum Index {
    Typeidx(u32),
    Funcidx(u32),
    Tableidx(u32),
    Memidx(u32),
    Globalidx(u32),
    Elemidx(u32),
    Dataidx(u32),
    Localidx(u32),
    Labelidx(u32),
}

impl Display for Index {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Typeidx(idx) => write!(f, "typeidx({idx})"),
            Self::Funcidx(idx) => write!(f, "funcidx({idx})"),
            Self::Tableidx(idx) => write!(f, "tableidx({idx})"),
            Self::Memidx(idx) => write!(f, "memidx({idx})"),
            Self::Globalidx(idx) => write!(f, "globalidx({idx})"),
            Self::Elemidx(idx) => write!(f, "elemidx({idx})"),
            Self::Dataidx(idx) => write!(f, "dataidx({idx})"),
            Self::Localidx(idx) => write!(f, "localidx({idx})"),
            Self::Labelidx(idx) => write!(f, "labelidx({idx})"),
        }
    }
}
