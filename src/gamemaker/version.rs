use std::cmp::Ordering;

use libgm::prelude::*;
use libgm::wad::GMVersion;
use libgm::wad::version::IdeVersion;

macro_rules! ver {
    ($major:literal, $minor:literal) => {
        UaVersion {
            major: $major,
            minor: $minor,
            release: 0,
            build: 0,
        }
    };
    ($major:literal, $minor:literal, $release:literal) => {
        UaVersion {
            major: $major,
            minor: $minor,
            release: $release,
            build: 0,
        }
    };
    ($major:literal, $minor:literal, $release:literal, $build:literal) => {
        UaVersion {
            major: $major,
            minor: $minor,
            release: $release,
            build: $build,
        }
    };
}

#[repr(C)]
pub struct UaVersion {
    major: u32,
    minor: u32,
    release: u32,
    build: u32,
}

#[repr(u8)]
pub enum LtsBranch {
    PreLts = 1,
    Lts = 2,
    PostLts = 3,
}

pub fn convert_lts_branch(version: GMVersion) -> LtsBranch {
    // i think this is right??
    match version.cmp(&GMVersion::Lts2022_0_3) {
        Ordering::Less => LtsBranch::PreLts,
        Ordering::Equal => LtsBranch::Lts,
        Ordering::Greater => LtsBranch::PostLts,
    }
}

pub fn convert_version(version: GMVersion, ide: IdeVersion) -> Result<UaVersion> {
    if ide != IdeVersion::GMS2 {
        return Ok(UaVersion {
            major: ide.major,
            minor: ide.minor,
            release: ide.release,
            build: ide.build,
        });
    }

    Ok(match version {
        GMVersion::Wad12
        | GMVersion::Wad13
        | GMVersion::Wad14
        | GMVersion::Wad14Beta
        | GMVersion::Wad15
        | GMVersion::Wad16
        | GMVersion::Wad16Pad => bail!("Old GMVersion {version} but GEN8 is GMS2"),
        GMVersion::GMS2 => ver!(2, 0),
        GMVersion::GMS2_0_6 => ver!(2, 0, 6),
        GMVersion::GMS2_2_1 => ver!(2, 2, 1),
        GMVersion::GMS2_2_2_302 => ver!(2, 2, 2, 302),
        GMVersion::GMS2_3 => ver!(2, 3),
        GMVersion::GMS2_3_1 => ver!(2, 3, 1),
        GMVersion::GMS2_3_2 => ver!(2, 3, 2),
        GMVersion::GMS2_3_6 => ver!(2, 3, 6),
        GMVersion::GM2022_1 => ver!(2022, 1),
        GMVersion::GM2022_2 => ver!(2022, 0),
        GMVersion::GM2022_3 => ver!(2022, 3),
        GMVersion::GM2022_5 => ver!(2022, 5),
        GMVersion::GM2022_6 => ver!(2022, 6),
        GMVersion::GM2022_8 => ver!(2022, 8),
        GMVersion::GM2022_9 => ver!(2022, 9),
        GMVersion::GM2023_1 => ver!(2023, 1),
        GMVersion::GM2023_2 => ver!(2023, 2),
        GMVersion::GM2023_4 => ver!(2023, 4),
        GMVersion::Lts2022_0_3 | GMVersion::GM2023_6 => ver!(2023, 6),
        GMVersion::GM2023_8 => ver!(2023, 8),
        GMVersion::GM2023_11 => ver!(2023, 11),
        GMVersion::GM2024_2 => ver!(2024, 2),
        GMVersion::GM2024_4 => ver!(2024, 4),
        GMVersion::GM2024_6 => ver!(2024, 6),
        GMVersion::GM2024_8 => ver!(2024, 8),
        GMVersion::GM2024_11 => ver!(2024, 11),
        GMVersion::GM2024_13 => ver!(2024, 13),
        GMVersion::GM2024_14 => ver!(2024, 14),
        GMVersion::GM2024_14_1 => ver!(2024, 14, 1),
        GMVersion::GM2026_1 => ver!(2026, 1),
        _ => bail!("Unknown version {version}; UnderanalyzerRS needs to be fixed"),
    })
}
