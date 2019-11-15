#![cfg(target_os = "android")]
#![allow(non_snake_case)]

use crate::hello;
use jni::objects::{JClass, JString};
use jni::sys::jstring;
use jni::JNIEnv;
use std::ffi::CString;

// NOTE: RustKt references the name rusty.kt, which will be the kotlin file exposing the functions below.
// Remember the JNI naming conventions.

#[no_mangle]
pub extern "system" fn Java_com_leviathan_boiler_1lib_BoilerKt_hello(
  env: JNIEnv,
  _: JClass,
  input: JString,
) -> jstring {
  let input: String = env.get_string(input).expect("Couldn't get Java string!").into();
  let output = env.new_string(format!("Hello from rust: {}", input)).expect("Couldn't create a Java string!");
  output.into_inner()
}

