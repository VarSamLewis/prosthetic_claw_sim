// Control method 1, proportional control and a deadzone but no dampining. Unrealistic
pub struct Control1 {
    max_angle: f32,
    deadzone: f32,
}

impl Control1 {
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
// Control method 2, proportional control, dampining (using EMA) and a deadzone 
pub struct Control2 {
    max_angle:      f32,
    deadzone:       f32,
    responsiveness: f32,
    last_angle:     f32,
}

impl Control2 {
    /// Create a new controller.
    /// `responsiveness` must be in (0, 1]; 1.0 reproduces the undamped behaviour.
    pub fn new(max_angle: f32, deadzone: f32, responsiveness: f32) -> Self {
        assert!(
            (0.0 < responsiveness) && (responsiveness <= 1.0),
            "responsiveness must be in (0, 1]"
        );
        Self {
            max_angle,
            deadzone,
            responsiveness,
            last_angle: 0.0,
        }
    }

    pub fn compute_target_angle(&mut self, emg_value: f32) -> f32 {
        // 1.  Dead‑zone & proportional term (undamped target)
        let desired = if emg_value >  self.deadzone {
             emg_value * self.max_angle
        } else if emg_value < -self.deadzone {
            -emg_value * self.max_angle
        } else {
            0.0
        };

        // 2.  Single‑knob first‑order damping
        self.last_angle = (1.0 - self.responsiveness) * self.last_angle
                        +  self.responsiveness       * desired;

        self.last_angle
    }
}



#[cfg(test)]
mod tests {

    use super::*;  

    #[test]
    fn test_Compute1_case1() {
        let controller = Control1::new(0.5, 0.1);

        // Test edge case
        assert_eq!(controller.compute_target_angle(0.0), 0.0);
        assert_eq!(controller.compute_target_angle(0.05), 0.0);
        assert_eq!(controller.compute_target_angle(-0.09), 0.0);
        assert_eq!(controller.compute_target_angle(0.1), 0.0);  // edge case
        assert_eq!(controller.compute_target_angle(-0.1), 0.0); // edge case
        assert_eq!(controller.compute_target_angle(0.09999999999999),0.0) // Test floating point percission
    }
    #[test]
    fn test_Compute2_case1() {
        let mut controller = Control2::new(0.5, 0.1, 0.5);

        assert_eq!(controller.compute_target_angle(1.0), 0.25);  
        
    }
}


//
