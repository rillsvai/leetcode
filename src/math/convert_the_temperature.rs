pub fn convert_temperature(celsius: f64) -> Vec<f64> {
    let mut result: Vec<f64> = vec![0f64; 2];

    result[0] = 273.15 + celsius;
    result[1] = 1.8 * celsius + 32.00;

    result
}

pub fn convert_temperature2(celsius: f64) -> Vec<f64> {
    [273.15 + celsius, 1.8 * celsius + 32.00].to_vec()
}
#[cfg(test)]
mod tests {
    use crate::math::convert_the_temperature::{convert_temperature, convert_temperature2};

    #[test]
    fn test_convert_temperature() {
        assert_eq!(convert_temperature(36.5), [309.65000, 97.70000]);
    }

    #[test]
    fn test_convert_temperature2() {
        assert_eq!(convert_temperature2(36.5), [309.65000, 97.70000]);
    }
}
