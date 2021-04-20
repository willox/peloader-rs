use std::cell::RefCell;

use crate::win32;

pub const WM_USER_ON_SCHEDULE_MESSAGE_PUMP_WORK: u32 = win32::WM_USER + 0x0400;
const TIMER_PERFORM_MESSAGE_LOOP_WORK: win32::WPARAM = win32::WPARAM(0);

// Returns the handle of our messaging window. `WM_USER_ON_SCHEDULE_MESSAGE_PUMP_WORK` can be posted to it.
pub fn init() -> win32::HWND {
    unsafe {
        let x = win32::WNDCLASSA {
            lpszClassName: std::mem::transmute(b"DreamLoader_MessageWindow\0".as_ptr()),
            lpfnWndProc: Some(MessageLoop::window_proc),
            cbWndExtra: std::mem::size_of::<*mut MessageLoop>() as i32,
            ..Default::default()
        };

        win32::RegisterClassA(&x);
    }

    let window = unsafe {
        win32::CreateWindowExA(
            win32::WINDOW_EX_STYLE::default(),
            "DreamLoader_MessageWindow",
            "DreamLoader Messaging Window",
            win32::WINDOW_STYLE::default(),
            0,
            0,
            0,
            0,
            win32::HWND_MESSAGE,
            win32::HMENU::default(),
            win32::HINSTANCE::default(),
            std::ptr::null_mut(),
        )
    };

    let message_loop = Box::new(MessageLoop {
        window: window.clone(),
        was_reentrant: RefCell::new(false),
        working: RefCell::new(()),
    });

    let ptr = Box::into_raw(message_loop);

    unsafe {
        win32::SetWindowLongA(window, win32::WINDOW_LONG_PTR_INDEX::default(), ptr as _);
        win32::PostMessageA(
            window,
            WM_USER_ON_SCHEDULE_MESSAGE_PUMP_WORK,
            win32::WPARAM(0),
            win32::LPARAM(0),
        );
    }

    window
}

struct MessageLoop {
    /// HWND to our messaging window. The window is never visible and is just used for receiving timer messages.
    window: win32::HWND,

    // This gets set to true if a call to do_message_loop_work fails due to Self::working already being locked.
    was_reentrant: RefCell<bool>,

    // This is locked while calling cef_do_message_loop_work.
    working: RefCell<()>,
}

impl MessageLoop {
    extern "system" fn window_proc(
        hwnd: win32::HWND,
        message: u32,
        w_param: win32::WPARAM,
        l_param: win32::LPARAM,
    ) -> win32::LRESULT {
        match message {
            WM_USER_ON_SCHEDULE_MESSAGE_PUMP_WORK => {
                let this: &Self = unsafe {
                    let ptr =
                        win32::GetWindowLongA(hwnd, win32::WINDOW_LONG_PTR_INDEX(0)) as *const Self;
                    &*ptr
                };

                this.on_schedule_message_pump_work(w_param.0);
                win32::LRESULT(0)
            }

            win32::WM_TIMER if w_param == TIMER_PERFORM_MESSAGE_LOOP_WORK => {
                let this: &Self = unsafe {
                    let ptr =
                        win32::GetWindowLongA(hwnd, win32::WINDOW_LONG_PTR_INDEX(0)) as *const Self;
                    &*ptr
                };

                this.on_timer_expired();
                win32::LRESULT(0)
            }

            win32::WM_CLOSE => {
                // TODO: Box::from_raw the state, remove it from the window, etc.
                // There's no point doing this unless we actually make sure nothing is referring to the MessageLoop, so for now just let it leak.
                unsafe { win32::DefWindowProcA(hwnd, message, w_param, l_param) }
            }

            _ => unsafe { win32::DefWindowProcA(hwnd, message, w_param, l_param) },
        }
    }

    fn do_message_loop_work(&self) {
        /// Returns true if a reentrant call to do_message_loop_work occured while running. This would mean that another do_message_loop_work call needs to be scheduled.
        fn inner_do_message_loop_work(this: &MessageLoop) -> bool {
            match this.working.try_borrow_mut() {
                Ok(_lock) => {
                    // Now that we've started a unit of work, make sure to reset was_reentrant
                    *this.was_reentrant.borrow_mut() = false;

                    ::cef::cef_do_message_loop_work();

                    // If it became true again, it means we blocked another unit of work from running
                    *this.was_reentrant.borrow()
                }

                Err(_) => {
                    // A unit of work was already processing - nbd, we just need to let it know so it can handle it for us
                    *this.was_reentrant.borrow_mut() = true;
                    false
                }
            }
        }

        if inner_do_message_loop_work(self) {
            // We've got to go again. Schedule a call at the end of our window's message queue.
            unsafe {
                win32::PostMessageA(
                    self.window,
                    WM_USER_ON_SCHEDULE_MESSAGE_PUMP_WORK,
                    win32::WPARAM(0),
                    win32::LPARAM(0),
                );
            }
            return;
        }

        // TODO: Didn't code this!
        // If no timer, make one for 15ms!
        unsafe {
            win32::PostMessageA(
                self.window,
                WM_USER_ON_SCHEDULE_MESSAGE_PUMP_WORK,
                win32::WPARAM(1),
                win32::LPARAM(0),
            );
        }
    }

    fn on_schedule_message_pump_work(&self, delay_ms: usize) {
        // Replace the current timer (or do work immediately)
        self.kill_timer();

        // Run immediately if there is no delay
        if delay_ms <= 0 {
            self.do_message_loop_work();
            return;
        }

        // Handle delays longer than our minimum
        let delay_ms = delay_ms.max(15);

        unsafe {
            win32::SetTimer(
                self.window,
                TIMER_PERFORM_MESSAGE_LOOP_WORK.0,
                delay_ms as u32,
                None,
            );
        }
    }

    fn on_timer_expired(&self) {
        // We don't want this timer to repeat.
        // We kill it early in case another ends up being created.
        self.kill_timer();
        self.do_message_loop_work();
    }

    fn kill_timer(&self) {
        unsafe {
            win32::KillTimer(self.window, TIMER_PERFORM_MESSAGE_LOOP_WORK.0);
        }
    }
}
