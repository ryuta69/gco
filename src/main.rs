extern crate skim;
use skim::prelude::*;
use std::io::Cursor;
use std::process::Command;

pub fn main() {
	let input = Command::new("git")
		.args(&["for-each-ref", "--format=%(refname:short)"])
		.output()
		.expect("fatal: not a git repository (or any of the parent directories): .git");
	let items = SkimItemReader::default().of_bufread(Cursor::new(input.stdout));

	let options = SkimOptionsBuilder::default()
		.height(Some("100%"))
		.multi(true)
		.reverse(true)
		.ansi(true)
		.color(Some("dark,matched:39,matched_bg:0,current:11,current_bg:0,current_match:39,current_match_bg:0,spinner:21,info:144,prompt:110,cursor:161,selected:168,header:109,border:0"))
		.preview(Some("echo {} | xargs -I% git log --color --decorate --stat %"))
		.preview_window(Some("right:50%:wrap"))
		.build()
		.unwrap();
	let selected_items = Skim::run_with(&options, Some(items))
		.map(|out| out.selected_items)
		.unwrap_or_else(|| Vec::new());

	if !selected_items.is_empty() {
		Command::new("git")
			.args(&["checkout", selected_items[0].text().as_ref().trim()])
			.spawn()
			.expect("error: pathspec did not match any file(s) known to git");
	}
}
