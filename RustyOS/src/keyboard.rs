use pc_keyboard::{layouts, HandleControl, Keyboard, ScancodeSet1};
use spin::Mutex;
use lazy_static::lazy_static;
use crossbeam_queue::ArrayQueue;

lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = {
        Mutex::new(Keyboard::new(
            ScancodeSet1::new(),
            layouts::Us104Key,
            HandleControl::MapLettersToControlCodes,
        ))
    };
    
    static ref SCANCODE_QUEUE: ArrayQueue<u8> = ArrayQueue::new(100);
}

pub fn add_scancode(scancode: u8) {
    if let Err(_) = SCANCODE_QUEUE.push(scancode) {
        println!("WARNING: scancode queue full; dropping scancode");
    }
}

pub fn process_keystrokes() {
    let mut keyboard = KEYBOARD.lock();
    
    while let Ok(scancode) = SCANCODE_QUEUE.pop() {
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let pc_keyboard::KeyEvent { code, state: pc_keyboard::KeyState::Down } = key_event {
                match code {
                    pc_keyboard::KeyCode::BackSpace => {
                        print!("{}", 8 as char);
                    },
                    pc_keyboard::KeyCode::Return => {
                        println!();
                    },
                    _ => {
                        if let Ok(c) = char::try_from(code) {
                            print!("{}", c);
                        }
                    }
                }
            }
        }
    }
}
