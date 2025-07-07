use kmobile_desktop::{
    xcode_integration::{BuildConfiguration, WorkflowStep},
    DeviceBridge, HardwareEmulator, XcodeConfig, XcodeIntegration, XcodeWorkflow,
};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("🍎 KMobile Desktop - Xcode Integration Example");
    println!("===============================================");

    // Initialize core components
    let device_bridge = Arc::new(RwLock::new(DeviceBridge::new("localhost", 3000).await?));
    let hardware_emulator = Arc::new(RwLock::new(HardwareEmulator::new().await?));

    // Configure Xcode integration
    let config = XcodeConfig {
        developer_team_id: Some("YOUR_TEAM_ID".to_string()),
        default_bundle_identifier: Some("com.example.kmobile".to_string()),
        enable_hardware_keyboard: true,
        enable_debug_logging: true,
        auto_boot_simulators: true,
        ..Default::default()
    };

    // Initialize Xcode integration
    let mut xcode = XcodeIntegration::new(device_bridge, hardware_emulator, config).await?;

    // Get system status
    let status = xcode.get_system_status().await?;
    println!("📊 System Status:");
    println!("   - Xcode installed: {}", status.xcode_installed);
    println!("   - simctl available: {}", status.simctl_available);
    println!("   - Active simulators: {}", status.active_simulators);
    println!("   - Connected devices: {}", status.connected_devices);

    // List available simulators
    println!("\n📱 Available iOS Simulators:");
    let simulators = xcode.list_simulators().await?;
    for (i, sim) in simulators.iter().enumerate().take(5) {
        println!(
            "   {}. {} ({}) - {:?}",
            i + 1,
            sim.name,
            sim.runtime,
            sim.state
        );
    }

    // Example: Boot a simulator (if available)
    if let Some(simulator) = simulators.first() {
        println!("\n🚀 Booting simulator: {}", simulator.name);
        match xcode.boot_simulator(&simulator.udid).await {
            Ok(()) => println!("   ✅ Simulator booted successfully"),
            Err(e) => println!("   ❌ Failed to boot simulator: {e}"),
        }

        // Example: Simulate location
        println!("\n📍 Simulating location on simulator");
        match xcode
            .simulate_location(&simulator.udid, 37.7749, -122.4194)
            .await
        {
            Ok(()) => println!("   ✅ Location simulated: San Francisco"),
            Err(e) => println!("   ❌ Failed to simulate location: {e}"),
        }

        // Example: Toggle hardware keyboard
        println!("\n⌨️ Enabling hardware keyboard");
        match xcode.toggle_hardware_keyboard(&simulator.udid, true).await {
            Ok(()) => println!("   ✅ Hardware keyboard enabled"),
            Err(e) => println!("   ❌ Failed to enable hardware keyboard: {e}"),
        }

        // Example: Take screenshot
        println!("\n📸 Taking screenshot");
        let screenshot_path = std::env::temp_dir().join("kmobile_screenshot.png");
        match xcode
            .take_device_screenshot(&simulator.udid, &screenshot_path)
            .await
        {
            Ok(()) => println!("   ✅ Screenshot saved to: {screenshot_path:?}"),
            Err(e) => println!("   ❌ Failed to take screenshot: {e}"),
        }
    }

    // Example: Build workflow
    println!("\n🔄 Executing Build Workflow");
    let workflow = XcodeWorkflow {
        name: "iOS App Build and Test".to_string(),
        description: "Complete iOS app build and test workflow".to_string(),
        steps: vec![
            WorkflowStep::BootSimulator {
                udid: "example-simulator-udid".to_string(),
            },
            WorkflowStep::BuildProject {
                project_path: PathBuf::from("./MyApp.xcodeproj"),
                scheme: "MyApp".to_string(),
                configuration: BuildConfiguration::Debug,
            },
            WorkflowStep::RunTests {
                project_path: PathBuf::from("./MyApp.xcodeproj"),
                scheme: "MyApp".to_string(),
                destination: "platform=iOS Simulator,name=iPhone 14".to_string(),
            },
        ],
    };

    match xcode.execute_workflow(workflow).await {
        Ok(result) => {
            println!("   ✅ Workflow completed: {}", result.name);
            println!("   📊 Success: {}", result.success);
            println!("   ⏱️  Duration: {:?}", result.duration);
            for step in result.steps {
                println!("     - {}: {}", step.step_name, step.message);
            }
        }
        Err(e) => println!("   ❌ Workflow failed: {e}"),
    }

    // Example: Advanced hardware simulation
    println!("\n🎛️ Advanced Hardware Simulation");
    if let Some(simulator) = simulators.first() {
        // Simulate push notification
        use kmobile_desktop::xcode_integration::{NotificationPriority, PushNotification};

        let notification = PushNotification {
            bundle_identifier: "com.example.myapp".to_string(),
            payload: serde_json::json!({
                "aps": {
                    "alert": "Hello from KMobile Desktop!",
                    "sound": "default"
                }
            }),
            device_token: None,
            priority: NotificationPriority::High,
            expiration: None,
        };

        match xcode
            .simulate_push_notification(&simulator.udid, notification)
            .await
        {
            Ok(()) => println!("   ✅ Push notification simulated"),
            Err(e) => println!("   ❌ Failed to simulate push notification: {e}"),
        }
    }

    println!("\n🎉 Xcode Integration Example Complete!");
    println!("💡 This example demonstrates the comprehensive iOS development");
    println!("   capabilities available through KMobile Desktop's Xcode integration.");

    Ok(())
}
