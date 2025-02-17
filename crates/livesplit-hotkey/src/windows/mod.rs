use crate::KeyCode;
use parking_lot::Mutex;
use std::{
    cell::RefCell,
    collections::hash_map::{Entry, HashMap},
    mem, ptr,
    sync::{
        mpsc::{channel, Sender},
        Arc,
    },
    thread,
};
use winapi::{
    ctypes::c_int,
    shared::{
        minwindef::{DWORD, LPARAM, LRESULT, UINT, WPARAM},
        windef::HHOOK,
    },
    um::{
        libloaderapi::GetModuleHandleW,
        processthreadsapi::GetCurrentThreadId,
        winuser::{
            CallNextHookEx, GetMessageW, PostThreadMessageW, SetWindowsHookExW,
            UnhookWindowsHookEx, KBDLLHOOKSTRUCT, LLKHF_EXTENDED, WH_KEYBOARD_LL, WM_KEYDOWN,
        },
    },
};

const MSG_EXIT: UINT = 0x400;

#[derive(Debug, snafu::Snafu)]
pub enum Error {
    AlreadyRegistered,
    NotRegistered,
    WindowsHook,
    ThreadStopped,
    MessageLoop,
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct Hook {
    thread_id: DWORD,
    hotkeys: Arc<Mutex<HashMap<KeyCode, Box<dyn FnMut() + Send + 'static>>>>,
}

impl Drop for Hook {
    fn drop(&mut self) {
        unsafe {
            PostThreadMessageW(self.thread_id, MSG_EXIT, 0, 0);
        }
    }
}

struct State {
    hook: HHOOK,
    events: Sender<KeyCode>,
}

thread_local! {
    static STATE: RefCell<Option<State>> = RefCell::new(None);
}

fn parse_scan_code(value: DWORD) -> Option<KeyCode> {
    use self::KeyCode::*;
    Some(match value {
        0x0001 => Escape,
        0x0002 => Digit0,
        0x0003 => Digit1,
        0x0004 => Digit2,
        0x0005 => Digit3,
        0x0006 => Digit4,
        0x0007 => Digit5,
        0x0008 => Digit6,
        0x0009 => Digit7,
        0x000A => Digit8,
        0x000B => Digit9,
        0x000C => Minus,
        0x000D => Equal,
        0x000E => Backspace,
        0x000F => Tab,
        0x0010 => KeyQ,
        0x0011 => KeyW,
        0x0012 => KeyE,
        0x0013 => KeyR,
        0x0014 => KeyT,
        0x0015 => KeyY,
        0x0016 => KeyU,
        0x0017 => KeyI,
        0x0018 => KeyO,
        0x0019 => KeyP,
        0x001A => BracketLeft,
        0x001B => BracketRight,
        0x001C => Enter,
        0x001D => ControlLeft,
        0x001E => KeyA,
        0x001F => KeyS,
        0x0020 => KeyD,
        0x0021 => KeyF,
        0x0022 => KeyG,
        0x0023 => KeyH,
        0x0024 => KeyJ,
        0x0025 => KeyK,
        0x0026 => KeyL,
        0x0027 => Semicolon,
        0x0028 => Quote,
        0x0029 => Backquote,
        0x002A => ShiftLeft,
        0x002B => Backslash,
        0x002C => KeyZ,
        0x002D => KeyX,
        0x002E => KeyC,
        0x002F => KeyV,
        0x0030 => KeyB,
        0x0031 => KeyN,
        0x0032 => KeyM,
        0x0033 => Comma,
        0x0034 => Period,
        0x0035 => Slash,
        0x0036 => ShiftRight,
        0x0037 => NumpadMultiply,
        0x0038 => AltLeft,
        0x0039 => Space,
        0x003A => CapsLock,
        0x003B => F1,
        0x003C => F2,
        0x003D => F3,
        0x003E => F4,
        0x003F => F5,
        0x0040 => F6,
        0x0041 => F7,
        0x0042 => F8,
        0x0043 => F9,
        0x0044 => F10,
        0x0045 => Pause,
        0x0046 => ScrollLock,
        0x0047 => Numpad7,
        0x0048 => Numpad8,
        0x0049 => Numpad9,
        0x004A => NumpadSubtract,
        0x004B => Numpad4,
        0x004C => Numpad5,
        0x004D => Numpad6,
        0x004E => NumpadAdd,
        0x004F => Numpad1,
        0x0050 => Numpad2,
        0x0051 => Numpad3,
        0x0052 => Numpad0,
        0x0053 => NumpadDecimal,
        0x0054 => PrintScreen,
        0x0056 => IntlBackslash,
        0x0057 => F11,
        0x0058 => F12,
        0x0059 => NumpadEqual,
        0x005B => F13,
        0x005C => F14,
        0x005D => F15,
        0x0063 => F16,
        0x0064 => F17,
        0x0065 => F18,
        0x0066 => F19,
        0x0067 => F20,
        0x0068 => F21,
        0x0069 => F22,
        0x006A => F23,
        0x006B => F24,
        0x0070 => KanaMode,
        0x0071 => Lang2,
        0x0072 => Lang1,
        0x0073 => IntlRo,
        0x0079 => Convert,
        0x007B => NonConvert,
        0x007D => IntlYen,
        0x007E => NumpadComma,
        0xE008 => Undo,
        0xE00A => Paste,
        0xE010 => MediaTrackPrevious,
        0xE017 => Cut,
        0xE018 => Copy,
        0xE019 => MediaTrackNext,
        0xE01C => NumpadEnter,
        0xE01D => ControlRight,
        0xE01E => LaunchMail,
        0xE020 => AudioVolumeMute,
        0xE021 => LaunchApp2,
        0xE022 => MediaPlayPause,
        0xE024 => MediaStop,
        0xE02C => Eject,
        0xE02E => AudioVolumeDown,
        0xE030 => AudioVolumeUp,
        0xE032 => BrowserHome,
        0xE035 => NumpadDivide,
        0xE037 => PrintScreen,
        0xE038 => AltRight,
        0xE03B => Help,
        0xE045 => NumLock,
        0xE046 => Pause,
        0xE047 => Home,
        0xE048 => ArrowUp,
        0xE049 => PageUp,
        0xE04B => ArrowLeft,
        0xE04D => ArrowRight,
        0xE04F => End,
        0xE050 => ArrowDown,
        0xE051 => PageDown,
        0xE052 => Insert,
        0xE053 => Delete,
        0xE05B => MetaLeft,
        0xE05C => MetaRight,
        0xE05D => ContextMenu,
        0xE05E => Power,
        0xE065 => BrowserSearch,
        0xE066 => BrowserFavorites,
        0xE067 => BrowserRefresh,
        0xE068 => BrowserStop,
        0xE069 => BrowserForward,
        0xE06A => BrowserBack,
        0xE06B => LaunchApp1,
        0xE06C => LaunchMail,
        0xE06D => LaunchMediaPlayer,
        0xE0F1 => Lang2,
        0xE0F2 => Lang1,
        _ => return None,
    })
}

unsafe extern "system" fn callback_proc(code: c_int, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        let state = state.as_mut().expect("State should be initialized by now");

        if code >= 0 {
            let hook_struct = *(lparam as *const KBDLLHOOKSTRUCT);
            let event = wparam as UINT;
            if event == WM_KEYDOWN {
                let scan_code =
                    hook_struct.scanCode + ((hook_struct.flags & LLKHF_EXTENDED) * 0xE000);
                if let Some(key_code) = parse_scan_code(scan_code) {
                    state
                        .events
                        .send(key_code)
                        .expect("Callback Thread disconnected");
                }
            }
        }

        CallNextHookEx(state.hook, code, wparam, lparam)
    })
}

