use crate::actor;
use crate::cheatsh;
use crate::extractor;
use crate::filesystem;
use crate::finder::structures::{Opts as FinderOpts, SuggestionType};
use crate::finder::Finder;
use crate::structures::cheat::VariableMap;
use crate::structures::config::Config;
use crate::structures::config::Source;
use crate::structures::fetcher::Fetcher;
use crate::tldr;
use crate::welcome;
use anyhow::Context;
use anyhow::Result;

fn gen_core_finder_opts(config: &Config) -> Result<FinderOpts> {
    let opts = FinderOpts {
        preview: Some(format!("{} preview {{}}", filesystem::exe_string()?)),
        overrides: config.fzf_overrides.clone(),
        suggestion_type: SuggestionType::SnippetSelection,
        query: if config.best_match {
            None
        } else {
            config.get_query()
        },
        filter: if config.best_match {
            config.get_query()
        } else {
            None
        },
        ..Default::default()
    };

    Ok(opts)
}

pub fn main(config: Config) -> Result<()> {
    let opts = gen_core_finder_opts(&config).context("Failed to generate finder options")?;

    let (raw_selection, variables, files) = config
        .finder
        .call(opts, |stdin, files| {
            let fetcher: Box<dyn Fetcher> = match config.source() {
                Source::Cheats(query) => Box::new(cheatsh::Fetcher::new(query)),
                Source::Tldr(query) => Box::new(tldr::Fetcher::new(query)),
                Source::Filesystem(path, rules) => Box::new(filesystem::Fetcher::new(path, rules)),
            };

            let res = fetcher
                .fetch(stdin, files)
                .context("Failed to parse variables intended for finder")?;

            if let Some(variables) = res {
                Ok(Some(variables))
            } else {
                welcome::populate_cheatsheet(stdin);
                Ok(Some(VariableMap::new()))
            }
        })
        .context("Failed getting selection and variables from finder")?;

    let extractions = extractor::extract_from_selections(&raw_selection, config.best_match);

    if extractions.is_err() {
        return main(config);
    }

    actor::act(extractions, config, files, variables)?;

    Ok(())
}