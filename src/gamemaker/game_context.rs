use libgm::{
    gamemaker::{
        elements::general_info::GMGeneralInfo,
        version::{GMVersion, LTSBranch},
    },
    gml::{Instruction, instruction::DataType},
    prelude::*,
};

use crate::primitives::{RawArray, RustStr};

#[repr(u8)]
enum RawBranch {
    Pre2022 = 1,
    LTS2022 = 2,
    Post2022 = 3,
}

#[repr(C)]
pub struct GameContext<'a> {
    ver_major: u32,
    ver_minor: u32,
    ver_release: u32,
    ver_build: u32,
    wad_version: u8,
    lts_branch: RawBranch,

    short_curcuit: bool,
    array_cow: bool,

    asset_object_names: RawArray<RustStr<'a>>,
    asset_sprite_names: RawArray<RustStr<'a>>,
    asset_sound_names: RawArray<RustStr<'a>>,
    asset_room_names: RawArray<RustStr<'a>>,
    asset_background_names: RawArray<RustStr<'a>>,
    asset_path_names: RawArray<RustStr<'a>>,
    asset_script_names: RawArray<RustStr<'a>>,
    asset_font_names: RawArray<RustStr<'a>>,
    asset_timeline_names: RawArray<RustStr<'a>>,
    asset_shader_names: RawArray<RustStr<'a>>,
    asset_sequence_names: RawArray<RustStr<'a>>,
    asset_animcurve_names: RawArray<RustStr<'a>>,
    asset_particlesystem_names: RawArray<RustStr<'a>>,
}

impl<'a> GameContext<'a> {
    pub(crate) fn try_from_libgm(data: &'a GMData) -> Result<Self> {
        let gen8: &GMGeneralInfo = &data.general_info;
        let ver: &GMVersion = &gen8.version;

        Ok(Self {
            ver_major: ver.major,
            ver_minor: ver.major,
            ver_release: ver.release,
            ver_build: ver.build,
            wad_version: gen8.wad_version,
            lts_branch: convert_lts_branch(ver.branch),
            short_curcuit: find_short_curcuit(data),
            array_cow: find_array_cow(data),
            asset_object_names: get_asset_names(&data.game_objects),
            asset_sprite_names: get_asset_names(&data.sprites),
            asset_sound_names: get_asset_names(&data.sounds),
            asset_room_names: get_asset_names(&data.rooms),
            asset_background_names: get_asset_names(&data.backgrounds),
            asset_path_names: get_asset_names(&data.paths),
            asset_script_names: get_asset_names(&data.scripts),
            asset_font_names: get_asset_names(&data.fonts),
            asset_timeline_names: get_asset_names(&data.timelines),
            asset_shader_names: get_asset_names(&data.shaders),
            asset_sequence_names: get_asset_names(&data.sequences),
            asset_animcurve_names: get_asset_names(&data.animation_curves),
            asset_particlesystem_names: get_asset_names(&data.particle_systems),
        })
    }
}

const fn convert_lts_branch(libgm_branch_type: LTSBranch) -> RawBranch {
    match libgm_branch_type {
        LTSBranch::PreLTS => RawBranch::Pre2022,
        LTSBranch::LTS => RawBranch::LTS2022,
        LTSBranch::PostLTS => RawBranch::Post2022,
    }
}

fn find_short_curcuit(data: &GMData) -> bool {
    for code in &data.codes {
        for instr in &code.instructions {
            // instructions like and.b.b / or.b.b imply the game was implied without short curcuiting.
            if matches!(
                instr,
                Instruction::And {
                    lhs: DataType::Boolean,
                    rhs: DataType::Boolean,
                } | Instruction::Or {
                    lhs: DataType::Boolean,
                    rhs: DataType::Boolean,
                }
            ) {
                return false;
            }
        }
    }
    true
}

fn find_array_cow(data: &GMData) -> bool {
    for code in &data.codes {
        for instr in &code.instructions {
            if *instr == Instruction::SetArrayOwner {
                // If a setowner instruction is found, the game must use array copy on write
                return true;
            }
        }
    }
    false
}

fn get_asset_names(chunk: &impl GMNamedListChunk) -> RawArray<RustStr<'_>> {
    let mut vector = Vec::with_capacity(chunk.len());
    for element in chunk.elements() {
        let name: &str = element.name();
        vector.push(RustStr::from_str(name));
    }
    RawArray::from_vec(vector)
}
