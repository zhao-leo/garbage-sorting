use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use rppal::gpio::{Gpio, InputPin, Level, Trigger};
use tauri::Manager;
use tauri::Emitter;
pub struct GpioManager {
    input_pin: Option<InputPin>,
    running: Arc<Mutex<bool>>,
}

impl GpioManager {
    pub fn new() -> Self {
        GpioManager {
            input_pin: None,
            running: Arc::new(Mutex::new(false)),
        }
    }

    pub fn initialize(&mut self, pin_number: u8) -> Result<(), String> {
        match Gpio::new() {
            Ok(gpio) => {
                match gpio.get(pin_number) {
                    Ok(pin) => {
                        let input_pin = pin.into_input_pullup();
                        self.input_pin = Some(input_pin);
                        Ok(())
                    },
                    Err(e) => Err(format!("Failed to get GPIO pin {}: {}", pin_number, e)),
                }
            },
            Err(e) => Err(format!("Failed to initialize GPIO: {}", e)),
        }
    }

    pub fn start_monitoring(&mut self, app_handle: tauri::AppHandle) -> Result<(), String> {
        if let Some(mut pin) = self.input_pin.take() {
            match pin.set_interrupt(Trigger::FallingEdge, Level::Low) {
                Ok(_) => {
                    *self.running.lock().unwrap() = true;
                    let running_clone = Arc::clone(&self.running);
                    
                    thread::spawn(move || {
                        while *running_clone.lock().unwrap() {
                            if let Ok(true) = pin.poll_interrupt(true, Some(Duration::from_millis(100))) {
                                // Emit Tauri event when falling edge is detected
                                app_handle.emit("gpio-falling-edge", pin.pin()).unwrap_or_else(|e| {
                                    eprintln!("Failed to emit event: {}", e);
                                });
                            }
                        }
                    });
                    
                    Ok(())
                },
                Err(e) => Err(format!("Failed to set interrupt: {}", e)),
            }
        } else {
            Err("GPIO pin not initialized".to_string())
        }
    }

    // pub fn stop_monitoring(&self) {
    //     if let Ok(mut running) = self.running.lock() {
    //         *running = false;
    //     }
    // }
}

// #[tauri::command]
// pub fn setup_gpio_monitoring(app_handle: tauri::AppHandle, pin: u8) -> Result<String, String> {
//     let mut manager = GpioManager::new();
//     manager.initialize(pin)?;
//     manager.start_monitoring(app_handle)?;
    
//     Ok(format!("Started monitoring GPIO pin {}", pin))
// }

// // Add this function to your main.rs to register the command
// pub fn register_gpio_commands(app: &mut tauri::App) {
//     // Store the GpioManager in app state if needed for longer lifecycle
//     app.manage(Arc::new(Mutex::new(GpioManager::new())));
// }