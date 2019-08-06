use reqwest;
use url::Url;
use std::os::raw::c_char;
use std::ffi::CStr;
use std::ffi::CString;
use serde_json as json;
use serde_pickle as pickle;
use std::collections::BTreeMap;
use std::fs;
use std::fs::File;
use std::io::{Read};

#[repr(C)]
pub struct ReqStruct {
    response: *const c_char
}


#[no_mangle]
pub extern "C" fn rust_get(link: *const c_char) -> ReqStruct {
    assert!(!link.is_null());
    let c_str = unsafe { CStr::from_ptr(link) };
    let string = c_str.to_str().expect("Not a valid UTF-8 string");
    let url = Url::parse(&string).unwrap();
    let resp = reqwest::get(url).unwrap().text().unwrap();

    //println!("{:#?}", resp);
    let c_to_print = CString::new(resp).expect("CString::new failed");
    ReqStruct {
        response: c_to_print.into_raw()
    }
}

#[no_mangle]
pub extern "C" fn dict_pass(input_temp_file: *const c_char, output_temp_file: *const c_char) {
    // File to read and write to
    let input_filename = unsafe { CStr::from_ptr(input_temp_file).to_str().expect("Not a valid UTF-8 string") };
    let output_filename = unsafe { CStr::from_ptr(output_temp_file).to_str().expect("Not a valid UTF-8 string") };
    
    //Processing
    let reader: Box<Read> = Box::new(File::open(input_filename).unwrap());
    let decoded: json::Value = pickle::from_reader(reader).unwrap();

    //Needs to be implemented
    let url = Url::parse(&decoded["url"].as_str().expect("Not a valid UTF-8 string")).unwrap();
    let resp = reqwest::get(url).unwrap().text().unwrap();
    let mut map = BTreeMap::new();
    map.insert("response".to_string(), resp);


    //Processing
    let serialized = serde_pickle::to_vec(&map, true).unwrap();
    fs::write(output_filename, serialized).expect("Unable to write file");
}
