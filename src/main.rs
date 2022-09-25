use taos_lib::{System, Commands,  FlightControlSys as fcs};
// use tokio;
use std::time::Duration;

fn main() {
    // Startup 

    println!("Startup");
    let mut sys_state = System::State::new();
    let mut fcs = fcs::FCS::default();
    fcs.init_hardware();
    fcs.elevator_d_bound = [-90.0,90.0];

    println!("Hardware initialized");
    // println!("Current Deflection Setting: {}", fcs.elevator_d);
    // println!("Current Hardware PW: {}", fcs.fc_hardware.l_elevator_servo.pos_pw);

    println!("Calibrating IMU");
    fcs.telem_hardware.calibrate_at_rest();
    println!("IMU Calibrated");

    let mut d:f32 = 0.0;
    
    loop {
        println!("=====================================================================");
        println!("IMU Reading: {:?}", fcs.get_telemetry());
        println!("Setting elevator deflection: {d} deg.");

        fcs.set_elevator_deflection(d);

        println!("Current Deflection Setting: {}", fcs.elevator_d);
        println!("Current Hardware pos: {}", fcs.fc_hardware.l_elevator_servo.pos);
        println!("Current Hardware PW: {}", fcs.fc_hardware.l_elevator_servo.pos_pw);

        fcs.update();
        println!("Sent flight control system update command.");

        d+=5.0;

        std::thread::sleep(Duration::from_millis(1000));
        if d > 90.0 {
            fcs.shutdown_hardware();
            return;
        }
    }




    // commands.add(Commands::CommandType::SetSysMode, na::)

}
