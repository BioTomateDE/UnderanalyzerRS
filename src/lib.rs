// #![warn(clippy::cargo)]
// #![warn(clippy::pedantic)]
// #![warn(clippy::nursery)]

mod dynlib;
mod gamemaker;
mod primitives;

use libgm::{
    error::Context,
    gamemaker::{data::GMData, reference::GMRef},
    gml::GMCode,
};

use crate::{dynlib::decompile_to_string, gamemaker::Code};

pub use crate::gamemaker::GameContext;

/// Tries to initialize to dynamic library cache.
/// Otherwise, it will be initialized on the first [`GameContext::decompile`] call.
///
/// Calling this function is not needed, but has two benefits:
/// * You can explicitly choose *when* to initialize the dynamic library,
///   since it may take a few hundred milliseconds
/// * You can handle errors. Although rare, errors loading the dynamic library can occur
///   and would result in a panic if not caught here.
///
/// This function does nothing if the dynamic library was already initialized.
///
/// # Errors
/// This function may fail in these ways:
/// * creating temporary file
/// * writing library data to temporary file
/// * loading dynamic library
/// * loading symbols from library
pub fn init_dynlib() -> libgm::Result<()> {
    dynlib::init_externs()
}

impl<'a> GameContext<'a> {
    /// Tries to create a new [`GameContext`] from a [`GMData`].
    ///
    /// This operation may take quite a while (100ms?).
    /// You should definitely reuse this struct when decompiling multiple code entries.
    ///
    /// Note that major changes to the underlying [`GMData`] may invalidate this [`GameContext`].
    /// In that case, you will need to call this function again to construct a new game context struct.
    /// It is currently not known (stablilized) exactly which parts of a GameMaker data file
    /// require reconstruction of the game context on modification.
    ///
    /// # Errors
    /// This function may fail if the GameMaker data is malformed.
    /// This mostly includes [`GMRef`]s out of bounds.
    ///
    /// [`GMRef`]: libgm::gamemaker::reference::GMRef
    pub fn new(gm_data: &'a GMData) -> libgm::Result<Self> {
        Self::try_from_libgm(gm_data).with_context(|| {
            format!(
                "constructing GameContext for {}",
                gm_data.general_info.game_name,
            )
        })
    }

    /// Tries to decompile the given code entry by calling `DecompileToString` in Underanalyzer.
    ///
    /// # Errors
    /// This function will return an error if:
    /// * any error occurred in Underanalyzer (signalled by the returned `error` byte being non-zero)
    /// * the returned string is null
    /// * the returned string contains invalid UTF-8
    ///
    /// The most likely error cause will definitely be a decompilation error in Underanalyzer, though.
    pub fn decompile(&self, code_ref: GMRef<GMCode>, gm_data: &GMData) -> libgm::Result<String> {
        let code = Code::try_from_libgm(code_ref, gm_data).with_context(|| {
            format!(
                "converting LibGM code entry #{} into FFI struct",
                u32::from(code_ref),
            )
        })?;

        let code = &raw const code;
        let ctx = self as *const Self;

        let ret = unsafe { decompile_to_string(ctx, code) };

        let string: &str = unsafe { ret.string.to_str() }
            .context("constructing string from return value of Underanalyzer DecompileToString")?;

        if ret.error == 0 {
            return Ok(string.to_owned());
        }

        // Error occurred
        let errno: u8 = ret.error;
        let message = if errno == 1 {
            string.to_owned()
        } else {
            // print extra stuff for unknown error codes
            format!("Decompilation failed with error code {errno}: {string}")
        };

        Err(libgm::Error::new(message).push_context("decompiling code entry using Underanalyzer"))
    }
}
