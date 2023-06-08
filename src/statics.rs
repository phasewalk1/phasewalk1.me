type StaticStr = &'static str;

pub(super) mod refs {
    use super::StaticStr;
    pub(crate) static MIRROR: StaticStr =
        "https://mirror.xyz/0x497437B2aC4B2D408138bE772233E3F2dF8b7064";
    pub(crate) static GITHUB: StaticStr = "https://github.com/phasewalk1";
    pub(crate) static PERSY: StaticStr = "https://phasewalk1.github.io";
    pub(crate) static TWITTER: StaticStr = "https://twitter.com/phasewalk1";
    pub(crate) static MAILTO: StaticStr = "mailto:kat@phasewalk1.net";
}

pub(super) mod guts {
    use super::StaticStr;
    pub(crate) static HEADER_TEXT: StaticStr = "phasewalk";
    pub(crate) static SUBHEADER_TEXT: StaticStr = "Pure mathematics, Dark software.";
}
