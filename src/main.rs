use rsbash::rash;
use std::io;


fn main() {
    println!("pick DE (case sensitive): 
    Hyprland
    -dynamic tyling wm with in this case end_4 ii dots.
    
    Gamescope
    -Steamos session for purely gaming, great on handhelds.");
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    if input == "Hyprland" {
        let (_ret_val, stdout, stderr) = rash!("switch-to-hyprland").unwrap();
        println!("session set to hyprland, {stdout} {stderr}");

    }
    else if input == "Gamescope" {
    let (_ret_val, stdout, stderr) = rash!("steamos-session-select oneshot").unwrap();
    println!("session set to: Gamescope, {stdout} {stderr}");
    }
    else {
        println!("pick 1 or 2, you typed: {input} ");
    };
    println!("please reboot for any changes to apply.");
}
