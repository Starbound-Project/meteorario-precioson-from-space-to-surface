use eframe::egui;
use egui::Color32;
use std::sync::{Arc, Mutex};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    
    // Create state
    let state = Arc::new(Mutex::new(MeteorState {
        diameter: 10.0,
        velocity: 17.0,
        angle: 45.0,
        altitude: 20.0,
        density: 3000.0,
        impact_x: 0.0,
        impact_z: 0.0,
        show_impact: false,
        animation_time: 0.0,
    }));
    
    // Run UI
    eframe::run_native(
        "Meteor Impact Simulator",
        options,
        Box::new(|_cc| Box::new(MeteorApp::new(state))),
    )
}

struct MeteorState {
    diameter: f32,
    velocity: f32,
    angle: f32,
    altitude: f32,
    density: f32,
    impact_x: f32,
    impact_z: f32,
    show_impact: bool,
    animation_time: f32,
}

struct MeteorApp {
    shared_state: Arc<Mutex<MeteorState>>,
}

impl MeteorApp {
    fn new(shared_state: Arc<Mutex<MeteorState>>) -> Self {
        Self { shared_state }
    }
}

impl eframe::App for MeteorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut state = self.shared_state.lock().unwrap();
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Meteor Impact Simulator");
            
            // Create a horizontal layout for controls and visualization
            ui.horizontal(|ui| {
                // Left side: Controls
                ui.vertical(|ui| {
                    ui.group(|ui| {
                        ui.heading("Meteor Parameters");
                        
                        ui.add(egui::Slider::new(&mut state.diameter, 1.0..=1000.0)
                            .text("Diameter (meters)"));
                        
                        ui.add(egui::Slider::new(&mut state.velocity, 11.0..=72.0)
                            .text("Velocity (km/s)"));
                        
                        ui.add(egui::Slider::new(&mut state.angle, 5.0..=90.0)
                            .text("Entry Angle (degrees)"));
                        
                        ui.add(egui::Slider::new(&mut state.altitude, 0.0..=100.0)
                            .text("Burst Altitude (km)"));
                    });
                    
                    ui.group(|ui| {
                        ui.heading("Impact Location");
                        ui.horizontal(|ui| {
                            ui.label("X:");
                            ui.add(egui::Slider::new(&mut state.impact_x, -5.0..=5.0));
                        });
                        ui.horizontal(|ui| {
                            ui.label("Z:");
                            ui.add(egui::Slider::new(&mut state.impact_z, -5.0..=5.0));
                        });
                    });
                    
                    ui.group(|ui| {
                        ui.heading("Simulation Controls");
                        ui.horizontal(|ui| {
                            if ui.button("Simulate Impact").clicked() {
                                state.show_impact = true;
                                state.animation_time = 0.0;
                            }
                            
                            if ui.button("Reset").clicked() {
                                state.show_impact = false;
                                state.animation_time = 0.0;
                            }
                        });
                    });
                });
                
                // Right side: Visualization and results
                ui.vertical(|ui| {
                    // Calculate impact effects
                    let volume = (4.0/3.0) * std::f32::consts::PI * (state.diameter/2.0).powi(3);
                    let mass = volume * state.density / 1000.0; // tons
                    let energy_joules = 0.5 * mass * 1000.0 * state.velocity.powi(2) * 1000.0 * 1000.0;
                    let energy_kilotons = energy_joules / 4.184e12;
                    
                    // Adjust for angle and altitude
                    let angle_factor = (state.angle.to_radians().sin());
                    let altitude_factor = if state.altitude <= 0.0 { 1.0 } else { 1.0 / (1.0 + state.altitude / 5.0) };
                    let effective_energy = energy_kilotons * angle_factor * altitude_factor;
                    
                    // Calculate crater size (if it reaches the ground)
                    let crater_diameter = if state.altitude <= 5.0 {
                        0.07 * effective_energy.powf(0.33)
                    } else {
                        0.0
                    };
                    
                    // Calculate damage radius
                    let damage_radius = 0.5 * effective_energy.powf(0.33) * 1000.0; // in meters
                    
                    // Draw a simple top-down view
                    ui.group(|ui| {
                        ui.heading("Impact Preview");
                        let rect = ui.available_rect_before_wrap();
                        let painter = ui.painter();
                        
                        let center = rect.center();
                        let radius = rect.width().min(rect.height()) * 0.4;
                        
                        // Draw water
                        painter.circle_filled(center, radius, Color32::from_rgb(0, 100, 200));
                        
                        // Draw island
                        painter.circle_filled(center, radius * 0.8, Color32::from_rgb(0, 150, 50));
                        
                        // Calculate impact position on the 2D preview
                        let impact_x = center.x + state.impact_x * radius * 0.8 / 5.0;
                        let impact_y = center.y + state.impact_z * radius * 0.8 / 5.0;
                        
                        // Draw impact point
                        painter.circle_stroke(
                            egui::pos2(impact_x, impact_y),
                            5.0,
                            egui::Stroke::new(2.0, Color32::RED)
                        );
                        
                        // Draw crater if applicable
                        if state.show_impact && state.animation_time > state.altitude * 0.1 / state.velocity && crater_diameter > 0.0 {
                            let crater_radius = crater_diameter * radius * 0.8 / 10.0; // Scale to view
                            
                            painter.circle_filled(
                                egui::pos2(impact_x, impact_y),
                                crater_radius,
                                Color32::from_rgb(100, 50, 0)
                            );
                        }
                        
                        // Draw meteor during animation
                        if state.show_impact {
                            let height_percent = (10.0 - state.animation_time * state.velocity * 0.1) / 10.0;
                            if height_percent > 0.0 {
                                let meteor_y = impact_y - height_percent * radius; // Start above impact point
                                let meteor_size = state.diameter / 20.0 * 5.0;
                                
                                painter.circle_filled(
                                    egui::pos2(impact_x, meteor_y),
                                    meteor_size,
                                    Color32::from_rgb(150, 75, 0)
                                );
                            }
                        }
                        
                        ui.allocate_rect(rect, egui::Sense::hover());
                    });
                    
                    // Results panel
                    ui.group(|ui| {
                        ui.heading("Impact Results");
                        
                        ui.label(format!("Mass: {:.1} tons", mass));
                        
                        // Format energy based on size
                        let energy_text = if energy_kilotons > 1000.0 {
                            format!("Impact Energy: {:.1} Megatons TNT", energy_kilotons / 1000.0)
                        } else {
                            format!("Impact Energy: {:.1} kilotons TNT", energy_kilotons)
                        };
                        ui.label(energy_text);
                        
                        let effective_text = if effective_energy > 1000.0 {
                            format!("Effective Energy: {:.1} Megatons TNT", effective_energy / 1000.0)
                        } else {
                            format!("Effective Energy: {:.1} kilotons TNT", effective_energy)
                        };
                        ui.label(effective_text);
                        
                        if crater_diameter > 0.0 {
                            ui.label(format!("Crater Diameter: {:.1} km", crater_diameter));
                        } else {
                            ui.label("No crater (airburst)");
                        }
                        
                        ui.label(format!("Damage Radius: {:.1} km", damage_radius / 1000.0));
                        
                        // Compare to historical events
                        ui.separator();
                        ui.heading("Impact Assessment");
                        
                        if effective_energy > 10000.0 {
                            ui.colored_label(Color32::RED, "DINOSAUR KILLER! (Larger than Chicxulub impact)");
                        } else if effective_energy > 440.0 {
                            ui.colored_label(Color32::RED, "LARGER THAN CHELYABINSK METEOR!");
                        } else if effective_energy > 15.0 {
                            ui.colored_label(Color32::GOLD, "Similar to Hiroshima bomb!");
                        } else if state.altitude > 5.0 || effective_energy < 0.1 {
                            ui.colored_label(Color32::GREEN, "Meteor just got burnt before reaching the ground. People are safe!");
                        }
                    });
                });
            });
        });
        
        // Request continuous updates if animation is running
        if state.show_impact {
            state.animation_time += 0.016; // ~60fps
            ctx.request_repaint();
        }
    }
}

