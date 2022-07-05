// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

#[allow(unused_imports)]
use font_types::*;

/// [STAT](https://docs.microsoft.com/en-us/typography/opentype/spec/stat) (Style Attributes Table)
pub struct Stat1_0<'a> {
    major_version: zerocopy::LayoutVerified<&'a [u8], BigEndian<u16>>,
    minor_version: zerocopy::LayoutVerified<&'a [u8], BigEndian<u16>>,
    design_axis_size: zerocopy::LayoutVerified<&'a [u8], BigEndian<u16>>,
    design_axis_count: zerocopy::LayoutVerified<&'a [u8], BigEndian<u16>>,
    design_axes_offset: zerocopy::LayoutVerified<&'a [u8], BigEndian<Offset32>>,
    axis_value_count: zerocopy::LayoutVerified<&'a [u8], BigEndian<u16>>,
    offset_to_axis_value_offsets: zerocopy::LayoutVerified<&'a [u8], BigEndian<Offset32>>,
    offset_bytes: &'a [u8],
}

impl<'a> font_types::FontRead<'a> for Stat1_0<'a> {
    fn read(bytes: &'a [u8]) -> Option<Self> {
        let offset_bytes = bytes;
        let (major_version, bytes) =
            zerocopy::LayoutVerified::<_, BigEndian<u16>>::new_unaligned_from_prefix(bytes)?;
        let (minor_version, bytes) =
            zerocopy::LayoutVerified::<_, BigEndian<u16>>::new_unaligned_from_prefix(bytes)?;
        let (design_axis_size, bytes) =
            zerocopy::LayoutVerified::<_, BigEndian<u16>>::new_unaligned_from_prefix(bytes)?;
        let (design_axis_count, bytes) =
            zerocopy::LayoutVerified::<_, BigEndian<u16>>::new_unaligned_from_prefix(bytes)?;
        let (design_axes_offset, bytes) =
            zerocopy::LayoutVerified::<_, BigEndian<Offset32>>::new_unaligned_from_prefix(bytes)?;
        let (axis_value_count, bytes) =
            zerocopy::LayoutVerified::<_, BigEndian<u16>>::new_unaligned_from_prefix(bytes)?;
        let (offset_to_axis_value_offsets, bytes) =
            zerocopy::LayoutVerified::<_, BigEndian<Offset32>>::new_unaligned_from_prefix(bytes)?;
        let _bytes = bytes;
        let result = Stat1_0 {
            major_version,
            minor_version,
            design_axis_size,
            design_axis_count,
            design_axes_offset,
            axis_value_count,
            offset_to_axis_value_offsets,
            offset_bytes,
        };
        Some(result)
    }
}

impl<'a> Stat1_0<'a> {
    /// Major version number of the style attributes table — set to 1.
    pub fn major_version(&self) -> u16 {
        self.major_version.get()
    }

    /// Minor version number of the style attributes table — set to 2.
    pub fn minor_version(&self) -> u16 {
        self.minor_version.get()
    }

    /// The size in bytes of each axis record.
    pub fn design_axis_size(&self) -> u16 {
        self.design_axis_size.get()
    }

    /// The number of axis records. In a font with an 'fvar' table,
    /// this value must be greater than or equal to the axisCount value
    /// in the 'fvar' table. In all fonts, must be greater than zero if
    /// axisValueCount is greater than zero.
    pub fn design_axis_count(&self) -> u16 {
        self.design_axis_count.get()
    }

    /// Offset in bytes from the beginning of the STAT table to the
    /// start of the design axes array. If designAxisCount is zero, set
    /// to zero; if designAxisCount is greater than zero, must be
    /// greater than zero.
    pub fn design_axes_offset(&self) -> Offset32 {
        self.design_axes_offset.get()
    }

    /// The number of axis value tables.
    pub fn axis_value_count(&self) -> u16 {
        self.axis_value_count.get()
    }

    /// Offset in bytes from the beginning of the STAT table to the
    /// start of the design axes value offsets array. If axisValueCount
    /// is zero, set to zero; if axisValueCount is greater than zero,
    /// must be greater than zero.
    pub fn offset_to_axis_value_offsets(&self) -> Offset32 {
        self.offset_to_axis_value_offsets.get()
    }
}

impl<'a> font_types::OffsetHost<'a> for Stat1_0<'a> {
    fn bytes(&self) -> &'a [u8] {
        self.offset_bytes
    }
}

