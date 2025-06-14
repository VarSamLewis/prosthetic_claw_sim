use ndarray::Array1;

fn compute_jerk(angles: &[f32], dt: f32) -> Vec<f32> {
    let n = angles.len();
    let mut jerk = vec![0.0; n];

    for i in 1..n-2 {
        jerk[i] = (angles[i+2] - 3.0 * angles[i+1] + 3.0 * angles[i] - angles[i-1]) / dt.powi(3);
    }

    jerk
}

fn mean_angular_velocity(angles: &[f32], dt: f32) -> f32 {
    let velocities: Vec<f32> = angles.windows(2)
        .map(|w| (w[1] - w[0]) / dt)
        .collect();

    let sum: f32 = velocities.iter().sum();
    sum / velocities.len() as f32
}

use statrs::statistics::Statistics;

fn std_dev_angular_velocity(angles: &[f32], dt: f32) -> f32 {
    let velocities: Vec<f64> = angles.windows(2)
        .map(|w| ((w[1] - w[0]) / dt) as f64)
        .collect();

    velocities.std_dev() as f32
}

fn rms_torque(values: &[f32]) -> f32 {
    let sum_of_squares: f32 = values.iter().map(|v| v * v).sum();
    (sum_of_squares / values.len() as f32).sqrt()
}


#[cfg(test)]
mod tests {

    use super::*;
    use float_cmp::approx_eq;

    #[test]
    fn test_compute_jerk_case1() {
        // Jerk should be zero for uniform acceleration
        let angles = vec![0.0, 1.0, 4.0, 9.0, 16.0]; // Quadratic: a = 2, jerk = 0
        let dt = 1.0;
        let result = compute_jerk(&angles, dt);
        // Skip edges, check inner value
        assert!(approx_eq!(f32, result[2], 0.0, ulps = 5));
    }
     #[test]
    fn test_mean_angular_velocity_case1() {
        let angles = vec![0.0, 1.0, 2.0, 3.0]; // Constant velocity
        let dt = 1.0;
        let result = mean_angular_velocity(&angles, dt);
        assert!(approx_eq!(f32, result, 1.0, ulps = 2));
    }

    #[test]
    fn test_std_dev_angular_velocity_case1() {
        let angles = vec![0.0, 1.0, 2.0, 3.0]; // Constant velocity = 1.0
        let dt = 1.0;
        let result = std_dev_angular_velocity(&angles, dt);
        assert!(approx_eq!(f32, result, 0.0, ulps = 2));
    }

    #[test]
    fn test_rms_torque_case1() {
        let torques = vec![1.0, -1.0, 1.0, -1.0]; // RMS should be 1.0
        let result = rms_torque(&torques);
        assert!(approx_eq!(f32, result, 1.0, ulps = 2));
    }
}
