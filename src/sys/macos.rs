//! Using [`IOPMAssertionCreateWithName`].
//!
//! Debug with `pmset -g assertions`.
//!
//! [`IOPMAssertionCreateWithName`]: https://developer.apple.com/documentation/iokit/1557134-iopmassertioncreatewithname

use anyhow::{anyhow, Result};
use apple_sys::IOKit::{
    kIOPMAssertionLevelOn, kIOReturnSuccess, CFStringRef, IOPMAssertionCreateWithName,
    IOPMAssertionRelease,
};
use core_foundation::{base::TCFType, string::CFString};

use crate::AwakeOptions;

#[allow(non_upper_case_globals)]
const kIOPMAssertionTypePreventUserIdleSystemSleep: &str = "PreventUserIdleSystemSleep";

#[allow(non_upper_case_globals)]
const kIOPMAssertionTypePreventUserIdleDisplaySleep: &str = "PreventUserIdleDisplaySleep";

#[allow(non_upper_case_globals)]
const kIOPMAssertionTypePreventSystemSleep: &str = "PreventSystemSleep";

pub struct Awake {
    options: AwakeOptions,

    display_assertion: u32,
    idle_assertion: u32,
    sleep_assertion: u32,
}

impl Awake {
    pub fn new(options: AwakeOptions) -> Result<Self> {
        let mut awake = Awake {
            options,
            display_assertion: 0,
            idle_assertion: 0,
            sleep_assertion: 0,
        };
        awake.set()?;
        Ok(awake)
    }

    fn set(&mut self) -> Result<()> {
        if self.options.display {
            unsafe {
                let result = IOPMAssertionCreateWithName(
                    // TODO Are those casts the best way? No way to make a const CFString?
                    CFString::from_static_string(kIOPMAssertionTypePreventUserIdleDisplaySleep)
                        .as_concrete_TypeRef() as CFStringRef,
                    kIOPMAssertionLevelOn,
                    CFString::new(self.options.reason()).as_concrete_TypeRef() as CFStringRef,
                    &mut self.display_assertion,
                );
                if result != kIOReturnSuccess as i32 {
                    // TODO Better error?
                    return Err(anyhow!("IO error: {:#x}", result));
                }
            }
        }

        if self.options.idle {
            unsafe {
                let result = IOPMAssertionCreateWithName(
                    CFString::from_static_string(kIOPMAssertionTypePreventUserIdleSystemSleep)
                        .as_concrete_TypeRef() as CFStringRef,
                    kIOPMAssertionLevelOn,
                    CFString::new(self.options.reason()).as_concrete_TypeRef() as CFStringRef,
                    &mut self.idle_assertion,
                );
                if result != kIOReturnSuccess as i32 {
                    return Err(anyhow!("IO error: {:#x}", result));
                }
            }
        }

        if self.options.sleep {
            unsafe {
                let result = IOPMAssertionCreateWithName(
                    CFString::from_static_string(kIOPMAssertionTypePreventSystemSleep)
                        .as_concrete_TypeRef() as CFStringRef,
                    kIOPMAssertionLevelOn,
                    CFString::new(self.options.reason()).as_concrete_TypeRef() as CFStringRef,
                    &mut self.sleep_assertion,
                );
                if result != kIOReturnSuccess as i32 {
                    return Err(anyhow!("IO error: {:#x}", result));
                }
            }
        }

        Ok(())
    }
}

impl Drop for Awake {
    fn drop(&mut self) {
        if self.display_assertion != 0 {
            unsafe {
                IOPMAssertionRelease(self.display_assertion);
            }
        }

        if self.display_assertion != 0 {
            unsafe {
                IOPMAssertionRelease(self.display_assertion);
            }
        }

        if self.sleep_assertion != 0 {
            unsafe {
                IOPMAssertionRelease(self.sleep_assertion);
            }
        }
    }
}