/// [STAT](https://docs.microsoft.com/en-us/typography/opentype/spec/stat) (Style Attributes Table)
pub struct Stat1_2<'a> {
    major_version: zerocopy::LayoutVerified<&'a [u8], BigEndian<u16>>,
    minor_version: zerocopy::LayoutVerified<&'a [u8], BigEndian<u16>>,
    design_axis_size: zerocopy::LayoutVerified<&'a [u8], BigEndian<u16>>,
    design_axis_count: zerocopy::LayoutVerified<&'a [u8], BigEndian<u16>>,
    design_axes_offset: zerocopy::LayoutVerified<&'a [u8], BigEndian<Offset32>>,
    axis_value_count: zerocopy::LayoutVerified<&'a [u8], BigEndian<u16>>,
    offset_to_axis_value_offsets: zerocopy::LayoutVerified<&'a [u8], BigEndian<Offset32>>,
    elided_fallback_name_id: zerocopy::LayoutVerified<&'a [u8], BigEndian<u16>>,
    offset_bytes: &'a [u8],
}

impl<'a> font_types::FontRead<'a> for Stat1_2<'a> {
    fn read(bytes: &'a [u8]) -> Option<Self> {
        let offset_bytes = bytes;
        let (major_version, bytes) =
            zerocopy::LayoutVerified::<_, BigEndian<u16>>::new_unaligned_from_prefix(bytes)?;
        let (minor_version, bytes) =
            zerocopy::LayoutVerified::<_, BigEndian<u16>>::new_unaligned_from_prefix(bytes)?;
        let (design_axis_size, bytes) =
            zerocopy::LayoutVerified::<_, BigEndian<u16>>::new_unaligned_from_prefix(bytes)?;
        let (design_axis_count, bytes) =
            zerocopy::LayoutVerified::<_, BigEndian<u16>>::new_unaligned_from_prefix(bytes)?;
        let (design_axes_offset, bytes) =
            zerocopy::LayoutVerified::<_, BigEndian<Offset32>>::new_unaligned_from_prefix(bytes)?;
        let (axis_value_count, bytes) =
            zerocopy::LayoutVerified::<_, BigEndian<u16>>::new_unaligned_from_prefix(bytes)?;
        let (offset_to_axis_value_offsets, bytes) =
            zerocopy::LayoutVerified::<_, BigEndian<Offset32>>::new_unaligned_from_prefix(bytes)?;
        let (elided_fallback_name_id, bytes) =
            zerocopy::LayoutVerified::<_, BigEndian<u16>>::new_unaligned_from_prefix(bytes)?;
        let _bytes = bytes;
        let result = Stat1_2 {
            major_version,
            minor_version,
            design_axis_size,
            design_axis_count,
            design_axes_offset,
            axis_value_count,
            offset_to_axis_value_offsets,
            elided_fallback_name_id,
            offset_bytes,
        };
        Some(result)
    }
}

impl<'a> Stat1_2<'a> {
    /// Major version number of the style attributes table — set to 1.
    pub fn major_version(&self) -> u16 {
        self.major_version.get()
    }

    /// Minor version number of the style attributes table — set to 2.
    pub fn minor_version(&self) -> u16 {
        self.minor_version.get()
    }

    /// The size in bytes of each axis record.
    pub fn design_axis_size(&self) -> u16 {
        self.design_axis_size.get()
    }

    /// The number of axis records. In a font with an 'fvar' table,
    /// this value must be greater than or equal to the axisCount value
    /// in the 'fvar' table. In all fonts, must be greater than zero if
    /// axisValueCount is greater than zero.
    pub fn design_axis_count(&self) -> u16 {
        self.design_axis_count.get()
    }

    /// Offset in bytes from the beginning of the STAT table to the
    /// start of the design axes array. If designAxisCount is zero, set
    /// to zero; if designAxisCount is greater than zero, must be
    /// greater than zero.
    pub fn design_axes_offset(&self) -> Offset32 {
        self.design_axes_offset.get()
    }

    /// The number of axis value tables.
    pub fn axis_value_count(&self) -> u16 {
        self.axis_value_count.get()
    }

    /// Offset in bytes from the beginning of the STAT table to the
    /// start of the design axes value offsets array. If axisValueCount
    /// is zero, set to zero; if axisValueCount is greater than zero,
    /// must be greater than zero.
    pub fn offset_to_axis_value_offsets(&self) -> Offset32 {
        self.offset_to_axis_value_offsets.get()
    }

