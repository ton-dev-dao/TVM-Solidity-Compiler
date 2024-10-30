/* automatically generated by rust-bindgen 0.60.1 */

pub const true_: u32 = 1;
pub const false_: u32 = 0;
pub const __bool_true_false_are_defined: u32 = 1;
pub type size_t = ::std::os::raw::c_ulonglong;
pub type wchar_t = ::std::os::raw::c_ushort;
pub type max_align_t = f64;
#[doc = " Callback used to retrieve additional source files or data."]
#[doc = ""]
#[doc = " @param _context The readContext passed to solidity_compile. Can be NULL."]
#[doc = " @param _kind The kind of callback (a string)."]
#[doc = " @param _data The data for the callback (a string)."]
#[doc = " @param o_contents A pointer to the contents of the file, if found. Allocated via solidity_alloc()."]
#[doc = " @param o_error A pointer to an error message, if there is one."]
#[doc = ""]
#[doc = " The file (as well as error) contents that is to be allocated by the callback"]
#[doc = " implementor must use the solidity_alloc() API to allocate its underlying"]
#[doc = " storage. Ownership is then transferred to the compiler which will take care"]
#[doc = " of the deallocation."]
#[doc = ""]
#[doc = " If the callback is not supported, *o_contents and *o_error must be set to NULL."]
pub type CStyleReadFileCallback = ::std::option::Option<
    unsafe extern "C" fn(
        _context: *mut ::std::os::raw::c_void,
        _kind: *const ::std::os::raw::c_char,
        _data: *const ::std::os::raw::c_char,
        o_contents: *mut *mut ::std::os::raw::c_char,
        o_error: *mut *mut ::std::os::raw::c_char,
    ),
>;
extern "C" {
    #[doc = " Returns the complete license document."]
    #[doc = ""]
    #[doc = " The pointer returned must NOT be freed by the caller."]
    pub fn solidity_license() -> *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " Returns the compiler version."]
    #[doc = ""]
    #[doc = " The pointer returned must NOT be freed by the caller."]
    pub fn solidity_version() -> *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " Allocates a chunk of memory of @p _size bytes."]
    #[doc = ""]
    #[doc = " Use this function inside callbacks to allocate data that is to be passed to"]
    #[doc = " the compiler. You may use solidity_free() or solidity_reset() to free this"]
    #[doc = " memory again but it is not required as the compiler takes ownership for any"]
    #[doc = " data passed to it via callbacks."]
    #[doc = ""]
    #[doc = " This function will return NULL if the requested memory region could not be allocated."]
    pub fn solidity_alloc(_size: size_t) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " Explicitly frees the memory (@p _data) that was being allocated with solidity_alloc()"]
    #[doc = " or returned by a call to solidity_compile()."]
    #[doc = ""]
    #[doc = " Important, this call will abort() in case of any invalid argument being passed to this call."]
    pub fn solidity_free(_data: *mut ::std::os::raw::c_char);
}
extern "C" {
    #[doc = " Takes a \"Standard Input JSON\" and an optional callback (can be set to null). Returns"]
    #[doc = " a \"Standard Output JSON\". Both are to be UTF-8 encoded."]
    #[doc = ""]
    #[doc = " @param _input The input JSON to process."]
    #[doc = " @param _readCallback The optional callback pointer. Can be NULL, but if not NULL,"]
    #[doc = "                      it can be called by the compiler to request additional input."]
    #[doc = "                      Please see the documentation of the type for details."]
    #[doc = " @param _readContext An optional context pointer passed to _readCallback. Can be NULL."]
    #[doc = ""]
    #[doc = " @returns A pointer to the result. The pointer returned must be freed by the caller using solidity_free() or solidity_reset()."]
    pub fn solidity_compile(
        _input: *const ::std::os::raw::c_char,
        _readCallback: CStyleReadFileCallback,
        _readContext: *mut ::std::os::raw::c_void,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " Frees up any allocated memory."]
    #[doc = ""]
    #[doc = " NOTE: the pointer returned by solidity_compile as well as any other pointer retrieved via solidity_alloc()"]
    #[doc = " is invalid after calling this!"]
    pub fn solidity_reset();
}
extern "C" {
    pub fn file_reader_new() -> *mut ::std::os::raw::c_void;
    pub fn file_reader_set_base_path(
        fr: *mut ::std::os::raw::c_void,
        path: *const ::std::os::raw::c_char,
    );
    pub fn file_reader_add_include_path(
        fr: *mut ::std::os::raw::c_void,
        path: *const ::std::os::raw::c_char,
    );
    pub fn file_reader_allow_directory(
        fr: *mut ::std::os::raw::c_void,
        path: *const ::std::os::raw::c_char,
    );
    pub fn file_reader_add_or_update_file(
        fr: *mut ::std::os::raw::c_void,
        path: *const ::std::os::raw::c_char,
        content: *const ::std::os::raw::c_char,
    );
    pub fn file_reader_source_unit_name(
        fr: *mut ::std::os::raw::c_void,
        path: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
    pub fn file_reader_read(
        fr: *mut ::std::os::raw::c_void,
        name: *const ::std::os::raw::c_char,
        success: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