impl Hook {
    pub fn new() -> Result<Self> {
        let hotkeys = Arc::new(Mutex::new(HashMap::<
            KeyCode,
            Box<dyn FnMut() + Send + 'static>,
        >::new()));

        let (initialized_tx, initialized_rx) = channel();
        let (events_tx, events_rx) = channel();

        thread::spawn(move || {
            let mut hook = ptr::null_mut();

            STATE.with(|state| {
                hook = unsafe {
                    SetWindowsHookExW(
                        WH_KEYBOARD_LL,
                        Some(callback_proc),
                        GetModuleHandleW(ptr::null()),
                        0,
                    )
                };

                if !hook.is_null() {
                    initialized_tx
                        .send(Ok(unsafe { GetCurrentThreadId() }))
                        .map_err(|_| Error::ThreadStopped)?;
                } else {
                    initialized_tx
                        .send(Err(Error::WindowsHook))
                        .map_err(|_| Error::ThreadStopped)?;
                }

                *state.borrow_mut() = Some(State {
                    hook,
                    events: events_tx,
                });

                Ok(())
            })?;

            loop {
                let mut msg = mem::MaybeUninit::uninit();
                let ret = unsafe { GetMessageW(msg.as_mut_ptr(), ptr::null_mut(), 0, 0) };
                if ret < 0 {
                    return Err(Error::MessageLoop);
                }
                if unsafe { msg.assume_init().message } == MSG_EXIT {
                    break;
                }
            }

            unsafe {
                UnhookWindowsHookEx(hook);
            }

            Ok(())
        });

        let hotkey_map = hotkeys.clone();

        thread::spawn(move || {
            while let Ok(key) = events_rx.recv() {
                if let Some(callback) = hotkey_map.lock().get_mut(&key) {
                    callback();
                }
            }
        });

        let thread_id = initialized_rx.recv().map_err(|_| Error::ThreadStopped)??;

        Ok(Hook { thread_id, hotkeys })
    }

    pub fn register<F>(&self, hotkey: KeyCode, callback: F) -> Result<()>
    where
        F: FnMut() + Send + 'static,
    {
        if let Entry::Vacant(vacant) = self.hotkeys.lock().entry(hotkey) {
            vacant.insert(Box::new(callback));
            Ok(())
        } else {
            Err(Error::AlreadyRegistered)
        }
    }

    pub fn unregister(&self, hotkey: KeyCode) -> Result<()> {
        if self.hotkeys.lock().remove(&hotkey).is_some() {
            Ok(())
        } else {
            Err(Error::NotRegistered)
        }
    }
}
