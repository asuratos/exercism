#[derive(Debug)]
pub struct Duration {
    secs: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { secs: s as f64 }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

macro_rules! planet_s2y_conv {
    ($planet:ident, $p:ty, $earthyear_mod:literal) => {
        pub struct $planet;

        impl Planet for $p {
            fn years_during(d: &Duration) -> f64 {
                (d.secs / (31_557_600_f64)) / ($earthyear_mod)
            }
        }
    };
}

planet_s2y_conv!(Mercury, Mercury, 0.2408467_f64);
planet_s2y_conv!(Venus, Venus, 0.61519726_f64);
planet_s2y_conv!(Earth, Earth, 1_f64);
planet_s2y_conv!(Mars, Mars, 1.8808158_f64);
planet_s2y_conv!(Jupiter, Jupiter, 11.862615_f64);
planet_s2y_conv!(Saturn, Saturn, 29.447498_f64);
planet_s2y_conv!(Uranus, Uranus, 84.016846_f64);
planet_s2y_conv!(Neptune, Neptune, 164.79132_f64);