    /// Name ID used as fallback when projection of names into a
    /// particular font model produces a subfamily name containing only
    /// elidable elements.
    pub fn elided_fallback_name_id(&self) -> u16 {
        self.elided_fallback_name_id.get()
    }
}

impl<'a> font_types::OffsetHost<'a> for Stat1_2<'a> {
    fn bytes(&self) -> &'a [u8] {
        self.offset_bytes
    }
}

/// [Axis Records](https://docs.microsoft.com/en-us/typography/opentype/spec/stat#axis-records)
#[derive(Clone, Copy, Debug, zerocopy :: FromBytes, zerocopy :: Unaligned)]
#[repr(C)]
pub struct AxisRecord {
    /// A tag identifying the axis of design variation.
    pub axis_tag: BigEndian<Tag>,
    /// The name ID for entries in the 'name' table that provide a
    /// display string for this axis.
    pub axis_name_id: BigEndian<u16>,
    /// A value that applications can use to determine primary sorting
    /// of face names, or for ordering of labels when composing family
    /// or face names.
    pub axis_ordering: BigEndian<u16>,
}

impl AxisRecord {
    /// A tag identifying the axis of design variation.
    pub fn axis_tag(&self) -> Tag {
        self.axis_tag.get()
    }

    /// The name ID for entries in the 'name' table that provide a
    /// display string for this axis.
    pub fn axis_name_id(&self) -> u16 {
        self.axis_name_id.get()
    }

    /// A value that applications can use to determine primary sorting
    /// of face names, or for ordering of labels when composing family
    /// or face names.
    pub fn axis_ordering(&self) -> u16 {
        self.axis_ordering.get()
    }
}

/// [STAT](https://docs.microsoft.com/en-us/typography/opentype/spec/stat) (Style Attributes Table)
pub enum Stat<'a> {
    Version1_0(Stat1_0<'a>),
    Version1_2(Stat1_2<'a>),
}

impl<'a> font_types::FontRead<'a> for Stat<'a> {
    fn read(bytes: &'a [u8]) -> Option<Self> {
        const _: MajorMinor = MajorMinor::VERSION_1_0;
        const _: MajorMinor = MajorMinor::VERSION_1_2;
        let version: BigEndian<MajorMinor> = font_types::FontRead::read(bytes)?;
        match version.get() {
            MajorMinor::VERSION_1_0 => Some(Self::Version1_0(font_types::FontRead::read(bytes)?)),
            MajorMinor::VERSION_1_2 => Some(Self::Version1_2(font_types::FontRead::read(bytes)?)),
            _other => {
                #[cfg(feature = "std")]
                {
                    eprintln!(
                        "unknown enum variant {:?} (table {})",
                        version,
                        stringify!(Stat)
                    );
                }
                None
            }
        }
    }
}

impl<'a> Stat<'a> {
    /// The number of axis value tables.
    pub fn axis_value_count(&self) -> u16 {
        match self {
            Self::Version1_0(_inner) => _inner.axis_value_count(),
            Self::Version1_2(_inner) => _inner.axis_value_count(),
        }
    }

    /// Offset in bytes from the beginning of the STAT table to the
    /// start of the design axes array. If designAxisCount is zero, set
    /// to zero; if designAxisCount is greater than zero, must be
    /// greater than zero.
    pub fn design_axes_offset(&self) -> Offset32 {
        match self {
            Self::Version1_0(_inner) => _inner.design_axes_offset(),
            Self::Version1_2(_inner) => _inner.design_axes_offset(),
        }
    }

    /// The number of axis records. In a font with an 'fvar' table,
    /// this value must be greater than or equal to the axisCount value
    /// in the 'fvar' table. In all fonts, must be greater than zero if
    /// axisValueCount is greater than zero.
    pub fn design_axis_count(&self) -> u16 {
        match self {
            Self::Version1_0(_inner) => _inner.design_axis_count(),
            Self::Version1_2(_inner) => _inner.design_axis_count(),
        }
    }

    /// The size in bytes of each axis record.
    pub fn design_axis_size(&self) -> u16 {
        match self {
            Self::Version1_0(_inner) => _inner.design_axis_size(),
            Self::Version1_2(_inner) => _inner.design_axis_size(),
        }
    }

    /// Name ID used as fallback when projection of names into a
    /// particular font model produces a subfamily name containing only
    /// elidable elements.
    pub fn elided_fallback_name_id(&self) -> Option<u16> {
        match self {
            Self::Version1_0(_inner) => None,
            Self::Version1_2(_inner) => Some(_inner.elided_fallback_name_id()),
        }
    }

