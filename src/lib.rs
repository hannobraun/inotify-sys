#![deny(missing_docs)]
#![deny(warnings)]


//! Bindings for inotify
//!
//! There are four types of statics:
//!
//! - __Flags__, to be passed to [`inotify_init1()`];
//! - __Events__, that describe which events should be
//!   watched for (when calling [`inotify_add_watch()`]),
//!   and which event has occured (when returned by
//!   [`read()`]);
//! - __Options__, which can be added to the bit mask
//!   passed to [`inotify_add_watch()`], to change default
//!   behavior;
//! - __Infos__, indicating further details of the event
//!   that occured (returned by [`read()`]).
//!
//! When events occur for monitored files and directories, those events
//! are made available to the application as structured data that can
//! be read from the inotify file descriptor using [`read()`].
//!
//! When all file descriptors referring to an inotify instance have been
//! closed (using [`close()`]), the underlying object and its resources
//! are freed for reuse by the kernel; all associated watches are
//! automatically freed.
//!
//! A programmer wanting to use inotify should also carefully read through
//! the [inotify(7)] man page, which contains many caveats, warnings, and
//! recommendations for proper, robust, and efficient usage of inotify.
//!
//! [`inotify_init1()`]: fn.inotify_init1.html
//! [`inotify_add_watch()`]: fn.inotify_add_watch.html
//! [`read()`]: ../libc/fn.read.html
//! [`close()`]: ../libc/fn.close.html
//! [inotify(7)]: http://man7.org/linux/man-pages/man7/inotify.7.html


extern crate libc;


use libc::{
    c_char,
    c_int,
    uint32_t };


/// Flag: Set the FD_CLOEXEC flag
///
/// The FD_CLOEXEC flag, or "close-on-exec", changes the
/// behavior of file descriptor when [execve(2)]'d:
///
/// > If the FD_CLOEXEC bit is 0, the file descriptor will
/// > remain open across an [execve(2)], otherwise it will be
/// > closed.
///
/// See [open(2)] and [fcntl(2)] for details.
///
/// [execve(2)]: http://man7.org/linux/man-pages/man2/execve.2.html
/// [open(2)]: http://man7.org/linux/man-pages/man2/open.2.html
/// [fcntl(2)]: http://man7.org/linux/man-pages/man2/fcntl.2.html
pub const IN_CLOEXEC: c_int = 0o2000000;

/// Flag: Set the O_NONBLOCK file status flag
///
/// The O_NONBLOCK flag changes the behavior of system
/// calls when accessing files with mandatory locks:
///
/// > By default, both traditional (process-associated) and
/// > open file description record locks are advisory.  Advisory
/// > locks are not enforced and are useful only between
/// > cooperating processes.
/// >
/// > Both lock types can also be mandatory. Mandatory locks
/// > are enforced for all processes. If a process tries to
/// > perform an incompatible access (e.g., [read(2)] or [write(2)])
/// > on a file region that has an incompatible mandatory lock,
/// > then the result depends upon whether the O_NONBLOCK flag
/// > is enabled for its open file description. If the O_NONBLOCK
/// > flag is not enabled, then the system call is blocked until
/// > the lock is removed or converted to a mode that is compatible
/// > with the access. If the O_NONBLOCK flag is enabled, then the
/// > system call fails with the error EAGAIN.
///
/// See [fcntl(2)] for more details.
///
/// [read(2)]:  http://man7.org/linux/man-pages/man2/read.2.html
/// [write(2)]: http://man7.org/linux/man-pages/man2/write.2.html
/// [fcntl(2)]: http://man7.org/linux/man-pages/man2/fcntl.2.html
pub const IN_NONBLOCK: c_int = 0o4000;

/// Event: File was accessed.
///
/// When monitoring a directory, the event may occur both for the
/// directory itself and the files within.
pub const IN_ACCESS: uint32_t = 0x00000001;

/// Event: File was modified.
pub const IN_MODIFY: uint32_t = 0x00000002;

