mod common;

use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;
use themectl::cli::{Cli, Commands, VariantCommands, ConfigCommands, BackupCommands};
use common::*;

fn create_test_theme_file(temp_dir: &TempDir, name: &str) -> PathBuf {
    let themes_dir = temp_dir.path();
    let theme_path = themes_dir.join(format!("{}.toml", name));
    
    let toml_content = r##"
name = "test-theme"
description = "Test theme"

[colors]
bg = "#282828"
fg = "#ebdbb2"
accent = "#fe8019"
red = "#cc241d"
green = "#98971a"
yellow = "#d79921"
blue = "#458588"
magenta = "#b16286"
cyan = "#689d6a"
"##;
    
    fs::write(&theme_path, toml_content).unwrap();
    theme_path
}

#[test]
fn test_cli_list_themes() {
    let temp_dir = create_temp_themes_dir();
    let themes_dir = get_themes_path(&temp_dir);
    
    create_test_theme_file(&temp_dir, "theme1");
    create_test_theme_file(&temp_dir, "theme2");
    
    let cli = Cli {
        command: Commands::List,
        themes_dir: Some(themes_dir.clone()),
        dry_run: false,
    };
    
    let result = cli.execute();
    assert!(result.is_ok());
}

#[test]
fn test_cli_list_empty_directory() {
    let temp_dir = create_temp_themes_dir();
    let themes_dir = get_themes_path(&temp_dir);
    
    let cli = Cli {
        command: Commands::List,
        themes_dir: Some(themes_dir.clone()),
        dry_run: false,
    };
    
    let result = cli.execute();
    assert!(result.is_ok());
}

#[test]
fn test_cli_validate_theme() {
    let temp_dir = create_temp_themes_dir();
    let theme_path = create_test_theme_file(&temp_dir, "valid-theme");
    
    let cli = Cli {
        command: Commands::Validate {
            path: theme_path,
        },
        themes_dir: None,
        dry_run: false,
    };
    
    let result = cli.execute();
    assert!(result.is_ok());
}

#[test]
fn test_cli_validate_invalid_theme() {
    let temp_dir = create_temp_themes_dir();
    let themes_dir = temp_dir.path();
    let invalid_path = themes_dir.join("invalid.toml");
    
    fs::write(&invalid_path, "invalid toml content").unwrap();
    
    let cli = Cli {
        command: Commands::Validate {
            path: invalid_path,
        },
        themes_dir: None,
        dry_run: false,
    };
    
    let result = cli.execute();
    assert!(result.is_err());
}

#[test]
fn test_cli_export_theme() {
    let temp_dir = create_temp_themes_dir();
    let themes_dir = get_themes_path(&temp_dir);
    create_test_theme_file(&temp_dir, "export-theme");
    
    let cli = Cli {
        command: Commands::Export {
            theme: "export-theme".to_string(),
            format: "kitty".to_string(),
            output: None,
            all: false,
        },
        themes_dir: Some(themes_dir),
        dry_run: false,
    };
    
    let result = cli.execute();
    assert!(result.is_ok());
}

#[test]
fn test_cli_export_theme_to_file() {
    let temp_dir = create_temp_themes_dir();
    let themes_dir = get_themes_path(&temp_dir);
    create_test_theme_file(&temp_dir, "export-theme");
    
    let output_path = temp_dir.path().join("output.conf");
    
    let cli = Cli {
        command: Commands::Export {
            theme: "export-theme".to_string(),
            format: "kitty".to_string(),
            output: Some(output_path.clone()),
            all: false,
        },
        themes_dir: Some(themes_dir),
        dry_run: false,
    };
    
    let result = cli.execute();
    assert!(result.is_ok());
    assert!(output_path.exists());
}

#[test]
fn test_cli_export_all_formats() {
    let temp_dir = create_temp_themes_dir();
    let themes_dir = get_themes_path(&temp_dir);
    create_test_theme_file(&temp_dir, "export-theme");
    
    let output_dir = temp_dir.path().join("output");
    fs::create_dir(&output_dir).unwrap();
    
    let cli = Cli {
        command: Commands::Export {
            theme: "export-theme".to_string(),
            format: "all".to_string(),
            output: Some(output_dir.clone()),
            all: false,
        },
        themes_dir: Some(themes_dir),
        dry_run: false,
    };
    
    let result = cli.execute();
    assert!(result.is_ok());
}

