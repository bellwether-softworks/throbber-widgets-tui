#[cfg(feature = "ratatui")]
use ratatui as tui;

#[derive(Default)]
struct App {
    ascii_state: throbber_widgets_tui::ThrobberState,
    box_drawing_state: throbber_widgets_tui::ThrobberState,
    arrow_state: throbber_widgets_tui::ThrobberState,
    double_arrow_state: throbber_widgets_tui::ThrobberState,
    vertical_block_state: throbber_widgets_tui::ThrobberState,
    horizontal_block_state: throbber_widgets_tui::ThrobberState,
    quadrant_block_state: throbber_widgets_tui::ThrobberState,
    quadrant_block_crack_state: throbber_widgets_tui::ThrobberState,
    white_square_state: throbber_widgets_tui::ThrobberState,
    white_circle_state: throbber_widgets_tui::ThrobberState,
    black_circle_state: throbber_widgets_tui::ThrobberState,
    clock_state: throbber_widgets_tui::ThrobberState,
    braille_one_state: throbber_widgets_tui::ThrobberState,
    braille_six_state: throbber_widgets_tui::ThrobberState,
    braille_eight_state: throbber_widgets_tui::ThrobberState,
    braille_double_state: throbber_widgets_tui::ThrobberState,
    braille_six_double_state: throbber_widgets_tui::ThrobberState,
    braille_eight_double_state: throbber_widgets_tui::ThrobberState,
    ogham_a_state: throbber_widgets_tui::ThrobberState,
    ogham_b_state: throbber_widgets_tui::ThrobberState,
    ogham_c_state: throbber_widgets_tui::ThrobberState,
}

impl App {
    fn on_tick(&mut self) {
        self.ascii_state.calc_next();
        self.box_drawing_state.calc_next();
        self.arrow_state.calc_next();
        self.double_arrow_state.calc_next();
        self.vertical_block_state.calc_next();
        self.horizontal_block_state.calc_next();
        self.quadrant_block_state.calc_next();
        self.quadrant_block_crack_state.calc_next();
        self.white_square_state.calc_next();
        self.white_circle_state.calc_next();
        self.black_circle_state.calc_next();
        self.clock_state.calc_next();
        self.braille_one_state.calc_next();
        self.braille_six_state.calc_next();
        self.braille_eight_state.calc_next();
        self.braille_double_state.calc_next();
        self.braille_six_double_state.calc_next();
        self.braille_eight_double_state.calc_next();
        self.ogham_a_state.calc_next();
        self.ogham_b_state.calc_next();
        self.ogham_c_state.calc_next();
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // setup terminal
    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    crossterm::execute!(
        stdout,
        crossterm::terminal::EnterAlternateScreen,
        crossterm::event::EnableMouseCapture
    )?;
    let backend = tui::backend::CrosstermBackend::new(stdout);
    let mut terminal = tui::Terminal::new(backend)?;

    // create app and run it
    let tick_rate = std::time::Duration::from_millis(250);
    let app = App::default();
    let res = run_app(&mut terminal, app, tick_rate);

    // restore terminal
    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        crossterm::terminal::LeaveAlternateScreen,
        crossterm::event::DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: tui::backend::Backend>(
    terminal: &mut tui::Terminal<B>,
    mut app: App,
    tick_rate: std::time::Duration,
) -> std::io::Result<()> {
    let mut last_tick = std::time::Instant::now();
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| std::time::Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                if let crossterm::event::KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = std::time::Instant::now();
        }
    }
}

