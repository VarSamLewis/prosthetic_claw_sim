use rapier2d::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use std::path::Path;
use csv::Writer;
use std::fs::File;


#[derive(Debug, Deserialize)]
struct EMGSample {
    time: f32,
    emg: f32,
}

#[derive(Debug, Serialize)]
struct EMGOutput {
    time: f32,
    emg: f32,
    left: f32,
    right: f32,
}

struct EMGReader {
    samples: Vec<EMGSample>,
    index: usize,
}

struct ClawController {
    kp: f32,
    target_angle: f32,
}

impl ClawController {
    fn new(kp: f32) -> Self {
        Self { kp, target_angle: 0.0 }
    }

    fn set_target(&mut self, angle: f32) {
        self.target_angle = angle;
    }

    fn compute_torque(&self, current_angle: f32) -> f32 {
        self.kp * (self.target_angle - current_angle)
    }
}

impl EMGReader {
    fn from_csv<P: AsRef<Path>>(path: P) -> Self {
        let mut rdr = csv::Reader::from_path(path).expect("Cannot open CSV file");
        let mut samples = Vec::new();
        for result in rdr.deserialize() {
            let record: EMGSample = result.expect("Error reading row");
            samples.push(record);
        }
        Self { samples, index: 0 }
    }

    fn next_value(&mut self) -> f32 {
        if self.index < self.samples.len() {
            let value = self.samples[self.index].emg;
            self.index += 1;
            value
        } else {
            0.0  // Default EMG value after end of file
        }
    }
}

fn write_emg_to_csv(path: &str, data: &[EMGOutput]) {
    let file = File::create(path).expect("Cannot create file");
    let mut wtr = Writer::from_writer(file);

    for row in data {
        wtr.serialize(row).expect("Failed to write row");
    }

    wtr.flush().expect("Failed to flush writer");
}

