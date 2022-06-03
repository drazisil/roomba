use nodejs_sys::{
    napi_callback_info, napi_create_function, napi_create_int32, napi_create_string_utf8, napi_env, napi_get_cb_info, napi_get_value_double, napi_set_named_property, napi_value,
};
use std::ffi::CString;

pub unsafe extern "C" fn implode(env: napi_env, _info: napi_callback_info) -> napi_value {
    #[allow(non_snake_case)]
    // The longest allowed repetition
    let MAX_REP_LENGTH = 0x204;      
    
    
    let mut local: napi_value = std::mem::zeroed();

    napi_create_int32(
        env,
        MAX_REP_LENGTH,
        &mut local
    );

    local
}

pub unsafe extern "C" fn say_hello(env: napi_env, info: napi_callback_info) -> napi_value {
// creating a buffer where napi_value of argument be written
let mut buffer: [napi_value; 2] = std::mem::MaybeUninit::zeroed().assume_init();
// max number of arguments
    let mut argc = 2 as usize;
// getting arguments and value of this
    napi_get_cb_info(
        env,
        info,
        &mut argc,
        buffer.as_mut_ptr(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    );
// converting napi to f64
    let mut x = 0 as f64;
    let mut y = 0 as f64;
    napi_get_value_double(env, buffer[0], &mut x);
    napi_get_value_double(env, buffer[1], &mut y);
// creating the return value

    let key = CString::new(String::from(x.to_string())).expect("CString::new failed");

        // creating a memory location where the pointer to napi_value will be saved
        let mut local: napi_value = std::mem::zeroed();

    napi_create_string_utf8(env, key.as_ptr(), key.as_c_str().to_bytes().len(), &mut local);

// returning the result
    local
}
#[no_mangle]
pub unsafe extern "C" fn napi_register_module_v1(
    env: napi_env,
    exports: napi_value,
) -> nodejs_sys::napi_value {
    // creating a C string
    let key = CString::new("hello").expect("CString::new failed");

    // creating a memory location where the pointer to napi_value will be saved
    let mut local: napi_value = std::mem::zeroed();
    // creating a C string
    let value = CString::new("world!").expect("CString::new failed");
    // creating napi_value for the string
    napi_create_string_utf8(env, value.as_ptr(), 6, &mut local);
    // setting the string on the exports object
    napi_set_named_property(env, exports, key.as_ptr(), local);


    // creating a C String
    let p = CString::new("myFunc").expect("CString::new failed");
    // returning the object
    napi_create_function(
        env,
// pointer to function name
        p.as_ptr(),
// length of function name
        5,
// rust function
        Some(say_hello),
// context which can be accessed by the rust function
        std::ptr::null_mut(),
// output napi_value
        &mut local,
    );
        // setting the string on the exports object
        napi_set_named_property(env, exports, p.as_ptr(), local);

            // creating a C String
    let implode_fn = CString::new("implode").expect("CString::new failed");
    // returning the object
    napi_create_function(
        env,
// pointer to function name
        implode_fn.as_ptr(),
// length of function name
        5,
// rust function
        Some(implode),
// context which can be accessed by the rust function
        std::ptr::null_mut(),
// output napi_value
        &mut local,
    );
        // setting the string on the exports object
        napi_set_named_property(env, exports, implode_fn.as_ptr(), local);
    exports
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
