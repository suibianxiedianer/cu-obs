use std::path::Path;

use crate::{obs::OBS, rpm::RPM, workspace::Workspace, Package};

#[derive(Debug)]
pub struct Add {
    file: String,
}

// TODO: file 可能是文件路径，可能是 url，需对其进行处理
impl Add {
    pub(crate) fn new(file: impl ToString) -> Self {
        Add {
            file: file.to_string(),
        }
    }

    /// 对于指家的参数文件，默认为 src.rpm 处理，将其解开并添加、更新至 OBS 对应的位置
    pub(crate) fn apply(&self, pkg: &Package, ws: &Workspace) -> crate::Result<()> {
        let _path = Path::new(&self.file);
        let obs = OBS::new(ws);

        obs.clean_source(pkg)?;

        RPM::install_src(Path::new(_path), Some(ws.package_dir(pkg)))?;

        obs.add_files(pkg)?;

        let mut comment = format!(
            "auto submit {}-{}.{}",
            RPM::get_name(_path).unwrap(),
            RPM::get_version(_path).unwrap(),
            RPM::get_release(_path).unwrap()
        );

        obs.commit(pkg, comment)?;
        obs.update(pkg)
    }
}
