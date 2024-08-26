use std::thread::sleep;
use std::time::Duration;
use rand::distributions::Distribution;
use rand::thread_rng;
use rand_distr::Normal;
use std::io::{self, Write};
use crossterm::event::{self, Event, KeyCode};

fn randomize_delay(base_delay: u64) -> u64 {
    // Random engine and normal distribution with 33% standard deviation
    let normal = Normal::new(1.0, 0.33).unwrap();
    let mut rng = thread_rng();

    // Calculate the randomized delay, ensuring it's non-negative
    let delay = (base_delay as f64 * normal.sample(&mut rng)).max(0.0);
    delay as u64
}

// Character-by-character typing effect triggered by key press
// WARNING: This function does not work on unix systems, only on windows (afaik)
// (cont) for unix builds use the automatic building .rs file
fn print_typing_effect_on_keypress(text: &str) {
    for c in text.chars() {
        // Wait for a key press
        loop {
            if event::poll(Duration::from_millis(500)).unwrap() {
                if let Event::Key(event) = event::read().unwrap() {
                    if let KeyCode::Char(_) = event.code {
                        print!("{}", c);
                        std::io::stdout().flush().unwrap();
                        break;
                    }
                }
            }
        }
    }
}

// Consume all key presses until Enter is pressed
fn consume_keypresses_until_enter() {
    loop {
        if event::poll(Duration::from_millis(500)).unwrap() {
            if let Event::Key(event) = event::read().unwrap() {
                // Break out of the loop only if Enter is pressed
                if event.code == KeyCode::Enter {
                    break;
                }
            }
        }
    }
}

// Line-by-line effect for ASCII art
fn print_line_by_line_effect(text: &str, delay_per_line: u64) {
    for line in text.lines() {
        println!("{}", line);
        std::io::stdout().flush().unwrap();
        sleep(Duration::from_millis(randomize_delay(delay_per_line)));
    }
}

fn main() {
    let tron = r#"
........'''''.      ..'',,,,,,,,,;;;;;;;;;;;;;;;;;;;;;;;;;;,..                ..';:cc::;,..           .;;;;,                   .''''''.'
,:'........,dc    ,loc;;;,,,,,,,,,,,;;;;;;;;;;;;;;;;;;;;;;;cod1'           'codlc:,,,,;:loxxc'      ..xx;;;dd.                 cx,....,o'
,c.        .x:  ,ol'...                                      .,oo'      .:ol,.           ..':xkl.   ..kk. ..ckl.               cO.    'o'
;l.        .kc cx:..     .......................................;d,   .cd:.        ...       .'oO1. .'kk.   .'lkc.             lO.    'd'
.;,,,,,,,,;;c;'ko..   ;ddllllllllooooooooooodddddddddddddddddddxxdo. .dc.     .,:oodddxdl;.    .;kO,.'Ok.    ..'dk:.           lO.    'd'
              ,x: .   .kd............................................ol.     ,dl,......'ckOc.....,OO''OO.      ..;xx'          l0.    .d'
              ,kc .   .ko..   ....,kdlclllxd'.:kdooodddddddddddddxx':x...  .lo............1Kx.....;0d'OO.        ..cko. .......o0.    ,d'
              ,kc .   .kd..  .....:Kd.....O0,.oK;...............:kc.dc.....ck'.............1Xc.....k0,OO.    .......;0x,xc'''',':.    ,x'
              ,kc .   .kd..    ...:0d.....O0,.oK;.............;dd,..d:.....lO..............cKc.....x0,0O.    .......;0x;0:            ,x'
              ,kc .   .kd..      .;0d.....O0'...ckd,.....':::cloc'....co.    ,ko............,OO.....,0x'0O.  ..ox:,,'';:''dx;           ,x,
              ,x:      ko..       ;0d.....O0'...;dd;.    .:dc.    .lc.       .........      .;Ox'   .'OO.  ..oO,       ..;xd.         ,x,
              ,x:      xo..       ;0o.   .k0'.....:xo,.....'ld;......'o'.    .'colc::cldd1'    ..xK:.'OO.  ..oO,          .cxc.       ,x'
              ,x;      xo..       ;Oo.   .k0'.     .;dd;.    .:dc.    .'lc'.              .'lkk;   .'Ok.  ..lO,            .cd;      ,x'
              ,x;      xl .       ,Oo.   .k0'.       .,od;,    .:dc.    .'lc'.              .'okxc.     .'kk.  ..lk,               .'oo.  ,d,
              .c;;''',;,          .oo::cccl:.             'ldigitalllc.      ..,;:ccllllc:'.         .:c:::::ll.                 .,,',,c."#;

    let stages = ["Initialising", "Loading Resources", "Starting Services", "Computing Probability Vectors"];

    // Display loading stages with the original character-by-character effect
    for stage in &stages {
        print!("{}", stage);
        std::io::stdout().flush().unwrap();
        sleep(Duration::from_millis(randomize_delay(1000)));  // Randomized 1-second delay

        for _ in 0..3 {
            print!(".");
            std::io::stdout().flush().unwrap();
            sleep(Duration::from_millis(randomize_delay(400)));  // Randomized 0.4-second delay
        }

        println!();
    }
    
    print!("Ready\n");
    std::io::stdout().flush().unwrap();
    sleep(Duration::from_millis(500));
    
    // Display TRON ASCII art with the line-by-line effect
    sleep(Duration::from_millis(randomize_delay(1500)));  // Randomized 1.5-second delay
    print_line_by_line_effect(tron, 120);  // Delay of 200ms between lines
    sleep(Duration::from_millis(1000));

    // Print the first part of the command instantly
    print!("\nroot@server:~# ");
    std::io::stdout().flush().unwrap();

    // Print the rest of the command character by character on key press
    print_typing_effect_on_keypress("LLLSDLaserControl -ok 1");

    // Consume all keypresses until Enter is pressed
    consume_keypresses_until_enter();

    // Program ends when Enter is pressed
}
