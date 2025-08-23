#![cfg(target_os = "macos")]

use core_foundation::array::CFArrayRef;
use core_foundation::base::{Boolean, CFIndex, CFTypeRef};
use core_foundation::string::{CFStringRef, kCFStringEncodingUTF8};
use std::os::raw::{c_char, c_void};

type OSStatus = i32;
type TISInputSourceRef = CFTypeRef;

#[link(name = "CoreFoundation", kind = "framework")]
unsafe extern "C" {
    fn CFArrayGetCount(theArray: CFArrayRef) -> CFIndex;
    fn CFArrayGetValueAtIndex(theArray: CFArrayRef, idx: CFIndex) -> *const c_void;
    fn CFRelease(cf: CFTypeRef);

    fn CFStringGetLength(s: CFStringRef) -> CFIndex;
    fn CFStringGetMaximumSizeForEncoding(len: CFIndex, encoding: u32) -> CFIndex;
    fn CFStringGetCString(
        s: CFStringRef,
        buffer: *mut c_char,
        buffer_size: CFIndex,
        encoding: u32,
    ) -> Boolean;
}

#[link(name = "Carbon", kind = "framework")]
unsafe extern "C" {
    fn TISCreateInputSourceList(
        properties: *const c_void,
        includeAllInstalled: Boolean,
    ) -> CFArrayRef;
    fn TISSelectInputSource(source: TISInputSourceRef) -> OSStatus;
    fn TISGetInputSourceProperty(source: TISInputSourceRef, key: CFStringRef) -> CFTypeRef;

    static kTISPropertyInputSourceID: CFStringRef;
}

const ENGLISH_ID: &str = "com.apple.keylayout.ABC";

/// CFString == &str 비교용 최소 유틸
#[inline]
unsafe fn cfstring_eq(s: CFStringRef, needle: &str) -> bool {
    if s.is_null() {
        return false;
    }
    let len = unsafe { CFStringGetLength(s) };
    if len <= 0 {
        return needle.is_empty();
    }
    let cap = unsafe { CFStringGetMaximumSizeForEncoding(len, kCFStringEncodingUTF8) } + 1;
    let mut buf = vec![0u8; cap as usize];
    let ok = unsafe {
        CFStringGetCString(
            s,
            buf.as_mut_ptr() as *mut c_char,
            cap,
            kCFStringEncodingUTF8,
        )
    };
    if ok == 0 {
        return false;
    }
    if let Some(pos) = buf.iter().position(|&b| b == 0) {
        buf.truncate(pos);
    }
    std::str::from_utf8(&buf)
        .map(|s| s == needle)
        .unwrap_or(false)
}

/// 영어 ABC 입력 소스로 전환
pub fn switch_to_english() -> Result<(), &'static str> {
    unsafe {
        let list = TISCreateInputSourceList(std::ptr::null(), 1); // 전체 설치 소스
        if list.is_null() {
            return Err("TISCreateInputSourceList 실패");
        }
        let count = CFArrayGetCount(list);
        if count <= 0 {
            CFRelease(list as CFTypeRef);
            return Err("사용 가능한 입력 소스가 없음");
        }

        let mut ok = false;
        for i in 0..count {
            let src = CFArrayGetValueAtIndex(list, i) as TISInputSourceRef;
            if src.is_null() {
                continue;
            }

            let id_cf = TISGetInputSourceProperty(src, kTISPropertyInputSourceID) as CFStringRef;
            if cfstring_eq(id_cf, ENGLISH_ID) {
                let status = TISSelectInputSource(src);
                ok = status == 0; // noErr
                break;
            }
        }

        CFRelease(list as CFTypeRef);

        if ok {
            Ok(())
        } else {
            Err("ABC 입력 소스를 찾지 못했거나 전환 실패")
        }
    }
}
