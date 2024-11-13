//! Reads and applies font patches to font binaries.
//!
//! Font patch formats are defined as part of the incremental font transfer specification:
//! <https://w3c.github.io/IFT/Overview.html#font-patch-formats>
//!
//! Two main types of font patches are implemented:
//! 1. Table Keyed Patch - these patches contain a per table brotli binary patch to be applied
//!    to the input font.
//! 2. Glyph Keyed - these patches contain blobs of data associated with combinations of
//!    glyph id + table. The patch inserts these blobs into the table at the location for
//!    the corresponding glyph id.

use std::collections::HashMap;

use crate::patch_group::PatchInfo;

use crate::glyph_keyed::apply_glyph_keyed_patches;

use crate::table_keyed::apply_table_keyed_patch;
use font_types::Tag;
use read_fonts::tables::ift::{CompatibilityId, GlyphKeyedPatch, TableKeyedPatch};

use read_fonts::{FontData, FontRead, FontRef, ReadError};

use shared_brotli_patch_decoder::DecodeError;

/// A trait for types to which an incremental font transfer patch can be applied.
///
/// See: <https://w3c.github.io/IFT/Overview.html#font-patch-formats> for details on the format of patches.
pub(crate) trait IncrementalFontPatchBase {
    /// Apply a table keyed incremental font patches (<https://w3c.github.io/IFT/Overview.html#font-patch-formats>)
    ///
    /// Applies the patches to this base.
    ///
    /// Returns the byte data for the new font produced as a result of the patch applications.
    fn apply_table_keyed_patch(
        &self,
        patch: &PatchInfo,
        patch_data: &[u8],
    ) -> Result<Vec<u8>, PatchingError>;

    /// Apply a set of glyph keyed incremental font patches (<https://w3c.github.io/IFT/Overview.html#font-patch-formats>)
    ///
    /// Applies the patches to this base.
    ///
    /// Returns the byte data for the new font produced as a result of the patch applications.
    fn apply_glyph_keyed_patches<'a>(
        &self,
        patches: impl Iterator<Item = (&'a PatchInfo, &'a [u8])>,
    ) -> Result<Vec<u8>, PatchingError>;
}

/// An error that occurs while trying to apply an IFT patch to a font file.
#[derive(Debug, Clone, PartialEq)]
pub enum PatchingError {
    PatchParsingFailed(ReadError),
    FontParsingFailed(ReadError),
    IncompatiblePatch,
    NonIncrementalFont,
    InvalidPatch(&'static str),
    EmptyPatchList,
    InternalError,
    MissingPatches,
}

impl From<DecodeError> for PatchingError {
    fn from(decoding_error: DecodeError) -> Self {
        match decoding_error {
            DecodeError::InitFailure => {
                PatchingError::InvalidPatch("Failure to init brotli encoder.")
            }
            DecodeError::InvalidStream => PatchingError::InvalidPatch("Malformed brotli stream."),
            DecodeError::InvalidDictionary => PatchingError::InvalidPatch("Malformed dictionary."),
            DecodeError::MaxSizeExceeded => PatchingError::InvalidPatch("Max size exceeded."),
            DecodeError::ExcessInputData => {
                PatchingError::InvalidPatch("Input brotli stream has excess bytes.")
            }
        }
    }
}

impl std::fmt::Display for PatchingError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            PatchingError::PatchParsingFailed(err) => {
                write!(f, "Failed to parse patch file: {}", err)
            }
            PatchingError::FontParsingFailed(err) => {
                write!(f, "Failed to parse font file: {}", err)
            }
            PatchingError::IncompatiblePatch => {
                write!(f, "Compatibility ID of the patch does not match the font.")
            }

            PatchingError::NonIncrementalFont => {
                write!(
                    f,
                    "Can't patch font as it's not an incremental transfer font."
                )
            }
            PatchingError::InvalidPatch(msg) => write!(f, "Invalid patch file: '{msg}'"),
            PatchingError::EmptyPatchList => write!(f, "At least one patch file must be provided."),
            PatchingError::InternalError => write!(
                f,
                "Internal constraint violated, typically should not happen."
            ),
            PatchingError::MissingPatches => write!(f, "Not all patch data has been supplied."),
        }
    }
}

impl std::error::Error for PatchingError {}

impl IncrementalFontPatchBase for FontRef<'_> {
    fn apply_table_keyed_patch(
        &self,
        patch: &PatchInfo,
        patch_data: &[u8],
    ) -> Result<Vec<u8>, PatchingError> {
        let font_compat_id = patch
            .tag()
            .font_compat_id(self)
            .map_err(PatchingError::FontParsingFailed)?;

        let patch = TableKeyedPatch::read(FontData::new(patch_data))
            .map_err(PatchingError::PatchParsingFailed)?;

        if patch.compatibility_id() != font_compat_id {
            return Err(PatchingError::IncompatiblePatch);
        }

        apply_table_keyed_patch(&patch, self)
    }

    fn apply_glyph_keyed_patches<'a>(
        &self,
        patches: impl Iterator<Item = (&'a PatchInfo, &'a [u8])>,
    ) -> Result<Vec<u8>, PatchingError> {
        let mut cached_compat_ids: HashMap<Tag, Result<CompatibilityId, PatchingError>> =
            Default::default();

        let mut raw_patches: Vec<GlyphKeyedPatch<'_>> = vec![];
        for (patch, patch_data) in patches {
            let tag = patch.tag();
            let font_compat_id = cached_compat_ids
                .entry(tag.tag())
                .or_insert_with(|| {
                    tag.font_compat_id(self)
                        .map_err(PatchingError::FontParsingFailed)
                })
                .as_ref()
                .map_err(Clone::clone)?;

            let patch = GlyphKeyedPatch::read(FontData::new(patch_data))
                .map_err(PatchingError::PatchParsingFailed)?;

            if *font_compat_id != patch.compatibility_id() {
                return Err(PatchingError::IncompatiblePatch);
            }

            raw_patches.push(patch);
        }

        apply_glyph_keyed_patches(&raw_patches, self)
    }
}

impl IncrementalFontPatchBase for &[u8] {
    fn apply_table_keyed_patch(
        &self,
        patch: &PatchInfo,
        patch_data: &[u8],
    ) -> Result<Vec<u8>, PatchingError> {
        FontRef::new(self)
            .map_err(PatchingError::FontParsingFailed)?
            .apply_table_keyed_patch(patch, patch_data)
    }

    fn apply_glyph_keyed_patches<'a>(
        &self,
        patches: impl Iterator<Item = (&'a PatchInfo, &'a [u8])>,
    ) -> Result<Vec<u8>, PatchingError> {
        FontRef::new(self)
            .map_err(PatchingError::FontParsingFailed)?
            .apply_glyph_keyed_patches(patches)
    }
}

#[cfg(test)]
mod tests {
    // TODO(garretrieger): apply table keyed with mismatched compat ids.
    // TODO(garretrieger): apply glyph keyed with mismatched compat ids.

    /*
    #[test]
      fn table_keyed_patch_compat_id_mismatch() {
          let uri = PatchUri::from_index(
              "",
              0,
              &CompatibilityId::from_u32s([1, 2, 2, 4]),
              PatchEncoding::TableKeyed {
                  fully_invalidating: false,
              },
          );
          let patch_data = table_keyed_patch();
          assert_eq!(
              PatchingError::IncompatiblePatch,
              uri.into_patch(patch_data.as_slice()).err().unwrap()
          );
      }
     */
}
