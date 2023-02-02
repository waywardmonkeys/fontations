//! The [glyf (Glyph Data)](https://docs.microsoft.com/en-us/typography/opentype/spec/glyf) table

use kurbo::{BezPath, Shape};

use read_fonts::tables::glyf::{CurvePoint, SimpleGlyphFlags};

use crate::FontWrite;

/// A single quadratic bezier contour
#[derive(Clone, Debug)]
pub struct Contour(Vec<CurvePoint>);

/// A simple (without components) glyph
pub struct SimpleGlyf {
    x_max: i16,
    y_max: i16,
    x_min: i16,
    y_min: i16,
    contours: Vec<Contour>,
    _instructions: Vec<u8>,
}

/// An error if an input curve is malformed
#[derive(Clone, Debug)]
pub enum BadKurbo {
    HasCubic,
    TooSmall,
    MissingMove,
}

/// A helper trait for converting other point types to open-type compatible reprs
pub trait OtPoint {
    fn get(self) -> (i16, i16);
}

impl OtPoint for kurbo::Point {
    fn get(self) -> (i16, i16) {
        (ot_round(self.x as f32), ot_round(self.y as f32))
    }
}

impl OtPoint for (i16, i16) {
    fn get(self) -> (i16, i16) {
        self
    }
}

// adapted from simon:
// https://github.com/simoncozens/rust-font-tools/blob/105436d3a617ddbebd25f790b041ff506bd90d44/otmath/src/lib.rs#L17
fn ot_round(val: f32) -> i16 {
    (val + 0.5).floor() as i16
}

impl Contour {
    /// Create a new contour begining at the provided point
    pub fn new(pt: impl OtPoint) -> Self {
        let (x, y) = pt.get();
        Self(vec![CurvePoint::on_curve(x, y)])
    }

    /// The total number of points in this contour
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// `true` if this contour is empty
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Add a line segment
    pub fn line_to(&mut self, pt: impl OtPoint) {
        let (x, y) = pt.get();
        self.0.push(CurvePoint::on_curve(x, y));
    }

    /// Add a quadratic curve segment
    pub fn quad_to(&mut self, p0: impl OtPoint, p1: impl OtPoint) {
        let (x0, y0) = p0.get();
        let (x1, y1) = p1.get();
        self.0.push(CurvePoint::off_curve(x0, y0));
        self.0.push(CurvePoint::on_curve(x1, y1));
    }
}

impl SimpleGlyf {
    /// Attempt to create a simple glyph from a kurbo `BezPath`
    ///
    /// The path may contain only line and quadratic bezier segments. The caller
    /// is responsible for converting any cubic segments to quadratics before
    /// calling.
    ///
    /// Returns an error if the input path is malformed; that is, if it is empty,
    /// contains cubic segments, or does not begin with a 'move' instruction.
    //TODO: figure out a more general API? maybe a nested builder thing, where you
    //build contours, and from those contours build a glyph? idk?
    pub fn from_kurbo(path: &BezPath) -> Result<Self, BadKurbo> {
        let mut contours = Vec::new();
        let mut current = None;

        for el in path.elements() {
            match el {
                kurbo::PathEl::MoveTo(pt) => {
                    if let Some(prev) = current.take() {
                        contours.push(prev);
                    }
                    current = Some(Contour::new(*pt));
                }
                kurbo::PathEl::LineTo(pt) => {
                    current.as_mut().ok_or(BadKurbo::MissingMove)?.line_to(*pt)
                }
                kurbo::PathEl::QuadTo(p0, p1) => current
                    .as_mut()
                    .ok_or(BadKurbo::MissingMove)?
                    .quad_to(*p0, *p1),
                kurbo::PathEl::CurveTo(_, _, _) => return Err(BadKurbo::HasCubic),
                // I think we can just ignore this, and remove duplicate points
                // at the end?
                kurbo::PathEl::ClosePath => (),
            }
        }

        contours.extend(current);

        for contour in &mut contours {
            //TODO: verify that single-point contours are actually meaningless?
            if contour.len() < 2 {
                return Err(BadKurbo::TooSmall);
            }
            if contour.0.first() == contour.0.last() {
                contour.0.pop();
            }

            // ot point order is reversed, and the last point is first
            contour.0.reverse();
            contour.0.rotate_right(1);
        }

        let bbox = path.bounding_box();
        Ok(SimpleGlyf {
            x_max: ot_round(bbox.max_x() as f32),
            y_max: ot_round(bbox.max_y() as f32),
            x_min: ot_round(bbox.min_x() as f32),
            y_min: ot_round(bbox.min_y() as f32),
            contours,
            _instructions: Default::default(),
        })
    }

