use std::cell::RefCell;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::mpsc;
use etcetera::BaseStrategy;
use tray_item::{
    TrayItem, IconSource
};

use windows::{
    core::{HSTRING, w},
    Win32::{
        System::Com::{CoCreateInstance, CoInitialize, CLSCTX_ALL},
        UI::{
            WindowsAndMessaging::{MessageBoxW, MB_ICONERROR},
            Shell::{DesktopWallpaper, IDesktopWallpaper}
        },

    }
};

use rand::prelude::*;
use crate::config::Config;


enum Message {
    ChangeWallpaper,
    OpenConfig,
    Quit,
}

struct List {
    pub content: Vec<String>,
    pub pages: u64
}

pub struct App {
    tray: TrayItem,
    wallpaper: IDesktopWallpaper,
    channel: (mpsc::SyncSender<Message>, mpsc::Receiver<Message>),
    list: RefCell<List>,
    config: Config,
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

macro_rules! parameter {
    ($entry:expr, $link:ident, $parameter:literal) => {
        if !$entry.is_empty() {
            $link = $link + $parameter + $entry.as_str();
        }
    };
}

macro_rules! config_path {
    () => {
        etcetera::choose_base_strategy().unwrap().config_dir().join("stenbug.toml")
    };
}

pub fn report_error(e: String) {
    unsafe {
        MessageBoxW(None, &HSTRING::from(format!("{e}")), w!("Stenbug"), MB_ICONERROR);
    }
}

impl App {
    pub fn new() -> Result<Self> {
        let tray = TrayItem::new(
            "Stenbug",
            IconSource::Resource("tray-icon")
        )?;

        let wallpaper: IDesktopWallpaper = unsafe {
            CoInitialize(None).unwrap();
            CoCreateInstance(&DesktopWallpaper, None, CLSCTX_ALL).unwrap()
        };

        let channel = mpsc::sync_channel(1);

        let config = {
            let path = config_path!();
            println!("{path:?}");

            if !path.exists() {
                let config = Config::default();
                let mut file = File::create_new(path)?;
                file.write(toml::to_string(&config).unwrap().as_bytes())?;
                config
            } else {
                Config::load(path)?
            }
        };

        Ok(Self {
            tray,
            wallpaper,
            channel,
            config,
            list: RefCell::from(List {
                content: vec![],
                pages: 1
            }),
        })
    }


    pub fn choose(&self) -> Option<String> {
        let mut rng = thread_rng();

        let list = &self.list.borrow().content;
        list.choose(&mut rng).cloned()
    }

    pub fn update_list(&self) -> Result<()> {
        let mut link = "https://wallhaven.cc/api/v1/search?".to_string();

        parameter!(self.config.search.query, link, "&q=");
        parameter!(self.config.search.categories, link, "&categories=");
        parameter!(self.config.search.purity, link, "&purity=");
        parameter!(self.config.search.sorting, link, "&sorting=");
        parameter!(self.config.search.order, link, "&order=");
        parameter!(self.config.search.top_range, link, "&topRange=");
        parameter!(self.config.search.at_least, link, "&atleast=");
        parameter!(self.config.search.resolutions, link, "&resolutions=");
        parameter!(self.config.search.ratios, link, "&ratios=");
        parameter!(self.config.search.colors, link, "&colors=");
        parameter!(self.config.search.api_key, link, "&apikey=");


        let mut list = self.list.borrow_mut();
        if list.pages != 1 {
            let mut rng = thread_rng();
            let page = rng.gen_range(1..list.pages + 1);

            link = link + "&page=" + page.to_string().as_str();
        }


        println!("{link}; pages {}", list.pages);
        let text = reqwest::blocking::get(link)?.text()?;

        let parsed = json::parse(&text)?;

        let images: Vec<String> = parsed["data"]
            .members()
            .map(|m| m["path"]
                .as_str().unwrap().to_string()
            )
            .collect();

        if parsed["data"].is_empty() {
            report_error("Nothing found. Check your config file and internet connection".to_string());
        }


        if list.pages == 1 {
            let pages = parsed["meta"]["last_page"].as_u64().unwrap();
            list.pages = pages;
        }

        list.content = images;

        Ok(())
    }

    pub fn download(link: &String) -> Result<(File, PathBuf)> {
        let mut file = tempfile::NamedTempFile::new()?;

        file.write(
            &reqwest::blocking::get(link).unwrap().bytes().unwrap()
        )?;

        Ok(file.keep()?)
    }

    pub fn apply(&self, path: PathBuf) -> Result<()> {
        unsafe {
            self.wallpaper.SetWallpaper(None, &HSTRING::from(path.to_str().unwrap()))?;
        }

        Ok(())
    }

    pub fn config_changed(&self) -> Result<bool> {
        let path = config_path!();
        let mut file = File::open(path)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;

        Ok(
            &self.config != &toml::from_str::<Config>(buf.as_str())?
        )
    }

    pub fn run(mut self) -> Result<()> {
        self.update_list()?;

        let (tx, rx) = &self.channel;

        self.tray.add_label("Stenbug")?;

        let open_config_tx = tx.clone();

        self.tray.add_menu_item("Open config", move || {
            open_config_tx.send(Message::OpenConfig).unwrap()
        })?;

        let change_wallpaper_tx = tx.clone();

        self.tray.add_menu_item("Change wallpaper", move || {
            change_wallpaper_tx.send(Message::ChangeWallpaper).unwrap()
        })?;

        let quit_tx = tx.clone();

        self.tray.add_menu_item("Quit", move || {
            quit_tx.send(Message::Quit).unwrap()
        })?;


        let timer_tx = tx.clone();

        std::thread::spawn(move || {
            loop {
                timer_tx.send(Message::ChangeWallpaper).unwrap();
                std::thread::sleep(self.config.system.duration);
            }
        });

        'main: loop {
            match rx.recv() {
                Ok(Message::ChangeWallpaper) => {
                    // In case of config change...
                    if self.config_changed()? {
                        // we reload config
                        self.config = Config::load(config_path!())?;
                        // and reset the page number to one
                        // since new search query might not have the same number
                        // of pages
                        self.list.borrow_mut().pages = 1;
                    }
                    self.update_list()?;
                    if let Some(link) = self.choose() {
                        let (_, path) = Self::download(&link)?;
                        self.apply(path)?;
                    }
                },
                Ok(Message::OpenConfig) => {
                    let path = config_path!();

                    std::thread::spawn(move || {
                        let _ = std::process::Command::new("notepad")
                            .arg(path.as_os_str())
                            .status();
                    });
                }
                Ok(Message::Quit) => break 'main,
                _ => {}
            }
        }

        Ok(())
    }
}