fn main() {
    // Set up the physics world
    let gravity = vector![0.0, 0.0]; // No gravity, just joints
    let mut physics_pipeline = PhysicsPipeline::new();
    let mut bodies = RigidBodySet::new();
    let mut colliders = ColliderSet::new();
    let mut joints = ImpulseJointSet::new();
    let integration_parameters = IntegrationParameters::default();
    let mut island_manager = IslandManager::new();
    let mut broad_phase = BroadPhase::new();
    let mut narrow_phase = NarrowPhase::new();
    let mut ccd_solver = CCDSolver::new();
    let mut query_pipeline = QueryPipeline::new();

    // Create base (static)
    let base_body = RigidBodyBuilder::fixed()
        .translation(vector![0.0, 0.0])
        .build();
    let base_handle = bodies.insert(base_body);

    // Create left claw (dynamic)
    let left_claw_body = RigidBodyBuilder::dynamic()
        .translation(vector![-0.5, 0.0])
        .build();
    let left_claw_handle = bodies.insert(left_claw_body);
    let left_claw_collider = ColliderBuilder::cuboid(0.2, 0.05).build();
    colliders.insert_with_parent(left_claw_collider, left_claw_handle, &mut bodies);

    // Create right claw (dynamic)
    let right_claw_body = RigidBodyBuilder::dynamic()
        .translation(vector![0.5, 0.0])
        .build();
    let right_claw_handle = bodies.insert(right_claw_body);
    let right_claw_collider = ColliderBuilder::cuboid(0.2, 0.05).build();
    colliders.insert_with_parent(right_claw_collider, right_claw_handle, &mut bodies);

    // Attach left claw to base with revolute joint
    let left_joint = RevoluteJointBuilder::new()
        .local_anchor1(point![0.5, 0.0])
        .local_anchor2(point![0.0, 0.0])
        .build();
    joints.insert(base_handle, left_claw_handle, left_joint, true); /*Trying true to see if that fixes things*/

    // Attach right claw to base with revolute joint
    let right_joint = RevoluteJointBuilder::new()
        .local_anchor1(point![-0.5, 0.0])
        .local_anchor2(point![0.0, 0.0])
        .build();
    joints.insert(base_handle, right_claw_handle, right_joint, true); /*Trying true to see if that fixes things*/
    
    // Controllers for each claw
    let mut left_ctrl = ClawController::new(10.0);
    let mut right_ctrl = ClawController::new(10.0);

    let mut emg_reader = EMGReader::from_csv("emg_data\\emg.csv");
    let mut simulated_data = Vec::new();

    // Simulate EMG pattern: open 0 - 199, close 200 - 399, repeat
    for step in 0..600 {
         // Then inside your loop:
        let emg_sim_value = emg_reader.next_value();

        //let emg_sim_value = if (step / 200) % 2 == 0 { 0.05 } else { 0.3 };
        
        // Simple threshold classifier
        let target_angle = if emg_sim_value > 0.2 { 0.5 } else { -0.5 };
        left_ctrl.set_target(target_angle);
        right_ctrl.set_target(-target_angle); // Mirror angle

        // Compute and apply torques
        if let Some(left_body) = bodies.get_mut(left_claw_handle) {
            let angle = left_body.position().rotation.angle();
            let torque = left_ctrl.compute_torque(angle);
            left_body.apply_torque_impulse(torque, true);
        }

        if let Some(right_body) = bodies.get_mut(right_claw_handle) {
            let angle = right_body.position().rotation.angle();
            let torque = right_ctrl.compute_torque(angle);
            right_body.apply_torque_impulse(torque, true);
        }

        // Simple threshold classifier
        let target_angle = if emg_sim_value > 0.2 { 0.5 } else { -0.5 };
        left_ctrl.set_target(target_angle);
        right_ctrl.set_target(-target_angle); // Mirror angle

        // Compute and apply torques
        if let Some(left_body) = bodies.get_mut(left_claw_handle) {
            let angle = left_body.position().rotation.angle();
            let torque = left_ctrl.compute_torque(angle);
            left_body.apply_torque_impulse(torque, true);
        }

        if let Some(right_body) = bodies.get_mut(right_claw_handle) {
            let angle = right_body.position().rotation.angle();
            let torque = right_ctrl.compute_torque(angle);
            right_body.apply_torque_impulse(torque, true);
        }
        

    // Simulate
    /*
    for step in 0..1000 {
        // --- Control Section ---
        // (Simulate EMG input: every 200 steps, toggle open/close)
        let closing = (step / 200) % 2 == 1;

        if closing {
            if let Some(left) = bodies.get_mut(left_claw_handle) {
                left.apply_torque_impulse(10.0, true);
            }
            if let Some(right) = bodies.get_mut(right_claw_handle) {
                right.apply_torque_impulse(-10.0, true);
            }
        } else {
            if let Some(left) = bodies.get_mut(left_claw_handle) {
                left.apply_torque_impulse(-10.0, true);
            }
            if let Some(right) = bodies.get_mut(right_claw_handle) {
                right.apply_torque_impulse(10.0, true);
            }
        }
        */

          let mut multibody_joint_set = MultibodyJointSet::new(); // Add this

    // Step the simulation
        physics_pipeline.step(
            &gravity,
            &integration_parameters,
            &mut island_manager,
            &mut broad_phase,
            &mut narrow_phase,
            &mut bodies,
            &mut colliders,
            &mut joints,
            &mut multibody_joint_set, // Pass the multibody joint set
            &mut ccd_solver,
            Some(&mut query_pipeline), // Pass the query pipeline
            &(),
            &(),
        );
        // Log status
        let left_angle = bodies[left_claw_handle].position().rotation.angle();
        let right_angle = bodies[right_claw_handle].position().rotation.angle();
        println!(
            "Step {:>3} | EMG: {:.2} | Left: {:.2} rad | Right: {:.2} rad",
            step, emg_sim_value, left_angle, right_angle
        );

        simulated_data.push(EMGOutput {
            time: step as f32 * 0.01, // Assuming each step is 10ms
            emg: emg_sim_value,
            left: left_angle,
            right: right_angle,
        });
    }
    write_emg_to_csv("emg_data\\output.csv", &simulated_data);
}