/// Event: Metadata has changed.
///
/// This can include e.g.
///
/// - permissions, see [chmod(2)];
/// - timestamps, see [utimensat(2)];
/// - extended attributes, see [setxattr(2)];
/// - link count, see [link(2)] and [unlink(2)];
/// - user/group, see [chown(2)].
///
/// When monitoring a directory, the event may occur both for the
/// directory itself and the files within.
///
/// [chmod(2)]: http://man7.org/linux/man-pages/man2/chmod.2.html
/// [utimensat(2)]: http://man7.org/linux/man-pages/man2/utimensat.2.html
/// [setxattr(2)]: http://man7.org/linux/man-pages/man2/fsetxattr.2.html
/// [link(2)]: http://man7.org/linux/man-pages/man2/link.2.html
/// [unlink(2)]: http://man7.org/linux/man-pages/man2/unlink.2.html
/// [chown(2)]: http://man7.org/linux/man-pages/man2/lchown.2.html
pub const IN_ATTRIB: uint32_t = 0x00000004;

/// Event: File opened for writing was closed.
///
/// When monitoring a directory, the event may occur both for the
/// directory itself and the files within.
pub const IN_CLOSE_WRITE: uint32_t = 0x00000008;

/// Event: File not opened for writing was closed.
///
/// When monitoring a directory, the event may occur both for the
/// directory itself and the files within.
pub const IN_CLOSE_NOWRITE: uint32_t = 0x00000010;

/// Event: File was opened.
///
/// When monitoring a directory, the event may occur both for the
/// directory itself and the files within.
pub const IN_OPEN: uint32_t = 0x00000020;

/// Event: File or directory was moved away.
///
/// When monitoring a directory, the event may occur *only* for
/// the files within, not the directory itself.
pub const IN_MOVED_FROM: uint32_t = 0x00000040;

/// Event: File or directory was moved in.
///
/// When monitoring a directory, the event may occur *only* for
/// the files within, not the directory itself.
pub const IN_MOVED_TO: uint32_t = 0x00000080;

/// Event: File or directory was created.
///
/// This may also include hard links, symlinks, and UNIX sockets.
///
/// When monitoring a directory, the event may occur *only* for
/// the files within, not the directory itself.
pub const IN_CREATE: uint32_t = 0x00000100;

/// Event: File or directory was deleted.
///
/// This may also include hard links, symlinks, and UNIX sockets.
///
/// When monitoring a directory, the event may occur *only* for
/// the files within, not the directory itself.
pub const IN_DELETE: uint32_t = 0x00000200;

/// Event: Watched file or directory was deleted.
///
/// This may also occur if the object is moved to another
/// filesystem, since [mv(1)] in effect copies the file to the
/// other filesystem and then deletes it from the original.
///
/// An IN_IGNORED event will subsequently be generated.
///
/// [mv(1)]: http://man7.org/linux/man-pages/man1/mv.1.html
pub const IN_DELETE_SELF: uint32_t = 0x00000400;

/// Event: Watched file or directory was moved.
pub const IN_MOVE_SELF: uint32_t = 0x00000800;

/// Event: File or directory was moved away or in.
///
/// When monitoring a directory, the event may occur *only* for
/// the files within, not the directory itself.
pub const IN_MOVE: uint32_t = (IN_MOVED_FROM | IN_MOVED_TO);

/// Event: File opened was closed.
///
/// When monitoring a directory, the event may occur both for the
/// directory itself and the files within.
pub const IN_CLOSE: uint32_t = (IN_CLOSE_WRITE | IN_CLOSE_NOWRITE);

/// Event: Any event occured.
pub const IN_ALL_EVENTS: uint32_t = (
    IN_ACCESS | IN_MODIFY | IN_ATTRIB | IN_CLOSE_WRITE | IN_CLOSE_NOWRITE
    | IN_OPEN | IN_MOVED_FROM | IN_MOVED_TO | IN_CREATE | IN_DELETE
    | IN_DELETE_SELF | IN_MOVE_SELF);

/// Option: Don't watch children (if self is a directory).
pub const IN_ONLYDIR: uint32_t = 0x01000000;

/// Option: Don't dereference (if self is a symlink).
pub const IN_DONT_FOLLOW: uint32_t = 0x02000000;