    /// Major version number of the style attributes table — set to 1.
    pub fn major_version(&self) -> u16 {
        match self {
            Self::Version1_0(_inner) => _inner.major_version(),
            Self::Version1_2(_inner) => _inner.major_version(),
        }
    }

    /// Minor version number of the style attributes table — set to 2.
    pub fn minor_version(&self) -> u16 {
        match self {
            Self::Version1_0(_inner) => _inner.minor_version(),
            Self::Version1_2(_inner) => _inner.minor_version(),
        }
    }

    /// Offset in bytes from the beginning of the STAT table to the
    /// start of the design axes value offsets array. If axisValueCount
    /// is zero, set to zero; if axisValueCount is greater than zero,
    /// must be greater than zero.
    pub fn offset_to_axis_value_offsets(&self) -> Offset32 {
        match self {
            Self::Version1_0(_inner) => _inner.offset_to_axis_value_offsets(),
            Self::Version1_2(_inner) => _inner.offset_to_axis_value_offsets(),
        }
    }
}

impl<'a> font_types::OffsetHost<'a> for Stat<'a> {
    fn bytes(&self) -> &'a [u8] {
        match self {
            Self::Version1_0(_inner) => _inner.bytes(),
            Self::Version1_2(_inner) => _inner.bytes(),
        }
    }
}

/// [Axis Value Tables](https://docs.microsoft.com/en-us/typography/opentype/spec/stat#axis-value-tables)
pub enum AxisValue<'a> {
    Format1(AxisValueFormat1),
    Format2(AxisValueFormat2),
    Format3(AxisValueFormat3),
    Format4(AxisValueFormat4<'a>),
}

impl<'a> font_types::FontRead<'a> for AxisValue<'a> {
    fn read(bytes: &'a [u8]) -> Option<Self> {
        let version: BigEndian<u16> = font_types::FontRead::read(bytes)?;
        match version.get() {
            1 => Some(Self::Format1(font_types::FontRead::read(bytes)?)),
            2 => Some(Self::Format2(font_types::FontRead::read(bytes)?)),
            3 => Some(Self::Format3(font_types::FontRead::read(bytes)?)),
            4 => Some(Self::Format4(font_types::FontRead::read(bytes)?)),
            _other => {
                #[cfg(feature = "std")]
                {
                    eprintln!(
                        "unknown enum variant {:?} (table {})",
                        version,
                        stringify!(AxisValue)
                    );
                }
                None
            }
        }
    }
}

/// [Axis value table format 1](https://docs.microsoft.com/en-us/typography/opentype/spec/stat#axis-value-table-format-1)
#[derive(Clone, Copy, Debug, zerocopy :: FromBytes, zerocopy :: Unaligned)]
#[repr(C)]
pub struct AxisValueFormat1 {
    /// Format identifier — set to 1.
    pub format: BigEndian<u16>,
    /// Zero-base index into the axis record array identifying the axis
    /// of design variation to which the axis value table applies. Must
    /// be less than designAxisCount.
    pub axis_index: BigEndian<u16>,
    /// Flags — see below for details.
    pub flags: BigEndian<AxisValueFlags>,
    /// The name ID for entries in the 'name' table that provide a
    /// display string for this attribute value.
    pub value_name_id: BigEndian<u16>,
    /// A numeric value for this attribute value.
    pub value: BigEndian<Fixed>,
}

impl AxisValueFormat1 {
    /// Format identifier — set to 1.
    pub fn format(&self) -> u16 {
        self.format.get()
    }

    /// Zero-base index into the axis record array identifying the axis
    /// of design variation to which the axis value table applies. Must
    /// be less than designAxisCount.
    pub fn axis_index(&self) -> u16 {
        self.axis_index.get()
    }

    /// Flags — see below for details.
    pub fn flags(&self) -> AxisValueFlags {
        self.flags.get()
    }

    /// The name ID for entries in the 'name' table that provide a
    /// display string for this attribute value.
    pub fn value_name_id(&self) -> u16 {
        self.value_name_id.get()
    }

    /// A numeric value for this attribute value.
    pub fn value(&self) -> Fixed {
        self.value.get()
    }
}

