//! Palette strategies: interchangeable recipes that turn the same seed into a
//! different theme style. The toolbar picks one; `PaletteState` regenerates
//! with it.
//!
//! One folder per strategy, one file per mode:
//!
//! ```text
//! strategy/
//! ├── default/  light.rs, dark.rs   tinted surfaces (base rules)
//! └── pure/     light.rs, dark.rs   white/black surfaces over default
//! ```

mod default;
mod pure;

use super::PaletteRecipe;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PaletteStrategy {
    /// Seed-tinted surfaces.
    #[default]
    Default,
    /// Plain white / black backgrounds.
    Pure,
}

impl PaletteStrategy {
    pub const ALL: [PaletteStrategy; 2] = [PaletteStrategy::Default, PaletteStrategy::Pure];

    pub fn title(self) -> &'static str {
        match self {
            PaletteStrategy::Default => "Default",
            PaletteStrategy::Pure => "Pure",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            PaletteStrategy::Default => "Tinted surfaces",
            PaletteStrategy::Pure => "White and black backgrounds",
        }
    }

    /// Stable key, used for persistence.
    pub fn as_str(self) -> &'static str {
        match self {
            PaletteStrategy::Default => "default",
            PaletteStrategy::Pure => "pure",
        }
    }

    /// Recipe that generates this strategy's palette.
    pub fn recipe(self) -> PaletteRecipe {
        match self {
            PaletteStrategy::Default => PaletteRecipe::new(default::LIGHT, default::DARK),
            PaletteStrategy::Pure => PaletteRecipe::new(pure::LIGHT, pure::DARK),
        }
    }
}

impl std::str::FromStr for PaletteStrategy {
    type Err = ();

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        PaletteStrategy::ALL
            .into_iter()
            .find(|strategy| strategy.as_str() == text)
            .ok_or(())
    }
}

#[cfg(test)]
mod tests {
    use super::PaletteStrategy;

    #[test]
    fn round_trips_every_strategy() {
        for strategy in PaletteStrategy::ALL {
            assert_eq!(strategy.as_str().parse(), Ok(strategy));
        }
    }

    #[test]
    fn rejects_unknown_text() {
        assert!("neon".parse::<PaletteStrategy>().is_err());
    }
}
