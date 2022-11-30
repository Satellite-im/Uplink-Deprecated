pub type SecurityCard = Vec<Node>;

/// Represents the minimum level required to access a given permission.
const MIN_LEVEL: usize = 2;

/// Represents the available levels of permission per node.
#[derive(Default, PartialEq)]
pub enum Level {
    ALLOW = 3,
    OPTIONAL = 2,
    DENY = 1,
}

#[derive(Default, PartialEq)]
pub enum Node {
    BASIC_UI(Option<Level>),
    DOM_ACCESS(Option<Level>),
    READ_HOME_FOLDER(Option<Level>),
    WRITE_HOME_FOLDER(Option<Level>),
    READ_DISK(Option<Level>),
    WRITE_DISK(Option<Level>),
    NOTIFICATION(Option<Level>),
    CAMERA(Option<Level>),
    MICROPHONE(Option<Level>),
    SCREEN_CAPTURE(Option<Level>),
    USB(Option<Level>),
    READ_STATE(Option<Level>),
    WRITE_STATE(Option<Level>),
    PUSH_CHATBAR(Option<Level>),
    READ_CHATBAR(Option<Level>),
    WRITE_CHATBAR(Option<Level>),
    CONFIG(Option<Level>),
    SYSTEM(Option<Level>),
    EVAL(Option<Level>),
    // This is just a subtle reminder.
    KEYS(None),
}

impl Default for SecurityCard {
    fn default() -> Self {
        vec![Node::BASIC_UI(3), Node::NOTIFICATION(2)]
    }
}

impl Node {
    /// Instead of comparing a bunch of permissions, an extension should just specify a security card of it's own.
    /// This card will be required to be included in the extension. The user can choose which permissions they want to allow.
    /// If the user opts out of a required permission the extension will not load.
    /// If an extension does not have required permissions to access specific things, it will not compile.
    /// If mutations are done to MAKE it compile, the hashes will not match, and the extension will not run.
    /// Long story short, you MUST specify in your extension which nodes you'd like to access.
    fn has(&self, card: SecurityCard) -> bool {
        for node in card {
            if node.eq(self) {
                match node {
                    Some(n) => n >= MIN_LEVEL,
                    None => false,
                }
            }
        }
        false
    }
}

impl SecurityCard {
    /// Compare a granted card against this card (i'm usually the one in your extension)
    fn compatible(&self, granted: SecurityCard) -> bool {
        let mut all_good = false;
        // All permissions required must be at least the level granted.
        for gnode in granted {
            for node in self {
                match gnode {
                    Some(gn) => match node {
                        Some(n) => {
                            // Node granted must be at least the level the extension needs.
                            // Optional nodes will still pass here. Extensions should only optionally require nodes which
                            // include fallbacks for if the user opt's out
                            if gnode >= n {
                                all_good = true;
                            }
                        }
                        None => all_good = false,
                    },
                    None => all_good = false,
                }
            }
        }
        all_good
    }
}
