use std::{
    env::current_dir,
    fs::{create_dir_all, read_dir, read_to_string, File},
    io::Write,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
pub struct Config {
    /// 房间 id
    pub room_id: u64,
    /// 黑暗模式
    pub dark_mode: bool,
}

impl Config {
    /// 初始化配置
    fn new() -> Self {
        let config = Config { room_id: "4553086".parse().unwrap(), dark_mode: true };
        config.update();
        config
    }

    /// 读取配置
    pub fn read() -> Self {
        let config = Path::new("config.json");
        if config.exists() {
            let config = read_to_string(config).unwrap();
            if let Ok(config) = serde_json::from_str::<Config>(&config) {
                config
            } else {
                Self::new()
            }
        } else {
            Self::new()
        }
    }

    /// 更新配置
    pub fn update(&self) {
        let mut file = File::create("config.json").unwrap();
        let config_str = serde_json::to_string_pretty(self).unwrap();
        file.write_all(config_str.as_bytes()).ok();
    }

    /// 获取字体列表
    pub fn fonts() -> Vec<PathBuf> {
        // 获得根目录
        let root = current_dir().unwrap();
        // 字体目录
        let fonts = root.join("resources").join("fonts");

        if !fonts.exists() {
            create_dir_all(&fonts).ok();
            return vec![];
        }

        get_font_paths(&fonts)
    }
}

/// 递归读取子目录
fn get_font_paths(dir: &Path) -> Vec<PathBuf> {
    let mut paths = vec![];

    // 递归读取子目录
    for entry in read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            paths.extend(get_font_paths(&path));
        } else if path.extension() == Some("otf".as_ref()) {
            paths.push(path);
        }
    }

    paths
}
