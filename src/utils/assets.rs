use crate::utils::errors::AssetEnumerationError;
use amethyst::utils::application_root_dir;
use std::{fs::read_dir, path::PathBuf};

const ASSET_PATH: &'static str = "assets";

// returns a tuple of (fileName, relativePath)
// path is relative from assets folder
pub fn enumerate_assets(
  sub_path: &str,
) -> Result<impl Iterator<Item = (String, String)>, AssetEnumerationError> {
  let path = application_root_dir()?.join(ASSET_PATH).join(sub_path);
  let base_path = PathBuf::from(sub_path);

  log::info!("Enumerating assets for {}", path.to_str().unwrap());

  Ok(read_dir(path)?.filter_map(move |entry| {
    if let Ok(entry) = entry {
      let path = entry.path();

      return Some((
        path.file_name()?.to_str()?.to_owned(),
        base_path.join(path.file_name()?).to_str()?.to_owned(),
      ));
    }

    None
  }))
}
