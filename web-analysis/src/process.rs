use std::path::Path;
use colored::Colorize;

pub fn process_all_snapshots(root_directory: &Path) {
    let _ = wax::Glob::new("**/source.snapshot.html")
        .unwrap()
        .walk(&root_directory)
        .into_iter()
        .map(|x| x.unwrap())
        .filter(|x| x.file_type().is_file())
        .map(|x| x.path().to_path_buf())
        .for_each(|snapshot_path| {
            process_snapshot(&snapshot_path)
        });
}

pub fn process_snapshot(snapshot_path: &Path) {
    eprintln!("{}", format!(
        "» {:?}",
        snapshot_path
    ).cyan());
    passes(snapshot_path)
}

fn passes(snapshot_path: &Path) {
    let snapshot_html = crate::pass::simplify::open_parse(snapshot_path).unwrap_unchecked();
    let directory = snapshot_path.parent().unwrap();
    {
        let html = snapshot_html.format_document_pretty();
        let out_path = directory.join("downstream.original.html");
        std::fs::write(out_path, html).unwrap()
    }
    {
        let html = crate::pass::simplify::to_normalized(snapshot_html.clone());
        let html = html.format_document_pretty();
        let out_path = directory.join("downstream.normalized.html");
        std::fs::write(out_path, html).unwrap()
    }
    {
        let html = crate::pass::simplify::to_text_tree(snapshot_html.clone());
        let html = html.format_document_pretty();
        let out_path = directory.join("downstream.text_tree.html");
        std::fs::write(out_path, html).unwrap()
    }
    {
        let html = crate::pass::simplify::to_plain_text(snapshot_html.clone());
        let out_path = directory.join("downstream.plain_text.html");
        std::fs::write(out_path, html).unwrap()
    }
    {
        let metadata = crate::pass::metadata::compile_report(&snapshot_html).unwrap();
        let metadata_str = serde_json::to_string_pretty(&metadata).unwrap();
        let out_path = directory.join("downstream.metadata.json");
        std::fs::write(out_path, metadata_str).unwrap()
    }
}

