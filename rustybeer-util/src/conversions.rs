use regex::Regex;
use std::num::ParseFloatError;

/// Used to build new measurements::Temperature structs.
///
/// To be removed if the dependency some time allows creating measurement units from
/// strings.
pub struct TemperatureBuilder;

impl TemperatureBuilder {
    /// Creates measurements::Temperature from string
    ///
    /// Tries to figure out the temperature unit from the string. If the string value is plain
    /// number, it will be considered as Celsius. Also empty strings are considered as
    /// zero Celsius in Temperature.
    pub fn new(val: &str) -> Result<measurements::Temperature, ParseFloatError> {
        if val.is_empty() {
            return Ok(measurements::Temperature::from_celsius(0.0));
        }

        let re = Regex::new(r"([0-9.]*)\s?([a-zA-Z]{1})$").unwrap();
        if let Some(caps) = re.captures(val) {
            let float_val = caps.get(1).unwrap().as_str();
            return Ok(
                match caps.get(2).unwrap().as_str().to_uppercase().as_str() {
                    "F" => measurements::Temperature::from_fahrenheit(float_val.parse::<f64>()?),
                    "C" => measurements::Temperature::from_celsius(float_val.parse::<f64>()?),
                    "K" => measurements::Temperature::from_kelvin(float_val.parse::<f64>()?),
                    "R" => measurements::Temperature::from_rankine(float_val.parse::<f64>()?),
                    _ => measurements::Temperature::from_celsius(val.parse::<f64>()?),
                },
            );
        }

        Ok(measurements::Temperature::from_celsius(val.parse::<f64>()?))
    }
}

/// Used to build new measurements::Volume structs.
///
/// To be removed if the dependency some time allows creating measurement units from
/// strings.
pub struct VolumeBuilder;

impl VolumeBuilder {
    /// Creates measurements::Volume from string
    ///
    /// Tries to figure out the volume unit from the string. If the string value is plain
    /// number, it will be considered as litres. Also empty strings are considered as
    /// zero litres in Volume.
    pub fn new(val: &str) -> Result<measurements::Volume, ParseFloatError> {
        if val.is_empty() {
            return Ok(measurements::Volume::from_litres(0.0));
        }

        let re = Regex::new(r"([0-9.]*)\s?([a-zA-Z]{1,3}[0-9]{0,1})$").unwrap();
        if let Some(caps) = re.captures(val) {
            let float_val = caps.get(1).unwrap().as_str();
            return Ok(
                match caps.get(2).unwrap().as_str().to_lowercase().as_str() {
                    "cm3" => {
                        measurements::Volume::from_cubic_centimeters(float_val.parse::<f64>()?)
                    }
                    "ft3" => measurements::Volume::from_cubic_feet(float_val.parse::<f64>()?),
                    "yd3" => measurements::Volume::from_cubic_yards(float_val.parse::<f64>()?),
                    "in3" => measurements::Volume::from_cubic_inches(float_val.parse::<f64>()?),
                    "gal" => measurements::Volume::from_gallons(float_val.parse::<f64>()?),
                    "cup" => measurements::Volume::from_cups(float_val.parse::<f64>()?),
                    "tsp" => measurements::Volume::from_teaspoons(float_val.parse::<f64>()?),
                    "ml" => measurements::Volume::from_milliliters(float_val.parse::<f64>()?),
                    "m3" => measurements::Volume::from_cubic_meters(float_val.parse::<f64>()?),
                    "μl" => measurements::Volume::from_drops(float_val.parse::<f64>()?),
                    "dr" => measurements::Volume::from_drams(float_val.parse::<f64>()?),
                    "l" => measurements::Volume::from_litres(float_val.parse::<f64>()?),
                    "p" => measurements::Volume::from_pints(float_val.parse::<f64>()?),
                    "ʒ" => measurements::Volume::from_pints(float_val.parse::<f64>()?),
                    _ => measurements::Volume::from_litres(val.parse::<f64>()?),
                },
            );
        }

        Ok(measurements::Volume::from_litres(val.parse::<f64>()?))
    }
}

/// Used to build new measurements::Mass structs.
///
/// To be removed if the dependency some time allows creating measurement units from
/// strings.
pub struct MassBuilder;

