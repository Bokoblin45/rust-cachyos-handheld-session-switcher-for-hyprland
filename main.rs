use rsbash::rash;
use std::io;


fn main() {
    println!("pick DE (case sensitive): 
    Hyprland
    -dynamic tyling wm with in this case end_4 ii dots.
    
    Gamescope
    -Steamos session for purely gaming, great on handhelds.
    
    Plasma
    -stacking DE, works great with a steam deck, thing that you use when you click desktop mode in gamescope.");
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    if input == "Hyprland" {
        let (_ret_val, stdout, stderr) = rash!("systemctl --user mask cachyos-gamescope-autologin.service && pkexec /usr/lib/steamos/steam-set-session hyprland.desktop").unwrap();
        println!("session set to hyprland, {stdout} {stderr}");

    }
    else if input == "Gamescope" {
    let (_ret_val, stdout, stderr) = rash!("steamos-session-select oneshot").unwrap();
    println!("session set to: Gamescope, {stdout} {stderr}");
    }
    else if input == "Plasma" {
        let (_ret_val, stdout, stderr) = rash!("systemctl --user mask cachyos-gamescope-autologin.service && pkexec /usr/lib/steamos/steam-set-session plasma.desktop").unwrap();
        println!("session set to plasma, {stdout} {stderr}");c
    }
    else {
        println!("pick a valid option, you typed: {input} ");
    };
    println!("please reboot for any changes to apply.");
}
