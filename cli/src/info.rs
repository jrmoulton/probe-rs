use crate::common::open_probe;
use crate::{common::CliError, SharedOptions};
use probe_rs::architecture::arm::ap::{valid_access_ports, APAccess, APClass, IDR};
use probe_rs::architecture::arm::ArmCommunicationInterface;

pub(crate) fn show_info_of_device(shared_options: &SharedOptions) -> Result<(), CliError> {
    let probe = open_probe(shared_options.n)?;

    /*
        The following code only works with debug port v2,
        which might not necessarily be present.

        Once the typed interface for the debug port is done, it
        can be enabled again.

    println!("Device information:");


    let target_info = link
        .read_register(PortType::DebugPort, 0x4)?;

    let target_info = parse_target_id(target_info);
    println!("\nTarget Identification Register (TARGETID):");
    println!(
        "\tRevision = {}, Part Number = {}, Designer = {}",
        target_info.0, target_info.3, target_info.2
    );

    */

    // Note: Temporary read to ensure the DP information is read at
    //       least once before reading the ROM table
    //       (necessary according to STM manual).
    //
    // TODO: Move to proper place somewhere in init code
    //

    let mut interface = ArmCommunicationInterface::new(probe);
    let target_info = interface.read_register_dp(0x0)?;
    println!("DP info: {:#08x}", target_info);

    println!("\nAvailable Access Ports:");

    for access_port in valid_access_ports(&mut interface) {
        let idr = interface.read_ap_register(access_port, IDR::default())?;
        println!("{:#x?}", idr);

        if idr.CLASS == APClass::MEMAP {
            // let access_port: MemoryAP = access_port.into();

            // let base_register = interface.read_ap_register(access_port, BASE::default())?;

            // let mut baseaddr = if BaseaddrFormat::ADIv5 == base_register.Format {
            //     let base2 = interface.read_ap_register(access_port, BASE2::default())?;
            //     (u64::from(base2.BASEADDR) << 32)
            // } else {
            //     0
            // };
            // baseaddr |= u64::from(base_register.BASEADDR << 12);

            // let rom_table = RomTable::try_parse(core, baseaddr as u64)
            //         .map_err(Error::architecture_specific)?;

            // for e in rom_table.entries() {
            //     println!(
            //         "ROM Table Entry: Component @ 0x{:08x}",
            //         e.component_id.component_address()
            //     );
            // }
        }
    }

    Ok(())
}
