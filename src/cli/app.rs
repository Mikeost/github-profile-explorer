use crate::RepositoryInfo;
use ratatui::widgets::*;
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

use crate::cli::ui;

const ITEM_HEIGHT: usize = 4;

pub struct App {
    pub state: TableState,
    pub items: Vec<RepositoryInfo>,
    pub longest_item_lens: (u16, u16, u16, u16, u16, u16, u16),
    pub scroll_state: ScrollbarState,
    pub colors: ui::TableColors,
    pub color_index: usize,
}

impl App {
    pub fn new(profile_info: Vec<RepositoryInfo>) -> App {
        App {
            state: TableState::default().with_selected(0),
            longest_item_lens: constraint_len_calculator(&profile_info),
            scroll_state: ScrollbarState::new((profile_info.len() - 1) * ITEM_HEIGHT),
            colors: ui::TableColors::new(&ui::PALETTES[0]),
            color_index: 0,
            items: profile_info,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn next_color(&mut self) {
        self.color_index = (self.color_index + 1) % ui::PALETTES.len();
    }

    pub fn previous_color(&mut self) {
        let count = ui::PALETTES.len();
        self.color_index = (self.color_index + count - 1) % count;
    }

    pub fn set_colors(&mut self) {
        self.colors = ui::TableColors::new(&ui::PALETTES[self.color_index]);
    }
}

fn constraint_len_calculator(items: &Vec<RepositoryInfo>) -> (u16, u16, u16, u16, u16, u16, u16) {
    let repo_name_len = items
        .iter()
        .filter_map(|info| info.name().map(String::from))
        .flat_map(|s| {
            s.lines()
                .map(|line| UnicodeWidthStr::width(line))
                .collect::<Vec<_>>()
        })
        .max()
        .unwrap_or(0);

    let repo_description_len = items
        .iter()
        .filter_map(|info| info.description().map(String::from))
        .flat_map(|s| {
            s.lines()
                .map(|line| UnicodeWidthStr::width(line))
                .collect::<Vec<_>>()
        })
        .max()
        .unwrap_or(0);

    let topics_len = items
        .iter()
        .map(|info| info.topics())
        .filter_map(|opt| opt.map(String::from))
        .flat_map(|s| s.chars().collect::<Vec<_>>())
        .map(|c| UnicodeWidthChar::width(c).unwrap_or(0) as u32)
        .max()
        .unwrap_or_default();

    let repo_last_update_len = items
        .iter()
        .filter_map(|info| info.last_update().map(String::from))
        .flat_map(|s| {
            s.lines()
                .map(|line| UnicodeWidthStr::width(line))
                .collect::<Vec<_>>()
        })
        .max()
        .unwrap_or(0);

    let repo_language_len = items
        .iter()
        .filter_map(|info| info.language().map(String::from))
        .flat_map(|s| {
            s.lines()
                .map(|line| UnicodeWidthStr::width(line))
                .collect::<Vec<_>>()
        })
        .max()
        .unwrap_or(0);

    let stargazers_count_len = items
        .iter()
        .map(|info| info.stargazers_count().to_string())
        .flat_map(|s| {
            s.lines()
                .map(|line| UnicodeWidthStr::width(line))
                .collect::<Vec<_>>()
        })
        .max()
        .unwrap_or(0);

    let forks_count_len = items
        .iter()
        .map(|info| info.forks_count().to_string())
        .flat_map(|s| {
            s.lines()
                .map(|line| UnicodeWidthStr::width(line))
                .collect::<Vec<_>>()
        })
        .max()
        .unwrap_or(0);

    (
        repo_name_len as u16,
        repo_description_len as u16,
        topics_len as u16,
        repo_last_update_len as u16,
        repo_language_len as u16,
        stargazers_count_len as u16,
        forks_count_len as u16,
    )
}
