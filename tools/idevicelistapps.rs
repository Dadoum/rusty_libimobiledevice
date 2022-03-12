// jkcoxson
// This one isn't an official tool, but something I think is necessary

use rusty_libimobiledevice::instproxy::InstProxyClient;
use rusty_libimobiledevice::libimobiledevice;
use rusty_libimobiledevice::plist::Plist;

fn main() {
    const VERSION: &str = "0.1.0";

    let mut udid = "".to_string();

    // Parse arguments
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-u" | "--udid" => {
                udid = args[i + 1].clone();
                i += 1;
            }
            "-h" | "--help" => {
                println!("Usage: idevicelistapps [options]");
                println!("");
                println!("Options:");
                println!("  -u, --udid <udid>    : udid of the device to mount");
                println!("  -h, --help           : display this help message");
                println!("  -v, --version        : display version");
                return;
            }
            "-v" | "--version" => {
                println!("v{}", VERSION);
                return;
            }
            _ => {
                if args[i].starts_with("-") {
                    println!("Unknown flag: {}", args[i]);
                    return;
                }
            }
        }
        i += 1;
    }
    if udid == "" {
        println!("Error: No UDID specified. Use -u or --udid to specify a device.");
        return;
    }

    // Get the device
    let device = match libimobiledevice::get_device(udid.to_string()) {
        Ok(device) => device,
        Err(e) => {
            println!("Error: Could not find device: {:?}", e);
            return;
        }
    };

    let instproxy_client = match device.new_instproxy_client("idevicelistapps".to_string()) {
        Ok(instproxy) => {
            println!("Successfully started instproxy");
            instproxy
        }
        Err(e) => {
            println!("Error starting instproxy: {:?}", e);
            return;
        }
    };

    let mut client_opts = InstProxyClient::options_new();
    InstProxyClient::options_add(
        &mut client_opts,
        vec![("ApplicationType".to_string(), Plist::new_string("Any"))],
    );
    InstProxyClient::options_set_return_attributes(
        &mut client_opts,
        vec![
            "CFBundleIdentifier".to_string(),
            "CFBundleExecutable".to_string(),
            "Container".to_string(),
        ],
    );
    let lookup_results = match instproxy_client.lookup(vec![], client_opts) {
        Ok(apps) => {
            println!("Successfully looked up apps");
            apps
        }
        Err(e) => {
            println!("Error looking up apps: {:?}", e);
            return;
        }
    };

    println!("{}", lookup_results.to_string());
}
