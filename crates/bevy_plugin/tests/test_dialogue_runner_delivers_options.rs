use anyhow::Result;
use bevy::prelude::*;
use bevy_yarn_slinger::prelude::*;
use std::fs;
use tempfile::tempdir;
use utils::prelude::*;

mod utils;

#[test]
fn errs_on_selection_without_start() -> Result<()> {
    let mut app = App::new();
    setup_dialogue_runner(&mut app)
        .select_option(OptionId(0))
        .unwrap_err();

    Ok(())
}

#[test]
fn delivers_option_set() -> Result<()> {
    let mut app = App::new();
    setup_dialogue_runner(&mut app).start();
    app.continue_dialogue_and_update_n_times(4);
    assert_events!(app contains [
        PresentLineEvent (n = 0),
        PresentOptionsEvent with |event| event.options.len() == 2,
        PresentOptionsEvent with |event| event.options.iter().all(|o| o.is_available),
    ]);

    Ok(())
}

#[test]
fn errs_on_unexpected_selection_timing() -> Result<()> {
    let mut app = App::new();
    setup_dialogue_runner(&mut app).start();
    app.continue_dialogue_and_update_n_times(3);
    app.dialogue_runner_mut()
        .select_option(OptionId(0))
        .unwrap_err();

    Ok(())
}
#[test]
fn errs_on_unexpected_selection_value() -> Result<()> {
    let mut app = App::new();
    setup_dialogue_runner(&mut app).start();
    app.continue_dialogue_and_update_n_times(4);
    app.dialogue_runner_mut()
        .select_option(OptionId(2))
        .unwrap_err();

    Ok(())
}

#[test]
fn option_selection_implies_continue() -> Result<()> {
    let mut app = App::new();
    setup_dialogue_runner(&mut app).start();
    app.continue_dialogue_and_update_n_times(4);
    app.dialogue_runner_mut().select_option(OptionId(0))?;
    app.update();
    assert_events!(app contains [
        PresentLineEvent with |event| event.line.text == lines()[6],
        PresentOptionsEvent (n = 0),
    ]);

    Ok(())
}

#[test]
fn can_show_option_selection_as_line() -> Result<()> {
    let mut app = App::new();
    {
        let mut dialogue_runner = setup_dialogue_runner(&mut app);
        dialogue_runner.start();
        dialogue_runner.run_selected_options_as_lines = true;
    }
    app.continue_dialogue_and_update_n_times(4);
    assert_events!(app contains [
        PresentLineEvent (n = 0),
        PresentOptionsEvent with |event| event.options.len() == 2,
        PresentOptionsEvent with |event| event.options.iter().all(|o| o.is_available),
    ]);
    app.dialogue_runner_mut().select_option(OptionId(0))?;
    app.update();
    assert_events!(app contains [
        PresentLineEvent with |event| event.line.text == "You: Never ever ever?",
        PresentOptionsEvent (n = 0),
    ]);
    app.continue_dialogue_and_update();
    assert_events!(app contains [
        PresentLineEvent with |event| event.line.text == lines()[6],
        PresentOptionsEvent (n = 0),
    ]);

    Ok(())
}

#[test]
fn can_jump_around_nodes() -> Result<()> {
    let mut app = App::new();
    setup_dialogue_runner(&mut app).start();
    app.continue_dialogue_and_update_n_times(4);
    app.dialogue_runner_mut().select_option(OptionId(1))?;
    app.update();
    assert_events!(app contains [
        PresentLineEvent with |event| event.line.text == lines()[10],
        PresentOptionsEvent (n = 0),
    ]);
    app.continue_dialogue_and_update();
    assert_events!(app contains [
        PresentLineEvent (n = 0),
        PresentOptionsEvent with |event| event.options.len() == 3,
    ]);

    Ok(())
}

#[test]
fn can_select_unavailable_choice() -> Result<()> {
    let mut app = App::new();
    setup_dialogue_runner(&mut app).start();
    app.continue_dialogue_and_update_n_times(4);
    app.dialogue_runner_mut().select_option(OptionId(0))?;
    app.continue_dialogue_and_update_n_times(2);
    assert_events!(app contains [
        PresentLineEvent (n = 0),
        PresentOptionsEvent with |event| event.options.len() == 2,
        PresentOptionsEvent with |event| event.options.iter().filter(|o| o.is_available).count() == 1,
        PresentOptionsEvent with |event| event.options.iter().filter(|o| !o.is_available).all(|o| o.id == OptionId(0)),
    ]);
    app.dialogue_runner_mut().select_option(OptionId(0))?;
    app.update();
    assert_events!(app contains [
        PresentLineEvent with |event| event.line.text == lines()[6],
        PresentOptionsEvent (n = 0),
    ]);

    Ok(())
}

#[test]
fn generates_files_in_dev_mode() -> Result<()> {
    let dir = tempdir()?;
    let original_yarn_path = project_root_path().join("assets/tests/options.yarn");
    let yarn_path = dir.path().join("options.yarn");
    fs::copy(original_yarn_path, yarn_path)?;
    let mut app = App::new();
    setup_default_plugins_for_path(&mut app, dir.path());

    setup_dialogue_runner_in_dev_mode(&mut app).start();
    app.update();
    assert_events!(app contains [
        PresentLineEvent with |event| event.line.text == lines()[0],
    ]);

    Ok(())
}

fn setup_dialogue_runner(app: &mut App) -> Mut<DialogueRunner> {
    setup_default_plugins(app)
        .add_plugin(YarnSlingerPlugin::with_yarn_files(vec!["options.yarn"]))
        .dialogue_runner_mut()
}

fn setup_dialogue_runner_in_dev_mode(app: &mut App) -> Mut<DialogueRunner> {
    app.add_plugin(
        YarnSlingerPlugin::with_yarn_files(vec!["options.yarn"]).with_localizations(
            Localizations {
                base_localization: "en-US".into(),
                translations: vec!["de-CH".into()],
                file_generation_mode: FileGenerationMode::Development,
            },
        ),
    )
    .dialogue_runner_mut()
}

fn lines() -> Vec<String> {
    let mut lines: Vec<_> = include_str!("../assets/tests/options.yarn")
        .lines()
        .filter(|l| !l.starts_with("title:"))
        .filter(|l| !l.starts_with("position:"))
        .filter(|l| !l.starts_with("---"))
        .filter(|l| !l.starts_with("==="))
        .filter(|l| !l.is_empty())
        .map(|line| line.trim().to_owned())
        .collect();
    lines.pop();
    lines
}
