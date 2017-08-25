use std::any::Any;

pub type ObdValue = Vec<u8>;

/// Convert internal representation into a byte-stream
trait Encode {
    fn encode(&self) -> ObdValue;
}

/// Convert byte-stream into internal representation
trait Decode {
    fn decode(&ObdValue) -> Self;
}

pub struct CoolantTemperature {
    value: u8,
}

impl Encode for CoolantTemperature {
    fn encode(&self) -> ObdValue {
        vec![self.value]
    }
}

impl Decode for CoolantTemperature {
    fn decode(value: &ObdValue) -> Self {
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

pub fn encode_vehicle_speed(speed: u8) -> ObdValue {
    vec![speed]
}

pub fn encode_engine_fuel_rate(fuel_rate: f32) -> ObdValue {
    let scaled = (fuel_rate * 20.0) as u16;
    transform_u16_to_array_of_u8(scaled)
}


pub fn encode(mode: u8, pid: u8, value: &Any) -> Result<ObdValue, &'static str> {
    if mode == 0x01 {
        if pid == 0x05 {
            match value.downcast_ref::<i16>() {
                Some(val) => {
                    return Ok(CoolantTemperature::from(*val).encode())
                }
                None => {
                    return Err("Incorrect type, should be i16")
                }
            }
        }
        else {
            return Err("Could not match PID")
        }
    }
    else {
        return Err("Could not match mode")
    }
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
        let r1: i16 = CoolantTemperature::decode(&vec![0x7B]).into();
        assert_eq!(r1, 83);

        let r2: ObdValue = CoolantTemperature::from(83).encode();
        assert_eq!(r2, vec![0x7B]);

        // Test round-trip
        let temperature = 91;
        let a1 = CoolantTemperature::from(temperature);  // Make the custom object
        let b1 = a1.encode();  // Encode it as a byte-stream
        let c1 = CoolantTemperature::decode(&b1);  // Decode the byte-stream
        let d1: i16 = c1.into();  // Convert it back to an integer
        assert_eq!(d1, temperature);

        // And round-trip the other way
        let encoded_temperature = vec![0xA4];
        let a2 = CoolantTemperature::decode(&encoded_temperature);
        let b2: i16 = a2.into();
        let c2 = CoolantTemperature::from(b2);
        let d2 = c2.encode();
        assert_eq!(d2, encoded_temperature);
    }

    #[test]
    fn test_encode() {
        let mode = 0x01;
        let pid = 0x05;

        let temp: i16 = 83; // Degrees C

        let encoded = encode(mode, pid, &temp);

        assert_eq!(encoded.unwrap(), vec![0x7B]);
    }
}
