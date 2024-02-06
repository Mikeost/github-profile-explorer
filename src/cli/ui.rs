use ratatui::{prelude::*, text::Text, widgets::*};
use style::palette::tailwind;
use textwrap::{wrap, Wrapper};

use crate::cli::App;

const INFO_TEXT: &str =
    "(Esc) quit | (↑) move up | (↓) move down | (→) next color | (←) previous color";

pub const PALETTES: [tailwind::Palette; 4] = [
    tailwind::BLUE,
    tailwind::EMERALD,
    tailwind::INDIGO,
    tailwind::RED,
];

pub struct TableColors {
    buffer_bg: Color,
    header_bg: Color,
    header_fg: Color,
    row_fg: Color,
    selected_style_fg: Color,
    normal_row_color: Color,
    alt_row_color: Color,
    footer_border_color: Color,
}

impl TableColors {
    pub fn new(color: &tailwind::Palette) -> Self {
        Self {
            buffer_bg: tailwind::SLATE.c950,
            header_bg: color.c900,
            header_fg: tailwind::SLATE.c200,
            row_fg: tailwind::SLATE.c200,
            selected_style_fg: color.c400,
            normal_row_color: tailwind::SLATE.c950,
            alt_row_color: tailwind::SLATE.c900,
            footer_border_color: color.c400,
        }
    }
}

pub fn ui(frame: &mut Frame, app: &mut App) {
    let rects = Layout::vertical([Constraint::Min(5), Constraint::Length(7)]).split(frame.size());

    app.set_colors();

    render_table(frame, app, rects[0]);

    render_scrollbar(frame, app, rects[0]);

    render_footer(frame, app, rects[1]);
}

fn render_table(frame: &mut Frame, app: &mut App, area: Rect) {
    let header_style = Style::default()
        .fg(app.colors.header_fg)
        .bg(app.colors.header_bg);

    let selected_style = Style::default()
        .add_modifier(Modifier::REVERSED)
        .fg(app.colors.selected_style_fg);

    let header = [
        "Name",
        "Description",
        "Topics",
        "Last update",
        "Language",
        "Star count",
        "Forks count",
    ]
    .iter()
    .map(|&s| Cell::from(Text::raw(s)).style(header_style))
    .collect::<Vec<_>>();

    let column_widths: [u16; 7] = [
        20, // Name
        50, // Description
        15, // Topics
        20, // Last update
        15, // Language
        5,  // Star count
        5,  // Forks count
    ];

    let header_row = Row::new(header);

    let rows: Vec<Row> = app
        .items
        .iter()
        .enumerate()
        .map(|(i, data)| {
            let color = match i % 2 {
                0 => app.colors.normal_row_color,
                _ => app.colors.alt_row_color,
            };
            let item = data.ref_array();

            let name = item
                .get(0)
                .map_or_else(|| String::from(""), |s| s.to_string());
            let description = item
                .get(1)
                .map_or_else(|| String::from(""), |s| s.to_string());
            let topics = item
                .get(2)
                .map_or_else(|| String::from(""), |s| s.to_string());
            let last_update = item
                .get(3)
                .map_or_else(|| String::from(""), |s| s.to_string());
            let language = item
                .get(4)
                .map_or_else(|| String::from(""), |s| s.to_string());
            let star_count = item
                .get(5)
                .map_or_else(|| String::from(""), |s| s.to_string());
            let forks_count = item
                .get(6)
                .map_or_else(|| String::from(""), |s| s.to_string());

            let cells: Vec<Cell> = vec![
                Cell::from(Text::raw(wrap_text(name, column_widths[0]))),
                Cell::from(Text::raw(wrap_text(description, column_widths[1]))),
                Cell::from(Text::raw(wrap_text(topics, column_widths[2]))),
                Cell::from(Text::raw(wrap_text(last_update, column_widths[3]))),
                Cell::from(Text::raw(wrap_text(language, column_widths[4]))),
                Cell::from(Text::raw(wrap_text(star_count, column_widths[5]))),
                Cell::from(Text::raw(wrap_text(forks_count, column_widths[6]))),
            ];

            Row::new(cells)
                .style(Style::new().fg(app.colors.row_fg).bg(color))
                .height(4)
        })
        .collect();

    let bar = " █ ";

    let t = Table::new(
        rows,
        vec![
            Constraint::Min(column_widths[0]),
            Constraint::Min(column_widths[1]),
            Constraint::Min(column_widths[2]),
            Constraint::Min(column_widths[3]),
            Constraint::Min(column_widths[4]),
            Constraint::Min(column_widths[5]),
            Constraint::Min(column_widths[6]),
        ],
    )
    .header(header_row)
    .highlight_style(selected_style)
    .highlight_symbol(Text::from(vec![
        "".into(),
        bar.into(),
        bar.into(),
        "".into(),
    ]))
    .bg(app.colors.buffer_bg)
    .highlight_spacing(HighlightSpacing::Always)
    .column_spacing(1);

    frame.render_stateful_widget(t, area, &mut app.state);
}

fn wrap_text(text: String, width: u16) -> String {
    let wrapper = Wrapper::new(width as usize).break_words(true);
    wrap(&text, width as usize)
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .join("\n")
}

fn render_scrollbar(frame: &mut Frame, app: &mut App, area: Rect) {
    frame.render_stateful_widget(
        Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(None)
            .end_symbol(None),
        area.inner(&Margin {
            vertical: 1,
            horizontal: 1,
        }),
        &mut app.scroll_state,
    );
}

fn render_footer(frame: &mut Frame, app: &mut App, area: Rect) {
    let info_footer = Paragraph::new(Line::from(INFO_TEXT))
        .style(Style::new().fg(app.colors.row_fg).bg(app.colors.buffer_bg))
        .centered()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::new().fg(app.colors.footer_border_color))
                .border_type(BorderType::Double),
        );

    frame.render_widget(info_footer, area);
}