impl MassBuilder {
    /// Creates measurements::Mass from string
    ///
    /// Tries to figure out the mass unit from the string. If the string value is plain
    /// number, it will be considered as grams. Also empty strings are considered as
    /// zero grams in Mass.
    pub fn new(val: &str) -> Result<measurements::Mass, ParseFloatError> {
        if val.is_empty() {
            return Ok(measurements::Mass::from_grams(0.0));
        }

        let re = Regex::new(r"([0-9.]*)\s?([a-zA-Zμ]{1,3})$").unwrap();
        if let Some(caps) = re.captures(val) {
            let float_val = caps.get(1).unwrap().as_str();
            return Ok(match caps.get(2).unwrap().as_str() {
                "ug" | "μg" => measurements::Mass::from_micrograms(float_val.parse::<f64>()?),
                "mg" => measurements::Mass::from_milligrams(float_val.parse::<f64>()?),
                "ct" => measurements::Mass::from_carats(float_val.parse::<f64>()?),
                "g" => measurements::Mass::from_grams(float_val.parse::<f64>()?),
                "kg" => measurements::Mass::from_kilograms(float_val.parse::<f64>()?),
                "T" => measurements::Mass::from_metric_tons(float_val.parse::<f64>()?),
                "gr" => measurements::Mass::from_grains(float_val.parse::<f64>()?),
                "dwt" => measurements::Mass::from_pennyweights(float_val.parse::<f64>()?),
                "oz" => measurements::Mass::from_ounces(float_val.parse::<f64>()?),
                "st" => measurements::Mass::from_stones(float_val.parse::<f64>()?),
                "lbs" => measurements::Mass::from_pounds(float_val.parse::<f64>()?),
                _ => measurements::Mass::from_grams(float_val.parse::<f64>()?),
            });
        }

        Ok(measurements::Mass::from_grams(val.parse::<f64>()?))
    }
}

#[cfg(test)]
mod tests {
    use super::{MassBuilder, TemperatureBuilder, VolumeBuilder};
    use approx::assert_relative_eq;

    #[test]
    fn fahrenheit_from_string() {
        assert_relative_eq!(
            123.0,
            TemperatureBuilder::new("123F").unwrap().as_fahrenheit(),
        );
        assert_relative_eq!(
            123.0,
            TemperatureBuilder::new("123 F").unwrap().as_fahrenheit(),
        );
        assert_relative_eq!(
            123.0,
            TemperatureBuilder::new("123 f").unwrap().as_fahrenheit(),
        );
    }

    #[test]
    fn celsius_from_string() {
        assert_relative_eq!(123.0, TemperatureBuilder::new("123C").unwrap().as_celsius(),);
        assert_relative_eq!(
            123.0,
            TemperatureBuilder::new("123 C").unwrap().as_celsius(),
        );
        assert_relative_eq!(
            123.0,
            TemperatureBuilder::new("123 c").unwrap().as_celsius(),
        );
    }

    #[test]
    fn kelvin_from_string() {
        assert_relative_eq!(123.0, TemperatureBuilder::new("123K").unwrap().as_kelvin(),);
        assert_relative_eq!(123.0, TemperatureBuilder::new("123 K").unwrap().as_kelvin(),);
        assert_relative_eq!(123.0, TemperatureBuilder::new("123 k").unwrap().as_kelvin(),);
    }

    #[test]
    fn rankine_from_string() {
        assert_relative_eq!(123.0, TemperatureBuilder::new("123R").unwrap().as_rankine(),);
        assert_relative_eq!(
            123.0,
            TemperatureBuilder::new("123 R").unwrap().as_rankine(),
        );
        assert_relative_eq!(
            123.0,
            TemperatureBuilder::new("123 r").unwrap().as_rankine(),
        );
    }

    #[test]
    fn default_from_string() {
        assert_relative_eq!(123.0, TemperatureBuilder::new("123").unwrap().as_celsius(),);

        assert_relative_eq!(123.0, VolumeBuilder::new("123").unwrap().as_litres(),);
        assert_relative_eq!(123.0, MassBuilder::new("123").unwrap().as_grams(),);
    }

    #[test]
    fn zero_from_string() {
        assert_relative_eq!(0., TemperatureBuilder::new("").unwrap().as_celsius(),);
        assert_relative_eq!(0., VolumeBuilder::new("").unwrap().as_litres());
        assert_relative_eq!(0., MassBuilder::new("").unwrap().as_grams());
    }

