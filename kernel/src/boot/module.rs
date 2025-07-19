use limine::file::File;

use crate::boot::requests::MODULE_REQUEST;

pub fn get_limine_module(name: &str) -> Option<&File> {
    let modules = MODULE_REQUEST.get_response().unwrap();

    for m in modules.modules() {
        let n = m.string().to_str().unwrap().replace("\"", "");

        if n.as_str() == name {
            return Some(&m);
        }
    }

    None
}