/// [Axis value table format 2](https://docs.microsoft.com/en-us/typography/opentype/spec/stat#axis-value-table-format-2)
#[derive(Clone, Copy, Debug, zerocopy :: FromBytes, zerocopy :: Unaligned)]
#[repr(C)]
pub struct AxisValueFormat2 {
    /// Format identifier — set to 2.
    pub format: BigEndian<u16>,
    /// Zero-base index into the axis record array identifying the axis
    /// of design variation to which the axis value table applies. Must
    /// be less than designAxisCount.
    pub axis_index: BigEndian<u16>,
    /// Flags — see below for details.
    pub flags: BigEndian<AxisValueFlags>,
    /// The name ID for entries in the 'name' table that provide a
    /// display string for this attribute value.
    pub value_name_id: BigEndian<u16>,
    /// A nominal numeric value for this attribute value.
    pub nominal_value: BigEndian<Fixed>,
    /// The minimum value for a range associated with the specified
    /// name ID.
    pub range_min_value: BigEndian<Fixed>,
    /// The maximum value for a range associated with the specified
    /// name ID.
    pub range_max_value: BigEndian<Fixed>,
}

impl AxisValueFormat2 {
    /// Format identifier — set to 2.
    pub fn format(&self) -> u16 {
        self.format.get()
    }

    /// Zero-base index into the axis record array identifying the axis
    /// of design variation to which the axis value table applies. Must
    /// be less than designAxisCount.
    pub fn axis_index(&self) -> u16 {
        self.axis_index.get()
    }

    /// Flags — see below for details.
    pub fn flags(&self) -> AxisValueFlags {
        self.flags.get()
    }

    /// The name ID for entries in the 'name' table that provide a
    /// display string for this attribute value.
    pub fn value_name_id(&self) -> u16 {
        self.value_name_id.get()
    }

    /// A nominal numeric value for this attribute value.
    pub fn nominal_value(&self) -> Fixed {
        self.nominal_value.get()
    }

    /// The minimum value for a range associated with the specified
    /// name ID.
    pub fn range_min_value(&self) -> Fixed {
        self.range_min_value.get()
    }

    /// The maximum value for a range associated with the specified
    /// name ID.
    pub fn range_max_value(&self) -> Fixed {
        self.range_max_value.get()
    }
}

/// [Axis value table format 3](https://docs.microsoft.com/en-us/typography/opentype/spec/stat#axis-value-table-format-3)
#[derive(Clone, Copy, Debug, zerocopy :: FromBytes, zerocopy :: Unaligned)]
#[repr(C)]
pub struct AxisValueFormat3 {
    /// Format identifier — set to 3.
    pub format: BigEndian<u16>,
    /// Zero-base index into the axis record array identifying the axis
    /// of design variation to which the axis value table applies. Must
    /// be less than designAxisCount.
    pub axis_index: BigEndian<u16>,
    /// Flags — see below for details.
    pub flags: BigEndian<AxisValueFlags>,
    /// The name ID for entries in the 'name' table that provide a
    /// display string for this attribute value.
    pub value_name_id: BigEndian<u16>,
    /// A numeric value for this attribute value.
    pub value: BigEndian<Fixed>,
    /// The numeric value for a style-linked mapping from this value.
    pub linked_value: BigEndian<Fixed>,
}

impl AxisValueFormat3 {
    /// Format identifier — set to 3.
    pub fn format(&self) -> u16 {
        self.format.get()
    }

    /// Zero-base index into the axis record array identifying the axis
    /// of design variation to which the axis value table applies. Must
    /// be less than designAxisCount.
    pub fn axis_index(&self) -> u16 {
        self.axis_index.get()
    }

    /// Flags — see below for details.
    pub fn flags(&self) -> AxisValueFlags {
        self.flags.get()
    }

    /// The name ID for entries in the 'name' table that provide a
    /// display string for this attribute value.
    pub fn value_name_id(&self) -> u16 {
        self.value_name_id.get()
    }

    /// A numeric value for this attribute value.
    pub fn value(&self) -> Fixed {
        self.value.get()
    }

    /// The numeric value for a style-linked mapping from this value.
    pub fn linked_value(&self) -> Fixed {
        self.linked_value.get()
    }
}

/// [Axis value table format 4](https://docs.microsoft.com/en-us/typography/opentype/spec/stat#axis-value-table-format-4)
pub struct AxisValueFormat4<'a> {
    format: zerocopy::LayoutVerified<&'a [u8], BigEndian<u16>>,
    axis_count: zerocopy::LayoutVerified<&'a [u8], BigEndian<u16>>,
    flags: zerocopy::LayoutVerified<&'a [u8], BigEndian<AxisValueFlags>>,
    value_name_id: zerocopy::LayoutVerified<&'a [u8], BigEndian<u16>>,
    axis_values: zerocopy::LayoutVerified<&'a [u8], [AxisValueRecord]>,
}

