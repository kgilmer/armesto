use crate::dbus::NotificationUrgency;
use crate::error::{Error, Result};
use colorsys::Rgb;
use serde::de::{Deserializer, Error as SerdeError};
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};
use sscanf::scanf;
use std::fs;
use std::result::Result as StdResult;
use std::str::FromStr;

/// Name of the default configuration file.
pub const DEFAULT_CONFIG: &str = concat!(env!("CARGO_PKG_NAME"), ".toml");

/// Configuration.
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    /// Global configuration.
    pub global: GlobalConfig,
    /// Configuration for low urgency.
    pub urgency_low: UrgencyConfig,
    /// Configuration for normal urgency.
    pub urgency_normal: UrgencyConfig,
    /// Configuration for critical urgency.
    pub urgency_critical: UrgencyConfig,
}

impl Config {
    /// Parses the configuration file.
    pub fn parse(file: &str) -> Result<Self> {
        let contents = fs::read_to_string(file)?;
        let config = toml::from_str(&contents)?;
        Ok(config)
    }

    /// Returns the appropriate urgency configuration.
    pub fn get_urgency_config(&self, urgency: &NotificationUrgency) -> UrgencyConfig {
        match urgency {
            NotificationUrgency::Low => self.urgency_low.clone(),
            NotificationUrgency::Normal => self.urgency_normal.clone(),
            NotificationUrgency::Critical => self.urgency_critical.clone(),
        }
    }
}

/// Global configuration.
#[derive(Debug, Deserialize, Serialize)]
pub struct GlobalConfig {
    /// Geometry of the notification window.
    #[serde(deserialize_with = "deserialize_geometry_from_string")]
    pub geometry: Geometry,
    /// Text font.
    pub font: String,
}

/// Custom deserializer implementation for converting `String` to [`Geometry`]
fn deserialize_geometry_from_string<'de, D>(deserializer: D) -> StdResult<Geometry, D::Error>
where
    D: Deserializer<'de>,
{
    let value: &str = Deserialize::deserialize(deserializer)?;
    Geometry::from_str(value).map_err(SerdeError::custom)
}

/// Window geometry.
#[derive(Debug, Deserialize, Serialize)]
pub struct Geometry {
    /// Width of the window.
    pub width: u32,
    /// Height of the window.
    pub height: u32,
    /// X coordinate.
    pub x: u32,
    /// Y coordinate.
    pub y: u32,
}

impl FromStr for Geometry {
    type Err = Error;
    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        let (width, height, x, y) =
            scanf!(s, "{u32}x{u32}+{u32}+{u32}").map_err(|e| Error::Scanf(e.to_string()))?;
        Ok(Self {
            width,
            height,
            x,
            y,
        })
    }
}

/// Urgency configuration.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UrgencyConfig {
    /// Background color.
    #[serde(
        deserialize_with = "deserialize_rgb_from_string",
        serialize_with = "serialize_rgb_to_string"
    )]
    pub background: Rgb,
    /// Foreground color.
    #[serde(
        deserialize_with = "deserialize_rgb_from_string",
        serialize_with = "serialize_rgb_to_string"
    )]
    pub foreground: Rgb,
    /// Timeout value.
    pub timeout: u32,
}

/// Custom deserializer implementation for converting `String` to [`Rgb`]
fn deserialize_rgb_from_string<'de, D>(deserializer: D) -> StdResult<Rgb, D::Error>
where
    D: Deserializer<'de>,
{
    let value: &str = Deserialize::deserialize(deserializer)?;
    Rgb::from_hex_str(value).map_err(SerdeError::custom)
}

/// Custom serializer implementation for converting [`Rgb`] to `String`
fn serialize_rgb_to_string<S>(value: &Rgb, s: S) -> StdResult<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&value.to_hex_string())
}
