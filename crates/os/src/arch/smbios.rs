use crate::request;

pub fn check_smbios() -> bool {
    let response = request::SMBIOS_REQUEST.get_response().unwrap();

    if let Some(_) = response.entry_64() {
        return true;
    }

    return false;
}