/// Option: Don't watch unlinked children.
///
/// > By default, when watching events on the children of a
/// > directory, events are generated for children even after
/// > they have been unlinked from the directory.  This can
/// > result in large numbers of uninteresting events for some
/// > applications (e.g., if watching /tmp, in which many
/// > applications create temporary files whose names are
/// > immediately unlinked).
/// >
/// > IN_EXCL_UNLINK changes this behavior, so that events are
/// > not generated for children after they have been unlinked
/// > from the watched directory.
pub const IN_EXCL_UNLINK: uint32_t = 0x04000000;

/// Option: Add events to an existing watch instead of replacing it.
///
/// > If a watch instance already exists for the filesystem
/// > object corresponding to self, add (|) the events to the
/// > watch mask instead of replacing it.
pub const IN_MASK_ADD: uint32_t = 0x20000000;

/// Option: Listen for one event, then remove the watch.
pub const IN_ONESHOT: uint32_t = 0x80000000;

/// Info: Subject of this event is a directory.
pub const IN_ISDIR: uint32_t = 0x40000000;

/// Info: Filesystem containing self was unmounted.
///
/// An IN_IGNORED event will subsequently be generated.
pub const IN_UNMOUNT: uint32_t = 0x00002000;

/// Info: Event queue overflowed.
pub const IN_Q_OVERFLOW: uint32_t = 0x00004000;

/// Info: Watch was removed.
///
/// This can occur either as a result of `inotify_rm_watch()`,
/// or because self was deleted or the containing filesystem
/// was unmounted, or after an IN_ONESHOT watch is complete.
///
/// See the BUGS section of [inotify(7)] for more details.
///
/// [inotify(7)]: http://man7.org/linux/man-pages/man7/inotify.7.html
pub const IN_IGNORED: uint32_t = 0x00008000;


/// Describes a file system event
///
/// From [inotify(7)]:
///
/// > To determine what events have occurred, an application [read(2)]s
/// > from the inotify file descriptor.  If no events have so far occurred,
/// > then, assuming a blocking file descriptor, [read(2)] will block until
/// > at least one event occurs (unless interrupted by a signal, in which
/// > case the call fails with the error EINTR; see [signal(7)]).
/// >
/// > Each successful [read(2)] returns a buffer containing one or more of
/// > this structure.
///
/// [inotify(7)]: http://man7.org/linux/man-pages/man7/inotify.7.html
/// [read(2)]: http://man7.org/linux/man-pages/man2/read.2.html
/// [signal(7)]: http://man7.org/linux/man-pages/man7/signal.7.html
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct inotify_event {
    /// Identifies the watch for which this event occurs
    ///
    /// This is one of the watch descriptors returned by a previous call to
    /// [`inotify_add_watch()`].
    ///
    /// [`inotify_add_watch()`]: fn.inotify_add_watch.html
    pub wd: c_int,

    /// Describes the type file system event
    ///
    /// One of the following bits will be set, to identify the type of event:
    ///
    /// - [`IN_ACCESS`]
    /// - [`IN_ATTRIB`]
    /// - [`IN_CLOSE_NOWRITE`]
    /// - [`IN_CLOSE_WRITE`]
    /// - [`IN_CREATE`]
    /// - [`IN_DELETE`]
    /// - [`IN_DELETE_SELF`]
    /// - [`IN_IGNORED`]
    /// - [`IN_MODIFY`]
    /// - [`IN_MOVED_FROM`]
    /// - [`IN_MOVED_TO`]
    /// - [`IN_MOVE_SELF`]
    /// - [`IN_OPEN`]
    /// - [`IN_Q_OVERFLOW`]
    /// - [`IN_UNMOUNT`]
    ///
    /// Some constants cover multiple bits, and can be used for a less precise
    /// check of the event type:
    ///
    /// - [`IN_CLOSE`]
    /// - [`IN_MOVE`]
    ///
    /// In addition, the [`IN_ISDIR`] bit can be set.
    ///
    /// [`IN_ACCESS`]: constant.IN_ACCESS.html
    /// [`IN_ATTRIB`]: constant.IN_ATTRIB.html
    /// [`IN_CLOSE`]: constant.IN_CLOSE.html
    /// [`IN_CLOSE_NOWRITE`]: constant.IN_CLOSE_NOWRITE.html
    /// [`IN_CLOSE_WRITE`]: constant.IN_CLOSE_WRITE.html
    /// [`IN_CREATE`]: constant.IN_CREATE.html
    /// [`IN_DELETE`]: constant.IN_DELETE.html
    /// [`IN_DELETE_SELF`]: constant.IN_DELETE_SELF.html
    /// [`IN_IGNORED`]: constant.IN_IGNORED.html
    /// [`IN_ISDIR`]: constant.IN_ISDIR.html
    /// [`IN_MODIFY`]: constant.IN_MODIFY.html
    /// [`IN_MOVE`]: constant.IN_MOVE.html
    /// [`IN_MOVED_FROM`]: constant.IN_MOVED_FROM.html
    /// [`IN_MOVED_TO`]: constant.IN_MOVED_TO.html
    /// [`IN_MOVE_SELF`]: constant.IN_MOVE_SELF.html
    /// [`IN_OPEN`]: constant.IN_OPEN.html
    /// [`IN_Q_OVERFLOW`]: constant.IN_Q_OVERFLOW.html
    /// [`IN_UNMOUNT`]: constant.IN_UNMOUNT.html
    pub mask: uint32_t,

    /// A number that connects related events
    ///
    /// Currently used only for rename events. A related pair of
    /// [`IN_MOVED_FROM`] and [`IN_MOVED_TO`] events will have the same,
    /// non-zero, cookie. For all other events, cookie is 0.
    ///
    /// [`IN_MOVED_FROM`]: constant.IN_MOVED_FROM.html
    /// [`IN_MOVED_TO`]: constant.IN_MOVED_TO.html
    pub cookie: uint32_t,

    /// The length of `name`
    ///
    /// Used to determine the size of this structure. When `name`
    /// isn't present (`name` is only present when an event occurs
    /// for a file inside a watched directory), it is 0. When `name`
    /// *is* present, it counts all of `name`'s bytes, including `\0`.
    ///
    /// > The `name` field is present only when an event is returned for
    /// > a file inside a watched directory; it identifies the file
    /// > pathname relative to the watched directory. This pathname is
    /// > null-terminated, and may include further null bytes ('\0') to
    /// > align subsequent reads to a suitable address boundary.
    ///
    /// The `name` field has been ommited in this struct's definition.
    pub len: uint32_t,
}