#[test]
fn test_cli_export_all_flag() {
    let temp_dir = create_temp_themes_dir();
    let themes_dir = get_themes_path(&temp_dir);
    create_test_theme_file(&temp_dir, "export-theme");
    
    let output_dir = temp_dir.path().join("output");
    fs::create_dir(&output_dir).unwrap();
    
    let cli = Cli {
        command: Commands::Export {
            theme: "export-theme".to_string(),
            format: "kitty".to_string(),
            output: Some(output_dir.clone()),
            all: true,
        },
        themes_dir: Some(themes_dir),
        dry_run: false,
    };
    
    let result = cli.execute();
    assert!(result.is_ok());
}

#[test]
fn test_cli_show_theme() {
    let temp_dir = create_temp_themes_dir();
    let themes_dir = get_themes_path(&temp_dir);
    create_test_theme_file(&temp_dir, "show-theme");
    
    let cli = Cli {
        command: Commands::Show {
            theme: "show-theme".to_string(),
        },
        themes_dir: Some(themes_dir),
        dry_run: false,
    };
    
    let result = cli.execute();
    assert!(result.is_ok());
}

#[test]
fn test_cli_show_nonexistent_theme() {
    let temp_dir = create_temp_themes_dir();
    let themes_dir = get_themes_path(&temp_dir);
    
    let cli = Cli {
        command: Commands::Show {
            theme: "nonexistent".to_string(),
        },
        themes_dir: Some(themes_dir),
        dry_run: false,
    };
    
    let result = cli.execute();
    assert!(result.is_err());
}

#[test]
fn test_cli_preview_theme() {
    let temp_dir = create_temp_themes_dir();
    let themes_dir = get_themes_path(&temp_dir);
    create_test_theme_file(&temp_dir, "preview-theme");
    
    let cli = Cli {
        command: Commands::Preview {
            theme: "preview-theme".to_string(),
            format: None,
        },
        themes_dir: Some(themes_dir),
        dry_run: false,
    };
    
    let result = cli.execute();
    assert!(result.is_ok());
}

#[test]
fn test_cli_preview_theme_with_format() {
    let temp_dir = create_temp_themes_dir();
    let themes_dir = get_themes_path(&temp_dir);
    create_test_theme_file(&temp_dir, "preview-theme");
    
    let cli = Cli {
        command: Commands::Preview {
            theme: "preview-theme".to_string(),
            format: Some("kitty".to_string()),
        },
        themes_dir: Some(themes_dir),
        dry_run: false,
    };
    
    let result = cli.execute();
    assert!(result.is_ok());
}

#[test]
fn test_cli_delete_theme() {
    let temp_dir = create_temp_themes_dir();
    let _themes_dir = get_themes_path(&temp_dir);
    let theme_path = create_test_theme_file(&temp_dir, "delete-theme");
    
    // Note: delete_theme is interactive, so we test that the file exists first
    assert!(theme_path.exists());
    
    // We can't easily test interactive commands, so we'll just verify the structure
    // In a real scenario, you'd mock stdin or use a different approach
}

#[test]
fn test_cli_rename_theme() {
    let temp_dir = create_temp_themes_dir();
    let themes_dir = get_themes_path(&temp_dir);
    create_test_theme_file(&temp_dir, "old-theme");
    
    let cli = Cli {
        command: Commands::Rename {
            old: "old-theme".to_string(),
            new: "new-theme".to_string(),
        },
        themes_dir: Some(themes_dir.clone()),
        dry_run: false,
    };
    
    let result = cli.execute();
    assert!(result.is_ok());
    
    // Check that new file exists and old doesn't
    let new_path = themes_dir.join("new-theme.toml");
    let old_path = themes_dir.join("old-theme.toml");
    assert!(new_path.exists());
    assert!(!old_path.exists());
}

#[test]
fn test_cli_duplicate_theme() {
    let temp_dir = create_temp_themes_dir();
    let themes_dir = get_themes_path(&temp_dir);
    create_test_theme_file(&temp_dir, "source-theme");
    
    let cli = Cli {
        command: Commands::Duplicate {
            theme: "source-theme".to_string(),
            new: "duplicated-theme".to_string(),
        },
        themes_dir: Some(themes_dir.clone()),
        dry_run: false,
    };
    
    let result = cli.execute();
    assert!(result.is_ok());
    
    // Check that both files exist
    let source_path = themes_dir.join("source-theme.toml");
    let dup_path = themes_dir.join("duplicated-theme.toml");
    assert!(source_path.exists());
    assert!(dup_path.exists());
}

#[test]
fn test_cli_search_themes() {
    let temp_dir = create_temp_themes_dir();
    let themes_dir = get_themes_path(&temp_dir);
    create_test_theme_file(&temp_dir, "searchable-theme");
    
    let cli = Cli {
        command: Commands::Search {
            query: "searchable".to_string(),
        },
        themes_dir: Some(themes_dir),
        dry_run: false,
    };
    
    let result = cli.execute();
    assert!(result.is_ok());
}

