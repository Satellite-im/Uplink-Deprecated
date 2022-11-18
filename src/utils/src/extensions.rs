use std::{collections::HashMap, fmt, fs, path::PathBuf};

use dioxus::prelude::*;
use libloading::{Library, Symbol};
use once_cell::sync::Lazy;
use warp::sync::RwLock;

type Render = unsafe fn() -> Box<fn(Scope) -> Element>;
type Info = unsafe fn() -> Box<Extension>;

static DEFAULT_PATH: Lazy<RwLock<PathBuf>> =
    Lazy::new(|| RwLock::new(dirs::home_dir().unwrap_or_default().join(".warp")));

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum ExtensionType {
    SidebarWidget,
    ChatbarIcon,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Extension {
    pub name: String,
    pub author: String,
    pub description: String,
    pub location: ExtensionType,
}

impl Default for Extension {
    fn default() -> Self {
        Self {
            name: Default::default(),
            author: Default::default(),
            description: Default::default(),
            location: ExtensionType::SidebarWidget,
        }
    }
}

pub trait BasicExtension {
    fn info() -> Extension;
    fn render(cx: Scope) -> Element;
}

#[derive(Clone)]
pub struct ExtensionManager {
    pub info: Extension,
    pub render: fn(Scope) -> Element,
}

impl std::fmt::Display for ExtensionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        match self {
            ExtensionType::ChatbarIcon => write!(f, "ChatbarIcon"),
            ExtensionType::SidebarWidget => write!(f, "SidebarWidget"),
        }
    }
}

impl std::fmt::Display for Extension {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let mut extension_manager_display = String::new();

        extension_manager_display.push_str(self.name.as_str());
        extension_manager_display.push_str(", \n");
        extension_manager_display.push_str(self.author.as_str());
        extension_manager_display.push_str(", \n");
        extension_manager_display.push_str(self.description.as_str());
        extension_manager_display.push_str(", \n");
        extension_manager_display.push_str(self.location.to_string().as_str());

        write!(f, "{}", extension_manager_display)
    }
}

impl std::fmt::Display for ExtensionManager {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let mut extension_manager_display = String::new();

        extension_manager_display.push_str(self.info.to_string().as_str());

        write!(f, "{}", extension_manager_display)
    }
}

pub fn get_extensions() -> HashMap<ExtensionType, Vec<ExtensionManager>> {
    let mut map_extensions: HashMap<ExtensionType, Vec<ExtensionManager>> = HashMap::new();
    let mut ext_mng;
    fs::create_dir_all(DEFAULT_PATH.read().join("extensions")).unwrap();
    let paths = fs::read_dir(DEFAULT_PATH.read().join("extensions")).expect("Directory is empty");
    for path in paths {
        let path_extension = path.unwrap().path();

        unsafe {
            let lib = Library::new(path_extension).unwrap();
            let render: Symbol<Render> = lib.get(b"ret_rend").unwrap();
            let info: Symbol<Info> = lib.get(b"ret_info").unwrap();
            ext_mng = ExtensionManager {
                info: *info(),
                render: *render(),
            };
        }
        let location = ext_mng.clone().info.location;
        map_extensions.entry(location).or_default().push(ext_mng);
    }
    map_extensions
}

#[allow(non_snake_case)]
pub fn get_renders<'src>(
    extension_type: ExtensionType,
    enable: bool,
) -> Vec<LazyNodes<'src, 'src>> {
    match enable {
        true => {
            let exts = get_extensions();
            let mut extensions = vec![];

            if let Some(em) = exts.get(&extension_type) {
                for extension in em {
                    extensions.push(extension.render);
                }
            };

            let closure = |&Ext: &fn(Scope) -> Option<VNode>| {
                rsx! (
                    div {
                        Ext {},
                    },
                )
            };

            let extensions_to_render = extensions.iter().map(closure).collect::<Vec<LazyNodes>>();
            extensions_to_render
        }
        false => vec![],
    }
}

pub fn get_info(
    name: Option<&str>,
    author: Option<&str>,
    location: Option<ExtensionType>,
) -> Vec<Extension> {
    let exts = get_extensions();
    let mut extensions = vec![];

    if name.is_none() && author.is_none() {
        if location.is_none() {
            for (_ext_type, ext_mngs) in exts {
                for ext_mng in ext_mngs {
                    extensions.push(ext_mng.info);
                }
            }
            return extensions;
        }

        let ext_mngs = exts.get(&location.unwrap()).unwrap().clone();

        for ext_mng in ext_mngs {
            extensions.push(ext_mng.info);
        }

        return extensions;
    }
    if let Some(name) = name {
        for (_ext_type, ext_mngs) in exts.clone() {
            for ext_mng in ext_mngs {
                if ext_mng.info.name.as_str() == name {
                    extensions.push(ext_mng.info);
                    return extensions;
                }
            }
        }
    }
    if let Some(author) = author {
        for (_ext_type, ext_mngs) in exts {
            for ext_mng in ext_mngs {
                if ext_mng.info.author.as_str() == author {
                    extensions.push(ext_mng.info);
                }
            }
        }
        return extensions;
    }

    extensions
}
