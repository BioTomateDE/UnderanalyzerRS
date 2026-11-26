use std::any::type_name;

use crate::gamemaker::version::LtsBranch;
use crate::gamemaker::version::UaVersion;
use crate::gamemaker::version::convert_lts_branch;
use crate::gamemaker::version::convert_version;
use crate::primitives::RawArray;
use crate::primitives::RustStr;
use libgm::gml::analysis::CodeAnalysis;
use libgm::gml::analysis::analyze;
use libgm::prelude::*;
use libgm::wad::elem::string::Strings;

#[repr(C)]
pub struct GameContext<'a> {
    // TODO: this *could* just become the enum.
    version: UaVersion,
    wad_version: u8,
    lts_branch: LtsBranch,

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
        let version: UaVersion = convert_version(data.meta.version, data.general_info.ide_version)
            .ctx("converting GMVersion enum to classic 4-part struct")?;
        let lts_branch: LtsBranch = convert_lts_branch(data.meta.version);
        let analysis: CodeAnalysis = analyze(data);
        let s = &data.strings;

        Ok(Self {
            version,
            wad_version: data.general_info.wad_version,
            lts_branch,
            short_curcuit: analysis.short_circuit,
            array_cow: analysis.array_cow,
            asset_object_names: get_asset_names(&data.game_objects, s)?,
            asset_sprite_names: get_asset_names(&data.sprites, s)?,
            asset_sound_names: get_asset_names(&data.sounds, s)?,
            asset_room_names: get_asset_names(&data.rooms, s)?,
            asset_background_names: get_asset_names(&data.tilesets, s)?,
            asset_path_names: get_asset_names(&data.paths, s)?,
            asset_script_names: get_asset_names(&data.scripts, s)?,
            asset_font_names: get_asset_names(&data.fonts, s)?,
            asset_timeline_names: get_asset_names(&data.timelines, s)?,
            asset_shader_names: get_asset_names(&data.shaders, s)?,
            asset_sequence_names: get_asset_names(&data.sequences, s)?,
            asset_animcurve_names: get_asset_names(&data.animation_curves, s)?,
            asset_particlesystem_names: get_asset_names(&data.particle_systems, s)?,
        })
    }
}

fn get_asset_names<'a, C: GMAssetListChunk>(chunk: &C, strings: &'a Strings) -> Result<RawArray<RustStr<'a>>> {
    let mut slice: Box<[RustStr]> = std::iter::repeat_with(|| RustStr::EMPTY).take(chunk.len()).collect();
    for (gmref, element) in chunk.element_refs() {
        let idx: usize = gmref.index().unwrap();
        let name: &str = element
            .name(strings)
            .ctx(|| format!("getting name of {} (chunk {})", type_name::<C::Element>(), C::NAME))?;
        slice[idx] = RustStr::from_str(name);
    }
    Ok(RawArray::from_boxed_slice(slice))
}
