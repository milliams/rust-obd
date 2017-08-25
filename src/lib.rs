pub type ObdValue = Vec<u8>;

pub struct CoolantTemperature {
    value: u8,
}

impl From<ObdValue> for CoolantTemperature {
    fn from(value: ObdValue) -> Self {
        CoolantTemperature{value: value[0]}
    }
}

impl From<i16> for CoolantTemperature {
    fn from(value: i16) -> Self {
        CoolantTemperature{value: (value + 40) as u8}
    }
}

impl Into<i16> for CoolantTemperature {
    fn into(self) -> i16 {
        self.value as i16 - 40
    }
}

impl Into<ObdValue> for CoolantTemperature {
    fn into(self) -> ObdValue {
        vec![self.value]
    }
}

pub fn encode_vehicle_speed(speed: u8) -> ObdValue {
    vec![speed]
}

pub fn encode_engine_fuel_rate(fuel_rate: f32) -> ObdValue {
    let scaled = (fuel_rate * 20.0) as u16;
    transform_u16_to_array_of_u8(scaled)
}


pub fn encode(mode: u8, pid: u8, value: i16) -> ObdValue {
    vec![0x7B]  // TODO
}


fn transform_u16_to_array_of_u8(x: u16) -> Vec<u8> {
    let b1: u8 = ((x >> 8) & 0xff) as u8;
    let b2: u8 = ((x >> 0) & 0xff) as u8;
    return vec![b1, b2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coolant_temperature() {
        let r1: i16 = CoolantTemperature::from(vec![0x7B]).into();
        assert_eq!(r1, 83);

        let r2: ObdValue = CoolantTemperature::from(83).into();
        assert_eq!(r2, vec![0x7B]);
    }

    #[test]
    fn test_encode() {
        let mode = 0x01;
        let pid = 0x05;

        let temp = 83; // Degrees C

        let encoded = encode(mode, pid, temp);

        assert_eq!(encoded, vec![0x7B]);
    }
}
