use crate::shell::Shell;
use anyhow::Result;

pub fn main(shell: &Shell) -> Result<()> {
    let content = match shell {
        Shell::Bash => include_str!("../../shell/navi.plugin.bash"),
        Shell::Zsh => include_str!("../../shell/navi.plugin.zsh"),
        Shell::Fish => include_str!("../../shell/navi.plugin.fish"),
        Shell::Elvish => include_str!("../../shell/navi.plugin.elv"),
        Shell::PowerShell => include_str!("../../shell/navi.plugin.ps1"),
    };

    println!("{}", content);

    Ok(())
}