    /// Compute the flags and deltas for this glyph's points.
    ///
    /// This does not do the final binary encoding, and it also does not handle
    /// repeating flags, which doesn't really work when we're an iterator.
    ///
    // this is adapted from simon's implementation at
    // https://github.com/simoncozens/rust-font-tools/blob/105436d3a617ddbebd25f790b041ff506bd90d44/fonttools-rs/src/tables/glyf/glyph.rs#L268
    fn compute_point_deltas(
        &self,
    ) -> impl Iterator<Item = (SimpleGlyphFlags, CoordDelta, CoordDelta)> + '_ {
        // reused for x & y by passing in the flags
        fn flag_and_delta(
            value: i16,
            short_flag: SimpleGlyphFlags,
            same_or_pos: SimpleGlyphFlags,
        ) -> (SimpleGlyphFlags, CoordDelta) {
            const SHORT_MAX: i16 = u8::MAX as i16;
            const SHORT_MIN: i16 = -SHORT_MAX;
            match value {
                0 => (same_or_pos, CoordDelta::Skip),
                SHORT_MIN..=-1 => (short_flag, CoordDelta::Short(value.unsigned_abs() as u8)),
                1..=SHORT_MAX => (short_flag | same_or_pos, CoordDelta::Short(value as _)),
                _other => (SimpleGlyphFlags::empty(), CoordDelta::Long(value)),
            }
        }

        let (mut last_x, mut last_y) = (0, 0);
        let mut iter = self.contours.iter().flatten();
        std::iter::from_fn(move || {
            let point = iter.next()?;
            let mut flag = SimpleGlyphFlags::empty();
            let d_x = point.x - last_x;
            let d_y = point.y - last_y;
            last_x = point.x;
            last_y = point.y;

            if point.on_curve {
                flag |= SimpleGlyphFlags::ON_CURVE_POINT;
            }
            let (x_flag, x_data) = flag_and_delta(
                d_x,
                SimpleGlyphFlags::X_SHORT_VECTOR,
                SimpleGlyphFlags::X_IS_SAME_OR_POSITIVE_X_SHORT_VECTOR,
            );
            let (y_flag, y_data) = flag_and_delta(
                d_y,
                SimpleGlyphFlags::Y_SHORT_VECTOR,
                SimpleGlyphFlags::Y_IS_SAME_OR_POSITIVE_Y_SHORT_VECTOR,
            );

            flag |= x_flag | y_flag;
            Some((flag, x_data, y_data))
        })
    }
}

/// A little helper for managing how we're representing a given delta
#[derive(Clone, Copy, Debug)]
enum CoordDelta {
    // this is a repeat (set in the flag) and so we write nothing
    Skip,
    Short(u8),
    Long(i16),
}

impl FontWrite for CoordDelta {
    fn write_into(&self, writer: &mut crate::TableWriter) {
        match self {
            CoordDelta::Skip => (),
            CoordDelta::Short(val) => val.write_into(writer),
            CoordDelta::Long(val) => val.write_into(writer),
        }
    }
}

/// A little helper for writing flags that may have a 'repeat' byte
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct RepeatableFlag {
    flag: SimpleGlyphFlags,
    repeat: u8,
}

impl FontWrite for RepeatableFlag {
    fn write_into(&self, writer: &mut crate::TableWriter) {
        debug_assert_eq!(
            self.flag.contains(SimpleGlyphFlags::REPEAT_FLAG),
            self.repeat > 0
        );

        self.flag.bits().write_into(writer);
        if self.flag.contains(SimpleGlyphFlags::REPEAT_FLAG) {
            self.repeat.write_into(writer);
        }
    }
}

impl RepeatableFlag {
    /// given an iterator over raw flags, return an iterator over flags + repeat values
    // writing this as an iterator instead of just returning a vec is very marginal
    // gains, but I'm just in the habit at this point
    fn iter_from_flags(
        flags: impl IntoIterator<Item = SimpleGlyphFlags>,
    ) -> impl Iterator<Item = RepeatableFlag> {
        let mut iter = flags.into_iter();
        let mut prev = None;

        std::iter::from_fn(move || loop {
            match (iter.next(), prev.take()) {
                (None, prev) => return prev,
                (Some(flag), None) => prev = Some(RepeatableFlag { flag, repeat: 0 }),
                (Some(flag), Some(mut last)) => {
                    if (last.flag & !SimpleGlyphFlags::REPEAT_FLAG) == flag && last.repeat < u8::MAX
                    {
                        last.repeat += 1;
                        last.flag |= SimpleGlyphFlags::REPEAT_FLAG;
                        prev = Some(last);
                    } else {
                        prev = Some(RepeatableFlag { flag, repeat: 0 });
                        return Some(last);
                    }
                }
            }
        })
    }
}

impl<'a> IntoIterator for &'a Contour {
    type Item = &'a CurvePoint;

    type IntoIter = std::slice::Iter<'a, CurvePoint>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl FontWrite for SimpleGlyf {
    fn write_into(&self, writer: &mut crate::TableWriter) {
        assert!(self.contours.len() < i16::MAX as usize);
        assert!(self._instructions.len() < u16::MAX as usize);
        let n_contours = self.contours.len() as i16;
        n_contours.write_into(writer);
        let bbox = [self.x_max, self.y_max, self.x_min, self.y_min];
        bbox.write_into(writer);
        // now write end points of contours:
        let mut cur = 0;
        for contour in &self.contours {
            cur += contour.len();
            (cur as u16 - 1).write_into(writer);
        }
        (self._instructions.len() as u16).write_into(writer);
        self._instructions.write_into(writer);

        let deltas = self.compute_point_deltas().collect::<Vec<_>>();
        RepeatableFlag::iter_from_flags(deltas.iter().map(|(flag, _, _)| *flag))
            .for_each(|flag| flag.write_into(writer));
        deltas.iter().for_each(|(_, x, _)| x.write_into(writer));
        deltas.iter().for_each(|(_, _, y)| y.write_into(writer));
    }
}