impl<'a> font_types::FontRead<'a> for AxisValueFormat4<'a> {
    fn read(bytes: &'a [u8]) -> Option<Self> {
        let (format, bytes) =
            zerocopy::LayoutVerified::<_, BigEndian<u16>>::new_unaligned_from_prefix(bytes)?;
        let (axis_count, bytes) =
            zerocopy::LayoutVerified::<_, BigEndian<u16>>::new_unaligned_from_prefix(bytes)?;
        let __resolved_axis_count = axis_count.get();
        let (flags, bytes) =
            zerocopy::LayoutVerified::<_, BigEndian<AxisValueFlags>>::new_unaligned_from_prefix(
                bytes,
            )?;
        let (value_name_id, bytes) =
            zerocopy::LayoutVerified::<_, BigEndian<u16>>::new_unaligned_from_prefix(bytes)?;
        let (axis_values, bytes) =
            zerocopy::LayoutVerified::<_, [AxisValueRecord]>::new_slice_unaligned_from_prefix(
                bytes,
                __resolved_axis_count as usize,
            )?;
        let _bytes = bytes;
        let result = AxisValueFormat4 {
            format,
            axis_count,
            flags,
            value_name_id,
            axis_values,
        };
        Some(result)
    }
}

impl<'a> AxisValueFormat4<'a> {
    /// Format identifier — set to 4.
    pub fn format(&self) -> u16 {
        self.format.get()
    }

    /// The total number of axes contributing to this axis-values
    /// combination.
    pub fn axis_count(&self) -> u16 {
        self.axis_count.get()
    }

    /// Flags — see below for details.
    pub fn flags(&self) -> AxisValueFlags {
        self.flags.get()
    }

    /// The name ID for entries in the 'name' table that provide a
    /// display string for this combination of axis values.
    pub fn value_name_id(&self) -> u16 {
        self.value_name_id.get()
    }

    /// Array of AxisValue records that provide the combination of axis
    /// values, one for each contributing axis.
    pub fn axis_values(&self) -> &[AxisValueRecord] {
        &self.axis_values
    }
}

/// Part of [AxisValueFormat4]
#[derive(Clone, Copy, Debug, zerocopy :: FromBytes, zerocopy :: Unaligned)]
#[repr(C)]
pub struct AxisValueRecord {
    /// Zero-base index into the axis record array identifying the axis
    /// to which this value applies. Must be less than designAxisCount.
    pub axis_index: BigEndian<u16>,
    /// A numeric value for this attribute value.
    pub value: BigEndian<Fixed>,
}

impl AxisValueRecord {
    /// Zero-base index into the axis record array identifying the axis
    /// to which this value applies. Must be less than designAxisCount.
    pub fn axis_index(&self) -> u16 {
        self.axis_index.get()
    }

    /// A numeric value for this attribute value.
    pub fn value(&self) -> Fixed {
        self.value.get()
    }
}

bitflags::bitflags! { # [doc = " [Axis value table flags](https://docs.microsoft.com/en-us/typography/opentype/spec/stat#flags)."] pub struct AxisValueFlags : u16 { # [doc = " If set, this axis value table provides axis value information"] # [doc = " that is applicable to other fonts within the same font family."] # [doc = " This is used if the other fonts were released earlier and did"] # [doc = " not include information about values for some axis. If newer"] # [doc = " versions of the other fonts include the information themselves"] # [doc = " and are present, then this table is ignored."] const OLDER_SIBLING_FONT_ATTRIBUTE = 0x0001 ; # [doc = " If set, it indicates that the axis value represents the"] # [doc = " “normal” value for the axis and may be omitted when"] # [doc = " composing name strings."] const ELIDABLE_AXIS_VALUE_NAME = 0x0002 ; } }

impl font_types::Scalar for AxisValueFlags {
    type Raw = <u16 as font_types::Scalar>::Raw;
    fn to_raw(self) -> Self::Raw {
        self.bits().to_raw()
    }
    fn from_raw(raw: Self::Raw) -> Self {
        let t = <u16>::from_raw(raw);
        Self::from_bits_truncate(t)
    }
}