fn ui<B: tui::backend::Backend>(f: &mut tui::Frame<B>, app: &mut App) {
    let verticals = tui::layout::Layout::default()
        .direction(tui::layout::Direction::Vertical)
        .constraints(
            [
                tui::layout::Constraint::Length(1),
                tui::layout::Constraint::Length(1),
                tui::layout::Constraint::Length(1),
                tui::layout::Constraint::Length(1),
                tui::layout::Constraint::Length(1),
                tui::layout::Constraint::Length(1),
                tui::layout::Constraint::Length(1),
                tui::layout::Constraint::Length(1),
            ]
            .as_ref(),
        )
        .split(f.size());
    let default_throbber = throbber_widgets_tui::Throbber::default()
        .label("Press q to exit. This is a default throbber (random step).")
        .style(tui::style::Style::default().fg(tui::style::Color::Yellow));
    f.render_widget(default_throbber, verticals[0]);

    let chunks1 = tui::layout::Layout::default()
        .direction(tui::layout::Direction::Horizontal)
        .constraints(
            [
                tui::layout::Constraint::Percentage(33),
                tui::layout::Constraint::Percentage(33),
                tui::layout::Constraint::Percentage(33),
            ]
            .as_ref(),
        )
        .split(verticals[1]);
    let chunks2 = tui::layout::Layout::default()
        .direction(tui::layout::Direction::Horizontal)
        .constraints(
            [
                tui::layout::Constraint::Percentage(33),
                tui::layout::Constraint::Percentage(33),
                tui::layout::Constraint::Percentage(33),
            ]
            .as_ref(),
        )
        .split(verticals[2]);
    let chunks3 = tui::layout::Layout::default()
        .direction(tui::layout::Direction::Horizontal)
        .constraints(
            [
                tui::layout::Constraint::Percentage(33),
                tui::layout::Constraint::Percentage(33),
                tui::layout::Constraint::Percentage(33),
            ]
            .as_ref(),
        )
        .split(verticals[3]);
    let chunks4 = tui::layout::Layout::default()
        .direction(tui::layout::Direction::Horizontal)
        .constraints(
            [
                tui::layout::Constraint::Percentage(33),
                tui::layout::Constraint::Percentage(33),
                tui::layout::Constraint::Percentage(33),
            ]
            .as_ref(),
        )
        .split(verticals[4]);
    let chunks5 = tui::layout::Layout::default()
        .direction(tui::layout::Direction::Horizontal)
        .constraints(
            [
                tui::layout::Constraint::Percentage(33),
                tui::layout::Constraint::Percentage(33),
                tui::layout::Constraint::Percentage(33),
            ]
            .as_ref(),
        )
        .split(verticals[5]);
    let chunks6 = tui::layout::Layout::default()
        .direction(tui::layout::Direction::Horizontal)
        .constraints(
            [
                tui::layout::Constraint::Percentage(33),
                tui::layout::Constraint::Percentage(33),
                tui::layout::Constraint::Percentage(33),
            ]
            .as_ref(),
        )
        .split(verticals[6]);
    let chunks7 = tui::layout::Layout::default()
        .direction(tui::layout::Direction::Horizontal)
        .constraints(
            [
                tui::layout::Constraint::Percentage(33),
                tui::layout::Constraint::Percentage(33),
                tui::layout::Constraint::Percentage(33),
            ]
            .as_ref(),
        )
        .split(verticals[7]);

    let ascii_throbber = throbber_widgets_tui::Throbber::default()
        .label("ASCII Set")
        .throbber_set(throbber_widgets_tui::ASCII);
    f.render_stateful_widget(ascii_throbber, chunks1[0], &mut app.ascii_state);

    let box_drawing_throbber = throbber_widgets_tui::Throbber::default()
        .label("BOX_DRAWING Set")
        .throbber_set(throbber_widgets_tui::BOX_DRAWING);
    f.render_stateful_widget(box_drawing_throbber, chunks1[1], &mut app.box_drawing_state);

    let arrow_throbber = throbber_widgets_tui::Throbber::default()
        .label("ARROW Set")
        .throbber_set(throbber_widgets_tui::ARROW);
    f.render_stateful_widget(arrow_throbber, chunks1[2], &mut app.arrow_state);

    let double_arrow_throbber = throbber_widgets_tui::Throbber::default()
        .label("DOUBLE_ARROW Set")
        .throbber_set(throbber_widgets_tui::DOUBLE_ARROW);
    f.render_stateful_widget(
        double_arrow_throbber,
        chunks2[0],
        &mut app.double_arrow_state,
    );

    let vertical_block_throbber = throbber_widgets_tui::Throbber::default()
        .label("VERTICAL_BLOCK Set")
        .throbber_set(throbber_widgets_tui::VERTICAL_BLOCK);
    f.render_stateful_widget(
        vertical_block_throbber,
        chunks2[1],
        &mut app.vertical_block_state,
    );

    let horizontal_block_throbber = throbber_widgets_tui::Throbber::default()
        .label("HORIZONTAL_BLOCK Set")
        .throbber_set(throbber_widgets_tui::HORIZONTAL_BLOCK);
    f.render_stateful_widget(
        horizontal_block_throbber,
        chunks2[2],
        &mut app.horizontal_block_state,
    );

    let quadrant_block_throbber = throbber_widgets_tui::Throbber::default()
        .label("QUADRANT_BLOCK Set")
        .throbber_set(throbber_widgets_tui::QUADRANT_BLOCK);
    f.render_stateful_widget(
        quadrant_block_throbber,
        chunks3[0],
        &mut app.quadrant_block_state,
    );

    let quadrant_block_crack_throbber = throbber_widgets_tui::Throbber::default()
        .label("QUADRANT_BLOCK_CRACK Set")
        .throbber_set(throbber_widgets_tui::QUADRANT_BLOCK_CRACK);
    f.render_stateful_widget(
        quadrant_block_crack_throbber,
        chunks3[1],
        &mut app.quadrant_block_crack_state,
    );

    let white_square_throbber = throbber_widgets_tui::Throbber::default()
        .label("WHITE_SQUARE Set")
        .throbber_set(throbber_widgets_tui::WHITE_SQUARE);
    f.render_stateful_widget(
        white_square_throbber,
        chunks3[2],
        &mut app.white_square_state,
    );

    let white_circle_throbber = throbber_widgets_tui::Throbber::default()
        .label("WHITE_CIRCLE Set")
        .throbber_set(throbber_widgets_tui::WHITE_CIRCLE);
    f.render_stateful_widget(
        white_circle_throbber,
        chunks4[0],
        &mut app.white_circle_state,
    );

    let black_circle_throbber = throbber_widgets_tui::Throbber::default()
        .label("BLACK_CIRCLE Set")
        .throbber_set(throbber_widgets_tui::BLACK_CIRCLE);
    f.render_stateful_widget(
        black_circle_throbber,
        chunks4[1],
        &mut app.black_circle_state,
    );

    let clock_throbber = throbber_widgets_tui::Throbber::default()
        .label("CLOCK Set")
        .throbber_set(throbber_widgets_tui::CLOCK);
    f.render_stateful_widget(clock_throbber, chunks4[2], &mut app.clock_state);

    let braille_one_throbber = throbber_widgets_tui::Throbber::default()
        .label("BRAILLE_ONE Set")
        .throbber_set(throbber_widgets_tui::BRAILLE_ONE);
    f.render_stateful_widget(braille_one_throbber, chunks5[0], &mut app.braille_one_state);

    let braille_six_throbber = throbber_widgets_tui::Throbber::default()
        .label("BRAILLE_SIX Set")
        .throbber_set(throbber_widgets_tui::BRAILLE_SIX);
    f.render_stateful_widget(braille_six_throbber, chunks5[1], &mut app.braille_six_state);

    let braille_eight_throbber = throbber_widgets_tui::Throbber::default()
        .label("BRAILLE_EIGHT Set")
        .throbber_set(throbber_widgets_tui::BRAILLE_EIGHT);
    f.render_stateful_widget(
        braille_eight_throbber,
        chunks5[2],
        &mut app.braille_eight_state,
    );

    let braille_double_throbber = throbber_widgets_tui::Throbber::default()
        .label("BRAILLE_DOUBLE Set")
        .throbber_set(throbber_widgets_tui::BRAILLE_DOUBLE);
    f.render_stateful_widget(
        braille_double_throbber,
        chunks6[0],
        &mut app.braille_double_state,
    );

    let braille_six_double_throbber = throbber_widgets_tui::Throbber::default()
        .label("BRAILLE_SIX_DOUBLE Set")
        .throbber_set(throbber_widgets_tui::BRAILLE_SIX_DOUBLE);
    f.render_stateful_widget(
        braille_six_double_throbber,
        chunks6[1],
        &mut app.braille_six_double_state,
    );

    let braille_eight_double_throbber = throbber_widgets_tui::Throbber::default()
        .label("BRAILLE_EIGHT_DOUBLE Set")
        .throbber_set(throbber_widgets_tui::BRAILLE_EIGHT_DOUBLE);
    f.render_stateful_widget(
        braille_eight_double_throbber,
        chunks6[2],
        &mut app.braille_eight_double_state,
    );

    let ogham_a_throbber = throbber_widgets_tui::Throbber::default()
        .label("OGHAM_A Set")
        .throbber_set(throbber_widgets_tui::OGHAM_A);
    f.render_stateful_widget(ogham_a_throbber, chunks7[0], &mut app.ogham_a_state);

    let ogham_b_throbber = throbber_widgets_tui::Throbber::default()
        .label("OGHAM_B Set")
        .throbber_set(throbber_widgets_tui::OGHAM_B);
    f.render_stateful_widget(ogham_b_throbber, chunks7[1], &mut app.ogham_b_state);

    let ogham_c_throbber = throbber_widgets_tui::Throbber::default()
        .label("OGHAM_C Set")
        .throbber_set(throbber_widgets_tui::OGHAM_C);
    f.render_stateful_widget(ogham_c_throbber, chunks7[2], &mut app.ogham_c_state);
}
