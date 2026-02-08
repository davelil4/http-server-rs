/// Find the starting index of `needle` within `haystack`.
///
/// Returns `None` if the subsequence is not present.
///
/// # Phase 1
/// Used to detect the end of HTTP headers.
pub fn find_subslice(
    haystack: &[u8],
    needle: &[u8]
) -> Option<usize>;

/// Attempt to split a buffer into a complete frame and remainder
/// using the given delimiter.
///
/// Returns the frame and the remaining bytes if the delimiter
/// is found.
///
/// # Phase 1
/// Optional helper for framing request data.
pub fn try_split_from(
    buf: &'a [u8],
    frame_delim: &[u8]
) -> Option<&'a [u8], &'a [u8]>;
// The lifetime explicitely states that the outputted slices are specifically
// slices from 'buf' and not from 'frame_delim'. They must live as long as
// that buffer lives