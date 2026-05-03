use std::time::Duration;

pub mod timing {
    use super::*;

    pub const EVENT_POLL: Duration = Duration::from_millis(200);
    pub const IP_REFRESH_SLEEP: Duration = Duration::from_millis(200);
    pub const TRAFFIC_REFRESH: Duration = Duration::from_millis(200);
    pub const RESTART_DELAY: Duration = Duration::from_millis(100);
}

pub mod net {
    pub const LOCAL_PROXY_ADDR: &str = "127.0.0.1:12334";
    pub const LOADING_IP: &str = "loading...";
    pub const FALLBACK_IP: &str = "0.0.0.0";
    pub const UNAVAILABLE_IP: &str = "ip unavailable";
}

pub mod traffic {
    use std::collections::HashMap;
    use std::sync::LazyLock;

    pub const HISTORY_LIMIT: usize = 900;
    pub const MIN_RATE: u64 = 64 * 1024;

    pub const KB: f64 = 1024.0;
    pub const MB: f64 = 1024.0 * 1024.0;

    pub const MIN_WIDTH: u16 = 3;
    pub const MIN_HEIGHT: u16 = 4;

    pub static BAR_MAP: LazyLock<HashMap<u8, char>> = LazyLock::new(|| {
        HashMap::from([
            (000, ' '),
            (001, '⢀'),
            (002, '⢠'),
            (003, '⢰'),
            (004, '⢸'),
            (010, '⡀'),
            (011, '⣀'),
            (012, '⣠'),
            (013, '⣰'),
            (014, '⣸'),
            (020, '⡄'),
            (021, '⣄'),
            (022, '⣤'),
            (023, '⣴'),
            (024, '⣼'),
            (030, '⡆'),
            (031, '⣆'),
            (032, '⣦'),
            (033, '⣶'),
            (034, '⣾'),
            (040, '⡇'),
            (041, '⣇'),
            (042, '⣧'),
            (043, '⣷'),
            (044, '⣿'),
            (100, ' '),
            (101, '⠈'),
            (102, '⠘'),
            (103, '⠸'),
            (104, '⢸'),
            (110, '⠁'),
            (111, '⠉'),
            (112, '⠙'),
            (113, '⠹'),
            (114, '⢹'),
            (120, '⠃'),
            (121, '⠋'),
            (122, '⠛'),
            (123, '⠻'),
            (124, '⢻'),
            (130, '⠇'),
            (131, '⠏'),
            (132, '⠟'),
            (133, '⠿'),
            (134, '⢿'),
            (140, '⡇'),
            (141, '⡏'),
            (142, '⡟'),
            (143, '⡿'),
            (144, '⣿'),
        ])
    });
}

pub mod ui {
    pub const HELP_HEIGHT: u16 = 1;

    pub const LEFT_PANEL_WEIGHT: u16 = 2;
    pub const RIGHT_PANEL_WEIGHT: u16 = 5;

    pub const INPUT_HEIGHT: u16 = 3;

    pub const CONTEXT_X_OFFSET: u16 = 4;
    pub const CONTEXT_Y_OFFSET: u16 = 2;
    pub const CONTEXT_WIDTH_PADDING: u16 = 8;
    pub const CONTEXT_HEIGHT: u16 = 7;

    pub const VALUE_INPUT_X_OFFSET: u16 = 2;
    pub const VALUE_INPUT_Y_OFFSET: u16 = 4;
    pub const VALUE_INPUT_WIDTH_PADDING: u16 = 4;
    pub const VALUE_INPUT_HEIGHT: u16 = 3;

    pub const ROUTE_ACTION_INDEX: usize = 0;
    pub const ROUTE_TYPE_INDEX: usize = 1;
    pub const ROUTE_VALUE_INDEX: usize = 2;
    pub const DNS_TYPE_INDEX: usize = 3;
    pub const DNS_VALUE1_INDEX: usize = 4;
    pub const DNS_VALUE2_INDEX: usize = 5;
    pub const ENTER_INDEX: usize = 6;

    pub const SETTINGS_FIELDS_COUNT: usize = 6;
    pub const ROUTE_FIELDS_COUNT: usize = 3;

    pub const ROUTE_ACTION_CUSTOM_INDEX: usize = 3;

    pub const SELECTED_SYMBOL: &str = ">> ";
    pub const RUNNING_SYMBOL: &str = "● ";
}

pub mod text {
    pub const EMPTY: &str = "empty";

    pub const CONFIGS_TITLE: &str = "Configs";
    pub const LOGS_TITLE: &str = "Logs";
    pub const SETTINGS_TITLE: &str = "Settings";
    pub const SELECT_TITLE: &str = "Select";

    pub const ERROR_INPUT: &str = "Error input";
    pub const ADD_CONFIG_URL: &str = "Add new config url";
    pub const ADD_TUN_CONFIG_URL: &str = "Add new config url with tun arg";
    pub const ENTER_VALUE: &str = "Enter value";

    pub const ROUTING_RULES_TITLE: &str = "Routing Rules";
    pub const DNS_SERVERS_TITLE: &str = "DNS Servers";

    pub const ACTION_LABEL: &str = "Action: ";
    pub const TYPE_LABEL: &str = "Type: ";
    pub const VALUE_LABEL: &str = "Value: ";
    pub const VALUE1_LABEL: &str = "Value 1: ";
    pub const VALUE2_LABEL: &str = "Value 2: ";

    pub const ENTER_BUTTON: &str = "[ENTER]";
    pub const PERSONAL: &str = "personal";
    pub const NO_ITEMS: &str = "No items!";
    pub const INPUT_PREFIX: &str = "Input: ";

    pub const HELP: &str =
        "↑/↓ navigate   q exit   a adding config  A adding tun config d delete config";

    pub const TRAFFIC_TITLE: &str = "Traffic";
}

pub mod keys {
    pub const QUIT: char = 'q';
    pub const ADD_CONFIG: char = 'a';
    pub const ADD_TUN_CONFIG: char = 'A';
    pub const DELETE_CONFIG: char = 'd';
    pub const DOWN_ALT: char = 'j';
    pub const UP_ALT: char = 'k';
}

pub mod route {
    pub const ACTIONS: &[&str] = &["r", "h", "s"];

    pub const TYPES: &[(&str, &str)] = &[
        ("inbound", "ib"),
        ("ip version", "iv"),
        ("auth user", "au"),
        ("protocol", "pl"),
        ("client", "cl"),
        ("domain", "dm"),
        ("domain suffix", "ds"),
        ("domain keyword", "dk"),
        ("domain regex", "dr"),
        ("geosite", "gs"),
        ("source geoip", "sg"),
        ("geoip", "gp"),
        ("source ip cidr", "sc"),
        ("ip is private", "si"),
        ("ip cidr", "ic"),
        ("ip is private", "ip"),
        ("source port", "sp"),
        ("range", "sr"),
        ("port", "pt"),
        ("range", "pr"),
        ("process name", "pn"),
        ("process path", "pp"),
        ("regex", "pg"),
        ("package name", "kn"),
        ("user", "ur"),
        ("user id", "ui"),
        ("clash mode", "cm"),
        ("network type", "nt"),
        ("network", "nk"),
        ("is expensive", "ne"),
        ("constrained", "nc"),
    ];
}
