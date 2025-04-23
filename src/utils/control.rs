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
