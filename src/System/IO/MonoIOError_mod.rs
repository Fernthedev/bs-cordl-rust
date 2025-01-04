#[cfg(feature = "System+IO+MonoIOError")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MonoIOError {
    #[default]
    ERROR_ACCESS_DENIED = 5i32,
    ERROR_ALREADY_EXISTS = 183i32,
    ERROR_BROKEN_PIPE = 109i32,
    ERROR_CANNOT_MAKE = 82i32,
    ERROR_DIRECTORY = 267i32,
    ERROR_DIR_NOT_EMPTY = 145i32,
    ERROR_ENCRYPTION_FAILED = 6000i32,
    ERROR_FILENAME_EXCED_RANGE = 206i32,
    ERROR_FILE_EXISTS = 80i32,
    ERROR_FILE_NOT_FOUND = 2i32,
    ERROR_GEN_FAILURE = 31i32,
    ERROR_HANDLE_DISK_FULL = 39i32,
    ERROR_INVALID_DRIVE = 15i32,
    ERROR_INVALID_HANDLE = 6i32,
    ERROR_INVALID_NAME = 123i32,
    ERROR_INVALID_PARAMETER = 87i32,
    ERROR_LOCK_VIOLATION = 33i32,
    ERROR_NOT_READY = 21i32,
    ERROR_NOT_SAME_DEVICE = 17i32,
    ERROR_NOT_SUPPORTED = 50i32,
    ERROR_NO_MORE_FILES = 18i32,
    ERROR_PATH_NOT_FOUND = 3i32,
    ERROR_READ_FAULT = 30i32,
    ERROR_SHARING_VIOLATION = 32i32,
    ERROR_SUCCESS = 0i32,
    ERROR_TOO_MANY_OPEN_FILES = 4i32,
    ERROR_WRITE_FAULT = 29i32,
}
#[cfg(feature = "System+IO+MonoIOError")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::IO::MonoIOError => "System.IO"
    ."MonoIOError"
);
