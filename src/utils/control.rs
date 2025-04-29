// Control method 1, proportional control and a deadzone but no dampining. Unrealistic
pub struct control_1 {
    max_angle: f32,
    deadzone: f32,
}

impl control_1 {
    // Initialise Control mechanism
    pub fn new(max_angle: f32, deadzone: f32) -> Self {
        Self { max_angle, deadzone }
    }

    // Compute the angle adjustments for emg reading
    pub fn compute_target_angle(&self, emg_value: f32) -> f32 {
        if emg_value > self.deadzone {
            emg_value * self.max_angle
        } else if emg_value < -self.deadzone {
            -emg_value * self.max_angle
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;  

    #[test]
    fn test_compute_1_case1() {
        let controller = control_1::new(30.0, 0.1);

        // Test edge case
        assert_eq!(controller.compute_target_angle(0.0), 0.0);
        assert_eq!(controller.compute_target_angle(0.05), 0.0);
        assert_eq!(controller.compute_target_angle(-0.09), 0.0);
        assert_eq!(controller.compute_target_angle(0.1), 0.0);  // edge case
        assert_eq!(controller.compute_target_angle(-0.1), 0.0); // edge case
        assert_eq!(controller.compute_target_angle(0.09999999999999),0.0) // Test floating point percission
    }
}


//
