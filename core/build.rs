use tauri_build::{DefaultPermissionRule, InlinedPlugin};

fn main() {
    let attrs = tauri_build::Attributes::new()
        .plugin(
            "notes",
            InlinedPlugin::new()
                .commands(&[
                    "page_notes",
                    "update_note",
                    "note_detail",
                    "search_notes",
                    "create_note",
                    "remove_note",
                ])
                .default_permission(DefaultPermissionRule::AllowAllCommands),
        )
        .plugin(
            "tags",
            InlinedPlugin::new()
                .commands(&[
                    "search_tags",
                    "create_tag",
                    "recent_tags",
                    "page_notes_by_tag",
                    "delete_tag",
                ])
                .default_permission(DefaultPermissionRule::AllowAllCommands),
        )
        .plugin(
            "note-tags",
            InlinedPlugin::new()
                .commands(&["query_tag_used_times", "add_note_tag", "delete_note_tag"])
                .default_permission(DefaultPermissionRule::AllowAllCommands),
        )
        .plugin(
            "notebooks",
            InlinedPlugin::new()
                .commands(&["page_notebooks", "create_notebook", "remove_notebook"])
                .default_permission(DefaultPermissionRule::AllowAllCommands),
        );

    tauri_build::try_build(attrs).expect("failed to run tauri build script");
}
