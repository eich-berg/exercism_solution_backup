// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration { seconds: u64 }

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        //todo!("s, measured in seconds: {s}")
        Duration { seconds: s }
    }
}

pub trait Planet {
    const orb_period: f64;
    fn years_during(d: &Duration) -> f64 {
        //todo!("convert a duration ({d:?}) to the number of years on this planet for that duration");
        // Earth's orbital period in seconds (365.25 days)
        const earth_orb_period_secs : f64 = 31557600.0;
        // Convert seconds to Earth years, then divide by planet's orbital period
        d.seconds as f64 / earth_orb_period_secs / Self::orb_period
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury { const orb_period: f64 = 0.2408467; }
impl Planet for Venus { const orb_period: f64 = 0.61519726; }
impl Planet for Earth { const orb_period: f64 = 1.0; }
impl Planet for Mars { const orb_period: f64 = 1.8808158; }
impl Planet for Jupiter { const orb_period: f64 = 11.862615; }
impl Planet for Saturn { const orb_period: f64 = 29.447498; }
impl Planet for Uranus { const orb_period: f64 = 84.016846; }
impl Planet for Neptune { const orb_period: f64 = 164.79132; }
