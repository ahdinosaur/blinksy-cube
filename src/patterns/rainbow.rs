use blinksy::{
    color::{Hsv, HsvHueRainbow},
    layout::Layout1d,
    markers::Dim1d,
    pattern::Pattern,
};

/// Configuration parameters for the Rainbow pattern.
#[derive(Debug)]
pub struct RainbowParams {
    /// Controls the speed of the animation (higher = faster)
    pub rainbow_speed: f32,
    /// Controls the spatial density of the rainbow (higher = more compressed)
    pub rainbow_zoom: f32,
}

impl Default for RainbowParams {
    fn default() -> Self {
        const MILLISECONDS_PER_SECOND: f32 = 1e3;
        Self {
            rainbow_speed: 0.3 / MILLISECONDS_PER_SECOND,
            rainbow_zoom: 1.,
        }
    }
}

/// Rainbow pattern implementation.
///
/// Creates a smooth transition through the full HSV spectrum across the LED layout.
#[derive(Debug)]
pub struct Rainbow {
    /// Configuration parameters
    params: RainbowParams,
}

impl<Layout> Pattern<Dim1d, Layout> for Rainbow
where
    Layout: Layout1d,
{
    type Params = RainbowParams;
    type Color = Hsv<HsvHueRainbow>;

    /// Creates a new Rainbow pattern with the specified parameters.
    fn new(params: Self::Params) -> Self {
        Self { params }
    }

    /// Generates colors for a 1D layout.
    ///
    /// The rainbow pattern creates a smooth transition of hues across the layout,
    /// which shifts over time to create a flowing effect.
    fn tick(&self, time_in_ms: u64) -> impl Iterator<Item = Self::Color> {
        let Self { params } = self;
        let RainbowParams {
            rainbow_speed,
            rainbow_zoom,
        } = params;

        let rainbow_time = time_in_ms as f32 * rainbow_speed;
        let rainbow_step = 0.5 * rainbow_zoom;

        Layout::points().map(move |x| {
            let hue = x * rainbow_step + rainbow_time;
            let saturation = 1.;
            let value = 1.;
            Hsv::new(hue, saturation, value)
        })
    }
}
