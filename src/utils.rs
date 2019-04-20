// for Common funtions

#[macro_export]
macro_rules! vec_cstr {
     ( $( $x:expr ),* ) => {{
        {
            use std::ffi::CString;
            let mut temp_vec = Vec::<CString>::new();
            $(
                temp_vec.push(
                CString::new($x).expect("Could not convert to CString")
                    );
            )*
            temp_vec
        }}
    };
}