extern {
    /// Creates an inotify instance
    ///
    /// If you need more flexibility, consider using [`inotify_init1`] instead.
    ///
    /// Returns `-1`, if an error occured, or an inotify file descriptor
    /// otherwise.
    ///
    /// Please refer to the [man page] for additional details.
    ///
    /// [`inotify_init1`]: fn.inotify_init1.html
    /// [man page]: http://man7.org/linux/man-pages/man2/inotify_init.2.html
    pub fn inotify_init() -> c_int;

    /// Creates an inotify instance
    ///
    /// Takes an argument to configure the new inotify instance. The following
    /// flags can be set:
    ///
    /// - [`IN_CLOEXEC`]
    /// - [`IN_NONBLOCK`]
    ///
    /// Returns `-1`, if an error occured, or an inotify file descriptor
    /// otherwise.
    ///
    /// Please refer to the [man page] for additional details.
    ///
    /// [`IN_CLOEXEC`]: constant.IN_CLOEXEC.html
    /// [`IN_NONBLOCK`]: constant.IN_NONBLOCK.html
    /// [man page]: http://man7.org/linux/man-pages/man2/inotify_init1.2.html
    pub fn inotify_init1(flags: c_int) -> c_int;

    /// Adds or updates an inotify watch
    ///
    /// Adds an item to the watch list of an inotify instance, or modifies an
    /// item on that list. This function takes the following arguments:
    ///
    /// - `fd` is the file descriptor of the inotify instance (created by
    ///   [`inotify_init`] or [`inotify_init1`])
    /// - `pathname` is the path of the file or directory watch
    /// - `mask` defines the behavior of this function and configures the watch
    ///
    /// The following flags in `mask` control the type of events to watch for:
    ///
    /// - [`IN_ACCESS`]
    /// - [`IN_ATTRIB`]
    /// - [`IN_CLOSE_NOWRITE`]
    /// - [`IN_CLOSE_WRITE`]
    /// - [`IN_CREATE`]
    /// - [`IN_DELETE`]
    /// - [`IN_DELETE_SELF`]
    /// - [`IN_MODIFY`]
    /// - [`IN_MOVED_FROM`]
    /// - [`IN_MOVED_TO`]
    /// - [`IN_MOVE_SELF`]
    /// - [`IN_OPEN`]
    ///
    /// The following constants can be used as shortcuts to set multiple event
    /// flags:
    ///
    /// - [`IN_ALL_EVENTS`]
    /// - [`IN_CLOSE`]
    /// - [`IN_MOVE`]
    ///
    /// In addition, the following flags can be set to control the behaviors of
    /// the watch and this function:
    ///
    /// - [`IN_DONT_FOLLOW`]
    /// - [`IN_EXCL_UNLINK`]
    /// - [`IN_MASK_ADD`]
    /// - [`IN_ONESHOT`]
    /// - [`IN_ONLYDIR`]
    ///
    /// The function returns `-1` if an error occured. Otherwise, it returns a
    /// watch descriptor that can be used to remove the watch using
    /// [`inotify_rm_watch`] or identify the watch via [`inotify_event`]'s [wd`]
    /// field.
    ///
    /// Please refer to the [man page] for additional details.
    ///
    /// [`inotify_init`]: fn.inotify_init.html
    /// [`inotify_init1`]: fn.inotify_init1.html
    /// [`IN_ACCESS`]: constant.IN_ACCESS.html
    /// [`IN_ATTRIB`]: constant.IN_ATTRIB.html
    /// [`IN_CLOSE_NOWRITE`]: constant.IN_CLOSE_NOWRITE.html
    /// [`IN_CLOSE_WRITE`]: constant.IN_CLOSE_WRITE.html
    /// [`IN_CREATE`]: constant.IN_CREATE.html
    /// [`IN_DELETE`]: constant.IN_DELETE.html
    /// [`IN_DELETE_SELF`]: constant.IN_DELETE_SELF.html
    /// [`IN_MODIFY`]: constant.IN_MODIFY.html
    /// [`IN_MOVED_FROM`]: constant.IN_MOVED_FROM.html
    /// [`IN_MOVED_TO`]: constant.IN_MOVED_TO.html
    /// [`IN_MOVE_SELF`]: constant.IN_MOVE_SELF.html
    /// [`IN_OPEN`]: constant.IN_OPEN.html
    /// [`IN_ALL_EVENTS`]: constant.IN_ALL_EVENTS.html
    /// [`IN_CLOSE`]: constant.IN_CLOSE.html
    /// [`IN_MOVE`]: constant.IN_MOVE.html
    /// [`IN_DONT_FOLLOW`]: constant.IN_DONT_FOLLOW.html
    /// [`IN_EXCL_UNLINK`]: constant.IN_EXCL_UNLINK.html
    /// [`IN_MASK_ADD`]: constant.IN_MASK_ADD.html
    /// [`IN_ONESHOT`]: constant.IN_ONESHOT.html
    /// [`IN_ONLYDIR`]: constant.IN_ONLYDIR.html
    /// [`inotify_rm_watch`]: fn.inotify_rm_watch.html
    /// [`inotify_event`]: struct.inotify_event.html
    /// [`wd`]: struct.inotify_event.html#structfield.wd
    /// [man page]: http://man7.org/linux/man-pages/man2/inotify_add_watch.2.html
    pub fn inotify_add_watch(fd: c_int, pathname: *const c_char, mask: uint32_t) -> c_int;

    /// Removes an inotify watch
    ///
    /// Removes an item from the watch list of an inotify instance. The inotify
    /// instance is identified by the `fd` argument. The watch is identified by
    /// the `wd` argument.
    ///
    /// Returns `0` on success, `-1` on failure.
    ///
    /// Please refer to the [man page] for additional details.
    ///
    /// [man page]: http://man7.org/linux/man-pages/man2/inotify_rm_watch.2.html
    pub fn inotify_rm_watch(fd: c_int, wd: c_int) -> c_int;
}

pub use libc::{
    close,
    read,
};
