/// A cute friend who's there for you when you make a typo.
use std::{thread::sleep, time::Duration};

use std::io::{stdout, Write};

static FRAMES: [&str; 80] = [
    "(\\°-°)\\ ┬─┬",
    "(\\°-°)\\ ┬─┬",
    "(\\°-°)\\ ┬─┬",
    "(\\°-°)\\ ┬─┬",
    "(\\°-°)\\ ┬─┬",
    "(\\°-°)\\ ┬─┬",
    "(\\°-°)\\ ┬─┬",
    "(\\°-°)\\ ┬─┬",
    "(\\°-°)\\ ┬─┬",
    "(\\°-°)\\ ┬─┬",
    "(\\°-°)\\ ┬─┬",
    "(\\°-°)\\ ┬─┬",
    "(\\°-°)\\ ┬─┬",
    "(\\°-°)\\ ┬─┬",
    "(\\°-°)\\ ┬─┬",
    "(\\°-°)\\ ┬─┬",
    "(\\°-°)\\ ┬─┬",
    "(\\°-°)\\ ┬─┬",
    "(\\°-°)\\ ┬─┬",
    "(\\°-°)\\ ┬─┬",
    "(\\°-°)\\ ┬─┬",
    "(\\°-°)\\ ┬─┬",
    "(\\°□°)\\  ┬─┬",
    "(\\°□°)\\  ┬─┬",
    "(-°□°)-  ┬─┬",
    "(-°□°)-  ┬─┬",
    "(╯°□°)╯    ]",
    "(╯°□°)╯    ]",
    "(╯°□°)╯    ]",
    "(╯°□°)╯    ]",
    "(╯°□°)╯  ︵  ┻━┻",
    "(╯°□°)╯  ︵  ┻━┻",
    "(╯°□°)╯  ︵  ┻━┻",
    "(╯°□°)╯  ︵  ┻━┻",
    "(╯°□°)╯       [",
    "(╯°□°)╯       [",
    "(╯°□°)╯       [",
    "(╯°□°)╯       [",
    "(╯°□°)╯       ︵  ┬─┬",
    "(╯°□°)╯       ︵  ┬─┬",
    "(╯°□°)╯       ︵  ┬─┬",
    "(╯°□°)╯       ︵  ┬─┬",
    "(╯°□°)╯                 ]",
    "(╯°□°)╯                 ]",
    "(╯°□°)╯                 ]",
    "(╯°□°)╯                 ]",
    "(╯°□°)╯               ︵  ┻━┻",
    "(╯°□°)╯               ︵  ┻━┻",
    "(╯°□°)╯               ︵  ┻━┻",
    "(╯°□°)╯               ︵  ┻━┻",
    "(╯°□°)╯                         [",
    "(╯°□°)╯                         [",
    "(╯°□°)╯                         [",
    "(╯°□°)╯                         [",
    "(\\°-°)\\                            ︵  ┬─┬",
    "(\\°-°)\\                            ︵  ┬─┬",
    "(\\°-°)\\                            ︵  ┬─┬",
    "(\\°-°)\\                            ︵  ┬─┬",
    "(\\°-°)\\                                     ]",
    "(\\°-°)\\                                     ]",
    "(\\°-°)\\                                     ]",
    "(\\°-°)\\                                     ]",
    "(\\°-°)\\                                     ︵ ┻━┻",
    "(\\°-°)\\                                     ︵ ┻━┻",
    "(\\°-°)\\                                     ︵ ┻━┻",
    "(\\°-°)\\                                     ︵ ┻━┻",
    "(\\°-°)\\                                               [",
    "(\\°-°)\\                                               [",
    "(\\°-°)\\                                               [",
    "(\\°-°)\\                                               [",
    "(\\°-°)\\                                              ┬─┬",
    "(\\°-°)\\                                              ┬─┬",
    "(\\°-°)\\                                              ┬─┬",
    "(\\°-°)\\                                              ┬─┬",
    "(\\°-°)\\                                              ┬─┬",
    "(\\°-°)\\                                              ┬─┬",
    "(\\°-°)\\                                              ┬─┬",
    "(\\°-°)\\                                              ┬─┬",
    "(\\°-°)\\                                              ┬─┬",
    "(\\°-°)\\                                              ┬─┬",
];

fn main() {
    let mut lock = stdout().lock();
    for frame in FRAMES {
        write!(lock, "\r{}", frame).unwrap();
        lock.flush().unwrap();
        sleep(Duration::from_millis(25));
    }
    write!(lock, "\n").unwrap();
}