impl crate::validate::Validate for SimpleGlyf {
    fn validate_impl(&self, _ctx: &mut crate::codegen_prelude::ValidationCtx) {
        // pass
    }
}

#[cfg(test)]
mod tests {
    use read::{FontData, FontRead};

    use super::*;

    #[test]
    fn very_simple_glyph() {
        let mut path = BezPath::new();
        path.move_to((20., -100.));
        path.quad_to((1337., 1338.), (-50., -69.0));
        path.quad_to((13., 255.), (-255., 256.));
        path.line_to((20., -100.));

        let glyph = SimpleGlyf::from_kurbo(&path).unwrap();
        let bytes = crate::dump_table(&glyph).unwrap();
        let read = read_fonts::tables::glyf::SimpleGlyph::read(FontData::new(&bytes)).unwrap();
        assert_eq!(read.number_of_contours(), 1);
        assert_eq!(read.num_points(), 5);
        assert_eq!(read.end_pts_of_contours(), &[4]);
        let mut points = read.points().collect::<Vec<_>>();
        points.rotate_left(1);
        points.reverse();
        assert_eq!(points[0].x, 20);
        assert_eq!(points[1].y, 1338);
        assert!(!points[1].on_curve);
        assert_eq!(points[4].x, -255);
        assert_eq!(points[4].y, 256);
        assert!(points[4].on_curve);
    }

    #[test]
    fn compile_repeatable_flags() {
        let mut path = BezPath::new();
        path.move_to((20., -100.));
        path.line_to((80., -20.));
        path.line_to((50., -69.));
        path.line_to((25., -90.));
        // before reversal/rotation: A B C D
        // after reversal D C B A
        // after r-rot-1 A D C B

        let glyph = SimpleGlyf::from_kurbo(&path).unwrap();
        let flags = glyph
            .compute_point_deltas()
            .map(|x| x.0)
            .collect::<Vec<_>>();
        let r_flags = RepeatableFlag::iter_from_flags(flags.iter().copied()).collect::<Vec<_>>();

        assert_eq!(r_flags.len(), 2, "{r_flags:?}");
        let bytes = crate::dump_table(&glyph).unwrap();
        let read = read_fonts::tables::glyf::SimpleGlyph::read(FontData::new(&bytes)).unwrap();
        assert_eq!(read.number_of_contours(), 1);
        assert_eq!(read.num_points(), 4);
        assert_eq!(read.end_pts_of_contours(), &[3]);
        let points = read.points().collect::<Vec<_>>();
        assert_eq!(points[0].x, 20);
        assert_eq!(points[0].y, -100);
        assert_eq!(points[1].x, 25);
        assert_eq!(points[1].y, -90);
        assert_eq!(points[2].x, 50);
        assert_eq!(points[2].y, -69);
        assert_eq!(points[3].x, 80);
        assert_eq!(points[3].y, -20);
    }

    #[test]
    fn repeatable_flags_basic() {
        let flags = [
            SimpleGlyphFlags::ON_CURVE_POINT,
            SimpleGlyphFlags::X_SHORT_VECTOR,
        ];
        let repeatable = RepeatableFlag::iter_from_flags(flags).collect::<Vec<_>>();
        assert_eq!(repeatable[0].repeat, 0);
        assert_eq!(repeatable[1].flag, flags[1]);
    }

    #[test]
    fn repeatable_flags_repeats() {
        let some_dupes = std::iter::repeat(SimpleGlyphFlags::ON_CURVE_POINT).take(4);
        let many_dupes = std::iter::repeat(SimpleGlyphFlags::Y_SHORT_VECTOR).take(257);
        let repeatable =
            RepeatableFlag::iter_from_flags(some_dupes.chain(many_dupes)).collect::<Vec<_>>();
        assert_eq!(repeatable.len(), 3);
        assert_eq!(
            repeatable[0],
            RepeatableFlag {
                flag: SimpleGlyphFlags::ON_CURVE_POINT | SimpleGlyphFlags::REPEAT_FLAG,
                repeat: 3
            }
        );
        assert_eq!(
            repeatable[1],
            RepeatableFlag {
                flag: SimpleGlyphFlags::Y_SHORT_VECTOR | SimpleGlyphFlags::REPEAT_FLAG,
                repeat: u8::MAX,
            }
        );

        assert_eq!(
            repeatable[2],
            RepeatableFlag {
                flag: SimpleGlyphFlags::Y_SHORT_VECTOR,
                repeat: 0,
            }
        )
    }
}