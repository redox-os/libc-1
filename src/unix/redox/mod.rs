pub type c_char = i8;
pub type c_long = i64;
pub type c_ulong = u64;
pub type wchar_t = i32;

pub type blkcnt_t = ::c_ulong;
pub type blksize_t = ::c_long;
pub type clock_t = ::c_long;
pub type dev_t = ::c_long;
pub type fsblkcnt_t = ::c_ulong;
pub type fsfilcnt_t = ::c_ulong;
pub type ino_t = ::c_ulong;
pub type mode_t = ::c_int;
pub type nfds_t = ::c_ulong;
pub type nlink_t = ::c_ulong;
pub type off_t = ::c_long;
pub type pthread_t = *mut ::c_void;
pub type pthread_attr_t = *mut ::c_void;
pub type pthread_cond_t = *mut ::c_void;
pub type pthread_condattr_t = *mut ::c_void;
pub type pthread_key_t = *mut ::c_void;
pub type pthread_mutex_t = *mut ::c_void;
pub type pthread_mutexattr_t = *mut ::c_void;
pub type pthread_rwlock_t = *mut ::c_void;
pub type pthread_rwlockattr_t = *mut ::c_void;
pub type rlim_t = ::c_ulonglong;
pub type sa_family_t = u16;
pub type sem_t = *mut ::c_void;
pub type sigset_t = ::c_ulong;
pub type socklen_t = u32;
pub type speed_t = u32;
pub type suseconds_t = ::c_int;
pub type tcflag_t = u32;
pub type time_t = ::c_long;

s! {
    pub struct addrinfo {
        pub ai_flags: ::c_int,
        pub ai_family: ::c_int,
        pub ai_socktype: ::c_int,
        pub ai_protocol: ::c_int,
        pub ai_addrlen: ::size_t,
        pub ai_canonname: *mut ::c_char,
        pub ai_addr: *mut ::sockaddr,
        pub ai_next: *mut ::addrinfo,
    }

    pub struct dirent {
        pub d_ino: ::ino_t,
        pub d_off: ::off_t,
        pub d_reclen: ::c_ushort,
        pub d_type: ::c_uchar,
        pub d_name: [::c_char; 256],
    }

    pub struct Dl_info {
        pub dli_fname: *const ::c_char,
        pub dli_fbase: *mut ::c_void,
        pub dli_sname: *const ::c_char,
        pub dli_saddr: *mut ::c_void,
    }

    pub struct fd_set {
        fds_bits: [::c_ulong; ::FD_SETSIZE / ULONG_SIZE],
    }

    pub struct lconv {
        pub currency_symbol: *const ::c_char,
        pub decimal_point: *const ::c_char,
        pub frac_digits: ::c_char,
        pub grouping: *const ::c_char,
        pub int_curr_symbol: *const ::c_char,
        pub int_frac_digits: ::c_char,
        pub mon_decimal_point: *const ::c_char,
        pub mon_grouping: *const ::c_char,
        pub mon_thousands_sep: *const ::c_char,
        pub negative_sign: *const ::c_char,
        pub n_cs_precedes: ::c_char,
        pub n_sep_by_space: ::c_char,
        pub n_sign_posn: ::c_char,
        pub positive_sign: *const ::c_char,
        pub p_cs_precedes: ::c_char,
        pub p_sep_by_space: ::c_char,
        pub p_sign_posn: ::c_char,
        pub thousands_sep: *const ::c_char,
    }

    pub struct passwd {
        pub pw_name: *mut ::c_char,
        pub pw_passwd: *mut ::c_char,
        pub pw_uid: ::uid_t,
        pub pw_gid: ::gid_t,
        pub pw_gecos: *mut ::c_char,
        pub pw_dir: *mut ::c_char,
        pub pw_shell: *mut ::c_char,
    }

    pub struct sigaction {
        pub sa_handler: ::sighandler_t,
        pub sa_flags: ::c_ulong,
        pub sa_restorer: ::Option<extern fn()>,
        pub sa_mask: ::sigset_t,
    }

    pub struct sockaddr {
        pub sa_family: ::sa_family_t,
        pub sa_data: [::c_char; 14],
    }

    pub struct stat {
        pub st_dev: ::dev_t,
        pub st_ino: ::ino_t,
        pub st_nlink: ::nlink_t,
        pub st_mode: ::mode_t,
        pub st_uid: ::uid_t,
        pub st_gid: ::gid_t,
        pub st_rdev: ::dev_t,
        pub st_size: ::off_t,
        pub st_blksize: ::blksize_t,
        pub st_blocks: ::blkcnt_t,
        pub st_atime: ::timespec,
        pub st_mtime: ::timespec,
        pub st_ctime: ::timespec,
        _pad: [::c_char; 24],
    }

    pub struct statvfs {
        pub f_bsize: ::c_ulong,
        pub f_frsize: ::c_ulong,
        pub f_blocks: ::fsblkcnt_t,
        pub f_bfree: ::fsblkcnt_t,
        pub f_bavail: ::fsblkcnt_t,
        pub f_files: ::fsfilcnt_t,
        pub f_ffree: ::fsfilcnt_t,
        pub f_favail: ::fsfilcnt_t,
        pub f_fsid: ::c_ulong,
        pub f_flag: ::c_ulong,
        pub f_namemax: ::c_ulong,
    }

    pub struct termios {
        pub c_iflag: ::tcflag_t,
        pub c_oflag: ::tcflag_t,
        pub c_cflag: ::tcflag_t,
        pub c_lflag: ::tcflag_t,
        pub c_line: ::cc_t,
        pub c_cc: [::cc_t; ::NCCS],
        pub c_ispeed: ::speed_t,
        pub c_ospeed: ::speed_t,
    }

    pub struct tm {
        pub tm_sec: ::c_int,
        pub tm_min: ::c_int,
        pub tm_hour: ::c_int,
        pub tm_mday: ::c_int,
        pub tm_mon: ::c_int,
        pub tm_year: ::c_int,
        pub tm_wday: ::c_int,
        pub tm_yday: ::c_int,
        pub tm_isdst: ::c_int,
        pub tm_gmtoff: ::c_long,
        pub tm_zone: *const ::c_char,
    }
}

pub const FD_SETSIZE: usize = 1024;
pub const NCCS: usize = 32;

// intentionally not public, only used for fd_set
cfg_if! {
    if #[cfg(target_pointer_width = "32")] {
        const ULONG_SIZE: usize = 32;
    } else if #[cfg(target_pointer_width = "64")] {
        const ULONG_SIZE: usize = 64;
    } else {
        // Unknown target_pointer_width
    }
}
