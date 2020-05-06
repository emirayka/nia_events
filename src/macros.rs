#[macro_export]
macro_rules! keyboard_listener_log {
    ($id:expr, $($arg:tt)*) => {
        {
            std::io::stdout().lock();
            magenta!("[nia-events]");
            dark_blue!("[Keyboard listener #{}] ", $id);
            white!($($arg)*);
            println!();
        }
    }
}

#[macro_export]
macro_rules! keyboard_listener_elog {
    ($id:expr, $($arg:tt)*) => {
        {
            std::io::stdout().lock();
            magenta!("[nia-events]");
            dark_blue!("[Keyboard listener #{}] ", $id);
            red!($($arg)*);
            println!();
        }
    }
}

#[macro_export]
macro_rules! keyboard_listener_aggregator_log {
    ($($arg:tt)*) => {
        {
            std::io::stdout().lock();
            magenta!("[nia-events]");
            dark_blue!("[Keyboard listener aggregator] ");
            white!($($arg)*);
            println!();
        }
    }
}

#[macro_export]
macro_rules! keyboard_listener_aggregator_elog {
    ($($arg:tt)*) => {
        {
            std::io::stdout().lock();
            magenta!("[nia-events]");
            dark_blue!("[Keyboard listener aggregator] ");
            red!($($arg)*);
            println!();
        }
    }
}

#[macro_export]
macro_rules! key_chord_producer_log {
    ($($arg:tt)*) => {
        {
            std::io::stdout().lock();
            magenta!("[nia-events]");
            dark_blue!("[Key chord producer] ");
            white!($($arg)*);
            println!();
        }
    }
}

#[macro_export]
macro_rules! key_chord_producer_elog {
    ($($arg:tt)*) => {
        {
            std::io::stdout().lock();
            magenta!("[nia-events]");
            dark_blue!("[Key chord producer] ");
            red!($($arg)*);
            println!();
        }
    }
}

#[macro_export]
macro_rules! listener_log {
    ($($arg:tt)*) => {
        {
            std::io::stdout().lock();
            magenta!("[nia-events]");
            dark_blue!("[Listener] ");
            white!($($arg)*);
            println!();
        }
    }
}

#[macro_export]
macro_rules! listener_elog {
    ($($arg:tt)*) => {
        {
            std::io::stdout().lock();
            magenta!("[nia-events]");
            dark_blue!("[Listener] ");
            red!($($arg)*);
            println!();
        }
    }
}

#[macro_export]
macro_rules! uinput_worker_log {
    ($($arg:tt)*) => {
        {
            std::io::stdout().lock();
            magenta!("[nia-events]");
            dark_blue!("[UInput worker] ");
            white!($($arg)*);
            println!();
        }
    }
}

#[macro_export]
macro_rules! uinput_worker_elog {
    ($($arg:tt)*) => {
        {
            std::io::stdout().lock();
            magenta!("[nia-events]");
            dark_blue!("[UInput worker] ");
            red!($($arg)*);
            println!();
        }
    }
}

#[macro_export]
macro_rules! xorg_worker_log {
    ($($arg:tt)*) => {
        {
            std::io::stdout().lock();
            magenta!("[nia-events]");
            dark_blue!("[Xorg worker] ");
            white!($($arg)*);
            println!();
        }
    }
}

#[macro_export]
macro_rules! xorg_worker_elog {
    ($($arg:tt)*) => {
        {
            std::io::stdout().lock();
            magenta!("[nia-events]");
            dark_blue!("[Xorg worker] ");
            red!($($arg)*);
            println!();
        }
    }
}

#[macro_export]
macro_rules! worker_log {
    ($($arg:tt)*) => {
        {
            std::io::stdout().lock();
            magenta!("[nia-events]");
            dark_blue!("[Worker] ");
            white!($($arg)*);
            println!();
        }
    }
}

#[macro_export]
macro_rules! worker_elog {
    ($($arg:tt)*) => {
        {
            std::io::stdout().lock();
            magenta!("[nia-events]");
            dark_blue!("[Worker] ");
            red!($($arg)*);
            println!();
        }
    }
}

#[macro_export]
macro_rules! main_log {
    ($($arg:tt)*) => {
        {
            std::io::stdout().lock();
            magenta!("[nia-events]");
            dark_blue!("[Main] ");
            white!($($arg)*);
            println!();
        }
    }
}

#[macro_export]
macro_rules! main_elog {
    ($($arg:tt)*) => {
        {
            std::io::stdout().lock();
            magenta!("[nia-events]");
            dark_blue!("[Main] ");
            red!($($arg)*);
            println!();
        }
    }
}
