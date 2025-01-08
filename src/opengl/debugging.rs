use core::{
    ffi::{c_void, CStr},
    ptr,
};

#[inline(never)]
extern "system" fn gl_debug_callback(source: u32, gltype: u32, id: u32, severity: u32, _length: i32, message: *const i8, _user_param: *mut c_void) {
    let source = match source {
        gl::DEBUG_SOURCE_API => "API",
        gl::DEBUG_SOURCE_WINDOW_SYSTEM => "Window System",
        gl::DEBUG_SOURCE_SHADER_COMPILER => "Shader Compiler",
        gl::DEBUG_SOURCE_THIRD_PARTY => "Third Party",
        gl::DEBUG_SOURCE_APPLICATION => "Application",
        gl::DEBUG_SOURCE_OTHER => "Other",
        _ => "Unknown",
    };

    let gltype = match gltype {
        gl::DEBUG_TYPE_ERROR => "Error",
        gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR => "Deprecated Behavior",
        gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR => "Undefined Behavior",
        gl::DEBUG_TYPE_PORTABILITY => "Portability",
        gl::DEBUG_TYPE_PERFORMANCE => "Performance",
        gl::DEBUG_TYPE_MARKER => "Marker",
        gl::DEBUG_TYPE_PUSH_GROUP => "Push Group",
        gl::DEBUG_TYPE_POP_GROUP => "Pop Group",
        gl::DEBUG_TYPE_OTHER => "Other",
        _ => "Unknown",
    };

    let severity = match severity {
        gl::DEBUG_SEVERITY_HIGH => "High",
        gl::DEBUG_SEVERITY_MEDIUM => "Medium",
        gl::DEBUG_SEVERITY_LOW => "Low",
        gl::DEBUG_SEVERITY_NOTIFICATION => "Notification",
        _ => "Unknown",
    };

    unsafe {
        let message = CStr::from_ptr(message).to_str().unwrap();
        println!("OpenGL Debug Message: [{}] [{}] [{}] [{}]", source, gltype, id, severity);
        print!("\t");

        let mut printed_chars = 0;
        let mut pending_newline = false;
        let mut last_char_was_newline = false;

        for char in message.chars() {
            if char == ' ' {
                if pending_newline {
                    println!("{}", char);
                    print!("\t");
                    pending_newline = false;
                    last_char_was_newline = true;
                    printed_chars = 0;
                } else {
                    print!("{}", char);
                    last_char_was_newline = true;
                }
            } else {
                print!("{}", char);
                last_char_was_newline = false;
            }

            printed_chars += 1;

            if printed_chars > 80 {
                pending_newline = true;
            }
        }

        if !last_char_was_newline {
            println!();
        }
    }
}

pub fn gl_initialize_debugging() {
    unsafe {
        gl::Enable(gl::DEBUG_OUTPUT);
        gl::DebugMessageCallback(Some(gl_debug_callback), ptr::null());
    }
}
