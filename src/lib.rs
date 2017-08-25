use std::any::Any;

mod util;

/// A byte-stream encoded value
pub type ObdValue = Vec<u8>;
/// A byte-stream encoded query message
pub type ObdQuery = Vec<u8>;
/// A byte-stream encoded response message
pub type ObdResponse = Vec<u8>;

/// Convert internal representation into a byte-stream
pub trait Encode {
    fn encode(&self) -> ObdValue;
}

/// Convert byte-stream into internal representation
pub trait Decode {
    fn decode(&ObdValue) -> Self;
}


/// Coolant temperature in ℃
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
        // TODO Check that this contains exactly one byte
        CoolantTemperature { value: value[0] }
    }
}

impl From<i16> for CoolantTemperature {
    fn from(value: i16) -> Self {
        let bound_value = util::bound(-40, 215, value);
        CoolantTemperature {
            value: (bound_value + 40) as u8,
        }
    }
}

impl Into<i16> for CoolantTemperature {
    fn into(self) -> i16 {
        self.value as i16 - 40
    }
}


/// Vehicle speed in km/h
pub struct VehicleSpeed {
    value: u8,
}

impl Encode for VehicleSpeed {
    fn encode(&self) -> ObdValue {
        vec![self.value]
    }
}

impl Decode for VehicleSpeed {
    fn decode(value: &ObdValue) -> Self {
        // TODO Check that this contains exactly one byte
        VehicleSpeed { value: value[0] }
    }
}

impl From<u8> for VehicleSpeed {
    fn from(value: u8) -> Self {
        VehicleSpeed { value: value }
    }
}

impl Into<u8> for VehicleSpeed {
    fn into(self) -> u8 {
        self.value
    }
}


/// Engine fuel rate in L/h
pub struct EngineFuelRate {
    value: [u8; 2],
}

impl Encode for EngineFuelRate {
    fn encode(&self) -> ObdValue {
        self.value.to_vec()
    }
}

impl Decode for EngineFuelRate {
    fn decode(value: &ObdValue) -> Self {
        // TODO Check that this contains exactly two bytes
        EngineFuelRate {
            value: [value[0], value[1]],
        }
    }
}

impl From<f32> for EngineFuelRate {
    fn from(value: f32) -> Self {
        let bound_value = if value < 0.0 {
            0.0
        } else if value > 3276.75 {
            3276.75
        } else {
            value
        };
        let scaled = (bound_value * 20.0) as u16;
        EngineFuelRate {
            value: util::transform_u16_to_array_of_u8(scaled),
        }
    }
}

impl Into<f32> for EngineFuelRate {
    fn into(self) -> f32 {
        util::transform_array_of_u8_to_u16(self.value) as f32 / 20.
    }
}


/// Encode the value for a given mode and PID
pub fn encode_pid(mode: u8, pid: u8, value: &Any) -> Result<ObdValue, &'static str> {
    if mode == 0x01 {
        if pid == 0x05 {
            match value.downcast_ref::<i16>() {
                Some(val) => Ok(CoolantTemperature::from(*val).encode()),
                None => Err("Incorrect type, should be i16"),
            }
        } else if pid == 0x0D {
            match value.downcast_ref::<u8>() {
                Some(val) => Ok(VehicleSpeed::from(*val).encode()),
                None => Err("Incorrect type, should be u8"),
            }
        } else if pid == 0x5E {
            match value.downcast_ref::<f32>() {
                Some(val) => Ok(EngineFuelRate::from(*val).encode()),
                None => Err("Incorrect type, should be f32"),
            }
        } else {
            Err("Could not match PID")
        }
    } else {
        Err("Could not match mode")
    }
}


/// Given a mode and pid, create the byte-stream to send
pub fn encode_query(mode: u8, pid: u8) -> Result<ObdQuery, &'static str> {
    // TODO check that Mode and PID match and are in range
    Ok(vec![mode, pid])
}

/// Given a byte-stream query, work out what it means
pub fn decode_query(query: &ObdQuery) -> Result<(u8, u8), &'static str> {
    // TODO Check that Mode and PID match and are in range
    Ok((query[0], query[1]))
}

/// Given a query and appropriate data to return, construct the byte-stream
pub fn construct_reponse(query: &ObdQuery, data: &ObdValue) -> Result<ObdResponse, &'static str> {
    let mut response = vec![query[0] + 0x40, query[1]];
    response.extend(data);
    Ok(response)
}

