use std::env;
use std::path::PathBuf;

use rand::RngCore;
use uuid::Uuid;

pub fn random_bytes<const N: usize>() -> [u8; N] {
    let mut ret = [0u8; N];

    rand::thread_rng().fill_bytes(&mut ret);

    ret
}

pub fn uuid_v7() -> Uuid {
    Uuid::now_v7()
}

pub fn uuid_v4() -> String {
    Uuid::new_v4().as_simple().to_string()
}

pub fn has_git_dir(path: &str) -> bool {
    let mut gitdir = PathBuf::from(path);
    gitdir.push(".git");

    gitdir.exists()
}

// detect if any parent dir has a git repo in it
// I really don't want to bring in libgit for something simple like this
// If we start to do anything more advanced, then perhaps
pub fn in_git_repo(path: &str) -> Option<PathBuf> {
    let mut gitdir = PathBuf::from(path);

    while gitdir.parent().is_some() && !has_git_dir(gitdir.to_str().unwrap()) {
        gitdir.pop();
    }

    // No parent? then we hit root, finding no git
    if gitdir.parent().is_some() {
        return Some(gitdir);
    }

    None
}

// TODO: more reliable, more tested
// I don't want to use ProjectDirs, it puts config in awkward places on
// mac. Data too. Seems to be more intended for GUI apps.

#[cfg(not(target_os = "windows"))]
pub fn home_dir() -> PathBuf {
    let home = std::env::var("HOME").expect("$HOME not found");
    PathBuf::from(home)
}

#[cfg(target_os = "windows")]
pub fn home_dir() -> PathBuf {
    let home = std::env::var("USERPROFILE").expect("%userprofile% not found");
    PathBuf::from(home)
}

pub fn config_dir() -> PathBuf {
    let config_dir =
        std::env::var("XDG_CONFIG_HOME").map_or_else(|_| home_dir().join(".config"), PathBuf::from);
    config_dir.join("atuin")
}

pub fn data_dir() -> PathBuf {
    let data_dir = std::env::var("XDG_DATA_HOME")
        .map_or_else(|_| home_dir().join(".local").join("share"), PathBuf::from);

    data_dir.join("atuin")
}

pub fn get_current_dir() -> String {
    // Prefer PWD environment variable over cwd if available to better support symbolic links
    match env::var("PWD") {
        Ok(v) => v,
        Err(_) => match env::current_dir() {
            Ok(dir) => dir.display().to_string(),
            Err(_) => String::from(""),
        },
    }
}

pub fn is_zsh() -> bool {
    // only set on zsh
    env::var("ATUIN_SHELL_ZSH").is_ok()
}

pub fn is_fish() -> bool {
    // only set on fish
    env::var("ATUIN_SHELL_FISH").is_ok()
}

pub fn is_bash() -> bool {
    // only set on bash
    env::var("ATUIN_SHELL_BASH").is_ok()
}

#[cfg(test)]
mod tests {
    use time::Month;

    use super::*;
    use std::env;

    use std::collections::HashSet;

    #[test]
    fn test_dirs() {
        // these tests need to be run sequentially to prevent race condition
        test_config_dir_xdg();
        test_config_dir();
        test_data_dir_xdg();
        test_data_dir();
    }

    fn test_config_dir_xdg() {
        env::remove_var("HOME");
        env::set_var("XDG_CONFIG_HOME", "/home/user/custom_config");
        assert_eq!(
            config_dir(),
            PathBuf::from("/home/user/custom_config/atuin")
        );
        env::remove_var("XDG_CONFIG_HOME");
    }

    fn test_config_dir() {
        env::set_var("HOME", "/home/user");
        env::remove_var("XDG_CONFIG_HOME");
        assert_eq!(config_dir(), PathBuf::from("/home/user/.config/atuin"));
        env::remove_var("HOME");
    }

    fn test_data_dir_xdg() {
        env::remove_var("HOME");
        env::set_var("XDG_DATA_HOME", "/home/user/custom_data");
        assert_eq!(data_dir(), PathBuf::from("/home/user/custom_data/atuin"));
        env::remove_var("XDG_DATA_HOME");
    }

    fn test_data_dir() {
        env::set_var("HOME", "/home/user");
        env::remove_var("XDG_DATA_HOME");
        assert_eq!(data_dir(), PathBuf::from("/home/user/.local/share/atuin"));
        env::remove_var("HOME");
    }

    #[test]
    fn days_from_month() {
        assert_eq!(time::util::days_in_year_month(2023, Month::January), 31);
        assert_eq!(time::util::days_in_year_month(2023, Month::February), 28);
        assert_eq!(time::util::days_in_year_month(2023, Month::March), 31);
        assert_eq!(time::util::days_in_year_month(2023, Month::April), 30);
        assert_eq!(time::util::days_in_year_month(2023, Month::May), 31);
        assert_eq!(time::util::days_in_year_month(2023, Month::June), 30);
        assert_eq!(time::util::days_in_year_month(2023, Month::July), 31);
        assert_eq!(time::util::days_in_year_month(2023, Month::August), 31);
        assert_eq!(time::util::days_in_year_month(2023, Month::September), 30);
        assert_eq!(time::util::days_in_year_month(2023, Month::October), 31);
        assert_eq!(time::util::days_in_year_month(2023, Month::November), 30);
        assert_eq!(time::util::days_in_year_month(2023, Month::December), 31);

        // leap years
        assert_eq!(time::util::days_in_year_month(2024, Month::February), 29);
    }

    #[test]
    fn uuid_is_unique() {
        let how_many: usize = 1000000;

        // for peace of mind
        let mut uuids: HashSet<Uuid> = HashSet::with_capacity(how_many);

        // there will be many in the same millisecond
        for _ in 0..how_many {
            let uuid = uuid_v7();
            uuids.insert(uuid);
        }

        assert_eq!(uuids.len(), how_many);
    }
}