#[test]
fn test_cli_search_no_results() {
    let temp_dir = create_temp_themes_dir();
    let themes_dir = get_themes_path(&temp_dir);
    create_test_theme_file(&temp_dir, "theme1");
    
    let cli = Cli {
        command: Commands::Search {
            query: "nonexistent-query".to_string(),
        },
        themes_dir: Some(themes_dir),
        dry_run: false,
    };
    
    let result = cli.execute();
    assert!(result.is_ok());
}

#[test]
fn test_cli_validate_all() {
    let temp_dir = create_temp_themes_dir();
    let themes_dir = get_themes_path(&temp_dir);
    create_test_theme_file(&temp_dir, "theme1");
    create_test_theme_file(&temp_dir, "theme2");
    
    let cli = Cli {
        command: Commands::ValidateAll,
        themes_dir: Some(themes_dir),
        dry_run: false,
    };
    
    let result = cli.execute();
    assert!(result.is_ok());
}

#[test]
fn test_cli_export_all_themes() {
    let temp_dir = create_temp_themes_dir();
    let themes_dir = get_themes_path(&temp_dir);
    create_test_theme_file(&temp_dir, "theme1");
    create_test_theme_file(&temp_dir, "theme2");
    
    let output_dir = temp_dir.path().join("exports");
    fs::create_dir(&output_dir).unwrap();
    
    let cli = Cli {
        command: Commands::ExportAll {
            format: "kitty".to_string(),
            output_dir: output_dir.clone(),
        },
        themes_dir: Some(themes_dir),
        dry_run: false,
    };
    
    let result = cli.execute();
    assert!(result.is_ok());
}

#[test]
fn test_cli_variant_create() {
    let temp_dir = create_temp_themes_dir();
    let themes_dir = get_themes_path(&temp_dir);
    create_test_theme_file(&temp_dir, "base-theme");
    
    let cli = Cli {
        command: Commands::Variant {
            command: VariantCommands::Create {
                theme: "base-theme".to_string(),
                variant: "light".to_string(),
                auto: true, // Auto-generate to avoid interactive prompts
            },
        },
        themes_dir: Some(themes_dir.clone()),
        dry_run: false,
    };
    
    let result = cli.execute();
    assert!(result.is_ok());
    
    // Check that variant file was created
    let variant_path = themes_dir.join("base-theme-light.toml");
    assert!(variant_path.exists());
}

#[test]
fn test_cli_variant_switch() {
    let temp_dir = create_temp_themes_dir();
    let themes_dir = get_themes_path(&temp_dir);
    create_test_theme_file(&temp_dir, "base-theme");
    
    // First create a variant
    let create_cli = Cli {
        command: Commands::Variant {
            command: VariantCommands::Create {
                theme: "base-theme".to_string(),
                variant: "light".to_string(),
                auto: true,
            },
        },
        themes_dir: Some(themes_dir.clone()),
        dry_run: false,
    };
    create_cli.execute().unwrap();
    
    // Then switch to it
    let switch_cli = Cli {
        command: Commands::Variant {
            command: VariantCommands::Switch {
                theme: "base-theme".to_string(),
                variant: "light".to_string(),
            },
        },
        themes_dir: Some(themes_dir.clone()),
        dry_run: false,
    };
    
    let result = switch_cli.execute();
    assert!(result.is_ok());
}

#[test]
fn test_cli_variant_list() {
    let temp_dir = create_temp_themes_dir();
    let themes_dir = get_themes_path(&temp_dir);
    create_test_theme_file(&temp_dir, "base-theme");
    
    // Create a variant first
    let create_cli = Cli {
        command: Commands::Variant {
            command: VariantCommands::Create {
                theme: "base-theme".to_string(),
                variant: "light".to_string(),
                auto: true,
            },
        },
        themes_dir: Some(themes_dir.clone()),
        dry_run: false,
    };
    create_cli.execute().unwrap();
    
    // List variants
    let list_cli = Cli {
        command: Commands::Variant {
            command: VariantCommands::List {
                theme: "base-theme".to_string(),
            },
        },
        themes_dir: Some(themes_dir),
        dry_run: false,
    };
    
    let result = list_cli.execute();
    assert!(result.is_ok());
}

