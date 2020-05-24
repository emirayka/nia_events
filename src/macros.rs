#[macro_export]
macro_rules! device_listener_log {
    ($id:expr, $format:expr $(, $arg:expr)*) => {
        debug!(concat!("[nia-events] [Device listener #{}] ", $format), $id, $($arg),*);
    };
}

#[macro_export]
macro_rules! device_listener_elog {
    ($id:expr, $format:expr $(, $arg:expr)*) => {
        error!(concat!("[nia-events] [Device listener #{}] ", $format), $id, $($arg),*);
    };
}

#[macro_export]
macro_rules! device_listener_aggregator_log {
    ($format:expr $(, $arg:expr)*) => {
        debug!(concat!("[nia-events] [Device listener aggregator] ", $format), $($arg),*);
    };
}

#[macro_export]
macro_rules! device_listener_aggregator_elog {
    ($format:expr $(, $arg:expr)*) => {
        error!(concat!("[nia-events] [Device listener aggregator] ", $format), $($arg),*);
    };
}

#[macro_export]
macro_rules! key_chord_producer_log {
    ($format:expr $(, $arg:expr)*) => {
        debug!(concat!("[nia-events] [Key chord producer] ", $format), $($arg),*);
    };
}

#[macro_export]
macro_rules! key_chord_producer_elog {
    ($format:expr $(, $arg:expr)*) => {
        error!(concat!("[nia-events] [Key chord producer] ", $format), $($arg),*);
    };
}

#[macro_export]
macro_rules! listener_log {
    ($format:expr $(, $arg:expr)*) => {
        debug!(concat!("[nia-events] [Listener] ", $format), $($arg),*);
    };
}

#[macro_export]
macro_rules! listener_elog {
    ($format:expr $(, $arg:expr)*) => {
        error!(concat!("[nia-events] [Listener] ", $format), $($arg),*);
    };
}

#[macro_export]
macro_rules! uinput_worker_log {
    ($format:expr $(, $arg:expr)*) => {
        debug!(concat!("[nia-events] [UInput Worker] ", $format), $($arg),*);
    };
}

#[macro_export]
macro_rules! uinput_worker_elog {
    ($format:expr $(, $arg:expr)*) => {
        error!(concat!("[nia-events] [UInput Worker] ", $format), $($arg),*);
    };
}

#[macro_export]
macro_rules! xorg_worker_log {
    ($format:expr $(, $arg:expr)*) => {
        debug!(concat!("[nia-events] [Xorg worker] ", $format), $($arg),*);
    };
}

#[macro_export]
macro_rules! xorg_worker_elog {
    ($format:expr $(, $arg:expr)*) => {
        error!(concat!("[nia-events] [Xorg worker] ", $format), $($arg),*);
    };
}

#[macro_export]
macro_rules! worker_log {
    ($format:expr $(, $arg:expr)*) => {
        debug!(concat!("[nia-events] [Worker] ", $format), $($arg),*);
    };
}

#[macro_export]
macro_rules! worker_elog {
    ($format:expr $(, $arg:expr)*) => {
        error!(concat!("[nia-events] [Worker] ", $format), $($arg),*);
    };
}

#[macro_export]
macro_rules! main_log {
    ($format:expr $(, $arg:expr)*) => {
        debug!(concat!("[nia-events] [Main] ", $format), $($arg),*);
    };
}

#[macro_export]
macro_rules! main_elog {
    ($format:expr $(, $arg:expr)*) => {
        error!(concat!("[nia-events] [Main] ", $format), $($arg),*);
    };
}