    #[test]
    fn cubic_centimeters_from_string() {
        assert_relative_eq!(
            123.0,
            VolumeBuilder::new("123cm3").unwrap().as_cubic_centimeters(),
        );
        assert_relative_eq!(
            123.0,
            VolumeBuilder::new("123 cm3")
                .unwrap()
                .as_cubic_centimeters(),
        );
        assert_relative_eq!(
            123.0,
            VolumeBuilder::new("123 CM3")
                .unwrap()
                .as_cubic_centimeters(),
        );
    }

    #[test]
    fn milliliters_from_string() {
        assert_relative_eq!(123.0, VolumeBuilder::new("123ml").unwrap().as_milliliters(),);
        assert_relative_eq!(
            123.0,
            VolumeBuilder::new("123 ml").unwrap().as_milliliters(),
        );
        assert_relative_eq!(
            123.0,
            VolumeBuilder::new("123 ml").unwrap().as_milliliters(),
        );
    }

    #[test]
    fn pints_from_string() {
        assert_relative_eq!(123.0, VolumeBuilder::new("123p").unwrap().as_pints(),);
        assert_relative_eq!(123.0, VolumeBuilder::new("123 p").unwrap().as_pints(),);
        assert_relative_eq!(123.0, VolumeBuilder::new("123 P").unwrap().as_pints(),);
    }

    #[test]
    fn micrograms_from_string() {
        assert_relative_eq!(123.0, MassBuilder::new("123ug").unwrap().as_micrograms(),);
        assert_relative_eq!(123.0, MassBuilder::new("123 ug").unwrap().as_micrograms(),);
        assert_relative_eq!(123.0, MassBuilder::new("123μg").unwrap().as_micrograms(),);
        assert_relative_eq!(123.0, MassBuilder::new("123 μg").unwrap().as_micrograms(),);
    }

    #[test]
    fn milligrams_from_string() {
        assert_relative_eq!(123.0, MassBuilder::new("123mg").unwrap().as_milligrams(),);
        assert_relative_eq!(123.0, MassBuilder::new("123 mg").unwrap().as_milligrams(),);
    }

    #[test]
    fn carats_from_string() {
        assert_relative_eq!(123.0, MassBuilder::new("123ct").unwrap().as_carats(),);
        assert_relative_eq!(123.0, MassBuilder::new("123 ct").unwrap().as_carats(),);
    }

    #[test]
    fn grams_from_string() {
        assert_relative_eq!(123.0, MassBuilder::new("123g").unwrap().as_grams(),);
        assert_relative_eq!(123.0, MassBuilder::new("123 g").unwrap().as_grams(),);
    }

    #[test]
    fn kilograms_from_string() {
        assert_relative_eq!(123.0, MassBuilder::new("123kg").unwrap().as_kilograms(),);
        assert_relative_eq!(123.0, MassBuilder::new("123 kg").unwrap().as_kilograms(),);
    }

    #[test]
    fn tonnes_from_string() {
        assert_relative_eq!(123.0, MassBuilder::new("123T").unwrap().as_tonnes(),);
        assert_relative_eq!(123.0, MassBuilder::new("123 T").unwrap().as_tonnes(),);
    }

    #[test]
    fn grains_from_string() {
        assert_relative_eq!(123.0, MassBuilder::new("123gr").unwrap().as_grains(),);
        assert_relative_eq!(123.0, MassBuilder::new("123 gr").unwrap().as_grains(),);
    }

    #[test]
    fn pennyweights_from_string() {
        assert_relative_eq!(123.0, MassBuilder::new("123dwt").unwrap().as_pennyweights(),);
        assert_relative_eq!(
            123.0,
            MassBuilder::new("123 dwt").unwrap().as_pennyweights(),
        );
    }

    #[test]
    fn ounces_from_string() {
        assert_relative_eq!(123.0, MassBuilder::new("123oz").unwrap().as_ounces(),);
        assert_relative_eq!(123.0, MassBuilder::new("123 oz").unwrap().as_ounces(),);
    }

    #[test]
    fn pounds_from_string() {
        assert_relative_eq!(123.0, MassBuilder::new("123lbs").unwrap().as_pounds(),);
        assert_relative_eq!(123.0, MassBuilder::new("123 lbs").unwrap().as_pounds(),);
    }
}
