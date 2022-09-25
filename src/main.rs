use taos_lib::{System, Commands,  FlightControlSys as fcs};
use tokio;
use std::time::Duration;

#[tokio::main]
async fn main() {
    // Startup 

    let mut sys_state = System::State::new();
    let mut fcs = fcs::FCS::default();

    let mut d:f32 = 0.0;
    
    loop {
        // Instantiate command collection
        // let mut commands = Commands::CommandPacket::new();

        // commands.add(Commands::CommandType::SetSysMode, System::Mode::Startup);

        // println!("{:?}", commands.values);

        fcs.set_elevator_deflection(d += 5.0);
        fcs.update();


        tokio::time::sleep(Duration::from_secs(1)).await;

    }




    // commands.add(Commands::CommandType::SetSysMode, na::)

}