/// Given a byte-stream response, extract the header and encoded value
pub fn parse_reponse(response: &ObdResponse) -> Result<(u8, u8, ObdValue), &'static str> {
    // Todo Check that the mode and PID is sensible
    let mode = response[0] - 0x40;
    let pid = response[1];
    let value = response[2..].to_vec();
    Ok((mode, pid, value))
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
        let a1 = CoolantTemperature::from(temperature); // Make the custom object
        let b1 = a1.encode(); // Encode it as a byte-stream
        let c1 = CoolantTemperature::decode(&b1); // Decode the byte-stream
        let d1: i16 = c1.into(); // Convert it back to an integer
        assert_eq!(d1, temperature);

        // And round-trip the other way
        let encoded_temperature = vec![0xA4];
        let a2 = CoolantTemperature::decode(&encoded_temperature); // Decode the byte-stream
        let b2: i16 = a2.into(); // Convert it to an integer
        let c2 = CoolantTemperature::from(b2); // Make the custom object
        let d2 = c2.encode(); // Encode it as a byte-stream
        assert_eq!(d2, encoded_temperature);

        let r3: ObdValue = CoolantTemperature::from(300).encode();
        assert_eq!(r3, vec![0xFF]);

        let r4: ObdValue = CoolantTemperature::from(-300).encode();
        assert_eq!(r4, vec![0x00]);
    }

    #[test]
    fn test_vehicle_speed() {
        let r1: u8 = VehicleSpeed::decode(&vec![0x7B]).into();
        assert_eq!(r1, 123);

        let r2: ObdValue = VehicleSpeed::from(123).encode();
        assert_eq!(r2, vec![0x7B]);

        // Test round-trip
        let speed = 91;
        let a1 = VehicleSpeed::from(speed); // Make the custom object
        let b1 = a1.encode(); // Encode it as a byte-stream
        let c1 = VehicleSpeed::decode(&b1); // Decode the byte-stream
        let d1: u8 = c1.into(); // Convert it back to an integer
        assert_eq!(d1, speed);

        // And round-trip the other way
        let encoded_speed = vec![0xA4];
        let a2 = VehicleSpeed::decode(&encoded_speed);
        let b2: u8 = a2.into();
        let c2 = VehicleSpeed::from(b2);
        let d2 = c2.encode();
        assert_eq!(d2, encoded_speed);
    }

    #[test]
    fn test_engine_fuel_rate() {
        let r1: f32 = EngineFuelRate::decode(&vec![0x7B, 0x28]).into();
        assert_eq!(r1, 1576.4);

        let r2: ObdValue = EngineFuelRate::from(1576.4).encode();
        assert_eq!(r2, vec![0x7B, 0x28]);

        // Test round-trip
        let rate = 493.7;
        let a1 = EngineFuelRate::from(rate); // Make the custom object
        let b1 = a1.encode(); // Encode it as a byte-stream
        let c1 = EngineFuelRate::decode(&b1); // Decode the byte-stream
        let d1: f32 = c1.into(); // Convert it back to an integer
        assert_eq!(d1, rate);

        // And round-trip the other way
        let encoded_rate = vec![0xA4, 0x01];
        let a2 = EngineFuelRate::decode(&encoded_rate); // Decode the byte-stream
        let b2: f32 = a2.into(); // Convert it to an integer
        let c2 = EngineFuelRate::from(b2); // Make the custom object
        let d2 = c2.encode(); // Encode it as a byte-stream
        assert_eq!(d2, encoded_rate);

        let r3: ObdValue = EngineFuelRate::from(70000.).encode();
        assert_eq!(r3, vec![0xFF, 0xFF]);

        let r4: ObdValue = EngineFuelRate::from(-10.).encode();
        assert_eq!(r4, vec![0x00, 0x00]);
    }

    #[test]
    fn test_encode_pid() {
        let mode = 0x01;
        let pid = 0x05;

        let temp: i16 = 83; // Degrees C

        let encoded = encode_pid(mode, pid, &temp);

        assert_eq!(encoded.unwrap(), vec![0x7B]);
    }

    #[test]
    fn test_roundtrip() {
        let mode = 0x01;
        let pid = 0x0D;
        let real_speed: u8 = 74;

        // At the local end, ask the question
        let query = encode_query(mode, pid).expect("Encoding failed");
        // At the remote end, receive the query and work out what it means
        let (remote_mode, remote_pid) = decode_query(&query).expect("Decoding failed");
        // Ask the system for the real value
        let remote_value =
            encode_pid(remote_mode, remote_pid, &real_speed).expect("Encoding failed");
        // Construct the message to send back
        let response = construct_reponse(&query, &remote_value).expect("Decoding failed");
        // At the local end again, unpack the response
        let (returned_mode, returned_pid, returned_value) =
            parse_reponse(&response).expect("Parse failed");
        // and decode the value returned
        let returned_speed = VehicleSpeed::decode(&returned_value);

        assert_eq!(mode, returned_mode);
        assert_eq!(pid, returned_pid);
        assert_eq!(real_speed, returned_speed.into());
    }
}
