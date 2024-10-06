use crate::statics::VERSION;
use stblib::colors::{BLUE, BOLD, C_RESET, GREEN, LIGHT_GREEN, RED, UNDERLINE, WHITE, YELLOW};

pub fn about() {
    println!(
        "\
* ---------- {BLUE}{BOLD}tunneled{C_RESET}{BOLD}{WHITE} ---------- *
|             v{}             |
| tunneled is a simple CLI tool  |
|  for making local tcp tunnels  |
* ------------------------------ *

* ------------------------------ *
|    Copyright (C) 2022 - 2024   |
|     Strawberry Foundations     |
|                                |
| Made possible with the help of |
|     github.com/ekzhang/bore    |
* ------------------------------ *

* ----------- {BOLD}{YELLOW}WARNING{C_RESET}{BOLD} ---------- *
|    This program comes with     |
|     absolutely {RED}{UNDERLINE}NO{C_RESET}{WHITE}{BOLD} warranty     |
|                                |
| {LIGHT_GREEN}This is free software, and you{C_RESET}{WHITE}{BOLD} |
| {LIGHT_GREEN}are welcome to redistribute it{C_RESET}{WHITE}{BOLD} |
* ------------------------------ *

* ------------- {BOLD}{GREEN}License{C_RESET}{WHITE}{BOLD} ------------- *
|        GPL-V3 (original MIT)        |
|         Open Source License         |
|                                     |
| https://opensource.org/license/gpl  |
* ----------------------------------- *{C_RESET}
",
        *VERSION
    );
    std::process::exit(0);
}
