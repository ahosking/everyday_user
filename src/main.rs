use eframe::egui;
use egui::{Color32, RichText, FontId};
use std::process::Command;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 400.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Everyday User",
        options,
        Box::new(|_cc| Box::new(EverydayUserApp::default())),
    )
}

#[derive(Default)]
struct EverydayUserApp {
    counter: i32,
    show_help: bool,
    computer_name: String,
    os_type: OsType,
    system_memory: Option<String>,
    video_adapter: Option<String>,
}

#[derive(Default, PartialEq, Clone)]
enum OsType {
    #[default]
    Unknown,
    Windows,
    MacOS,
    Linux,
}

impl EverydayUserApp {
    fn render_ascii_banner(&self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            // Define colors for the banner
            let primary_color = Color32::from_rgb(52, 152, 219); // Blue
            let secondary_color = Color32::from_rgb(231, 76, 60); // Red
            
            // ASCII art banner
            let banner_lines = vec![
                "███████╗██╗   ██╗███████╗██████╗ ██╗   ██╗██████╗  █████╗ ██╗   ██╗",
                "██╔════╝██║   ██║██╔════╝██╔══██╗╚██╗ ██╔╝██╔══██╗██╔══██╗╚██╗ ██╔╝",
                "█████╗  ██║   ██║█████╗  ██████╔╝ ╚████╔╝ ██║  ██║███████║ ╚████╔╝ ",
                "██╔══╝  ╚██╗ ██╔╝██╔══╝  ██╔══██╗  ╚██╔╝  ██║  ██║██╔══██║  ╚██╔╝  ",
                "███████╗ ╚████╔╝ ███████╗██║  ██║   ██║   ██████╔╝██║  ██║   ██║   ",
                "╚══════╝  ╚═══╝  ╚══════╝╚═╝  ╚═╝   ╚═╝   ╚═════╝ ╚═╝  ╚═╝   ╚═╝   ",
                "                                                                    ",
                "                         ██╗   ██╗███████╗███████╗██████╗           ",
                "                         ██║   ██║██╔════╝██╔════╝██╔══██╗          ",
                "                         ██║   ██║███████╗█████╗  ██████╔╝          ",
                "                         ██║   ██║╚════██║██╔══╝  ██╔══██╗          ",
                "                         ╚██████╔╝███████║███████╗██║  ██║          ",
                "                          ╚═════╝ ╚══════╝╚══════╝╚═╝  ╚═╝          ",
            ];
            
            // Create a monospace font for the ASCII art
            let font = FontId::monospace(14.0);
            
            // Alternate colors for each line
            for (i, line) in banner_lines.iter().enumerate() {
                let color = if i % 2 == 0 { primary_color } else { secondary_color };
                ui.label(RichText::new(*line).font(font.clone()).color(color));
            }
            
            // Add some space after the banner
            ui.add_space(10.0);
        });
    }

    fn get_computer_name(&mut self) {
        if !self.computer_name.is_empty() {
            return;
        }

        // Detect OS and get computer name
        if cfg!(target_os = "windows") {
            self.os_type = OsType::Windows;
            if let Ok(output) = Command::new("hostname").output() {
                if let Ok(name) = String::from_utf8(output.stdout) {
                    self.computer_name = name.trim().to_string();
                }
            }
        } else if cfg!(target_os = "macos") {
            self.os_type = OsType::MacOS;
            if let Ok(output) = Command::new("scutil").args(["--get", "ComputerName"]).output() {
                if let Ok(name) = String::from_utf8(output.stdout) {
                    self.computer_name = name.trim().to_string();
                }
            }
        } else if cfg!(target_os = "linux") {
            self.os_type = OsType::Linux;
            if let Ok(output) = Command::new("hostname").output() {
                if let Ok(name) = String::from_utf8(output.stdout) {
                    self.computer_name = name.trim().to_string();
                }
            }
        } else {
            self.os_type = OsType::Unknown;
            self.computer_name = "Unknown".to_string();
        }
    }

    fn get_system_memory(&mut self) -> Option<String> {
        if self.system_memory.is_none() {
            // Get system memory based on OS
            if cfg!(target_os = "windows") {
                // Windows - use PowerShell
                if let Ok(output) = Command::new("powershell")
                    .args(["-Command", "Get-CimInstance Win32_ComputerSystem | Select-Object -ExpandProperty TotalPhysicalMemory"])
                    .output() 
                {
                    if let Ok(mem_bytes_str) = String::from_utf8(output.stdout) {
                        if let Ok(mem_bytes) = mem_bytes_str.trim().parse::<u64>() {
                            let mem_gb = mem_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
                            self.system_memory = Some(format!("{:.2} GB", mem_gb));
                        }
                    }
                }
            } else if cfg!(target_os = "macos") {
                // macOS - use sysctl
                if let Ok(output) = Command::new("sysctl")
                    .args(["-n", "hw.memsize"])
                    .output() 
                {
                    if let Ok(mem_bytes_str) = String::from_utf8(output.stdout) {
                        if let Ok(mem_bytes) = mem_bytes_str.trim().parse::<u64>() {
                            let mem_gb = mem_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
                            self.system_memory = Some(format!("{:.2} GB", mem_gb));
                        }
                    }
                }
            } else if cfg!(target_os = "linux") {
                // Linux - read from /proc/meminfo
                if let Ok(mem_info) = std::fs::read_to_string("/proc/meminfo") {
                    for line in mem_info.lines() {
                        if line.starts_with("MemTotal:") {
                            // Format: "MemTotal:       16333852 kB"
                            let parts: Vec<&str> = line.split_whitespace().collect();
                            if parts.len() >= 2 {
                                if let Ok(mem_kb) = parts[1].parse::<u64>() {
                                    let mem_gb = mem_kb as f64 / (1024.0 * 1024.0);
                                    self.system_memory = Some(format!("{:.2} GB", mem_gb));
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            
            // If all methods failed, provide a fallback
            if self.system_memory.is_none() {
                self.system_memory = Some("Unknown".to_string());
            }
        }
        
        self.system_memory.clone()
    }
    
    fn get_video_adapter(&mut self) -> Option<String> {
        if self.video_adapter.is_none() {
            // Get video adapter info based on OS
            if cfg!(target_os = "windows") {
                // Windows - use PowerShell
                if let Ok(output) = Command::new("powershell")
                    .args(["-Command", "Get-CimInstance Win32_VideoController | Select-Object -ExpandProperty Name"])
                    .output() 
                {
                    if let Ok(adapter_name) = String::from_utf8(output.stdout) {
                        let adapter_name = adapter_name.trim();
                        if !adapter_name.is_empty() {
                            self.video_adapter = Some(adapter_name.to_string());
                        }
                    }
                }
            } else if cfg!(target_os = "macos") {
                // macOS - use system_profiler
                if let Ok(output) = Command::new("system_profiler")
                    .args(["SPDisplaysDataType"])
                    .output() 
                {
                    if let Ok(info) = String::from_utf8(output.stdout) {
                        // Parse the output to find the graphics card model
                        for line in info.lines() {
                            if line.contains("Chipset Model:") {
                                if let Some(model) = line.split(':').nth(1) {
                                    self.video_adapter = Some(model.trim().to_string());
                                    break;
                                }
                            }
                        }
                    }
                }
            } else if cfg!(target_os = "linux") {
                // Linux - use lspci
                if let Ok(output) = Command::new("lspci")
                    .args(["-v"])
                    .output() 
                {
                    if let Ok(pci_info) = String::from_utf8(output.stdout) {
                        // Find VGA or 3D controller
                        for line in pci_info.lines() {
                            if line.contains("VGA compatible controller") || line.contains("3D controller") {
                                // Extract the adapter name (everything after ":")
                                if let Some(adapter_info) = line.split(':').nth(2) {
                                    self.video_adapter = Some(adapter_info.trim().to_string());
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            
            // If all methods failed, provide a fallback
            if self.video_adapter.is_none() {
                self.video_adapter = Some("Unknown".to_string());
            }
        }
        
        self.video_adapter.clone()
    }

    fn get_os_logo_and_color(&self) -> (Vec<&'static str>, Color32) {
        match self.os_type {
            OsType::Windows => (
                vec![
                    "   ████████████   ",
                    " ██            ██ ",
                    "██   ██    ██   ██",
                    "██   ██    ██   ██",
                    "██              ██",
                    "██              ██",
                    "██   ██    ██   ██",
                    "██   ██    ██   ██",
                    " ██            ██ ",
                    "   ████████████   ",
                ],
                Color32::from_rgb(0, 120, 215) // Windows blue
            ),
            OsType::MacOS => (
                vec![
                    "     ████████     ",
                    "   ██        ██   ",
                    " ██            ██ ",
                    "██     ████     ██",
                    "██   ████████   ██",
                    "██  ██████████  ██",
                    "██  ██████████  ██",
                    "██    ██████    ██",
                    " ██            ██ ",
                    "   ██        ██   ",
                    "     ████████     ",
                ],
                Color32::from_rgb(128, 128, 128) // Apple gray
            ),
            OsType::Linux => (
                vec![
                    "     ████████     ",
                    "   ██        ██   ",
                    "  ██  ██  ██  ██  ",
                    "  ██          ██  ",
                    "  ██  ██████  ██  ",
                    "  ██  ██  ██  ██  ",
                    "  ██  ██  ██  ██  ",
                    "   ██        ██   ",
                    "     ████████     ",
                ],
                Color32::from_rgb(252, 175, 62) // Tux yellow
            ),
            OsType::Unknown => (
                vec![
                    "     ????????     ",
                    "   ??        ??   ",
                    "  ??          ??  ",
                    "  ??  ??  ??  ??  ",
                    "  ??          ??  ",
                    "  ??  ??  ??  ??  ",
                    "  ??          ??  ",
                    "   ??        ??   ",
                    "     ????????     ",
                ],
                Color32::GRAY
            ),
        }
    }

    fn show_help_window(&mut self, ctx: &egui::Context) {
        if self.show_help {
            // Get computer name if not already fetched
            self.get_computer_name();
            let system_memory = self.get_system_memory();
            let video_adapter = self.get_video_adapter();
            
            // Extract all the data we need before the closure
            let computer_name = self.computer_name.clone();
            let os_type = self.os_type.clone();
            let os_name = match os_type {
                OsType::Windows => "Windows",
                OsType::MacOS => "macOS",
                OsType::Linux => "Linux",
                OsType::Unknown => "Unknown",
            };
            
            // Get OS logo and color
            let (logo, color) = self.get_os_logo_and_color();
            
            // Create a local copy of the window state
            let mut show_window = self.show_help;
            
            egui::Window::new("Help & System Info")
                .open(&mut show_window)
                .resizable(false)
                .min_width(900.0)
                .min_height(500.0)
                .show(ctx, |ui| {
                    ui.heading("System Information");
                    ui.add_space(10.0);
                    
                    ui.horizontal(|ui| {
                        // Left column with fixed width for system info
                        ui.with_layout(egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true), |ui| {
                            // Set a fixed width for the info panel
                            ui.set_width(500.0);
                            
                            // Add a title for this section
                            ui.strong("SYSTEM INFORMATION");
                            ui.add_space(12.0);
                            
                            // Computer name
                            ui.horizontal(|ui| {
                                ui.strong("Computer Name:");
                                ui.add_space(5.0);
                                ui.label(&computer_name);
                            });
                            
                            ui.add_space(10.0);
                            
                            // Operating system
                            ui.horizontal(|ui| {
                                ui.strong("Operating System:");
                                ui.add_space(5.0);
                                ui.label(os_name);
                            });
                            
                            ui.add_space(10.0);
                            
                            // System memory
                            ui.horizontal(|ui| {
                                ui.strong("System Memory:");
                                ui.add_space(5.0);
                                ui.label(system_memory.as_deref().unwrap_or("Unknown"));
                            });
                            
                            ui.add_space(10.0);
                            
                            // Graphics adapter
                            ui.horizontal(|ui| {
                                ui.strong("Graphics Adapter:");
                                ui.add_space(5.0);
                                ui.label(video_adapter.as_deref().unwrap_or("Unknown"));
                            });
                        });
                        
                        ui.add_space(100.0);
                        
                        // Right column with OS logo
                        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                            // Set fixed width for logo area
                            ui.set_width(300.0);
                            
                            // OS Logo - use OS-specific font sizes
                            let font_size = match os_type {
                                OsType::Windows => 9.0,
                                OsType::Linux => 10.0,
                                OsType::MacOS => 9.0,
                                OsType::Unknown => 9.0,
                            };
                            
                            let font = FontId::monospace(font_size);
                            
                            // Add some space at the top for better vertical alignment
                            ui.add_space(20.0);
                            
                            for line in &logo {
                                ui.label(RichText::new(*line).font(font.clone()).color(color));
                            }
                        });
                    });
                    
                    ui.add_space(10.0);
                    ui.separator();
                    ui.add_space(10.0);
                    
                    ui.label("Everyday User is a cross-platform utility to help maintain consistent computing experiences.");
                    ui.label("Version: 0.1.0");
                });
            
            // Update the original field after the window is closed
            self.show_help = show_window;
        }
    }
}

impl eframe::App for EverydayUserApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Add top menu bar
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Exit").clicked() {
                         std::process::exit(0);
                    }
                });
                
                ui.menu_button("Help", |ui| {
                    if ui.button("About").clicked() {
                        self.show_help = true;
                        ui.close_menu();
                    }
                });
            });
        });

        // Show help window if needed
        self.show_help_window(ctx);
        
        egui::CentralPanel::default().show(ctx, |ui| {
            // Render the ASCII banner at the top
            self.render_ascii_banner(ui);
            
            // Add a separator
            ui.add(egui::Separator::default());
            ui.add_space(10.0);
            
            // Main content
            ui.vertical_centered(|ui| {
                ui.label("Welcome to your cross-platform Rust app.");
                ui.add_space(5.0);
                
                if ui.button("Click me").clicked() {
                    self.counter += 1;
                }
                
                ui.label(format!("Button clicked {} times", self.counter));
            });
        });
    }
}