use color_eyre::eyre::{eyre, Report, Result};
use regex::Regex;
use std::{process::Command, thread::sleep, time::Duration};
use which::which;

pub fn attach() -> Result<(), Report> {
    // Await for a zellij session
    while !Command::new("zellij")
        .arg("list-sessions")
        .status()?
        .success()
    {
        sleep(Duration::from_secs(1));
    }

    // Rename first tab to editor
    let editor_tab_name = "editor";
    Command::new("zellij")
        .args(&["action", "rename-tab", editor_tab_name])
        .status()?;

    // Find a usable CLI editor and start it on the first tab
    let mut editor: Option<String> = None;
    if let Ok(value) = std::env::var("EDITOR") {
        if !value.trim().is_empty() {
            if value == "/usr/bin/nano" {
                let editors = ["nvim", "vim", "emacs", "helix"];
                for e in editors {
                    if let Ok(path) = which(e) {
                        editor = Some(path.to_string_lossy().to_string());
                        break;
                    }
                }
            }
            // In case no match was found (unlikely) and EDITOR is not empty
            if editor.is_none() {
                editor = Some(value);
            }
        }
    }
    Command::new("zellij")
        .args(&[
            "action",
            "write-chars",
            &format!("{}\n", editor.unwrap_or("gp".to_owned())),
        ])
        .status()?;

    let gp_tasks_list_raw = Command::new("gp")
        .args(&["tasks", "list", "--no-color"])
        .output()?;
    if !gp_tasks_list_raw.status.success() {
        return Err(eyre!("gp tasks list failed"));
    }
    let gp_tasks_list_raw = String::from_utf8(gp_tasks_list_raw.stdout)?;
    let tasks_re = Regex::new(
        r#"\|\s*(?P<task_id>.{36})\s*\|\s*(?P<task_name>.*?)\s*\|\s*(?P<task_status>.*?)\s*\|"#,
    )?;

    let mut captures = tasks_re.captures_iter(&gp_tasks_list_raw);
    captures.next(); // skip the first capture, which is the header

    for capture in captures {
        let task_id = &capture["task_id"];
        let task_name = &capture["task_name"];
        let task_status = &capture["task_status"];

        if task_status == "stopped" || task_id.is_empty() {
            continue;
        }

        if !Command::new("zellij")
            .args(&["action", "new-tab", "--name", task_name])
            .status()?
            .success()
        {
            return Err(eyre!("zellij action new-tab failed for {}", task_name));
        }

        if !Command::new("zellij")
            .args(&[
                "action",
                "write-chars",
                &format!("exec gp tasks attach {task_id}\n"),
            ])
            .status()?
            .success()
        {
            return Err(eyre!("zellij action write-chars failed for {}", task_id));
        }
    }

    // Switch back to "editor" tab
    // NOTE: We could use `zellij action go-to-tab 0` (index) here, but that might not be reliable
    Command::new("zellij")
        .args(&["action", "go-to-tab-name", editor_tab_name])
        .status()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_empty_editor_variable_handling() {
        // Test that empty EDITOR variable doesn't cause issues
        env::set_var("EDITOR", "");
        
        // This should not panic and should fall back to default
        let mut editor: Option<String> = None;
        if let Ok(value) = std::env::var("EDITOR") {
            if !value.trim().is_empty() {
                if value == "/usr/bin/nano" {
                    let editors = ["nvim", "vim", "emacs", "helix"];
                    for e in editors {
                        if let Ok(path) = which(e) {
                            editor = Some(path.to_string_lossy().to_string());
                            break;
                        }
                    }
                }
                if editor.is_none() {
                    editor = Some(value);
                }
            }
        }
        
        // With empty EDITOR, editor should remain None
        assert!(editor.is_none());
        
        // Test with whitespace-only EDITOR
        env::set_var("EDITOR", "   ");
        let mut editor: Option<String> = None;
        if let Ok(value) = std::env::var("EDITOR") {
            if !value.trim().is_empty() {
                editor = Some(value);
            }
        }
        assert!(editor.is_none());
        
        // Clean up
        env::remove_var("EDITOR");
    }
}
