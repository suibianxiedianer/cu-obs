use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use curl::easy::Easy;
use regex::Regex;

use crate::{obs::OBS, rpm::RPM, workspace::Workspace, Package};

#[derive(Debug)]
pub struct Add {
    uri: String,
}

// TODO: uri 可能是文件路径，可能是 url，需对其进行处理
impl Add {
    pub(crate) fn new(uri: impl ToString) -> Self {
        Add {
            uri: uri.to_string(),
        }
    }

    /// 对于指家的参数文件，默认为 src.rpm 处理，将其解开并添加、更新至 OBS 对应的位置
    pub(crate) fn apply(&self, pkg: &Package, ws: &Workspace) -> crate::Result<()> {
        // 如果是 url，需先下载
        let mut file = PathBuf::from(&self.uri);
        let re = Regex::new(r"^http(|s)://").unwrap();
        if re.is_match(&self.uri) {
            let name = &self.uri.rsplit("/").next().unwrap();
            file = ws.temp();
            file.push(name);
            let mut output = File::create(&file)?;

            let mut curl = Easy::new();
            curl.url(&self.uri)?;
            curl.write_function(move |data| {
                output.write(&data).unwrap();
                Ok(data.len())
            });

            curl.perform()?;
        }

        let rpm = RPM::new(&file)?;
        let obs = OBS::new(ws);

        // clean & install
        obs.clean_source(pkg)?;
        rpm.install_src(Some(ws.package_dir(pkg)))?;

        obs.add_files(pkg)?;
        let comment = format!("auto submit {}-{}.{}", &rpm.name(), &rpm.version(), &rpm.release());
        obs.commit(pkg, comment)?;
        obs.update(pkg)
    }
}