#[test]
fn test_cli_config_set_path() {
    let cli = Cli {
        command: Commands::Config {
            command: ConfigCommands::SetPath {
                app: "kitty".to_string(),
                path: PathBuf::from("/tmp/test.conf"),
            },
        },
        themes_dir: None,
        dry_run: false,
    };
    
    let result = cli.execute();
    // May succeed or fail depending on config file permissions
    // Just verify it doesn't panic
    let _ = result;
}

#[test]
fn test_cli_config_get_path() {
    let cli = Cli {
        command: Commands::Config {
            command: ConfigCommands::GetPath {
                app: "kitty".to_string(),
            },
        },
        themes_dir: None,
        dry_run: false,
    };
    
    let result = cli.execute();
    // May succeed or fail depending on config
    // Just verify it doesn't panic
    let _ = result;
}

#[test]
fn test_cli_backups_list() {
    let cli = Cli {
        command: Commands::Backups {
            command: BackupCommands::List {
                config_dir: None,
            },
        },
        themes_dir: None,
        dry_run: false,
    };
    
    let result = cli.execute();
    // Should succeed even if no backups exist
    assert!(result.is_ok());
}

#[test]
fn test_cli_init() {
    let temp_dir = create_temp_themes_dir();
    let themes_dir = get_themes_path(&temp_dir);
    
    let cli = Cli {
        command: Commands::Init,
        themes_dir: Some(themes_dir.clone()),
        dry_run: false,
    };
    
    let result = cli.execute();
    assert!(result.is_ok());
    
    // Check that themes directory exists
    assert!(themes_dir.exists());
}

#[test]
fn test_cli_apply_theme_dry_run() {
    let temp_dir = create_temp_themes_dir();
    let themes_dir = get_themes_path(&temp_dir);
    create_test_theme_file(&temp_dir, "apply-theme");
    
    let cli = Cli {
        command: Commands::Apply {
            theme: "apply-theme".to_string(),
            config_dir: None,
            apps: None,
            variant: None,
        },
        themes_dir: Some(themes_dir),
        dry_run: true, // Dry run should not modify files
    };
    
    let result = cli.execute();
    // Should succeed in dry run mode
    assert!(result.is_ok());
}

#[test]
fn test_cli_apply_theme_with_apps() {
    let temp_dir = create_temp_themes_dir();
    let themes_dir = get_themes_path(&temp_dir);
    create_test_theme_file(&temp_dir, "apply-theme");
    
    let cli = Cli {
        command: Commands::Apply {
            theme: "apply-theme".to_string(),
            config_dir: None,
            apps: Some("kitty,waybar".to_string()),
            variant: None,
        },
        themes_dir: Some(themes_dir),
        dry_run: true,
    };
    
    let result = cli.execute();
    assert!(result.is_ok());
}

#[test]
fn test_cli_apply_theme_with_variant() {
    let temp_dir = create_temp_themes_dir();
    let themes_dir = get_themes_path(&temp_dir);
    create_test_theme_file(&temp_dir, "base-theme");
    
    // Create variant first
    let create_cli = Cli {
        command: Commands::Variant {
            command: VariantCommands::Create {
                theme: "base-theme".to_string(),
                variant: "light".to_string(),
                auto: true,
            },
        },
        themes_dir: Some(themes_dir.clone()),
        dry_run: false,
    };
    create_cli.execute().unwrap();
    
    // Apply variant
    let apply_cli = Cli {
        command: Commands::Apply {
            theme: "base-theme".to_string(),
            config_dir: None,
            apps: None,
            variant: Some("light".to_string()),
        },
        themes_dir: Some(themes_dir),
        dry_run: true,
    };
    
    let result = apply_cli.execute();
    assert!(result.is_ok());
}

#[test]
fn test_cli_export_unknown_format() {
    let temp_dir = create_temp_themes_dir();
    let themes_dir = get_themes_path(&temp_dir);
    create_test_theme_file(&temp_dir, "export-theme");
    
    let cli = Cli {
        command: Commands::Export {
            theme: "export-theme".to_string(),
            format: "unknown-format".to_string(),
            output: None,
            all: false,
        },
        themes_dir: Some(themes_dir),
        dry_run: false,
    };
    
    let result = cli.execute();
    assert!(result.is_err());
}

#[test]
fn test_cli_export_nonexistent_theme() {
    let temp_dir = create_temp_themes_dir();
    let themes_dir = get_themes_path(&temp_dir);
    
    let cli = Cli {
        command: Commands::Export {
            theme: "nonexistent".to_string(),
            format: "kitty".to_string(),
            output: None,
            all: false,
        },
        themes_dir: Some(themes_dir),
        dry_run: false,
    };
    
    let result = cli.execute();
    assert!(result.is_err());
}
