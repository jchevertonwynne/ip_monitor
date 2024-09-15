use anyhow::Context;

const PATH: &str = "/home/jcw/projects/my_ip/README.md";

fn main() -> anyhow::Result<()> {
    let external_ip = get_external_ip()?;

    let stored_ip = get_stored_ip()?;

    println!("stored ip = {stored_ip} external ip = {external_ip}");

    if external_ip == stored_ip {
        return Ok(());
    }

    update_stored_ip(external_ip)?;

    Ok(())
}

fn get_external_ip() -> anyhow::Result<String> {
    ureq::get("https://api.ipify.org")
        .call()
        .context("failed to perform ip lookup request")?
        .into_string()
        .context("failed to read response body")
}

fn get_stored_ip() -> anyhow::Result<String> {
    std::fs::read_to_string(PATH).context("failed to read stored ip file")
}

fn update_stored_ip(ip: String) -> anyhow::Result<()> {
    std::fs::write(PATH, ip.as_bytes()).context("failed to update the stored ip file")?;

    println!("updated README.md");

    let cwd = std::path::Path::new(PATH)
        .parent()
        .context("should remove filename from path")?;
    std::env::set_current_dir(cwd)?;

    let out = String::from_utf8(
        std::process::Command::new("git")
            .args(["add", "README.md"])
            .output()
            .context("failed to add README.md update to git")?
            .stdout,
    )?;
    println!("{out}");

    let out = String::from_utf8(
        std::process::Command::new("git")
            .args(["commit", "-m", "automated ip update"])
            .output()
            .context("failed to commit README.md update to git")?
            .stdout,
    )?;
    println!("{out}");

    let out = String::from_utf8(
        std::process::Command::new("git")
            .args(["push"])
            .output()
            .context("failed to push README.md update to git")?
            .stdout,
    )?;
    println!("{out}");

    Ok(())
}
