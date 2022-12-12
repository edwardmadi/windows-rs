#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn BstrFromVector(psa: *const super::Com::SAFEARRAY) -> ::windows::core::Result<::windows::core::BSTR> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn BstrFromVector ( psa : *const super::Com:: SAFEARRAY , pbstr : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    BstrFromVector(psa, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn ClearCustData(pcustdata: *mut super::Com::CUSTDATA) {
    ::windows::core::link ! ( "oleaut32.dll""system" fn ClearCustData ( pcustdata : *mut super::Com:: CUSTDATA ) -> ( ) );
    ClearCustData(pcustdata)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn CreateDispTypeInfo(pidata: *mut INTERFACEDATA, lcid: u32, pptinfo: *mut ::core::option::Option<super::Com::ITypeInfo>) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn CreateDispTypeInfo ( pidata : *mut INTERFACEDATA , lcid : u32 , pptinfo : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    CreateDispTypeInfo(pidata, lcid, ::core::mem::transmute(pptinfo)).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn CreateErrorInfo() -> ::windows::core::Result<ICreateErrorInfo> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn CreateErrorInfo ( pperrinfo : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CreateErrorInfo(result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn CreateOleAdviseHolder() -> ::windows::core::Result<IOleAdviseHolder> {
    ::windows::core::link ! ( "ole32.dll""system" fn CreateOleAdviseHolder ( ppoaholder : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CreateOleAdviseHolder(result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn CreateStdDispatch<P0, P1>(punkouter: P0, pvthis: *mut ::core::ffi::c_void, ptinfo: P1, ppunkstddisp: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    P1: ::std::convert::Into<::windows::core::InParam<super::Com::ITypeInfo>>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn CreateStdDispatch ( punkouter : * mut::core::ffi::c_void , pvthis : *mut ::core::ffi::c_void , ptinfo : * mut::core::ffi::c_void , ppunkstddisp : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    CreateStdDispatch(punkouter.into().abi(), pvthis, ptinfo.into().abi(), ::core::mem::transmute(ppunkstddisp)).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn CreateTypeLib<P0>(syskind: super::Com::SYSKIND, szfile: P0) -> ::windows::core::Result<ICreateTypeLib>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn CreateTypeLib ( syskind : super::Com:: SYSKIND , szfile : :: windows::core::PCWSTR , ppctlib : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CreateTypeLib(syskind, szfile.into().abi(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn CreateTypeLib2<P0>(syskind: super::Com::SYSKIND, szfile: P0) -> ::windows::core::Result<ICreateTypeLib2>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn CreateTypeLib2 ( syskind : super::Com:: SYSKIND , szfile : :: windows::core::PCWSTR , ppctlib : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CreateTypeLib2(syskind, szfile.into().abi(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn DispCallFunc(pvinstance: ::core::option::Option<*const ::core::ffi::c_void>, ovft: usize, cc: super::Com::CALLCONV, vtreturn: super::Com::VARENUM, cactuals: u32, prgvt: *const u16, prgpvarg: *const *const super::Com::VARIANT) -> ::windows::core::Result<super::Com::VARIANT> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn DispCallFunc ( pvinstance : *const ::core::ffi::c_void , ovft : usize , cc : super::Com:: CALLCONV , vtreturn : super::Com:: VARENUM , cactuals : u32 , prgvt : *const u16 , prgpvarg : *const *const super::Com:: VARIANT , pvargresult : *mut super::Com:: VARIANT ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    DispCallFunc(::core::mem::transmute(pvinstance.unwrap_or(::std::ptr::null())), ovft, cc, vtreturn, cactuals, prgvt, prgpvarg, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn DispGetIDsOfNames<P0>(ptinfo: P0, rgsznames: *const ::windows::core::PCWSTR, cnames: u32, rgdispid: *mut i32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<super::Com::ITypeInfo>>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn DispGetIDsOfNames ( ptinfo : * mut::core::ffi::c_void , rgsznames : *const :: windows::core::PCWSTR , cnames : u32 , rgdispid : *mut i32 ) -> :: windows::core::HRESULT );
    DispGetIDsOfNames(ptinfo.into().abi(), rgsznames, cnames, rgdispid).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn DispGetParam(pdispparams: *const super::Com::DISPPARAMS, position: u32, vttarg: super::Com::VARENUM, pvarresult: *mut super::Com::VARIANT, puargerr: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn DispGetParam ( pdispparams : *const super::Com:: DISPPARAMS , position : u32 , vttarg : super::Com:: VARENUM , pvarresult : *mut super::Com:: VARIANT , puargerr : *mut u32 ) -> :: windows::core::HRESULT );
    DispGetParam(pdispparams, position, vttarg, pvarresult, ::core::mem::transmute(puargerr.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn DispInvoke<P0>(_this: *mut ::core::ffi::c_void, ptinfo: P0, dispidmember: i32, wflags: u16, pparams: *mut super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<super::Com::ITypeInfo>>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn DispInvoke ( _this : *mut ::core::ffi::c_void , ptinfo : * mut::core::ffi::c_void , dispidmember : i32 , wflags : u16 , pparams : *mut super::Com:: DISPPARAMS , pvarresult : *mut super::Com:: VARIANT , pexcepinfo : *mut super::Com:: EXCEPINFO , puargerr : *mut u32 ) -> :: windows::core::HRESULT );
    DispInvoke(_this, ptinfo.into().abi(), dispidmember, wflags, pparams, pvarresult, pexcepinfo, puargerr).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn DoDragDrop<P0, P1>(pdataobj: P0, pdropsource: P1, dwokeffects: DROPEFFECT, pdweffect: *mut DROPEFFECT) -> ::windows::core::HRESULT
where
    P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDataObject>>,
    P1: ::std::convert::Into<::windows::core::InParam<IDropSource>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn DoDragDrop ( pdataobj : * mut::core::ffi::c_void , pdropsource : * mut::core::ffi::c_void , dwokeffects : DROPEFFECT , pdweffect : *mut DROPEFFECT ) -> :: windows::core::HRESULT );
    DoDragDrop(pdataobj.into().abi(), pdropsource.into().abi(), dwokeffects, pdweffect)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn DosDateTimeToVariantTime(wdosdate: u16, wdostime: u16, pvtime: *mut f64) -> i32 {
    ::windows::core::link ! ( "oleaut32.dll""system" fn DosDateTimeToVariantTime ( wdosdate : u16 , wdostime : u16 , pvtime : *mut f64 ) -> i32 );
    DosDateTimeToVariantTime(wdosdate, wdostime, pvtime)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn GetActiveObject(rclsid: *const ::windows::core::GUID, pvreserved: *mut ::core::ffi::c_void, ppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn GetActiveObject ( rclsid : *const :: windows::core::GUID , pvreserved : *mut ::core::ffi::c_void , ppunk : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    GetActiveObject(rclsid, pvreserved, ::core::mem::transmute(ppunk)).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn GetAltMonthNames(lcid: u32) -> ::windows::core::Result<*mut ::windows::core::PWSTR> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn GetAltMonthNames ( lcid : u32 , prgp : *mut *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    GetAltMonthNames(lcid, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn GetRecordInfoFromGuids(rguidtypelib: *const ::windows::core::GUID, uvermajor: u32, uverminor: u32, lcid: u32, rguidtypeinfo: *const ::windows::core::GUID) -> ::windows::core::Result<IRecordInfo> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn GetRecordInfoFromGuids ( rguidtypelib : *const :: windows::core::GUID , uvermajor : u32 , uverminor : u32 , lcid : u32 , rguidtypeinfo : *const :: windows::core::GUID , pprecinfo : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    GetRecordInfoFromGuids(rguidtypelib, uvermajor, uverminor, lcid, rguidtypeinfo, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn GetRecordInfoFromTypeInfo<P0>(ptypeinfo: P0) -> ::windows::core::Result<IRecordInfo>
where
    P0: ::std::convert::Into<::windows::core::InParam<super::Com::ITypeInfo>>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn GetRecordInfoFromTypeInfo ( ptypeinfo : * mut::core::ffi::c_void , pprecinfo : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    GetRecordInfoFromTypeInfo(ptypeinfo.into().abi(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HRGN_UserFree(param0: *const u32, param1: *const super::super::Graphics::Gdi::HRGN) {
    ::windows::core::link ! ( "ole32.dll""system" fn HRGN_UserFree ( param0 : *const u32 , param1 : *const super::super::Graphics::Gdi:: HRGN ) -> ( ) );
    HRGN_UserFree(param0, param1)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HRGN_UserFree64(param0: *const u32, param1: *const super::super::Graphics::Gdi::HRGN) {
    ::windows::core::link ! ( "api-ms-win-core-marshal-l1-1-0.dll""system" fn HRGN_UserFree64 ( param0 : *const u32 , param1 : *const super::super::Graphics::Gdi:: HRGN ) -> ( ) );
    HRGN_UserFree64(param0, param1)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HRGN_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::Graphics::Gdi::HRGN) -> *mut u8 {
    ::windows::core::link ! ( "ole32.dll""system" fn HRGN_UserMarshal ( param0 : *const u32 , param1 : *mut u8 , param2 : *const super::super::Graphics::Gdi:: HRGN ) -> *mut u8 );
    HRGN_UserMarshal(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HRGN_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::Graphics::Gdi::HRGN) -> *mut u8 {
    ::windows::core::link ! ( "api-ms-win-core-marshal-l1-1-0.dll""system" fn HRGN_UserMarshal64 ( param0 : *const u32 , param1 : *mut u8 , param2 : *const super::super::Graphics::Gdi:: HRGN ) -> *mut u8 );
    HRGN_UserMarshal64(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HRGN_UserSize(param0: *const u32, param1: u32, param2: *const super::super::Graphics::Gdi::HRGN) -> u32 {
    ::windows::core::link ! ( "ole32.dll""system" fn HRGN_UserSize ( param0 : *const u32 , param1 : u32 , param2 : *const super::super::Graphics::Gdi:: HRGN ) -> u32 );
    HRGN_UserSize(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HRGN_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::Graphics::Gdi::HRGN) -> u32 {
    ::windows::core::link ! ( "api-ms-win-core-marshal-l1-1-0.dll""system" fn HRGN_UserSize64 ( param0 : *const u32 , param1 : u32 , param2 : *const super::super::Graphics::Gdi:: HRGN ) -> u32 );
    HRGN_UserSize64(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HRGN_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::Graphics::Gdi::HRGN) -> *mut u8 {
    ::windows::core::link ! ( "ole32.dll""system" fn HRGN_UserUnmarshal ( param0 : *const u32 , param1 : *const u8 , param2 : *mut super::super::Graphics::Gdi:: HRGN ) -> *mut u8 );
    HRGN_UserUnmarshal(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HRGN_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::Graphics::Gdi::HRGN) -> *mut u8 {
    ::windows::core::link ! ( "api-ms-win-core-marshal-l1-1-0.dll""system" fn HRGN_UserUnmarshal64 ( param0 : *const u32 , param1 : *const u8 , param2 : *mut super::super::Graphics::Gdi:: HRGN ) -> *mut u8 );
    HRGN_UserUnmarshal64(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn IsAccelerator<P0>(haccel: P0, caccelentries: i32, lpmsg: *const super::super::UI::WindowsAndMessaging::MSG, lpwcmd: *mut u16) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::UI::WindowsAndMessaging::HACCEL>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn IsAccelerator ( haccel : super::super::UI::WindowsAndMessaging:: HACCEL , caccelentries : i32 , lpmsg : *const super::super::UI::WindowsAndMessaging:: MSG , lpwcmd : *mut u16 ) -> super::super::Foundation:: BOOL );
    IsAccelerator(haccel.into(), caccelentries, lpmsg, lpwcmd)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn LHashValOfNameSys<P0>(syskind: super::Com::SYSKIND, lcid: u32, szname: P0) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn LHashValOfNameSys ( syskind : super::Com:: SYSKIND , lcid : u32 , szname : :: windows::core::PCWSTR ) -> u32 );
    LHashValOfNameSys(syskind, lcid, szname.into().abi())
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn LHashValOfNameSysA<P0>(syskind: super::Com::SYSKIND, lcid: u32, szname: P0) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn LHashValOfNameSysA ( syskind : super::Com:: SYSKIND , lcid : u32 , szname : :: windows::core::PCSTR ) -> u32 );
    LHashValOfNameSysA(syskind, lcid, szname.into().abi())
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn LoadRegTypeLib(rguid: *const ::windows::core::GUID, wvermajor: u16, wverminor: u16, lcid: u32) -> ::windows::core::Result<super::Com::ITypeLib> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn LoadRegTypeLib ( rguid : *const :: windows::core::GUID , wvermajor : u16 , wverminor : u16 , lcid : u32 , pptlib : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    LoadRegTypeLib(rguid, wvermajor, wverminor, lcid, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn LoadTypeLib<P0>(szfile: P0) -> ::windows::core::Result<super::Com::ITypeLib>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn LoadTypeLib ( szfile : :: windows::core::PCWSTR , pptlib : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    LoadTypeLib(szfile.into().abi(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn LoadTypeLibEx<P0>(szfile: P0, regkind: REGKIND) -> ::windows::core::Result<super::Com::ITypeLib>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn LoadTypeLibEx ( szfile : :: windows::core::PCWSTR , regkind : REGKIND , pptlib : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    LoadTypeLibEx(szfile.into().abi(), regkind, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn OaBuildVersion() -> u32 {
    ::windows::core::link ! ( "oleaut32.dll""system" fn OaBuildVersion ( ) -> u32 );
    OaBuildVersion()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn OaEnablePerUserTLibRegistration() {
    ::windows::core::link ! ( "oleaut32.dll""system" fn OaEnablePerUserTLibRegistration ( ) -> ( ) );
    OaEnablePerUserTLibRegistration()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn OleBuildVersion() -> u32 {
    ::windows::core::link ! ( "ole32.dll""system" fn OleBuildVersion ( ) -> u32 );
    OleBuildVersion()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn OleCreate<P0, P1>(rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, renderopt: OLERENDER, pformatetc: *const super::Com::FORMATETC, pclientsite: P0, pstg: P1, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<IOleClientSite>>,
    P1: ::std::convert::Into<::windows::core::InParam<super::Com::StructuredStorage::IStorage>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn OleCreate ( rclsid : *const :: windows::core::GUID , riid : *const :: windows::core::GUID , renderopt : OLERENDER , pformatetc : *const super::Com:: FORMATETC , pclientsite : * mut::core::ffi::c_void , pstg : * mut::core::ffi::c_void , ppvobj : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    OleCreate(rclsid, riid, renderopt, pformatetc, pclientsite.into().abi(), pstg.into().abi(), ppvobj).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn OleCreateDefaultHandler<P0>(clsid: *const ::windows::core::GUID, punkouter: P0, riid: *const ::windows::core::GUID, lplpobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn OleCreateDefaultHandler ( clsid : *const :: windows::core::GUID , punkouter : * mut::core::ffi::c_void , riid : *const :: windows::core::GUID , lplpobj : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    OleCreateDefaultHandler(clsid, punkouter.into().abi(), riid, lplpobj).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn OleCreateEmbeddingHelper<P0, P1>(clsid: *const ::windows::core::GUID, punkouter: P0, flags: EMBDHLP_FLAGS, pcf: P1, riid: *const ::windows::core::GUID, lplpobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    P1: ::std::convert::Into<::windows::core::InParam<super::Com::IClassFactory>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn OleCreateEmbeddingHelper ( clsid : *const :: windows::core::GUID , punkouter : * mut::core::ffi::c_void , flags : EMBDHLP_FLAGS , pcf : * mut::core::ffi::c_void , riid : *const :: windows::core::GUID , lplpobj : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    OleCreateEmbeddingHelper(clsid, punkouter.into().abi(), flags, pcf.into().abi(), riid, lplpobj).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn OleCreateEx<P0, P1, P2>(rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, dwflags: OLECREATE, renderopt: OLERENDER, cformats: u32, rgadvf: *const u32, rgformatetc: *const super::Com::FORMATETC, lpadvisesink: P0, rgdwconnection: *mut u32, pclientsite: P1, pstg: P2, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<super::Com::IAdviseSink>>,
    P1: ::std::convert::Into<::windows::core::InParam<IOleClientSite>>,
    P2: ::std::convert::Into<::windows::core::InParam<super::Com::StructuredStorage::IStorage>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn OleCreateEx ( rclsid : *const :: windows::core::GUID , riid : *const :: windows::core::GUID , dwflags : OLECREATE , renderopt : OLERENDER , cformats : u32 , rgadvf : *const u32 , rgformatetc : *const super::Com:: FORMATETC , lpadvisesink : * mut::core::ffi::c_void , rgdwconnection : *mut u32 , pclientsite : * mut::core::ffi::c_void , pstg : * mut::core::ffi::c_void , ppvobj : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    OleCreateEx(rclsid, riid, dwflags, renderopt, cformats, rgadvf, rgformatetc, lpadvisesink.into().abi(), rgdwconnection, pclientsite.into().abi(), pstg.into().abi(), ppvobj).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn OleCreateFontIndirect(lpfontdesc: *const FONTDESC, riid: *const ::windows::core::GUID, lplpvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn OleCreateFontIndirect ( lpfontdesc : *const FONTDESC , riid : *const :: windows::core::GUID , lplpvobj : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    OleCreateFontIndirect(lpfontdesc, riid, lplpvobj).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn OleCreateFromData<P0, P1, P2>(psrcdataobj: P0, riid: *const ::windows::core::GUID, renderopt: OLERENDER, pformatetc: *const super::Com::FORMATETC, pclientsite: P1, pstg: P2, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDataObject>>,
    P1: ::std::convert::Into<::windows::core::InParam<IOleClientSite>>,
    P2: ::std::convert::Into<::windows::core::InParam<super::Com::StructuredStorage::IStorage>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn OleCreateFromData ( psrcdataobj : * mut::core::ffi::c_void , riid : *const :: windows::core::GUID , renderopt : OLERENDER , pformatetc : *const super::Com:: FORMATETC , pclientsite : * mut::core::ffi::c_void , pstg : * mut::core::ffi::c_void , ppvobj : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    OleCreateFromData(psrcdataobj.into().abi(), riid, renderopt, pformatetc, pclientsite.into().abi(), pstg.into().abi(), ppvobj).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn OleCreateFromDataEx<P0, P1, P2, P3>(psrcdataobj: P0, riid: *const ::windows::core::GUID, dwflags: OLECREATE, renderopt: OLERENDER, cformats: u32, rgadvf: *const u32, rgformatetc: *const super::Com::FORMATETC, lpadvisesink: P1, rgdwconnection: *mut u32, pclientsite: P2, pstg: P3, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDataObject>>,
    P1: ::std::convert::Into<::windows::core::InParam<super::Com::IAdviseSink>>,
    P2: ::std::convert::Into<::windows::core::InParam<IOleClientSite>>,
    P3: ::std::convert::Into<::windows::core::InParam<super::Com::StructuredStorage::IStorage>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn OleCreateFromDataEx ( psrcdataobj : * mut::core::ffi::c_void , riid : *const :: windows::core::GUID , dwflags : OLECREATE , renderopt : OLERENDER , cformats : u32 , rgadvf : *const u32 , rgformatetc : *const super::Com:: FORMATETC , lpadvisesink : * mut::core::ffi::c_void , rgdwconnection : *mut u32 , pclientsite : * mut::core::ffi::c_void , pstg : * mut::core::ffi::c_void , ppvobj : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    OleCreateFromDataEx(psrcdataobj.into().abi(), riid, dwflags, renderopt, cformats, rgadvf, rgformatetc, lpadvisesink.into().abi(), rgdwconnection, pclientsite.into().abi(), pstg.into().abi(), ppvobj).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn OleCreateFromFile<P0, P1, P2>(rclsid: *const ::windows::core::GUID, lpszfilename: P0, riid: *const ::windows::core::GUID, renderopt: OLERENDER, lpformatetc: *const super::Com::FORMATETC, pclientsite: P1, pstg: P2, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<IOleClientSite>>,
    P2: ::std::convert::Into<::windows::core::InParam<super::Com::StructuredStorage::IStorage>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn OleCreateFromFile ( rclsid : *const :: windows::core::GUID , lpszfilename : :: windows::core::PCWSTR , riid : *const :: windows::core::GUID , renderopt : OLERENDER , lpformatetc : *const super::Com:: FORMATETC , pclientsite : * mut::core::ffi::c_void , pstg : * mut::core::ffi::c_void , ppvobj : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    OleCreateFromFile(rclsid, lpszfilename.into().abi(), riid, renderopt, lpformatetc, pclientsite.into().abi(), pstg.into().abi(), ppvobj).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn OleCreateFromFileEx<P0, P1, P2, P3>(rclsid: *const ::windows::core::GUID, lpszfilename: P0, riid: *const ::windows::core::GUID, dwflags: OLECREATE, renderopt: OLERENDER, cformats: u32, rgadvf: *const u32, rgformatetc: *const super::Com::FORMATETC, lpadvisesink: P1, rgdwconnection: *mut u32, pclientsite: P2, pstg: P3, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<super::Com::IAdviseSink>>,
    P2: ::std::convert::Into<::windows::core::InParam<IOleClientSite>>,
    P3: ::std::convert::Into<::windows::core::InParam<super::Com::StructuredStorage::IStorage>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn OleCreateFromFileEx ( rclsid : *const :: windows::core::GUID , lpszfilename : :: windows::core::PCWSTR , riid : *const :: windows::core::GUID , dwflags : OLECREATE , renderopt : OLERENDER , cformats : u32 , rgadvf : *const u32 , rgformatetc : *const super::Com:: FORMATETC , lpadvisesink : * mut::core::ffi::c_void , rgdwconnection : *mut u32 , pclientsite : * mut::core::ffi::c_void , pstg : * mut::core::ffi::c_void , ppvobj : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    OleCreateFromFileEx(rclsid, lpszfilename.into().abi(), riid, dwflags, renderopt, cformats, rgadvf, rgformatetc, lpadvisesink.into().abi(), rgdwconnection, pclientsite.into().abi(), pstg.into().abi(), ppvobj).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn OleCreateLink<P0, P1, P2>(pmklinksrc: P0, riid: *const ::windows::core::GUID, renderopt: OLERENDER, lpformatetc: *const super::Com::FORMATETC, pclientsite: P1, pstg: P2, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<super::Com::IMoniker>>,
    P1: ::std::convert::Into<::windows::core::InParam<IOleClientSite>>,
    P2: ::std::convert::Into<::windows::core::InParam<super::Com::StructuredStorage::IStorage>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn OleCreateLink ( pmklinksrc : * mut::core::ffi::c_void , riid : *const :: windows::core::GUID , renderopt : OLERENDER , lpformatetc : *const super::Com:: FORMATETC , pclientsite : * mut::core::ffi::c_void , pstg : * mut::core::ffi::c_void , ppvobj : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    OleCreateLink(pmklinksrc.into().abi(), riid, renderopt, lpformatetc, pclientsite.into().abi(), pstg.into().abi(), ppvobj).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn OleCreateLinkEx<P0, P1, P2, P3>(pmklinksrc: P0, riid: *const ::windows::core::GUID, dwflags: OLECREATE, renderopt: OLERENDER, cformats: u32, rgadvf: *const u32, rgformatetc: *const super::Com::FORMATETC, lpadvisesink: P1, rgdwconnection: *mut u32, pclientsite: P2, pstg: P3, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<super::Com::IMoniker>>,
    P1: ::std::convert::Into<::windows::core::InParam<super::Com::IAdviseSink>>,
    P2: ::std::convert::Into<::windows::core::InParam<IOleClientSite>>,
    P3: ::std::convert::Into<::windows::core::InParam<super::Com::StructuredStorage::IStorage>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn OleCreateLinkEx ( pmklinksrc : * mut::core::ffi::c_void , riid : *const :: windows::core::GUID , dwflags : OLECREATE , renderopt : OLERENDER , cformats : u32 , rgadvf : *const u32 , rgformatetc : *const super::Com:: FORMATETC , lpadvisesink : * mut::core::ffi::c_void , rgdwconnection : *mut u32 , pclientsite : * mut::core::ffi::c_void , pstg : * mut::core::ffi::c_void , ppvobj : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    OleCreateLinkEx(pmklinksrc.into().abi(), riid, dwflags, renderopt, cformats, rgadvf, rgformatetc, lpadvisesink.into().abi(), rgdwconnection, pclientsite.into().abi(), pstg.into().abi(), ppvobj).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn OleCreateLinkFromData<P0, P1, P2>(psrcdataobj: P0, riid: *const ::windows::core::GUID, renderopt: OLERENDER, pformatetc: *const super::Com::FORMATETC, pclientsite: P1, pstg: P2, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDataObject>>,
    P1: ::std::convert::Into<::windows::core::InParam<IOleClientSite>>,
    P2: ::std::convert::Into<::windows::core::InParam<super::Com::StructuredStorage::IStorage>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn OleCreateLinkFromData ( psrcdataobj : * mut::core::ffi::c_void , riid : *const :: windows::core::GUID , renderopt : OLERENDER , pformatetc : *const super::Com:: FORMATETC , pclientsite : * mut::core::ffi::c_void , pstg : * mut::core::ffi::c_void , ppvobj : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    OleCreateLinkFromData(psrcdataobj.into().abi(), riid, renderopt, pformatetc, pclientsite.into().abi(), pstg.into().abi(), ppvobj).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn OleCreateLinkFromDataEx<P0, P1, P2, P3>(psrcdataobj: P0, riid: *const ::windows::core::GUID, dwflags: OLECREATE, renderopt: OLERENDER, cformats: u32, rgadvf: *const u32, rgformatetc: *const super::Com::FORMATETC, lpadvisesink: P1, rgdwconnection: *mut u32, pclientsite: P2, pstg: P3, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDataObject>>,
    P1: ::std::convert::Into<::windows::core::InParam<super::Com::IAdviseSink>>,
    P2: ::std::convert::Into<::windows::core::InParam<IOleClientSite>>,
    P3: ::std::convert::Into<::windows::core::InParam<super::Com::StructuredStorage::IStorage>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn OleCreateLinkFromDataEx ( psrcdataobj : * mut::core::ffi::c_void , riid : *const :: windows::core::GUID , dwflags : OLECREATE , renderopt : OLERENDER , cformats : u32 , rgadvf : *const u32 , rgformatetc : *const super::Com:: FORMATETC , lpadvisesink : * mut::core::ffi::c_void , rgdwconnection : *mut u32 , pclientsite : * mut::core::ffi::c_void , pstg : * mut::core::ffi::c_void , ppvobj : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    OleCreateLinkFromDataEx(psrcdataobj.into().abi(), riid, dwflags, renderopt, cformats, rgadvf, rgformatetc, lpadvisesink.into().abi(), rgdwconnection, pclientsite.into().abi(), pstg.into().abi(), ppvobj).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn OleCreateLinkToFile<P0, P1, P2>(lpszfilename: P0, riid: *const ::windows::core::GUID, renderopt: OLERENDER, lpformatetc: *const super::Com::FORMATETC, pclientsite: P1, pstg: P2, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<IOleClientSite>>,
    P2: ::std::convert::Into<::windows::core::InParam<super::Com::StructuredStorage::IStorage>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn OleCreateLinkToFile ( lpszfilename : :: windows::core::PCWSTR , riid : *const :: windows::core::GUID , renderopt : OLERENDER , lpformatetc : *const super::Com:: FORMATETC , pclientsite : * mut::core::ffi::c_void , pstg : * mut::core::ffi::c_void , ppvobj : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    OleCreateLinkToFile(lpszfilename.into().abi(), riid, renderopt, lpformatetc, pclientsite.into().abi(), pstg.into().abi(), ppvobj).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn OleCreateLinkToFileEx<P0, P1, P2, P3>(lpszfilename: P0, riid: *const ::windows::core::GUID, dwflags: OLECREATE, renderopt: OLERENDER, cformats: u32, rgadvf: *const u32, rgformatetc: *const super::Com::FORMATETC, lpadvisesink: P1, rgdwconnection: *mut u32, pclientsite: P2, pstg: P3, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<super::Com::IAdviseSink>>,
    P2: ::std::convert::Into<::windows::core::InParam<IOleClientSite>>,
    P3: ::std::convert::Into<::windows::core::InParam<super::Com::StructuredStorage::IStorage>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn OleCreateLinkToFileEx ( lpszfilename : :: windows::core::PCWSTR , riid : *const :: windows::core::GUID , dwflags : OLECREATE , renderopt : OLERENDER , cformats : u32 , rgadvf : *const u32 , rgformatetc : *const super::Com:: FORMATETC , lpadvisesink : * mut::core::ffi::c_void , rgdwconnection : *mut u32 , pclientsite : * mut::core::ffi::c_void , pstg : * mut::core::ffi::c_void , ppvobj : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    OleCreateLinkToFileEx(lpszfilename.into().abi(), riid, dwflags, renderopt, cformats, rgadvf, rgformatetc, lpadvisesink.into().abi(), rgdwconnection, pclientsite.into().abi(), pstg.into().abi(), ppvobj).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn OleCreateMenuDescriptor<P0>(hmenucombined: P0, lpmenuwidths: *const OLEMENUGROUPWIDTHS) -> isize
where
    P0: ::std::convert::Into<super::super::UI::WindowsAndMessaging::HMENU>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn OleCreateMenuDescriptor ( hmenucombined : super::super::UI::WindowsAndMessaging:: HMENU , lpmenuwidths : *const OLEMENUGROUPWIDTHS ) -> isize );
    OleCreateMenuDescriptor(hmenucombined.into(), lpmenuwidths)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn OleCreatePictureIndirect<P0, T>(lppictdesc: *const PICTDESC, fown: P0) -> ::windows::core::Result<T>
where
    P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    T: ::windows::core::Interface,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn OleCreatePictureIndirect ( lppictdesc : *const PICTDESC , riid : *const :: windows::core::GUID , fown : super::super::Foundation:: BOOL , lplpvobj : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    OleCreatePictureIndirect(lppictdesc, &<T as ::windows::core::Interface>::IID, fown.into(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OleCreatePropertyFrame<P0, P1>(hwndowner: P0, x: u32, y: u32, lpszcaption: P1, cobjects: u32, ppunk: *const ::core::option::Option<::windows::core::IUnknown>, cpages: u32, ppageclsid: *const ::windows::core::GUID, lcid: u32, dwreserved: u32, pvreserved: *const ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn OleCreatePropertyFrame ( hwndowner : super::super::Foundation:: HWND , x : u32 , y : u32 , lpszcaption : :: windows::core::PCWSTR , cobjects : u32 , ppunk : *const * mut::core::ffi::c_void , cpages : u32 , ppageclsid : *const :: windows::core::GUID , lcid : u32 , dwreserved : u32 , pvreserved : *const ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    OleCreatePropertyFrame(hwndowner.into(), x, y, lpszcaption.into().abi(), cobjects, ::core::mem::transmute(ppunk), cpages, ppageclsid, lcid, dwreserved, pvreserved).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OleCreatePropertyFrameIndirect(lpparams: *const OCPFIPARAMS) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn OleCreatePropertyFrameIndirect ( lpparams : *const OCPFIPARAMS ) -> :: windows::core::HRESULT );
    OleCreatePropertyFrameIndirect(lpparams).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn OleCreateStaticFromData<P0, P1, P2>(psrcdataobj: P0, iid: *const ::windows::core::GUID, renderopt: OLERENDER, pformatetc: *const super::Com::FORMATETC, pclientsite: P1, pstg: P2, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDataObject>>,
    P1: ::std::convert::Into<::windows::core::InParam<IOleClientSite>>,
    P2: ::std::convert::Into<::windows::core::InParam<super::Com::StructuredStorage::IStorage>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn OleCreateStaticFromData ( psrcdataobj : * mut::core::ffi::c_void , iid : *const :: windows::core::GUID , renderopt : OLERENDER , pformatetc : *const super::Com:: FORMATETC , pclientsite : * mut::core::ffi::c_void , pstg : * mut::core::ffi::c_void , ppvobj : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    OleCreateStaticFromData(psrcdataobj.into().abi(), iid, renderopt, pformatetc, pclientsite.into().abi(), pstg.into().abi(), ppvobj).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn OleDestroyMenuDescriptor(holemenu: isize) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "ole32.dll""system" fn OleDestroyMenuDescriptor ( holemenu : isize ) -> :: windows::core::HRESULT );
    OleDestroyMenuDescriptor(holemenu).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn OleDoAutoConvert<P0>(pstg: P0) -> ::windows::core::Result<::windows::core::GUID>
where
    P0: ::std::convert::Into<::windows::core::InParam<super::Com::StructuredStorage::IStorage>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn OleDoAutoConvert ( pstg : * mut::core::ffi::c_void , pclsidnew : *mut :: windows::core::GUID ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    OleDoAutoConvert(pstg.into().abi(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn OleDraw<P0, P1>(punknown: P0, dwaspect: u32, hdcdraw: P1, lprcbounds: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    P1: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn OleDraw ( punknown : * mut::core::ffi::c_void , dwaspect : u32 , hdcdraw : super::super::Graphics::Gdi:: HDC , lprcbounds : *const super::super::Foundation:: RECT ) -> :: windows::core::HRESULT );
    OleDraw(punknown.into().abi(), dwaspect, hdcdraw.into(), lprcbounds).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Memory\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
#[inline]
pub unsafe fn OleDuplicateData<P0>(hsrc: P0, cfformat: CLIPBOARD_FORMAT, uiflags: super::Memory::GLOBAL_ALLOC_FLAGS) -> super::super::Foundation::HANDLE
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn OleDuplicateData ( hsrc : super::super::Foundation:: HANDLE , cfformat : CLIPBOARD_FORMAT , uiflags : super::Memory:: GLOBAL_ALLOC_FLAGS ) -> super::super::Foundation:: HANDLE );
    OleDuplicateData(hsrc.into(), cfformat, uiflags)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn OleFlushClipboard() -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "ole32.dll""system" fn OleFlushClipboard ( ) -> :: windows::core::HRESULT );
    OleFlushClipboard().ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn OleGetAutoConvert(clsidold: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::GUID> {
    ::windows::core::link ! ( "ole32.dll""system" fn OleGetAutoConvert ( clsidold : *const :: windows::core::GUID , pclsidnew : *mut :: windows::core::GUID ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    OleGetAutoConvert(clsidold, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn OleGetClipboard() -> ::windows::core::Result<super::Com::IDataObject> {
    ::windows::core::link ! ( "ole32.dll""system" fn OleGetClipboard ( ppdataobj : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    OleGetClipboard(result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn OleGetClipboardWithEnterpriseInfo(dataobject: *mut ::core::option::Option<super::Com::IDataObject>, dataenterpriseid: *mut ::windows::core::PWSTR, sourcedescription: *mut ::windows::core::PWSTR, targetdescription: *mut ::windows::core::PWSTR, datadescription: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "ole32.dll""system" fn OleGetClipboardWithEnterpriseInfo ( dataobject : *mut * mut::core::ffi::c_void , dataenterpriseid : *mut :: windows::core::PWSTR , sourcedescription : *mut :: windows::core::PWSTR , targetdescription : *mut :: windows::core::PWSTR , datadescription : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    OleGetClipboardWithEnterpriseInfo(::core::mem::transmute(dataobject), dataenterpriseid, sourcedescription, targetdescription, datadescription).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OleGetIconOfClass<P0, P1>(rclsid: *const ::windows::core::GUID, lpszlabel: P0, fusetypeaslabel: P1) -> isize
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn OleGetIconOfClass ( rclsid : *const :: windows::core::GUID , lpszlabel : :: windows::core::PCWSTR , fusetypeaslabel : super::super::Foundation:: BOOL ) -> isize );
    OleGetIconOfClass(rclsid, lpszlabel.into().abi(), fusetypeaslabel.into())
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OleGetIconOfFile<P0, P1>(lpszpath: P0, fusefileaslabel: P1) -> isize
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn OleGetIconOfFile ( lpszpath : :: windows::core::PCWSTR , fusefileaslabel : super::super::Foundation:: BOOL ) -> isize );
    OleGetIconOfFile(lpszpath.into().abi(), fusefileaslabel.into())
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn OleIconToCursor<P0, P1>(hinstexe: P0, hicon: P1) -> super::super::UI::WindowsAndMessaging::HCURSOR
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    P1: ::std::convert::Into<super::super::UI::WindowsAndMessaging::HICON>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn OleIconToCursor ( hinstexe : super::super::Foundation:: HINSTANCE , hicon : super::super::UI::WindowsAndMessaging:: HICON ) -> super::super::UI::WindowsAndMessaging:: HCURSOR );
    OleIconToCursor(hinstexe.into(), hicon.into())
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn OleInitialize(pvreserved: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "ole32.dll""system" fn OleInitialize ( pvreserved : *const ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    OleInitialize(pvreserved).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn OleIsCurrentClipboard<P0>(pdataobj: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDataObject>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn OleIsCurrentClipboard ( pdataobj : * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    OleIsCurrentClipboard(pdataobj.into().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OleIsRunning<P0>(pobject: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<IOleObject>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn OleIsRunning ( pobject : * mut::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    OleIsRunning(pobject.into().abi())
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn OleLoad<P0, P1>(pstg: P0, riid: *const ::windows::core::GUID, pclientsite: P1, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<super::Com::StructuredStorage::IStorage>>,
    P1: ::std::convert::Into<::windows::core::InParam<IOleClientSite>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn OleLoad ( pstg : * mut::core::ffi::c_void , riid : *const :: windows::core::GUID , pclientsite : * mut::core::ffi::c_void , ppvobj : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    OleLoad(pstg.into().abi(), riid, pclientsite.into().abi(), ppvobj).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn OleLoadFromStream<P0>(pstm: P0, iidinterface: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<super::Com::IStream>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn OleLoadFromStream ( pstm : * mut::core::ffi::c_void , iidinterface : *const :: windows::core::GUID , ppvobj : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    OleLoadFromStream(pstm.into().abi(), iidinterface, ppvobj).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn OleLoadPicture<P0, P1>(lpstream: P0, lsize: i32, frunmode: P1, riid: *const ::windows::core::GUID, lplpvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<super::Com::IStream>>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn OleLoadPicture ( lpstream : * mut::core::ffi::c_void , lsize : i32 , frunmode : super::super::Foundation:: BOOL , riid : *const :: windows::core::GUID , lplpvobj : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    OleLoadPicture(lpstream.into().abi(), lsize, frunmode.into(), riid, lplpvobj).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn OleLoadPictureEx<P0, P1>(lpstream: P0, lsize: i32, frunmode: P1, riid: *const ::windows::core::GUID, xsizedesired: u32, ysizedesired: u32, dwflags: LOAD_PICTURE_FLAGS, lplpvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<super::Com::IStream>>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn OleLoadPictureEx ( lpstream : * mut::core::ffi::c_void , lsize : i32 , frunmode : super::super::Foundation:: BOOL , riid : *const :: windows::core::GUID , xsizedesired : u32 , ysizedesired : u32 , dwflags : LOAD_PICTURE_FLAGS , lplpvobj : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    OleLoadPictureEx(lpstream.into().abi(), lsize, frunmode.into(), riid, xsizedesired, ysizedesired, dwflags, lplpvobj).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn OleLoadPictureFile(varfilename: super::Com::VARIANT) -> ::windows::core::Result<super::Com::IDispatch> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn OleLoadPictureFile ( varfilename : super::Com:: VARIANT , lplpdisppicture : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    OleLoadPictureFile(::core::mem::transmute(varfilename), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn OleLoadPictureFileEx(varfilename: super::Com::VARIANT, xsizedesired: u32, ysizedesired: u32, dwflags: LOAD_PICTURE_FLAGS) -> ::windows::core::Result<super::Com::IDispatch> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn OleLoadPictureFileEx ( varfilename : super::Com:: VARIANT , xsizedesired : u32 , ysizedesired : u32 , dwflags : LOAD_PICTURE_FLAGS , lplpdisppicture : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    OleLoadPictureFileEx(::core::mem::transmute(varfilename), xsizedesired, ysizedesired, dwflags, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn OleLoadPicturePath<P0, P1>(szurlorpath: P0, punkcaller: P1, dwreserved: u32, clrreserved: u32, riid: *const ::windows::core::GUID, ppvret: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn OleLoadPicturePath ( szurlorpath : :: windows::core::PCWSTR , punkcaller : * mut::core::ffi::c_void , dwreserved : u32 , clrreserved : u32 , riid : *const :: windows::core::GUID , ppvret : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    OleLoadPicturePath(szurlorpath.into().abi(), punkcaller.into().abi(), dwreserved, clrreserved, riid, ppvret).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OleLockRunning<P0, P1, P2>(punknown: P0, flock: P1, flastunlockcloses: P2) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    P2: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn OleLockRunning ( punknown : * mut::core::ffi::c_void , flock : super::super::Foundation:: BOOL , flastunlockcloses : super::super::Foundation:: BOOL ) -> :: windows::core::HRESULT );
    OleLockRunning(punknown.into().abi(), flock.into(), flastunlockcloses.into()).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn OleMetafilePictFromIconAndLabel<P0, P1, P2>(hicon: P0, lpszlabel: P1, lpszsourcefile: P2, iiconindex: u32) -> isize
where
    P0: ::std::convert::Into<super::super::UI::WindowsAndMessaging::HICON>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn OleMetafilePictFromIconAndLabel ( hicon : super::super::UI::WindowsAndMessaging:: HICON , lpszlabel : :: windows::core::PCWSTR , lpszsourcefile : :: windows::core::PCWSTR , iiconindex : u32 ) -> isize );
    OleMetafilePictFromIconAndLabel(hicon.into(), lpszlabel.into().abi(), lpszsourcefile.into().abi(), iiconindex)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OleNoteObjectVisible<P0, P1>(punknown: P0, fvisible: P1) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn OleNoteObjectVisible ( punknown : * mut::core::ffi::c_void , fvisible : super::super::Foundation:: BOOL ) -> :: windows::core::HRESULT );
    OleNoteObjectVisible(punknown.into().abi(), fvisible.into()).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn OleQueryCreateFromData<P0>(psrcdataobject: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDataObject>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn OleQueryCreateFromData ( psrcdataobject : * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    OleQueryCreateFromData(psrcdataobject.into().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn OleQueryLinkFromData<P0>(psrcdataobject: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDataObject>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn OleQueryLinkFromData ( psrcdataobject : * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    OleQueryLinkFromData(psrcdataobject.into().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn OleRegEnumFormatEtc(clsid: *const ::windows::core::GUID, dwdirection: u32) -> ::windows::core::Result<super::Com::IEnumFORMATETC> {
    ::windows::core::link ! ( "ole32.dll""system" fn OleRegEnumFormatEtc ( clsid : *const :: windows::core::GUID , dwdirection : u32 , ppenum : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    OleRegEnumFormatEtc(clsid, dwdirection, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn OleRegEnumVerbs(clsid: *const ::windows::core::GUID) -> ::windows::core::Result<IEnumOLEVERB> {
    ::windows::core::link ! ( "ole32.dll""system" fn OleRegEnumVerbs ( clsid : *const :: windows::core::GUID , ppenum : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    OleRegEnumVerbs(clsid, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn OleRegGetMiscStatus(clsid: *const ::windows::core::GUID, dwaspect: u32) -> ::windows::core::Result<u32> {
    ::windows::core::link ! ( "ole32.dll""system" fn OleRegGetMiscStatus ( clsid : *const :: windows::core::GUID , dwaspect : u32 , pdwstatus : *mut u32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    OleRegGetMiscStatus(clsid, dwaspect, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn OleRegGetUserType(clsid: *const ::windows::core::GUID, dwformoftype: USERCLASSTYPE) -> ::windows::core::Result<::windows::core::PWSTR> {
    ::windows::core::link ! ( "ole32.dll""system" fn OleRegGetUserType ( clsid : *const :: windows::core::GUID , dwformoftype : USERCLASSTYPE , pszusertype : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    OleRegGetUserType(clsid, dwformoftype, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn OleRun<P0>(punknown: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn OleRun ( punknown : * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    OleRun(punknown.into().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn OleSave<P0, P1, P2>(pps: P0, pstg: P1, fsameasload: P2) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<super::Com::StructuredStorage::IPersistStorage>>,
    P1: ::std::convert::Into<::windows::core::InParam<super::Com::StructuredStorage::IStorage>>,
    P2: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn OleSave ( pps : * mut::core::ffi::c_void , pstg : * mut::core::ffi::c_void , fsameasload : super::super::Foundation:: BOOL ) -> :: windows::core::HRESULT );
    OleSave(pps.into().abi(), pstg.into().abi(), fsameasload.into()).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn OleSavePictureFile<P0>(lpdisppicture: P0, bstrfilename: &::windows::core::BSTR) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn OleSavePictureFile ( lpdisppicture : * mut::core::ffi::c_void , bstrfilename : * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    OleSavePictureFile(lpdisppicture.into().abi(), ::core::mem::transmute_copy(bstrfilename)).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn OleSaveToStream<P0, P1>(ppstm: P0, pstm: P1) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<super::Com::IPersistStream>>,
    P1: ::std::convert::Into<::windows::core::InParam<super::Com::IStream>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn OleSaveToStream ( ppstm : * mut::core::ffi::c_void , pstm : * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    OleSaveToStream(ppstm.into().abi(), pstm.into().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn OleSetAutoConvert(clsidold: *const ::windows::core::GUID, clsidnew: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "ole32.dll""system" fn OleSetAutoConvert ( clsidold : *const :: windows::core::GUID , clsidnew : *const :: windows::core::GUID ) -> :: windows::core::HRESULT );
    OleSetAutoConvert(clsidold, clsidnew).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn OleSetClipboard<P0>(pdataobj: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDataObject>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn OleSetClipboard ( pdataobj : * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    OleSetClipboard(pdataobj.into().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OleSetContainedObject<P0, P1>(punknown: P0, fcontained: P1) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn OleSetContainedObject ( punknown : * mut::core::ffi::c_void , fcontained : super::super::Foundation:: BOOL ) -> :: windows::core::HRESULT );
    OleSetContainedObject(punknown.into().abi(), fcontained.into()).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OleSetMenuDescriptor<P0, P1, P2, P3>(holemenu: isize, hwndframe: P0, hwndactiveobject: P1, lpframe: P2, lpactiveobj: P3) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Foundation::HWND>,
    P2: ::std::convert::Into<::windows::core::InParam<IOleInPlaceFrame>>,
    P3: ::std::convert::Into<::windows::core::InParam<IOleInPlaceActiveObject>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn OleSetMenuDescriptor ( holemenu : isize , hwndframe : super::super::Foundation:: HWND , hwndactiveobject : super::super::Foundation:: HWND , lpframe : * mut::core::ffi::c_void , lpactiveobj : * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    OleSetMenuDescriptor(holemenu, hwndframe.into(), hwndactiveobject.into(), lpframe.into().abi(), lpactiveobj.into().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn OleTranslateAccelerator<P0>(lpframe: P0, lpframeinfo: *const OLEINPLACEFRAMEINFO, lpmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<IOleInPlaceFrame>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn OleTranslateAccelerator ( lpframe : * mut::core::ffi::c_void , lpframeinfo : *const OLEINPLACEFRAMEINFO , lpmsg : *const super::super::UI::WindowsAndMessaging:: MSG ) -> :: windows::core::HRESULT );
    OleTranslateAccelerator(lpframe.into().abi(), lpframeinfo, lpmsg).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn OleTranslateColor<P0>(clr: u32, hpal: P0) -> ::windows::core::Result<super::super::Foundation::COLORREF>
where
    P0: ::std::convert::Into<super::super::Graphics::Gdi::HPALETTE>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn OleTranslateColor ( clr : u32 , hpal : super::super::Graphics::Gdi:: HPALETTE , lpcolorref : *mut super::super::Foundation:: COLORREF ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    OleTranslateColor(clr, hpal.into(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn OleUIAddVerbMenuA<P0, P1, P2, P3>(lpoleobj: P0, lpszshorttype: P1, hmenu: P2, upos: u32, uidverbmin: u32, uidverbmax: u32, baddconvert: P3, idconvert: u32, lphmenu: *mut super::super::UI::WindowsAndMessaging::HMENU) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<IOleObject>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P2: ::std::convert::Into<super::super::UI::WindowsAndMessaging::HMENU>,
    P3: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "oledlg.dll""system" fn OleUIAddVerbMenuA ( lpoleobj : * mut::core::ffi::c_void , lpszshorttype : :: windows::core::PCSTR , hmenu : super::super::UI::WindowsAndMessaging:: HMENU , upos : u32 , uidverbmin : u32 , uidverbmax : u32 , baddconvert : super::super::Foundation:: BOOL , idconvert : u32 , lphmenu : *mut super::super::UI::WindowsAndMessaging:: HMENU ) -> super::super::Foundation:: BOOL );
    OleUIAddVerbMenuA(lpoleobj.into().abi(), lpszshorttype.into().abi(), hmenu.into(), upos, uidverbmin, uidverbmax, baddconvert.into(), idconvert, lphmenu)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn OleUIAddVerbMenuW<P0, P1, P2, P3>(lpoleobj: P0, lpszshorttype: P1, hmenu: P2, upos: u32, uidverbmin: u32, uidverbmax: u32, baddconvert: P3, idconvert: u32, lphmenu: *mut super::super::UI::WindowsAndMessaging::HMENU) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<IOleObject>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<super::super::UI::WindowsAndMessaging::HMENU>,
    P3: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "oledlg.dll""system" fn OleUIAddVerbMenuW ( lpoleobj : * mut::core::ffi::c_void , lpszshorttype : :: windows::core::PCWSTR , hmenu : super::super::UI::WindowsAndMessaging:: HMENU , upos : u32 , uidverbmin : u32 , uidverbmax : u32 , baddconvert : super::super::Foundation:: BOOL , idconvert : u32 , lphmenu : *mut super::super::UI::WindowsAndMessaging:: HMENU ) -> super::super::Foundation:: BOOL );
    OleUIAddVerbMenuW(lpoleobj.into().abi(), lpszshorttype.into().abi(), hmenu.into(), upos, uidverbmin, uidverbmax, baddconvert.into(), idconvert, lphmenu)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_Media\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media"))]
#[inline]
pub unsafe fn OleUIBusyA(param0: *const OLEUIBUSYA) -> u32 {
    ::windows::core::link ! ( "oledlg.dll""system" fn OleUIBusyA ( param0 : *const OLEUIBUSYA ) -> u32 );
    OleUIBusyA(param0)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_Media\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media"))]
#[inline]
pub unsafe fn OleUIBusyW(param0: *const OLEUIBUSYW) -> u32 {
    ::windows::core::link ! ( "oledlg.dll""system" fn OleUIBusyW ( param0 : *const OLEUIBUSYW ) -> u32 );
    OleUIBusyW(param0)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OleUICanConvertOrActivateAs<P0>(rclsid: *const ::windows::core::GUID, fislinkedobject: P0, wformat: u16) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "oledlg.dll""system" fn OleUICanConvertOrActivateAs ( rclsid : *const :: windows::core::GUID , fislinkedobject : super::super::Foundation:: BOOL , wformat : u16 ) -> super::super::Foundation:: BOOL );
    OleUICanConvertOrActivateAs(rclsid, fislinkedobject.into(), wformat)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OleUIChangeIconA(param0: *const OLEUICHANGEICONA) -> u32 {
    ::windows::core::link ! ( "oledlg.dll""system" fn OleUIChangeIconA ( param0 : *const OLEUICHANGEICONA ) -> u32 );
    OleUIChangeIconA(param0)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OleUIChangeIconW(param0: *const OLEUICHANGEICONW) -> u32 {
    ::windows::core::link ! ( "oledlg.dll""system" fn OleUIChangeIconW ( param0 : *const OLEUICHANGEICONW ) -> u32 );
    OleUIChangeIconW(param0)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Controls_Dialogs\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls_Dialogs"))]
#[inline]
pub unsafe fn OleUIChangeSourceA(param0: *const OLEUICHANGESOURCEA) -> u32 {
    ::windows::core::link ! ( "oledlg.dll""system" fn OleUIChangeSourceA ( param0 : *const OLEUICHANGESOURCEA ) -> u32 );
    OleUIChangeSourceA(param0)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Controls_Dialogs\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls_Dialogs"))]
#[inline]
pub unsafe fn OleUIChangeSourceW(param0: *const OLEUICHANGESOURCEW) -> u32 {
    ::windows::core::link ! ( "oledlg.dll""system" fn OleUIChangeSourceW ( param0 : *const OLEUICHANGESOURCEW ) -> u32 );
    OleUIChangeSourceW(param0)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OleUIConvertA(param0: *const OLEUICONVERTA) -> u32 {
    ::windows::core::link ! ( "oledlg.dll""system" fn OleUIConvertA ( param0 : *const OLEUICONVERTA ) -> u32 );
    OleUIConvertA(param0)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OleUIConvertW(param0: *const OLEUICONVERTW) -> u32 {
    ::windows::core::link ! ( "oledlg.dll""system" fn OleUIConvertW ( param0 : *const OLEUICONVERTW ) -> u32 );
    OleUIConvertW(param0)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OleUIEditLinksA(param0: *const OLEUIEDITLINKSA) -> u32 {
    ::windows::core::link ! ( "oledlg.dll""system" fn OleUIEditLinksA ( param0 : *const OLEUIEDITLINKSA ) -> u32 );
    OleUIEditLinksA(param0)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OleUIEditLinksW(param0: *const OLEUIEDITLINKSW) -> u32 {
    ::windows::core::link ! ( "oledlg.dll""system" fn OleUIEditLinksW ( param0 : *const OLEUIEDITLINKSW ) -> u32 );
    OleUIEditLinksW(param0)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn OleUIInsertObjectA(param0: *const OLEUIINSERTOBJECTA) -> u32 {
    ::windows::core::link ! ( "oledlg.dll""system" fn OleUIInsertObjectA ( param0 : *const OLEUIINSERTOBJECTA ) -> u32 );
    OleUIInsertObjectA(param0)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn OleUIInsertObjectW(param0: *const OLEUIINSERTOBJECTW) -> u32 {
    ::windows::core::link ! ( "oledlg.dll""system" fn OleUIInsertObjectW ( param0 : *const OLEUIINSERTOBJECTW ) -> u32 );
    OleUIInsertObjectW(param0)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_Controls\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn OleUIObjectPropertiesA(param0: *const OLEUIOBJECTPROPSA) -> u32 {
    ::windows::core::link ! ( "oledlg.dll""system" fn OleUIObjectPropertiesA ( param0 : *const OLEUIOBJECTPROPSA ) -> u32 );
    OleUIObjectPropertiesA(param0)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_Controls\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn OleUIObjectPropertiesW(param0: *const OLEUIOBJECTPROPSW) -> u32 {
    ::windows::core::link ! ( "oledlg.dll""system" fn OleUIObjectPropertiesW ( param0 : *const OLEUIOBJECTPROPSW ) -> u32 );
    OleUIObjectPropertiesW(param0)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn OleUIPasteSpecialA(param0: *const OLEUIPASTESPECIALA) -> u32 {
    ::windows::core::link ! ( "oledlg.dll""system" fn OleUIPasteSpecialA ( param0 : *const OLEUIPASTESPECIALA ) -> u32 );
    OleUIPasteSpecialA(param0)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn OleUIPasteSpecialW(param0: *const OLEUIPASTESPECIALW) -> u32 {
    ::windows::core::link ! ( "oledlg.dll""system" fn OleUIPasteSpecialW ( param0 : *const OLEUIPASTESPECIALW ) -> u32 );
    OleUIPasteSpecialW(param0)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OleUIPromptUserA<P0>(ntemplate: i32, hwndparent: P0) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "oledlg.dll""cdecl" fn OleUIPromptUserA ( ntemplate : i32 , hwndparent : super::super::Foundation:: HWND ) -> i32 );
    OleUIPromptUserA(ntemplate, hwndparent.into())
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OleUIPromptUserW<P0>(ntemplate: i32, hwndparent: P0) -> i32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "oledlg.dll""cdecl" fn OleUIPromptUserW ( ntemplate : i32 , hwndparent : super::super::Foundation:: HWND ) -> i32 );
    OleUIPromptUserW(ntemplate, hwndparent.into())
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OleUIUpdateLinksA<P0, P1, P2>(lpoleuilinkcntr: P0, hwndparent: P1, lpsztitle: P2, clinks: i32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<IOleUILinkContainerA>>,
    P1: ::std::convert::Into<super::super::Foundation::HWND>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "oledlg.dll""system" fn OleUIUpdateLinksA ( lpoleuilinkcntr : * mut::core::ffi::c_void , hwndparent : super::super::Foundation:: HWND , lpsztitle : :: windows::core::PCSTR , clinks : i32 ) -> super::super::Foundation:: BOOL );
    OleUIUpdateLinksA(lpoleuilinkcntr.into().abi(), hwndparent.into(), lpsztitle.into().abi(), clinks)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OleUIUpdateLinksW<P0, P1, P2>(lpoleuilinkcntr: P0, hwndparent: P1, lpsztitle: P2, clinks: i32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<IOleUILinkContainerW>>,
    P1: ::std::convert::Into<super::super::Foundation::HWND>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "oledlg.dll""system" fn OleUIUpdateLinksW ( lpoleuilinkcntr : * mut::core::ffi::c_void , hwndparent : super::super::Foundation:: HWND , lpsztitle : :: windows::core::PCWSTR , clinks : i32 ) -> super::super::Foundation:: BOOL );
    OleUIUpdateLinksW(lpoleuilinkcntr.into().abi(), hwndparent.into(), lpsztitle.into().abi(), clinks)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn OleUninitialize() {
    ::windows::core::link ! ( "ole32.dll""system" fn OleUninitialize ( ) -> ( ) );
    OleUninitialize()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn QueryPathOfRegTypeLib(guid: *const ::windows::core::GUID, wmaj: u16, wmin: u16, lcid: u32) -> ::windows::core::Result<*mut u16> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn QueryPathOfRegTypeLib ( guid : *const :: windows::core::GUID , wmaj : u16 , wmin : u16 , lcid : u32 , lpbstrpathname : *mut *mut u16 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    QueryPathOfRegTypeLib(guid, wmaj, wmin, lcid, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn RegisterActiveObject<P0>(punk: P0, rclsid: *const ::windows::core::GUID, dwflags: ACTIVEOBJECT_FLAGS, pdwregister: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn RegisterActiveObject ( punk : * mut::core::ffi::c_void , rclsid : *const :: windows::core::GUID , dwflags : ACTIVEOBJECT_FLAGS , pdwregister : *mut u32 ) -> :: windows::core::HRESULT );
    RegisterActiveObject(punk.into().abi(), rclsid, dwflags, pdwregister).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterDragDrop<P0, P1>(hwnd: P0, pdroptarget: P1) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<::windows::core::InParam<IDropTarget>>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn RegisterDragDrop ( hwnd : super::super::Foundation:: HWND , pdroptarget : * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    RegisterDragDrop(hwnd.into(), pdroptarget.into().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn RegisterTypeLib<P0, P1, P2>(ptlib: P0, szfullpath: P1, szhelpdir: P2) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<super::Com::ITypeLib>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn RegisterTypeLib ( ptlib : * mut::core::ffi::c_void , szfullpath : :: windows::core::PCWSTR , szhelpdir : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    RegisterTypeLib(ptlib.into().abi(), szfullpath.into().abi(), szhelpdir.into().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn RegisterTypeLibForUser<P0, P1, P2>(ptlib: P0, szfullpath: P1, szhelpdir: P2) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<super::Com::ITypeLib>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn RegisterTypeLibForUser ( ptlib : * mut::core::ffi::c_void , szfullpath : :: windows::core::PCWSTR , szhelpdir : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    RegisterTypeLibForUser(ptlib.into().abi(), szfullpath.into().abi(), szhelpdir.into().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn ReleaseStgMedium(param0: *mut super::Com::STGMEDIUM) {
    ::windows::core::link ! ( "ole32.dll""system" fn ReleaseStgMedium ( param0 : *mut super::Com:: STGMEDIUM ) -> ( ) );
    ReleaseStgMedium(param0)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn RevokeActiveObject(dwregister: u32, pvreserved: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn RevokeActiveObject ( dwregister : u32 , pvreserved : *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    RevokeActiveObject(dwregister, pvreserved).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RevokeDragDrop<P0>(hwnd: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    ::windows::core::link ! ( "ole32.dll""system" fn RevokeDragDrop ( hwnd : super::super::Foundation:: HWND ) -> :: windows::core::HRESULT );
    RevokeDragDrop(hwnd.into()).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SafeArrayAccessData(psa: *const super::Com::SAFEARRAY, ppvdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn SafeArrayAccessData ( psa : *const super::Com:: SAFEARRAY , ppvdata : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    SafeArrayAccessData(psa, ppvdata).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SafeArrayAddRef(psa: *const super::Com::SAFEARRAY, ppdatatorelease: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn SafeArrayAddRef ( psa : *const super::Com:: SAFEARRAY , ppdatatorelease : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    SafeArrayAddRef(psa, ppdatatorelease).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SafeArrayAllocData(psa: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn SafeArrayAllocData ( psa : *const super::Com:: SAFEARRAY ) -> :: windows::core::HRESULT );
    SafeArrayAllocData(psa).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SafeArrayAllocDescriptor(cdims: u32) -> ::windows::core::Result<*mut super::Com::SAFEARRAY> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn SafeArrayAllocDescriptor ( cdims : u32 , ppsaout : *mut *mut super::Com:: SAFEARRAY ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    SafeArrayAllocDescriptor(cdims, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SafeArrayAllocDescriptorEx(vt: super::Com::VARENUM, cdims: u32) -> ::windows::core::Result<*mut super::Com::SAFEARRAY> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn SafeArrayAllocDescriptorEx ( vt : super::Com:: VARENUM , cdims : u32 , ppsaout : *mut *mut super::Com:: SAFEARRAY ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    SafeArrayAllocDescriptorEx(vt, cdims, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SafeArrayCopy(psa: *const super::Com::SAFEARRAY) -> ::windows::core::Result<*mut super::Com::SAFEARRAY> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn SafeArrayCopy ( psa : *const super::Com:: SAFEARRAY , ppsaout : *mut *mut super::Com:: SAFEARRAY ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    SafeArrayCopy(psa, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SafeArrayCopyData(psasource: *const super::Com::SAFEARRAY, psatarget: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn SafeArrayCopyData ( psasource : *const super::Com:: SAFEARRAY , psatarget : *const super::Com:: SAFEARRAY ) -> :: windows::core::HRESULT );
    SafeArrayCopyData(psasource, psatarget).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SafeArrayCreate(vt: super::Com::VARENUM, cdims: u32, rgsabound: *const super::Com::SAFEARRAYBOUND) -> *mut super::Com::SAFEARRAY {
    ::windows::core::link ! ( "oleaut32.dll""system" fn SafeArrayCreate ( vt : super::Com:: VARENUM , cdims : u32 , rgsabound : *const super::Com:: SAFEARRAYBOUND ) -> *mut super::Com:: SAFEARRAY );
    SafeArrayCreate(vt, cdims, rgsabound)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SafeArrayCreateEx(vt: super::Com::VARENUM, cdims: u32, rgsabound: *const super::Com::SAFEARRAYBOUND, pvextra: *const ::core::ffi::c_void) -> *mut super::Com::SAFEARRAY {
    ::windows::core::link ! ( "oleaut32.dll""system" fn SafeArrayCreateEx ( vt : super::Com:: VARENUM , cdims : u32 , rgsabound : *const super::Com:: SAFEARRAYBOUND , pvextra : *const ::core::ffi::c_void ) -> *mut super::Com:: SAFEARRAY );
    SafeArrayCreateEx(vt, cdims, rgsabound, pvextra)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SafeArrayCreateVector(vt: super::Com::VARENUM, llbound: i32, celements: u32) -> *mut super::Com::SAFEARRAY {
    ::windows::core::link ! ( "oleaut32.dll""system" fn SafeArrayCreateVector ( vt : super::Com:: VARENUM , llbound : i32 , celements : u32 ) -> *mut super::Com:: SAFEARRAY );
    SafeArrayCreateVector(vt, llbound, celements)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SafeArrayCreateVectorEx(vt: super::Com::VARENUM, llbound: i32, celements: u32, pvextra: *const ::core::ffi::c_void) -> *mut super::Com::SAFEARRAY {
    ::windows::core::link ! ( "oleaut32.dll""system" fn SafeArrayCreateVectorEx ( vt : super::Com:: VARENUM , llbound : i32 , celements : u32 , pvextra : *const ::core::ffi::c_void ) -> *mut super::Com:: SAFEARRAY );
    SafeArrayCreateVectorEx(vt, llbound, celements, pvextra)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SafeArrayDestroy(psa: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn SafeArrayDestroy ( psa : *const super::Com:: SAFEARRAY ) -> :: windows::core::HRESULT );
    SafeArrayDestroy(psa).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SafeArrayDestroyData(psa: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn SafeArrayDestroyData ( psa : *const super::Com:: SAFEARRAY ) -> :: windows::core::HRESULT );
    SafeArrayDestroyData(psa).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SafeArrayDestroyDescriptor(psa: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn SafeArrayDestroyDescriptor ( psa : *const super::Com:: SAFEARRAY ) -> :: windows::core::HRESULT );
    SafeArrayDestroyDescriptor(psa).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SafeArrayGetDim(psa: *const super::Com::SAFEARRAY) -> u32 {
    ::windows::core::link ! ( "oleaut32.dll""system" fn SafeArrayGetDim ( psa : *const super::Com:: SAFEARRAY ) -> u32 );
    SafeArrayGetDim(psa)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SafeArrayGetElement(psa: *const super::Com::SAFEARRAY, rgindices: *const i32, pv: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn SafeArrayGetElement ( psa : *const super::Com:: SAFEARRAY , rgindices : *const i32 , pv : *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    SafeArrayGetElement(psa, rgindices, pv).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SafeArrayGetElemsize(psa: *const super::Com::SAFEARRAY) -> u32 {
    ::windows::core::link ! ( "oleaut32.dll""system" fn SafeArrayGetElemsize ( psa : *const super::Com:: SAFEARRAY ) -> u32 );
    SafeArrayGetElemsize(psa)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SafeArrayGetIID(psa: *const super::Com::SAFEARRAY) -> ::windows::core::Result<::windows::core::GUID> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn SafeArrayGetIID ( psa : *const super::Com:: SAFEARRAY , pguid : *mut :: windows::core::GUID ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    SafeArrayGetIID(psa, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SafeArrayGetLBound(psa: *const super::Com::SAFEARRAY, ndim: u32) -> ::windows::core::Result<i32> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn SafeArrayGetLBound ( psa : *const super::Com:: SAFEARRAY , ndim : u32 , pllbound : *mut i32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    SafeArrayGetLBound(psa, ndim, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SafeArrayGetRecordInfo(psa: *const super::Com::SAFEARRAY) -> ::windows::core::Result<IRecordInfo> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn SafeArrayGetRecordInfo ( psa : *const super::Com:: SAFEARRAY , prinfo : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    SafeArrayGetRecordInfo(psa, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SafeArrayGetUBound(psa: *const super::Com::SAFEARRAY, ndim: u32) -> ::windows::core::Result<i32> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn SafeArrayGetUBound ( psa : *const super::Com:: SAFEARRAY , ndim : u32 , plubound : *mut i32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    SafeArrayGetUBound(psa, ndim, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SafeArrayGetVartype(psa: *const super::Com::SAFEARRAY) -> ::windows::core::Result<super::Com::VARENUM> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn SafeArrayGetVartype ( psa : *const super::Com:: SAFEARRAY , pvt : *mut super::Com:: VARENUM ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    SafeArrayGetVartype(psa, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SafeArrayLock(psa: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn SafeArrayLock ( psa : *const super::Com:: SAFEARRAY ) -> :: windows::core::HRESULT );
    SafeArrayLock(psa).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SafeArrayPtrOfIndex(psa: *const super::Com::SAFEARRAY, rgindices: *const i32, ppvdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn SafeArrayPtrOfIndex ( psa : *const super::Com:: SAFEARRAY , rgindices : *const i32 , ppvdata : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    SafeArrayPtrOfIndex(psa, rgindices, ppvdata).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SafeArrayPutElement(psa: *const super::Com::SAFEARRAY, rgindices: *const i32, pv: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn SafeArrayPutElement ( psa : *const super::Com:: SAFEARRAY , rgindices : *const i32 , pv : *const ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    SafeArrayPutElement(psa, rgindices, pv).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SafeArrayRedim(psa: *mut super::Com::SAFEARRAY, psaboundnew: *const super::Com::SAFEARRAYBOUND) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn SafeArrayRedim ( psa : *mut super::Com:: SAFEARRAY , psaboundnew : *const super::Com:: SAFEARRAYBOUND ) -> :: windows::core::HRESULT );
    SafeArrayRedim(psa, psaboundnew).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn SafeArrayReleaseData(pdata: *const ::core::ffi::c_void) {
    ::windows::core::link ! ( "oleaut32.dll""system" fn SafeArrayReleaseData ( pdata : *const ::core::ffi::c_void ) -> ( ) );
    SafeArrayReleaseData(pdata)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SafeArrayReleaseDescriptor(psa: *const super::Com::SAFEARRAY) {
    ::windows::core::link ! ( "oleaut32.dll""system" fn SafeArrayReleaseDescriptor ( psa : *const super::Com:: SAFEARRAY ) -> ( ) );
    SafeArrayReleaseDescriptor(psa)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SafeArraySetIID(psa: *const super::Com::SAFEARRAY, guid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn SafeArraySetIID ( psa : *const super::Com:: SAFEARRAY , guid : *const :: windows::core::GUID ) -> :: windows::core::HRESULT );
    SafeArraySetIID(psa, guid).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SafeArraySetRecordInfo<P0>(psa: *const super::Com::SAFEARRAY, prinfo: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<IRecordInfo>>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn SafeArraySetRecordInfo ( psa : *const super::Com:: SAFEARRAY , prinfo : * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    SafeArraySetRecordInfo(psa, prinfo.into().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SafeArrayUnaccessData(psa: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn SafeArrayUnaccessData ( psa : *const super::Com:: SAFEARRAY ) -> :: windows::core::HRESULT );
    SafeArrayUnaccessData(psa).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SafeArrayUnlock(psa: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn SafeArrayUnlock ( psa : *const super::Com:: SAFEARRAY ) -> :: windows::core::HRESULT );
    SafeArrayUnlock(psa).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SystemTimeToVariantTime(lpsystemtime: *const super::super::Foundation::SYSTEMTIME, pvtime: *mut f64) -> i32 {
    ::windows::core::link ! ( "oleaut32.dll""system" fn SystemTimeToVariantTime ( lpsystemtime : *const super::super::Foundation:: SYSTEMTIME , pvtime : *mut f64 ) -> i32 );
    SystemTimeToVariantTime(lpsystemtime, pvtime)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn UnRegisterTypeLib(libid: *const ::windows::core::GUID, wvermajor: u16, wverminor: u16, lcid: u32, syskind: super::Com::SYSKIND) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn UnRegisterTypeLib ( libid : *const :: windows::core::GUID , wvermajor : u16 , wverminor : u16 , lcid : u32 , syskind : super::Com:: SYSKIND ) -> :: windows::core::HRESULT );
    UnRegisterTypeLib(libid, wvermajor, wverminor, lcid, syskind).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn UnRegisterTypeLibForUser(libid: *const ::windows::core::GUID, wmajorvernum: u16, wminorvernum: u16, lcid: u32, syskind: super::Com::SYSKIND) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn UnRegisterTypeLibForUser ( libid : *const :: windows::core::GUID , wmajorvernum : u16 , wminorvernum : u16 , lcid : u32 , syskind : super::Com:: SYSKIND ) -> :: windows::core::HRESULT );
    UnRegisterTypeLibForUser(libid, wmajorvernum, wminorvernum, lcid, syskind).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn VarAbs(pvarin: *const super::Com::VARIANT) -> ::windows::core::Result<super::Com::VARIANT> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarAbs ( pvarin : *const super::Com:: VARIANT , pvarresult : *mut super::Com:: VARIANT ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarAbs(pvarin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn VarAdd(pvarleft: *const super::Com::VARIANT, pvarright: *const super::Com::VARIANT) -> ::windows::core::Result<super::Com::VARIANT> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarAdd ( pvarleft : *const super::Com:: VARIANT , pvarright : *const super::Com:: VARIANT , pvarresult : *mut super::Com:: VARIANT ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarAdd(pvarleft, pvarright, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn VarAnd(pvarleft: *const super::Com::VARIANT, pvarright: *const super::Com::VARIANT) -> ::windows::core::Result<super::Com::VARIANT> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarAnd ( pvarleft : *const super::Com:: VARIANT , pvarright : *const super::Com:: VARIANT , pvarresult : *mut super::Com:: VARIANT ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarAnd(pvarleft, pvarright, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn VarBoolFromCy(cyin: super::Com::CY) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarBoolFromCy ( cyin : super::Com:: CY , pboolout : *mut super::super::Foundation:: VARIANT_BOOL ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarBoolFromCy(::core::mem::transmute(cyin), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarBoolFromDate(datein: f64) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarBoolFromDate ( datein : f64 , pboolout : *mut super::super::Foundation:: VARIANT_BOOL ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarBoolFromDate(datein, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarBoolFromDec(pdecin: *const super::super::Foundation::DECIMAL) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarBoolFromDec ( pdecin : *const super::super::Foundation:: DECIMAL , pboolout : *mut super::super::Foundation:: VARIANT_BOOL ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarBoolFromDec(pdecin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn VarBoolFromDisp<P0>(pdispin: P0, lcid: u32) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>
where
    P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarBoolFromDisp ( pdispin : * mut::core::ffi::c_void , lcid : u32 , pboolout : *mut super::super::Foundation:: VARIANT_BOOL ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarBoolFromDisp(pdispin.into().abi(), lcid, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarBoolFromI1<P0>(cin: P0) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>
where
    P0: ::std::convert::Into<super::super::Foundation::CHAR>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarBoolFromI1 ( cin : super::super::Foundation:: CHAR , pboolout : *mut super::super::Foundation:: VARIANT_BOOL ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarBoolFromI1(cin.into(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarBoolFromI2(sin: i16) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarBoolFromI2 ( sin : i16 , pboolout : *mut super::super::Foundation:: VARIANT_BOOL ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarBoolFromI2(sin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarBoolFromI4(lin: i32) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarBoolFromI4 ( lin : i32 , pboolout : *mut super::super::Foundation:: VARIANT_BOOL ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarBoolFromI4(lin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarBoolFromI8(i64in: i64) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarBoolFromI8 ( i64in : i64 , pboolout : *mut super::super::Foundation:: VARIANT_BOOL ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarBoolFromI8(i64in, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarBoolFromR4(fltin: f32) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarBoolFromR4 ( fltin : f32 , pboolout : *mut super::super::Foundation:: VARIANT_BOOL ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarBoolFromR4(fltin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarBoolFromR8(dblin: f64) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarBoolFromR8 ( dblin : f64 , pboolout : *mut super::super::Foundation:: VARIANT_BOOL ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarBoolFromR8(dblin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarBoolFromStr<P0>(strin: P0, lcid: u32, dwflags: u32) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarBoolFromStr ( strin : :: windows::core::PCWSTR , lcid : u32 , dwflags : u32 , pboolout : *mut super::super::Foundation:: VARIANT_BOOL ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarBoolFromStr(strin.into().abi(), lcid, dwflags, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarBoolFromUI1(bin: u8) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarBoolFromUI1 ( bin : u8 , pboolout : *mut super::super::Foundation:: VARIANT_BOOL ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarBoolFromUI1(bin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarBoolFromUI2(uiin: u16) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarBoolFromUI2 ( uiin : u16 , pboolout : *mut super::super::Foundation:: VARIANT_BOOL ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarBoolFromUI2(uiin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarBoolFromUI4(ulin: u32) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarBoolFromUI4 ( ulin : u32 , pboolout : *mut super::super::Foundation:: VARIANT_BOOL ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarBoolFromUI4(ulin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarBoolFromUI8(i64in: u64) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarBoolFromUI8 ( i64in : u64 , pboolout : *mut super::super::Foundation:: VARIANT_BOOL ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarBoolFromUI8(i64in, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarBstrCat(bstrleft: &::windows::core::BSTR, bstrright: &::windows::core::BSTR) -> ::windows::core::Result<*mut u16> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarBstrCat ( bstrleft : * mut::core::ffi::c_void , bstrright : * mut::core::ffi::c_void , pbstrresult : *mut *mut u16 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarBstrCat(::core::mem::transmute_copy(bstrleft), ::core::mem::transmute_copy(bstrright), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarBstrCmp(bstrleft: &::windows::core::BSTR, bstrright: &::windows::core::BSTR, lcid: u32, dwflags: u32) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarBstrCmp ( bstrleft : * mut::core::ffi::c_void , bstrright : * mut::core::ffi::c_void , lcid : u32 , dwflags : u32 ) -> :: windows::core::HRESULT );
    VarBstrCmp(::core::mem::transmute_copy(bstrleft), ::core::mem::transmute_copy(bstrright), lcid, dwflags).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarBstrFromBool<P0>(boolin: P0, lcid: u32, dwflags: u32) -> ::windows::core::Result<::windows::core::BSTR>
where
    P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarBstrFromBool ( boolin : super::super::Foundation:: VARIANT_BOOL , lcid : u32 , dwflags : u32 , pbstrout : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarBstrFromBool(boolin.into(), lcid, dwflags, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarBstrFromCy(cyin: super::Com::CY, lcid: u32, dwflags: u32) -> ::windows::core::Result<::windows::core::BSTR> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarBstrFromCy ( cyin : super::Com:: CY , lcid : u32 , dwflags : u32 , pbstrout : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarBstrFromCy(::core::mem::transmute(cyin), lcid, dwflags, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarBstrFromDate(datein: f64, lcid: u32, dwflags: u32) -> ::windows::core::Result<::windows::core::BSTR> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarBstrFromDate ( datein : f64 , lcid : u32 , dwflags : u32 , pbstrout : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarBstrFromDate(datein, lcid, dwflags, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarBstrFromDec(pdecin: *const super::super::Foundation::DECIMAL, lcid: u32, dwflags: u32) -> ::windows::core::Result<::windows::core::BSTR> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarBstrFromDec ( pdecin : *const super::super::Foundation:: DECIMAL , lcid : u32 , dwflags : u32 , pbstrout : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarBstrFromDec(pdecin, lcid, dwflags, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarBstrFromDisp<P0>(pdispin: P0, lcid: u32, dwflags: u32) -> ::windows::core::Result<::windows::core::BSTR>
where
    P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarBstrFromDisp ( pdispin : * mut::core::ffi::c_void , lcid : u32 , dwflags : u32 , pbstrout : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarBstrFromDisp(pdispin.into().abi(), lcid, dwflags, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarBstrFromI1<P0>(cin: P0, lcid: u32, dwflags: u32) -> ::windows::core::Result<::windows::core::BSTR>
where
    P0: ::std::convert::Into<super::super::Foundation::CHAR>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarBstrFromI1 ( cin : super::super::Foundation:: CHAR , lcid : u32 , dwflags : u32 , pbstrout : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarBstrFromI1(cin.into(), lcid, dwflags, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarBstrFromI2(ival: i16, lcid: u32, dwflags: u32) -> ::windows::core::Result<::windows::core::BSTR> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarBstrFromI2 ( ival : i16 , lcid : u32 , dwflags : u32 , pbstrout : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarBstrFromI2(ival, lcid, dwflags, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarBstrFromI4(lin: i32, lcid: u32, dwflags: u32) -> ::windows::core::Result<::windows::core::BSTR> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarBstrFromI4 ( lin : i32 , lcid : u32 , dwflags : u32 , pbstrout : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarBstrFromI4(lin, lcid, dwflags, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarBstrFromI8(i64in: i64, lcid: u32, dwflags: u32) -> ::windows::core::Result<::windows::core::BSTR> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarBstrFromI8 ( i64in : i64 , lcid : u32 , dwflags : u32 , pbstrout : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarBstrFromI8(i64in, lcid, dwflags, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarBstrFromR4(fltin: f32, lcid: u32, dwflags: u32) -> ::windows::core::Result<::windows::core::BSTR> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarBstrFromR4 ( fltin : f32 , lcid : u32 , dwflags : u32 , pbstrout : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarBstrFromR4(fltin, lcid, dwflags, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarBstrFromR8(dblin: f64, lcid: u32, dwflags: u32) -> ::windows::core::Result<::windows::core::BSTR> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarBstrFromR8 ( dblin : f64 , lcid : u32 , dwflags : u32 , pbstrout : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarBstrFromR8(dblin, lcid, dwflags, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarBstrFromUI1(bval: u8, lcid: u32, dwflags: u32) -> ::windows::core::Result<::windows::core::BSTR> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarBstrFromUI1 ( bval : u8 , lcid : u32 , dwflags : u32 , pbstrout : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarBstrFromUI1(bval, lcid, dwflags, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarBstrFromUI2(uiin: u16, lcid: u32, dwflags: u32) -> ::windows::core::Result<::windows::core::BSTR> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarBstrFromUI2 ( uiin : u16 , lcid : u32 , dwflags : u32 , pbstrout : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarBstrFromUI2(uiin, lcid, dwflags, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarBstrFromUI4(ulin: u32, lcid: u32, dwflags: u32) -> ::windows::core::Result<::windows::core::BSTR> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarBstrFromUI4 ( ulin : u32 , lcid : u32 , dwflags : u32 , pbstrout : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarBstrFromUI4(ulin, lcid, dwflags, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarBstrFromUI8(ui64in: u64, lcid: u32, dwflags: u32) -> ::windows::core::Result<::windows::core::BSTR> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarBstrFromUI8 ( ui64in : u64 , lcid : u32 , dwflags : u32 , pbstrout : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarBstrFromUI8(ui64in, lcid, dwflags, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn VarCat(pvarleft: *const super::Com::VARIANT, pvarright: *const super::Com::VARIANT) -> ::windows::core::Result<super::Com::VARIANT> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarCat ( pvarleft : *const super::Com:: VARIANT , pvarright : *const super::Com:: VARIANT , pvarresult : *mut super::Com:: VARIANT ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarCat(pvarleft, pvarright, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn VarCmp(pvarleft: *const super::Com::VARIANT, pvarright: *const super::Com::VARIANT, lcid: u32, dwflags: u32) -> VARCMP {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarCmp ( pvarleft : *const super::Com:: VARIANT , pvarright : *const super::Com:: VARIANT , lcid : u32 , dwflags : u32 ) -> VARCMP );
    VarCmp(pvarleft, pvarright, lcid, dwflags)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarCyAbs(cyin: super::Com::CY) -> ::windows::core::Result<super::Com::CY> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarCyAbs ( cyin : super::Com:: CY , pcyresult : *mut super::Com:: CY ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarCyAbs(::core::mem::transmute(cyin), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarCyAdd(cyleft: super::Com::CY, cyright: super::Com::CY) -> ::windows::core::Result<super::Com::CY> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarCyAdd ( cyleft : super::Com:: CY , cyright : super::Com:: CY , pcyresult : *mut super::Com:: CY ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarCyAdd(::core::mem::transmute(cyleft), ::core::mem::transmute(cyright), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarCyCmp(cyleft: super::Com::CY, cyright: super::Com::CY) -> VARCMP {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarCyCmp ( cyleft : super::Com:: CY , cyright : super::Com:: CY ) -> VARCMP );
    VarCyCmp(::core::mem::transmute(cyleft), ::core::mem::transmute(cyright))
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarCyCmpR8(cyleft: super::Com::CY, dblright: f64) -> VARCMP {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarCyCmpR8 ( cyleft : super::Com:: CY , dblright : f64 ) -> VARCMP );
    VarCyCmpR8(::core::mem::transmute(cyleft), dblright)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarCyFix(cyin: super::Com::CY) -> ::windows::core::Result<super::Com::CY> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarCyFix ( cyin : super::Com:: CY , pcyresult : *mut super::Com:: CY ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarCyFix(::core::mem::transmute(cyin), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn VarCyFromBool<P0>(boolin: P0) -> ::windows::core::Result<super::Com::CY>
where
    P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarCyFromBool ( boolin : super::super::Foundation:: VARIANT_BOOL , pcyout : *mut super::Com:: CY ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarCyFromBool(boolin.into(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarCyFromDate(datein: f64) -> ::windows::core::Result<super::Com::CY> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarCyFromDate ( datein : f64 , pcyout : *mut super::Com:: CY ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarCyFromDate(datein, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn VarCyFromDec(pdecin: *const super::super::Foundation::DECIMAL) -> ::windows::core::Result<super::Com::CY> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarCyFromDec ( pdecin : *const super::super::Foundation:: DECIMAL , pcyout : *mut super::Com:: CY ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarCyFromDec(pdecin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarCyFromDisp<P0>(pdispin: P0, lcid: u32) -> ::windows::core::Result<super::Com::CY>
where
    P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarCyFromDisp ( pdispin : * mut::core::ffi::c_void , lcid : u32 , pcyout : *mut super::Com:: CY ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarCyFromDisp(pdispin.into().abi(), lcid, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn VarCyFromI1<P0>(cin: P0) -> ::windows::core::Result<super::Com::CY>
where
    P0: ::std::convert::Into<super::super::Foundation::CHAR>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarCyFromI1 ( cin : super::super::Foundation:: CHAR , pcyout : *mut super::Com:: CY ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarCyFromI1(cin.into(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarCyFromI2(sin: i16) -> ::windows::core::Result<super::Com::CY> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarCyFromI2 ( sin : i16 , pcyout : *mut super::Com:: CY ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarCyFromI2(sin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarCyFromI4(lin: i32) -> ::windows::core::Result<super::Com::CY> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarCyFromI4 ( lin : i32 , pcyout : *mut super::Com:: CY ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarCyFromI4(lin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarCyFromI8(i64in: i64) -> ::windows::core::Result<super::Com::CY> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarCyFromI8 ( i64in : i64 , pcyout : *mut super::Com:: CY ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarCyFromI8(i64in, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarCyFromR4(fltin: f32) -> ::windows::core::Result<super::Com::CY> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarCyFromR4 ( fltin : f32 , pcyout : *mut super::Com:: CY ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarCyFromR4(fltin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarCyFromR8(dblin: f64) -> ::windows::core::Result<super::Com::CY> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarCyFromR8 ( dblin : f64 , pcyout : *mut super::Com:: CY ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarCyFromR8(dblin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarCyFromStr<P0>(strin: P0, lcid: u32, dwflags: u32) -> ::windows::core::Result<super::Com::CY>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarCyFromStr ( strin : :: windows::core::PCWSTR , lcid : u32 , dwflags : u32 , pcyout : *mut super::Com:: CY ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarCyFromStr(strin.into().abi(), lcid, dwflags, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarCyFromUI1(bin: u8) -> ::windows::core::Result<super::Com::CY> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarCyFromUI1 ( bin : u8 , pcyout : *mut super::Com:: CY ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarCyFromUI1(bin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarCyFromUI2(uiin: u16) -> ::windows::core::Result<super::Com::CY> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarCyFromUI2 ( uiin : u16 , pcyout : *mut super::Com:: CY ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarCyFromUI2(uiin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarCyFromUI4(ulin: u32) -> ::windows::core::Result<super::Com::CY> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarCyFromUI4 ( ulin : u32 , pcyout : *mut super::Com:: CY ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarCyFromUI4(ulin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarCyFromUI8(ui64in: u64) -> ::windows::core::Result<super::Com::CY> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarCyFromUI8 ( ui64in : u64 , pcyout : *mut super::Com:: CY ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarCyFromUI8(ui64in, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarCyInt(cyin: super::Com::CY) -> ::windows::core::Result<super::Com::CY> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarCyInt ( cyin : super::Com:: CY , pcyresult : *mut super::Com:: CY ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarCyInt(::core::mem::transmute(cyin), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarCyMul(cyleft: super::Com::CY, cyright: super::Com::CY) -> ::windows::core::Result<super::Com::CY> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarCyMul ( cyleft : super::Com:: CY , cyright : super::Com:: CY , pcyresult : *mut super::Com:: CY ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarCyMul(::core::mem::transmute(cyleft), ::core::mem::transmute(cyright), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarCyMulI4(cyleft: super::Com::CY, lright: i32) -> ::windows::core::Result<super::Com::CY> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarCyMulI4 ( cyleft : super::Com:: CY , lright : i32 , pcyresult : *mut super::Com:: CY ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarCyMulI4(::core::mem::transmute(cyleft), lright, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarCyMulI8(cyleft: super::Com::CY, lright: i64) -> ::windows::core::Result<super::Com::CY> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarCyMulI8 ( cyleft : super::Com:: CY , lright : i64 , pcyresult : *mut super::Com:: CY ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarCyMulI8(::core::mem::transmute(cyleft), lright, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarCyNeg(cyin: super::Com::CY) -> ::windows::core::Result<super::Com::CY> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarCyNeg ( cyin : super::Com:: CY , pcyresult : *mut super::Com:: CY ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarCyNeg(::core::mem::transmute(cyin), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarCyRound(cyin: super::Com::CY, cdecimals: i32) -> ::windows::core::Result<super::Com::CY> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarCyRound ( cyin : super::Com:: CY , cdecimals : i32 , pcyresult : *mut super::Com:: CY ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarCyRound(::core::mem::transmute(cyin), cdecimals, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarCySub(cyleft: super::Com::CY, cyright: super::Com::CY) -> ::windows::core::Result<super::Com::CY> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarCySub ( cyleft : super::Com:: CY , cyright : super::Com:: CY , pcyresult : *mut super::Com:: CY ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarCySub(::core::mem::transmute(cyleft), ::core::mem::transmute(cyright), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarDateFromBool<P0>(boolin: P0) -> ::windows::core::Result<f64>
where
    P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarDateFromBool ( boolin : super::super::Foundation:: VARIANT_BOOL , pdateout : *mut f64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarDateFromBool(boolin.into(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarDateFromCy(cyin: super::Com::CY) -> ::windows::core::Result<f64> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarDateFromCy ( cyin : super::Com:: CY , pdateout : *mut f64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarDateFromCy(::core::mem::transmute(cyin), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarDateFromDec(pdecin: *const super::super::Foundation::DECIMAL) -> ::windows::core::Result<f64> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarDateFromDec ( pdecin : *const super::super::Foundation:: DECIMAL , pdateout : *mut f64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarDateFromDec(pdecin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarDateFromDisp<P0>(pdispin: P0, lcid: u32) -> ::windows::core::Result<f64>
where
    P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarDateFromDisp ( pdispin : * mut::core::ffi::c_void , lcid : u32 , pdateout : *mut f64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarDateFromDisp(pdispin.into().abi(), lcid, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarDateFromI1<P0>(cin: P0) -> ::windows::core::Result<f64>
where
    P0: ::std::convert::Into<super::super::Foundation::CHAR>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarDateFromI1 ( cin : super::super::Foundation:: CHAR , pdateout : *mut f64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarDateFromI1(cin.into(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarDateFromI2(sin: i16) -> ::windows::core::Result<f64> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarDateFromI2 ( sin : i16 , pdateout : *mut f64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarDateFromI2(sin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarDateFromI4(lin: i32) -> ::windows::core::Result<f64> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarDateFromI4 ( lin : i32 , pdateout : *mut f64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarDateFromI4(lin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarDateFromI8(i64in: i64) -> ::windows::core::Result<f64> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarDateFromI8 ( i64in : i64 , pdateout : *mut f64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarDateFromI8(i64in, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarDateFromR4(fltin: f32) -> ::windows::core::Result<f64> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarDateFromR4 ( fltin : f32 , pdateout : *mut f64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarDateFromR4(fltin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarDateFromR8(dblin: f64) -> ::windows::core::Result<f64> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarDateFromR8 ( dblin : f64 , pdateout : *mut f64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarDateFromR8(dblin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarDateFromStr<P0>(strin: P0, lcid: u32, dwflags: u32) -> ::windows::core::Result<f64>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarDateFromStr ( strin : :: windows::core::PCWSTR , lcid : u32 , dwflags : u32 , pdateout : *mut f64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarDateFromStr(strin.into().abi(), lcid, dwflags, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarDateFromUI1(bin: u8) -> ::windows::core::Result<f64> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarDateFromUI1 ( bin : u8 , pdateout : *mut f64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarDateFromUI1(bin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarDateFromUI2(uiin: u16) -> ::windows::core::Result<f64> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarDateFromUI2 ( uiin : u16 , pdateout : *mut f64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarDateFromUI2(uiin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarDateFromUI4(ulin: u32) -> ::windows::core::Result<f64> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarDateFromUI4 ( ulin : u32 , pdateout : *mut f64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarDateFromUI4(ulin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarDateFromUI8(ui64in: u64) -> ::windows::core::Result<f64> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarDateFromUI8 ( ui64in : u64 , pdateout : *mut f64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarDateFromUI8(ui64in, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarDateFromUdate(pudatein: *const UDATE, dwflags: u32) -> ::windows::core::Result<f64> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarDateFromUdate ( pudatein : *const UDATE , dwflags : u32 , pdateout : *mut f64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarDateFromUdate(pudatein, dwflags, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarDateFromUdateEx(pudatein: *const UDATE, lcid: u32, dwflags: u32) -> ::windows::core::Result<f64> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarDateFromUdateEx ( pudatein : *const UDATE , lcid : u32 , dwflags : u32 , pdateout : *mut f64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarDateFromUdateEx(pudatein, lcid, dwflags, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarDecAbs(pdecin: *const super::super::Foundation::DECIMAL) -> ::windows::core::Result<super::super::Foundation::DECIMAL> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarDecAbs ( pdecin : *const super::super::Foundation:: DECIMAL , pdecresult : *mut super::super::Foundation:: DECIMAL ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarDecAbs(pdecin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarDecAdd(pdecleft: *const super::super::Foundation::DECIMAL, pdecright: *const super::super::Foundation::DECIMAL) -> ::windows::core::Result<super::super::Foundation::DECIMAL> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarDecAdd ( pdecleft : *const super::super::Foundation:: DECIMAL , pdecright : *const super::super::Foundation:: DECIMAL , pdecresult : *mut super::super::Foundation:: DECIMAL ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarDecAdd(pdecleft, pdecright, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarDecCmp(pdecleft: *const super::super::Foundation::DECIMAL, pdecright: *const super::super::Foundation::DECIMAL) -> VARCMP {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarDecCmp ( pdecleft : *const super::super::Foundation:: DECIMAL , pdecright : *const super::super::Foundation:: DECIMAL ) -> VARCMP );
    VarDecCmp(pdecleft, pdecright)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarDecCmpR8(pdecleft: *const super::super::Foundation::DECIMAL, dblright: f64) -> VARCMP {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarDecCmpR8 ( pdecleft : *const super::super::Foundation:: DECIMAL , dblright : f64 ) -> VARCMP );
    VarDecCmpR8(pdecleft, dblright)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarDecDiv(pdecleft: *const super::super::Foundation::DECIMAL, pdecright: *const super::super::Foundation::DECIMAL) -> ::windows::core::Result<super::super::Foundation::DECIMAL> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarDecDiv ( pdecleft : *const super::super::Foundation:: DECIMAL , pdecright : *const super::super::Foundation:: DECIMAL , pdecresult : *mut super::super::Foundation:: DECIMAL ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarDecDiv(pdecleft, pdecright, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarDecFix(pdecin: *const super::super::Foundation::DECIMAL) -> ::windows::core::Result<super::super::Foundation::DECIMAL> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarDecFix ( pdecin : *const super::super::Foundation:: DECIMAL , pdecresult : *mut super::super::Foundation:: DECIMAL ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarDecFix(pdecin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarDecFromBool<P0>(boolin: P0) -> ::windows::core::Result<super::super::Foundation::DECIMAL>
where
    P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarDecFromBool ( boolin : super::super::Foundation:: VARIANT_BOOL , pdecout : *mut super::super::Foundation:: DECIMAL ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarDecFromBool(boolin.into(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn VarDecFromCy(cyin: super::Com::CY) -> ::windows::core::Result<super::super::Foundation::DECIMAL> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarDecFromCy ( cyin : super::Com:: CY , pdecout : *mut super::super::Foundation:: DECIMAL ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarDecFromCy(::core::mem::transmute(cyin), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarDecFromDate(datein: f64) -> ::windows::core::Result<super::super::Foundation::DECIMAL> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarDecFromDate ( datein : f64 , pdecout : *mut super::super::Foundation:: DECIMAL ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarDecFromDate(datein, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn VarDecFromDisp<P0>(pdispin: P0, lcid: u32) -> ::windows::core::Result<super::super::Foundation::DECIMAL>
where
    P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarDecFromDisp ( pdispin : * mut::core::ffi::c_void , lcid : u32 , pdecout : *mut super::super::Foundation:: DECIMAL ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarDecFromDisp(pdispin.into().abi(), lcid, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarDecFromI1<P0>(cin: P0) -> ::windows::core::Result<super::super::Foundation::DECIMAL>
where
    P0: ::std::convert::Into<super::super::Foundation::CHAR>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarDecFromI1 ( cin : super::super::Foundation:: CHAR , pdecout : *mut super::super::Foundation:: DECIMAL ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarDecFromI1(cin.into(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarDecFromI2(uiin: i16) -> ::windows::core::Result<super::super::Foundation::DECIMAL> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarDecFromI2 ( uiin : i16 , pdecout : *mut super::super::Foundation:: DECIMAL ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarDecFromI2(uiin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarDecFromI4(lin: i32) -> ::windows::core::Result<super::super::Foundation::DECIMAL> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarDecFromI4 ( lin : i32 , pdecout : *mut super::super::Foundation:: DECIMAL ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarDecFromI4(lin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarDecFromI8(i64in: i64) -> ::windows::core::Result<super::super::Foundation::DECIMAL> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarDecFromI8 ( i64in : i64 , pdecout : *mut super::super::Foundation:: DECIMAL ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarDecFromI8(i64in, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarDecFromR4(fltin: f32) -> ::windows::core::Result<super::super::Foundation::DECIMAL> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarDecFromR4 ( fltin : f32 , pdecout : *mut super::super::Foundation:: DECIMAL ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarDecFromR4(fltin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarDecFromR8(dblin: f64) -> ::windows::core::Result<super::super::Foundation::DECIMAL> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarDecFromR8 ( dblin : f64 , pdecout : *mut super::super::Foundation:: DECIMAL ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarDecFromR8(dblin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarDecFromStr<P0>(strin: P0, lcid: u32, dwflags: u32) -> ::windows::core::Result<super::super::Foundation::DECIMAL>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarDecFromStr ( strin : :: windows::core::PCWSTR , lcid : u32 , dwflags : u32 , pdecout : *mut super::super::Foundation:: DECIMAL ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarDecFromStr(strin.into().abi(), lcid, dwflags, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarDecFromUI1(bin: u8) -> ::windows::core::Result<super::super::Foundation::DECIMAL> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarDecFromUI1 ( bin : u8 , pdecout : *mut super::super::Foundation:: DECIMAL ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarDecFromUI1(bin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarDecFromUI2(uiin: u16) -> ::windows::core::Result<super::super::Foundation::DECIMAL> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarDecFromUI2 ( uiin : u16 , pdecout : *mut super::super::Foundation:: DECIMAL ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarDecFromUI2(uiin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarDecFromUI4(ulin: u32) -> ::windows::core::Result<super::super::Foundation::DECIMAL> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarDecFromUI4 ( ulin : u32 , pdecout : *mut super::super::Foundation:: DECIMAL ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarDecFromUI4(ulin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarDecFromUI8(ui64in: u64) -> ::windows::core::Result<super::super::Foundation::DECIMAL> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarDecFromUI8 ( ui64in : u64 , pdecout : *mut super::super::Foundation:: DECIMAL ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarDecFromUI8(ui64in, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarDecInt(pdecin: *const super::super::Foundation::DECIMAL) -> ::windows::core::Result<super::super::Foundation::DECIMAL> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarDecInt ( pdecin : *const super::super::Foundation:: DECIMAL , pdecresult : *mut super::super::Foundation:: DECIMAL ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarDecInt(pdecin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarDecMul(pdecleft: *const super::super::Foundation::DECIMAL, pdecright: *const super::super::Foundation::DECIMAL) -> ::windows::core::Result<super::super::Foundation::DECIMAL> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarDecMul ( pdecleft : *const super::super::Foundation:: DECIMAL , pdecright : *const super::super::Foundation:: DECIMAL , pdecresult : *mut super::super::Foundation:: DECIMAL ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarDecMul(pdecleft, pdecright, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarDecNeg(pdecin: *const super::super::Foundation::DECIMAL) -> ::windows::core::Result<super::super::Foundation::DECIMAL> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarDecNeg ( pdecin : *const super::super::Foundation:: DECIMAL , pdecresult : *mut super::super::Foundation:: DECIMAL ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarDecNeg(pdecin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarDecRound(pdecin: *const super::super::Foundation::DECIMAL, cdecimals: i32) -> ::windows::core::Result<super::super::Foundation::DECIMAL> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarDecRound ( pdecin : *const super::super::Foundation:: DECIMAL , cdecimals : i32 , pdecresult : *mut super::super::Foundation:: DECIMAL ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarDecRound(pdecin, cdecimals, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarDecSub(pdecleft: *const super::super::Foundation::DECIMAL, pdecright: *const super::super::Foundation::DECIMAL) -> ::windows::core::Result<super::super::Foundation::DECIMAL> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarDecSub ( pdecleft : *const super::super::Foundation:: DECIMAL , pdecright : *const super::super::Foundation:: DECIMAL , pdecresult : *mut super::super::Foundation:: DECIMAL ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarDecSub(pdecleft, pdecright, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn VarDiv(pvarleft: *const super::Com::VARIANT, pvarright: *const super::Com::VARIANT) -> ::windows::core::Result<super::Com::VARIANT> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarDiv ( pvarleft : *const super::Com:: VARIANT , pvarright : *const super::Com:: VARIANT , pvarresult : *mut super::Com:: VARIANT ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarDiv(pvarleft, pvarright, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn VarEqv(pvarleft: *const super::Com::VARIANT, pvarright: *const super::Com::VARIANT) -> ::windows::core::Result<super::Com::VARIANT> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarEqv ( pvarleft : *const super::Com:: VARIANT , pvarright : *const super::Com:: VARIANT , pvarresult : *mut super::Com:: VARIANT ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarEqv(pvarleft, pvarright, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn VarFix(pvarin: *const super::Com::VARIANT) -> ::windows::core::Result<super::Com::VARIANT> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarFix ( pvarin : *const super::Com:: VARIANT , pvarresult : *mut super::Com:: VARIANT ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarFix(pvarin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn VarFormat<P0>(pvarin: *const super::Com::VARIANT, pstrformat: P0, ifirstday: VARFORMAT_FIRST_DAY, ifirstweek: VARFORMAT_FIRST_WEEK, dwflags: u32) -> ::windows::core::Result<::windows::core::BSTR>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarFormat ( pvarin : *const super::Com:: VARIANT , pstrformat : :: windows::core::PCWSTR , ifirstday : VARFORMAT_FIRST_DAY , ifirstweek : VARFORMAT_FIRST_WEEK , dwflags : u32 , pbstrout : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarFormat(pvarin, pstrformat.into().abi(), ifirstday, ifirstweek, dwflags, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn VarFormatCurrency(pvarin: *const super::Com::VARIANT, inumdig: i32, iinclead: i32, iuseparens: i32, igroup: i32, dwflags: u32) -> ::windows::core::Result<::windows::core::BSTR> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarFormatCurrency ( pvarin : *const super::Com:: VARIANT , inumdig : i32 , iinclead : i32 , iuseparens : i32 , igroup : i32 , dwflags : u32 , pbstrout : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarFormatCurrency(pvarin, inumdig, iinclead, iuseparens, igroup, dwflags, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn VarFormatDateTime(pvarin: *const super::Com::VARIANT, inamedformat: VARFORMAT_NAMED_FORMAT, dwflags: u32) -> ::windows::core::Result<::windows::core::BSTR> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarFormatDateTime ( pvarin : *const super::Com:: VARIANT , inamedformat : VARFORMAT_NAMED_FORMAT , dwflags : u32 , pbstrout : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarFormatDateTime(pvarin, inamedformat, dwflags, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn VarFormatFromTokens<P0>(pvarin: *const super::Com::VARIANT, pstrformat: P0, pbtokcur: *const u8, dwflags: u32, pbstrout: *mut ::windows::core::BSTR, lcid: u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarFormatFromTokens ( pvarin : *const super::Com:: VARIANT , pstrformat : :: windows::core::PCWSTR , pbtokcur : *const u8 , dwflags : u32 , pbstrout : *mut * mut::core::ffi::c_void , lcid : u32 ) -> :: windows::core::HRESULT );
    VarFormatFromTokens(pvarin, pstrformat.into().abi(), pbtokcur, dwflags, ::core::mem::transmute(pbstrout), lcid).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn VarFormatNumber(pvarin: *const super::Com::VARIANT, inumdig: i32, iinclead: VARFORMAT_LEADING_DIGIT, iuseparens: VARFORMAT_PARENTHESES, igroup: VARFORMAT_GROUP, dwflags: u32) -> ::windows::core::Result<::windows::core::BSTR> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarFormatNumber ( pvarin : *const super::Com:: VARIANT , inumdig : i32 , iinclead : VARFORMAT_LEADING_DIGIT , iuseparens : VARFORMAT_PARENTHESES , igroup : VARFORMAT_GROUP , dwflags : u32 , pbstrout : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarFormatNumber(pvarin, inumdig, iinclead, iuseparens, igroup, dwflags, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn VarFormatPercent(pvarin: *const super::Com::VARIANT, inumdig: i32, iinclead: VARFORMAT_LEADING_DIGIT, iuseparens: VARFORMAT_PARENTHESES, igroup: VARFORMAT_GROUP, dwflags: u32) -> ::windows::core::Result<::windows::core::BSTR> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarFormatPercent ( pvarin : *const super::Com:: VARIANT , inumdig : i32 , iinclead : VARFORMAT_LEADING_DIGIT , iuseparens : VARFORMAT_PARENTHESES , igroup : VARFORMAT_GROUP , dwflags : u32 , pbstrout : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarFormatPercent(pvarin, inumdig, iinclead, iuseparens, igroup, dwflags, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarI1FromBool<P0>(boolin: P0, pcout: ::windows::core::PSTR) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI1FromBool ( boolin : super::super::Foundation:: VARIANT_BOOL , pcout : :: windows::core::PSTR ) -> :: windows::core::HRESULT );
    VarI1FromBool(boolin.into(), ::core::mem::transmute(pcout)).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarI1FromCy(cyin: super::Com::CY, pcout: ::windows::core::PSTR) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI1FromCy ( cyin : super::Com:: CY , pcout : :: windows::core::PSTR ) -> :: windows::core::HRESULT );
    VarI1FromCy(::core::mem::transmute(cyin), ::core::mem::transmute(pcout)).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarI1FromDate(datein: f64, pcout: ::windows::core::PSTR) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI1FromDate ( datein : f64 , pcout : :: windows::core::PSTR ) -> :: windows::core::HRESULT );
    VarI1FromDate(datein, ::core::mem::transmute(pcout)).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarI1FromDec(pdecin: *const super::super::Foundation::DECIMAL, pcout: ::windows::core::PSTR) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI1FromDec ( pdecin : *const super::super::Foundation:: DECIMAL , pcout : :: windows::core::PSTR ) -> :: windows::core::HRESULT );
    VarI1FromDec(pdecin, ::core::mem::transmute(pcout)).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarI1FromDisp<P0>(pdispin: P0, lcid: u32, pcout: ::windows::core::PSTR) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI1FromDisp ( pdispin : * mut::core::ffi::c_void , lcid : u32 , pcout : :: windows::core::PSTR ) -> :: windows::core::HRESULT );
    VarI1FromDisp(pdispin.into().abi(), lcid, ::core::mem::transmute(pcout)).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarI1FromI2(uiin: i16, pcout: ::windows::core::PSTR) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI1FromI2 ( uiin : i16 , pcout : :: windows::core::PSTR ) -> :: windows::core::HRESULT );
    VarI1FromI2(uiin, ::core::mem::transmute(pcout)).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarI1FromI4(lin: i32, pcout: ::windows::core::PSTR) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI1FromI4 ( lin : i32 , pcout : :: windows::core::PSTR ) -> :: windows::core::HRESULT );
    VarI1FromI4(lin, ::core::mem::transmute(pcout)).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarI1FromI8(i64in: i64, pcout: ::windows::core::PSTR) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI1FromI8 ( i64in : i64 , pcout : :: windows::core::PSTR ) -> :: windows::core::HRESULT );
    VarI1FromI8(i64in, ::core::mem::transmute(pcout)).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarI1FromR4(fltin: f32, pcout: ::windows::core::PSTR) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI1FromR4 ( fltin : f32 , pcout : :: windows::core::PSTR ) -> :: windows::core::HRESULT );
    VarI1FromR4(fltin, ::core::mem::transmute(pcout)).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarI1FromR8(dblin: f64, pcout: ::windows::core::PSTR) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI1FromR8 ( dblin : f64 , pcout : :: windows::core::PSTR ) -> :: windows::core::HRESULT );
    VarI1FromR8(dblin, ::core::mem::transmute(pcout)).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarI1FromStr<P0>(strin: P0, lcid: u32, dwflags: u32, pcout: ::windows::core::PSTR) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI1FromStr ( strin : :: windows::core::PCWSTR , lcid : u32 , dwflags : u32 , pcout : :: windows::core::PSTR ) -> :: windows::core::HRESULT );
    VarI1FromStr(strin.into().abi(), lcid, dwflags, ::core::mem::transmute(pcout)).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarI1FromUI1(bin: u8, pcout: ::windows::core::PSTR) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI1FromUI1 ( bin : u8 , pcout : :: windows::core::PSTR ) -> :: windows::core::HRESULT );
    VarI1FromUI1(bin, ::core::mem::transmute(pcout)).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarI1FromUI2(uiin: u16, pcout: ::windows::core::PSTR) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI1FromUI2 ( uiin : u16 , pcout : :: windows::core::PSTR ) -> :: windows::core::HRESULT );
    VarI1FromUI2(uiin, ::core::mem::transmute(pcout)).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarI1FromUI4(ulin: u32, pcout: ::windows::core::PSTR) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI1FromUI4 ( ulin : u32 , pcout : :: windows::core::PSTR ) -> :: windows::core::HRESULT );
    VarI1FromUI4(ulin, ::core::mem::transmute(pcout)).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarI1FromUI8(i64in: u64, pcout: ::windows::core::PSTR) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI1FromUI8 ( i64in : u64 , pcout : :: windows::core::PSTR ) -> :: windows::core::HRESULT );
    VarI1FromUI8(i64in, ::core::mem::transmute(pcout)).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarI2FromBool<P0>(boolin: P0) -> ::windows::core::Result<i16>
where
    P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI2FromBool ( boolin : super::super::Foundation:: VARIANT_BOOL , psout : *mut i16 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarI2FromBool(boolin.into(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarI2FromCy(cyin: super::Com::CY, psout: *mut i16) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI2FromCy ( cyin : super::Com:: CY , psout : *mut i16 ) -> :: windows::core::HRESULT );
    VarI2FromCy(::core::mem::transmute(cyin), psout).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarI2FromDate(datein: f64) -> ::windows::core::Result<i16> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI2FromDate ( datein : f64 , psout : *mut i16 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarI2FromDate(datein, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarI2FromDec(pdecin: *const super::super::Foundation::DECIMAL) -> ::windows::core::Result<i16> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI2FromDec ( pdecin : *const super::super::Foundation:: DECIMAL , psout : *mut i16 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarI2FromDec(pdecin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarI2FromDisp<P0>(pdispin: P0, lcid: u32) -> ::windows::core::Result<i16>
where
    P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI2FromDisp ( pdispin : * mut::core::ffi::c_void , lcid : u32 , psout : *mut i16 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarI2FromDisp(pdispin.into().abi(), lcid, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarI2FromI1<P0>(cin: P0) -> ::windows::core::Result<i16>
where
    P0: ::std::convert::Into<super::super::Foundation::CHAR>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI2FromI1 ( cin : super::super::Foundation:: CHAR , psout : *mut i16 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarI2FromI1(cin.into(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarI2FromI4(lin: i32) -> ::windows::core::Result<i16> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI2FromI4 ( lin : i32 , psout : *mut i16 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarI2FromI4(lin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarI2FromI8(i64in: i64) -> ::windows::core::Result<i16> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI2FromI8 ( i64in : i64 , psout : *mut i16 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarI2FromI8(i64in, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarI2FromR4(fltin: f32) -> ::windows::core::Result<i16> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI2FromR4 ( fltin : f32 , psout : *mut i16 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarI2FromR4(fltin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarI2FromR8(dblin: f64) -> ::windows::core::Result<i16> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI2FromR8 ( dblin : f64 , psout : *mut i16 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarI2FromR8(dblin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarI2FromStr<P0>(strin: P0, lcid: u32, dwflags: u32) -> ::windows::core::Result<i16>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI2FromStr ( strin : :: windows::core::PCWSTR , lcid : u32 , dwflags : u32 , psout : *mut i16 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarI2FromStr(strin.into().abi(), lcid, dwflags, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarI2FromUI1(bin: u8) -> ::windows::core::Result<i16> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI2FromUI1 ( bin : u8 , psout : *mut i16 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarI2FromUI1(bin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarI2FromUI2(uiin: u16) -> ::windows::core::Result<i16> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI2FromUI2 ( uiin : u16 , psout : *mut i16 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarI2FromUI2(uiin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarI2FromUI4(ulin: u32) -> ::windows::core::Result<i16> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI2FromUI4 ( ulin : u32 , psout : *mut i16 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarI2FromUI4(ulin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarI2FromUI8(ui64in: u64) -> ::windows::core::Result<i16> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI2FromUI8 ( ui64in : u64 , psout : *mut i16 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarI2FromUI8(ui64in, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarI4FromBool<P0>(boolin: P0) -> ::windows::core::Result<i32>
where
    P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI4FromBool ( boolin : super::super::Foundation:: VARIANT_BOOL , plout : *mut i32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarI4FromBool(boolin.into(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarI4FromCy(cyin: super::Com::CY) -> ::windows::core::Result<i32> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI4FromCy ( cyin : super::Com:: CY , plout : *mut i32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarI4FromCy(::core::mem::transmute(cyin), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarI4FromDate(datein: f64) -> ::windows::core::Result<i32> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI4FromDate ( datein : f64 , plout : *mut i32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarI4FromDate(datein, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarI4FromDec(pdecin: *const super::super::Foundation::DECIMAL) -> ::windows::core::Result<i32> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI4FromDec ( pdecin : *const super::super::Foundation:: DECIMAL , plout : *mut i32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarI4FromDec(pdecin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarI4FromDisp<P0>(pdispin: P0, lcid: u32) -> ::windows::core::Result<i32>
where
    P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI4FromDisp ( pdispin : * mut::core::ffi::c_void , lcid : u32 , plout : *mut i32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarI4FromDisp(pdispin.into().abi(), lcid, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarI4FromI1<P0>(cin: P0) -> ::windows::core::Result<i32>
where
    P0: ::std::convert::Into<super::super::Foundation::CHAR>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI4FromI1 ( cin : super::super::Foundation:: CHAR , plout : *mut i32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarI4FromI1(cin.into(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarI4FromI2(sin: i16) -> ::windows::core::Result<i32> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI4FromI2 ( sin : i16 , plout : *mut i32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarI4FromI2(sin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarI4FromI8(i64in: i64) -> ::windows::core::Result<i32> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI4FromI8 ( i64in : i64 , plout : *mut i32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarI4FromI8(i64in, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarI4FromR4(fltin: f32) -> ::windows::core::Result<i32> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI4FromR4 ( fltin : f32 , plout : *mut i32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarI4FromR4(fltin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarI4FromR8(dblin: f64) -> ::windows::core::Result<i32> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI4FromR8 ( dblin : f64 , plout : *mut i32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarI4FromR8(dblin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarI4FromStr<P0>(strin: P0, lcid: u32, dwflags: u32) -> ::windows::core::Result<i32>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI4FromStr ( strin : :: windows::core::PCWSTR , lcid : u32 , dwflags : u32 , plout : *mut i32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarI4FromStr(strin.into().abi(), lcid, dwflags, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarI4FromUI1(bin: u8) -> ::windows::core::Result<i32> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI4FromUI1 ( bin : u8 , plout : *mut i32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarI4FromUI1(bin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarI4FromUI2(uiin: u16) -> ::windows::core::Result<i32> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI4FromUI2 ( uiin : u16 , plout : *mut i32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarI4FromUI2(uiin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarI4FromUI4(ulin: u32) -> ::windows::core::Result<i32> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI4FromUI4 ( ulin : u32 , plout : *mut i32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarI4FromUI4(ulin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarI4FromUI8(ui64in: u64) -> ::windows::core::Result<i32> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI4FromUI8 ( ui64in : u64 , plout : *mut i32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarI4FromUI8(ui64in, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarI8FromBool<P0>(boolin: P0) -> ::windows::core::Result<i64>
where
    P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI8FromBool ( boolin : super::super::Foundation:: VARIANT_BOOL , pi64out : *mut i64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarI8FromBool(boolin.into(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarI8FromCy(cyin: super::Com::CY) -> ::windows::core::Result<i64> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI8FromCy ( cyin : super::Com:: CY , pi64out : *mut i64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarI8FromCy(::core::mem::transmute(cyin), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarI8FromDate(datein: f64) -> ::windows::core::Result<i64> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI8FromDate ( datein : f64 , pi64out : *mut i64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarI8FromDate(datein, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarI8FromDec(pdecin: *const super::super::Foundation::DECIMAL) -> ::windows::core::Result<i64> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI8FromDec ( pdecin : *const super::super::Foundation:: DECIMAL , pi64out : *mut i64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarI8FromDec(pdecin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarI8FromDisp<P0>(pdispin: P0, lcid: u32) -> ::windows::core::Result<i64>
where
    P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI8FromDisp ( pdispin : * mut::core::ffi::c_void , lcid : u32 , pi64out : *mut i64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarI8FromDisp(pdispin.into().abi(), lcid, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarI8FromI1<P0>(cin: P0) -> ::windows::core::Result<i64>
where
    P0: ::std::convert::Into<super::super::Foundation::CHAR>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI8FromI1 ( cin : super::super::Foundation:: CHAR , pi64out : *mut i64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarI8FromI1(cin.into(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarI8FromI2(sin: i16) -> ::windows::core::Result<i64> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI8FromI2 ( sin : i16 , pi64out : *mut i64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarI8FromI2(sin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarI8FromR4(fltin: f32) -> ::windows::core::Result<i64> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI8FromR4 ( fltin : f32 , pi64out : *mut i64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarI8FromR4(fltin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarI8FromR8(dblin: f64) -> ::windows::core::Result<i64> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI8FromR8 ( dblin : f64 , pi64out : *mut i64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarI8FromR8(dblin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarI8FromStr<P0>(strin: P0, lcid: u32, dwflags: u32) -> ::windows::core::Result<i64>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI8FromStr ( strin : :: windows::core::PCWSTR , lcid : u32 , dwflags : u32 , pi64out : *mut i64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarI8FromStr(strin.into().abi(), lcid, dwflags, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarI8FromUI1(bin: u8) -> ::windows::core::Result<i64> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI8FromUI1 ( bin : u8 , pi64out : *mut i64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarI8FromUI1(bin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarI8FromUI2(uiin: u16) -> ::windows::core::Result<i64> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI8FromUI2 ( uiin : u16 , pi64out : *mut i64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarI8FromUI2(uiin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarI8FromUI4(ulin: u32) -> ::windows::core::Result<i64> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI8FromUI4 ( ulin : u32 , pi64out : *mut i64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarI8FromUI4(ulin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarI8FromUI8(ui64in: u64) -> ::windows::core::Result<i64> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarI8FromUI8 ( ui64in : u64 , pi64out : *mut i64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarI8FromUI8(ui64in, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn VarIdiv(pvarleft: *const super::Com::VARIANT, pvarright: *const super::Com::VARIANT) -> ::windows::core::Result<super::Com::VARIANT> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarIdiv ( pvarleft : *const super::Com:: VARIANT , pvarright : *const super::Com:: VARIANT , pvarresult : *mut super::Com:: VARIANT ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarIdiv(pvarleft, pvarright, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn VarImp(pvarleft: *const super::Com::VARIANT, pvarright: *const super::Com::VARIANT) -> ::windows::core::Result<super::Com::VARIANT> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarImp ( pvarleft : *const super::Com:: VARIANT , pvarright : *const super::Com:: VARIANT , pvarresult : *mut super::Com:: VARIANT ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarImp(pvarleft, pvarright, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn VarInt(pvarin: *const super::Com::VARIANT) -> ::windows::core::Result<super::Com::VARIANT> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarInt ( pvarin : *const super::Com:: VARIANT , pvarresult : *mut super::Com:: VARIANT ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarInt(pvarin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn VarMod(pvarleft: *const super::Com::VARIANT, pvarright: *const super::Com::VARIANT) -> ::windows::core::Result<super::Com::VARIANT> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarMod ( pvarleft : *const super::Com:: VARIANT , pvarright : *const super::Com:: VARIANT , pvarresult : *mut super::Com:: VARIANT ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarMod(pvarleft, pvarright, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarMonthName(imonth: i32, fabbrev: i32, dwflags: u32) -> ::windows::core::Result<::windows::core::BSTR> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarMonthName ( imonth : i32 , fabbrev : i32 , dwflags : u32 , pbstrout : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarMonthName(imonth, fabbrev, dwflags, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn VarMul(pvarleft: *const super::Com::VARIANT, pvarright: *const super::Com::VARIANT) -> ::windows::core::Result<super::Com::VARIANT> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarMul ( pvarleft : *const super::Com:: VARIANT , pvarright : *const super::Com:: VARIANT , pvarresult : *mut super::Com:: VARIANT ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarMul(pvarleft, pvarright, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn VarNeg(pvarin: *const super::Com::VARIANT) -> ::windows::core::Result<super::Com::VARIANT> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarNeg ( pvarin : *const super::Com:: VARIANT , pvarresult : *mut super::Com:: VARIANT ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarNeg(pvarin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn VarNot(pvarin: *const super::Com::VARIANT) -> ::windows::core::Result<super::Com::VARIANT> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarNot ( pvarin : *const super::Com:: VARIANT , pvarresult : *mut super::Com:: VARIANT ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarNot(pvarin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn VarNumFromParseNum(pnumprs: *const NUMPARSE, rgbdig: *const u8, dwvtbits: u32) -> ::windows::core::Result<super::Com::VARIANT> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarNumFromParseNum ( pnumprs : *const NUMPARSE , rgbdig : *const u8 , dwvtbits : u32 , pvar : *mut super::Com:: VARIANT ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarNumFromParseNum(pnumprs, rgbdig, dwvtbits, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn VarOr(pvarleft: *const super::Com::VARIANT, pvarright: *const super::Com::VARIANT) -> ::windows::core::Result<super::Com::VARIANT> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarOr ( pvarleft : *const super::Com:: VARIANT , pvarright : *const super::Com:: VARIANT , pvarresult : *mut super::Com:: VARIANT ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarOr(pvarleft, pvarright, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarParseNumFromStr<P0>(strin: P0, lcid: u32, dwflags: u32, pnumprs: *mut NUMPARSE, rgbdig: *mut u8) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarParseNumFromStr ( strin : :: windows::core::PCWSTR , lcid : u32 , dwflags : u32 , pnumprs : *mut NUMPARSE , rgbdig : *mut u8 ) -> :: windows::core::HRESULT );
    VarParseNumFromStr(strin.into().abi(), lcid, dwflags, pnumprs, rgbdig).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn VarPow(pvarleft: *const super::Com::VARIANT, pvarright: *const super::Com::VARIANT) -> ::windows::core::Result<super::Com::VARIANT> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarPow ( pvarleft : *const super::Com:: VARIANT , pvarright : *const super::Com:: VARIANT , pvarresult : *mut super::Com:: VARIANT ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarPow(pvarleft, pvarright, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarR4CmpR8(fltleft: f32, dblright: f64) -> VARCMP {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarR4CmpR8 ( fltleft : f32 , dblright : f64 ) -> VARCMP );
    VarR4CmpR8(fltleft, dblright)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarR4FromBool<P0>(boolin: P0) -> ::windows::core::Result<f32>
where
    P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarR4FromBool ( boolin : super::super::Foundation:: VARIANT_BOOL , pfltout : *mut f32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarR4FromBool(boolin.into(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarR4FromCy(cyin: super::Com::CY, pfltout: *mut f32) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarR4FromCy ( cyin : super::Com:: CY , pfltout : *mut f32 ) -> :: windows::core::HRESULT );
    VarR4FromCy(::core::mem::transmute(cyin), pfltout).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarR4FromDate(datein: f64) -> ::windows::core::Result<f32> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarR4FromDate ( datein : f64 , pfltout : *mut f32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarR4FromDate(datein, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarR4FromDec(pdecin: *const super::super::Foundation::DECIMAL) -> ::windows::core::Result<f32> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarR4FromDec ( pdecin : *const super::super::Foundation:: DECIMAL , pfltout : *mut f32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarR4FromDec(pdecin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarR4FromDisp<P0>(pdispin: P0, lcid: u32) -> ::windows::core::Result<f32>
where
    P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarR4FromDisp ( pdispin : * mut::core::ffi::c_void , lcid : u32 , pfltout : *mut f32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarR4FromDisp(pdispin.into().abi(), lcid, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarR4FromI1<P0>(cin: P0) -> ::windows::core::Result<f32>
where
    P0: ::std::convert::Into<super::super::Foundation::CHAR>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarR4FromI1 ( cin : super::super::Foundation:: CHAR , pfltout : *mut f32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarR4FromI1(cin.into(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarR4FromI2(sin: i16) -> ::windows::core::Result<f32> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarR4FromI2 ( sin : i16 , pfltout : *mut f32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarR4FromI2(sin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarR4FromI4(lin: i32) -> ::windows::core::Result<f32> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarR4FromI4 ( lin : i32 , pfltout : *mut f32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarR4FromI4(lin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarR4FromI8(i64in: i64) -> ::windows::core::Result<f32> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarR4FromI8 ( i64in : i64 , pfltout : *mut f32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarR4FromI8(i64in, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarR4FromR8(dblin: f64) -> ::windows::core::Result<f32> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarR4FromR8 ( dblin : f64 , pfltout : *mut f32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarR4FromR8(dblin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarR4FromStr<P0>(strin: P0, lcid: u32, dwflags: u32) -> ::windows::core::Result<f32>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarR4FromStr ( strin : :: windows::core::PCWSTR , lcid : u32 , dwflags : u32 , pfltout : *mut f32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarR4FromStr(strin.into().abi(), lcid, dwflags, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarR4FromUI1(bin: u8) -> ::windows::core::Result<f32> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarR4FromUI1 ( bin : u8 , pfltout : *mut f32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarR4FromUI1(bin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarR4FromUI2(uiin: u16) -> ::windows::core::Result<f32> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarR4FromUI2 ( uiin : u16 , pfltout : *mut f32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarR4FromUI2(uiin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarR4FromUI4(ulin: u32) -> ::windows::core::Result<f32> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarR4FromUI4 ( ulin : u32 , pfltout : *mut f32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarR4FromUI4(ulin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarR4FromUI8(ui64in: u64) -> ::windows::core::Result<f32> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarR4FromUI8 ( ui64in : u64 , pfltout : *mut f32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarR4FromUI8(ui64in, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarR8FromBool<P0>(boolin: P0) -> ::windows::core::Result<f64>
where
    P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarR8FromBool ( boolin : super::super::Foundation:: VARIANT_BOOL , pdblout : *mut f64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarR8FromBool(boolin.into(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarR8FromCy(cyin: super::Com::CY, pdblout: *mut f64) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarR8FromCy ( cyin : super::Com:: CY , pdblout : *mut f64 ) -> :: windows::core::HRESULT );
    VarR8FromCy(::core::mem::transmute(cyin), pdblout).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarR8FromDate(datein: f64) -> ::windows::core::Result<f64> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarR8FromDate ( datein : f64 , pdblout : *mut f64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarR8FromDate(datein, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarR8FromDec(pdecin: *const super::super::Foundation::DECIMAL) -> ::windows::core::Result<f64> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarR8FromDec ( pdecin : *const super::super::Foundation:: DECIMAL , pdblout : *mut f64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarR8FromDec(pdecin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarR8FromDisp<P0>(pdispin: P0, lcid: u32) -> ::windows::core::Result<f64>
where
    P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarR8FromDisp ( pdispin : * mut::core::ffi::c_void , lcid : u32 , pdblout : *mut f64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarR8FromDisp(pdispin.into().abi(), lcid, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarR8FromI1<P0>(cin: P0, pdblout: *mut f64) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::CHAR>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarR8FromI1 ( cin : super::super::Foundation:: CHAR , pdblout : *mut f64 ) -> :: windows::core::HRESULT );
    VarR8FromI1(cin.into(), pdblout).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarR8FromI2(sin: i16) -> ::windows::core::Result<f64> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarR8FromI2 ( sin : i16 , pdblout : *mut f64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarR8FromI2(sin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarR8FromI4(lin: i32) -> ::windows::core::Result<f64> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarR8FromI4 ( lin : i32 , pdblout : *mut f64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarR8FromI4(lin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarR8FromI8(i64in: i64) -> ::windows::core::Result<f64> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarR8FromI8 ( i64in : i64 , pdblout : *mut f64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarR8FromI8(i64in, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarR8FromR4(fltin: f32) -> ::windows::core::Result<f64> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarR8FromR4 ( fltin : f32 , pdblout : *mut f64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarR8FromR4(fltin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarR8FromStr<P0>(strin: P0, lcid: u32, dwflags: u32) -> ::windows::core::Result<f64>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarR8FromStr ( strin : :: windows::core::PCWSTR , lcid : u32 , dwflags : u32 , pdblout : *mut f64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarR8FromStr(strin.into().abi(), lcid, dwflags, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarR8FromUI1(bin: u8) -> ::windows::core::Result<f64> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarR8FromUI1 ( bin : u8 , pdblout : *mut f64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarR8FromUI1(bin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarR8FromUI2(uiin: u16) -> ::windows::core::Result<f64> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarR8FromUI2 ( uiin : u16 , pdblout : *mut f64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarR8FromUI2(uiin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarR8FromUI4(ulin: u32) -> ::windows::core::Result<f64> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarR8FromUI4 ( ulin : u32 , pdblout : *mut f64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarR8FromUI4(ulin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarR8FromUI8(ui64in: u64) -> ::windows::core::Result<f64> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarR8FromUI8 ( ui64in : u64 , pdblout : *mut f64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarR8FromUI8(ui64in, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarR8Pow(dblleft: f64, dblright: f64) -> ::windows::core::Result<f64> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarR8Pow ( dblleft : f64 , dblright : f64 , pdblresult : *mut f64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarR8Pow(dblleft, dblright, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarR8Round(dblin: f64, cdecimals: i32) -> ::windows::core::Result<f64> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarR8Round ( dblin : f64 , cdecimals : i32 , pdblresult : *mut f64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarR8Round(dblin, cdecimals, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn VarRound(pvarin: *const super::Com::VARIANT, cdecimals: i32) -> ::windows::core::Result<super::Com::VARIANT> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarRound ( pvarin : *const super::Com:: VARIANT , cdecimals : i32 , pvarresult : *mut super::Com:: VARIANT ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarRound(pvarin, cdecimals, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn VarSub(pvarleft: *const super::Com::VARIANT, pvarright: *const super::Com::VARIANT) -> ::windows::core::Result<super::Com::VARIANT> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarSub ( pvarleft : *const super::Com:: VARIANT , pvarright : *const super::Com:: VARIANT , pvarresult : *mut super::Com:: VARIANT ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarSub(pvarleft, pvarright, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarTokenizeFormatString<P0>(pstrformat: P0, rgbtok: &mut [u8], ifirstday: VARFORMAT_FIRST_DAY, ifirstweek: VARFORMAT_FIRST_WEEK, lcid: u32, pcbactual: ::core::option::Option<*const i32>) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarTokenizeFormatString ( pstrformat : :: windows::core::PCWSTR , rgbtok : *mut u8 , cbtok : i32 , ifirstday : VARFORMAT_FIRST_DAY , ifirstweek : VARFORMAT_FIRST_WEEK , lcid : u32 , pcbactual : *const i32 ) -> :: windows::core::HRESULT );
    VarTokenizeFormatString(pstrformat.into().abi(), ::core::mem::transmute(rgbtok.as_ptr()), rgbtok.len() as _, ifirstday, ifirstweek, lcid, ::core::mem::transmute(pcbactual.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarUI1FromBool<P0>(boolin: P0) -> ::windows::core::Result<u8>
where
    P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI1FromBool ( boolin : super::super::Foundation:: VARIANT_BOOL , pbout : *mut u8 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUI1FromBool(boolin.into(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarUI1FromCy(cyin: super::Com::CY) -> ::windows::core::Result<u8> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI1FromCy ( cyin : super::Com:: CY , pbout : *mut u8 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUI1FromCy(::core::mem::transmute(cyin), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarUI1FromDate(datein: f64) -> ::windows::core::Result<u8> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI1FromDate ( datein : f64 , pbout : *mut u8 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUI1FromDate(datein, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarUI1FromDec(pdecin: *const super::super::Foundation::DECIMAL) -> ::windows::core::Result<u8> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI1FromDec ( pdecin : *const super::super::Foundation:: DECIMAL , pbout : *mut u8 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUI1FromDec(pdecin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarUI1FromDisp<P0>(pdispin: P0, lcid: u32) -> ::windows::core::Result<u8>
where
    P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI1FromDisp ( pdispin : * mut::core::ffi::c_void , lcid : u32 , pbout : *mut u8 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUI1FromDisp(pdispin.into().abi(), lcid, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarUI1FromI1<P0>(cin: P0) -> ::windows::core::Result<u8>
where
    P0: ::std::convert::Into<super::super::Foundation::CHAR>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI1FromI1 ( cin : super::super::Foundation:: CHAR , pbout : *mut u8 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUI1FromI1(cin.into(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarUI1FromI2(sin: i16) -> ::windows::core::Result<u8> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI1FromI2 ( sin : i16 , pbout : *mut u8 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUI1FromI2(sin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarUI1FromI4(lin: i32) -> ::windows::core::Result<u8> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI1FromI4 ( lin : i32 , pbout : *mut u8 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUI1FromI4(lin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarUI1FromI8(i64in: i64) -> ::windows::core::Result<u8> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI1FromI8 ( i64in : i64 , pbout : *mut u8 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUI1FromI8(i64in, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarUI1FromR4(fltin: f32) -> ::windows::core::Result<u8> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI1FromR4 ( fltin : f32 , pbout : *mut u8 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUI1FromR4(fltin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarUI1FromR8(dblin: f64) -> ::windows::core::Result<u8> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI1FromR8 ( dblin : f64 , pbout : *mut u8 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUI1FromR8(dblin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarUI1FromStr<P0>(strin: P0, lcid: u32, dwflags: u32) -> ::windows::core::Result<u8>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI1FromStr ( strin : :: windows::core::PCWSTR , lcid : u32 , dwflags : u32 , pbout : *mut u8 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUI1FromStr(strin.into().abi(), lcid, dwflags, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarUI1FromUI2(uiin: u16) -> ::windows::core::Result<u8> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI1FromUI2 ( uiin : u16 , pbout : *mut u8 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUI1FromUI2(uiin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarUI1FromUI4(ulin: u32) -> ::windows::core::Result<u8> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI1FromUI4 ( ulin : u32 , pbout : *mut u8 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUI1FromUI4(ulin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarUI1FromUI8(ui64in: u64) -> ::windows::core::Result<u8> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI1FromUI8 ( ui64in : u64 , pbout : *mut u8 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUI1FromUI8(ui64in, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarUI2FromBool<P0>(boolin: P0) -> ::windows::core::Result<u16>
where
    P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI2FromBool ( boolin : super::super::Foundation:: VARIANT_BOOL , puiout : *mut u16 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUI2FromBool(boolin.into(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarUI2FromCy(cyin: super::Com::CY) -> ::windows::core::Result<u16> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI2FromCy ( cyin : super::Com:: CY , puiout : *mut u16 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUI2FromCy(::core::mem::transmute(cyin), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarUI2FromDate(datein: f64) -> ::windows::core::Result<u16> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI2FromDate ( datein : f64 , puiout : *mut u16 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUI2FromDate(datein, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarUI2FromDec(pdecin: *const super::super::Foundation::DECIMAL) -> ::windows::core::Result<u16> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI2FromDec ( pdecin : *const super::super::Foundation:: DECIMAL , puiout : *mut u16 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUI2FromDec(pdecin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarUI2FromDisp<P0>(pdispin: P0, lcid: u32) -> ::windows::core::Result<u16>
where
    P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI2FromDisp ( pdispin : * mut::core::ffi::c_void , lcid : u32 , puiout : *mut u16 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUI2FromDisp(pdispin.into().abi(), lcid, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarUI2FromI1<P0>(cin: P0) -> ::windows::core::Result<u16>
where
    P0: ::std::convert::Into<super::super::Foundation::CHAR>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI2FromI1 ( cin : super::super::Foundation:: CHAR , puiout : *mut u16 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUI2FromI1(cin.into(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarUI2FromI2(uiin: i16) -> ::windows::core::Result<u16> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI2FromI2 ( uiin : i16 , puiout : *mut u16 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUI2FromI2(uiin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarUI2FromI4(lin: i32) -> ::windows::core::Result<u16> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI2FromI4 ( lin : i32 , puiout : *mut u16 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUI2FromI4(lin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarUI2FromI8(i64in: i64) -> ::windows::core::Result<u16> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI2FromI8 ( i64in : i64 , puiout : *mut u16 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUI2FromI8(i64in, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarUI2FromR4(fltin: f32) -> ::windows::core::Result<u16> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI2FromR4 ( fltin : f32 , puiout : *mut u16 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUI2FromR4(fltin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarUI2FromR8(dblin: f64, puiout: *mut u16) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI2FromR8 ( dblin : f64 , puiout : *mut u16 ) -> :: windows::core::HRESULT );
    VarUI2FromR8(dblin, puiout).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarUI2FromStr<P0>(strin: P0, lcid: u32, dwflags: u32) -> ::windows::core::Result<u16>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI2FromStr ( strin : :: windows::core::PCWSTR , lcid : u32 , dwflags : u32 , puiout : *mut u16 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUI2FromStr(strin.into().abi(), lcid, dwflags, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarUI2FromUI1(bin: u8) -> ::windows::core::Result<u16> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI2FromUI1 ( bin : u8 , puiout : *mut u16 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUI2FromUI1(bin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarUI2FromUI4(ulin: u32) -> ::windows::core::Result<u16> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI2FromUI4 ( ulin : u32 , puiout : *mut u16 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUI2FromUI4(ulin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarUI2FromUI8(i64in: u64) -> ::windows::core::Result<u16> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI2FromUI8 ( i64in : u64 , puiout : *mut u16 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUI2FromUI8(i64in, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarUI4FromBool<P0>(boolin: P0) -> ::windows::core::Result<u32>
where
    P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI4FromBool ( boolin : super::super::Foundation:: VARIANT_BOOL , pulout : *mut u32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUI4FromBool(boolin.into(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarUI4FromCy(cyin: super::Com::CY) -> ::windows::core::Result<u32> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI4FromCy ( cyin : super::Com:: CY , pulout : *mut u32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUI4FromCy(::core::mem::transmute(cyin), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarUI4FromDate(datein: f64) -> ::windows::core::Result<u32> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI4FromDate ( datein : f64 , pulout : *mut u32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUI4FromDate(datein, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarUI4FromDec(pdecin: *const super::super::Foundation::DECIMAL) -> ::windows::core::Result<u32> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI4FromDec ( pdecin : *const super::super::Foundation:: DECIMAL , pulout : *mut u32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUI4FromDec(pdecin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarUI4FromDisp<P0>(pdispin: P0, lcid: u32) -> ::windows::core::Result<u32>
where
    P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI4FromDisp ( pdispin : * mut::core::ffi::c_void , lcid : u32 , pulout : *mut u32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUI4FromDisp(pdispin.into().abi(), lcid, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarUI4FromI1<P0>(cin: P0) -> ::windows::core::Result<u32>
where
    P0: ::std::convert::Into<super::super::Foundation::CHAR>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI4FromI1 ( cin : super::super::Foundation:: CHAR , pulout : *mut u32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUI4FromI1(cin.into(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarUI4FromI2(uiin: i16) -> ::windows::core::Result<u32> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI4FromI2 ( uiin : i16 , pulout : *mut u32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUI4FromI2(uiin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarUI4FromI4(lin: i32) -> ::windows::core::Result<u32> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI4FromI4 ( lin : i32 , pulout : *mut u32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUI4FromI4(lin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarUI4FromI8(i64in: i64) -> ::windows::core::Result<u32> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI4FromI8 ( i64in : i64 , plout : *mut u32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUI4FromI8(i64in, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarUI4FromR4(fltin: f32) -> ::windows::core::Result<u32> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI4FromR4 ( fltin : f32 , pulout : *mut u32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUI4FromR4(fltin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarUI4FromR8(dblin: f64) -> ::windows::core::Result<u32> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI4FromR8 ( dblin : f64 , pulout : *mut u32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUI4FromR8(dblin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarUI4FromStr<P0>(strin: P0, lcid: u32, dwflags: u32) -> ::windows::core::Result<u32>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI4FromStr ( strin : :: windows::core::PCWSTR , lcid : u32 , dwflags : u32 , pulout : *mut u32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUI4FromStr(strin.into().abi(), lcid, dwflags, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarUI4FromUI1(bin: u8) -> ::windows::core::Result<u32> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI4FromUI1 ( bin : u8 , pulout : *mut u32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUI4FromUI1(bin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarUI4FromUI2(uiin: u16) -> ::windows::core::Result<u32> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI4FromUI2 ( uiin : u16 , pulout : *mut u32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUI4FromUI2(uiin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarUI4FromUI8(ui64in: u64) -> ::windows::core::Result<u32> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI4FromUI8 ( ui64in : u64 , plout : *mut u32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUI4FromUI8(ui64in, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarUI8FromBool<P0>(boolin: P0) -> ::windows::core::Result<u64>
where
    P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI8FromBool ( boolin : super::super::Foundation:: VARIANT_BOOL , pi64out : *mut u64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUI8FromBool(boolin.into(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarUI8FromCy(cyin: super::Com::CY) -> ::windows::core::Result<u64> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI8FromCy ( cyin : super::Com:: CY , pi64out : *mut u64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUI8FromCy(::core::mem::transmute(cyin), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarUI8FromDate(datein: f64) -> ::windows::core::Result<u64> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI8FromDate ( datein : f64 , pi64out : *mut u64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUI8FromDate(datein, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarUI8FromDec(pdecin: *const super::super::Foundation::DECIMAL) -> ::windows::core::Result<u64> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI8FromDec ( pdecin : *const super::super::Foundation:: DECIMAL , pi64out : *mut u64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUI8FromDec(pdecin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarUI8FromDisp<P0>(pdispin: P0, lcid: u32) -> ::windows::core::Result<u64>
where
    P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI8FromDisp ( pdispin : * mut::core::ffi::c_void , lcid : u32 , pi64out : *mut u64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUI8FromDisp(pdispin.into().abi(), lcid, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarUI8FromI1<P0>(cin: P0) -> ::windows::core::Result<u64>
where
    P0: ::std::convert::Into<super::super::Foundation::CHAR>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI8FromI1 ( cin : super::super::Foundation:: CHAR , pi64out : *mut u64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUI8FromI1(cin.into(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarUI8FromI2(sin: i16) -> ::windows::core::Result<u64> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI8FromI2 ( sin : i16 , pi64out : *mut u64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUI8FromI2(sin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarUI8FromI8(ui64in: i64) -> ::windows::core::Result<u64> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI8FromI8 ( ui64in : i64 , pi64out : *mut u64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUI8FromI8(ui64in, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarUI8FromR4(fltin: f32) -> ::windows::core::Result<u64> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI8FromR4 ( fltin : f32 , pi64out : *mut u64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUI8FromR4(fltin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarUI8FromR8(dblin: f64) -> ::windows::core::Result<u64> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI8FromR8 ( dblin : f64 , pi64out : *mut u64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUI8FromR8(dblin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarUI8FromStr<P0>(strin: P0, lcid: u32, dwflags: u32) -> ::windows::core::Result<u64>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI8FromStr ( strin : :: windows::core::PCWSTR , lcid : u32 , dwflags : u32 , pi64out : *mut u64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUI8FromStr(strin.into().abi(), lcid, dwflags, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarUI8FromUI1(bin: u8) -> ::windows::core::Result<u64> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI8FromUI1 ( bin : u8 , pi64out : *mut u64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUI8FromUI1(bin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarUI8FromUI2(uiin: u16) -> ::windows::core::Result<u64> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI8FromUI2 ( uiin : u16 , pi64out : *mut u64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUI8FromUI2(uiin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarUI8FromUI4(ulin: u32) -> ::windows::core::Result<u64> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUI8FromUI4 ( ulin : u32 , pi64out : *mut u64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUI8FromUI4(ulin, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VarUdateFromDate(datein: f64, dwflags: u32) -> ::windows::core::Result<UDATE> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarUdateFromDate ( datein : f64 , dwflags : u32 , pudateout : *mut UDATE ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarUdateFromDate(datein, dwflags, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VarWeekdayName(iweekday: i32, fabbrev: i32, ifirstday: i32, dwflags: u32) -> ::windows::core::Result<::windows::core::BSTR> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarWeekdayName ( iweekday : i32 , fabbrev : i32 , ifirstday : i32 , dwflags : u32 , pbstrout : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarWeekdayName(iweekday, fabbrev, ifirstday, dwflags, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn VarXor(pvarleft: *const super::Com::VARIANT, pvarright: *const super::Com::VARIANT) -> ::windows::core::Result<super::Com::VARIANT> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VarXor ( pvarleft : *const super::Com:: VARIANT , pvarright : *const super::Com:: VARIANT , pvarresult : *mut super::Com:: VARIANT ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VarXor(pvarleft, pvarright, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn VariantChangeType(pvargdest: *mut super::Com::VARIANT, pvarsrc: *const super::Com::VARIANT, wflags: u16, vt: super::Com::VARENUM) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VariantChangeType ( pvargdest : *mut super::Com:: VARIANT , pvarsrc : *const super::Com:: VARIANT , wflags : u16 , vt : super::Com:: VARENUM ) -> :: windows::core::HRESULT );
    VariantChangeType(pvargdest, pvarsrc, wflags, vt).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn VariantChangeTypeEx(pvargdest: *mut super::Com::VARIANT, pvarsrc: *const super::Com::VARIANT, lcid: u32, wflags: u16, vt: super::Com::VARENUM) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VariantChangeTypeEx ( pvargdest : *mut super::Com:: VARIANT , pvarsrc : *const super::Com:: VARIANT , lcid : u32 , wflags : u16 , vt : super::Com:: VARENUM ) -> :: windows::core::HRESULT );
    VariantChangeTypeEx(pvargdest, pvarsrc, lcid, wflags, vt).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn VariantClear(pvarg: *mut super::Com::VARIANT) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VariantClear ( pvarg : *mut super::Com:: VARIANT ) -> :: windows::core::HRESULT );
    VariantClear(pvarg).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn VariantCopy(pvargdest: *mut super::Com::VARIANT, pvargsrc: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VariantCopy ( pvargdest : *mut super::Com:: VARIANT , pvargsrc : *const super::Com:: VARIANT ) -> :: windows::core::HRESULT );
    VariantCopy(pvargdest, pvargsrc).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn VariantCopyInd(pvardest: *mut super::Com::VARIANT, pvargsrc: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VariantCopyInd ( pvardest : *mut super::Com:: VARIANT , pvargsrc : *const super::Com:: VARIANT ) -> :: windows::core::HRESULT );
    VariantCopyInd(pvardest, pvargsrc).ok()
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn VariantInit(pvarg: *mut super::Com::VARIANT) {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VariantInit ( pvarg : *mut super::Com:: VARIANT ) -> ( ) );
    VariantInit(pvarg)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[inline]
pub unsafe fn VariantTimeToDosDateTime(vtime: f64, pwdosdate: *mut u16, pwdostime: *mut u16) -> i32 {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VariantTimeToDosDateTime ( vtime : f64 , pwdosdate : *mut u16 , pwdostime : *mut u16 ) -> i32 );
    VariantTimeToDosDateTime(vtime, pwdosdate, pwdostime)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VariantTimeToSystemTime(vtime: f64, lpsystemtime: *mut super::super::Foundation::SYSTEMTIME) -> i32 {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VariantTimeToSystemTime ( vtime : f64 , lpsystemtime : *mut super::super::Foundation:: SYSTEMTIME ) -> i32 );
    VariantTimeToSystemTime(vtime, lpsystemtime)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VectorFromBstr(bstr: &::windows::core::BSTR) -> ::windows::core::Result<*mut super::Com::SAFEARRAY> {
    ::windows::core::link ! ( "oleaut32.dll""system" fn VectorFromBstr ( bstr : * mut::core::ffi::c_void , ppsa : *mut *mut super::Com:: SAFEARRAY ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    VectorFromBstr(::core::mem::transmute_copy(bstr), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAdviseSinkEx(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAdviseSinkEx {
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn OnDataChange(&self, pformatetc: *const super::Com::FORMATETC, pstgmed: *const super::Com::STGMEDIUM) {
        (::windows::core::Vtable::vtable(self).base__.OnDataChange)(::windows::core::Vtable::as_raw(self), pformatetc, pstgmed)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnViewChange(&self, dwaspect: u32, lindex: i32) {
        (::windows::core::Vtable::vtable(self).base__.OnViewChange)(::windows::core::Vtable::as_raw(self), dwaspect, lindex)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnRename<P0>(&self, pmk: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IMoniker>>,
    {
        (::windows::core::Vtable::vtable(self).base__.OnRename)(::windows::core::Vtable::as_raw(self), pmk.into().abi())
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnSave(&self) {
        (::windows::core::Vtable::vtable(self).base__.OnSave)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnClose(&self) {
        (::windows::core::Vtable::vtable(self).base__.OnClose)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn OnViewStatusChange(&self, dwviewstatus: u32) {
        (::windows::core::Vtable::vtable(self).OnViewStatusChange)(::windows::core::Vtable::as_raw(self), dwviewstatus)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IAdviseSinkEx, ::windows::core::IUnknown, super::Com::IAdviseSink);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAdviseSinkEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAdviseSinkEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAdviseSinkEx {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAdviseSinkEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAdviseSinkEx").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IAdviseSinkEx {
    type Vtable = IAdviseSinkEx_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IAdviseSinkEx {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3af24290_0c96_11ce_a0cf_00aa00600ab8);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAdviseSinkEx_Vtbl {
    pub base__: super::Com::IAdviseSink_Vtbl,
    pub OnViewStatusChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwviewstatus: u32),
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct ICanHandleException(::windows::core::IUnknown);
impl ICanHandleException {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CanHandleException(&self, pexcepinfo: *const super::Com::EXCEPINFO, pvar: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).CanHandleException)(::windows::core::Vtable::as_raw(self), pexcepinfo, pvar).ok()
    }
}
::windows::core::interface_hierarchy!(ICanHandleException, ::windows::core::IUnknown);
impl ::core::clone::Clone for ICanHandleException {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICanHandleException {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICanHandleException {}
impl ::core::fmt::Debug for ICanHandleException {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICanHandleException").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for ICanHandleException {
    type Vtable = ICanHandleException_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanHandleException {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc5598e60_b307_11d1_b27d_006008c3fbfb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanHandleException_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CanHandleException: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pexcepinfo: *const super::Com::EXCEPINFO, pvar: *const super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CanHandleException: usize,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IClassFactory2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IClassFactory2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateInstance<P0, T>(&self, punkouter: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateInstance)(::windows::core::Vtable::as_raw(self), punkouter.into().abi(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn LockServer<P0>(&self, flock: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.LockServer)(::windows::core::Vtable::as_raw(self), flock.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLicInfo(&self, plicinfo: *mut LICINFO) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetLicInfo)(::windows::core::Vtable::as_raw(self), plicinfo).ok()
    }
    pub unsafe fn RequestLicKey(&self, dwreserved: u32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RequestLicKey)(::windows::core::Vtable::as_raw(self), dwreserved, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateInstanceLic<P0, P1, T>(&self, punkouter: P0, punkreserved: P1, bstrkey: &::windows::core::BSTR) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateInstanceLic)(::windows::core::Vtable::as_raw(self), punkouter.into().abi(), punkreserved.into().abi(), &<T as ::windows::core::Interface>::IID, ::core::mem::transmute_copy(bstrkey), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IClassFactory2, ::windows::core::IUnknown, super::Com::IClassFactory);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IClassFactory2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IClassFactory2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IClassFactory2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IClassFactory2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IClassFactory2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IClassFactory2 {
    type Vtable = IClassFactory2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IClassFactory2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb196b28f_bab4_101a_b69c_00aa00341d07);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IClassFactory2_Vtbl {
    pub base__: super::Com::IClassFactory_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLicInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plicinfo: *mut LICINFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLicInfo: usize,
    pub RequestLicKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwreserved: u32, pbstrkey: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateInstanceLic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, punkreserved: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, bstrkey: *mut ::core::ffi::c_void, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IContinue(::windows::core::IUnknown);
impl IContinue {
    pub unsafe fn FContinue(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).FContinue)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IContinue, ::windows::core::IUnknown);
impl ::core::clone::Clone for IContinue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContinue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContinue {}
impl ::core::fmt::Debug for IContinue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContinue").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IContinue {
    type Vtable = IContinue_Vtbl;
}
unsafe impl ::windows::core::Interface for IContinue {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000012a_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContinue_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub FContinue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IContinueCallback(::windows::core::IUnknown);
impl IContinueCallback {
    pub unsafe fn FContinue(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).FContinue)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn FContinuePrinting<P0>(&self, ncntprinted: i32, ncurpage: i32, pwszprintstatus: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).FContinuePrinting)(::windows::core::Vtable::as_raw(self), ncntprinted, ncurpage, pwszprintstatus.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IContinueCallback, ::windows::core::IUnknown);
impl ::core::clone::Clone for IContinueCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContinueCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContinueCallback {}
impl ::core::fmt::Debug for IContinueCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContinueCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IContinueCallback {
    type Vtable = IContinueCallback_Vtbl;
}
unsafe impl ::windows::core::Interface for IContinueCallback {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb722bcca_4e68_101b_a2bc_00aa00404770);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContinueCallback_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub FContinue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FContinuePrinting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ncntprinted: i32, ncurpage: i32, pwszprintstatus: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct ICreateErrorInfo(::windows::core::IUnknown);
impl ICreateErrorInfo {
    pub unsafe fn SetGUID(&self, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetGUID)(::windows::core::Vtable::as_raw(self), rguid).ok()
    }
    pub unsafe fn SetSource<P0>(&self, szsource: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetSource)(::windows::core::Vtable::as_raw(self), szsource.into().abi()).ok()
    }
    pub unsafe fn SetDescription<P0>(&self, szdescription: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetDescription)(::windows::core::Vtable::as_raw(self), szdescription.into().abi()).ok()
    }
    pub unsafe fn SetHelpFile<P0>(&self, szhelpfile: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetHelpFile)(::windows::core::Vtable::as_raw(self), szhelpfile.into().abi()).ok()
    }
    pub unsafe fn SetHelpContext(&self, dwhelpcontext: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetHelpContext)(::windows::core::Vtable::as_raw(self), dwhelpcontext).ok()
    }
}
::windows::core::interface_hierarchy!(ICreateErrorInfo, ::windows::core::IUnknown);
impl ::core::clone::Clone for ICreateErrorInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICreateErrorInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICreateErrorInfo {}
impl ::core::fmt::Debug for ICreateErrorInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICreateErrorInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for ICreateErrorInfo {
    type Vtable = ICreateErrorInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for ICreateErrorInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x22f03340_547d_101b_8e65_08002b2bd119);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateErrorInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetGUID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szsource: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szdescription: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SetHelpFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szhelpfile: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SetHelpContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwhelpcontext: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct ICreateTypeInfo(::windows::core::IUnknown);
impl ICreateTypeInfo {
    pub unsafe fn SetGuid(&self, guid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetGuid)(::windows::core::Vtable::as_raw(self), guid).ok()
    }
    pub unsafe fn SetTypeFlags(&self, utypeflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetTypeFlags)(::windows::core::Vtable::as_raw(self), utypeflags).ok()
    }
    pub unsafe fn SetDocString<P0>(&self, pstrdoc: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetDocString)(::windows::core::Vtable::as_raw(self), pstrdoc.into().abi()).ok()
    }
    pub unsafe fn SetHelpContext(&self, dwhelpcontext: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetHelpContext)(::windows::core::Vtable::as_raw(self), dwhelpcontext).ok()
    }
    pub unsafe fn SetVersion(&self, wmajorvernum: u16, wminorvernum: u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetVersion)(::windows::core::Vtable::as_raw(self), wmajorvernum, wminorvernum).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddRefTypeInfo<P0>(&self, ptinfo: P0, phreftype: *const u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::ITypeInfo>>,
    {
        (::windows::core::Vtable::vtable(self).AddRefTypeInfo)(::windows::core::Vtable::as_raw(self), ptinfo.into().abi(), phreftype).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn AddFuncDesc(&self, index: u32, pfuncdesc: *const super::Com::FUNCDESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).AddFuncDesc)(::windows::core::Vtable::as_raw(self), index, pfuncdesc).ok()
    }
    pub unsafe fn AddImplType(&self, index: u32, hreftype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).AddImplType)(::windows::core::Vtable::as_raw(self), index, hreftype).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetImplTypeFlags(&self, index: u32, impltypeflags: super::Com::IMPLTYPEFLAGS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetImplTypeFlags)(::windows::core::Vtable::as_raw(self), index, impltypeflags).ok()
    }
    pub unsafe fn SetAlignment(&self, cbalignment: u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetAlignment)(::windows::core::Vtable::as_raw(self), cbalignment).ok()
    }
    pub unsafe fn SetSchema<P0>(&self, pstrschema: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetSchema)(::windows::core::Vtable::as_raw(self), pstrschema.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn AddVarDesc(&self, index: u32, pvardesc: *const super::Com::VARDESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).AddVarDesc)(::windows::core::Vtable::as_raw(self), index, pvardesc).ok()
    }
    pub unsafe fn SetFuncAndParamNames(&self, index: u32, rgsznames: &[::windows::core::PCWSTR]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetFuncAndParamNames)(::windows::core::Vtable::as_raw(self), index, ::core::mem::transmute(rgsznames.as_ptr()), rgsznames.len() as _).ok()
    }
    pub unsafe fn SetVarName<P0>(&self, index: u32, szname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetVarName)(::windows::core::Vtable::as_raw(self), index, szname.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetTypeDescAlias(&self, ptdescalias: *const super::Com::TYPEDESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetTypeDescAlias)(::windows::core::Vtable::as_raw(self), ptdescalias).ok()
    }
    pub unsafe fn DefineFuncAsDllEntry<P0, P1>(&self, index: u32, szdllname: P0, szprocname: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).DefineFuncAsDllEntry)(::windows::core::Vtable::as_raw(self), index, szdllname.into().abi(), szprocname.into().abi()).ok()
    }
    pub unsafe fn SetFuncDocString<P0>(&self, index: u32, szdocstring: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetFuncDocString)(::windows::core::Vtable::as_raw(self), index, szdocstring.into().abi()).ok()
    }
    pub unsafe fn SetVarDocString<P0>(&self, index: u32, szdocstring: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetVarDocString)(::windows::core::Vtable::as_raw(self), index, szdocstring.into().abi()).ok()
    }
    pub unsafe fn SetFuncHelpContext(&self, index: u32, dwhelpcontext: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetFuncHelpContext)(::windows::core::Vtable::as_raw(self), index, dwhelpcontext).ok()
    }
    pub unsafe fn SetVarHelpContext(&self, index: u32, dwhelpcontext: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetVarHelpContext)(::windows::core::Vtable::as_raw(self), index, dwhelpcontext).ok()
    }
    pub unsafe fn SetMops(&self, index: u32, bstrmops: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetMops)(::windows::core::Vtable::as_raw(self), index, ::core::mem::transmute_copy(bstrmops)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetTypeIdldesc(&self, pidldesc: *const super::Com::IDLDESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetTypeIdldesc)(::windows::core::Vtable::as_raw(self), pidldesc).ok()
    }
    pub unsafe fn LayOut(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).LayOut)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(ICreateTypeInfo, ::windows::core::IUnknown);
impl ::core::clone::Clone for ICreateTypeInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICreateTypeInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICreateTypeInfo {}
impl ::core::fmt::Debug for ICreateTypeInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICreateTypeInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for ICreateTypeInfo {
    type Vtable = ICreateTypeInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for ICreateTypeInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00020405_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateTypeInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetTypeFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, utypeflags: u32) -> ::windows::core::HRESULT,
    pub SetDocString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstrdoc: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SetHelpContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwhelpcontext: u32) -> ::windows::core::HRESULT,
    pub SetVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wmajorvernum: u16, wminorvernum: u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddRefTypeInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptinfo: *mut ::core::ffi::c_void, phreftype: *const u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddRefTypeInfo: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub AddFuncDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, pfuncdesc: *const super::Com::FUNCDESC) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    AddFuncDesc: usize,
    pub AddImplType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, hreftype: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SetImplTypeFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, impltypeflags: super::Com::IMPLTYPEFLAGS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetImplTypeFlags: usize,
    pub SetAlignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbalignment: u16) -> ::windows::core::HRESULT,
    pub SetSchema: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstrschema: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub AddVarDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, pvardesc: *const super::Com::VARDESC) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    AddVarDesc: usize,
    pub SetFuncAndParamNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, rgsznames: *const ::windows::core::PCWSTR, cnames: u32) -> ::windows::core::HRESULT,
    pub SetVarName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, szname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SetTypeDescAlias: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptdescalias: *const super::Com::TYPEDESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetTypeDescAlias: usize,
    pub DefineFuncAsDllEntry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, szdllname: ::windows::core::PCWSTR, szprocname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SetFuncDocString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, szdocstring: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SetVarDocString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, szdocstring: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SetFuncHelpContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, dwhelpcontext: u32) -> ::windows::core::HRESULT,
    pub SetVarHelpContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, dwhelpcontext: u32) -> ::windows::core::HRESULT,
    pub SetMops: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, bstrmops: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SetTypeIdldesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidldesc: *const super::Com::IDLDESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetTypeIdldesc: usize,
    pub LayOut: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct ICreateTypeInfo2(::windows::core::IUnknown);
impl ICreateTypeInfo2 {
    pub unsafe fn SetGuid(&self, guid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetGuid)(::windows::core::Vtable::as_raw(self), guid).ok()
    }
    pub unsafe fn SetTypeFlags(&self, utypeflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetTypeFlags)(::windows::core::Vtable::as_raw(self), utypeflags).ok()
    }
    pub unsafe fn SetDocString<P0>(&self, pstrdoc: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetDocString)(::windows::core::Vtable::as_raw(self), pstrdoc.into().abi()).ok()
    }
    pub unsafe fn SetHelpContext(&self, dwhelpcontext: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetHelpContext)(::windows::core::Vtable::as_raw(self), dwhelpcontext).ok()
    }
    pub unsafe fn SetVersion(&self, wmajorvernum: u16, wminorvernum: u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetVersion)(::windows::core::Vtable::as_raw(self), wmajorvernum, wminorvernum).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddRefTypeInfo<P0>(&self, ptinfo: P0, phreftype: *const u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::ITypeInfo>>,
    {
        (::windows::core::Vtable::vtable(self).base__.AddRefTypeInfo)(::windows::core::Vtable::as_raw(self), ptinfo.into().abi(), phreftype).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn AddFuncDesc(&self, index: u32, pfuncdesc: *const super::Com::FUNCDESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.AddFuncDesc)(::windows::core::Vtable::as_raw(self), index, pfuncdesc).ok()
    }
    pub unsafe fn AddImplType(&self, index: u32, hreftype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.AddImplType)(::windows::core::Vtable::as_raw(self), index, hreftype).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetImplTypeFlags(&self, index: u32, impltypeflags: super::Com::IMPLTYPEFLAGS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetImplTypeFlags)(::windows::core::Vtable::as_raw(self), index, impltypeflags).ok()
    }
    pub unsafe fn SetAlignment(&self, cbalignment: u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetAlignment)(::windows::core::Vtable::as_raw(self), cbalignment).ok()
    }
    pub unsafe fn SetSchema<P0>(&self, pstrschema: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetSchema)(::windows::core::Vtable::as_raw(self), pstrschema.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn AddVarDesc(&self, index: u32, pvardesc: *const super::Com::VARDESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.AddVarDesc)(::windows::core::Vtable::as_raw(self), index, pvardesc).ok()
    }
    pub unsafe fn SetFuncAndParamNames(&self, index: u32, rgsznames: &[::windows::core::PCWSTR]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetFuncAndParamNames)(::windows::core::Vtable::as_raw(self), index, ::core::mem::transmute(rgsznames.as_ptr()), rgsznames.len() as _).ok()
    }
    pub unsafe fn SetVarName<P0>(&self, index: u32, szname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetVarName)(::windows::core::Vtable::as_raw(self), index, szname.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetTypeDescAlias(&self, ptdescalias: *const super::Com::TYPEDESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetTypeDescAlias)(::windows::core::Vtable::as_raw(self), ptdescalias).ok()
    }
    pub unsafe fn DefineFuncAsDllEntry<P0, P1>(&self, index: u32, szdllname: P0, szprocname: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DefineFuncAsDllEntry)(::windows::core::Vtable::as_raw(self), index, szdllname.into().abi(), szprocname.into().abi()).ok()
    }
    pub unsafe fn SetFuncDocString<P0>(&self, index: u32, szdocstring: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetFuncDocString)(::windows::core::Vtable::as_raw(self), index, szdocstring.into().abi()).ok()
    }
    pub unsafe fn SetVarDocString<P0>(&self, index: u32, szdocstring: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetVarDocString)(::windows::core::Vtable::as_raw(self), index, szdocstring.into().abi()).ok()
    }
    pub unsafe fn SetFuncHelpContext(&self, index: u32, dwhelpcontext: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetFuncHelpContext)(::windows::core::Vtable::as_raw(self), index, dwhelpcontext).ok()
    }
    pub unsafe fn SetVarHelpContext(&self, index: u32, dwhelpcontext: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetVarHelpContext)(::windows::core::Vtable::as_raw(self), index, dwhelpcontext).ok()
    }
    pub unsafe fn SetMops(&self, index: u32, bstrmops: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetMops)(::windows::core::Vtable::as_raw(self), index, ::core::mem::transmute_copy(bstrmops)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetTypeIdldesc(&self, pidldesc: *const super::Com::IDLDESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetTypeIdldesc)(::windows::core::Vtable::as_raw(self), pidldesc).ok()
    }
    pub unsafe fn LayOut(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.LayOut)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn DeleteFuncDesc(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DeleteFuncDesc)(::windows::core::Vtable::as_raw(self), index).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DeleteFuncDescByMemId(&self, memid: i32, invkind: super::Com::INVOKEKIND) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DeleteFuncDescByMemId)(::windows::core::Vtable::as_raw(self), memid, invkind).ok()
    }
    pub unsafe fn DeleteVarDesc(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DeleteVarDesc)(::windows::core::Vtable::as_raw(self), index).ok()
    }
    pub unsafe fn DeleteVarDescByMemId(&self, memid: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DeleteVarDescByMemId)(::windows::core::Vtable::as_raw(self), memid).ok()
    }
    pub unsafe fn DeleteImplType(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DeleteImplType)(::windows::core::Vtable::as_raw(self), index).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetCustData(&self, guid: *const ::windows::core::GUID, pvarval: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetCustData)(::windows::core::Vtable::as_raw(self), guid, pvarval).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetFuncCustData(&self, index: u32, guid: *const ::windows::core::GUID, pvarval: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetFuncCustData)(::windows::core::Vtable::as_raw(self), index, guid, pvarval).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetParamCustData(&self, indexfunc: u32, indexparam: u32, guid: *const ::windows::core::GUID, pvarval: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetParamCustData)(::windows::core::Vtable::as_raw(self), indexfunc, indexparam, guid, pvarval).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetVarCustData(&self, index: u32, guid: *const ::windows::core::GUID, pvarval: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetVarCustData)(::windows::core::Vtable::as_raw(self), index, guid, pvarval).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetImplTypeCustData(&self, index: u32, guid: *const ::windows::core::GUID, pvarval: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetImplTypeCustData)(::windows::core::Vtable::as_raw(self), index, guid, pvarval).ok()
    }
    pub unsafe fn SetHelpStringContext(&self, dwhelpstringcontext: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetHelpStringContext)(::windows::core::Vtable::as_raw(self), dwhelpstringcontext).ok()
    }
    pub unsafe fn SetFuncHelpStringContext(&self, index: u32, dwhelpstringcontext: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetFuncHelpStringContext)(::windows::core::Vtable::as_raw(self), index, dwhelpstringcontext).ok()
    }
    pub unsafe fn SetVarHelpStringContext(&self, index: u32, dwhelpstringcontext: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetVarHelpStringContext)(::windows::core::Vtable::as_raw(self), index, dwhelpstringcontext).ok()
    }
    pub unsafe fn Invalidate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Invalidate)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetName<P0>(&self, szname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetName)(::windows::core::Vtable::as_raw(self), szname.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(ICreateTypeInfo2, ::windows::core::IUnknown, ICreateTypeInfo);
impl ::core::clone::Clone for ICreateTypeInfo2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICreateTypeInfo2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICreateTypeInfo2 {}
impl ::core::fmt::Debug for ICreateTypeInfo2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICreateTypeInfo2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for ICreateTypeInfo2 {
    type Vtable = ICreateTypeInfo2_Vtbl;
}
unsafe impl ::windows::core::Interface for ICreateTypeInfo2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0002040e_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateTypeInfo2_Vtbl {
    pub base__: ICreateTypeInfo_Vtbl,
    pub DeleteFuncDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub DeleteFuncDescByMemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, memid: i32, invkind: super::Com::INVOKEKIND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DeleteFuncDescByMemId: usize,
    pub DeleteVarDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
    pub DeleteVarDescByMemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, memid: i32) -> ::windows::core::HRESULT,
    pub DeleteImplType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub SetCustData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pvarval: *const super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    SetCustData: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub SetFuncCustData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, guid: *const ::windows::core::GUID, pvarval: *const super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    SetFuncCustData: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub SetParamCustData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, indexfunc: u32, indexparam: u32, guid: *const ::windows::core::GUID, pvarval: *const super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    SetParamCustData: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub SetVarCustData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, guid: *const ::windows::core::GUID, pvarval: *const super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    SetVarCustData: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub SetImplTypeCustData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, guid: *const ::windows::core::GUID, pvarval: *const super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    SetImplTypeCustData: usize,
    pub SetHelpStringContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwhelpstringcontext: u32) -> ::windows::core::HRESULT,
    pub SetFuncHelpStringContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, dwhelpstringcontext: u32) -> ::windows::core::HRESULT,
    pub SetVarHelpStringContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, dwhelpstringcontext: u32) -> ::windows::core::HRESULT,
    pub Invalidate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct ICreateTypeLib(::windows::core::IUnknown);
impl ICreateTypeLib {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateTypeInfo<P0>(&self, szname: P0, tkind: super::Com::TYPEKIND) -> ::windows::core::Result<ICreateTypeInfo>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateTypeInfo)(::windows::core::Vtable::as_raw(self), szname.into().abi(), tkind, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, szname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetName)(::windows::core::Vtable::as_raw(self), szname.into().abi()).ok()
    }
    pub unsafe fn SetVersion(&self, wmajorvernum: u16, wminorvernum: u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetVersion)(::windows::core::Vtable::as_raw(self), wmajorvernum, wminorvernum).ok()
    }
    pub unsafe fn SetGuid(&self, guid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetGuid)(::windows::core::Vtable::as_raw(self), guid).ok()
    }
    pub unsafe fn SetDocString<P0>(&self, szdoc: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetDocString)(::windows::core::Vtable::as_raw(self), szdoc.into().abi()).ok()
    }
    pub unsafe fn SetHelpFileName<P0>(&self, szhelpfilename: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetHelpFileName)(::windows::core::Vtable::as_raw(self), szhelpfilename.into().abi()).ok()
    }
    pub unsafe fn SetHelpContext(&self, dwhelpcontext: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetHelpContext)(::windows::core::Vtable::as_raw(self), dwhelpcontext).ok()
    }
    pub unsafe fn SetLcid(&self, lcid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetLcid)(::windows::core::Vtable::as_raw(self), lcid).ok()
    }
    pub unsafe fn SetLibFlags(&self, ulibflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetLibFlags)(::windows::core::Vtable::as_raw(self), ulibflags).ok()
    }
    pub unsafe fn SaveAllChanges(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SaveAllChanges)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(ICreateTypeLib, ::windows::core::IUnknown);
impl ::core::clone::Clone for ICreateTypeLib {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICreateTypeLib {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICreateTypeLib {}
impl ::core::fmt::Debug for ICreateTypeLib {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICreateTypeLib").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for ICreateTypeLib {
    type Vtable = ICreateTypeLib_Vtbl;
}
unsafe impl ::windows::core::Interface for ICreateTypeLib {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00020406_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateTypeLib_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateTypeInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szname: ::windows::core::PCWSTR, tkind: super::Com::TYPEKIND, ppctinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateTypeInfo: usize,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SetVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wmajorvernum: u16, wminorvernum: u16) -> ::windows::core::HRESULT,
    pub SetGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetDocString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szdoc: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SetHelpFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szhelpfilename: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SetHelpContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwhelpcontext: u32) -> ::windows::core::HRESULT,
    pub SetLcid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcid: u32) -> ::windows::core::HRESULT,
    pub SetLibFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulibflags: u32) -> ::windows::core::HRESULT,
    pub SaveAllChanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct ICreateTypeLib2(::windows::core::IUnknown);
impl ICreateTypeLib2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateTypeInfo<P0>(&self, szname: P0, tkind: super::Com::TYPEKIND) -> ::windows::core::Result<ICreateTypeInfo>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateTypeInfo)(::windows::core::Vtable::as_raw(self), szname.into().abi(), tkind, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, szname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetName)(::windows::core::Vtable::as_raw(self), szname.into().abi()).ok()
    }
    pub unsafe fn SetVersion(&self, wmajorvernum: u16, wminorvernum: u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetVersion)(::windows::core::Vtable::as_raw(self), wmajorvernum, wminorvernum).ok()
    }
    pub unsafe fn SetGuid(&self, guid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetGuid)(::windows::core::Vtable::as_raw(self), guid).ok()
    }
    pub unsafe fn SetDocString<P0>(&self, szdoc: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetDocString)(::windows::core::Vtable::as_raw(self), szdoc.into().abi()).ok()
    }
    pub unsafe fn SetHelpFileName<P0>(&self, szhelpfilename: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetHelpFileName)(::windows::core::Vtable::as_raw(self), szhelpfilename.into().abi()).ok()
    }
    pub unsafe fn SetHelpContext(&self, dwhelpcontext: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetHelpContext)(::windows::core::Vtable::as_raw(self), dwhelpcontext).ok()
    }
    pub unsafe fn SetLcid(&self, lcid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetLcid)(::windows::core::Vtable::as_raw(self), lcid).ok()
    }
    pub unsafe fn SetLibFlags(&self, ulibflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetLibFlags)(::windows::core::Vtable::as_raw(self), ulibflags).ok()
    }
    pub unsafe fn SaveAllChanges(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SaveAllChanges)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn DeleteTypeInfo<P0>(&self, szname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).DeleteTypeInfo)(::windows::core::Vtable::as_raw(self), szname.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetCustData(&self, guid: *const ::windows::core::GUID, pvarval: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetCustData)(::windows::core::Vtable::as_raw(self), guid, pvarval).ok()
    }
    pub unsafe fn SetHelpStringContext(&self, dwhelpstringcontext: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetHelpStringContext)(::windows::core::Vtable::as_raw(self), dwhelpstringcontext).ok()
    }
    pub unsafe fn SetHelpStringDll<P0>(&self, szfilename: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetHelpStringDll)(::windows::core::Vtable::as_raw(self), szfilename.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(ICreateTypeLib2, ::windows::core::IUnknown, ICreateTypeLib);
impl ::core::clone::Clone for ICreateTypeLib2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICreateTypeLib2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICreateTypeLib2 {}
impl ::core::fmt::Debug for ICreateTypeLib2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICreateTypeLib2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for ICreateTypeLib2 {
    type Vtable = ICreateTypeLib2_Vtbl;
}
unsafe impl ::windows::core::Interface for ICreateTypeLib2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0002040f_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateTypeLib2_Vtbl {
    pub base__: ICreateTypeLib_Vtbl,
    pub DeleteTypeInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub SetCustData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pvarval: *const super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    SetCustData: usize,
    pub SetHelpStringContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwhelpstringcontext: u32) -> ::windows::core::HRESULT,
    pub SetHelpStringDll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szfilename: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IDispError(::windows::core::IUnknown);
impl IDispError {
    pub unsafe fn QueryErrorInfo(&self, guiderrortype: ::windows::core::GUID) -> ::windows::core::Result<IDispError> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).QueryErrorInfo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(guiderrortype), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetNext(&self) -> ::windows::core::Result<IDispError> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetNext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetHresult(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetHresult)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSource(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSource)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetHelpInfo(&self, pbstrfilename: *mut ::windows::core::BSTR, pdwcontext: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetHelpInfo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrfilename), pdwcontext).ok()
    }
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDescription)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IDispError, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDispError {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDispError {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDispError {}
impl ::core::fmt::Debug for IDispError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDispError").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDispError {
    type Vtable = IDispError_Vtbl;
}
unsafe impl ::windows::core::Interface for IDispError {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6ef9861_c720_11d0_9337_00a0c90dcaa9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispError_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub QueryErrorInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guiderrortype: ::windows::core::GUID, ppde: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppde: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetHresult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phr: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub GetSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetHelpInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrfilename: *mut *mut ::core::ffi::c_void, pdwcontext: *mut u32) -> ::windows::core::HRESULT,
    pub GetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdescription: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IDispatchEx(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IDispatchEx {
    pub unsafe fn GetDispID(&self, bstrname: &::windows::core::BSTR, grfdex: u32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDispID)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), grfdex, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn InvokeEx<P0>(&self, id: i32, lcid: u32, wflags: u16, pdp: *const super::Com::DISPPARAMS, pvarres: ::core::option::Option<*mut super::Com::VARIANT>, pei: ::core::option::Option<*mut super::Com::EXCEPINFO>, pspcaller: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IServiceProvider>>,
    {
        (::windows::core::Vtable::vtable(self).InvokeEx)(::windows::core::Vtable::as_raw(self), id, lcid, wflags, pdp, ::core::mem::transmute(pvarres.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pei.unwrap_or(::std::ptr::null_mut())), pspcaller.into().abi()).ok()
    }
    pub unsafe fn DeleteMemberByName(&self, bstrname: &::windows::core::BSTR, grfdex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DeleteMemberByName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname), grfdex).ok()
    }
    pub unsafe fn DeleteMemberByDispID(&self, id: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DeleteMemberByDispID)(::windows::core::Vtable::as_raw(self), id).ok()
    }
    pub unsafe fn GetMemberProperties(&self, id: i32, grfdexfetch: u32) -> ::windows::core::Result<FDEX_PROP_FLAGS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetMemberProperties)(::windows::core::Vtable::as_raw(self), id, grfdexfetch, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetMemberName(&self, id: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetMemberName)(::windows::core::Vtable::as_raw(self), id, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetNextDispID(&self, grfdex: u32, id: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetNextDispID)(::windows::core::Vtable::as_raw(self), grfdex, id, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetNameSpaceParent(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetNameSpaceParent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IDispatchEx, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IDispatchEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IDispatchEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IDispatchEx {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IDispatchEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDispatchEx").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IDispatchEx {
    type Vtable = IDispatchEx_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IDispatchEx {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6ef9860_c720_11d0_9337_00a0c90dcaa9);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDispatchEx_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub GetDispID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: *mut ::core::ffi::c_void, grfdex: u32, pid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub InvokeEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: i32, lcid: u32, wflags: u16, pdp: *const super::Com::DISPPARAMS, pvarres: *mut super::Com::VARIANT, pei: *mut super::Com::EXCEPINFO, pspcaller: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    InvokeEx: usize,
    pub DeleteMemberByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: *mut ::core::ffi::c_void, grfdex: u32) -> ::windows::core::HRESULT,
    pub DeleteMemberByDispID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: i32) -> ::windows::core::HRESULT,
    pub GetMemberProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: i32, grfdexfetch: u32, pgrfdex: *mut FDEX_PROP_FLAGS) -> ::windows::core::HRESULT,
    pub GetMemberName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: i32, pbstrname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetNextDispID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, grfdex: u32, id: i32, pid: *mut i32) -> ::windows::core::HRESULT,
    pub GetNameSpaceParent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IDropSource(::windows::core::IUnknown);
impl IDropSource {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_SystemServices\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub unsafe fn QueryContinueDrag<P0>(&self, fescapepressed: P0, grfkeystate: super::SystemServices::MODIFIERKEYS_FLAGS) -> ::windows::core::HRESULT
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).QueryContinueDrag)(::windows::core::Vtable::as_raw(self), fescapepressed.into(), grfkeystate)
    }
    pub unsafe fn GiveFeedback(&self, dweffect: DROPEFFECT) -> ::windows::core::HRESULT {
        (::windows::core::Vtable::vtable(self).GiveFeedback)(::windows::core::Vtable::as_raw(self), dweffect)
    }
}
::windows::core::interface_hierarchy!(IDropSource, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDropSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDropSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDropSource {}
impl ::core::fmt::Debug for IDropSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDropSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDropSource {
    type Vtable = IDropSource_Vtbl;
}
unsafe impl ::windows::core::Interface for IDropSource {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000121_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDropSource_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub QueryContinueDrag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fescapepressed: super::super::Foundation::BOOL, grfkeystate: super::SystemServices::MODIFIERKEYS_FLAGS) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices")))]
    QueryContinueDrag: usize,
    pub GiveFeedback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dweffect: DROPEFFECT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IDropSourceNotify(::windows::core::IUnknown);
impl IDropSourceNotify {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DragEnterTarget<P0>(&self, hwndtarget: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).DragEnterTarget)(::windows::core::Vtable::as_raw(self), hwndtarget.into()).ok()
    }
    pub unsafe fn DragLeaveTarget(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DragLeaveTarget)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IDropSourceNotify, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDropSourceNotify {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDropSourceNotify {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDropSourceNotify {}
impl ::core::fmt::Debug for IDropSourceNotify {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDropSourceNotify").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDropSourceNotify {
    type Vtable = IDropSourceNotify_Vtbl;
}
unsafe impl ::windows::core::Interface for IDropSourceNotify {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000012b_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDropSourceNotify_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub DragEnterTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndtarget: super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DragEnterTarget: usize,
    pub DragLeaveTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IDropTarget(::windows::core::IUnknown);
impl IDropTarget {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_SystemServices\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_SystemServices"))]
    pub unsafe fn DragEnter<P0>(&self, pdataobj: P0, grfkeystate: super::SystemServices::MODIFIERKEYS_FLAGS, pt: super::super::Foundation::POINTL, pdweffect: *mut DROPEFFECT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDataObject>>,
    {
        (::windows::core::Vtable::vtable(self).DragEnter)(::windows::core::Vtable::as_raw(self), pdataobj.into().abi(), grfkeystate, ::core::mem::transmute(pt), pdweffect).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_SystemServices\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub unsafe fn DragOver(&self, grfkeystate: super::SystemServices::MODIFIERKEYS_FLAGS, pt: super::super::Foundation::POINTL, pdweffect: *mut DROPEFFECT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DragOver)(::windows::core::Vtable::as_raw(self), grfkeystate, ::core::mem::transmute(pt), pdweffect).ok()
    }
    pub unsafe fn DragLeave(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DragLeave)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_SystemServices\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_SystemServices"))]
    pub unsafe fn Drop<P0>(&self, pdataobj: P0, grfkeystate: super::SystemServices::MODIFIERKEYS_FLAGS, pt: super::super::Foundation::POINTL, pdweffect: *mut DROPEFFECT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDataObject>>,
    {
        (::windows::core::Vtable::vtable(self).Drop)(::windows::core::Vtable::as_raw(self), pdataobj.into().abi(), grfkeystate, ::core::mem::transmute(pt), pdweffect).ok()
    }
}
::windows::core::interface_hierarchy!(IDropTarget, ::windows::core::IUnknown);
impl ::core::clone::Clone for IDropTarget {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDropTarget {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDropTarget {}
impl ::core::fmt::Debug for IDropTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDropTarget").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IDropTarget {
    type Vtable = IDropTarget_Vtbl;
}
unsafe impl ::windows::core::Interface for IDropTarget {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000122_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDropTarget_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_SystemServices"))]
    pub DragEnter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdataobj: *mut ::core::ffi::c_void, grfkeystate: super::SystemServices::MODIFIERKEYS_FLAGS, pt: super::super::Foundation::POINTL, pdweffect: *mut DROPEFFECT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_SystemServices")))]
    DragEnter: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub DragOver: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, grfkeystate: super::SystemServices::MODIFIERKEYS_FLAGS, pt: super::super::Foundation::POINTL, pdweffect: *mut DROPEFFECT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices")))]
    DragOver: usize,
    pub DragLeave: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_SystemServices"))]
    pub Drop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdataobj: *mut ::core::ffi::c_void, grfkeystate: super::SystemServices::MODIFIERKEYS_FLAGS, pt: super::super::Foundation::POINTL, pdweffect: *mut DROPEFFECT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_SystemServices")))]
    Drop: usize,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IEnterpriseDropTarget(::windows::core::IUnknown);
impl IEnterpriseDropTarget {
    pub unsafe fn SetDropSourceEnterpriseId<P0>(&self, identity: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetDropSourceEnterpriseId)(::windows::core::Vtable::as_raw(self), identity.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsEvaluatingEdpPolicy(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).IsEvaluatingEdpPolicy)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IEnterpriseDropTarget, ::windows::core::IUnknown);
impl ::core::clone::Clone for IEnterpriseDropTarget {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnterpriseDropTarget {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnterpriseDropTarget {}
impl ::core::fmt::Debug for IEnterpriseDropTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnterpriseDropTarget").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IEnterpriseDropTarget {
    type Vtable = IEnterpriseDropTarget_Vtbl;
}
unsafe impl ::windows::core::Interface for IEnterpriseDropTarget {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x390e3878_fd55_4e18_819d_4682081c0cfd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnterpriseDropTarget_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetDropSourceEnterpriseId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identity: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsEvaluatingEdpPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsEvaluatingEdpPolicy: usize,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IEnumOLEVERB(::windows::core::IUnknown);
impl IEnumOLEVERB {
    #[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn Next(&self, rgelt: &mut [OLEVERB], pceltfetched: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Next)(::windows::core::Vtable::as_raw(self), rgelt.len() as _, ::core::mem::transmute(rgelt.as_ptr()), ::core::mem::transmute(pceltfetched.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Skip)(::windows::core::Vtable::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumOLEVERB> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IEnumOLEVERB, ::windows::core::IUnknown);
impl ::core::clone::Clone for IEnumOLEVERB {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumOLEVERB {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumOLEVERB {}
impl ::core::fmt::Debug for IEnumOLEVERB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumOLEVERB").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IEnumOLEVERB {
    type Vtable = IEnumOLEVERB_Vtbl;
}
unsafe impl ::windows::core::Interface for IEnumOLEVERB {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000104_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumOLEVERB_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut OLEVERB, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IEnumOleDocumentViews(::windows::core::IUnknown);
impl IEnumOleDocumentViews {
    pub unsafe fn Next(&self, cviews: u32, rgpview: *mut ::core::option::Option<IOleDocumentView>, pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Next)(::windows::core::Vtable::as_raw(self), cviews, ::core::mem::transmute(rgpview), pcfetched).ok()
    }
    pub unsafe fn Skip(&self, cviews: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Skip)(::windows::core::Vtable::as_raw(self), cviews).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumOleDocumentViews> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IEnumOleDocumentViews, ::windows::core::IUnknown);
impl ::core::clone::Clone for IEnumOleDocumentViews {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumOleDocumentViews {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumOleDocumentViews {}
impl ::core::fmt::Debug for IEnumOleDocumentViews {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumOleDocumentViews").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IEnumOleDocumentViews {
    type Vtable = IEnumOleDocumentViews_Vtbl;
}
unsafe impl ::windows::core::Interface for IEnumOleDocumentViews {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb722bcc8_4e68_101b_a2bc_00aa00404770);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumOleDocumentViews_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cviews: u32, rgpview: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cviews: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IEnumOleUndoUnits(::windows::core::IUnknown);
impl IEnumOleUndoUnits {
    pub unsafe fn Next(&self, rgelt: &mut [::core::option::Option<IOleUndoUnit>], pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Next)(::windows::core::Vtable::as_raw(self), rgelt.len() as _, ::core::mem::transmute(rgelt.as_ptr()), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Skip)(::windows::core::Vtable::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumOleUndoUnits> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IEnumOleUndoUnits, ::windows::core::IUnknown);
impl ::core::clone::Clone for IEnumOleUndoUnits {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumOleUndoUnits {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumOleUndoUnits {}
impl ::core::fmt::Debug for IEnumOleUndoUnits {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumOleUndoUnits").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IEnumOleUndoUnits {
    type Vtable = IEnumOleUndoUnits_Vtbl;
}
unsafe impl ::windows::core::Interface for IEnumOleUndoUnits {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3e7c340_ef97_11ce_9bc9_00aa00608e01);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumOleUndoUnits_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IEnumVARIANT(::windows::core::IUnknown);
impl IEnumVARIANT {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Next(&self, rgvar: &mut [super::Com::VARIANT], pceltfetched: *mut u32) -> ::windows::core::HRESULT {
        (::windows::core::Vtable::vtable(self).Next)(::windows::core::Vtable::as_raw(self), rgvar.len() as _, ::core::mem::transmute(rgvar.as_ptr()), pceltfetched)
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::HRESULT {
        (::windows::core::Vtable::vtable(self).Skip)(::windows::core::Vtable::as_raw(self), celt)
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumVARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IEnumVARIANT, ::windows::core::IUnknown);
impl ::core::clone::Clone for IEnumVARIANT {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumVARIANT {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumVARIANT {}
impl ::core::fmt::Debug for IEnumVARIANT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumVARIANT").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IEnumVARIANT {
    type Vtable = IEnumVARIANT_Vtbl;
}
unsafe impl ::windows::core::Interface for IEnumVARIANT {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00020404_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumVARIANT_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgvar: *mut super::Com::VARIANT, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IFont(::windows::core::IUnknown);
impl IFont {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetName(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(name)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Size(&self) -> ::windows::core::Result<super::Com::CY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Size)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSize(&self, size: super::Com::CY) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetSize)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(size)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Bold(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Bold)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBold<P0>(&self, bold: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetBold)(::windows::core::Vtable::as_raw(self), bold.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Italic(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Italic)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetItalic<P0>(&self, italic: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetItalic)(::windows::core::Vtable::as_raw(self), italic.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Underline(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Underline)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUnderline<P0>(&self, underline: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetUnderline)(::windows::core::Vtable::as_raw(self), underline.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Strikethrough(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Strikethrough)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStrikethrough<P0>(&self, strikethrough: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetStrikethrough)(::windows::core::Vtable::as_raw(self), strikethrough.into()).ok()
    }
    pub unsafe fn Weight(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Weight)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetWeight(&self, weight: i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetWeight)(::windows::core::Vtable::as_raw(self), weight).ok()
    }
    pub unsafe fn Charset(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Charset)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetCharset(&self, charset: i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetCharset)(::windows::core::Vtable::as_raw(self), charset).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn hFont(&self) -> ::windows::core::Result<super::super::Graphics::Gdi::HFONT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).hFont)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IFont> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn IsEqual<P0>(&self, pfontother: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IFont>>,
    {
        (::windows::core::Vtable::vtable(self).IsEqual)(::windows::core::Vtable::as_raw(self), pfontother.into().abi()).ok()
    }
    pub unsafe fn SetRatio(&self, cylogical: i32, cyhimetric: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetRatio)(::windows::core::Vtable::as_raw(self), cylogical, cyhimetric).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn QueryTextMetrics(&self) -> ::windows::core::Result<super::super::Graphics::Gdi::TEXTMETRICW> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).QueryTextMetrics)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn AddRefHfont<P0>(&self, hfont: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Graphics::Gdi::HFONT>,
    {
        (::windows::core::Vtable::vtable(self).AddRefHfont)(::windows::core::Vtable::as_raw(self), hfont.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn ReleaseHfont<P0>(&self, hfont: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Graphics::Gdi::HFONT>,
    {
        (::windows::core::Vtable::vtable(self).ReleaseHfont)(::windows::core::Vtable::as_raw(self), hfont.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn SetHdc<P0>(&self, hdc: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
    {
        (::windows::core::Vtable::vtable(self).SetHdc)(::windows::core::Vtable::as_raw(self), hdc.into()).ok()
    }
}
::windows::core::interface_hierarchy!(IFont, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFont {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFont {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFont {}
impl ::core::fmt::Debug for IFont {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFont").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IFont {
    type Vtable = IFont_Vtbl;
}
unsafe impl ::windows::core::Interface for IFont {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbef6e002_a874_101a_8bba_00aa00300cab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFont_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psize: *mut super::Com::CY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Size: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, size: super::Com::CY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSize: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Bold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbold: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Bold: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bold: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBold: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Italic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pitalic: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Italic: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetItalic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, italic: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetItalic: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Underline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punderline: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Underline: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetUnderline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, underline: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetUnderline: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Strikethrough: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstrikethrough: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Strikethrough: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetStrikethrough: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strikethrough: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetStrikethrough: usize,
    pub Weight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pweight: *mut i16) -> ::windows::core::HRESULT,
    pub SetWeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, weight: i16) -> ::windows::core::HRESULT,
    pub Charset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcharset: *mut i16) -> ::windows::core::HRESULT,
    pub SetCharset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, charset: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub hFont: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phfont: *mut super::super::Graphics::Gdi::HFONT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    hFont: usize,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfont: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsEqual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfontother: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetRatio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cylogical: i32, cyhimetric: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub QueryTextMetrics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptm: *mut super::super::Graphics::Gdi::TEXTMETRICW) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    QueryTextMetrics: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub AddRefHfont: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hfont: super::super::Graphics::Gdi::HFONT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    AddRefHfont: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub ReleaseHfont: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hfont: super::super::Graphics::Gdi::HFONT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    ReleaseHfont: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub SetHdc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hdc: super::super::Graphics::Gdi::HDC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    SetHdc: usize,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFontDisp(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFontDisp {}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFontDisp, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFontDisp {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFontDisp {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFontDisp {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFontDisp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFontDisp").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFontDisp {
    type Vtable = IFontDisp_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFontDisp {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbef6e003_a874_101a_8bba_00aa00300cab);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFontDisp_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFontEventsDisp(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFontEventsDisp {}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFontEventsDisp, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFontEventsDisp {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFontEventsDisp {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFontEventsDisp {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFontEventsDisp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFontEventsDisp").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFontEventsDisp {
    type Vtable = IFontEventsDisp_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFontEventsDisp {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ef6100a_af88_11d0_9846_00c04fc29993);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFontEventsDisp_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IGetOleObject(::windows::core::IUnknown);
impl IGetOleObject {
    pub unsafe fn GetOleObject(&self, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetOleObject)(::windows::core::Vtable::as_raw(self), riid, ppvobj).ok()
    }
}
::windows::core::interface_hierarchy!(IGetOleObject, ::windows::core::IUnknown);
impl ::core::clone::Clone for IGetOleObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IGetOleObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGetOleObject {}
impl ::core::fmt::Debug for IGetOleObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGetOleObject").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IGetOleObject {
    type Vtable = IGetOleObject_Vtbl;
}
unsafe impl ::windows::core::Interface for IGetOleObject {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a701da0_4feb_101b_a82e_08002b2b2337);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetOleObject_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetOleObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IGetVBAObject(::windows::core::IUnknown);
impl IGetVBAObject {
    pub unsafe fn GetObject(&self, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetObject)(::windows::core::Vtable::as_raw(self), riid, ppvobj, dwreserved).ok()
    }
}
::windows::core::interface_hierarchy!(IGetVBAObject, ::windows::core::IUnknown);
impl ::core::clone::Clone for IGetVBAObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IGetVBAObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGetVBAObject {}
impl ::core::fmt::Debug for IGetVBAObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGetVBAObject").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IGetVBAObject {
    type Vtable = IGetVBAObject_Vtbl;
}
unsafe impl ::windows::core::Interface for IGetVBAObject {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x91733a60_3f4c_101b_a3f6_00aa0034e4e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetVBAObject_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IObjectIdentity(::windows::core::IUnknown);
impl IObjectIdentity {
    pub unsafe fn IsEqualObject<P0>(&self, punk: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).IsEqualObject)(::windows::core::Vtable::as_raw(self), punk.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IObjectIdentity, ::windows::core::IUnknown);
impl ::core::clone::Clone for IObjectIdentity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IObjectIdentity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IObjectIdentity {}
impl ::core::fmt::Debug for IObjectIdentity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IObjectIdentity").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IObjectIdentity {
    type Vtable = IObjectIdentity_Vtbl;
}
unsafe impl ::windows::core::Interface for IObjectIdentity {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca04b7e6_0d21_11d1_8cc5_00c04fc2b085);
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectIdentity_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub IsEqualObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IObjectWithSite(::windows::core::IUnknown);
impl IObjectWithSite {
    pub unsafe fn SetSite<P0>(&self, punksite: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).SetSite)(::windows::core::Vtable::as_raw(self), punksite.into().abi()).ok()
    }
    pub unsafe fn GetSite<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSite)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IObjectWithSite, ::windows::core::IUnknown);
impl ::core::clone::Clone for IObjectWithSite {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IObjectWithSite {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IObjectWithSite {}
impl ::core::fmt::Debug for IObjectWithSite {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IObjectWithSite").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IObjectWithSite {
    type Vtable = IObjectWithSite_Vtbl;
}
unsafe impl ::windows::core::Interface for IObjectWithSite {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfc4801a3_2ba9_11cf_a229_00aa003d7352);
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectWithSite_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetSite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punksite: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetSite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvsite: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IOleAdviseHolder(::windows::core::IUnknown);
impl IOleAdviseHolder {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Advise<P0>(&self, padvise: P0) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IAdviseSink>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Advise)(::windows::core::Vtable::as_raw(self), padvise.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Unadvise(&self, dwconnection: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Unadvise)(::windows::core::Vtable::as_raw(self), dwconnection).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumAdvise(&self) -> ::windows::core::Result<super::Com::IEnumSTATDATA> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumAdvise)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SendOnRename<P0>(&self, pmk: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IMoniker>>,
    {
        (::windows::core::Vtable::vtable(self).SendOnRename)(::windows::core::Vtable::as_raw(self), pmk.into().abi()).ok()
    }
    pub unsafe fn SendOnSave(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SendOnSave)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SendOnClose(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SendOnClose)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IOleAdviseHolder, ::windows::core::IUnknown);
impl ::core::clone::Clone for IOleAdviseHolder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IOleAdviseHolder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleAdviseHolder {}
impl ::core::fmt::Debug for IOleAdviseHolder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleAdviseHolder").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IOleAdviseHolder {
    type Vtable = IOleAdviseHolder_Vtbl;
}
unsafe impl ::windows::core::Interface for IOleAdviseHolder {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000111_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleAdviseHolder_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Advise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, padvise: *mut ::core::ffi::c_void, pdwconnection: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Advise: usize,
    pub Unadvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwconnection: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumAdvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumadvise: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumAdvise: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SendOnRename: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SendOnRename: usize,
    pub SendOnSave: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SendOnClose: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IOleCache(::windows::core::IUnknown);
impl IOleCache {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Cache(&self, pformatetc: *const super::Com::FORMATETC, advf: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Cache)(::windows::core::Vtable::as_raw(self), pformatetc, advf, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Uncache(&self, dwconnection: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Uncache)(::windows::core::Vtable::as_raw(self), dwconnection).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumCache(&self) -> ::windows::core::Result<super::Com::IEnumSTATDATA> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumCache)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitCache<P0>(&self, pdataobject: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDataObject>>,
    {
        (::windows::core::Vtable::vtable(self).InitCache)(::windows::core::Vtable::as_raw(self), pdataobject.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn SetData<P0>(&self, pformatetc: *const super::Com::FORMATETC, pmedium: *const super::Com::STGMEDIUM, frelease: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetData)(::windows::core::Vtable::as_raw(self), pformatetc, pmedium, frelease.into()).ok()
    }
}
::windows::core::interface_hierarchy!(IOleCache, ::windows::core::IUnknown);
impl ::core::clone::Clone for IOleCache {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IOleCache {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleCache {}
impl ::core::fmt::Debug for IOleCache {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleCache").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IOleCache {
    type Vtable = IOleCache_Vtbl;
}
unsafe impl ::windows::core::Interface for IOleCache {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000011e_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleCache_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Cache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformatetc: *const super::Com::FORMATETC, advf: u32, pdwconnection: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Cache: usize,
    pub Uncache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwconnection: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumstatdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumCache: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub InitCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdataobject: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitCache: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformatetc: *const super::Com::FORMATETC, pmedium: *const super::Com::STGMEDIUM, frelease: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))]
    SetData: usize,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IOleCache2(::windows::core::IUnknown);
impl IOleCache2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Cache(&self, pformatetc: *const super::Com::FORMATETC, advf: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Cache)(::windows::core::Vtable::as_raw(self), pformatetc, advf, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Uncache(&self, dwconnection: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Uncache)(::windows::core::Vtable::as_raw(self), dwconnection).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumCache(&self) -> ::windows::core::Result<super::Com::IEnumSTATDATA> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumCache)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitCache<P0>(&self, pdataobject: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDataObject>>,
    {
        (::windows::core::Vtable::vtable(self).base__.InitCache)(::windows::core::Vtable::as_raw(self), pdataobject.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn SetData<P0>(&self, pformatetc: *const super::Com::FORMATETC, pmedium: *const super::Com::STGMEDIUM, frelease: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetData)(::windows::core::Vtable::as_raw(self), pformatetc, pmedium, frelease.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UpdateCache<P0>(&self, pdataobject: P0, grfupdf: UPDFCACHE_FLAGS, preserved: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDataObject>>,
    {
        (::windows::core::Vtable::vtable(self).UpdateCache)(::windows::core::Vtable::as_raw(self), pdataobject.into().abi(), grfupdf, ::core::mem::transmute(preserved.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn DiscardCache(&self, dwdiscardoptions: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DiscardCache)(::windows::core::Vtable::as_raw(self), dwdiscardoptions).ok()
    }
}
::windows::core::interface_hierarchy!(IOleCache2, ::windows::core::IUnknown, IOleCache);
impl ::core::clone::Clone for IOleCache2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IOleCache2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleCache2 {}
impl ::core::fmt::Debug for IOleCache2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleCache2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IOleCache2 {
    type Vtable = IOleCache2_Vtbl;
}
unsafe impl ::windows::core::Interface for IOleCache2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000128_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleCache2_Vtbl {
    pub base__: IOleCache_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub UpdateCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdataobject: *mut ::core::ffi::c_void, grfupdf: UPDFCACHE_FLAGS, preserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    UpdateCache: usize,
    pub DiscardCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwdiscardoptions: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IOleCacheControl(::windows::core::IUnknown);
impl IOleCacheControl {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnRun<P0>(&self, pdataobject: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDataObject>>,
    {
        (::windows::core::Vtable::vtable(self).OnRun)(::windows::core::Vtable::as_raw(self), pdataobject.into().abi()).ok()
    }
    pub unsafe fn OnStop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).OnStop)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IOleCacheControl, ::windows::core::IUnknown);
impl ::core::clone::Clone for IOleCacheControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IOleCacheControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleCacheControl {}
impl ::core::fmt::Debug for IOleCacheControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleCacheControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IOleCacheControl {
    type Vtable = IOleCacheControl_Vtbl;
}
unsafe impl ::windows::core::Interface for IOleCacheControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000129_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleCacheControl_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub OnRun: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdataobject: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnRun: usize,
    pub OnStop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IOleClientSite(::windows::core::IUnknown);
impl IOleClientSite {
    pub unsafe fn SaveObject(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SaveObject)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetMoniker(&self, dwassign: OLEGETMONIKER, dwwhichmoniker: OLEWHICHMK) -> ::windows::core::Result<super::Com::IMoniker> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetMoniker)(::windows::core::Vtable::as_raw(self), dwassign, dwwhichmoniker, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetContainer(&self) -> ::windows::core::Result<IOleContainer> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetContainer)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ShowObject(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ShowObject)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnShowWindow<P0>(&self, fshow: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).OnShowWindow)(::windows::core::Vtable::as_raw(self), fshow.into()).ok()
    }
    pub unsafe fn RequestNewObjectLayout(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RequestNewObjectLayout)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IOleClientSite, ::windows::core::IUnknown);
impl ::core::clone::Clone for IOleClientSite {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IOleClientSite {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleClientSite {}
impl ::core::fmt::Debug for IOleClientSite {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleClientSite").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IOleClientSite {
    type Vtable = IOleClientSite_Vtbl;
}
unsafe impl ::windows::core::Interface for IOleClientSite {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000118_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleClientSite_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SaveObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetMoniker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwassign: OLEGETMONIKER, dwwhichmoniker: OLEWHICHMK, ppmk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetMoniker: usize,
    pub GetContainer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcontainer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ShowObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub OnShowWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fshow: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnShowWindow: usize,
    pub RequestNewObjectLayout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IOleCommandTarget(::windows::core::IUnknown);
impl IOleCommandTarget {
    pub unsafe fn QueryStatus(&self, pguidcmdgroup: *const ::windows::core::GUID, ccmds: u32, prgcmds: *mut OLECMD, pcmdtext: *mut OLECMDTEXT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).QueryStatus)(::windows::core::Vtable::as_raw(self), pguidcmdgroup, ccmds, prgcmds, pcmdtext).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Exec(&self, pguidcmdgroup: *const ::windows::core::GUID, ncmdid: u32, ncmdexecopt: u32, pvain: *const super::Com::VARIANT, pvaout: *mut super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Exec)(::windows::core::Vtable::as_raw(self), pguidcmdgroup, ncmdid, ncmdexecopt, pvain, pvaout).ok()
    }
}
::windows::core::interface_hierarchy!(IOleCommandTarget, ::windows::core::IUnknown);
impl ::core::clone::Clone for IOleCommandTarget {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IOleCommandTarget {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleCommandTarget {}
impl ::core::fmt::Debug for IOleCommandTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleCommandTarget").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IOleCommandTarget {
    type Vtable = IOleCommandTarget_Vtbl;
}
unsafe impl ::windows::core::Interface for IOleCommandTarget {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb722bccb_4e68_101b_a2bc_00aa00404770);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleCommandTarget_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub QueryStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidcmdgroup: *const ::windows::core::GUID, ccmds: u32, prgcmds: *mut OLECMD, pcmdtext: *mut OLECMDTEXT) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Exec: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidcmdgroup: *const ::windows::core::GUID, ncmdid: u32, ncmdexecopt: u32, pvain: *const super::Com::VARIANT, pvaout: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Exec: usize,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IOleContainer(::windows::core::IUnknown);
impl IOleContainer {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ParseDisplayName<P0, P1>(&self, pbc: P0, pszdisplayname: P1, pcheaten: *mut u32, ppmkout: *mut ::core::option::Option<super::Com::IMoniker>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IBindCtx>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ParseDisplayName)(::windows::core::Vtable::as_raw(self), pbc.into().abi(), pszdisplayname.into().abi(), pcheaten, ::core::mem::transmute(ppmkout)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumObjects(&self, grfflags: OLECONTF) -> ::windows::core::Result<super::Com::IEnumUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumObjects)(::windows::core::Vtable::as_raw(self), grfflags, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LockContainer<P0>(&self, flock: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).LockContainer)(::windows::core::Vtable::as_raw(self), flock.into()).ok()
    }
}
::windows::core::interface_hierarchy!(IOleContainer, ::windows::core::IUnknown, IParseDisplayName);
impl ::core::clone::Clone for IOleContainer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IOleContainer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleContainer {}
impl ::core::fmt::Debug for IOleContainer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleContainer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IOleContainer {
    type Vtable = IOleContainer_Vtbl;
}
unsafe impl ::windows::core::Interface for IOleContainer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000011b_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleContainer_Vtbl {
    pub base__: IParseDisplayName_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumObjects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, grfflags: OLECONTF, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumObjects: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub LockContainer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flock: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LockContainer: usize,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IOleControl(::windows::core::IUnknown);
impl IOleControl {
    #[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn GetControlInfo(&self, pci: *mut CONTROLINFO) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetControlInfo)(::windows::core::Vtable::as_raw(self), pci).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn OnMnemonic(&self, pmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).OnMnemonic)(::windows::core::Vtable::as_raw(self), pmsg).ok()
    }
    pub unsafe fn OnAmbientPropertyChange(&self, dispid: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).OnAmbientPropertyChange)(::windows::core::Vtable::as_raw(self), dispid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FreezeEvents<P0>(&self, bfreeze: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).FreezeEvents)(::windows::core::Vtable::as_raw(self), bfreeze.into()).ok()
    }
}
::windows::core::interface_hierarchy!(IOleControl, ::windows::core::IUnknown);
impl ::core::clone::Clone for IOleControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IOleControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleControl {}
impl ::core::fmt::Debug for IOleControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IOleControl {
    type Vtable = IOleControl_Vtbl;
}
unsafe impl ::windows::core::Interface for IOleControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb196b288_bab4_101a_b69c_00aa00341d07);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleControl_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub GetControlInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pci: *mut CONTROLINFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    GetControlInfo: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub OnMnemonic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))]
    OnMnemonic: usize,
    pub OnAmbientPropertyChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispid: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub FreezeEvents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bfreeze: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FreezeEvents: usize,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IOleControlSite(::windows::core::IUnknown);
impl IOleControlSite {
    pub unsafe fn OnControlInfoChanged(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).OnControlInfoChanged)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LockInPlaceActive<P0>(&self, flock: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).LockInPlaceActive)(::windows::core::Vtable::as_raw(self), flock.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetExtendedControl(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetExtendedControl)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TransformCoords(&self, pptlhimetric: *mut super::super::Foundation::POINTL, pptfcontainer: *mut POINTF, dwflags: XFORMCOORDS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).TransformCoords)(::windows::core::Vtable::as_raw(self), pptlhimetric, pptfcontainer, dwflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn TranslateAccelerator(&self, pmsg: *const super::super::UI::WindowsAndMessaging::MSG, grfmodifiers: KEYMODIFIERS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).TranslateAccelerator)(::windows::core::Vtable::as_raw(self), pmsg, grfmodifiers).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnFocus<P0>(&self, fgotfocus: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).OnFocus)(::windows::core::Vtable::as_raw(self), fgotfocus.into()).ok()
    }
    pub unsafe fn ShowPropertyFrame(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ShowPropertyFrame)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IOleControlSite, ::windows::core::IUnknown);
impl ::core::clone::Clone for IOleControlSite {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IOleControlSite {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleControlSite {}
impl ::core::fmt::Debug for IOleControlSite {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleControlSite").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IOleControlSite {
    type Vtable = IOleControlSite_Vtbl;
}
unsafe impl ::windows::core::Interface for IOleControlSite {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb196b289_bab4_101a_b69c_00aa00341d07);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleControlSite_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnControlInfoChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub LockInPlaceActive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flock: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LockInPlaceActive: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetExtendedControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdisp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetExtendedControl: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TransformCoords: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptlhimetric: *mut super::super::Foundation::POINTL, pptfcontainer: *mut POINTF, dwflags: XFORMCOORDS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TransformCoords: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub TranslateAccelerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *const super::super::UI::WindowsAndMessaging::MSG, grfmodifiers: KEYMODIFIERS) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))]
    TranslateAccelerator: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fgotfocus: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnFocus: usize,
    pub ShowPropertyFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IOleDocument(::windows::core::IUnknown);
impl IOleDocument {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateView<P0, P1>(&self, pipsite: P0, pstm: P1, dwreserved: u32) -> ::windows::core::Result<IOleDocumentView>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IOleInPlaceSite>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Com::IStream>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateView)(::windows::core::Vtable::as_raw(self), pipsite.into().abi(), pstm.into().abi(), dwreserved, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDocMiscStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDocMiscStatus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumViews(&self, ppenum: *mut ::core::option::Option<IEnumOleDocumentViews>, ppview: *mut ::core::option::Option<IOleDocumentView>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).EnumViews)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppenum), ::core::mem::transmute(ppview)).ok()
    }
}
::windows::core::interface_hierarchy!(IOleDocument, ::windows::core::IUnknown);
impl ::core::clone::Clone for IOleDocument {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IOleDocument {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleDocument {}
impl ::core::fmt::Debug for IOleDocument {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleDocument").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IOleDocument {
    type Vtable = IOleDocument_Vtbl;
}
unsafe impl ::windows::core::Interface for IOleDocument {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb722bcc5_4e68_101b_a2bc_00aa00404770);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleDocument_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pipsite: *mut ::core::ffi::c_void, pstm: *mut ::core::ffi::c_void, dwreserved: u32, ppview: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateView: usize,
    pub GetDocMiscStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT,
    pub EnumViews: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void, ppview: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IOleDocumentSite(::windows::core::IUnknown);
impl IOleDocumentSite {
    pub unsafe fn ActivateMe<P0>(&self, pviewtoactivate: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IOleDocumentView>>,
    {
        (::windows::core::Vtable::vtable(self).ActivateMe)(::windows::core::Vtable::as_raw(self), pviewtoactivate.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IOleDocumentSite, ::windows::core::IUnknown);
impl ::core::clone::Clone for IOleDocumentSite {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IOleDocumentSite {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleDocumentSite {}
impl ::core::fmt::Debug for IOleDocumentSite {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleDocumentSite").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IOleDocumentSite {
    type Vtable = IOleDocumentSite_Vtbl;
}
unsafe impl ::windows::core::Interface for IOleDocumentSite {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb722bcc7_4e68_101b_a2bc_00aa00404770);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleDocumentSite_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ActivateMe: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pviewtoactivate: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IOleDocumentView(::windows::core::IUnknown);
impl IOleDocumentView {
    pub unsafe fn SetInPlaceSite<P0>(&self, pipsite: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IOleInPlaceSite>>,
    {
        (::windows::core::Vtable::vtable(self).SetInPlaceSite)(::windows::core::Vtable::as_raw(self), pipsite.into().abi()).ok()
    }
    pub unsafe fn GetInPlaceSite(&self) -> ::windows::core::Result<IOleInPlaceSite> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetInPlaceSite)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDocument(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDocument)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRect(&self, prcview: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetRect)(::windows::core::Vtable::as_raw(self), prcview).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRect(&self) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetRect)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRectComplex(&self, prcview: *const super::super::Foundation::RECT, prchscroll: *const super::super::Foundation::RECT, prcvscroll: *const super::super::Foundation::RECT, prcsizebox: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetRectComplex)(::windows::core::Vtable::as_raw(self), prcview, prchscroll, prcvscroll, prcsizebox).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Show<P0>(&self, fshow: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).Show)(::windows::core::Vtable::as_raw(self), fshow.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UIActivate<P0>(&self, fuiactivate: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).UIActivate)(::windows::core::Vtable::as_raw(self), fuiactivate.into()).ok()
    }
    pub unsafe fn Open(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Open)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn CloseView(&self, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).CloseView)(::windows::core::Vtable::as_raw(self), dwreserved).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SaveViewState<P0>(&self, pstm: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IStream>>,
    {
        (::windows::core::Vtable::vtable(self).SaveViewState)(::windows::core::Vtable::as_raw(self), pstm.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ApplyViewState<P0>(&self, pstm: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IStream>>,
    {
        (::windows::core::Vtable::vtable(self).ApplyViewState)(::windows::core::Vtable::as_raw(self), pstm.into().abi()).ok()
    }
    pub unsafe fn Clone<P0>(&self, pipsitenew: P0) -> ::windows::core::Result<IOleDocumentView>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IOleInPlaceSite>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), pipsitenew.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IOleDocumentView, ::windows::core::IUnknown);
impl ::core::clone::Clone for IOleDocumentView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IOleDocumentView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleDocumentView {}
impl ::core::fmt::Debug for IOleDocumentView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleDocumentView").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IOleDocumentView {
    type Vtable = IOleDocumentView_Vtbl;
}
unsafe impl ::windows::core::Interface for IOleDocumentView {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb722bcc6_4e68_101b_a2bc_00aa00404770);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleDocumentView_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetInPlaceSite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pipsite: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetInPlaceSite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppipsite: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDocument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prcview: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetRect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prcview: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetRect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetRectComplex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prcview: *const super::super::Foundation::RECT, prchscroll: *const super::super::Foundation::RECT, prcvscroll: *const super::super::Foundation::RECT, prcsizebox: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetRectComplex: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Show: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fshow: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Show: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UIActivate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fuiactivate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UIActivate: usize,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CloseView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SaveViewState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstm: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SaveViewState: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ApplyViewState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstm: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ApplyViewState: usize,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pipsitenew: *mut ::core::ffi::c_void, ppviewnew: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IOleInPlaceActiveObject(::windows::core::IUnknown);
impl IOleInPlaceActiveObject {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindow(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetWindow)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ContextSensitiveHelp<P0>(&self, fentermode: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.ContextSensitiveHelp)(::windows::core::Vtable::as_raw(self), fentermode.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn TranslateAccelerator(&self, lpmsg: ::core::option::Option<*const super::super::UI::WindowsAndMessaging::MSG>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).TranslateAccelerator)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(lpmsg.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnFrameWindowActivate<P0>(&self, factivate: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).OnFrameWindowActivate)(::windows::core::Vtable::as_raw(self), factivate.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnDocWindowActivate<P0>(&self, factivate: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).OnDocWindowActivate)(::windows::core::Vtable::as_raw(self), factivate.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ResizeBorder<P0, P1>(&self, prcborder: *const super::super::Foundation::RECT, puiwindow: P0, fframewindow: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IOleInPlaceUIWindow>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).ResizeBorder)(::windows::core::Vtable::as_raw(self), prcborder, puiwindow.into().abi(), fframewindow.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnableModeless<P0>(&self, fenable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).EnableModeless)(::windows::core::Vtable::as_raw(self), fenable.into()).ok()
    }
}
::windows::core::interface_hierarchy!(IOleInPlaceActiveObject, ::windows::core::IUnknown, IOleWindow);
impl ::core::clone::Clone for IOleInPlaceActiveObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IOleInPlaceActiveObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleInPlaceActiveObject {}
impl ::core::fmt::Debug for IOleInPlaceActiveObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleInPlaceActiveObject").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IOleInPlaceActiveObject {
    type Vtable = IOleInPlaceActiveObject_Vtbl;
}
unsafe impl ::windows::core::Interface for IOleInPlaceActiveObject {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000117_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleInPlaceActiveObject_Vtbl {
    pub base__: IOleWindow_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub TranslateAccelerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))]
    TranslateAccelerator: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnFrameWindowActivate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, factivate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnFrameWindowActivate: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnDocWindowActivate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, factivate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnDocWindowActivate: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ResizeBorder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prcborder: *const super::super::Foundation::RECT, puiwindow: *mut ::core::ffi::c_void, fframewindow: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ResizeBorder: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EnableModeless: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnableModeless: usize,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IOleInPlaceFrame(::windows::core::IUnknown);
impl IOleInPlaceFrame {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindow(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetWindow)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ContextSensitiveHelp<P0>(&self, fentermode: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.ContextSensitiveHelp)(::windows::core::Vtable::as_raw(self), fentermode.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBorder(&self) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetBorder)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestBorderSpace(&self, pborderwidths: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.RequestBorderSpace)(::windows::core::Vtable::as_raw(self), pborderwidths).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBorderSpace(&self, pborderwidths: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetBorderSpace)(::windows::core::Vtable::as_raw(self), pborderwidths).ok()
    }
    pub unsafe fn SetActiveObject<P0, P1>(&self, pactiveobject: P0, pszobjname: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IOleInPlaceActiveObject>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetActiveObject)(::windows::core::Vtable::as_raw(self), pactiveobject.into().abi(), pszobjname.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn InsertMenus<P0>(&self, hmenushared: P0, lpmenuwidths: *mut OLEMENUGROUPWIDTHS) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::UI::WindowsAndMessaging::HMENU>,
    {
        (::windows::core::Vtable::vtable(self).InsertMenus)(::windows::core::Vtable::as_raw(self), hmenushared.into(), lpmenuwidths).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn SetMenu<P0, P1>(&self, hmenushared: P0, holemenu: isize, hwndactiveobject: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::UI::WindowsAndMessaging::HMENU>,
        P1: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).SetMenu)(::windows::core::Vtable::as_raw(self), hmenushared.into(), holemenu, hwndactiveobject.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn RemoveMenus<P0>(&self, hmenushared: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::UI::WindowsAndMessaging::HMENU>,
    {
        (::windows::core::Vtable::vtable(self).RemoveMenus)(::windows::core::Vtable::as_raw(self), hmenushared.into()).ok()
    }
    pub unsafe fn SetStatusText<P0>(&self, pszstatustext: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetStatusText)(::windows::core::Vtable::as_raw(self), pszstatustext.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnableModeless<P0>(&self, fenable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).EnableModeless)(::windows::core::Vtable::as_raw(self), fenable.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn TranslateAccelerator(&self, lpmsg: *const super::super::UI::WindowsAndMessaging::MSG, wid: u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).TranslateAccelerator)(::windows::core::Vtable::as_raw(self), lpmsg, wid).ok()
    }
}
::windows::core::interface_hierarchy!(IOleInPlaceFrame, ::windows::core::IUnknown, IOleWindow, IOleInPlaceUIWindow);
impl ::core::clone::Clone for IOleInPlaceFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IOleInPlaceFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleInPlaceFrame {}
impl ::core::fmt::Debug for IOleInPlaceFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleInPlaceFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IOleInPlaceFrame {
    type Vtable = IOleInPlaceFrame_Vtbl;
}
unsafe impl ::windows::core::Interface for IOleInPlaceFrame {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000116_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleInPlaceFrame_Vtbl {
    pub base__: IOleInPlaceUIWindow_Vtbl,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub InsertMenus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hmenushared: super::super::UI::WindowsAndMessaging::HMENU, lpmenuwidths: *mut OLEMENUGROUPWIDTHS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    InsertMenus: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub SetMenu: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hmenushared: super::super::UI::WindowsAndMessaging::HMENU, holemenu: isize, hwndactiveobject: super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))]
    SetMenu: usize,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub RemoveMenus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hmenushared: super::super::UI::WindowsAndMessaging::HMENU) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    RemoveMenus: usize,
    pub SetStatusText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszstatustext: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EnableModeless: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnableModeless: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub TranslateAccelerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpmsg: *const super::super::UI::WindowsAndMessaging::MSG, wid: u16) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))]
    TranslateAccelerator: usize,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IOleInPlaceObject(::windows::core::IUnknown);
impl IOleInPlaceObject {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindow(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetWindow)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ContextSensitiveHelp<P0>(&self, fentermode: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.ContextSensitiveHelp)(::windows::core::Vtable::as_raw(self), fentermode.into()).ok()
    }
    pub unsafe fn InPlaceDeactivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).InPlaceDeactivate)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn UIDeactivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).UIDeactivate)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetObjectRects(&self, lprcposrect: *const super::super::Foundation::RECT, lprccliprect: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetObjectRects)(::windows::core::Vtable::as_raw(self), lprcposrect, lprccliprect).ok()
    }
    pub unsafe fn ReactivateAndUndo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ReactivateAndUndo)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IOleInPlaceObject, ::windows::core::IUnknown, IOleWindow);
impl ::core::clone::Clone for IOleInPlaceObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IOleInPlaceObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleInPlaceObject {}
impl ::core::fmt::Debug for IOleInPlaceObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleInPlaceObject").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IOleInPlaceObject {
    type Vtable = IOleInPlaceObject_Vtbl;
}
unsafe impl ::windows::core::Interface for IOleInPlaceObject {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000113_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleInPlaceObject_Vtbl {
    pub base__: IOleWindow_Vtbl,
    pub InPlaceDeactivate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UIDeactivate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetObjectRects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lprcposrect: *const super::super::Foundation::RECT, lprccliprect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetObjectRects: usize,
    pub ReactivateAndUndo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IOleInPlaceObjectWindowless(::windows::core::IUnknown);
impl IOleInPlaceObjectWindowless {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindow(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetWindow)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ContextSensitiveHelp<P0>(&self, fentermode: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.ContextSensitiveHelp)(::windows::core::Vtable::as_raw(self), fentermode.into()).ok()
    }
    pub unsafe fn InPlaceDeactivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.InPlaceDeactivate)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn UIDeactivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.UIDeactivate)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetObjectRects(&self, lprcposrect: *const super::super::Foundation::RECT, lprccliprect: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetObjectRects)(::windows::core::Vtable::as_raw(self), lprcposrect, lprccliprect).ok()
    }
    pub unsafe fn ReactivateAndUndo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ReactivateAndUndo)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnWindowMessage<P0, P1>(&self, msg: u32, wparam: P0, lparam: P1) -> ::windows::core::Result<super::super::Foundation::LRESULT>
    where
        P0: ::std::convert::Into<super::super::Foundation::WPARAM>,
        P1: ::std::convert::Into<super::super::Foundation::LPARAM>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).OnWindowMessage)(::windows::core::Vtable::as_raw(self), msg, wparam.into(), lparam.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDropTarget(&self) -> ::windows::core::Result<IDropTarget> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDropTarget)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IOleInPlaceObjectWindowless, ::windows::core::IUnknown, IOleWindow, IOleInPlaceObject);
impl ::core::clone::Clone for IOleInPlaceObjectWindowless {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IOleInPlaceObjectWindowless {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleInPlaceObjectWindowless {}
impl ::core::fmt::Debug for IOleInPlaceObjectWindowless {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleInPlaceObjectWindowless").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IOleInPlaceObjectWindowless {
    type Vtable = IOleInPlaceObjectWindowless_Vtbl;
}
unsafe impl ::windows::core::Interface for IOleInPlaceObjectWindowless {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1c2056cc_5ef4_101b_8bc8_00aa003e3b29);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleInPlaceObjectWindowless_Vtbl {
    pub base__: IOleInPlaceObject_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub OnWindowMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plresult: *mut super::super::Foundation::LRESULT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnWindowMessage: usize,
    pub GetDropTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdroptarget: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IOleInPlaceSite(::windows::core::IUnknown);
impl IOleInPlaceSite {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindow(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetWindow)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ContextSensitiveHelp<P0>(&self, fentermode: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.ContextSensitiveHelp)(::windows::core::Vtable::as_raw(self), fentermode.into()).ok()
    }
    pub unsafe fn CanInPlaceActivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).CanInPlaceActivate)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn OnInPlaceActivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).OnInPlaceActivate)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn OnUIActivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).OnUIActivate)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn GetWindowContext(&self, ppframe: *mut ::core::option::Option<IOleInPlaceFrame>, ppdoc: *mut ::core::option::Option<IOleInPlaceUIWindow>, lprcposrect: *mut super::super::Foundation::RECT, lprccliprect: *mut super::super::Foundation::RECT, lpframeinfo: *mut OLEINPLACEFRAMEINFO) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetWindowContext)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppframe), ::core::mem::transmute(ppdoc), lprcposrect, lprccliprect, lpframeinfo).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Scroll(&self, scrollextant: super::super::Foundation::SIZE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Scroll)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(scrollextant)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnUIDeactivate<P0>(&self, fundoable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).OnUIDeactivate)(::windows::core::Vtable::as_raw(self), fundoable.into()).ok()
    }
    pub unsafe fn OnInPlaceDeactivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).OnInPlaceDeactivate)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn DiscardUndoState(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DiscardUndoState)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn DeactivateAndUndo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DeactivateAndUndo)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnPosRectChange(&self, lprcposrect: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).OnPosRectChange)(::windows::core::Vtable::as_raw(self), lprcposrect).ok()
    }
}
::windows::core::interface_hierarchy!(IOleInPlaceSite, ::windows::core::IUnknown, IOleWindow);
impl ::core::clone::Clone for IOleInPlaceSite {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IOleInPlaceSite {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleInPlaceSite {}
impl ::core::fmt::Debug for IOleInPlaceSite {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleInPlaceSite").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IOleInPlaceSite {
    type Vtable = IOleInPlaceSite_Vtbl;
}
unsafe impl ::windows::core::Interface for IOleInPlaceSite {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000119_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleInPlaceSite_Vtbl {
    pub base__: IOleWindow_Vtbl,
    pub CanInPlaceActivate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnInPlaceActivate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnUIActivate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub GetWindowContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppframe: *mut *mut ::core::ffi::c_void, ppdoc: *mut *mut ::core::ffi::c_void, lprcposrect: *mut super::super::Foundation::RECT, lprccliprect: *mut super::super::Foundation::RECT, lpframeinfo: *mut OLEINPLACEFRAMEINFO) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))]
    GetWindowContext: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Scroll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scrollextant: super::super::Foundation::SIZE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Scroll: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnUIDeactivate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fundoable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnUIDeactivate: usize,
    pub OnInPlaceDeactivate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DiscardUndoState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DeactivateAndUndo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub OnPosRectChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lprcposrect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnPosRectChange: usize,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IOleInPlaceSiteEx(::windows::core::IUnknown);
impl IOleInPlaceSiteEx {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindow(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetWindow)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ContextSensitiveHelp<P0>(&self, fentermode: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.ContextSensitiveHelp)(::windows::core::Vtable::as_raw(self), fentermode.into()).ok()
    }
    pub unsafe fn CanInPlaceActivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CanInPlaceActivate)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn OnInPlaceActivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnInPlaceActivate)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn OnUIActivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnUIActivate)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn GetWindowContext(&self, ppframe: *mut ::core::option::Option<IOleInPlaceFrame>, ppdoc: *mut ::core::option::Option<IOleInPlaceUIWindow>, lprcposrect: *mut super::super::Foundation::RECT, lprccliprect: *mut super::super::Foundation::RECT, lpframeinfo: *mut OLEINPLACEFRAMEINFO) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetWindowContext)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppframe), ::core::mem::transmute(ppdoc), lprcposrect, lprccliprect, lpframeinfo).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Scroll(&self, scrollextant: super::super::Foundation::SIZE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Scroll)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(scrollextant)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnUIDeactivate<P0>(&self, fundoable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.OnUIDeactivate)(::windows::core::Vtable::as_raw(self), fundoable.into()).ok()
    }
    pub unsafe fn OnInPlaceDeactivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnInPlaceDeactivate)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn DiscardUndoState(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DiscardUndoState)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn DeactivateAndUndo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DeactivateAndUndo)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnPosRectChange(&self, lprcposrect: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnPosRectChange)(::windows::core::Vtable::as_raw(self), lprcposrect).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnInPlaceActivateEx(&self, pfnoredraw: *mut super::super::Foundation::BOOL, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).OnInPlaceActivateEx)(::windows::core::Vtable::as_raw(self), pfnoredraw, dwflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnInPlaceDeactivateEx<P0>(&self, fnoredraw: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).OnInPlaceDeactivateEx)(::windows::core::Vtable::as_raw(self), fnoredraw.into()).ok()
    }
    pub unsafe fn RequestUIActivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RequestUIActivate)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IOleInPlaceSiteEx, ::windows::core::IUnknown, IOleWindow, IOleInPlaceSite);
impl ::core::clone::Clone for IOleInPlaceSiteEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IOleInPlaceSiteEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleInPlaceSiteEx {}
impl ::core::fmt::Debug for IOleInPlaceSiteEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleInPlaceSiteEx").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IOleInPlaceSiteEx {
    type Vtable = IOleInPlaceSiteEx_Vtbl;
}
unsafe impl ::windows::core::Interface for IOleInPlaceSiteEx {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9c2cad80_3424_11cf_b670_00aa004cd6d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleInPlaceSiteEx_Vtbl {
    pub base__: IOleInPlaceSite_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub OnInPlaceActivateEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfnoredraw: *mut super::super::Foundation::BOOL, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnInPlaceActivateEx: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnInPlaceDeactivateEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fnoredraw: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnInPlaceDeactivateEx: usize,
    pub RequestUIActivate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IOleInPlaceSiteWindowless(::windows::core::IUnknown);
impl IOleInPlaceSiteWindowless {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindow(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetWindow)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ContextSensitiveHelp<P0>(&self, fentermode: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ContextSensitiveHelp)(::windows::core::Vtable::as_raw(self), fentermode.into()).ok()
    }
    pub unsafe fn CanInPlaceActivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.CanInPlaceActivate)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn OnInPlaceActivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.OnInPlaceActivate)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn OnUIActivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.OnUIActivate)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn GetWindowContext(&self, ppframe: *mut ::core::option::Option<IOleInPlaceFrame>, ppdoc: *mut ::core::option::Option<IOleInPlaceUIWindow>, lprcposrect: *mut super::super::Foundation::RECT, lprccliprect: *mut super::super::Foundation::RECT, lpframeinfo: *mut OLEINPLACEFRAMEINFO) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetWindowContext)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppframe), ::core::mem::transmute(ppdoc), lprcposrect, lprccliprect, lpframeinfo).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Scroll(&self, scrollextant: super::super::Foundation::SIZE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Scroll)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(scrollextant)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnUIDeactivate<P0>(&self, fundoable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.OnUIDeactivate)(::windows::core::Vtable::as_raw(self), fundoable.into()).ok()
    }
    pub unsafe fn OnInPlaceDeactivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.OnInPlaceDeactivate)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn DiscardUndoState(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.DiscardUndoState)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn DeactivateAndUndo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.DeactivateAndUndo)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnPosRectChange(&self, lprcposrect: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.OnPosRectChange)(::windows::core::Vtable::as_raw(self), lprcposrect).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnInPlaceActivateEx(&self, pfnoredraw: *mut super::super::Foundation::BOOL, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnInPlaceActivateEx)(::windows::core::Vtable::as_raw(self), pfnoredraw, dwflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnInPlaceDeactivateEx<P0>(&self, fnoredraw: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.OnInPlaceDeactivateEx)(::windows::core::Vtable::as_raw(self), fnoredraw.into()).ok()
    }
    pub unsafe fn RequestUIActivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.RequestUIActivate)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn CanWindowlessActivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).CanWindowlessActivate)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetCapture(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetCapture)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCapture<P0>(&self, fcapture: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetCapture)(::windows::core::Vtable::as_raw(self), fcapture.into()).ok()
    }
    pub unsafe fn GetFocus(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetFocus)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFocus<P0>(&self, ffocus: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetFocus)(::windows::core::Vtable::as_raw(self), ffocus.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetDC(&self, prect: *const super::super::Foundation::RECT, grfflags: u32) -> ::windows::core::Result<super::super::Graphics::Gdi::HDC> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDC)(::windows::core::Vtable::as_raw(self), prect, grfflags, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn ReleaseDC<P0>(&self, hdc: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
    {
        (::windows::core::Vtable::vtable(self).ReleaseDC)(::windows::core::Vtable::as_raw(self), hdc.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InvalidateRect<P0>(&self, prect: *const super::super::Foundation::RECT, ferase: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).InvalidateRect)(::windows::core::Vtable::as_raw(self), prect, ferase.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn InvalidateRgn<P0, P1>(&self, hrgn: P0, ferase: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Graphics::Gdi::HRGN>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).InvalidateRgn)(::windows::core::Vtable::as_raw(self), hrgn.into(), ferase.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ScrollRect(&self, dx: i32, dy: i32, prectscroll: *const super::super::Foundation::RECT, prectclip: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ScrollRect)(::windows::core::Vtable::as_raw(self), dx, dy, prectscroll, prectclip).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AdjustRect(&self, prc: *mut super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).AdjustRect)(::windows::core::Vtable::as_raw(self), prc).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnDefWindowMessage<P0, P1>(&self, msg: u32, wparam: P0, lparam: P1) -> ::windows::core::Result<super::super::Foundation::LRESULT>
    where
        P0: ::std::convert::Into<super::super::Foundation::WPARAM>,
        P1: ::std::convert::Into<super::super::Foundation::LPARAM>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).OnDefWindowMessage)(::windows::core::Vtable::as_raw(self), msg, wparam.into(), lparam.into(), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IOleInPlaceSiteWindowless, ::windows::core::IUnknown, IOleWindow, IOleInPlaceSite, IOleInPlaceSiteEx);
impl ::core::clone::Clone for IOleInPlaceSiteWindowless {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IOleInPlaceSiteWindowless {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleInPlaceSiteWindowless {}
impl ::core::fmt::Debug for IOleInPlaceSiteWindowless {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleInPlaceSiteWindowless").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IOleInPlaceSiteWindowless {
    type Vtable = IOleInPlaceSiteWindowless_Vtbl;
}
unsafe impl ::windows::core::Interface for IOleInPlaceSiteWindowless {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x922eada0_3424_11cf_b670_00aa004cd6d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleInPlaceSiteWindowless_Vtbl {
    pub base__: IOleInPlaceSiteEx_Vtbl,
    pub CanWindowlessActivate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCapture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCapture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fcapture: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCapture: usize,
    pub GetFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ffocus: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFocus: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub GetDC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prect: *const super::super::Foundation::RECT, grfflags: u32, phdc: *mut super::super::Graphics::Gdi::HDC) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))]
    GetDC: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub ReleaseDC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hdc: super::super::Graphics::Gdi::HDC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    ReleaseDC: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InvalidateRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prect: *const super::super::Foundation::RECT, ferase: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InvalidateRect: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub InvalidateRgn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrgn: super::super::Graphics::Gdi::HRGN, ferase: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))]
    InvalidateRgn: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ScrollRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dx: i32, dy: i32, prectscroll: *const super::super::Foundation::RECT, prectclip: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ScrollRect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AdjustRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prc: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AdjustRect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnDefWindowMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plresult: *mut super::super::Foundation::LRESULT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnDefWindowMessage: usize,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IOleInPlaceUIWindow(::windows::core::IUnknown);
impl IOleInPlaceUIWindow {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindow(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetWindow)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ContextSensitiveHelp<P0>(&self, fentermode: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.ContextSensitiveHelp)(::windows::core::Vtable::as_raw(self), fentermode.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBorder(&self) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetBorder)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestBorderSpace(&self, pborderwidths: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RequestBorderSpace)(::windows::core::Vtable::as_raw(self), pborderwidths).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBorderSpace(&self, pborderwidths: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetBorderSpace)(::windows::core::Vtable::as_raw(self), pborderwidths).ok()
    }
    pub unsafe fn SetActiveObject<P0, P1>(&self, pactiveobject: P0, pszobjname: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IOleInPlaceActiveObject>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetActiveObject)(::windows::core::Vtable::as_raw(self), pactiveobject.into().abi(), pszobjname.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IOleInPlaceUIWindow, ::windows::core::IUnknown, IOleWindow);
impl ::core::clone::Clone for IOleInPlaceUIWindow {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IOleInPlaceUIWindow {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleInPlaceUIWindow {}
impl ::core::fmt::Debug for IOleInPlaceUIWindow {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleInPlaceUIWindow").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IOleInPlaceUIWindow {
    type Vtable = IOleInPlaceUIWindow_Vtbl;
}
unsafe impl ::windows::core::Interface for IOleInPlaceUIWindow {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000115_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleInPlaceUIWindow_Vtbl {
    pub base__: IOleWindow_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetBorder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lprectborder: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetBorder: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RequestBorderSpace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pborderwidths: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RequestBorderSpace: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBorderSpace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pborderwidths: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBorderSpace: usize,
    pub SetActiveObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pactiveobject: *mut ::core::ffi::c_void, pszobjname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IOleItemContainer(::windows::core::IUnknown);
impl IOleItemContainer {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ParseDisplayName<P0, P1>(&self, pbc: P0, pszdisplayname: P1, pcheaten: *mut u32, ppmkout: *mut ::core::option::Option<super::Com::IMoniker>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IBindCtx>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.ParseDisplayName)(::windows::core::Vtable::as_raw(self), pbc.into().abi(), pszdisplayname.into().abi(), pcheaten, ::core::mem::transmute(ppmkout)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumObjects(&self, grfflags: OLECONTF) -> ::windows::core::Result<super::Com::IEnumUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumObjects)(::windows::core::Vtable::as_raw(self), grfflags, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LockContainer<P0>(&self, flock: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.LockContainer)(::windows::core::Vtable::as_raw(self), flock.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetObject<P0, P1, T>(&self, pszitem: P0, dwspeedneeded: u32, pbc: P1) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Com::IBindCtx>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetObject)(::windows::core::Vtable::as_raw(self), pszitem.into().abi(), dwspeedneeded, pbc.into().abi(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetObjectStorage<P0, P1, T>(&self, pszitem: P0, pbc: P1) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Com::IBindCtx>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetObjectStorage)(::windows::core::Vtable::as_raw(self), pszitem.into().abi(), pbc.into().abi(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn IsRunning<P0>(&self, pszitem: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).IsRunning)(::windows::core::Vtable::as_raw(self), pszitem.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IOleItemContainer, ::windows::core::IUnknown, IParseDisplayName, IOleContainer);
impl ::core::clone::Clone for IOleItemContainer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IOleItemContainer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleItemContainer {}
impl ::core::fmt::Debug for IOleItemContainer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleItemContainer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IOleItemContainer {
    type Vtable = IOleItemContainer_Vtbl;
}
unsafe impl ::windows::core::Interface for IOleItemContainer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000011c_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleItemContainer_Vtbl {
    pub base__: IOleContainer_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszitem: ::windows::core::PCWSTR, dwspeedneeded: u32, pbc: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetObject: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetObjectStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszitem: ::windows::core::PCWSTR, pbc: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvstorage: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetObjectStorage: usize,
    pub IsRunning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszitem: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IOleLink(::windows::core::IUnknown);
impl IOleLink {
    pub unsafe fn SetUpdateOptions(&self, dwupdateopt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetUpdateOptions)(::windows::core::Vtable::as_raw(self), dwupdateopt).ok()
    }
    pub unsafe fn GetUpdateOptions(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetUpdateOptions)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSourceMoniker<P0>(&self, pmk: P0, rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IMoniker>>,
    {
        (::windows::core::Vtable::vtable(self).SetSourceMoniker)(::windows::core::Vtable::as_raw(self), pmk.into().abi(), rclsid).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSourceMoniker(&self) -> ::windows::core::Result<super::Com::IMoniker> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSourceMoniker)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetSourceDisplayName<P0>(&self, pszstatustext: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetSourceDisplayName)(::windows::core::Vtable::as_raw(self), pszstatustext.into().abi()).ok()
    }
    pub unsafe fn GetSourceDisplayName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSourceDisplayName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BindToSource<P0>(&self, bindflags: u32, pbc: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IBindCtx>>,
    {
        (::windows::core::Vtable::vtable(self).BindToSource)(::windows::core::Vtable::as_raw(self), bindflags, pbc.into().abi()).ok()
    }
    pub unsafe fn BindIfRunning(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).BindIfRunning)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetBoundSource(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetBoundSource)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn UnbindSource(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).UnbindSource)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Update<P0>(&self, pbc: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IBindCtx>>,
    {
        (::windows::core::Vtable::vtable(self).Update)(::windows::core::Vtable::as_raw(self), pbc.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IOleLink, ::windows::core::IUnknown);
impl ::core::clone::Clone for IOleLink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IOleLink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleLink {}
impl ::core::fmt::Debug for IOleLink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleLink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IOleLink {
    type Vtable = IOleLink_Vtbl;
}
unsafe impl ::windows::core::Interface for IOleLink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000011d_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleLink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetUpdateOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwupdateopt: u32) -> ::windows::core::HRESULT,
    pub GetUpdateOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwupdateopt: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSourceMoniker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmk: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSourceMoniker: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSourceMoniker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppmk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSourceMoniker: usize,
    pub SetSourceDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszstatustext: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetSourceDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszdisplayname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub BindToSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bindflags: u32, pbc: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BindToSource: usize,
    pub BindIfRunning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetBoundSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UnbindSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Update: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbc: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Update: usize,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IOleObject(::windows::core::IUnknown);
impl IOleObject {
    pub unsafe fn SetClientSite<P0>(&self, pclientsite: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IOleClientSite>>,
    {
        (::windows::core::Vtable::vtable(self).SetClientSite)(::windows::core::Vtable::as_raw(self), pclientsite.into().abi()).ok()
    }
    pub unsafe fn GetClientSite(&self) -> ::windows::core::Result<IOleClientSite> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetClientSite)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetHostNames<P0, P1>(&self, szcontainerapp: P0, szcontainerobj: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetHostNames)(::windows::core::Vtable::as_raw(self), szcontainerapp.into().abi(), szcontainerobj.into().abi()).ok()
    }
    pub unsafe fn Close(&self, dwsaveoption: OLECLOSE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Close)(::windows::core::Vtable::as_raw(self), dwsaveoption).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetMoniker<P0>(&self, dwwhichmoniker: OLEWHICHMK, pmk: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IMoniker>>,
    {
        (::windows::core::Vtable::vtable(self).SetMoniker)(::windows::core::Vtable::as_raw(self), dwwhichmoniker, pmk.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetMoniker(&self, dwassign: OLEGETMONIKER, dwwhichmoniker: OLEWHICHMK) -> ::windows::core::Result<super::Com::IMoniker> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetMoniker)(::windows::core::Vtable::as_raw(self), dwassign, dwwhichmoniker, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn InitFromData<P0, P1>(&self, pdataobject: P0, fcreation: P1, dwreserved: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDataObject>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).InitFromData)(::windows::core::Vtable::as_raw(self), pdataobject.into().abi(), fcreation.into(), dwreserved).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetClipboardData(&self, dwreserved: u32) -> ::windows::core::Result<super::Com::IDataObject> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetClipboardData)(::windows::core::Vtable::as_raw(self), dwreserved, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn DoVerb<P0, P1>(&self, iverb: i32, lpmsg: *const super::super::UI::WindowsAndMessaging::MSG, pactivesite: P0, lindex: i32, hwndparent: P1, lprcposrect: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IOleClientSite>>,
        P1: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).DoVerb)(::windows::core::Vtable::as_raw(self), iverb, lpmsg, pactivesite.into().abi(), lindex, hwndparent.into(), lprcposrect).ok()
    }
    pub unsafe fn EnumVerbs(&self) -> ::windows::core::Result<IEnumOLEVERB> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumVerbs)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Update(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Update)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn IsUpToDate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).IsUpToDate)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetUserClassID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetUserClassID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetUserType(&self, dwformoftype: USERCLASSTYPE) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetUserType)(::windows::core::Vtable::as_raw(self), dwformoftype, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetExtent(&self, dwdrawaspect: super::Com::DVASPECT, psizel: *const super::super::Foundation::SIZE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetExtent)(::windows::core::Vtable::as_raw(self), dwdrawaspect, psizel).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetExtent(&self, dwdrawaspect: super::Com::DVASPECT) -> ::windows::core::Result<super::super::Foundation::SIZE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetExtent)(::windows::core::Vtable::as_raw(self), dwdrawaspect, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Advise<P0>(&self, padvsink: P0) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IAdviseSink>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Advise)(::windows::core::Vtable::as_raw(self), padvsink.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Unadvise(&self, dwconnection: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Unadvise)(::windows::core::Vtable::as_raw(self), dwconnection).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumAdvise(&self) -> ::windows::core::Result<super::Com::IEnumSTATDATA> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumAdvise)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetMiscStatus(&self, dwaspect: super::Com::DVASPECT) -> ::windows::core::Result<OLEMISC> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetMiscStatus)(::windows::core::Vtable::as_raw(self), dwaspect, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn SetColorScheme(&self, plogpal: *const super::super::Graphics::Gdi::LOGPALETTE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetColorScheme)(::windows::core::Vtable::as_raw(self), plogpal).ok()
    }
}
::windows::core::interface_hierarchy!(IOleObject, ::windows::core::IUnknown);
impl ::core::clone::Clone for IOleObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IOleObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleObject {}
impl ::core::fmt::Debug for IOleObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleObject").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IOleObject {
    type Vtable = IOleObject_Vtbl;
}
unsafe impl ::windows::core::Interface for IOleObject {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000112_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleObject_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetClientSite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclientsite: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetClientSite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppclientsite: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetHostNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szcontainerapp: ::windows::core::PCWSTR, szcontainerobj: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsaveoption: OLECLOSE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SetMoniker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwwhichmoniker: OLEWHICHMK, pmk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetMoniker: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetMoniker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwassign: OLEGETMONIKER, dwwhichmoniker: OLEWHICHMK, ppmk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetMoniker: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub InitFromData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdataobject: *mut ::core::ffi::c_void, fcreation: super::super::Foundation::BOOL, dwreserved: u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    InitFromData: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetClipboardData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwreserved: u32, ppdataobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetClipboardData: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub DoVerb: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iverb: i32, lpmsg: *const super::super::UI::WindowsAndMessaging::MSG, pactivesite: *mut ::core::ffi::c_void, lindex: i32, hwndparent: super::super::Foundation::HWND, lprcposrect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))]
    DoVerb: usize,
    pub EnumVerbs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumoleverb: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Update: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsUpToDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetUserClassID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetUserType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwformoftype: USERCLASSTYPE, pszusertype: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub SetExtent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwdrawaspect: super::Com::DVASPECT, psizel: *const super::super::Foundation::SIZE) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    SetExtent: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetExtent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwdrawaspect: super::Com::DVASPECT, psizel: *mut super::super::Foundation::SIZE) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetExtent: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Advise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, padvsink: *mut ::core::ffi::c_void, pdwconnection: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Advise: usize,
    pub Unadvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwconnection: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumAdvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumadvise: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumAdvise: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetMiscStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwaspect: super::Com::DVASPECT, pdwstatus: *mut OLEMISC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetMiscStatus: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub SetColorScheme: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plogpal: *const super::super::Graphics::Gdi::LOGPALETTE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    SetColorScheme: usize,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IOleParentUndoUnit(::windows::core::IUnknown);
impl IOleParentUndoUnit {
    pub unsafe fn Do<P0>(&self, pundomanager: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IOleUndoManager>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Do)(::windows::core::Vtable::as_raw(self), pundomanager.into().abi()).ok()
    }
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDescription)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetUnitType(&self, pclsid: *mut ::windows::core::GUID, plid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetUnitType)(::windows::core::Vtable::as_raw(self), pclsid, plid).ok()
    }
    pub unsafe fn OnNextAdd(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnNextAdd)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Open<P0>(&self, ppuu: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IOleParentUndoUnit>>,
    {
        (::windows::core::Vtable::vtable(self).Open)(::windows::core::Vtable::as_raw(self), ppuu.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Close<P0, P1>(&self, ppuu: P0, fcommit: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IOleParentUndoUnit>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).Close)(::windows::core::Vtable::as_raw(self), ppuu.into().abi(), fcommit.into()).ok()
    }
    pub unsafe fn Add<P0>(&self, puu: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IOleUndoUnit>>,
    {
        (::windows::core::Vtable::vtable(self).Add)(::windows::core::Vtable::as_raw(self), puu.into().abi()).ok()
    }
    pub unsafe fn FindUnit<P0>(&self, puu: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IOleUndoUnit>>,
    {
        (::windows::core::Vtable::vtable(self).FindUnit)(::windows::core::Vtable::as_raw(self), puu.into().abi()).ok()
    }
    pub unsafe fn GetParentState(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetParentState)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IOleParentUndoUnit, ::windows::core::IUnknown, IOleUndoUnit);
impl ::core::clone::Clone for IOleParentUndoUnit {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IOleParentUndoUnit {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleParentUndoUnit {}
impl ::core::fmt::Debug for IOleParentUndoUnit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleParentUndoUnit").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IOleParentUndoUnit {
    type Vtable = IOleParentUndoUnit_Vtbl;
}
unsafe impl ::windows::core::Interface for IOleParentUndoUnit {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa1faf330_ef97_11ce_9bc9_00aa00608e01);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleParentUndoUnit_Vtbl {
    pub base__: IOleUndoUnit_Vtbl,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppuu: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppuu: *mut ::core::ffi::c_void, fcommit: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Close: usize,
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puu: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FindUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puu: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetParentState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwstate: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IOleUILinkContainerA(::windows::core::IUnknown);
impl IOleUILinkContainerA {
    pub unsafe fn GetNextLink(&self, dwlink: u32) -> u32 {
        (::windows::core::Vtable::vtable(self).GetNextLink)(::windows::core::Vtable::as_raw(self), dwlink)
    }
    pub unsafe fn SetLinkUpdateOptions(&self, dwlink: u32, dwupdateopt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetLinkUpdateOptions)(::windows::core::Vtable::as_raw(self), dwlink, dwupdateopt).ok()
    }
    pub unsafe fn GetLinkUpdateOptions(&self, dwlink: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetLinkUpdateOptions)(::windows::core::Vtable::as_raw(self), dwlink, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLinkSource<P0, P1>(&self, dwlink: u32, lpszdisplayname: P0, lenfilename: u32, pcheaten: *mut u32, fvalidatesource: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetLinkSource)(::windows::core::Vtable::as_raw(self), dwlink, lpszdisplayname.into().abi(), lenfilename, pcheaten, fvalidatesource.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLinkSource(&self, dwlink: u32, lplpszdisplayname: ::core::option::Option<*mut ::windows::core::PSTR>, lplenfilename: *mut u32, lplpszfulllinktype: ::core::option::Option<*mut ::windows::core::PSTR>, lplpszshortlinktype: ::core::option::Option<*mut ::windows::core::PSTR>, lpfsourceavailable: *mut super::super::Foundation::BOOL, lpfisselected: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetLinkSource)(::windows::core::Vtable::as_raw(self), dwlink, ::core::mem::transmute(lplpszdisplayname.unwrap_or(::std::ptr::null_mut())), lplenfilename, ::core::mem::transmute(lplpszfulllinktype.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lplpszshortlinktype.unwrap_or(::std::ptr::null_mut())), lpfsourceavailable, lpfisselected).ok()
    }
    pub unsafe fn OpenLinkSource(&self, dwlink: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).OpenLinkSource)(::windows::core::Vtable::as_raw(self), dwlink).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UpdateLink<P0, P1>(&self, dwlink: u32, ferrormessage: P0, freserved: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).UpdateLink)(::windows::core::Vtable::as_raw(self), dwlink, ferrormessage.into(), freserved.into()).ok()
    }
    pub unsafe fn CancelLink(&self, dwlink: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).CancelLink)(::windows::core::Vtable::as_raw(self), dwlink).ok()
    }
}
::windows::core::interface_hierarchy!(IOleUILinkContainerA, ::windows::core::IUnknown);
impl ::core::clone::Clone for IOleUILinkContainerA {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IOleUILinkContainerA {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleUILinkContainerA {}
impl ::core::fmt::Debug for IOleUILinkContainerA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleUILinkContainerA").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IOleUILinkContainerA {
    type Vtable = IOleUILinkContainerA_Vtbl;
}
unsafe impl ::windows::core::Interface for IOleUILinkContainerA {
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleUILinkContainerA_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetNextLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwlink: u32) -> u32,
    pub SetLinkUpdateOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwlink: u32, dwupdateopt: u32) -> ::windows::core::HRESULT,
    pub GetLinkUpdateOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwlink: u32, lpdwupdateopt: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLinkSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwlink: u32, lpszdisplayname: ::windows::core::PCSTR, lenfilename: u32, pcheaten: *mut u32, fvalidatesource: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLinkSource: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLinkSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwlink: u32, lplpszdisplayname: *mut ::windows::core::PSTR, lplenfilename: *mut u32, lplpszfulllinktype: *mut ::windows::core::PSTR, lplpszshortlinktype: *mut ::windows::core::PSTR, lpfsourceavailable: *mut super::super::Foundation::BOOL, lpfisselected: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLinkSource: usize,
    pub OpenLinkSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwlink: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub UpdateLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwlink: u32, ferrormessage: super::super::Foundation::BOOL, freserved: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UpdateLink: usize,
    pub CancelLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwlink: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IOleUILinkContainerW(::windows::core::IUnknown);
impl IOleUILinkContainerW {
    pub unsafe fn GetNextLink(&self, dwlink: u32) -> u32 {
        (::windows::core::Vtable::vtable(self).GetNextLink)(::windows::core::Vtable::as_raw(self), dwlink)
    }
    pub unsafe fn SetLinkUpdateOptions(&self, dwlink: u32, dwupdateopt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetLinkUpdateOptions)(::windows::core::Vtable::as_raw(self), dwlink, dwupdateopt).ok()
    }
    pub unsafe fn GetLinkUpdateOptions(&self, dwlink: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetLinkUpdateOptions)(::windows::core::Vtable::as_raw(self), dwlink, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLinkSource<P0, P1>(&self, dwlink: u32, lpszdisplayname: P0, lenfilename: u32, pcheaten: *mut u32, fvalidatesource: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetLinkSource)(::windows::core::Vtable::as_raw(self), dwlink, lpszdisplayname.into().abi(), lenfilename, pcheaten, fvalidatesource.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLinkSource(&self, dwlink: u32, lplpszdisplayname: ::core::option::Option<*mut ::windows::core::PWSTR>, lplenfilename: *mut u32, lplpszfulllinktype: ::core::option::Option<*mut ::windows::core::PWSTR>, lplpszshortlinktype: ::core::option::Option<*mut ::windows::core::PWSTR>, lpfsourceavailable: *mut super::super::Foundation::BOOL, lpfisselected: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetLinkSource)(::windows::core::Vtable::as_raw(self), dwlink, ::core::mem::transmute(lplpszdisplayname.unwrap_or(::std::ptr::null_mut())), lplenfilename, ::core::mem::transmute(lplpszfulllinktype.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lplpszshortlinktype.unwrap_or(::std::ptr::null_mut())), lpfsourceavailable, lpfisselected).ok()
    }
    pub unsafe fn OpenLinkSource(&self, dwlink: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).OpenLinkSource)(::windows::core::Vtable::as_raw(self), dwlink).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UpdateLink<P0, P1>(&self, dwlink: u32, ferrormessage: P0, freserved: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).UpdateLink)(::windows::core::Vtable::as_raw(self), dwlink, ferrormessage.into(), freserved.into()).ok()
    }
    pub unsafe fn CancelLink(&self, dwlink: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).CancelLink)(::windows::core::Vtable::as_raw(self), dwlink).ok()
    }
}
::windows::core::interface_hierarchy!(IOleUILinkContainerW, ::windows::core::IUnknown);
impl ::core::clone::Clone for IOleUILinkContainerW {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IOleUILinkContainerW {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleUILinkContainerW {}
impl ::core::fmt::Debug for IOleUILinkContainerW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleUILinkContainerW").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IOleUILinkContainerW {
    type Vtable = IOleUILinkContainerW_Vtbl;
}
unsafe impl ::windows::core::Interface for IOleUILinkContainerW {
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleUILinkContainerW_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetNextLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwlink: u32) -> u32,
    pub SetLinkUpdateOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwlink: u32, dwupdateopt: u32) -> ::windows::core::HRESULT,
    pub GetLinkUpdateOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwlink: u32, lpdwupdateopt: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLinkSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwlink: u32, lpszdisplayname: ::windows::core::PCWSTR, lenfilename: u32, pcheaten: *mut u32, fvalidatesource: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLinkSource: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLinkSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwlink: u32, lplpszdisplayname: *mut ::windows::core::PWSTR, lplenfilename: *mut u32, lplpszfulllinktype: *mut ::windows::core::PWSTR, lplpszshortlinktype: *mut ::windows::core::PWSTR, lpfsourceavailable: *mut super::super::Foundation::BOOL, lpfisselected: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLinkSource: usize,
    pub OpenLinkSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwlink: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub UpdateLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwlink: u32, ferrormessage: super::super::Foundation::BOOL, freserved: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UpdateLink: usize,
    pub CancelLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwlink: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IOleUILinkInfoA(::windows::core::IUnknown);
impl IOleUILinkInfoA {
    pub unsafe fn GetNextLink(&self, dwlink: u32) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetNextLink)(::windows::core::Vtable::as_raw(self), dwlink)
    }
    pub unsafe fn SetLinkUpdateOptions(&self, dwlink: u32, dwupdateopt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetLinkUpdateOptions)(::windows::core::Vtable::as_raw(self), dwlink, dwupdateopt).ok()
    }
    pub unsafe fn GetLinkUpdateOptions(&self, dwlink: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetLinkUpdateOptions)(::windows::core::Vtable::as_raw(self), dwlink, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLinkSource<P0, P1>(&self, dwlink: u32, lpszdisplayname: P0, lenfilename: u32, pcheaten: *mut u32, fvalidatesource: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetLinkSource)(::windows::core::Vtable::as_raw(self), dwlink, lpszdisplayname.into().abi(), lenfilename, pcheaten, fvalidatesource.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLinkSource(&self, dwlink: u32, lplpszdisplayname: ::core::option::Option<*mut ::windows::core::PSTR>, lplenfilename: *mut u32, lplpszfulllinktype: ::core::option::Option<*mut ::windows::core::PSTR>, lplpszshortlinktype: ::core::option::Option<*mut ::windows::core::PSTR>, lpfsourceavailable: *mut super::super::Foundation::BOOL, lpfisselected: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetLinkSource)(::windows::core::Vtable::as_raw(self), dwlink, ::core::mem::transmute(lplpszdisplayname.unwrap_or(::std::ptr::null_mut())), lplenfilename, ::core::mem::transmute(lplpszfulllinktype.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lplpszshortlinktype.unwrap_or(::std::ptr::null_mut())), lpfsourceavailable, lpfisselected).ok()
    }
    pub unsafe fn OpenLinkSource(&self, dwlink: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OpenLinkSource)(::windows::core::Vtable::as_raw(self), dwlink).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UpdateLink<P0, P1>(&self, dwlink: u32, ferrormessage: P0, freserved: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.UpdateLink)(::windows::core::Vtable::as_raw(self), dwlink, ferrormessage.into(), freserved.into()).ok()
    }
    pub unsafe fn CancelLink(&self, dwlink: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CancelLink)(::windows::core::Vtable::as_raw(self), dwlink).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLastUpdate(&self, dwlink: u32) -> ::windows::core::Result<super::super::Foundation::FILETIME> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetLastUpdate)(::windows::core::Vtable::as_raw(self), dwlink, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IOleUILinkInfoA, ::windows::core::IUnknown, IOleUILinkContainerA);
impl ::core::clone::Clone for IOleUILinkInfoA {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IOleUILinkInfoA {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleUILinkInfoA {}
impl ::core::fmt::Debug for IOleUILinkInfoA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleUILinkInfoA").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IOleUILinkInfoA {
    type Vtable = IOleUILinkInfoA_Vtbl;
}
unsafe impl ::windows::core::Interface for IOleUILinkInfoA {
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleUILinkInfoA_Vtbl {
    pub base__: IOleUILinkContainerA_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLastUpdate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwlink: u32, lplastupdate: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLastUpdate: usize,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IOleUILinkInfoW(::windows::core::IUnknown);
impl IOleUILinkInfoW {
    pub unsafe fn GetNextLink(&self, dwlink: u32) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetNextLink)(::windows::core::Vtable::as_raw(self), dwlink)
    }
    pub unsafe fn SetLinkUpdateOptions(&self, dwlink: u32, dwupdateopt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetLinkUpdateOptions)(::windows::core::Vtable::as_raw(self), dwlink, dwupdateopt).ok()
    }
    pub unsafe fn GetLinkUpdateOptions(&self, dwlink: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetLinkUpdateOptions)(::windows::core::Vtable::as_raw(self), dwlink, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLinkSource<P0, P1>(&self, dwlink: u32, lpszdisplayname: P0, lenfilename: u32, pcheaten: *mut u32, fvalidatesource: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetLinkSource)(::windows::core::Vtable::as_raw(self), dwlink, lpszdisplayname.into().abi(), lenfilename, pcheaten, fvalidatesource.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLinkSource(&self, dwlink: u32, lplpszdisplayname: ::core::option::Option<*mut ::windows::core::PWSTR>, lplenfilename: *mut u32, lplpszfulllinktype: ::core::option::Option<*mut ::windows::core::PWSTR>, lplpszshortlinktype: ::core::option::Option<*mut ::windows::core::PWSTR>, lpfsourceavailable: *mut super::super::Foundation::BOOL, lpfisselected: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetLinkSource)(::windows::core::Vtable::as_raw(self), dwlink, ::core::mem::transmute(lplpszdisplayname.unwrap_or(::std::ptr::null_mut())), lplenfilename, ::core::mem::transmute(lplpszfulllinktype.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lplpszshortlinktype.unwrap_or(::std::ptr::null_mut())), lpfsourceavailable, lpfisselected).ok()
    }
    pub unsafe fn OpenLinkSource(&self, dwlink: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OpenLinkSource)(::windows::core::Vtable::as_raw(self), dwlink).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UpdateLink<P0, P1>(&self, dwlink: u32, ferrormessage: P0, freserved: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.UpdateLink)(::windows::core::Vtable::as_raw(self), dwlink, ferrormessage.into(), freserved.into()).ok()
    }
    pub unsafe fn CancelLink(&self, dwlink: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CancelLink)(::windows::core::Vtable::as_raw(self), dwlink).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLastUpdate(&self, dwlink: u32) -> ::windows::core::Result<super::super::Foundation::FILETIME> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetLastUpdate)(::windows::core::Vtable::as_raw(self), dwlink, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IOleUILinkInfoW, ::windows::core::IUnknown, IOleUILinkContainerW);
impl ::core::clone::Clone for IOleUILinkInfoW {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IOleUILinkInfoW {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleUILinkInfoW {}
impl ::core::fmt::Debug for IOleUILinkInfoW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleUILinkInfoW").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IOleUILinkInfoW {
    type Vtable = IOleUILinkInfoW_Vtbl;
}
unsafe impl ::windows::core::Interface for IOleUILinkInfoW {
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleUILinkInfoW_Vtbl {
    pub base__: IOleUILinkContainerW_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLastUpdate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwlink: u32, lplastupdate: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLastUpdate: usize,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IOleUIObjInfoA(::windows::core::IUnknown);
impl IOleUIObjInfoA {
    pub unsafe fn GetObjectInfo(&self, dwobject: u32, lpdwobjsize: *mut u32, lplpszlabel: ::core::option::Option<*mut ::windows::core::PSTR>, lplpsztype: ::core::option::Option<*mut ::windows::core::PSTR>, lplpszshorttype: ::core::option::Option<*mut ::windows::core::PSTR>, lplpszlocation: ::core::option::Option<*mut ::windows::core::PSTR>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetObjectInfo)(::windows::core::Vtable::as_raw(self), dwobject, lpdwobjsize, ::core::mem::transmute(lplpszlabel.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lplpsztype.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lplpszshorttype.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lplpszlocation.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetConvertInfo(&self, dwobject: u32, lpclassid: *mut ::windows::core::GUID, lpwformat: *mut u16, lpconvertdefaultclassid: *mut ::windows::core::GUID, lplpclsidexclude: *mut *mut ::windows::core::GUID, lpcclsidexclude: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetConvertInfo)(::windows::core::Vtable::as_raw(self), dwobject, lpclassid, lpwformat, lpconvertdefaultclassid, lplpclsidexclude, ::core::mem::transmute(lpcclsidexclude.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn ConvertObject(&self, dwobject: u32, clsidnew: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ConvertObject)(::windows::core::Vtable::as_raw(self), dwobject, clsidnew).ok()
    }
    pub unsafe fn GetViewInfo(&self, dwobject: u32, phmetapict: ::core::option::Option<*const isize>, pdvaspect: ::core::option::Option<*const u32>, pncurrentscale: ::core::option::Option<*const i32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetViewInfo)(::windows::core::Vtable::as_raw(self), dwobject, ::core::mem::transmute(phmetapict.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pdvaspect.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pncurrentscale.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetViewInfo<P0>(&self, dwobject: u32, hmetapict: isize, dvaspect: u32, ncurrentscale: i32, brelativetoorig: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetViewInfo)(::windows::core::Vtable::as_raw(self), dwobject, hmetapict, dvaspect, ncurrentscale, brelativetoorig.into()).ok()
    }
}
::windows::core::interface_hierarchy!(IOleUIObjInfoA, ::windows::core::IUnknown);
impl ::core::clone::Clone for IOleUIObjInfoA {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IOleUIObjInfoA {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleUIObjInfoA {}
impl ::core::fmt::Debug for IOleUIObjInfoA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleUIObjInfoA").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IOleUIObjInfoA {
    type Vtable = IOleUIObjInfoA_Vtbl;
}
unsafe impl ::windows::core::Interface for IOleUIObjInfoA {
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleUIObjInfoA_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetObjectInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwobject: u32, lpdwobjsize: *mut u32, lplpszlabel: *mut ::windows::core::PSTR, lplpsztype: *mut ::windows::core::PSTR, lplpszshorttype: *mut ::windows::core::PSTR, lplpszlocation: *mut ::windows::core::PSTR) -> ::windows::core::HRESULT,
    pub GetConvertInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwobject: u32, lpclassid: *mut ::windows::core::GUID, lpwformat: *mut u16, lpconvertdefaultclassid: *mut ::windows::core::GUID, lplpclsidexclude: *mut *mut ::windows::core::GUID, lpcclsidexclude: *mut u32) -> ::windows::core::HRESULT,
    pub ConvertObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwobject: u32, clsidnew: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetViewInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwobject: u32, phmetapict: *const isize, pdvaspect: *const u32, pncurrentscale: *const i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetViewInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwobject: u32, hmetapict: isize, dvaspect: u32, ncurrentscale: i32, brelativetoorig: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetViewInfo: usize,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IOleUIObjInfoW(::windows::core::IUnknown);
impl IOleUIObjInfoW {
    pub unsafe fn GetObjectInfo(&self, dwobject: u32, lpdwobjsize: *mut u32, lplpszlabel: ::core::option::Option<*mut ::windows::core::PWSTR>, lplpsztype: ::core::option::Option<*mut ::windows::core::PWSTR>, lplpszshorttype: ::core::option::Option<*mut ::windows::core::PWSTR>, lplpszlocation: ::core::option::Option<*mut ::windows::core::PWSTR>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetObjectInfo)(::windows::core::Vtable::as_raw(self), dwobject, lpdwobjsize, ::core::mem::transmute(lplpszlabel.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lplpsztype.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lplpszshorttype.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lplpszlocation.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetConvertInfo(&self, dwobject: u32, lpclassid: *mut ::windows::core::GUID, lpwformat: *mut u16, lpconvertdefaultclassid: *mut ::windows::core::GUID, lplpclsidexclude: *mut *mut ::windows::core::GUID, lpcclsidexclude: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetConvertInfo)(::windows::core::Vtable::as_raw(self), dwobject, lpclassid, lpwformat, lpconvertdefaultclassid, lplpclsidexclude, ::core::mem::transmute(lpcclsidexclude.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn ConvertObject(&self, dwobject: u32, clsidnew: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ConvertObject)(::windows::core::Vtable::as_raw(self), dwobject, clsidnew).ok()
    }
    pub unsafe fn GetViewInfo(&self, dwobject: u32, phmetapict: ::core::option::Option<*const isize>, pdvaspect: ::core::option::Option<*const u32>, pncurrentscale: ::core::option::Option<*const i32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetViewInfo)(::windows::core::Vtable::as_raw(self), dwobject, ::core::mem::transmute(phmetapict.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pdvaspect.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pncurrentscale.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetViewInfo<P0>(&self, dwobject: u32, hmetapict: isize, dvaspect: u32, ncurrentscale: i32, brelativetoorig: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetViewInfo)(::windows::core::Vtable::as_raw(self), dwobject, hmetapict, dvaspect, ncurrentscale, brelativetoorig.into()).ok()
    }
}
::windows::core::interface_hierarchy!(IOleUIObjInfoW, ::windows::core::IUnknown);
impl ::core::clone::Clone for IOleUIObjInfoW {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IOleUIObjInfoW {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleUIObjInfoW {}
impl ::core::fmt::Debug for IOleUIObjInfoW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleUIObjInfoW").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IOleUIObjInfoW {
    type Vtable = IOleUIObjInfoW_Vtbl;
}
unsafe impl ::windows::core::Interface for IOleUIObjInfoW {
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleUIObjInfoW_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetObjectInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwobject: u32, lpdwobjsize: *mut u32, lplpszlabel: *mut ::windows::core::PWSTR, lplpsztype: *mut ::windows::core::PWSTR, lplpszshorttype: *mut ::windows::core::PWSTR, lplpszlocation: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetConvertInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwobject: u32, lpclassid: *mut ::windows::core::GUID, lpwformat: *mut u16, lpconvertdefaultclassid: *mut ::windows::core::GUID, lplpclsidexclude: *mut *mut ::windows::core::GUID, lpcclsidexclude: *mut u32) -> ::windows::core::HRESULT,
    pub ConvertObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwobject: u32, clsidnew: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetViewInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwobject: u32, phmetapict: *const isize, pdvaspect: *const u32, pncurrentscale: *const i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetViewInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwobject: u32, hmetapict: isize, dvaspect: u32, ncurrentscale: i32, brelativetoorig: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetViewInfo: usize,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IOleUndoManager(::windows::core::IUnknown);
impl IOleUndoManager {
    pub unsafe fn Open<P0>(&self, ppuu: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IOleParentUndoUnit>>,
    {
        (::windows::core::Vtable::vtable(self).Open)(::windows::core::Vtable::as_raw(self), ppuu.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Close<P0, P1>(&self, ppuu: P0, fcommit: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IOleParentUndoUnit>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).Close)(::windows::core::Vtable::as_raw(self), ppuu.into().abi(), fcommit.into()).ok()
    }
    pub unsafe fn Add<P0>(&self, puu: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IOleUndoUnit>>,
    {
        (::windows::core::Vtable::vtable(self).Add)(::windows::core::Vtable::as_raw(self), puu.into().abi()).ok()
    }
    pub unsafe fn GetOpenParentState(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetOpenParentState)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DiscardFrom<P0>(&self, puu: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IOleUndoUnit>>,
    {
        (::windows::core::Vtable::vtable(self).DiscardFrom)(::windows::core::Vtable::as_raw(self), puu.into().abi()).ok()
    }
    pub unsafe fn UndoTo<P0>(&self, puu: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IOleUndoUnit>>,
    {
        (::windows::core::Vtable::vtable(self).UndoTo)(::windows::core::Vtable::as_raw(self), puu.into().abi()).ok()
    }
    pub unsafe fn RedoTo<P0>(&self, puu: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IOleUndoUnit>>,
    {
        (::windows::core::Vtable::vtable(self).RedoTo)(::windows::core::Vtable::as_raw(self), puu.into().abi()).ok()
    }
    pub unsafe fn EnumUndoable(&self) -> ::windows::core::Result<IEnumOleUndoUnits> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumUndoable)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumRedoable(&self) -> ::windows::core::Result<IEnumOleUndoUnits> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumRedoable)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetLastUndoDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetLastUndoDescription)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetLastRedoDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetLastRedoDescription)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enable<P0>(&self, fenable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).Enable)(::windows::core::Vtable::as_raw(self), fenable.into()).ok()
    }
}
::windows::core::interface_hierarchy!(IOleUndoManager, ::windows::core::IUnknown);
impl ::core::clone::Clone for IOleUndoManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IOleUndoManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleUndoManager {}
impl ::core::fmt::Debug for IOleUndoManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleUndoManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IOleUndoManager {
    type Vtable = IOleUndoManager_Vtbl;
}
unsafe impl ::windows::core::Interface for IOleUndoManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd001f200_ef97_11ce_9bc9_00aa00608e01);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleUndoManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppuu: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppuu: *mut ::core::ffi::c_void, fcommit: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Close: usize,
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puu: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetOpenParentState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwstate: *mut u32) -> ::windows::core::HRESULT,
    pub DiscardFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puu: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UndoTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puu: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RedoTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puu: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumUndoable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumRedoable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetLastUndoDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetLastRedoDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Enable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Enable: usize,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IOleUndoUnit(::windows::core::IUnknown);
impl IOleUndoUnit {
    pub unsafe fn Do<P0>(&self, pundomanager: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IOleUndoManager>>,
    {
        (::windows::core::Vtable::vtable(self).Do)(::windows::core::Vtable::as_raw(self), pundomanager.into().abi()).ok()
    }
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDescription)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetUnitType(&self, pclsid: *mut ::windows::core::GUID, plid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetUnitType)(::windows::core::Vtable::as_raw(self), pclsid, plid).ok()
    }
    pub unsafe fn OnNextAdd(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).OnNextAdd)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IOleUndoUnit, ::windows::core::IUnknown);
impl ::core::clone::Clone for IOleUndoUnit {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IOleUndoUnit {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleUndoUnit {}
impl ::core::fmt::Debug for IOleUndoUnit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleUndoUnit").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IOleUndoUnit {
    type Vtable = IOleUndoUnit_Vtbl;
}
unsafe impl ::windows::core::Interface for IOleUndoUnit {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x894ad3b0_ef97_11ce_9bc9_00aa00608e01);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleUndoUnit_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Do: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pundomanager: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetUnitType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID, plid: *mut i32) -> ::windows::core::HRESULT,
    pub OnNextAdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IOleWindow(::windows::core::IUnknown);
impl IOleWindow {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindow(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetWindow)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ContextSensitiveHelp<P0>(&self, fentermode: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).ContextSensitiveHelp)(::windows::core::Vtable::as_raw(self), fentermode.into()).ok()
    }
}
::windows::core::interface_hierarchy!(IOleWindow, ::windows::core::IUnknown);
impl ::core::clone::Clone for IOleWindow {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IOleWindow {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOleWindow {}
impl ::core::fmt::Debug for IOleWindow {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOleWindow").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IOleWindow {
    type Vtable = IOleWindow_Vtbl;
}
unsafe impl ::windows::core::Interface for IOleWindow {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000114_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOleWindow_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetWindow: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ContextSensitiveHelp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fentermode: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ContextSensitiveHelp: usize,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IParseDisplayName(::windows::core::IUnknown);
impl IParseDisplayName {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ParseDisplayName<P0, P1>(&self, pbc: P0, pszdisplayname: P1, pcheaten: *mut u32, ppmkout: *mut ::core::option::Option<super::Com::IMoniker>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IBindCtx>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).ParseDisplayName)(::windows::core::Vtable::as_raw(self), pbc.into().abi(), pszdisplayname.into().abi(), pcheaten, ::core::mem::transmute(ppmkout)).ok()
    }
}
::windows::core::interface_hierarchy!(IParseDisplayName, ::windows::core::IUnknown);
impl ::core::clone::Clone for IParseDisplayName {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IParseDisplayName {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IParseDisplayName {}
impl ::core::fmt::Debug for IParseDisplayName {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IParseDisplayName").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IParseDisplayName {
    type Vtable = IParseDisplayName_Vtbl;
}
unsafe impl ::windows::core::Interface for IParseDisplayName {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000011a_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IParseDisplayName_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub ParseDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbc: *mut ::core::ffi::c_void, pszdisplayname: ::windows::core::PCWSTR, pcheaten: *mut u32, ppmkout: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ParseDisplayName: usize,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IPerPropertyBrowsing(::windows::core::IUnknown);
impl IPerPropertyBrowsing {
    pub unsafe fn GetDisplayString(&self, dispid: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDisplayString)(::windows::core::Vtable::as_raw(self), dispid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn MapPropertyToPage(&self, dispid: i32) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MapPropertyToPage)(::windows::core::Vtable::as_raw(self), dispid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPredefinedStrings(&self, dispid: i32, pcastringsout: *mut CALPOLESTR, pcacookiesout: *mut CADWORD) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetPredefinedStrings)(::windows::core::Vtable::as_raw(self), dispid, pcastringsout, pcacookiesout).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetPredefinedValue(&self, dispid: i32, dwcookie: u32) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPredefinedValue)(::windows::core::Vtable::as_raw(self), dispid, dwcookie, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IPerPropertyBrowsing, ::windows::core::IUnknown);
impl ::core::clone::Clone for IPerPropertyBrowsing {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPerPropertyBrowsing {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPerPropertyBrowsing {}
impl ::core::fmt::Debug for IPerPropertyBrowsing {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPerPropertyBrowsing").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IPerPropertyBrowsing {
    type Vtable = IPerPropertyBrowsing_Vtbl;
}
unsafe impl ::windows::core::Interface for IPerPropertyBrowsing {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x376bd3aa_3845_101b_84ed_08002b2ec713);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerPropertyBrowsing_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetDisplayString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispid: i32, pbstr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MapPropertyToPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispid: i32, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetPredefinedStrings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispid: i32, pcastringsout: *mut CALPOLESTR, pcacookiesout: *mut CADWORD) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetPredefinedValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispid: i32, dwcookie: u32, pvarout: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetPredefinedValue: usize,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IPersistPropertyBag(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IPersistPropertyBag {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetClassID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetClassID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn InitNew(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).InitNew)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn Load<P0, P1>(&self, ppropbag: P0, perrorlog: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::StructuredStorage::IPropertyBag>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Com::IErrorLog>>,
    {
        (::windows::core::Vtable::vtable(self).Load)(::windows::core::Vtable::as_raw(self), ppropbag.into().abi(), perrorlog.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn Save<P0, P1, P2>(&self, ppropbag: P0, fcleardirty: P1, fsaveallproperties: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::StructuredStorage::IPropertyBag>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
        P2: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).Save)(::windows::core::Vtable::as_raw(self), ppropbag.into().abi(), fcleardirty.into(), fsaveallproperties.into()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IPersistPropertyBag, ::windows::core::IUnknown, super::Com::IPersist);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IPersistPropertyBag {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPersistPropertyBag {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPersistPropertyBag {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPersistPropertyBag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPersistPropertyBag").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IPersistPropertyBag {
    type Vtable = IPersistPropertyBag_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IPersistPropertyBag {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x37d84f60_42cb_11ce_8135_00aa004bb851);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPersistPropertyBag_Vtbl {
    pub base__: super::Com::IPersist_Vtbl,
    pub InitNew: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub Load: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppropbag: *mut ::core::ffi::c_void, perrorlog: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    Load: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppropbag: *mut ::core::ffi::c_void, fcleardirty: super::super::Foundation::BOOL, fsaveallproperties: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    Save: usize,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IPersistPropertyBag2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IPersistPropertyBag2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetClassID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetClassID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn InitNew(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).InitNew)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn Load<P0, P1>(&self, ppropbag: P0, perrlog: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::StructuredStorage::IPropertyBag2>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Com::IErrorLog>>,
    {
        (::windows::core::Vtable::vtable(self).Load)(::windows::core::Vtable::as_raw(self), ppropbag.into().abi(), perrlog.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn Save<P0, P1, P2>(&self, ppropbag: P0, fcleardirty: P1, fsaveallproperties: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::StructuredStorage::IPropertyBag2>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
        P2: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).Save)(::windows::core::Vtable::as_raw(self), ppropbag.into().abi(), fcleardirty.into(), fsaveallproperties.into()).ok()
    }
    pub unsafe fn IsDirty(&self) -> ::windows::core::HRESULT {
        (::windows::core::Vtable::vtable(self).IsDirty)(::windows::core::Vtable::as_raw(self))
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IPersistPropertyBag2, ::windows::core::IUnknown, super::Com::IPersist);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IPersistPropertyBag2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPersistPropertyBag2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPersistPropertyBag2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPersistPropertyBag2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPersistPropertyBag2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IPersistPropertyBag2 {
    type Vtable = IPersistPropertyBag2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IPersistPropertyBag2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x22f55881_280b_11d0_a8a9_00a0c90c2004);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPersistPropertyBag2_Vtbl {
    pub base__: super::Com::IPersist_Vtbl,
    pub InitNew: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub Load: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppropbag: *mut ::core::ffi::c_void, perrlog: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    Load: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppropbag: *mut ::core::ffi::c_void, fcleardirty: super::super::Foundation::BOOL, fsaveallproperties: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    Save: usize,
    pub IsDirty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IPicture(::windows::core::IUnknown);
impl IPicture {
    pub unsafe fn Handle(&self) -> ::windows::core::Result<OLE_HANDLE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Handle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn hPal(&self) -> ::windows::core::Result<OLE_HANDLE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).hPal)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Type(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Type)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Width(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Width)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Height(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Height)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn Render<P0>(&self, hdc: P0, x: i32, y: i32, cx: i32, cy: i32, xsrc: i32, ysrc: i32, cxsrc: i32, cysrc: i32, prcwbounds: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
    {
        (::windows::core::Vtable::vtable(self).Render)(::windows::core::Vtable::as_raw(self), hdc.into(), x, y, cx, cy, xsrc, ysrc, cxsrc, cysrc, prcwbounds).ok()
    }
    pub unsafe fn set_hPal<P0>(&self, hpal: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<OLE_HANDLE>,
    {
        (::windows::core::Vtable::vtable(self).set_hPal)(::windows::core::Vtable::as_raw(self), hpal.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn CurDC(&self) -> ::windows::core::Result<super::super::Graphics::Gdi::HDC> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurDC)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn SelectPicture<P0>(&self, hdcin: P0, phdcout: *mut super::super::Graphics::Gdi::HDC, phbmpout: *mut OLE_HANDLE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
    {
        (::windows::core::Vtable::vtable(self).SelectPicture)(::windows::core::Vtable::as_raw(self), hdcin.into(), phdcout, phbmpout).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn KeepOriginalFormat(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).KeepOriginalFormat)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetKeepOriginalFormat<P0>(&self, keep: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetKeepOriginalFormat)(::windows::core::Vtable::as_raw(self), keep.into()).ok()
    }
    pub unsafe fn PictureChanged(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).PictureChanged)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SaveAsFile<P0, P1>(&self, pstream: P0, fsavememcopy: P1) -> ::windows::core::Result<i32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IStream>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).SaveAsFile)(::windows::core::Vtable::as_raw(self), pstream.into().abi(), fsavememcopy.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Attributes(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Attributes)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IPicture, ::windows::core::IUnknown);
impl ::core::clone::Clone for IPicture {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPicture {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPicture {}
impl ::core::fmt::Debug for IPicture {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPicture").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IPicture {
    type Vtable = IPicture_Vtbl;
}
unsafe impl ::windows::core::Interface for IPicture {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7bf80980_bf32_101a_8bbb_00aa00300cab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPicture_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Handle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phandle: *mut OLE_HANDLE) -> ::windows::core::HRESULT,
    pub hPal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phpal: *mut OLE_HANDLE) -> ::windows::core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut i16) -> ::windows::core::HRESULT,
    pub Width: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwidth: *mut i32) -> ::windows::core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pheight: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub Render: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hdc: super::super::Graphics::Gdi::HDC, x: i32, y: i32, cx: i32, cy: i32, xsrc: i32, ysrc: i32, cxsrc: i32, cysrc: i32, prcwbounds: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))]
    Render: usize,
    pub set_hPal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hpal: OLE_HANDLE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub CurDC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phdc: *mut super::super::Graphics::Gdi::HDC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    CurDC: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub SelectPicture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hdcin: super::super::Graphics::Gdi::HDC, phdcout: *mut super::super::Graphics::Gdi::HDC, phbmpout: *mut OLE_HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    SelectPicture: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub KeepOriginalFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pkeep: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    KeepOriginalFormat: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetKeepOriginalFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keep: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetKeepOriginalFormat: usize,
    pub PictureChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub SaveAsFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void, fsavememcopy: super::super::Foundation::BOOL, pcbsize: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    SaveAsFile: usize,
    pub Attributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwattr: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IPicture2(::windows::core::IUnknown);
impl IPicture2 {
    pub unsafe fn Handle(&self) -> ::windows::core::Result<usize> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Handle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn hPal(&self) -> ::windows::core::Result<usize> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).hPal)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Type(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Type)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Width(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Width)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Height(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Height)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn Render<P0>(&self, hdc: P0, x: i32, y: i32, cx: i32, cy: i32, xsrc: i32, ysrc: i32, cxsrc: i32, cysrc: i32, prcwbounds: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
    {
        (::windows::core::Vtable::vtable(self).Render)(::windows::core::Vtable::as_raw(self), hdc.into(), x, y, cx, cy, xsrc, ysrc, cxsrc, cysrc, prcwbounds).ok()
    }
    pub unsafe fn set_hPal(&self, hpal: usize) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).set_hPal)(::windows::core::Vtable::as_raw(self), hpal).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn CurDC(&self) -> ::windows::core::Result<super::super::Graphics::Gdi::HDC> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CurDC)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn SelectPicture<P0>(&self, hdcin: P0, phdcout: *mut super::super::Graphics::Gdi::HDC, phbmpout: *mut usize) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
    {
        (::windows::core::Vtable::vtable(self).SelectPicture)(::windows::core::Vtable::as_raw(self), hdcin.into(), phdcout, phbmpout).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn KeepOriginalFormat(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).KeepOriginalFormat)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetKeepOriginalFormat<P0>(&self, keep: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetKeepOriginalFormat)(::windows::core::Vtable::as_raw(self), keep.into()).ok()
    }
    pub unsafe fn PictureChanged(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).PictureChanged)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SaveAsFile<P0, P1>(&self, pstream: P0, fsavememcopy: P1) -> ::windows::core::Result<i32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IStream>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).SaveAsFile)(::windows::core::Vtable::as_raw(self), pstream.into().abi(), fsavememcopy.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Attributes(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Attributes)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IPicture2, ::windows::core::IUnknown);
impl ::core::clone::Clone for IPicture2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPicture2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPicture2 {}
impl ::core::fmt::Debug for IPicture2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPicture2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IPicture2 {
    type Vtable = IPicture2_Vtbl;
}
unsafe impl ::windows::core::Interface for IPicture2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf5185dd8_2012_4b0b_aad9_f052c6bd482b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPicture2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Handle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phandle: *mut usize) -> ::windows::core::HRESULT,
    pub hPal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phpal: *mut usize) -> ::windows::core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut i16) -> ::windows::core::HRESULT,
    pub Width: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwidth: *mut i32) -> ::windows::core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pheight: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub Render: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hdc: super::super::Graphics::Gdi::HDC, x: i32, y: i32, cx: i32, cy: i32, xsrc: i32, ysrc: i32, cxsrc: i32, cysrc: i32, prcwbounds: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))]
    Render: usize,
    pub set_hPal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hpal: usize) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub CurDC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phdc: *mut super::super::Graphics::Gdi::HDC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    CurDC: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub SelectPicture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hdcin: super::super::Graphics::Gdi::HDC, phdcout: *mut super::super::Graphics::Gdi::HDC, phbmpout: *mut usize) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    SelectPicture: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub KeepOriginalFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pkeep: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    KeepOriginalFormat: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetKeepOriginalFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keep: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetKeepOriginalFormat: usize,
    pub PictureChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub SaveAsFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void, fsavememcopy: super::super::Foundation::BOOL, pcbsize: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    SaveAsFile: usize,
    pub Attributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwattr: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IPictureDisp(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IPictureDisp {}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IPictureDisp, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IPictureDisp {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPictureDisp {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPictureDisp {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPictureDisp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPictureDisp").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IPictureDisp {
    type Vtable = IPictureDisp_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IPictureDisp {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7bf80981_bf32_101a_8bbb_00aa00300cab);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPictureDisp_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IPointerInactive(::windows::core::IUnknown);
impl IPointerInactive {
    pub unsafe fn GetActivationPolicy(&self) -> ::windows::core::Result<POINTERINACTIVE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetActivationPolicy)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnInactiveMouseMove(&self, prectbounds: *const super::super::Foundation::RECT, x: i32, y: i32, grfkeystate: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).OnInactiveMouseMove)(::windows::core::Vtable::as_raw(self), prectbounds, x, y, grfkeystate).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnInactiveSetCursor<P0>(&self, prectbounds: *const super::super::Foundation::RECT, x: i32, y: i32, dwmousemsg: u32, fsetalways: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).OnInactiveSetCursor)(::windows::core::Vtable::as_raw(self), prectbounds, x, y, dwmousemsg, fsetalways.into()).ok()
    }
}
::windows::core::interface_hierarchy!(IPointerInactive, ::windows::core::IUnknown);
impl ::core::clone::Clone for IPointerInactive {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPointerInactive {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPointerInactive {}
impl ::core::fmt::Debug for IPointerInactive {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPointerInactive").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IPointerInactive {
    type Vtable = IPointerInactive_Vtbl;
}
unsafe impl ::windows::core::Interface for IPointerInactive {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55980ba0_35aa_11cf_b671_00aa004cd6d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerInactive_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetActivationPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwpolicy: *mut POINTERINACTIVE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub OnInactiveMouseMove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prectbounds: *const super::super::Foundation::RECT, x: i32, y: i32, grfkeystate: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnInactiveMouseMove: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnInactiveSetCursor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prectbounds: *const super::super::Foundation::RECT, x: i32, y: i32, dwmousemsg: u32, fsetalways: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnInactiveSetCursor: usize,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IPrint(::windows::core::IUnknown);
impl IPrint {
    pub unsafe fn SetInitialPageNum(&self, nfirstpage: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetInitialPageNum)(::windows::core::Vtable::as_raw(self), nfirstpage).ok()
    }
    pub unsafe fn GetPageInfo(&self, pnfirstpage: *mut i32, pcpages: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetPageInfo)(::windows::core::Vtable::as_raw(self), pnfirstpage, pcpages).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn Print<P0>(&self, grfflags: u32, pptd: *mut *mut super::Com::DVTARGETDEVICE, pppageset: *mut *mut PAGESET, pstgmoptions: *mut super::Com::STGMEDIUM, pcallback: P0, nfirstpage: i32, pcpagesprinted: *mut i32, pnlastpage: *mut i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IContinueCallback>>,
    {
        (::windows::core::Vtable::vtable(self).Print)(::windows::core::Vtable::as_raw(self), grfflags, pptd, pppageset, pstgmoptions, pcallback.into().abi(), nfirstpage, pcpagesprinted, pnlastpage).ok()
    }
}
::windows::core::interface_hierarchy!(IPrint, ::windows::core::IUnknown);
impl ::core::clone::Clone for IPrint {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPrint {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrint {}
impl ::core::fmt::Debug for IPrint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrint").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IPrint {
    type Vtable = IPrint_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrint {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb722bcc9_4e68_101b_a2bc_00aa00404770);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrint_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetInitialPageNum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nfirstpage: i32) -> ::windows::core::HRESULT,
    pub GetPageInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnfirstpage: *mut i32, pcpages: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub Print: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, grfflags: u32, pptd: *mut *mut super::Com::DVTARGETDEVICE, pppageset: *mut *mut PAGESET, pstgmoptions: *mut super::Com::STGMEDIUM, pcallback: *mut ::core::ffi::c_void, nfirstpage: i32, pcpagesprinted: *mut i32, pnlastpage: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))]
    Print: usize,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IPropertyNotifySink(::windows::core::IUnknown);
impl IPropertyNotifySink {
    pub unsafe fn OnChanged(&self, dispid: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).OnChanged)(::windows::core::Vtable::as_raw(self), dispid).ok()
    }
    pub unsafe fn OnRequestEdit(&self, dispid: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).OnRequestEdit)(::windows::core::Vtable::as_raw(self), dispid).ok()
    }
}
::windows::core::interface_hierarchy!(IPropertyNotifySink, ::windows::core::IUnknown);
impl ::core::clone::Clone for IPropertyNotifySink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPropertyNotifySink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyNotifySink {}
impl ::core::fmt::Debug for IPropertyNotifySink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyNotifySink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IPropertyNotifySink {
    type Vtable = IPropertyNotifySink_Vtbl;
}
unsafe impl ::windows::core::Interface for IPropertyNotifySink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9bfbbc02_eff1_101a_84ed_00aa00341d07);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyNotifySink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispid: i32) -> ::windows::core::HRESULT,
    pub OnRequestEdit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispid: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IPropertyPage(::windows::core::IUnknown);
impl IPropertyPage {
    pub unsafe fn SetPageSite<P0>(&self, ppagesite: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IPropertyPageSite>>,
    {
        (::windows::core::Vtable::vtable(self).SetPageSite)(::windows::core::Vtable::as_raw(self), ppagesite.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Activate<P0, P1>(&self, hwndparent: P0, prect: *const super::super::Foundation::RECT, bmodal: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).Activate)(::windows::core::Vtable::as_raw(self), hwndparent.into(), prect, bmodal.into()).ok()
    }
    pub unsafe fn Deactivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Deactivate)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPageInfo(&self) -> ::windows::core::Result<PROPPAGEINFO> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPageInfo)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetObjects(&self, ppunk: &[::windows::core::IUnknown]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetObjects)(::windows::core::Vtable::as_raw(self), ppunk.len() as _, ::core::mem::transmute(ppunk.as_ptr())).ok()
    }
    pub unsafe fn Show(&self, ncmdshow: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Show)(::windows::core::Vtable::as_raw(self), ncmdshow).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Move(&self, prect: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Move)(::windows::core::Vtable::as_raw(self), prect).ok()
    }
    pub unsafe fn IsPageDirty(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).IsPageDirty)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Apply(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Apply)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Help<P0>(&self, pszhelpdir: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).Help)(::windows::core::Vtable::as_raw(self), pszhelpdir.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn TranslateAccelerator(&self, pmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).TranslateAccelerator)(::windows::core::Vtable::as_raw(self), pmsg).ok()
    }
}
::windows::core::interface_hierarchy!(IPropertyPage, ::windows::core::IUnknown);
impl ::core::clone::Clone for IPropertyPage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPropertyPage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyPage {}
impl ::core::fmt::Debug for IPropertyPage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyPage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IPropertyPage {
    type Vtable = IPropertyPage_Vtbl;
}
unsafe impl ::windows::core::Interface for IPropertyPage {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb196b28d_bab4_101a_b69c_00aa00341d07);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyPage_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetPageSite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppagesite: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Activate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, prect: *const super::super::Foundation::RECT, bmodal: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Activate: usize,
    pub Deactivate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPageInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppageinfo: *mut PROPPAGEINFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPageInfo: usize,
    pub SetObjects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cobjects: u32, ppunk: *const *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Show: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ncmdshow: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Move: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Move: usize,
    pub IsPageDirty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Apply: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Help: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszhelpdir: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub TranslateAccelerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))]
    TranslateAccelerator: usize,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IPropertyPage2(::windows::core::IUnknown);
impl IPropertyPage2 {
    pub unsafe fn SetPageSite<P0>(&self, ppagesite: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IPropertyPageSite>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPageSite)(::windows::core::Vtable::as_raw(self), ppagesite.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Activate<P0, P1>(&self, hwndparent: P0, prect: *const super::super::Foundation::RECT, bmodal: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.Activate)(::windows::core::Vtable::as_raw(self), hwndparent.into(), prect, bmodal.into()).ok()
    }
    pub unsafe fn Deactivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Deactivate)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPageInfo(&self) -> ::windows::core::Result<PROPPAGEINFO> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPageInfo)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetObjects(&self, ppunk: &[::windows::core::IUnknown]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetObjects)(::windows::core::Vtable::as_raw(self), ppunk.len() as _, ::core::mem::transmute(ppunk.as_ptr())).ok()
    }
    pub unsafe fn Show(&self, ncmdshow: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Show)(::windows::core::Vtable::as_raw(self), ncmdshow).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Move(&self, prect: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Move)(::windows::core::Vtable::as_raw(self), prect).ok()
    }
    pub unsafe fn IsPageDirty(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.IsPageDirty)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Apply(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Apply)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Help<P0>(&self, pszhelpdir: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Help)(::windows::core::Vtable::as_raw(self), pszhelpdir.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn TranslateAccelerator(&self, pmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.TranslateAccelerator)(::windows::core::Vtable::as_raw(self), pmsg).ok()
    }
    pub unsafe fn EditProperty(&self, dispid: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).EditProperty)(::windows::core::Vtable::as_raw(self), dispid).ok()
    }
}
::windows::core::interface_hierarchy!(IPropertyPage2, ::windows::core::IUnknown, IPropertyPage);
impl ::core::clone::Clone for IPropertyPage2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPropertyPage2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyPage2 {}
impl ::core::fmt::Debug for IPropertyPage2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyPage2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IPropertyPage2 {
    type Vtable = IPropertyPage2_Vtbl;
}
unsafe impl ::windows::core::Interface for IPropertyPage2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x01e44665_24ac_101b_84ed_08002b2ec713);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyPage2_Vtbl {
    pub base__: IPropertyPage_Vtbl,
    pub EditProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispid: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IPropertyPageSite(::windows::core::IUnknown);
impl IPropertyPageSite {
    pub unsafe fn OnStatusChange(&self, dwflags: PROPPAGESTATUS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).OnStatusChange)(::windows::core::Vtable::as_raw(self), dwflags).ok()
    }
    pub unsafe fn GetLocaleID(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetLocaleID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPageContainer(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPageContainer)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn TranslateAccelerator(&self, pmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).TranslateAccelerator)(::windows::core::Vtable::as_raw(self), pmsg).ok()
    }
}
::windows::core::interface_hierarchy!(IPropertyPageSite, ::windows::core::IUnknown);
impl ::core::clone::Clone for IPropertyPageSite {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPropertyPageSite {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyPageSite {}
impl ::core::fmt::Debug for IPropertyPageSite {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyPageSite").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IPropertyPageSite {
    type Vtable = IPropertyPageSite_Vtbl;
}
unsafe impl ::windows::core::Interface for IPropertyPageSite {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb196b28c_bab4_101a_b69c_00aa00341d07);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyPageSite_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnStatusChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: PROPPAGESTATUS) -> ::windows::core::HRESULT,
    pub GetLocaleID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plocaleid: *mut u32) -> ::windows::core::HRESULT,
    pub GetPageContainer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub TranslateAccelerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))]
    TranslateAccelerator: usize,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IProtectFocus(::windows::core::IUnknown);
impl IProtectFocus {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AllowFocusChange(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AllowFocusChange)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IProtectFocus, ::windows::core::IUnknown);
impl ::core::clone::Clone for IProtectFocus {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IProtectFocus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProtectFocus {}
impl ::core::fmt::Debug for IProtectFocus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProtectFocus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IProtectFocus {
    type Vtable = IProtectFocus_Vtbl;
}
unsafe impl ::windows::core::Interface for IProtectFocus {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd81f90a3_8156_44f7_ad28_5abb87003274);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectFocus_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub AllowFocusChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfallow: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AllowFocusChange: usize,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IProtectedModeMenuServices(::windows::core::IUnknown);
impl IProtectedModeMenuServices {
    #[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn CreateMenu(&self) -> ::windows::core::Result<super::super::UI::WindowsAndMessaging::HMENU> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateMenu)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn LoadMenu<P0, P1>(&self, pszmodulename: P0, pszmenuname: P1) -> ::windows::core::Result<super::super::UI::WindowsAndMessaging::HMENU>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LoadMenu)(::windows::core::Vtable::as_raw(self), pszmodulename.into().abi(), pszmenuname.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn LoadMenuID<P0>(&self, pszmodulename: P0, wresourceid: u16) -> ::windows::core::Result<super::super::UI::WindowsAndMessaging::HMENU>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LoadMenuID)(::windows::core::Vtable::as_raw(self), pszmodulename.into().abi(), wresourceid, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IProtectedModeMenuServices, ::windows::core::IUnknown);
impl ::core::clone::Clone for IProtectedModeMenuServices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IProtectedModeMenuServices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProtectedModeMenuServices {}
impl ::core::fmt::Debug for IProtectedModeMenuServices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProtectedModeMenuServices").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IProtectedModeMenuServices {
    type Vtable = IProtectedModeMenuServices_Vtbl;
}
unsafe impl ::windows::core::Interface for IProtectedModeMenuServices {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73c105ee_9dff_4a07_b83c_7eff290c266e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectedModeMenuServices_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub CreateMenu: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phmenu: *mut super::super::UI::WindowsAndMessaging::HMENU) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    CreateMenu: usize,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub LoadMenu: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszmodulename: ::windows::core::PCWSTR, pszmenuname: ::windows::core::PCWSTR, phmenu: *mut super::super::UI::WindowsAndMessaging::HMENU) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    LoadMenu: usize,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub LoadMenuID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszmodulename: ::windows::core::PCWSTR, wresourceid: u16, phmenu: *mut super::super::UI::WindowsAndMessaging::HMENU) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    LoadMenuID: usize,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IProvideClassInfo(::windows::core::IUnknown);
impl IProvideClassInfo {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetClassInfo(&self) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetClassInfo)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IProvideClassInfo, ::windows::core::IUnknown);
impl ::core::clone::Clone for IProvideClassInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IProvideClassInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProvideClassInfo {}
impl ::core::fmt::Debug for IProvideClassInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProvideClassInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IProvideClassInfo {
    type Vtable = IProvideClassInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IProvideClassInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb196b283_bab4_101a_b69c_00aa00341d07);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProvideClassInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetClassInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppti: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetClassInfo: usize,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IProvideClassInfo2(::windows::core::IUnknown);
impl IProvideClassInfo2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetClassInfo(&self) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetClassInfo)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetGUID(&self, dwguidkind: u32) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetGUID)(::windows::core::Vtable::as_raw(self), dwguidkind, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IProvideClassInfo2, ::windows::core::IUnknown, IProvideClassInfo);
impl ::core::clone::Clone for IProvideClassInfo2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IProvideClassInfo2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProvideClassInfo2 {}
impl ::core::fmt::Debug for IProvideClassInfo2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProvideClassInfo2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IProvideClassInfo2 {
    type Vtable = IProvideClassInfo2_Vtbl;
}
unsafe impl ::windows::core::Interface for IProvideClassInfo2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6bc3ac0_dbaa_11ce_9de3_00aa004bb851);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProvideClassInfo2_Vtbl {
    pub base__: IProvideClassInfo_Vtbl,
    pub GetGUID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwguidkind: u32, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IProvideMultipleClassInfo(::windows::core::IUnknown);
impl IProvideMultipleClassInfo {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetClassInfo(&self) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetClassInfo)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetGUID(&self, dwguidkind: u32) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetGUID)(::windows::core::Vtable::as_raw(self), dwguidkind, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetMultiTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetMultiTypeInfoCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetInfoOfIndex(&self, iti: u32, dwflags: MULTICLASSINFO_FLAGS, ppticoclass: *mut ::core::option::Option<super::Com::ITypeInfo>, pdwtiflags: *mut u32, pcdispidreserved: *mut u32, piidprimary: *mut ::windows::core::GUID, piidsource: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetInfoOfIndex)(::windows::core::Vtable::as_raw(self), iti, dwflags, ::core::mem::transmute(ppticoclass), pdwtiflags, pcdispidreserved, piidprimary, piidsource).ok()
    }
}
::windows::core::interface_hierarchy!(IProvideMultipleClassInfo, ::windows::core::IUnknown, IProvideClassInfo, IProvideClassInfo2);
impl ::core::clone::Clone for IProvideMultipleClassInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IProvideMultipleClassInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProvideMultipleClassInfo {}
impl ::core::fmt::Debug for IProvideMultipleClassInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProvideMultipleClassInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IProvideMultipleClassInfo {
    type Vtable = IProvideMultipleClassInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IProvideMultipleClassInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa7aba9c1_8983_11cf_8f20_00805f2cd064);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProvideMultipleClassInfo_Vtbl {
    pub base__: IProvideClassInfo2_Vtbl,
    pub GetMultiTypeInfoCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcti: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetInfoOfIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iti: u32, dwflags: MULTICLASSINFO_FLAGS, ppticoclass: *mut *mut ::core::ffi::c_void, pdwtiflags: *mut u32, pcdispidreserved: *mut u32, piidprimary: *mut ::windows::core::GUID, piidsource: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetInfoOfIndex: usize,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IProvideRuntimeContext(::windows::core::IUnknown);
impl IProvideRuntimeContext {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCurrentSourceContext(&self, pdwcontext: *mut usize, pfexecutingglobalcode: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetCurrentSourceContext)(::windows::core::Vtable::as_raw(self), pdwcontext, pfexecutingglobalcode).ok()
    }
}
::windows::core::interface_hierarchy!(IProvideRuntimeContext, ::windows::core::IUnknown);
impl ::core::clone::Clone for IProvideRuntimeContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IProvideRuntimeContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProvideRuntimeContext {}
impl ::core::fmt::Debug for IProvideRuntimeContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProvideRuntimeContext").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IProvideRuntimeContext {
    type Vtable = IProvideRuntimeContext_Vtbl;
}
unsafe impl ::windows::core::Interface for IProvideRuntimeContext {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10e2414a_ec59_49d2_bc51_5add2c36febc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProvideRuntimeContext_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetCurrentSourceContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcontext: *mut usize, pfexecutingglobalcode: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetCurrentSourceContext: usize,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IQuickActivate(::windows::core::IUnknown);
impl IQuickActivate {
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn QuickActivate(&self, pqacontainer: *const QACONTAINER, pqacontrol: *mut QACONTROL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).QuickActivate)(::windows::core::Vtable::as_raw(self), pqacontainer, pqacontrol).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetContentExtent(&self, psizel: *const super::super::Foundation::SIZE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetContentExtent)(::windows::core::Vtable::as_raw(self), psizel).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetContentExtent(&self) -> ::windows::core::Result<super::super::Foundation::SIZE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetContentExtent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IQuickActivate, ::windows::core::IUnknown);
impl ::core::clone::Clone for IQuickActivate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IQuickActivate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IQuickActivate {}
impl ::core::fmt::Debug for IQuickActivate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IQuickActivate").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IQuickActivate {
    type Vtable = IQuickActivate_Vtbl;
}
unsafe impl ::windows::core::Interface for IQuickActivate {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf51ed10_62fe_11cf_bf86_00a0c9034836);
}
#[repr(C)]
#[doc(hidden)]
pub struct IQuickActivate_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub QuickActivate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pqacontainer: *const QACONTAINER, pqacontrol: *mut QACONTROL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))]
    QuickActivate: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetContentExtent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psizel: *const super::super::Foundation::SIZE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetContentExtent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetContentExtent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psizel: *mut super::super::Foundation::SIZE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetContentExtent: usize,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IRecordInfo(::windows::core::IUnknown);
impl IRecordInfo {
    pub unsafe fn RecordInit(&self, pvnew: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RecordInit)(::windows::core::Vtable::as_raw(self), pvnew).ok()
    }
    pub unsafe fn RecordClear(&self, pvexisting: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RecordClear)(::windows::core::Vtable::as_raw(self), pvexisting).ok()
    }
    pub unsafe fn RecordCopy(&self, pvexisting: *const ::core::ffi::c_void, pvnew: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RecordCopy)(::windows::core::Vtable::as_raw(self), pvexisting, pvnew).ok()
    }
    pub unsafe fn GetGuid(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetGuid)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSize(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSize)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetTypeInfo)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetField<P0>(&self, pvdata: *const ::core::ffi::c_void, szfieldname: P0) -> ::windows::core::Result<super::Com::VARIANT>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetField)(::windows::core::Vtable::as_raw(self), pvdata, szfieldname.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetFieldNoCopy<P0>(&self, pvdata: *const ::core::ffi::c_void, szfieldname: P0, pvarfield: *mut super::Com::VARIANT, ppvdatacarray: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).GetFieldNoCopy)(::windows::core::Vtable::as_raw(self), pvdata, szfieldname.into().abi(), pvarfield, ppvdatacarray).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn PutField<P0>(&self, wflags: super::Com::INVOKEKIND, pvdata: *mut ::core::ffi::c_void, szfieldname: P0, pvarfield: *const super::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).PutField)(::windows::core::Vtable::as_raw(self), wflags, pvdata, szfieldname.into().abi(), pvarfield).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn PutFieldNoCopy<P0>(&self, wflags: super::Com::INVOKEKIND, pvdata: *mut ::core::ffi::c_void, szfieldname: P0, pvarfield: *const super::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).PutFieldNoCopy)(::windows::core::Vtable::as_raw(self), wflags, pvdata, szfieldname.into().abi(), pvarfield).ok()
    }
    pub unsafe fn GetFieldNames(&self, pcnames: *mut u32, rgbstrnames: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetFieldNames)(::windows::core::Vtable::as_raw(self), pcnames, ::core::mem::transmute(rgbstrnames)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsMatchingType<P0>(&self, precordinfo: P0) -> super::super::Foundation::BOOL
    where
        P0: ::std::convert::Into<::windows::core::InParam<IRecordInfo>>,
    {
        (::windows::core::Vtable::vtable(self).IsMatchingType)(::windows::core::Vtable::as_raw(self), precordinfo.into().abi())
    }
    pub unsafe fn RecordCreate(&self) -> *mut ::core::ffi::c_void {
        (::windows::core::Vtable::vtable(self).RecordCreate)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn RecordCreateCopy(&self, pvsource: *const ::core::ffi::c_void, ppvdest: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RecordCreateCopy)(::windows::core::Vtable::as_raw(self), pvsource, ppvdest).ok()
    }
    pub unsafe fn RecordDestroy(&self, pvrecord: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RecordDestroy)(::windows::core::Vtable::as_raw(self), pvrecord).ok()
    }
}
::windows::core::interface_hierarchy!(IRecordInfo, ::windows::core::IUnknown);
impl ::core::clone::Clone for IRecordInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRecordInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRecordInfo {}
impl ::core::fmt::Debug for IRecordInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRecordInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IRecordInfo {
    type Vtable = IRecordInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IRecordInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000002f_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRecordInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub RecordInit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvnew: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RecordClear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvexisting: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RecordCopy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvexisting: *const ::core::ffi::c_void, pvnew: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbsize: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetTypeInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptypeinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetTypeInfo: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetField: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvdata: *const ::core::ffi::c_void, szfieldname: ::windows::core::PCWSTR, pvarfield: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetField: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetFieldNoCopy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvdata: *const ::core::ffi::c_void, szfieldname: ::windows::core::PCWSTR, pvarfield: *mut super::Com::VARIANT, ppvdatacarray: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetFieldNoCopy: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub PutField: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wflags: super::Com::INVOKEKIND, pvdata: *mut ::core::ffi::c_void, szfieldname: ::windows::core::PCWSTR, pvarfield: *const super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    PutField: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub PutFieldNoCopy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wflags: super::Com::INVOKEKIND, pvdata: *mut ::core::ffi::c_void, szfieldname: ::windows::core::PCWSTR, pvarfield: *const super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    PutFieldNoCopy: usize,
    pub GetFieldNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcnames: *mut u32, rgbstrnames: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsMatchingType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, precordinfo: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsMatchingType: usize,
    pub RecordCreate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
    pub RecordCreateCopy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvsource: *const ::core::ffi::c_void, ppvdest: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RecordDestroy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvrecord: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct ISimpleFrameSite(::windows::core::IUnknown);
impl ISimpleFrameSite {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PreMessageFilter<P0, P1, P2>(&self, hwnd: P0, msg: u32, wp: P1, lp: P2, plresult: *mut super::super::Foundation::LRESULT, pdwcookie: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<super::super::Foundation::WPARAM>,
        P2: ::std::convert::Into<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Vtable::vtable(self).PreMessageFilter)(::windows::core::Vtable::as_raw(self), hwnd.into(), msg, wp.into(), lp.into(), plresult, pdwcookie).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PostMessageFilter<P0, P1, P2>(&self, hwnd: P0, msg: u32, wp: P1, lp: P2, plresult: *mut super::super::Foundation::LRESULT, dwcookie: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<super::super::Foundation::WPARAM>,
        P2: ::std::convert::Into<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Vtable::vtable(self).PostMessageFilter)(::windows::core::Vtable::as_raw(self), hwnd.into(), msg, wp.into(), lp.into(), plresult, dwcookie).ok()
    }
}
::windows::core::interface_hierarchy!(ISimpleFrameSite, ::windows::core::IUnknown);
impl ::core::clone::Clone for ISimpleFrameSite {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISimpleFrameSite {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISimpleFrameSite {}
impl ::core::fmt::Debug for ISimpleFrameSite {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISimpleFrameSite").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for ISimpleFrameSite {
    type Vtable = ISimpleFrameSite_Vtbl;
}
unsafe impl ::windows::core::Interface for ISimpleFrameSite {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x742b0e01_14e6_101b_914e_00aa00300cab);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimpleFrameSite_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub PreMessageFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, msg: u32, wp: super::super::Foundation::WPARAM, lp: super::super::Foundation::LPARAM, plresult: *mut super::super::Foundation::LRESULT, pdwcookie: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PreMessageFilter: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PostMessageFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, msg: u32, wp: super::super::Foundation::WPARAM, lp: super::super::Foundation::LPARAM, plresult: *mut super::super::Foundation::LRESULT, dwcookie: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PostMessageFilter: usize,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct ISpecifyPropertyPages(::windows::core::IUnknown);
impl ISpecifyPropertyPages {
    pub unsafe fn GetPages(&self) -> ::windows::core::Result<CAUUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPages)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ISpecifyPropertyPages, ::windows::core::IUnknown);
impl ::core::clone::Clone for ISpecifyPropertyPages {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISpecifyPropertyPages {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpecifyPropertyPages {}
impl ::core::fmt::Debug for ISpecifyPropertyPages {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpecifyPropertyPages").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for ISpecifyPropertyPages {
    type Vtable = ISpecifyPropertyPages_Vtbl;
}
unsafe impl ::windows::core::Interface for ISpecifyPropertyPages {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb196b28b_bab4_101a_b69c_00aa00341d07);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpecifyPropertyPages_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetPages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppages: *mut CAUUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct ITypeChangeEvents(::windows::core::IUnknown);
impl ITypeChangeEvents {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RequestTypeChange<P0, P1>(&self, changekind: CHANGEKIND, ptinfobefore: P0, pstrname: P1) -> ::windows::core::Result<i32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::ITypeInfo>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RequestTypeChange)(::windows::core::Vtable::as_raw(self), changekind, ptinfobefore.into().abi(), pstrname.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AfterTypeChange<P0, P1>(&self, changekind: CHANGEKIND, ptinfoafter: P0, pstrname: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::ITypeInfo>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).AfterTypeChange)(::windows::core::Vtable::as_raw(self), changekind, ptinfoafter.into().abi(), pstrname.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(ITypeChangeEvents, ::windows::core::IUnknown);
impl ::core::clone::Clone for ITypeChangeEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITypeChangeEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITypeChangeEvents {}
impl ::core::fmt::Debug for ITypeChangeEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITypeChangeEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for ITypeChangeEvents {
    type Vtable = ITypeChangeEvents_Vtbl;
}
unsafe impl ::windows::core::Interface for ITypeChangeEvents {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00020410_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeChangeEvents_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub RequestTypeChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, changekind: CHANGEKIND, ptinfobefore: *mut ::core::ffi::c_void, pstrname: ::windows::core::PCWSTR, pfcancel: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RequestTypeChange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AfterTypeChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, changekind: CHANGEKIND, ptinfoafter: *mut ::core::ffi::c_void, pstrname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AfterTypeChange: usize,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct ITypeFactory(::windows::core::IUnknown);
impl ITypeFactory {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateFromTypeInfo<P0>(&self, ptypeinfo: P0, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::ITypeInfo>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateFromTypeInfo)(::windows::core::Vtable::as_raw(self), ptypeinfo.into().abi(), riid, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(ITypeFactory, ::windows::core::IUnknown);
impl ::core::clone::Clone for ITypeFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITypeFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITypeFactory {}
impl ::core::fmt::Debug for ITypeFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITypeFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for ITypeFactory {
    type Vtable = ITypeFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for ITypeFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000002e_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeFactory_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateFromTypeInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptypeinfo: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateFromTypeInfo: usize,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct ITypeMarshal(::windows::core::IUnknown);
impl ITypeMarshal {
    pub unsafe fn Size(&self, pvtype: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Size)(::windows::core::Vtable::as_raw(self), pvtype, dwdestcontext, pvdestcontext, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Marshal(&self, pvtype: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void, pbuffer: &mut [u8], pcbwritten: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Marshal)(::windows::core::Vtable::as_raw(self), pvtype, dwdestcontext, pvdestcontext, pbuffer.len() as _, ::core::mem::transmute(pbuffer.as_ptr()), pcbwritten).ok()
    }
    pub unsafe fn Unmarshal(&self, pvtype: *mut ::core::ffi::c_void, dwflags: u32, pbuffer: &[u8], pcbread: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Unmarshal)(::windows::core::Vtable::as_raw(self), pvtype, dwflags, pbuffer.len() as _, ::core::mem::transmute(pbuffer.as_ptr()), pcbread).ok()
    }
    pub unsafe fn Free(&self, pvtype: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Free)(::windows::core::Vtable::as_raw(self), pvtype).ok()
    }
}
::windows::core::interface_hierarchy!(ITypeMarshal, ::windows::core::IUnknown);
impl ::core::clone::Clone for ITypeMarshal {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITypeMarshal {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITypeMarshal {}
impl ::core::fmt::Debug for ITypeMarshal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITypeMarshal").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for ITypeMarshal {
    type Vtable = ITypeMarshal_Vtbl;
}
unsafe impl ::windows::core::Interface for ITypeMarshal {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000002d_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeMarshal_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvtype: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void, psize: *mut u32) -> ::windows::core::HRESULT,
    pub Marshal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvtype: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void, cbbufferlength: u32, pbuffer: *mut u8, pcbwritten: *mut u32) -> ::windows::core::HRESULT,
    pub Unmarshal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvtype: *mut ::core::ffi::c_void, dwflags: u32, cbbufferlength: u32, pbuffer: *const u8, pcbread: *mut u32) -> ::windows::core::HRESULT,
    pub Free: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvtype: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IVBFormat(::windows::core::IUnknown);
impl IVBFormat {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Format(&self, vdata: *mut super::Com::VARIANT, bstrformat: &::windows::core::BSTR, lpbuffer: *mut ::core::ffi::c_void, cb: u16, lcid: i32, sfirstdayofweek: i16, sfirstweekofyear: u16, rcb: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Format)(::windows::core::Vtable::as_raw(self), vdata, ::core::mem::transmute_copy(bstrformat), lpbuffer, cb, lcid, sfirstdayofweek, sfirstweekofyear, rcb).ok()
    }
}
::windows::core::interface_hierarchy!(IVBFormat, ::windows::core::IUnknown);
impl ::core::clone::Clone for IVBFormat {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVBFormat {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVBFormat {}
impl ::core::fmt::Debug for IVBFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVBFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IVBFormat {
    type Vtable = IVBFormat_Vtbl;
}
unsafe impl ::windows::core::Interface for IVBFormat {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9849fd60_3768_101b_8d72_ae6164ffe3cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVBFormat_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Format: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vdata: *mut super::Com::VARIANT, bstrformat: *mut ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, cb: u16, lcid: i32, sfirstdayofweek: i16, sfirstweekofyear: u16, rcb: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Format: usize,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IVBGetControl(::windows::core::IUnknown);
impl IVBGetControl {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumControls(&self, dwolecontf: OLECONTF, dwwhich: ENUM_CONTROLS_WHICH_FLAGS) -> ::windows::core::Result<super::Com::IEnumUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumControls)(::windows::core::Vtable::as_raw(self), dwolecontf, dwwhich, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IVBGetControl, ::windows::core::IUnknown);
impl ::core::clone::Clone for IVBGetControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVBGetControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVBGetControl {}
impl ::core::fmt::Debug for IVBGetControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVBGetControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IVBGetControl {
    type Vtable = IVBGetControl_Vtbl;
}
unsafe impl ::windows::core::Interface for IVBGetControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x40a050a0_3c31_101b_a82e_08002b2b2337);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVBGetControl_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumControls: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwolecontf: OLECONTF, dwwhich: ENUM_CONTROLS_WHICH_FLAGS, ppenumunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumControls: usize,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IVariantChangeType(::windows::core::IUnknown);
impl IVariantChangeType {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn ChangeType(&self, pvardst: *mut super::Com::VARIANT, pvarsrc: *const super::Com::VARIANT, lcid: u32, vtnew: super::Com::VARENUM) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ChangeType)(::windows::core::Vtable::as_raw(self), pvardst, pvarsrc, lcid, vtnew).ok()
    }
}
::windows::core::interface_hierarchy!(IVariantChangeType, ::windows::core::IUnknown);
impl ::core::clone::Clone for IVariantChangeType {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVariantChangeType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVariantChangeType {}
impl ::core::fmt::Debug for IVariantChangeType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVariantChangeType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IVariantChangeType {
    type Vtable = IVariantChangeType_Vtbl;
}
unsafe impl ::windows::core::Interface for IVariantChangeType {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6ef9862_c720_11d0_9337_00a0c90dcaa9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVariantChangeType_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub ChangeType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvardst: *mut super::Com::VARIANT, pvarsrc: *const super::Com::VARIANT, lcid: u32, vtnew: super::Com::VARENUM) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    ChangeType: usize,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IViewObject(::windows::core::IUnknown);
impl IViewObject {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn Draw<P0, P1>(&self, dwdrawaspect: super::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: ::core::option::Option<*const super::Com::DVTARGETDEVICE>, hdctargetdev: P0, hdcdraw: P1, lprcbounds: ::core::option::Option<*const super::super::Foundation::RECTL>, lprcwbounds: ::core::option::Option<*const super::super::Foundation::RECTL>, pfncontinue: isize, dwcontinue: usize) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
        P1: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
    {
        (::windows::core::Vtable::vtable(self).Draw)(::windows::core::Vtable::as_raw(self), dwdrawaspect, lindex, pvaspect, ::core::mem::transmute(ptd.unwrap_or(::std::ptr::null())), hdctargetdev.into(), hdcdraw.into(), ::core::mem::transmute(lprcbounds.unwrap_or(::std::ptr::null())), ::core::mem::transmute(lprcwbounds.unwrap_or(::std::ptr::null())), pfncontinue, dwcontinue).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn GetColorSet<P0>(&self, dwdrawaspect: super::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: ::core::option::Option<*const super::Com::DVTARGETDEVICE>, hictargetdev: P0, ppcolorset: *mut *mut super::super::Graphics::Gdi::LOGPALETTE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
    {
        (::windows::core::Vtable::vtable(self).GetColorSet)(::windows::core::Vtable::as_raw(self), dwdrawaspect, lindex, pvaspect, ::core::mem::transmute(ptd.unwrap_or(::std::ptr::null())), hictargetdev.into(), ppcolorset).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Freeze(&self, dwdrawaspect: super::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, pdwfreeze: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Freeze)(::windows::core::Vtable::as_raw(self), dwdrawaspect, lindex, pvaspect, pdwfreeze).ok()
    }
    pub unsafe fn Unfreeze(&self, dwfreeze: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Unfreeze)(::windows::core::Vtable::as_raw(self), dwfreeze).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetAdvise<P0>(&self, aspects: super::Com::DVASPECT, advf: super::Com::ADVF, padvsink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IAdviseSink>>,
    {
        (::windows::core::Vtable::vtable(self).SetAdvise)(::windows::core::Vtable::as_raw(self), aspects, advf, padvsink.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetAdvise(&self, paspects: ::core::option::Option<*mut u32>, padvf: ::core::option::Option<*mut u32>, ppadvsink: *mut ::core::option::Option<super::Com::IAdviseSink>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetAdvise)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(paspects.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(padvf.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppadvsink)).ok()
    }
}
::windows::core::interface_hierarchy!(IViewObject, ::windows::core::IUnknown);
impl ::core::clone::Clone for IViewObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IViewObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IViewObject {}
impl ::core::fmt::Debug for IViewObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IViewObject").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IViewObject {
    type Vtable = IViewObject_Vtbl;
}
unsafe impl ::windows::core::Interface for IViewObject {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000010d_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IViewObject_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub Draw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwdrawaspect: super::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *const super::Com::DVTARGETDEVICE, hdctargetdev: super::super::Graphics::Gdi::HDC, hdcdraw: super::super::Graphics::Gdi::HDC, lprcbounds: *const super::super::Foundation::RECTL, lprcwbounds: *const super::super::Foundation::RECTL, pfncontinue: isize, dwcontinue: usize) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))]
    Draw: usize,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub GetColorSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwdrawaspect: super::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *const super::Com::DVTARGETDEVICE, hictargetdev: super::super::Graphics::Gdi::HDC, ppcolorset: *mut *mut super::super::Graphics::Gdi::LOGPALETTE) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))]
    GetColorSet: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Freeze: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwdrawaspect: super::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, pdwfreeze: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Freeze: usize,
    pub Unfreeze: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwfreeze: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SetAdvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aspects: super::Com::DVASPECT, advf: super::Com::ADVF, padvsink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetAdvise: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetAdvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paspects: *mut u32, padvf: *mut u32, ppadvsink: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetAdvise: usize,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IViewObject2(::windows::core::IUnknown);
impl IViewObject2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn Draw<P0, P1>(&self, dwdrawaspect: super::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: ::core::option::Option<*const super::Com::DVTARGETDEVICE>, hdctargetdev: P0, hdcdraw: P1, lprcbounds: ::core::option::Option<*const super::super::Foundation::RECTL>, lprcwbounds: ::core::option::Option<*const super::super::Foundation::RECTL>, pfncontinue: isize, dwcontinue: usize) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
        P1: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
    {
        (::windows::core::Vtable::vtable(self).base__.Draw)(::windows::core::Vtable::as_raw(self), dwdrawaspect, lindex, pvaspect, ::core::mem::transmute(ptd.unwrap_or(::std::ptr::null())), hdctargetdev.into(), hdcdraw.into(), ::core::mem::transmute(lprcbounds.unwrap_or(::std::ptr::null())), ::core::mem::transmute(lprcwbounds.unwrap_or(::std::ptr::null())), pfncontinue, dwcontinue).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn GetColorSet<P0>(&self, dwdrawaspect: super::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: ::core::option::Option<*const super::Com::DVTARGETDEVICE>, hictargetdev: P0, ppcolorset: *mut *mut super::super::Graphics::Gdi::LOGPALETTE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetColorSet)(::windows::core::Vtable::as_raw(self), dwdrawaspect, lindex, pvaspect, ::core::mem::transmute(ptd.unwrap_or(::std::ptr::null())), hictargetdev.into(), ppcolorset).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Freeze(&self, dwdrawaspect: super::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, pdwfreeze: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Freeze)(::windows::core::Vtable::as_raw(self), dwdrawaspect, lindex, pvaspect, pdwfreeze).ok()
    }
    pub unsafe fn Unfreeze(&self, dwfreeze: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Unfreeze)(::windows::core::Vtable::as_raw(self), dwfreeze).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetAdvise<P0>(&self, aspects: super::Com::DVASPECT, advf: super::Com::ADVF, padvsink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IAdviseSink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetAdvise)(::windows::core::Vtable::as_raw(self), aspects, advf, padvsink.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetAdvise(&self, paspects: ::core::option::Option<*mut u32>, padvf: ::core::option::Option<*mut u32>, ppadvsink: *mut ::core::option::Option<super::Com::IAdviseSink>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetAdvise)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(paspects.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(padvf.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppadvsink)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetExtent(&self, dwdrawaspect: super::Com::DVASPECT, lindex: i32, ptd: *const super::Com::DVTARGETDEVICE) -> ::windows::core::Result<super::super::Foundation::SIZE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetExtent)(::windows::core::Vtable::as_raw(self), dwdrawaspect, lindex, ptd, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IViewObject2, ::windows::core::IUnknown, IViewObject);
impl ::core::clone::Clone for IViewObject2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IViewObject2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IViewObject2 {}
impl ::core::fmt::Debug for IViewObject2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IViewObject2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IViewObject2 {
    type Vtable = IViewObject2_Vtbl;
}
unsafe impl ::windows::core::Interface for IViewObject2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000127_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IViewObject2_Vtbl {
    pub base__: IViewObject_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetExtent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwdrawaspect: super::Com::DVASPECT, lindex: i32, ptd: *const super::Com::DVTARGETDEVICE, lpsizel: *mut super::super::Foundation::SIZE) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetExtent: usize,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IViewObjectEx(::windows::core::IUnknown);
impl IViewObjectEx {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn Draw<P0, P1>(&self, dwdrawaspect: super::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: ::core::option::Option<*const super::Com::DVTARGETDEVICE>, hdctargetdev: P0, hdcdraw: P1, lprcbounds: ::core::option::Option<*const super::super::Foundation::RECTL>, lprcwbounds: ::core::option::Option<*const super::super::Foundation::RECTL>, pfncontinue: isize, dwcontinue: usize) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
        P1: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.Draw)(::windows::core::Vtable::as_raw(self), dwdrawaspect, lindex, pvaspect, ::core::mem::transmute(ptd.unwrap_or(::std::ptr::null())), hdctargetdev.into(), hdcdraw.into(), ::core::mem::transmute(lprcbounds.unwrap_or(::std::ptr::null())), ::core::mem::transmute(lprcwbounds.unwrap_or(::std::ptr::null())), pfncontinue, dwcontinue).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn GetColorSet<P0>(&self, dwdrawaspect: super::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: ::core::option::Option<*const super::Com::DVTARGETDEVICE>, hictargetdev: P0, ppcolorset: *mut *mut super::super::Graphics::Gdi::LOGPALETTE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetColorSet)(::windows::core::Vtable::as_raw(self), dwdrawaspect, lindex, pvaspect, ::core::mem::transmute(ptd.unwrap_or(::std::ptr::null())), hictargetdev.into(), ppcolorset).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Freeze(&self, dwdrawaspect: super::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, pdwfreeze: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Freeze)(::windows::core::Vtable::as_raw(self), dwdrawaspect, lindex, pvaspect, pdwfreeze).ok()
    }
    pub unsafe fn Unfreeze(&self, dwfreeze: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Unfreeze)(::windows::core::Vtable::as_raw(self), dwfreeze).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetAdvise<P0>(&self, aspects: super::Com::DVASPECT, advf: super::Com::ADVF, padvsink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IAdviseSink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetAdvise)(::windows::core::Vtable::as_raw(self), aspects, advf, padvsink.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetAdvise(&self, paspects: ::core::option::Option<*mut u32>, padvf: ::core::option::Option<*mut u32>, ppadvsink: *mut ::core::option::Option<super::Com::IAdviseSink>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetAdvise)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(paspects.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(padvf.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppadvsink)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetExtent(&self, dwdrawaspect: super::Com::DVASPECT, lindex: i32, ptd: *const super::Com::DVTARGETDEVICE) -> ::windows::core::Result<super::super::Foundation::SIZE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetExtent)(::windows::core::Vtable::as_raw(self), dwdrawaspect, lindex, ptd, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRect(&self, dwaspect: u32) -> ::windows::core::Result<super::super::Foundation::RECTL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetRect)(::windows::core::Vtable::as_raw(self), dwaspect, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetViewStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetViewStatus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryHitPoint(&self, dwaspect: u32, prectbounds: *const super::super::Foundation::RECT, ptlloc: super::super::Foundation::POINT, lclosehint: i32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).QueryHitPoint)(::windows::core::Vtable::as_raw(self), dwaspect, prectbounds, ::core::mem::transmute(ptlloc), lclosehint, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryHitRect(&self, dwaspect: u32, prectbounds: *const super::super::Foundation::RECT, prectloc: *const super::super::Foundation::RECT, lclosehint: i32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).QueryHitRect)(::windows::core::Vtable::as_raw(self), dwaspect, prectbounds, prectloc, lclosehint, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn GetNaturalExtent<P0>(&self, dwaspect: super::Com::DVASPECT, lindex: i32, ptd: *const super::Com::DVTARGETDEVICE, hictargetdev: P0, pextentinfo: *const DVEXTENTINFO) -> ::windows::core::Result<super::super::Foundation::SIZE>
    where
        P0: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetNaturalExtent)(::windows::core::Vtable::as_raw(self), dwaspect, lindex, ptd, hictargetdev.into(), pextentinfo, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IViewObjectEx, ::windows::core::IUnknown, IViewObject, IViewObject2);
impl ::core::clone::Clone for IViewObjectEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IViewObjectEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IViewObjectEx {}
impl ::core::fmt::Debug for IViewObjectEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IViewObjectEx").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IViewObjectEx {
    type Vtable = IViewObjectEx_Vtbl;
}
unsafe impl ::windows::core::Interface for IViewObjectEx {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3af24292_0c96_11ce_a0cf_00aa00600ab8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IViewObjectEx_Vtbl {
    pub base__: IViewObject2_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwaspect: u32, prect: *mut super::super::Foundation::RECTL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetRect: usize,
    pub GetViewStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub QueryHitPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwaspect: u32, prectbounds: *const super::super::Foundation::RECT, ptlloc: super::super::Foundation::POINT, lclosehint: i32, phitresult: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    QueryHitPoint: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub QueryHitRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwaspect: u32, prectbounds: *const super::super::Foundation::RECT, prectloc: *const super::super::Foundation::RECT, lclosehint: i32, phitresult: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    QueryHitRect: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub GetNaturalExtent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwaspect: super::Com::DVASPECT, lindex: i32, ptd: *const super::Com::DVTARGETDEVICE, hictargetdev: super::super::Graphics::Gdi::HDC, pextentinfo: *const DVEXTENTINFO, psizel: *mut super::super::Foundation::SIZE) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))]
    GetNaturalExtent: usize,
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
pub struct IZoomEvents(::windows::core::IUnknown);
impl IZoomEvents {
    pub unsafe fn OnZoomPercentChanged(&self, ulzoompercent: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).OnZoomPercentChanged)(::windows::core::Vtable::as_raw(self), ulzoompercent).ok()
    }
}
::windows::core::interface_hierarchy!(IZoomEvents, ::windows::core::IUnknown);
impl ::core::clone::Clone for IZoomEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IZoomEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IZoomEvents {}
impl ::core::fmt::Debug for IZoomEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IZoomEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IZoomEvents {
    type Vtable = IZoomEvents_Vtbl;
}
unsafe impl ::windows::core::Interface for IZoomEvents {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x41b68150_904c_4e17_a0ba_a438182e359d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IZoomEvents_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnZoomPercentChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulzoompercent: u32) -> ::windows::core::HRESULT,
}
pub const CLSID_CColorPropPage: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0be35201_8f91_11ce_9de3_00aa004bb851);
pub const CLSID_CFontPropPage: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0be35200_8f91_11ce_9de3_00aa004bb851);
pub const CLSID_CPicturePropPage: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0be35202_8f91_11ce_9de3_00aa004bb851);
pub const CLSID_ConvertVBX: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb8f0822_0164_101b_84ed_08002b2ec713);
pub const CLSID_PersistPropset: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb8f0821_0164_101b_84ed_08002b2ec713);
pub const CLSID_StdFont: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0be35203_8f91_11ce_9de3_00aa004bb851);
pub const CLSID_StdPicture: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0be35204_8f91_11ce_9de3_00aa004bb851);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CONNECT_E_ADVISELIMIT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220991i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CONNECT_E_CANNOTCONNECT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220990i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CONNECT_E_FIRST: i32 = -2147220992i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CONNECT_E_LAST: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220977i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CONNECT_E_NOCONNECTION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220992i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CONNECT_E_OVERRIDDEN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220989i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CONNECT_S_FIRST: ::windows::core::HRESULT = ::windows::core::HRESULT(262656i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CONNECT_S_LAST: ::windows::core::HRESULT = ::windows::core::HRESULT(262671i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CTL_E_ILLEGALFUNCTIONCALL: i32 = -2146828283i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DD_DEFDRAGDELAY: u32 = 200u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DD_DEFDRAGMINDIST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DD_DEFSCROLLDELAY: u32 = 50u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DD_DEFSCROLLINSET: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DD_DEFSCROLLINTERVAL: u32 = 50u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPATCH_CONSTRUCT: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_ABOUTBOX: i32 = -552i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_ACCELERATOR: i32 = -543i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_ADDITEM: i32 = -553i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_AMBIENT_APPEARANCE: i32 = -716i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_AMBIENT_AUTOCLIP: i32 = -715i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_AMBIENT_BACKCOLOR: i32 = -701i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_AMBIENT_CHARSET: i32 = -727i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_AMBIENT_CODEPAGE: i32 = -725i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_AMBIENT_DISPLAYASDEFAULT: i32 = -713i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_AMBIENT_DISPLAYNAME: i32 = -702i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_AMBIENT_FONT: i32 = -703i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_AMBIENT_FORECOLOR: i32 = -704i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_AMBIENT_LOCALEID: i32 = -705i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_AMBIENT_MESSAGEREFLECT: i32 = -706i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_AMBIENT_PALETTE: i32 = -726i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_AMBIENT_RIGHTTOLEFT: i32 = -732i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_AMBIENT_SCALEUNITS: i32 = -707i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_AMBIENT_SHOWGRABHANDLES: i32 = -711i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_AMBIENT_SHOWHATCHING: i32 = -712i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_AMBIENT_SUPPORTSMNEMONICS: i32 = -714i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_AMBIENT_TEXTALIGN: i32 = -708i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_AMBIENT_TOPTOBOTTOM: i32 = -733i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_AMBIENT_TRANSFERPRIORITY: i32 = -728i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_AMBIENT_UIDEAD: i32 = -710i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_AMBIENT_USERMODE: i32 = -709i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_APPEARANCE: i32 = -520i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_AUTOSIZE: i32 = -500i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_BACKCOLOR: i32 = -501i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_BACKSTYLE: i32 = -502i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_BORDERCOLOR: i32 = -503i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_BORDERSTYLE: i32 = -504i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_BORDERVISIBLE: i32 = -519i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_BORDERWIDTH: i32 = -505i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_CAPTION: i32 = -518i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_CLEAR: i32 = -554i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_CLICK: i32 = -600i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_CLICK_VALUE: i32 = -610i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_COLLECT: i32 = -8i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_COLUMN: i32 = -529i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_CONSTRUCTOR: i32 = -6i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_DBLCLICK: i32 = -601i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_DESTRUCTOR: i32 = -7i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_DISPLAYSTYLE: i32 = -540i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_DOCLICK: i32 = -551i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_DRAWMODE: i32 = -507i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_DRAWSTYLE: i32 = -508i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_DRAWWIDTH: i32 = -509i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_Delete: i32 = -801i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_ENABLED: i32 = -514i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_ENTERKEYBEHAVIOR: i32 = -544i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_ERROREVENT: i32 = -608i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_EVALUATE: i32 = -5i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_FILLCOLOR: i32 = -510i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_FILLSTYLE: i32 = -511i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_FONT: i32 = -512i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_FONT_BOLD: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_FONT_CHANGED: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_FONT_CHARSET: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_FONT_ITALIC: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_FONT_NAME: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_FONT_SIZE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_FONT_STRIKE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_FONT_UNDER: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_FONT_WEIGHT: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_FORECOLOR: i32 = -513i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_GROUPNAME: i32 = -541i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_HWND: i32 = -515i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_IMEMODE: i32 = -542i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_KEYDOWN: i32 = -602i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_KEYPRESS: i32 = -603i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_KEYUP: i32 = -604i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_LIST: i32 = -528i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_LISTCOUNT: i32 = -531i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_LISTINDEX: i32 = -526i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_MAXLENGTH: i32 = -533i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_MOUSEDOWN: i32 = -605i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_MOUSEICON: i32 = -522i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_MOUSEMOVE: i32 = -606i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_MOUSEPOINTER: i32 = -521i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_MOUSEUP: i32 = -607i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_MULTILINE: i32 = -537i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_MULTISELECT: i32 = -532i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_NEWENUM: i32 = -4i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_NUMBEROFCOLUMNS: i32 = -539i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_NUMBEROFROWS: i32 = -538i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_Name: i32 = -800i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_Object: i32 = -802i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_PASSWORDCHAR: i32 = -534i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_PICTURE: i32 = -523i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_PICT_HANDLE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_PICT_HEIGHT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_PICT_HPAL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_PICT_RENDER: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_PICT_TYPE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_PICT_WIDTH: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_PROPERTYPUT: i32 = -3i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_Parent: i32 = -803i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_READYSTATE: i32 = -525i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_READYSTATECHANGE: i32 = -609i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_REFRESH: i32 = -550i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_REMOVEITEM: i32 = -555i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_RIGHTTOLEFT: i32 = -611i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_SCROLLBARS: i32 = -535i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_SELECTED: i32 = -527i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_SELLENGTH: i32 = -548i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_SELSTART: i32 = -547i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_SELTEXT: i32 = -546i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_STARTENUM: i32 = -1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_TABKEYBEHAVIOR: i32 = -545i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_TABSTOP: i32 = -516i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_TEXT: i32 = -517i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_THIS: i32 = -613i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_TOPTOBOTTOM: i32 = -612i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_UNKNOWN: i32 = -1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_VALID: i32 = -524i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_VALUE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISPID_WORDWRAP: i32 = -536i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const GC_WCH_SIBLING: i32 = 1i32;
pub const GUID_CHECKVALUEEXCLUSIVE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6650430c_be0f_101a_8bbb_00aa00300cab);
pub const GUID_COLOR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x66504301_be0f_101a_8bbb_00aa00300cab);
pub const GUID_FONTBOLD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6650430f_be0f_101a_8bbb_00aa00300cab);
pub const GUID_FONTITALIC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x66504310_be0f_101a_8bbb_00aa00300cab);
pub const GUID_FONTNAME: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6650430d_be0f_101a_8bbb_00aa00300cab);
pub const GUID_FONTSIZE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6650430e_be0f_101a_8bbb_00aa00300cab);
pub const GUID_FONTSTRIKETHROUGH: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x66504312_be0f_101a_8bbb_00aa00300cab);
pub const GUID_FONTUNDERSCORE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x66504311_be0f_101a_8bbb_00aa00300cab);
pub const GUID_HANDLE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x66504313_be0f_101a_8bbb_00aa00300cab);
pub const GUID_HIMETRIC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x66504300_be0f_101a_8bbb_00aa00300cab);
pub const GUID_OPTIONVALUEEXCLUSIVE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6650430b_be0f_101a_8bbb_00aa00300cab);
pub const GUID_TRISTATE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6650430a_be0f_101a_8bbb_00aa00300cab);
pub const GUID_XPOS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x66504306_be0f_101a_8bbb_00aa00300cab);
pub const GUID_XPOSPIXEL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x66504302_be0f_101a_8bbb_00aa00300cab);
pub const GUID_XSIZE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x66504308_be0f_101a_8bbb_00aa00300cab);
pub const GUID_XSIZEPIXEL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x66504304_be0f_101a_8bbb_00aa00300cab);
pub const GUID_YPOS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x66504307_be0f_101a_8bbb_00aa00300cab);
pub const GUID_YPOSPIXEL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x66504303_be0f_101a_8bbb_00aa00300cab);
pub const GUID_YSIZE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x66504309_be0f_101a_8bbb_00aa00300cab);
pub const GUID_YSIZEPIXEL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x66504305_be0f_101a_8bbb_00aa00300cab);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_BZ_ICON: u32 = 601u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_BZ_MESSAGE1: u32 = 602u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_BZ_RETRY: u32 = 600u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_BZ_SWITCHTO: u32 = 604u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_CI_BROWSE: u32 = 130u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_CI_CURRENT: u32 = 121u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_CI_CURRENTICON: u32 = 122u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_CI_DEFAULT: u32 = 123u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_CI_DEFAULTICON: u32 = 124u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_CI_FROMFILE: u32 = 125u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_CI_FROMFILEEDIT: u32 = 126u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_CI_GROUP: u32 = 120u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_CI_ICONDISPLAY: u32 = 131u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_CI_ICONLIST: u32 = 127u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_CI_LABEL: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_CI_LABELEDIT: u32 = 129u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_CV_ACTIVATEAS: u32 = 156u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_CV_ACTIVATELIST: u32 = 154u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_CV_CHANGEICON: u32 = 153u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_CV_CONVERTLIST: u32 = 158u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_CV_CONVERTTO: u32 = 155u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_CV_DISPLAYASICON: u32 = 152u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_CV_ICONDISPLAY: u32 = 165u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_CV_OBJECTTYPE: u32 = 150u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_CV_RESULTTEXT: u32 = 157u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_EL_AUTOMATIC: u32 = 202u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_EL_CANCELLINK: u32 = 209u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_EL_CHANGESOURCE: u32 = 201u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_EL_COL1: u32 = 220u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_EL_COL2: u32 = 221u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_EL_COL3: u32 = 222u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_EL_LINKSLISTBOX: u32 = 206u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_EL_LINKSOURCE: u32 = 216u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_EL_LINKTYPE: u32 = 217u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_EL_MANUAL: u32 = 212u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_EL_OPENSOURCE: u32 = 211u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_EL_UPDATENOW: u32 = 210u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_GP_CONVERT: u32 = 1013u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_GP_OBJECTICON: u32 = 1014u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_GP_OBJECTLOCATION: u32 = 1022u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_GP_OBJECTNAME: u32 = 1009u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_GP_OBJECTSIZE: u32 = 1011u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_GP_OBJECTTYPE: u32 = 1010u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_IO_ADDCONTROL: u32 = 2115u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_IO_CHANGEICON: u32 = 2105u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_IO_CONTROLTYPELIST: u32 = 2116u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_IO_CREATEFROMFILE: u32 = 2101u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_IO_CREATENEW: u32 = 2100u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_IO_DISPLAYASICON: u32 = 2104u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_IO_FILE: u32 = 2106u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_IO_FILEDISPLAY: u32 = 2107u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_IO_FILETEXT: u32 = 2112u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_IO_FILETYPE: u32 = 2113u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_IO_ICONDISPLAY: u32 = 2110u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_IO_INSERTCONTROL: u32 = 2114u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_IO_LINKFILE: u32 = 2102u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_IO_OBJECTTYPELIST: u32 = 2103u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_IO_OBJECTTYPETEXT: u32 = 2111u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_IO_RESULTIMAGE: u32 = 2108u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_IO_RESULTTEXT: u32 = 2109u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_LP_AUTOMATIC: u32 = 1016u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_LP_BREAKLINK: u32 = 1008u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_LP_CHANGESOURCE: u32 = 1015u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_LP_DATE: u32 = 1018u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_LP_LINKSOURCE: u32 = 1012u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_LP_MANUAL: u32 = 1017u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_LP_OPENSOURCE: u32 = 1006u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_LP_TIME: u32 = 1019u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_LP_UPDATENOW: u32 = 1007u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_OLEUIHELP: u32 = 99u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_PS_CHANGEICON: u32 = 508u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_PS_DISPLAYASICON: u32 = 506u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_PS_DISPLAYLIST: u32 = 505u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_PS_ICONDISPLAY: u32 = 507u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_PS_PASTE: u32 = 500u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_PS_PASTELINK: u32 = 501u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_PS_PASTELINKLIST: u32 = 504u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_PS_PASTELIST: u32 = 503u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_PS_RESULTIMAGE: u32 = 509u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_PS_RESULTTEXT: u32 = 510u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_PS_SOURCETEXT: u32 = 502u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_PU_CONVERT: u32 = 902u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_PU_ICON: u32 = 908u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_PU_LINKS: u32 = 900u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_PU_TEXT: u32 = 901u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_UL_METER: u32 = 1029u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_UL_PERCENT: u32 = 1031u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_UL_PROGRESS: u32 = 1032u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_UL_STOP: u32 = 1030u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_VP_ASICON: u32 = 1003u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_VP_CHANGEICON: u32 = 1001u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_VP_EDITABLE: u32 = 1002u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_VP_ICONDISPLAY: u32 = 1021u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_VP_PERCENT: u32 = 1000u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_VP_RELATIVE: u32 = 1005u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_VP_RESULTIMAGE: u32 = 1033u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_VP_SCALETXT: u32 = 1034u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDC_VP_SPIN: u32 = 1006u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_BUSY: u32 = 1006u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_CANNOTUPDATELINK: u32 = 1008u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_CHANGEICON: u32 = 1001u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_CHANGEICONBROWSE: u32 = 1011u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_CHANGESOURCE: u32 = 1009u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_CHANGESOURCE4: u32 = 1013u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_CONVERT: u32 = 1002u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_CONVERT4: u32 = 1103u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_CONVERTONLY: u32 = 1012u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_CONVERTONLY4: u32 = 1104u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_EDITLINKS: u32 = 1004u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_EDITLINKS4: u32 = 1105u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_GNRLPROPS: u32 = 1100u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_GNRLPROPS4: u32 = 1106u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_INSERTFILEBROWSE: u32 = 1010u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_INSERTOBJECT: u32 = 1000u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_LINKPROPS: u32 = 1102u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_LINKPROPS4: u32 = 1107u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_LINKSOURCEUNAVAILABLE: u32 = 1020u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_LINKTYPECHANGED: u32 = 1022u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_LINKTYPECHANGEDA: u32 = 1026u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_LINKTYPECHANGEDW: u32 = 1022u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_OUTOFMEMORY: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_PASTESPECIAL: u32 = 1003u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_PASTESPECIAL4: u32 = 1108u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_SERVERNOTFOUND: u32 = 1023u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_SERVERNOTREG: u32 = 1021u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_SERVERNOTREGA: u32 = 1025u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_SERVERNOTREGW: u32 = 1021u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_UPDATELINKS: u32 = 1007u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IDD_VIEWPROPS: u32 = 1101u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const ID_BROWSE_ADDCONTROL: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const ID_BROWSE_CHANGEICON: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const ID_BROWSE_CHANGESOURCE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const ID_BROWSE_INSERTFILE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const ID_DEFAULTINST: i32 = -2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const INSTALL_SCOPE_INVALID: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const INSTALL_SCOPE_MACHINE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const INSTALL_SCOPE_USER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const LOAD_TLB_AS_32BIT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const LOAD_TLB_AS_64BIT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const LOCALE_USE_NLS: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const MEMBERID_NIL: i32 = -1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const MK_ALT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const MSOCMDERR_E_CANCELED: i32 = -2147221245i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const MSOCMDERR_E_DISABLED: i32 = -2147221247i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const MSOCMDERR_E_FIRST: i32 = -2147221248i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const MSOCMDERR_E_NOHELP: i32 = -2147221246i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const MSOCMDERR_E_NOTSUPPORTED: i32 = -2147221248i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const MSOCMDERR_E_UNKNOWNGROUP: i32 = -2147221244i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OCM__BASE: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OF_GET: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OF_HANDLER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OF_SET: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDARGINDEX_ACTIVEXINSTALL_CLSID: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDARGINDEX_ACTIVEXINSTALL_DISPLAYNAME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDARGINDEX_ACTIVEXINSTALL_INSTALLSCOPE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDARGINDEX_ACTIVEXINSTALL_PUBLISHER: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDARGINDEX_ACTIVEXINSTALL_SOURCEURL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDARGINDEX_SHOWPAGEACTIONMENU_HWND: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDARGINDEX_SHOWPAGEACTIONMENU_X: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDARGINDEX_SHOWPAGEACTIONMENU_Y: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDERR_E_CANCELED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221245i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDERR_E_DISABLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221247i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDERR_E_FIRST: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221248i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDERR_E_NOHELP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221246i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDERR_E_NOTSUPPORTED: i32 = -2147221248i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDERR_E_UNKNOWNGROUP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147221244i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMD_TASKDLGID_ONBEFOREUNLOAD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEIVERB_PROPERTIES: i32 = -7i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLESTDDELIM: ::windows::core::PCWSTR = ::windows::w!("\\");
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_BZERR_HTASKINVALID: u32 = 116u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_BZ_CALLUNBLOCKED: u32 = 119u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_BZ_RETRYSELECTED: u32 = 118u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_BZ_SWITCHTOSELECTED: u32 = 117u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_CANCEL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_CIERR_MUSTHAVECLSID: u32 = 116u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_CIERR_MUSTHAVECURRENTMETAFILE: u32 = 117u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_CIERR_SZICONEXEINVALID: u32 = 118u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_CSERR_FROMNOTNULL: u32 = 118u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_CSERR_LINKCNTRINVALID: u32 = 117u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_CSERR_LINKCNTRNULL: u32 = 116u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_CSERR_SOURCEINVALID: u32 = 121u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_CSERR_SOURCENULL: u32 = 120u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_CSERR_SOURCEPARSEERROR: u32 = 122u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_CSERR_SOURCEPARSERROR: u32 = 122u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_CSERR_TONOTNULL: u32 = 119u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_CTERR_CBFORMATINVALID: u32 = 119u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_CTERR_CLASSIDINVALID: u32 = 117u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_CTERR_DVASPECTINVALID: u32 = 118u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_CTERR_HMETAPICTINVALID: u32 = 120u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_CTERR_STRINGINVALID: u32 = 121u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_ELERR_LINKCNTRINVALID: u32 = 117u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_ELERR_LINKCNTRNULL: u32 = 116u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_ERR_CBSTRUCTINCORRECT: u32 = 103u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_ERR_DIALOGFAILURE: u32 = 112u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_ERR_FINDTEMPLATEFAILURE: u32 = 110u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_ERR_GLOBALMEMALLOC: u32 = 114u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_ERR_HINSTANCEINVALID: u32 = 107u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_ERR_HRESOURCEINVALID: u32 = 109u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_ERR_HWNDOWNERINVALID: u32 = 104u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_ERR_LOADSTRING: u32 = 115u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_ERR_LOADTEMPLATEFAILURE: u32 = 111u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_ERR_LOCALMEMALLOC: u32 = 113u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_ERR_LPFNHOOKINVALID: u32 = 106u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_ERR_LPSZCAPTIONINVALID: u32 = 105u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_ERR_LPSZTEMPLATEINVALID: u32 = 108u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_ERR_OLEMEMALLOC: u32 = 100u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_ERR_STANDARDMAX: u32 = 116u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_ERR_STANDARDMIN: u32 = 100u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_ERR_STRUCTUREINVALID: u32 = 102u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_ERR_STRUCTURENULL: u32 = 101u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_FALSE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_GPERR_CBFORMATINVALID: u32 = 130u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_GPERR_CLASSIDINVALID: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_GPERR_LPCLSIDEXCLUDEINVALID: u32 = 129u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_GPERR_STRINGINVALID: u32 = 127u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_IOERR_ARRLINKTYPESINVALID: u32 = 118u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_IOERR_ARRPASTEENTRIESINVALID: u32 = 117u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_IOERR_CCHFILEINVALID: u32 = 125u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_IOERR_HICONINVALID: u32 = 118u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_IOERR_LPCLSIDEXCLUDEINVALID: u32 = 124u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_IOERR_LPFORMATETCINVALID: u32 = 119u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_IOERR_LPIOLECLIENTSITEINVALID: u32 = 121u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_IOERR_LPISTORAGEINVALID: u32 = 122u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_IOERR_LPSZFILEINVALID: u32 = 116u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_IOERR_LPSZLABELINVALID: u32 = 117u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_IOERR_PPVOBJINVALID: u32 = 120u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_IOERR_SCODEHASERROR: u32 = 123u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_IOERR_SRCDATAOBJECTINVALID: u32 = 116u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_LPERR_LINKCNTRINVALID: u32 = 134u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_LPERR_LINKCNTRNULL: u32 = 133u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_OK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_OPERR_DLGPROCNOTNULL: u32 = 125u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_OPERR_INVALIDPAGES: u32 = 123u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_OPERR_LINKINFOINVALID: u32 = 137u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_OPERR_LPARAMNOTZERO: u32 = 126u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_OPERR_NOTSUPPORTED: u32 = 124u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_OPERR_OBJINFOINVALID: u32 = 136u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_OPERR_PAGESINCORRECT: u32 = 122u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_OPERR_PROPERTYSHEET: u32 = 135u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_OPERR_PROPSHEETINVALID: u32 = 119u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_OPERR_PROPSHEETNULL: u32 = 118u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_OPERR_PROPSINVALID: u32 = 121u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_OPERR_SUBPROPINVALID: u32 = 117u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_OPERR_SUBPROPNULL: u32 = 116u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_OPERR_SUPPROP: u32 = 120u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_PSERR_CLIPBOARDCHANGED: u32 = 119u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_PSERR_GETCLIPBOARDFAILED: u32 = 120u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_QUERY_GETCLASSID: u32 = 65280u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_QUERY_LINKBROKEN: u32 = 65281u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_SUCCESS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_VPERR_DVASPECTINVALID: u32 = 132u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUI_VPERR_METAPICTINVALID: u32 = 131u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEVERB_PRIMARY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OT_EMBEDDED: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OT_LINK: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OT_STATIC: i32 = 3i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PERPROP_E_FIRST: i32 = -2147220992i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PERPROP_E_LAST: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220977i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PERPROP_E_NOPAGEAVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220992i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PERPROP_S_FIRST: ::windows::core::HRESULT = ::windows::core::HRESULT(262656i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PERPROP_S_LAST: ::windows::core::HRESULT = ::windows::core::HRESULT(262671i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PROP_HWND_CHGICONDLG: ::windows::core::PCWSTR = ::windows::w!("HWND_CIDLG");
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PS_MAXLINKTYPES: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const SELFREG_E_CLASS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220991i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const SELFREG_E_FIRST: i32 = -2147220992i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const SELFREG_E_LAST: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220977i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const SELFREG_E_TYPELIB: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220992i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const SELFREG_S_FIRST: ::windows::core::HRESULT = ::windows::core::HRESULT(262656i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const SELFREG_S_LAST: ::windows::core::HRESULT = ::windows::core::HRESULT(262671i32);
pub const SID_GetCaller: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4717cc40_bcb9_11d0_9336_00a0c90dcaa9);
pub const SID_ProvideRuntimeContext: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x74a5040c_dd0c_48f0_ac85_194c3259180a);
pub const SID_VariantConversion: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1f101481_bccd_11d0_9336_00a0c90dcaa9);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const STDOLE2_LCID: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const STDOLE2_MAJORVERNUM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const STDOLE2_MINORVERNUM: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const STDOLE_LCID: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const STDOLE_MAJORVERNUM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const STDOLE_MINORVERNUM: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const STDOLE_TLB: ::windows::core::PCSTR = ::windows::s!("stdole2.tlb");
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const STDTYPE_TLB: ::windows::core::PCSTR = ::windows::s!("stdole2.tlb");
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const SZOLEUI_MSG_ADDCONTROL: ::windows::core::PCWSTR = ::windows::w!("OLEUI_MSG_ADDCONTROL");
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const SZOLEUI_MSG_BROWSE: ::windows::core::PCWSTR = ::windows::w!("OLEUI_MSG_BROWSE");
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const SZOLEUI_MSG_BROWSE_OFN: ::windows::core::PCWSTR = ::windows::w!("OLEUI_MSG_BROWSE_OFN");
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const SZOLEUI_MSG_CHANGEICON: ::windows::core::PCWSTR = ::windows::w!("OLEUI_MSG_CHANGEICON");
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const SZOLEUI_MSG_CHANGESOURCE: ::windows::core::PCWSTR = ::windows::w!("OLEUI_MSG_CHANGESOURCE");
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const SZOLEUI_MSG_CLOSEBUSYDIALOG: ::windows::core::PCWSTR = ::windows::w!("OLEUI_MSG_CLOSEBUSYDIALOG");
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const SZOLEUI_MSG_CONVERT: ::windows::core::PCWSTR = ::windows::w!("OLEUI_MSG_CONVERT");
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const SZOLEUI_MSG_ENDDIALOG: ::windows::core::PCWSTR = ::windows::w!("OLEUI_MSG_ENDDIALOG");
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const SZOLEUI_MSG_HELP: ::windows::core::PCWSTR = ::windows::w!("OLEUI_MSG_HELP");
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const TIFLAGS_EXTENDDISPATCHONLY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARIANT_ALPHABOOL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARIANT_CALENDAR_GREGORIAN: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARIANT_CALENDAR_HIJRI: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARIANT_CALENDAR_THAI: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARIANT_LOCALBOOL: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARIANT_NOUSEROVERRIDE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARIANT_NOVALUEPROP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARIANT_USE_NLS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VAR_CALENDAR_GREGORIAN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VAR_CALENDAR_HIJRI: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VAR_CALENDAR_THAI: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VAR_DATEVALUEONLY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VAR_FORMAT_NOSUBSTITUTE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VAR_FOURDIGITYEARS: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VAR_LOCALBOOL: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VAR_TIMEVALUEONLY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VAR_VALIDDATE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VTDATEGRE_MAX: u32 = 2958465u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VTDATEGRE_MIN: i32 = -657434i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_BLOB_PROPSET: u32 = 75u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_STORED_PROPSET: u32 = 74u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_STREAMED_PROPSET: u32 = 73u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VT_VERBOSE_ENUM: u32 = 76u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const WIN32: u32 = 100u32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const fdexEnumAll: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const fdexEnumDefault: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const fdexNameCaseInsensitive: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const fdexNameCaseSensitive: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const fdexNameEnsure: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const fdexNameImplicit: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const fdexNameInternal: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const fdexNameNoDynamicProperties: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ACTIVATEFLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const ACTIVATE_WINDOWLESS: ACTIVATEFLAGS = ACTIVATEFLAGS(1i32);
impl ::core::marker::Copy for ACTIVATEFLAGS {}
impl ::core::clone::Clone for ACTIVATEFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ACTIVATEFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ACTIVATEFLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for ACTIVATEFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ACTIVATEFLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ACTIVEOBJECT_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const ACTIVEOBJECT_STRONG: ACTIVEOBJECT_FLAGS = ACTIVEOBJECT_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const ACTIVEOBJECT_WEAK: ACTIVEOBJECT_FLAGS = ACTIVEOBJECT_FLAGS(1u32);
impl ::core::marker::Copy for ACTIVEOBJECT_FLAGS {}
impl ::core::clone::Clone for ACTIVEOBJECT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ACTIVEOBJECT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ACTIVEOBJECT_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for ACTIVEOBJECT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ACTIVEOBJECT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for ACTIVEOBJECT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for ACTIVEOBJECT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for ACTIVEOBJECT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for ACTIVEOBJECT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for ACTIVEOBJECT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BINDSPEED(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const BINDSPEED_INDEFINITE: BINDSPEED = BINDSPEED(1i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const BINDSPEED_MODERATE: BINDSPEED = BINDSPEED(2i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const BINDSPEED_IMMEDIATE: BINDSPEED = BINDSPEED(3i32);
impl ::core::marker::Copy for BINDSPEED {}
impl ::core::clone::Clone for BINDSPEED {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BINDSPEED {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for BINDSPEED {
    type Abi = Self;
}
impl ::core::fmt::Debug for BINDSPEED {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BINDSPEED").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BUSY_DIALOG_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const BZ_DISABLECANCELBUTTON: BUSY_DIALOG_FLAGS = BUSY_DIALOG_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const BZ_DISABLESWITCHTOBUTTON: BUSY_DIALOG_FLAGS = BUSY_DIALOG_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const BZ_DISABLERETRYBUTTON: BUSY_DIALOG_FLAGS = BUSY_DIALOG_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const BZ_NOTRESPONDINGDIALOG: BUSY_DIALOG_FLAGS = BUSY_DIALOG_FLAGS(8u32);
impl ::core::marker::Copy for BUSY_DIALOG_FLAGS {}
impl ::core::clone::Clone for BUSY_DIALOG_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BUSY_DIALOG_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for BUSY_DIALOG_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for BUSY_DIALOG_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BUSY_DIALOG_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for BUSY_DIALOG_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for BUSY_DIALOG_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for BUSY_DIALOG_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for BUSY_DIALOG_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for BUSY_DIALOG_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CHANGEKIND(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CHANGEKIND_ADDMEMBER: CHANGEKIND = CHANGEKIND(0i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CHANGEKIND_DELETEMEMBER: CHANGEKIND = CHANGEKIND(1i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CHANGEKIND_SETNAMES: CHANGEKIND = CHANGEKIND(2i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CHANGEKIND_SETDOCUMENTATION: CHANGEKIND = CHANGEKIND(3i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CHANGEKIND_GENERAL: CHANGEKIND = CHANGEKIND(4i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CHANGEKIND_INVALIDATE: CHANGEKIND = CHANGEKIND(5i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CHANGEKIND_CHANGEFAILED: CHANGEKIND = CHANGEKIND(6i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CHANGEKIND_MAX: CHANGEKIND = CHANGEKIND(7i32);
impl ::core::marker::Copy for CHANGEKIND {}
impl ::core::clone::Clone for CHANGEKIND {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CHANGEKIND {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CHANGEKIND {
    type Abi = Self;
}
impl ::core::fmt::Debug for CHANGEKIND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CHANGEKIND").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CHANGE_ICON_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CIF_SHOWHELP: CHANGE_ICON_FLAGS = CHANGE_ICON_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CIF_SELECTCURRENT: CHANGE_ICON_FLAGS = CHANGE_ICON_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CIF_SELECTDEFAULT: CHANGE_ICON_FLAGS = CHANGE_ICON_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CIF_SELECTFROMFILE: CHANGE_ICON_FLAGS = CHANGE_ICON_FLAGS(8i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CIF_USEICONEXE: CHANGE_ICON_FLAGS = CHANGE_ICON_FLAGS(16i32);
impl ::core::marker::Copy for CHANGE_ICON_FLAGS {}
impl ::core::clone::Clone for CHANGE_ICON_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CHANGE_ICON_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CHANGE_ICON_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CHANGE_ICON_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CHANGE_ICON_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CHANGE_ICON_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CHANGE_ICON_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CHANGE_ICON_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CHANGE_ICON_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CHANGE_ICON_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CHANGE_SOURCE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CSF_SHOWHELP: CHANGE_SOURCE_FLAGS = CHANGE_SOURCE_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CSF_VALIDSOURCE: CHANGE_SOURCE_FLAGS = CHANGE_SOURCE_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CSF_ONLYGETSOURCE: CHANGE_SOURCE_FLAGS = CHANGE_SOURCE_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CSF_EXPLORER: CHANGE_SOURCE_FLAGS = CHANGE_SOURCE_FLAGS(8u32);
impl ::core::marker::Copy for CHANGE_SOURCE_FLAGS {}
impl ::core::clone::Clone for CHANGE_SOURCE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CHANGE_SOURCE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CHANGE_SOURCE_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CHANGE_SOURCE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CHANGE_SOURCE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CHANGE_SOURCE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CHANGE_SOURCE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CHANGE_SOURCE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CHANGE_SOURCE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CHANGE_SOURCE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CLIPBOARD_FORMAT(pub u16);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CF_TEXT: CLIPBOARD_FORMAT = CLIPBOARD_FORMAT(1u16);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CF_BITMAP: CLIPBOARD_FORMAT = CLIPBOARD_FORMAT(2u16);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CF_METAFILEPICT: CLIPBOARD_FORMAT = CLIPBOARD_FORMAT(3u16);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CF_SYLK: CLIPBOARD_FORMAT = CLIPBOARD_FORMAT(4u16);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CF_DIF: CLIPBOARD_FORMAT = CLIPBOARD_FORMAT(5u16);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CF_TIFF: CLIPBOARD_FORMAT = CLIPBOARD_FORMAT(6u16);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CF_OEMTEXT: CLIPBOARD_FORMAT = CLIPBOARD_FORMAT(7u16);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CF_DIB: CLIPBOARD_FORMAT = CLIPBOARD_FORMAT(8u16);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CF_PALETTE: CLIPBOARD_FORMAT = CLIPBOARD_FORMAT(9u16);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CF_PENDATA: CLIPBOARD_FORMAT = CLIPBOARD_FORMAT(10u16);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CF_RIFF: CLIPBOARD_FORMAT = CLIPBOARD_FORMAT(11u16);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CF_WAVE: CLIPBOARD_FORMAT = CLIPBOARD_FORMAT(12u16);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CF_UNICODETEXT: CLIPBOARD_FORMAT = CLIPBOARD_FORMAT(13u16);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CF_ENHMETAFILE: CLIPBOARD_FORMAT = CLIPBOARD_FORMAT(14u16);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CF_HDROP: CLIPBOARD_FORMAT = CLIPBOARD_FORMAT(15u16);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CF_LOCALE: CLIPBOARD_FORMAT = CLIPBOARD_FORMAT(16u16);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CF_DIBV5: CLIPBOARD_FORMAT = CLIPBOARD_FORMAT(17u16);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CF_MAX: CLIPBOARD_FORMAT = CLIPBOARD_FORMAT(18u16);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CF_OWNERDISPLAY: CLIPBOARD_FORMAT = CLIPBOARD_FORMAT(128u16);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CF_DSPTEXT: CLIPBOARD_FORMAT = CLIPBOARD_FORMAT(129u16);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CF_DSPBITMAP: CLIPBOARD_FORMAT = CLIPBOARD_FORMAT(130u16);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CF_DSPMETAFILEPICT: CLIPBOARD_FORMAT = CLIPBOARD_FORMAT(131u16);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CF_DSPENHMETAFILE: CLIPBOARD_FORMAT = CLIPBOARD_FORMAT(142u16);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CF_PRIVATEFIRST: CLIPBOARD_FORMAT = CLIPBOARD_FORMAT(512u16);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CF_PRIVATELAST: CLIPBOARD_FORMAT = CLIPBOARD_FORMAT(767u16);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CF_GDIOBJFIRST: CLIPBOARD_FORMAT = CLIPBOARD_FORMAT(768u16);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CF_GDIOBJLAST: CLIPBOARD_FORMAT = CLIPBOARD_FORMAT(1023u16);
impl ::core::marker::Copy for CLIPBOARD_FORMAT {}
impl ::core::clone::Clone for CLIPBOARD_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CLIPBOARD_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CLIPBOARD_FORMAT {
    type Abi = Self;
}
impl ::core::fmt::Debug for CLIPBOARD_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLIPBOARD_FORMAT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CTRLINFO(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CTRLINFO_EATS_RETURN: CTRLINFO = CTRLINFO(1i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CTRLINFO_EATS_ESCAPE: CTRLINFO = CTRLINFO(2i32);
impl ::core::marker::Copy for CTRLINFO {}
impl ::core::clone::Clone for CTRLINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CTRLINFO {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CTRLINFO {
    type Abi = Self;
}
impl ::core::fmt::Debug for CTRLINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CTRLINFO").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISCARDCACHE(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISCARDCACHE_SAVEIFDIRTY: DISCARDCACHE = DISCARDCACHE(0i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DISCARDCACHE_NOSAVE: DISCARDCACHE = DISCARDCACHE(1i32);
impl ::core::marker::Copy for DISCARDCACHE {}
impl ::core::clone::Clone for DISCARDCACHE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISCARDCACHE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DISCARDCACHE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DISCARDCACHE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISCARDCACHE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DOCMISC(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DOCMISC_CANCREATEMULTIPLEVIEWS: DOCMISC = DOCMISC(1i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DOCMISC_SUPPORTCOMPLEXRECTANGLES: DOCMISC = DOCMISC(2i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DOCMISC_CANTOPENEDIT: DOCMISC = DOCMISC(4i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DOCMISC_NOFILESUPPORT: DOCMISC = DOCMISC(8i32);
impl ::core::marker::Copy for DOCMISC {}
impl ::core::clone::Clone for DOCMISC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DOCMISC {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DOCMISC {
    type Abi = Self;
}
impl ::core::fmt::Debug for DOCMISC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOCMISC").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DROPEFFECT(pub u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DROPEFFECT_NONE: DROPEFFECT = DROPEFFECT(0u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DROPEFFECT_COPY: DROPEFFECT = DROPEFFECT(1u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DROPEFFECT_MOVE: DROPEFFECT = DROPEFFECT(2u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DROPEFFECT_LINK: DROPEFFECT = DROPEFFECT(4u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DROPEFFECT_SCROLL: DROPEFFECT = DROPEFFECT(2147483648u32);
impl ::core::marker::Copy for DROPEFFECT {}
impl ::core::clone::Clone for DROPEFFECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DROPEFFECT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DROPEFFECT {
    type Abi = Self;
}
impl ::core::fmt::Debug for DROPEFFECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DROPEFFECT").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DROPEFFECT {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DROPEFFECT {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DROPEFFECT {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DROPEFFECT {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DROPEFFECT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DVASPECTINFOFLAG(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DVASPECTINFOFLAG_CANOPTIMIZE: DVASPECTINFOFLAG = DVASPECTINFOFLAG(1i32);
impl ::core::marker::Copy for DVASPECTINFOFLAG {}
impl ::core::clone::Clone for DVASPECTINFOFLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DVASPECTINFOFLAG {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DVASPECTINFOFLAG {
    type Abi = Self;
}
impl ::core::fmt::Debug for DVASPECTINFOFLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DVASPECTINFOFLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DVEXTENTMODE(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DVEXTENT_CONTENT: DVEXTENTMODE = DVEXTENTMODE(0i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const DVEXTENT_INTEGRAL: DVEXTENTMODE = DVEXTENTMODE(1i32);
impl ::core::marker::Copy for DVEXTENTMODE {}
impl ::core::clone::Clone for DVEXTENTMODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DVEXTENTMODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DVEXTENTMODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DVEXTENTMODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DVEXTENTMODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EDIT_LINKS_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const ELF_SHOWHELP: EDIT_LINKS_FLAGS = EDIT_LINKS_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const ELF_DISABLEUPDATENOW: EDIT_LINKS_FLAGS = EDIT_LINKS_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const ELF_DISABLEOPENSOURCE: EDIT_LINKS_FLAGS = EDIT_LINKS_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const ELF_DISABLECHANGESOURCE: EDIT_LINKS_FLAGS = EDIT_LINKS_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const ELF_DISABLECANCELLINK: EDIT_LINKS_FLAGS = EDIT_LINKS_FLAGS(16u32);
impl ::core::marker::Copy for EDIT_LINKS_FLAGS {}
impl ::core::clone::Clone for EDIT_LINKS_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EDIT_LINKS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EDIT_LINKS_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for EDIT_LINKS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EDIT_LINKS_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for EDIT_LINKS_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for EDIT_LINKS_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for EDIT_LINKS_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for EDIT_LINKS_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for EDIT_LINKS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EMBDHLP_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const EMBDHLP_INPROC_HANDLER: EMBDHLP_FLAGS = EMBDHLP_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const EMBDHLP_INPROC_SERVER: EMBDHLP_FLAGS = EMBDHLP_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const EMBDHLP_CREATENOW: EMBDHLP_FLAGS = EMBDHLP_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const EMBDHLP_DELAYCREATE: EMBDHLP_FLAGS = EMBDHLP_FLAGS(65536u32);
impl ::core::marker::Copy for EMBDHLP_FLAGS {}
impl ::core::clone::Clone for EMBDHLP_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EMBDHLP_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EMBDHLP_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for EMBDHLP_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EMBDHLP_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for EMBDHLP_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for EMBDHLP_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for EMBDHLP_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for EMBDHLP_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for EMBDHLP_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ENUM_CONTROLS_WHICH_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const GCW_WCH_SIBLING: ENUM_CONTROLS_WHICH_FLAGS = ENUM_CONTROLS_WHICH_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const GC_WCH_CONTAINER: ENUM_CONTROLS_WHICH_FLAGS = ENUM_CONTROLS_WHICH_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const GC_WCH_CONTAINED: ENUM_CONTROLS_WHICH_FLAGS = ENUM_CONTROLS_WHICH_FLAGS(3u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const GC_WCH_ALL: ENUM_CONTROLS_WHICH_FLAGS = ENUM_CONTROLS_WHICH_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const GC_WCH_FREVERSEDIR: ENUM_CONTROLS_WHICH_FLAGS = ENUM_CONTROLS_WHICH_FLAGS(134217728u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const GC_WCH_FONLYAFTER: ENUM_CONTROLS_WHICH_FLAGS = ENUM_CONTROLS_WHICH_FLAGS(268435456u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const GC_WCH_FONLYBEFORE: ENUM_CONTROLS_WHICH_FLAGS = ENUM_CONTROLS_WHICH_FLAGS(536870912u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const GC_WCH_FSELECTED: ENUM_CONTROLS_WHICH_FLAGS = ENUM_CONTROLS_WHICH_FLAGS(1073741824u32);
impl ::core::marker::Copy for ENUM_CONTROLS_WHICH_FLAGS {}
impl ::core::clone::Clone for ENUM_CONTROLS_WHICH_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ENUM_CONTROLS_WHICH_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ENUM_CONTROLS_WHICH_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for ENUM_CONTROLS_WHICH_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ENUM_CONTROLS_WHICH_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FDEX_PROP_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const fdexPropCanGet: FDEX_PROP_FLAGS = FDEX_PROP_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const fdexPropCannotGet: FDEX_PROP_FLAGS = FDEX_PROP_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const fdexPropCanPut: FDEX_PROP_FLAGS = FDEX_PROP_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const fdexPropCannotPut: FDEX_PROP_FLAGS = FDEX_PROP_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const fdexPropCanPutRef: FDEX_PROP_FLAGS = FDEX_PROP_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const fdexPropCannotPutRef: FDEX_PROP_FLAGS = FDEX_PROP_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const fdexPropNoSideEffects: FDEX_PROP_FLAGS = FDEX_PROP_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const fdexPropDynamicType: FDEX_PROP_FLAGS = FDEX_PROP_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const fdexPropCanCall: FDEX_PROP_FLAGS = FDEX_PROP_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const fdexPropCannotCall: FDEX_PROP_FLAGS = FDEX_PROP_FLAGS(512u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const fdexPropCanConstruct: FDEX_PROP_FLAGS = FDEX_PROP_FLAGS(1024u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const fdexPropCannotConstruct: FDEX_PROP_FLAGS = FDEX_PROP_FLAGS(2048u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const fdexPropCanSourceEvents: FDEX_PROP_FLAGS = FDEX_PROP_FLAGS(4096u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const fdexPropCannotSourceEvents: FDEX_PROP_FLAGS = FDEX_PROP_FLAGS(8192u32);
impl ::core::marker::Copy for FDEX_PROP_FLAGS {}
impl ::core::clone::Clone for FDEX_PROP_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FDEX_PROP_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FDEX_PROP_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for FDEX_PROP_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FDEX_PROP_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for FDEX_PROP_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for FDEX_PROP_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for FDEX_PROP_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for FDEX_PROP_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for FDEX_PROP_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GUIDKIND(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const GUIDKIND_DEFAULT_SOURCE_DISP_IID: GUIDKIND = GUIDKIND(1i32);
impl ::core::marker::Copy for GUIDKIND {}
impl ::core::clone::Clone for GUIDKIND {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GUIDKIND {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GUIDKIND {
    type Abi = Self;
}
impl ::core::fmt::Debug for GUIDKIND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GUIDKIND").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HITRESULT(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const HITRESULT_OUTSIDE: HITRESULT = HITRESULT(0i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const HITRESULT_TRANSPARENT: HITRESULT = HITRESULT(1i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const HITRESULT_CLOSE: HITRESULT = HITRESULT(2i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const HITRESULT_HIT: HITRESULT = HITRESULT(3i32);
impl ::core::marker::Copy for HITRESULT {}
impl ::core::clone::Clone for HITRESULT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HITRESULT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HITRESULT {
    type Abi = Self;
}
impl ::core::fmt::Debug for HITRESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HITRESULT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IGNOREMIME(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IGNOREMIME_PROMPT: IGNOREMIME = IGNOREMIME(1i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IGNOREMIME_TEXT: IGNOREMIME = IGNOREMIME(2i32);
impl ::core::marker::Copy for IGNOREMIME {}
impl ::core::clone::Clone for IGNOREMIME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IGNOREMIME {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IGNOREMIME {
    type Abi = Self;
}
impl ::core::fmt::Debug for IGNOREMIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGNOREMIME").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct INSERT_OBJECT_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IOF_SHOWHELP: INSERT_OBJECT_FLAGS = INSERT_OBJECT_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IOF_SELECTCREATENEW: INSERT_OBJECT_FLAGS = INSERT_OBJECT_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IOF_SELECTCREATEFROMFILE: INSERT_OBJECT_FLAGS = INSERT_OBJECT_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IOF_CHECKLINK: INSERT_OBJECT_FLAGS = INSERT_OBJECT_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IOF_CHECKDISPLAYASICON: INSERT_OBJECT_FLAGS = INSERT_OBJECT_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IOF_CREATENEWOBJECT: INSERT_OBJECT_FLAGS = INSERT_OBJECT_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IOF_CREATEFILEOBJECT: INSERT_OBJECT_FLAGS = INSERT_OBJECT_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IOF_CREATELINKOBJECT: INSERT_OBJECT_FLAGS = INSERT_OBJECT_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IOF_DISABLELINK: INSERT_OBJECT_FLAGS = INSERT_OBJECT_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IOF_VERIFYSERVERSEXIST: INSERT_OBJECT_FLAGS = INSERT_OBJECT_FLAGS(512u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IOF_DISABLEDISPLAYASICON: INSERT_OBJECT_FLAGS = INSERT_OBJECT_FLAGS(1024u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IOF_HIDECHANGEICON: INSERT_OBJECT_FLAGS = INSERT_OBJECT_FLAGS(2048u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IOF_SHOWINSERTCONTROL: INSERT_OBJECT_FLAGS = INSERT_OBJECT_FLAGS(4096u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const IOF_SELECTCREATECONTROL: INSERT_OBJECT_FLAGS = INSERT_OBJECT_FLAGS(8192u32);
impl ::core::marker::Copy for INSERT_OBJECT_FLAGS {}
impl ::core::clone::Clone for INSERT_OBJECT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for INSERT_OBJECT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for INSERT_OBJECT_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for INSERT_OBJECT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INSERT_OBJECT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for INSERT_OBJECT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for INSERT_OBJECT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for INSERT_OBJECT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for INSERT_OBJECT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for INSERT_OBJECT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KEYMODIFIERS(pub u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const KEYMOD_SHIFT: KEYMODIFIERS = KEYMODIFIERS(1u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const KEYMOD_CONTROL: KEYMODIFIERS = KEYMODIFIERS(2u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const KEYMOD_ALT: KEYMODIFIERS = KEYMODIFIERS(4u32);
impl ::core::marker::Copy for KEYMODIFIERS {}
impl ::core::clone::Clone for KEYMODIFIERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KEYMODIFIERS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KEYMODIFIERS {
    type Abi = Self;
}
impl ::core::fmt::Debug for KEYMODIFIERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KEYMODIFIERS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for KEYMODIFIERS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for KEYMODIFIERS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for KEYMODIFIERS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for KEYMODIFIERS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for KEYMODIFIERS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LIBFLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const LIBFLAG_FRESTRICTED: LIBFLAGS = LIBFLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const LIBFLAG_FCONTROL: LIBFLAGS = LIBFLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const LIBFLAG_FHIDDEN: LIBFLAGS = LIBFLAGS(4i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const LIBFLAG_FHASDISKIMAGE: LIBFLAGS = LIBFLAGS(8i32);
impl ::core::marker::Copy for LIBFLAGS {}
impl ::core::clone::Clone for LIBFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LIBFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for LIBFLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for LIBFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LIBFLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LOAD_PICTURE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const LP_DEFAULT: LOAD_PICTURE_FLAGS = LOAD_PICTURE_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const LP_MONOCHROME: LOAD_PICTURE_FLAGS = LOAD_PICTURE_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const LP_VGACOLOR: LOAD_PICTURE_FLAGS = LOAD_PICTURE_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const LP_COLOR: LOAD_PICTURE_FLAGS = LOAD_PICTURE_FLAGS(4u32);
impl ::core::marker::Copy for LOAD_PICTURE_FLAGS {}
impl ::core::clone::Clone for LOAD_PICTURE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LOAD_PICTURE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for LOAD_PICTURE_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for LOAD_PICTURE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LOAD_PICTURE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for LOAD_PICTURE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for LOAD_PICTURE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for LOAD_PICTURE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for LOAD_PICTURE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for LOAD_PICTURE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MEDIAPLAYBACK_STATE(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const MEDIAPLAYBACK_RESUME: MEDIAPLAYBACK_STATE = MEDIAPLAYBACK_STATE(0i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const MEDIAPLAYBACK_PAUSE: MEDIAPLAYBACK_STATE = MEDIAPLAYBACK_STATE(1i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const MEDIAPLAYBACK_PAUSE_AND_SUSPEND: MEDIAPLAYBACK_STATE = MEDIAPLAYBACK_STATE(2i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const MEDIAPLAYBACK_RESUME_FROM_SUSPEND: MEDIAPLAYBACK_STATE = MEDIAPLAYBACK_STATE(3i32);
impl ::core::marker::Copy for MEDIAPLAYBACK_STATE {}
impl ::core::clone::Clone for MEDIAPLAYBACK_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MEDIAPLAYBACK_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MEDIAPLAYBACK_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MEDIAPLAYBACK_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MEDIAPLAYBACK_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MULTICLASSINFO_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const MULTICLASSINFO_GETTYPEINFO: MULTICLASSINFO_FLAGS = MULTICLASSINFO_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const MULTICLASSINFO_GETNUMRESERVEDDISPIDS: MULTICLASSINFO_FLAGS = MULTICLASSINFO_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const MULTICLASSINFO_GETIIDPRIMARY: MULTICLASSINFO_FLAGS = MULTICLASSINFO_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const MULTICLASSINFO_GETIIDSOURCE: MULTICLASSINFO_FLAGS = MULTICLASSINFO_FLAGS(8u32);
impl ::core::marker::Copy for MULTICLASSINFO_FLAGS {}
impl ::core::clone::Clone for MULTICLASSINFO_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MULTICLASSINFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MULTICLASSINFO_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for MULTICLASSINFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MULTICLASSINFO_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NUMPARSE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const NUMPRS_LEADING_WHITE: NUMPARSE_FLAGS = NUMPARSE_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const NUMPRS_TRAILING_WHITE: NUMPARSE_FLAGS = NUMPARSE_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const NUMPRS_LEADING_PLUS: NUMPARSE_FLAGS = NUMPARSE_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const NUMPRS_TRAILING_PLUS: NUMPARSE_FLAGS = NUMPARSE_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const NUMPRS_LEADING_MINUS: NUMPARSE_FLAGS = NUMPARSE_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const NUMPRS_TRAILING_MINUS: NUMPARSE_FLAGS = NUMPARSE_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const NUMPRS_HEX_OCT: NUMPARSE_FLAGS = NUMPARSE_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const NUMPRS_PARENS: NUMPARSE_FLAGS = NUMPARSE_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const NUMPRS_DECIMAL: NUMPARSE_FLAGS = NUMPARSE_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const NUMPRS_THOUSANDS: NUMPARSE_FLAGS = NUMPARSE_FLAGS(512u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const NUMPRS_CURRENCY: NUMPARSE_FLAGS = NUMPARSE_FLAGS(1024u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const NUMPRS_EXPONENT: NUMPARSE_FLAGS = NUMPARSE_FLAGS(2048u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const NUMPRS_USE_ALL: NUMPARSE_FLAGS = NUMPARSE_FLAGS(4096u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const NUMPRS_STD: NUMPARSE_FLAGS = NUMPARSE_FLAGS(8191u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const NUMPRS_NEG: NUMPARSE_FLAGS = NUMPARSE_FLAGS(65536u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const NUMPRS_INEXACT: NUMPARSE_FLAGS = NUMPARSE_FLAGS(131072u32);
impl ::core::marker::Copy for NUMPARSE_FLAGS {}
impl ::core::clone::Clone for NUMPARSE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NUMPARSE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for NUMPARSE_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for NUMPARSE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NUMPARSE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for NUMPARSE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for NUMPARSE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for NUMPARSE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for NUMPARSE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for NUMPARSE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OBJECT_PROPERTIES_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OPF_OBJECTISLINK: OBJECT_PROPERTIES_FLAGS = OBJECT_PROPERTIES_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OPF_NOFILLDEFAULT: OBJECT_PROPERTIES_FLAGS = OBJECT_PROPERTIES_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OPF_SHOWHELP: OBJECT_PROPERTIES_FLAGS = OBJECT_PROPERTIES_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OPF_DISABLECONVERT: OBJECT_PROPERTIES_FLAGS = OBJECT_PROPERTIES_FLAGS(8u32);
impl ::core::marker::Copy for OBJECT_PROPERTIES_FLAGS {}
impl ::core::clone::Clone for OBJECT_PROPERTIES_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OBJECT_PROPERTIES_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for OBJECT_PROPERTIES_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for OBJECT_PROPERTIES_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OBJECT_PROPERTIES_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for OBJECT_PROPERTIES_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for OBJECT_PROPERTIES_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for OBJECT_PROPERTIES_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for OBJECT_PROPERTIES_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for OBJECT_PROPERTIES_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OLECLOSE(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECLOSE_SAVEIFDIRTY: OLECLOSE = OLECLOSE(0i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECLOSE_NOSAVE: OLECLOSE = OLECLOSE(1i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECLOSE_PROMPTSAVE: OLECLOSE = OLECLOSE(2i32);
impl ::core::marker::Copy for OLECLOSE {}
impl ::core::clone::Clone for OLECLOSE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OLECLOSE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for OLECLOSE {
    type Abi = Self;
}
impl ::core::fmt::Debug for OLECLOSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OLECLOSE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OLECMDEXECOPT(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDEXECOPT_DODEFAULT: OLECMDEXECOPT = OLECMDEXECOPT(0i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDEXECOPT_PROMPTUSER: OLECMDEXECOPT = OLECMDEXECOPT(1i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDEXECOPT_DONTPROMPTUSER: OLECMDEXECOPT = OLECMDEXECOPT(2i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDEXECOPT_SHOWHELP: OLECMDEXECOPT = OLECMDEXECOPT(3i32);
impl ::core::marker::Copy for OLECMDEXECOPT {}
impl ::core::clone::Clone for OLECMDEXECOPT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OLECMDEXECOPT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for OLECMDEXECOPT {
    type Abi = Self;
}
impl ::core::fmt::Debug for OLECMDEXECOPT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OLECMDEXECOPT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OLECMDF(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDF_SUPPORTED: OLECMDF = OLECMDF(1i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDF_ENABLED: OLECMDF = OLECMDF(2i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDF_LATCHED: OLECMDF = OLECMDF(4i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDF_NINCHED: OLECMDF = OLECMDF(8i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDF_INVISIBLE: OLECMDF = OLECMDF(16i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDF_DEFHIDEONCTXTMENU: OLECMDF = OLECMDF(32i32);
impl ::core::marker::Copy for OLECMDF {}
impl ::core::clone::Clone for OLECMDF {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OLECMDF {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for OLECMDF {
    type Abi = Self;
}
impl ::core::fmt::Debug for OLECMDF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OLECMDF").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OLECMDID(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_OPEN: OLECMDID = OLECMDID(1i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_NEW: OLECMDID = OLECMDID(2i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_SAVE: OLECMDID = OLECMDID(3i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_SAVEAS: OLECMDID = OLECMDID(4i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_SAVECOPYAS: OLECMDID = OLECMDID(5i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_PRINT: OLECMDID = OLECMDID(6i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_PRINTPREVIEW: OLECMDID = OLECMDID(7i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_PAGESETUP: OLECMDID = OLECMDID(8i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_SPELL: OLECMDID = OLECMDID(9i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_PROPERTIES: OLECMDID = OLECMDID(10i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_CUT: OLECMDID = OLECMDID(11i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_COPY: OLECMDID = OLECMDID(12i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_PASTE: OLECMDID = OLECMDID(13i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_PASTESPECIAL: OLECMDID = OLECMDID(14i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_UNDO: OLECMDID = OLECMDID(15i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_REDO: OLECMDID = OLECMDID(16i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_SELECTALL: OLECMDID = OLECMDID(17i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_CLEARSELECTION: OLECMDID = OLECMDID(18i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_ZOOM: OLECMDID = OLECMDID(19i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_GETZOOMRANGE: OLECMDID = OLECMDID(20i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_UPDATECOMMANDS: OLECMDID = OLECMDID(21i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_REFRESH: OLECMDID = OLECMDID(22i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_STOP: OLECMDID = OLECMDID(23i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_HIDETOOLBARS: OLECMDID = OLECMDID(24i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_SETPROGRESSMAX: OLECMDID = OLECMDID(25i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_SETPROGRESSPOS: OLECMDID = OLECMDID(26i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_SETPROGRESSTEXT: OLECMDID = OLECMDID(27i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_SETTITLE: OLECMDID = OLECMDID(28i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_SETDOWNLOADSTATE: OLECMDID = OLECMDID(29i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_STOPDOWNLOAD: OLECMDID = OLECMDID(30i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_ONTOOLBARACTIVATED: OLECMDID = OLECMDID(31i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_FIND: OLECMDID = OLECMDID(32i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_DELETE: OLECMDID = OLECMDID(33i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_HTTPEQUIV: OLECMDID = OLECMDID(34i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_HTTPEQUIV_DONE: OLECMDID = OLECMDID(35i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_ENABLE_INTERACTION: OLECMDID = OLECMDID(36i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_ONUNLOAD: OLECMDID = OLECMDID(37i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_PROPERTYBAG2: OLECMDID = OLECMDID(38i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_PREREFRESH: OLECMDID = OLECMDID(39i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_SHOWSCRIPTERROR: OLECMDID = OLECMDID(40i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_SHOWMESSAGE: OLECMDID = OLECMDID(41i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_SHOWFIND: OLECMDID = OLECMDID(42i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_SHOWPAGESETUP: OLECMDID = OLECMDID(43i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_SHOWPRINT: OLECMDID = OLECMDID(44i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_CLOSE: OLECMDID = OLECMDID(45i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_ALLOWUILESSSAVEAS: OLECMDID = OLECMDID(46i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_DONTDOWNLOADCSS: OLECMDID = OLECMDID(47i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_UPDATEPAGESTATUS: OLECMDID = OLECMDID(48i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_PRINT2: OLECMDID = OLECMDID(49i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_PRINTPREVIEW2: OLECMDID = OLECMDID(50i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_SETPRINTTEMPLATE: OLECMDID = OLECMDID(51i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_GETPRINTTEMPLATE: OLECMDID = OLECMDID(52i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_PAGEACTIONBLOCKED: OLECMDID = OLECMDID(55i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_PAGEACTIONUIQUERY: OLECMDID = OLECMDID(56i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_FOCUSVIEWCONTROLS: OLECMDID = OLECMDID(57i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_FOCUSVIEWCONTROLSQUERY: OLECMDID = OLECMDID(58i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_SHOWPAGEACTIONMENU: OLECMDID = OLECMDID(59i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_ADDTRAVELENTRY: OLECMDID = OLECMDID(60i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_UPDATETRAVELENTRY: OLECMDID = OLECMDID(61i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_UPDATEBACKFORWARDSTATE: OLECMDID = OLECMDID(62i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_OPTICAL_ZOOM: OLECMDID = OLECMDID(63i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_OPTICAL_GETZOOMRANGE: OLECMDID = OLECMDID(64i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_WINDOWSTATECHANGED: OLECMDID = OLECMDID(65i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_ACTIVEXINSTALLSCOPE: OLECMDID = OLECMDID(66i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_UPDATETRAVELENTRY_DATARECOVERY: OLECMDID = OLECMDID(67i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_SHOWTASKDLG: OLECMDID = OLECMDID(68i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_POPSTATEEVENT: OLECMDID = OLECMDID(69i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_VIEWPORT_MODE: OLECMDID = OLECMDID(70i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_LAYOUT_VIEWPORT_WIDTH: OLECMDID = OLECMDID(71i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_VISUAL_VIEWPORT_EXCLUDE_BOTTOM: OLECMDID = OLECMDID(72i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_USER_OPTICAL_ZOOM: OLECMDID = OLECMDID(73i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_PAGEAVAILABLE: OLECMDID = OLECMDID(74i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_GETUSERSCALABLE: OLECMDID = OLECMDID(75i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_UPDATE_CARET: OLECMDID = OLECMDID(76i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_ENABLE_VISIBILITY: OLECMDID = OLECMDID(77i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_MEDIA_PLAYBACK: OLECMDID = OLECMDID(78i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_SETFAVICON: OLECMDID = OLECMDID(79i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_SET_HOST_FULLSCREENMODE: OLECMDID = OLECMDID(80i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_EXITFULLSCREEN: OLECMDID = OLECMDID(81i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_SCROLLCOMPLETE: OLECMDID = OLECMDID(82i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_ONBEFOREUNLOAD: OLECMDID = OLECMDID(83i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_SHOWMESSAGE_BLOCKABLE: OLECMDID = OLECMDID(84i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDID_SHOWTASKDLG_BLOCKABLE: OLECMDID = OLECMDID(85i32);
impl ::core::marker::Copy for OLECMDID {}
impl ::core::clone::Clone for OLECMDID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OLECMDID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for OLECMDID {
    type Abi = Self;
}
impl ::core::fmt::Debug for OLECMDID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OLECMDID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OLECMDID_BROWSERSTATEFLAG(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_BROWSERSTATE_EXTENSIONSOFF: OLECMDID_BROWSERSTATEFLAG = OLECMDID_BROWSERSTATEFLAG(1i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_BROWSERSTATE_IESECURITY: OLECMDID_BROWSERSTATEFLAG = OLECMDID_BROWSERSTATEFLAG(2i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_BROWSERSTATE_PROTECTEDMODE_OFF: OLECMDID_BROWSERSTATEFLAG = OLECMDID_BROWSERSTATEFLAG(4i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_BROWSERSTATE_RESET: OLECMDID_BROWSERSTATEFLAG = OLECMDID_BROWSERSTATEFLAG(8i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_BROWSERSTATE_REQUIRESACTIVEX: OLECMDID_BROWSERSTATEFLAG = OLECMDID_BROWSERSTATEFLAG(16i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_BROWSERSTATE_DESKTOPHTMLDIALOG: OLECMDID_BROWSERSTATEFLAG = OLECMDID_BROWSERSTATEFLAG(32i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_BROWSERSTATE_BLOCKEDVERSION: OLECMDID_BROWSERSTATEFLAG = OLECMDID_BROWSERSTATEFLAG(64i32);
impl ::core::marker::Copy for OLECMDID_BROWSERSTATEFLAG {}
impl ::core::clone::Clone for OLECMDID_BROWSERSTATEFLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OLECMDID_BROWSERSTATEFLAG {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for OLECMDID_BROWSERSTATEFLAG {
    type Abi = Self;
}
impl ::core::fmt::Debug for OLECMDID_BROWSERSTATEFLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OLECMDID_BROWSERSTATEFLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OLECMDID_OPTICAL_ZOOMFLAG(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_OPTICAL_ZOOM_NOPERSIST: OLECMDID_OPTICAL_ZOOMFLAG = OLECMDID_OPTICAL_ZOOMFLAG(1i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_OPTICAL_ZOOM_NOLAYOUT: OLECMDID_OPTICAL_ZOOMFLAG = OLECMDID_OPTICAL_ZOOMFLAG(16i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_OPTICAL_ZOOM_NOTRANSIENT: OLECMDID_OPTICAL_ZOOMFLAG = OLECMDID_OPTICAL_ZOOMFLAG(32i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_OPTICAL_ZOOM_RELOADFORNEWTAB: OLECMDID_OPTICAL_ZOOMFLAG = OLECMDID_OPTICAL_ZOOMFLAG(64i32);
impl ::core::marker::Copy for OLECMDID_OPTICAL_ZOOMFLAG {}
impl ::core::clone::Clone for OLECMDID_OPTICAL_ZOOMFLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OLECMDID_OPTICAL_ZOOMFLAG {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for OLECMDID_OPTICAL_ZOOMFLAG {
    type Abi = Self;
}
impl ::core::fmt::Debug for OLECMDID_OPTICAL_ZOOMFLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OLECMDID_OPTICAL_ZOOMFLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OLECMDID_PAGEACTIONFLAG(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_FILEDOWNLOAD: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(1i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_ACTIVEXINSTALL: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(2i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_ACTIVEXTRUSTFAIL: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(4i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_ACTIVEXUSERDISABLE: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(8i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_ACTIVEXDISALLOW: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(16i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_ACTIVEXUNSAFE: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(32i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_POPUPWINDOW: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(64i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_LOCALMACHINE: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(128i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_MIMETEXTPLAIN: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(256i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_SCRIPTNAVIGATE: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(512i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_SCRIPTNAVIGATE_ACTIVEXINSTALL: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(512i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_PROTLOCKDOWNLOCALMACHINE: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(1024i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_PROTLOCKDOWNTRUSTED: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(2048i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_PROTLOCKDOWNINTRANET: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(4096i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_PROTLOCKDOWNINTERNET: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(8192i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_PROTLOCKDOWNRESTRICTED: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(16384i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_PROTLOCKDOWNDENY: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(32768i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_POPUPALLOWED: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(65536i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_SCRIPTPROMPT: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(131072i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_ACTIVEXUSERAPPROVAL: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(262144i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_MIXEDCONTENT: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(524288i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_INVALID_CERT: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(1048576i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_INTRANETZONEREQUEST: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(2097152i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_XSSFILTERED: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(4194304i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_SPOOFABLEIDNHOST: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(8388608i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_ACTIVEX_EPM_INCOMPATIBLE: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(16777216i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_SCRIPTNAVIGATE_ACTIVEXUSERAPPROVAL: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(33554432i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_WPCBLOCKED: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(67108864i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_WPCBLOCKED_ACTIVEX: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(134217728i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_EXTENSION_COMPAT_BLOCKED: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(268435456i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_NORESETACTIVEX: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(536870912i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_GENERIC_STATE: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(1073741824i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_PAGEACTION_RESET: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(-2147483648i32);
impl ::core::marker::Copy for OLECMDID_PAGEACTIONFLAG {}
impl ::core::clone::Clone for OLECMDID_PAGEACTIONFLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OLECMDID_PAGEACTIONFLAG {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for OLECMDID_PAGEACTIONFLAG {
    type Abi = Self;
}
impl ::core::fmt::Debug for OLECMDID_PAGEACTIONFLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OLECMDID_PAGEACTIONFLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OLECMDID_REFRESHFLAG(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_REFRESH_NORMAL: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(0i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_REFRESH_IFEXPIRED: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(1i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_REFRESH_CONTINUE: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(2i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_REFRESH_COMPLETELY: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(3i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_REFRESH_NO_CACHE: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(4i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_REFRESH_RELOAD: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(5i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_REFRESH_LEVELMASK: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(255i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_REFRESH_CLEARUSERINPUT: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(4096i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_REFRESH_PROMPTIFOFFLINE: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(8192i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_REFRESH_THROUGHSCRIPT: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(16384i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_REFRESH_SKIPBEFOREUNLOADEVENT: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(32768i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_REFRESH_PAGEACTION_ACTIVEXINSTALL: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(65536i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_REFRESH_PAGEACTION_FILEDOWNLOAD: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(131072i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_REFRESH_PAGEACTION_LOCALMACHINE: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(262144i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_REFRESH_PAGEACTION_POPUPWINDOW: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(524288i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_REFRESH_PAGEACTION_PROTLOCKDOWNLOCALMACHINE: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(1048576i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_REFRESH_PAGEACTION_PROTLOCKDOWNTRUSTED: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(2097152i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_REFRESH_PAGEACTION_PROTLOCKDOWNINTRANET: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(4194304i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_REFRESH_PAGEACTION_PROTLOCKDOWNINTERNET: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(8388608i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_REFRESH_PAGEACTION_PROTLOCKDOWNRESTRICTED: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(16777216i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_REFRESH_PAGEACTION_MIXEDCONTENT: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(33554432i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_REFRESH_PAGEACTION_INVALID_CERT: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(67108864i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_REFRESH_PAGEACTION_ALLOW_VERSION: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(134217728i32);
impl ::core::marker::Copy for OLECMDID_REFRESHFLAG {}
impl ::core::clone::Clone for OLECMDID_REFRESHFLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OLECMDID_REFRESHFLAG {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for OLECMDID_REFRESHFLAG {
    type Abi = Self;
}
impl ::core::fmt::Debug for OLECMDID_REFRESHFLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OLECMDID_REFRESHFLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OLECMDID_VIEWPORT_MODE_FLAG(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_VIEWPORTMODE_FIXED_LAYOUT_WIDTH: OLECMDID_VIEWPORT_MODE_FLAG = OLECMDID_VIEWPORT_MODE_FLAG(1i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_VIEWPORTMODE_EXCLUDE_VISUAL_BOTTOM: OLECMDID_VIEWPORT_MODE_FLAG = OLECMDID_VIEWPORT_MODE_FLAG(2i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_VIEWPORTMODE_FIXED_LAYOUT_WIDTH_VALID: OLECMDID_VIEWPORT_MODE_FLAG = OLECMDID_VIEWPORT_MODE_FLAG(65536i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_VIEWPORTMODE_EXCLUDE_VISUAL_BOTTOM_VALID: OLECMDID_VIEWPORT_MODE_FLAG = OLECMDID_VIEWPORT_MODE_FLAG(131072i32);
impl ::core::marker::Copy for OLECMDID_VIEWPORT_MODE_FLAG {}
impl ::core::clone::Clone for OLECMDID_VIEWPORT_MODE_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OLECMDID_VIEWPORT_MODE_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for OLECMDID_VIEWPORT_MODE_FLAG {
    type Abi = Self;
}
impl ::core::fmt::Debug for OLECMDID_VIEWPORT_MODE_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OLECMDID_VIEWPORT_MODE_FLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OLECMDID_WINDOWSTATE_FLAG(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_WINDOWSTATE_USERVISIBLE: OLECMDID_WINDOWSTATE_FLAG = OLECMDID_WINDOWSTATE_FLAG(1i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_WINDOWSTATE_ENABLED: OLECMDID_WINDOWSTATE_FLAG = OLECMDID_WINDOWSTATE_FLAG(2i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_WINDOWSTATE_USERVISIBLE_VALID: OLECMDID_WINDOWSTATE_FLAG = OLECMDID_WINDOWSTATE_FLAG(65536i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDIDF_WINDOWSTATE_ENABLED_VALID: OLECMDID_WINDOWSTATE_FLAG = OLECMDID_WINDOWSTATE_FLAG(131072i32);
impl ::core::marker::Copy for OLECMDID_WINDOWSTATE_FLAG {}
impl ::core::clone::Clone for OLECMDID_WINDOWSTATE_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OLECMDID_WINDOWSTATE_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for OLECMDID_WINDOWSTATE_FLAG {
    type Abi = Self;
}
impl ::core::fmt::Debug for OLECMDID_WINDOWSTATE_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OLECMDID_WINDOWSTATE_FLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OLECMDTEXTF(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDTEXTF_NONE: OLECMDTEXTF = OLECMDTEXTF(0i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDTEXTF_NAME: OLECMDTEXTF = OLECMDTEXTF(1i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECMDTEXTF_STATUS: OLECMDTEXTF = OLECMDTEXTF(2i32);
impl ::core::marker::Copy for OLECMDTEXTF {}
impl ::core::clone::Clone for OLECMDTEXTF {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OLECMDTEXTF {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for OLECMDTEXTF {
    type Abi = Self;
}
impl ::core::fmt::Debug for OLECMDTEXTF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OLECMDTEXTF").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OLECONTF(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECONTF_EMBEDDINGS: OLECONTF = OLECONTF(1i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECONTF_LINKS: OLECONTF = OLECONTF(2i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECONTF_OTHERS: OLECONTF = OLECONTF(4i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECONTF_ONLYUSER: OLECONTF = OLECONTF(8i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECONTF_ONLYIFRUNNING: OLECONTF = OLECONTF(16i32);
impl ::core::marker::Copy for OLECONTF {}
impl ::core::clone::Clone for OLECONTF {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OLECONTF {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for OLECONTF {
    type Abi = Self;
}
impl ::core::fmt::Debug for OLECONTF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OLECONTF").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OLECREATE(pub u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECREATE_ZERO: OLECREATE = OLECREATE(0u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLECREATE_LEAVERUNNING: OLECREATE = OLECREATE(1u32);
impl ::core::marker::Copy for OLECREATE {}
impl ::core::clone::Clone for OLECREATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OLECREATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for OLECREATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for OLECREATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OLECREATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OLEDCFLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEDC_NODRAW: OLEDCFLAGS = OLEDCFLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEDC_PAINTBKGND: OLEDCFLAGS = OLEDCFLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEDC_OFFSCREEN: OLEDCFLAGS = OLEDCFLAGS(4i32);
impl ::core::marker::Copy for OLEDCFLAGS {}
impl ::core::clone::Clone for OLEDCFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OLEDCFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for OLEDCFLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for OLEDCFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OLEDCFLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OLEGETMONIKER(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEGETMONIKER_ONLYIFTHERE: OLEGETMONIKER = OLEGETMONIKER(1i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEGETMONIKER_FORCEASSIGN: OLEGETMONIKER = OLEGETMONIKER(2i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEGETMONIKER_UNASSIGN: OLEGETMONIKER = OLEGETMONIKER(3i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEGETMONIKER_TEMPFORUSER: OLEGETMONIKER = OLEGETMONIKER(4i32);
impl ::core::marker::Copy for OLEGETMONIKER {}
impl ::core::clone::Clone for OLEGETMONIKER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OLEGETMONIKER {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for OLEGETMONIKER {
    type Abi = Self;
}
impl ::core::fmt::Debug for OLEGETMONIKER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OLEGETMONIKER").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OLEIVERB(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEIVERB_PRIMARY: OLEIVERB = OLEIVERB(0i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEIVERB_SHOW: OLEIVERB = OLEIVERB(-1i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEIVERB_OPEN: OLEIVERB = OLEIVERB(-2i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEIVERB_HIDE: OLEIVERB = OLEIVERB(-3i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEIVERB_UIACTIVATE: OLEIVERB = OLEIVERB(-4i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEIVERB_INPLACEACTIVATE: OLEIVERB = OLEIVERB(-5i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEIVERB_DISCARDUNDOSTATE: OLEIVERB = OLEIVERB(-6i32);
impl ::core::marker::Copy for OLEIVERB {}
impl ::core::clone::Clone for OLEIVERB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OLEIVERB {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for OLEIVERB {
    type Abi = Self;
}
impl ::core::fmt::Debug for OLEIVERB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OLEIVERB").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OLELINKBIND(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLELINKBIND_EVENIFCLASSDIFF: OLELINKBIND = OLELINKBIND(1i32);
impl ::core::marker::Copy for OLELINKBIND {}
impl ::core::clone::Clone for OLELINKBIND {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OLELINKBIND {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for OLELINKBIND {
    type Abi = Self;
}
impl ::core::fmt::Debug for OLELINKBIND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OLELINKBIND").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OLEMISC(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEMISC_RECOMPOSEONRESIZE: OLEMISC = OLEMISC(1i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEMISC_ONLYICONIC: OLEMISC = OLEMISC(2i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEMISC_INSERTNOTREPLACE: OLEMISC = OLEMISC(4i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEMISC_STATIC: OLEMISC = OLEMISC(8i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEMISC_CANTLINKINSIDE: OLEMISC = OLEMISC(16i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEMISC_CANLINKBYOLE1: OLEMISC = OLEMISC(32i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEMISC_ISLINKOBJECT: OLEMISC = OLEMISC(64i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEMISC_INSIDEOUT: OLEMISC = OLEMISC(128i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEMISC_ACTIVATEWHENVISIBLE: OLEMISC = OLEMISC(256i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEMISC_RENDERINGISDEVICEINDEPENDENT: OLEMISC = OLEMISC(512i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEMISC_INVISIBLEATRUNTIME: OLEMISC = OLEMISC(1024i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEMISC_ALWAYSRUN: OLEMISC = OLEMISC(2048i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEMISC_ACTSLIKEBUTTON: OLEMISC = OLEMISC(4096i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEMISC_ACTSLIKELABEL: OLEMISC = OLEMISC(8192i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEMISC_NOUIACTIVATE: OLEMISC = OLEMISC(16384i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEMISC_ALIGNABLE: OLEMISC = OLEMISC(32768i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEMISC_SIMPLEFRAME: OLEMISC = OLEMISC(65536i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEMISC_SETCLIENTSITEFIRST: OLEMISC = OLEMISC(131072i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEMISC_IMEMODE: OLEMISC = OLEMISC(262144i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEMISC_IGNOREACTIVATEWHENVISIBLE: OLEMISC = OLEMISC(524288i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEMISC_WANTSTOMENUMERGE: OLEMISC = OLEMISC(1048576i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEMISC_SUPPORTSMULTILEVELUNDO: OLEMISC = OLEMISC(2097152i32);
impl ::core::marker::Copy for OLEMISC {}
impl ::core::clone::Clone for OLEMISC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OLEMISC {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for OLEMISC {
    type Abi = Self;
}
impl ::core::fmt::Debug for OLEMISC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OLEMISC").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OLERENDER(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLERENDER_NONE: OLERENDER = OLERENDER(0i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLERENDER_DRAW: OLERENDER = OLERENDER(1i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLERENDER_FORMAT: OLERENDER = OLERENDER(2i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLERENDER_ASIS: OLERENDER = OLERENDER(3i32);
impl ::core::marker::Copy for OLERENDER {}
impl ::core::clone::Clone for OLERENDER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OLERENDER {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for OLERENDER {
    type Abi = Self;
}
impl ::core::fmt::Debug for OLERENDER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OLERENDER").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OLEUIPASTEFLAG(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUIPASTE_ENABLEICON: OLEUIPASTEFLAG = OLEUIPASTEFLAG(2048i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUIPASTE_PASTEONLY: OLEUIPASTEFLAG = OLEUIPASTEFLAG(0i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUIPASTE_PASTE: OLEUIPASTEFLAG = OLEUIPASTEFLAG(512i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUIPASTE_LINKANYTYPE: OLEUIPASTEFLAG = OLEUIPASTEFLAG(1024i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUIPASTE_LINKTYPE1: OLEUIPASTEFLAG = OLEUIPASTEFLAG(1i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUIPASTE_LINKTYPE2: OLEUIPASTEFLAG = OLEUIPASTEFLAG(2i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUIPASTE_LINKTYPE3: OLEUIPASTEFLAG = OLEUIPASTEFLAG(4i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUIPASTE_LINKTYPE4: OLEUIPASTEFLAG = OLEUIPASTEFLAG(8i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUIPASTE_LINKTYPE5: OLEUIPASTEFLAG = OLEUIPASTEFLAG(16i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUIPASTE_LINKTYPE6: OLEUIPASTEFLAG = OLEUIPASTEFLAG(32i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUIPASTE_LINKTYPE7: OLEUIPASTEFLAG = OLEUIPASTEFLAG(64i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUIPASTE_LINKTYPE8: OLEUIPASTEFLAG = OLEUIPASTEFLAG(128i32);
impl ::core::marker::Copy for OLEUIPASTEFLAG {}
impl ::core::clone::Clone for OLEUIPASTEFLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OLEUIPASTEFLAG {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for OLEUIPASTEFLAG {
    type Abi = Self;
}
impl ::core::fmt::Debug for OLEUIPASTEFLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OLEUIPASTEFLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OLEUPDATE(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUPDATE_ALWAYS: OLEUPDATE = OLEUPDATE(1i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEUPDATE_ONCALL: OLEUPDATE = OLEUPDATE(3i32);
impl ::core::marker::Copy for OLEUPDATE {}
impl ::core::clone::Clone for OLEUPDATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OLEUPDATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for OLEUPDATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for OLEUPDATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OLEUPDATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OLEVERBATTRIB(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEVERBATTRIB_NEVERDIRTIES: OLEVERBATTRIB = OLEVERBATTRIB(1i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEVERBATTRIB_ONCONTAINERMENU: OLEVERBATTRIB = OLEVERBATTRIB(2i32);
impl ::core::marker::Copy for OLEVERBATTRIB {}
impl ::core::clone::Clone for OLEVERBATTRIB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OLEVERBATTRIB {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for OLEVERBATTRIB {
    type Abi = Self;
}
impl ::core::fmt::Debug for OLEVERBATTRIB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OLEVERBATTRIB").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OLEWHICHMK(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEWHICHMK_CONTAINER: OLEWHICHMK = OLEWHICHMK(1i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEWHICHMK_OBJREL: OLEWHICHMK = OLEWHICHMK(2i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const OLEWHICHMK_OBJFULL: OLEWHICHMK = OLEWHICHMK(3i32);
impl ::core::marker::Copy for OLEWHICHMK {}
impl ::core::clone::Clone for OLEWHICHMK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OLEWHICHMK {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for OLEWHICHMK {
    type Abi = Self;
}
impl ::core::fmt::Debug for OLEWHICHMK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OLEWHICHMK").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OLE_TRISTATE(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const triUnchecked: OLE_TRISTATE = OLE_TRISTATE(0i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const triChecked: OLE_TRISTATE = OLE_TRISTATE(1i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const triGray: OLE_TRISTATE = OLE_TRISTATE(2i32);
impl ::core::marker::Copy for OLE_TRISTATE {}
impl ::core::clone::Clone for OLE_TRISTATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OLE_TRISTATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for OLE_TRISTATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for OLE_TRISTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OLE_TRISTATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PAGEACTION_UI(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PAGEACTION_UI_DEFAULT: PAGEACTION_UI = PAGEACTION_UI(0i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PAGEACTION_UI_MODAL: PAGEACTION_UI = PAGEACTION_UI(1i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PAGEACTION_UI_MODELESS: PAGEACTION_UI = PAGEACTION_UI(2i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PAGEACTION_UI_SILENT: PAGEACTION_UI = PAGEACTION_UI(3i32);
impl ::core::marker::Copy for PAGEACTION_UI {}
impl ::core::clone::Clone for PAGEACTION_UI {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PAGEACTION_UI {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PAGEACTION_UI {
    type Abi = Self;
}
impl ::core::fmt::Debug for PAGEACTION_UI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PAGEACTION_UI").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PARAMFLAGS(pub u16);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PARAMFLAG_NONE: PARAMFLAGS = PARAMFLAGS(0u16);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PARAMFLAG_FIN: PARAMFLAGS = PARAMFLAGS(1u16);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PARAMFLAG_FOUT: PARAMFLAGS = PARAMFLAGS(2u16);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PARAMFLAG_FLCID: PARAMFLAGS = PARAMFLAGS(4u16);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PARAMFLAG_FRETVAL: PARAMFLAGS = PARAMFLAGS(8u16);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PARAMFLAG_FOPT: PARAMFLAGS = PARAMFLAGS(16u16);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PARAMFLAG_FHASDEFAULT: PARAMFLAGS = PARAMFLAGS(32u16);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PARAMFLAG_FHASCUSTDATA: PARAMFLAGS = PARAMFLAGS(64u16);
impl ::core::marker::Copy for PARAMFLAGS {}
impl ::core::clone::Clone for PARAMFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PARAMFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PARAMFLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for PARAMFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PARAMFLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PARAMFLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PARAMFLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PARAMFLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PARAMFLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PARAMFLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PASTE_SPECIAL_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PSF_SHOWHELP: PASTE_SPECIAL_FLAGS = PASTE_SPECIAL_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PSF_SELECTPASTE: PASTE_SPECIAL_FLAGS = PASTE_SPECIAL_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PSF_SELECTPASTELINK: PASTE_SPECIAL_FLAGS = PASTE_SPECIAL_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PSF_CHECKDISPLAYASICON: PASTE_SPECIAL_FLAGS = PASTE_SPECIAL_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PSF_DISABLEDISPLAYASICON: PASTE_SPECIAL_FLAGS = PASTE_SPECIAL_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PSF_HIDECHANGEICON: PASTE_SPECIAL_FLAGS = PASTE_SPECIAL_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PSF_STAYONCLIPBOARDCHANGE: PASTE_SPECIAL_FLAGS = PASTE_SPECIAL_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PSF_NOREFRESHDATAOBJECT: PASTE_SPECIAL_FLAGS = PASTE_SPECIAL_FLAGS(128u32);
impl ::core::marker::Copy for PASTE_SPECIAL_FLAGS {}
impl ::core::clone::Clone for PASTE_SPECIAL_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PASTE_SPECIAL_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PASTE_SPECIAL_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for PASTE_SPECIAL_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PASTE_SPECIAL_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PASTE_SPECIAL_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PASTE_SPECIAL_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PASTE_SPECIAL_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PASTE_SPECIAL_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PASTE_SPECIAL_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PICTUREATTRIBUTES(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PICTURE_SCALABLE: PICTUREATTRIBUTES = PICTUREATTRIBUTES(1i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PICTURE_TRANSPARENT: PICTUREATTRIBUTES = PICTUREATTRIBUTES(2i32);
impl ::core::marker::Copy for PICTUREATTRIBUTES {}
impl ::core::clone::Clone for PICTUREATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PICTUREATTRIBUTES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PICTUREATTRIBUTES {
    type Abi = Self;
}
impl ::core::fmt::Debug for PICTUREATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PICTUREATTRIBUTES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PICTYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PICTYPE_UNINITIALIZED: PICTYPE = PICTYPE(-1i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PICTYPE_NONE: PICTYPE = PICTYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PICTYPE_BITMAP: PICTYPE = PICTYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PICTYPE_METAFILE: PICTYPE = PICTYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PICTYPE_ICON: PICTYPE = PICTYPE(3i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PICTYPE_ENHMETAFILE: PICTYPE = PICTYPE(4i32);
impl ::core::marker::Copy for PICTYPE {}
impl ::core::clone::Clone for PICTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PICTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PICTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for PICTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PICTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct POINTERINACTIVE(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const POINTERINACTIVE_ACTIVATEONENTRY: POINTERINACTIVE = POINTERINACTIVE(1i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const POINTERINACTIVE_DEACTIVATEONLEAVE: POINTERINACTIVE = POINTERINACTIVE(2i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const POINTERINACTIVE_ACTIVATEONDRAG: POINTERINACTIVE = POINTERINACTIVE(4i32);
impl ::core::marker::Copy for POINTERINACTIVE {}
impl ::core::clone::Clone for POINTERINACTIVE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for POINTERINACTIVE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for POINTERINACTIVE {
    type Abi = Self;
}
impl ::core::fmt::Debug for POINTERINACTIVE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("POINTERINACTIVE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PRINTFLAG(pub u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PRINTFLAG_MAYBOTHERUSER: PRINTFLAG = PRINTFLAG(1u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PRINTFLAG_PROMPTUSER: PRINTFLAG = PRINTFLAG(2u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PRINTFLAG_USERMAYCHANGEPRINTER: PRINTFLAG = PRINTFLAG(4u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PRINTFLAG_RECOMPOSETODEVICE: PRINTFLAG = PRINTFLAG(8u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PRINTFLAG_DONTACTUALLYPRINT: PRINTFLAG = PRINTFLAG(16u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PRINTFLAG_FORCEPROPERTIES: PRINTFLAG = PRINTFLAG(32u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PRINTFLAG_PRINTTOFILE: PRINTFLAG = PRINTFLAG(64u32);
impl ::core::marker::Copy for PRINTFLAG {}
impl ::core::clone::Clone for PRINTFLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PRINTFLAG {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PRINTFLAG {
    type Abi = Self;
}
impl ::core::fmt::Debug for PRINTFLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PRINTFLAG").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PRINTFLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PRINTFLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PRINTFLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PRINTFLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PRINTFLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROPBAG2_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PROPBAG2_TYPE_UNDEFINED: PROPBAG2_TYPE = PROPBAG2_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PROPBAG2_TYPE_DATA: PROPBAG2_TYPE = PROPBAG2_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PROPBAG2_TYPE_URL: PROPBAG2_TYPE = PROPBAG2_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PROPBAG2_TYPE_OBJECT: PROPBAG2_TYPE = PROPBAG2_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PROPBAG2_TYPE_STREAM: PROPBAG2_TYPE = PROPBAG2_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PROPBAG2_TYPE_STORAGE: PROPBAG2_TYPE = PROPBAG2_TYPE(5i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PROPBAG2_TYPE_MONIKER: PROPBAG2_TYPE = PROPBAG2_TYPE(6i32);
impl ::core::marker::Copy for PROPBAG2_TYPE {}
impl ::core::clone::Clone for PROPBAG2_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPBAG2_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PROPBAG2_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for PROPBAG2_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPBAG2_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROPPAGESTATUS(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PROPPAGESTATUS_DIRTY: PROPPAGESTATUS = PROPPAGESTATUS(1i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PROPPAGESTATUS_VALIDATE: PROPPAGESTATUS = PROPPAGESTATUS(2i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const PROPPAGESTATUS_CLEAN: PROPPAGESTATUS = PROPPAGESTATUS(4i32);
impl ::core::marker::Copy for PROPPAGESTATUS {}
impl ::core::clone::Clone for PROPPAGESTATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPPAGESTATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PROPPAGESTATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for PROPPAGESTATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPPAGESTATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct QACONTAINERFLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const QACONTAINER_SHOWHATCHING: QACONTAINERFLAGS = QACONTAINERFLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const QACONTAINER_SHOWGRABHANDLES: QACONTAINERFLAGS = QACONTAINERFLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const QACONTAINER_USERMODE: QACONTAINERFLAGS = QACONTAINERFLAGS(4i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const QACONTAINER_DISPLAYASDEFAULT: QACONTAINERFLAGS = QACONTAINERFLAGS(8i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const QACONTAINER_UIDEAD: QACONTAINERFLAGS = QACONTAINERFLAGS(16i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const QACONTAINER_AUTOCLIP: QACONTAINERFLAGS = QACONTAINERFLAGS(32i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const QACONTAINER_MESSAGEREFLECT: QACONTAINERFLAGS = QACONTAINERFLAGS(64i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const QACONTAINER_SUPPORTSMNEMONICS: QACONTAINERFLAGS = QACONTAINERFLAGS(128i32);
impl ::core::marker::Copy for QACONTAINERFLAGS {}
impl ::core::clone::Clone for QACONTAINERFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for QACONTAINERFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for QACONTAINERFLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for QACONTAINERFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QACONTAINERFLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct READYSTATE(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const READYSTATE_UNINITIALIZED: READYSTATE = READYSTATE(0i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const READYSTATE_LOADING: READYSTATE = READYSTATE(1i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const READYSTATE_LOADED: READYSTATE = READYSTATE(2i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const READYSTATE_INTERACTIVE: READYSTATE = READYSTATE(3i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const READYSTATE_COMPLETE: READYSTATE = READYSTATE(4i32);
impl ::core::marker::Copy for READYSTATE {}
impl ::core::clone::Clone for READYSTATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for READYSTATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for READYSTATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for READYSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("READYSTATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct REGKIND(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const REGKIND_DEFAULT: REGKIND = REGKIND(0i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const REGKIND_REGISTER: REGKIND = REGKIND(1i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const REGKIND_NONE: REGKIND = REGKIND(2i32);
impl ::core::marker::Copy for REGKIND {}
impl ::core::clone::Clone for REGKIND {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REGKIND {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for REGKIND {
    type Abi = Self;
}
impl ::core::fmt::Debug for REGKIND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REGKIND").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SF_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const SF_ERROR: SF_TYPE = SF_TYPE(10i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const SF_I1: SF_TYPE = SF_TYPE(16i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const SF_I2: SF_TYPE = SF_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const SF_I4: SF_TYPE = SF_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const SF_I8: SF_TYPE = SF_TYPE(20i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const SF_BSTR: SF_TYPE = SF_TYPE(8i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const SF_UNKNOWN: SF_TYPE = SF_TYPE(13i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const SF_DISPATCH: SF_TYPE = SF_TYPE(9i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const SF_VARIANT: SF_TYPE = SF_TYPE(12i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const SF_RECORD: SF_TYPE = SF_TYPE(36i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const SF_HAVEIID: SF_TYPE = SF_TYPE(32781i32);
impl ::core::marker::Copy for SF_TYPE {}
impl ::core::clone::Clone for SF_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SF_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SF_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for SF_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SF_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TYPEFLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const TYPEFLAG_FAPPOBJECT: TYPEFLAGS = TYPEFLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const TYPEFLAG_FCANCREATE: TYPEFLAGS = TYPEFLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const TYPEFLAG_FLICENSED: TYPEFLAGS = TYPEFLAGS(4i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const TYPEFLAG_FPREDECLID: TYPEFLAGS = TYPEFLAGS(8i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const TYPEFLAG_FHIDDEN: TYPEFLAGS = TYPEFLAGS(16i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const TYPEFLAG_FCONTROL: TYPEFLAGS = TYPEFLAGS(32i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const TYPEFLAG_FDUAL: TYPEFLAGS = TYPEFLAGS(64i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const TYPEFLAG_FNONEXTENSIBLE: TYPEFLAGS = TYPEFLAGS(128i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const TYPEFLAG_FOLEAUTOMATION: TYPEFLAGS = TYPEFLAGS(256i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const TYPEFLAG_FRESTRICTED: TYPEFLAGS = TYPEFLAGS(512i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const TYPEFLAG_FAGGREGATABLE: TYPEFLAGS = TYPEFLAGS(1024i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const TYPEFLAG_FREPLACEABLE: TYPEFLAGS = TYPEFLAGS(2048i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const TYPEFLAG_FDISPATCHABLE: TYPEFLAGS = TYPEFLAGS(4096i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const TYPEFLAG_FREVERSEBIND: TYPEFLAGS = TYPEFLAGS(8192i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const TYPEFLAG_FPROXY: TYPEFLAGS = TYPEFLAGS(16384i32);
impl ::core::marker::Copy for TYPEFLAGS {}
impl ::core::clone::Clone for TYPEFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TYPEFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TYPEFLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for TYPEFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TYPEFLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UASFLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const UAS_NORMAL: UASFLAGS = UASFLAGS(0i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const UAS_BLOCKED: UASFLAGS = UASFLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const UAS_NOPARENTENABLE: UASFLAGS = UASFLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const UAS_MASK: UASFLAGS = UASFLAGS(3i32);
impl ::core::marker::Copy for UASFLAGS {}
impl ::core::clone::Clone for UASFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UASFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UASFLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for UASFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UASFLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UI_CONVERT_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CF_SHOWHELPBUTTON: UI_CONVERT_FLAGS = UI_CONVERT_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CF_SETCONVERTDEFAULT: UI_CONVERT_FLAGS = UI_CONVERT_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CF_SETACTIVATEDEFAULT: UI_CONVERT_FLAGS = UI_CONVERT_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CF_SELECTCONVERTTO: UI_CONVERT_FLAGS = UI_CONVERT_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CF_SELECTACTIVATEAS: UI_CONVERT_FLAGS = UI_CONVERT_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CF_DISABLEDISPLAYASICON: UI_CONVERT_FLAGS = UI_CONVERT_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CF_DISABLEACTIVATEAS: UI_CONVERT_FLAGS = UI_CONVERT_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CF_HIDECHANGEICON: UI_CONVERT_FLAGS = UI_CONVERT_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const CF_CONVERTONLY: UI_CONVERT_FLAGS = UI_CONVERT_FLAGS(256u32);
impl ::core::marker::Copy for UI_CONVERT_FLAGS {}
impl ::core::clone::Clone for UI_CONVERT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UI_CONVERT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UI_CONVERT_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for UI_CONVERT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_CONVERT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for UI_CONVERT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for UI_CONVERT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for UI_CONVERT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for UI_CONVERT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for UI_CONVERT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UPDFCACHE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const UPDFCACHE_ALL: UPDFCACHE_FLAGS = UPDFCACHE_FLAGS(2147483647u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const UPDFCACHE_ALLBUTNODATACACHE: UPDFCACHE_FLAGS = UPDFCACHE_FLAGS(2147483646u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const UPDFCACHE_NORMALCACHE: UPDFCACHE_FLAGS = UPDFCACHE_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const UPDFCACHE_IFBLANK: UPDFCACHE_FLAGS = UPDFCACHE_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const UPDFCACHE_ONLYIFBLANK: UPDFCACHE_FLAGS = UPDFCACHE_FLAGS(2147483648u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const UPDFCACHE_NODATACACHE: UPDFCACHE_FLAGS = UPDFCACHE_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const UPDFCACHE_ONSAVECACHE: UPDFCACHE_FLAGS = UPDFCACHE_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const UPDFCACHE_ONSTOPCACHE: UPDFCACHE_FLAGS = UPDFCACHE_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const UPDFCACHE_IFBLANKORONSAVECACHE: UPDFCACHE_FLAGS = UPDFCACHE_FLAGS(18u32);
impl ::core::marker::Copy for UPDFCACHE_FLAGS {}
impl ::core::clone::Clone for UPDFCACHE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UPDFCACHE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UPDFCACHE_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for UPDFCACHE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UPDFCACHE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for UPDFCACHE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for UPDFCACHE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for UPDFCACHE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for UPDFCACHE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for UPDFCACHE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct USERCLASSTYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const USERCLASSTYPE_FULL: USERCLASSTYPE = USERCLASSTYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const USERCLASSTYPE_SHORT: USERCLASSTYPE = USERCLASSTYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const USERCLASSTYPE_APPNAME: USERCLASSTYPE = USERCLASSTYPE(3i32);
impl ::core::marker::Copy for USERCLASSTYPE {}
impl ::core::clone::Clone for USERCLASSTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for USERCLASSTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for USERCLASSTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for USERCLASSTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("USERCLASSTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VARCMP(pub u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARCMP_LT: VARCMP = VARCMP(0u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARCMP_EQ: VARCMP = VARCMP(1u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARCMP_GT: VARCMP = VARCMP(2u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARCMP_NULL: VARCMP = VARCMP(3u32);
impl ::core::marker::Copy for VARCMP {}
impl ::core::clone::Clone for VARCMP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VARCMP {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VARCMP {
    type Abi = Self;
}
impl ::core::fmt::Debug for VARCMP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VARCMP").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VARFORMAT_FIRST_DAY(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARFORMAT_FIRST_DAY_SYSTEMDEFAULT: VARFORMAT_FIRST_DAY = VARFORMAT_FIRST_DAY(0i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARFORMAT_FIRST_DAY_MONDAY: VARFORMAT_FIRST_DAY = VARFORMAT_FIRST_DAY(1i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARFORMAT_FIRST_DAY_TUESDAY: VARFORMAT_FIRST_DAY = VARFORMAT_FIRST_DAY(2i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARFORMAT_FIRST_DAY_WEDNESDAY: VARFORMAT_FIRST_DAY = VARFORMAT_FIRST_DAY(3i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARFORMAT_FIRST_DAY_THURSDAY: VARFORMAT_FIRST_DAY = VARFORMAT_FIRST_DAY(4i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARFORMAT_FIRST_DAY_FRIDAY: VARFORMAT_FIRST_DAY = VARFORMAT_FIRST_DAY(5i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARFORMAT_FIRST_DAY_SATURDAY: VARFORMAT_FIRST_DAY = VARFORMAT_FIRST_DAY(6i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARFORMAT_FIRST_DAY_SUNDAY: VARFORMAT_FIRST_DAY = VARFORMAT_FIRST_DAY(7i32);
impl ::core::marker::Copy for VARFORMAT_FIRST_DAY {}
impl ::core::clone::Clone for VARFORMAT_FIRST_DAY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VARFORMAT_FIRST_DAY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VARFORMAT_FIRST_DAY {
    type Abi = Self;
}
impl ::core::fmt::Debug for VARFORMAT_FIRST_DAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VARFORMAT_FIRST_DAY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VARFORMAT_FIRST_WEEK(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARFORMAT_FIRST_WEEK_SYSTEMDEFAULT: VARFORMAT_FIRST_WEEK = VARFORMAT_FIRST_WEEK(0i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARFORMAT_FIRST_WEEK_CONTAINS_JANUARY_FIRST: VARFORMAT_FIRST_WEEK = VARFORMAT_FIRST_WEEK(1i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARFORMAT_FIRST_WEEK_LARGER_HALF_IN_CURRENT_YEAR: VARFORMAT_FIRST_WEEK = VARFORMAT_FIRST_WEEK(2i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARFORMAT_FIRST_WEEK_HAS_SEVEN_DAYS: VARFORMAT_FIRST_WEEK = VARFORMAT_FIRST_WEEK(3i32);
impl ::core::marker::Copy for VARFORMAT_FIRST_WEEK {}
impl ::core::clone::Clone for VARFORMAT_FIRST_WEEK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VARFORMAT_FIRST_WEEK {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VARFORMAT_FIRST_WEEK {
    type Abi = Self;
}
impl ::core::fmt::Debug for VARFORMAT_FIRST_WEEK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VARFORMAT_FIRST_WEEK").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VARFORMAT_GROUP(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARFORMAT_GROUP_SYSTEMDEFAULT: VARFORMAT_GROUP = VARFORMAT_GROUP(-2i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARFORMAT_GROUP_THOUSANDS: VARFORMAT_GROUP = VARFORMAT_GROUP(-1i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARFORMAT_GROUP_NOTTHOUSANDS: VARFORMAT_GROUP = VARFORMAT_GROUP(0i32);
impl ::core::marker::Copy for VARFORMAT_GROUP {}
impl ::core::clone::Clone for VARFORMAT_GROUP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VARFORMAT_GROUP {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VARFORMAT_GROUP {
    type Abi = Self;
}
impl ::core::fmt::Debug for VARFORMAT_GROUP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VARFORMAT_GROUP").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VARFORMAT_LEADING_DIGIT(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARFORMAT_LEADING_DIGIT_SYSTEMDEFAULT: VARFORMAT_LEADING_DIGIT = VARFORMAT_LEADING_DIGIT(-2i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARFORMAT_LEADING_DIGIT_INCLUDED: VARFORMAT_LEADING_DIGIT = VARFORMAT_LEADING_DIGIT(-1i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARFORMAT_LEADING_DIGIT_NOTINCLUDED: VARFORMAT_LEADING_DIGIT = VARFORMAT_LEADING_DIGIT(0i32);
impl ::core::marker::Copy for VARFORMAT_LEADING_DIGIT {}
impl ::core::clone::Clone for VARFORMAT_LEADING_DIGIT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VARFORMAT_LEADING_DIGIT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VARFORMAT_LEADING_DIGIT {
    type Abi = Self;
}
impl ::core::fmt::Debug for VARFORMAT_LEADING_DIGIT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VARFORMAT_LEADING_DIGIT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VARFORMAT_NAMED_FORMAT(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARFORMAT_NAMED_FORMAT_GENERALDATE: VARFORMAT_NAMED_FORMAT = VARFORMAT_NAMED_FORMAT(0i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARFORMAT_NAMED_FORMAT_LONGDATE: VARFORMAT_NAMED_FORMAT = VARFORMAT_NAMED_FORMAT(1i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARFORMAT_NAMED_FORMAT_SHORTDATE: VARFORMAT_NAMED_FORMAT = VARFORMAT_NAMED_FORMAT(2i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARFORMAT_NAMED_FORMAT_LONGTIME: VARFORMAT_NAMED_FORMAT = VARFORMAT_NAMED_FORMAT(3i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARFORMAT_NAMED_FORMAT_SHORTTIME: VARFORMAT_NAMED_FORMAT = VARFORMAT_NAMED_FORMAT(4i32);
impl ::core::marker::Copy for VARFORMAT_NAMED_FORMAT {}
impl ::core::clone::Clone for VARFORMAT_NAMED_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VARFORMAT_NAMED_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VARFORMAT_NAMED_FORMAT {
    type Abi = Self;
}
impl ::core::fmt::Debug for VARFORMAT_NAMED_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VARFORMAT_NAMED_FORMAT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VARFORMAT_PARENTHESES(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARFORMAT_PARENTHESES_SYSTEMDEFAULT: VARFORMAT_PARENTHESES = VARFORMAT_PARENTHESES(-2i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARFORMAT_PARENTHESES_USED: VARFORMAT_PARENTHESES = VARFORMAT_PARENTHESES(-1i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VARFORMAT_PARENTHESES_NOTUSED: VARFORMAT_PARENTHESES = VARFORMAT_PARENTHESES(0i32);
impl ::core::marker::Copy for VARFORMAT_PARENTHESES {}
impl ::core::clone::Clone for VARFORMAT_PARENTHESES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VARFORMAT_PARENTHESES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VARFORMAT_PARENTHESES {
    type Abi = Self;
}
impl ::core::fmt::Debug for VARFORMAT_PARENTHESES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VARFORMAT_PARENTHESES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VIEWSTATUS(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VIEWSTATUS_OPAQUE: VIEWSTATUS = VIEWSTATUS(1i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VIEWSTATUS_SOLIDBKGND: VIEWSTATUS = VIEWSTATUS(2i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VIEWSTATUS_DVASPECTOPAQUE: VIEWSTATUS = VIEWSTATUS(4i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VIEWSTATUS_DVASPECTTRANSPARENT: VIEWSTATUS = VIEWSTATUS(8i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VIEWSTATUS_SURFACE: VIEWSTATUS = VIEWSTATUS(16i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VIEWSTATUS_3DSURFACE: VIEWSTATUS = VIEWSTATUS(32i32);
impl ::core::marker::Copy for VIEWSTATUS {}
impl ::core::clone::Clone for VIEWSTATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VIEWSTATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VIEWSTATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for VIEWSTATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VIEWSTATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VIEW_OBJECT_PROPERTIES_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VPF_SELECTRELATIVE: VIEW_OBJECT_PROPERTIES_FLAGS = VIEW_OBJECT_PROPERTIES_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VPF_DISABLERELATIVE: VIEW_OBJECT_PROPERTIES_FLAGS = VIEW_OBJECT_PROPERTIES_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const VPF_DISABLESCALE: VIEW_OBJECT_PROPERTIES_FLAGS = VIEW_OBJECT_PROPERTIES_FLAGS(4u32);
impl ::core::marker::Copy for VIEW_OBJECT_PROPERTIES_FLAGS {}
impl ::core::clone::Clone for VIEW_OBJECT_PROPERTIES_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VIEW_OBJECT_PROPERTIES_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VIEW_OBJECT_PROPERTIES_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for VIEW_OBJECT_PROPERTIES_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VIEW_OBJECT_PROPERTIES_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for VIEW_OBJECT_PROPERTIES_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for VIEW_OBJECT_PROPERTIES_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for VIEW_OBJECT_PROPERTIES_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for VIEW_OBJECT_PROPERTIES_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for VIEW_OBJECT_PROPERTIES_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WPCSETTING(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const WPCSETTING_LOGGING_ENABLED: WPCSETTING = WPCSETTING(1i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const WPCSETTING_FILEDOWNLOAD_BLOCKED: WPCSETTING = WPCSETTING(2i32);
impl ::core::marker::Copy for WPCSETTING {}
impl ::core::clone::Clone for WPCSETTING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WPCSETTING {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WPCSETTING {
    type Abi = Self;
}
impl ::core::fmt::Debug for WPCSETTING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPCSETTING").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XFORMCOORDS(pub i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const XFORMCOORDS_POSITION: XFORMCOORDS = XFORMCOORDS(1i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const XFORMCOORDS_SIZE: XFORMCOORDS = XFORMCOORDS(2i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const XFORMCOORDS_HIMETRICTOCONTAINER: XFORMCOORDS = XFORMCOORDS(4i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const XFORMCOORDS_CONTAINERTOHIMETRIC: XFORMCOORDS = XFORMCOORDS(8i32);
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub const XFORMCOORDS_EVENTCOMPAT: XFORMCOORDS = XFORMCOORDS(16i32);
impl ::core::marker::Copy for XFORMCOORDS {}
impl ::core::clone::Clone for XFORMCOORDS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XFORMCOORDS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XFORMCOORDS {
    type Abi = Self;
}
impl ::core::fmt::Debug for XFORMCOORDS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XFORMCOORDS").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct ARRAYDESC {
    pub tdescElem: super::Com::TYPEDESC,
    pub cDims: u16,
    pub rgbounds: [super::Com::SAFEARRAYBOUND; 1],
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for ARRAYDESC {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ARRAYDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for ARRAYDESC {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for ARRAYDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub struct CADWORD {
    pub cElems: u32,
    pub pElems: *mut u32,
}
impl ::core::marker::Copy for CADWORD {}
impl ::core::clone::Clone for CADWORD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CADWORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CADWORD").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
unsafe impl ::windows::core::Abi for CADWORD {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CADWORD {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
impl ::core::cmp::Eq for CADWORD {}
impl ::core::default::Default for CADWORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub struct CALPOLESTR {
    pub cElems: u32,
    pub pElems: *mut ::windows::core::PWSTR,
}
impl ::core::marker::Copy for CALPOLESTR {}
impl ::core::clone::Clone for CALPOLESTR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CALPOLESTR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CALPOLESTR").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
unsafe impl ::windows::core::Abi for CALPOLESTR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CALPOLESTR {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
impl ::core::cmp::Eq for CALPOLESTR {}
impl ::core::default::Default for CALPOLESTR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub struct CAUUID {
    pub cElems: u32,
    pub pElems: *mut ::windows::core::GUID,
}
impl ::core::marker::Copy for CAUUID {}
impl ::core::clone::Clone for CAUUID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CAUUID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CAUUID").field("cElems", &self.cElems).field("pElems", &self.pElems).finish()
    }
}
unsafe impl ::windows::core::Abi for CAUUID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CAUUID {
    fn eq(&self, other: &Self) -> bool {
        self.cElems == other.cElems && self.pElems == other.pElems
    }
}
impl ::core::cmp::Eq for CAUUID {}
impl ::core::default::Default for CAUUID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub struct CLEANLOCALSTORAGE {
    pub pInterface: ::windows::core::ManuallyDrop<::windows::core::IUnknown>,
    pub pStorage: *mut ::core::ffi::c_void,
    pub flags: u32,
}
impl ::core::clone::Clone for CLEANLOCALSTORAGE {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::core::fmt::Debug for CLEANLOCALSTORAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLEANLOCALSTORAGE").field("pInterface", &self.pInterface).field("pStorage", &self.pStorage).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows::core::Abi for CLEANLOCALSTORAGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CLEANLOCALSTORAGE {
    fn eq(&self, other: &Self) -> bool {
        self.pInterface == other.pInterface && self.pStorage == other.pStorage && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for CLEANLOCALSTORAGE {}
impl ::core::default::Default for CLEANLOCALSTORAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub struct CONTROLINFO {
    pub cb: u32,
    pub hAccel: super::super::UI::WindowsAndMessaging::HACCEL,
    pub cAccel: u16,
    pub dwFlags: CTRLINFO,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::marker::Copy for CONTROLINFO {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::clone::Clone for CONTROLINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::fmt::Debug for CONTROLINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONTROLINFO").field("cb", &self.cb).field("hAccel", &self.hAccel).field("cAccel", &self.cAccel).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
unsafe impl ::windows::core::Abi for CONTROLINFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::cmp::PartialEq for CONTROLINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.hAccel == other.hAccel && self.cAccel == other.cAccel && self.dwFlags == other.dwFlags
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::cmp::Eq for CONTROLINFO {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::default::Default for CONTROLINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub struct DVASPECTINFO {
    pub cb: u32,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for DVASPECTINFO {}
impl ::core::clone::Clone for DVASPECTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DVASPECTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DVASPECTINFO").field("cb", &self.cb).field("dwFlags", &self.dwFlags).finish()
    }
}
unsafe impl ::windows::core::Abi for DVASPECTINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DVASPECTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for DVASPECTINFO {}
impl ::core::default::Default for DVASPECTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DVEXTENTINFO {
    pub cb: u32,
    pub dwExtentMode: u32,
    pub sizelProposed: super::super::Foundation::SIZE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DVEXTENTINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DVEXTENTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DVEXTENTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DVEXTENTINFO").field("cb", &self.cb).field("dwExtentMode", &self.dwExtentMode).field("sizelProposed", &self.sizelProposed).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DVEXTENTINFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DVEXTENTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.dwExtentMode == other.dwExtentMode && self.sizelProposed == other.sizelProposed
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DVEXTENTINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DVEXTENTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct FONTDESC {
    pub cbSizeofstruct: u32,
    pub lpstrName: ::windows::core::PWSTR,
    pub cySize: super::Com::CY,
    pub sWeight: i16,
    pub sCharset: i16,
    pub fItalic: super::super::Foundation::BOOL,
    pub fUnderline: super::super::Foundation::BOOL,
    pub fStrikethrough: super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for FONTDESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for FONTDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for FONTDESC {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for FONTDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct INTERFACEDATA {
    pub pmethdata: *mut METHODDATA,
    pub cMembers: u32,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for INTERFACEDATA {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for INTERFACEDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for INTERFACEDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERFACEDATA").field("pmethdata", &self.pmethdata).field("cMembers", &self.cMembers).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for INTERFACEDATA {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for INTERFACEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.pmethdata == other.pmethdata && self.cMembers == other.cMembers
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for INTERFACEDATA {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for INTERFACEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LICINFO {
    pub cbLicInfo: i32,
    pub fRuntimeKeyAvail: super::super::Foundation::BOOL,
    pub fLicVerified: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LICINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LICINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LICINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LICINFO").field("cbLicInfo", &self.cbLicInfo).field("fRuntimeKeyAvail", &self.fRuntimeKeyAvail).field("fLicVerified", &self.fLicVerified).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for LICINFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LICINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbLicInfo == other.cbLicInfo && self.fRuntimeKeyAvail == other.fRuntimeKeyAvail && self.fLicVerified == other.fLicVerified
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LICINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LICINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct METHODDATA {
    pub szName: ::windows::core::PWSTR,
    pub ppdata: *mut PARAMDATA,
    pub dispid: i32,
    pub iMeth: u32,
    pub cc: super::Com::CALLCONV,
    pub cArgs: u32,
    pub wFlags: u16,
    pub vtReturn: super::Com::VARENUM,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for METHODDATA {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for METHODDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for METHODDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("METHODDATA").field("szName", &self.szName).field("ppdata", &self.ppdata).field("dispid", &self.dispid).field("iMeth", &self.iMeth).field("cc", &self.cc).field("cArgs", &self.cArgs).field("wFlags", &self.wFlags).field("vtReturn", &self.vtReturn).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for METHODDATA {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for METHODDATA {
    fn eq(&self, other: &Self) -> bool {
        self.szName == other.szName && self.ppdata == other.ppdata && self.dispid == other.dispid && self.iMeth == other.iMeth && self.cc == other.cc && self.cArgs == other.cArgs && self.wFlags == other.wFlags && self.vtReturn == other.vtReturn
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for METHODDATA {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for METHODDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub struct NUMPARSE {
    pub cDig: i32,
    pub dwInFlags: NUMPARSE_FLAGS,
    pub dwOutFlags: NUMPARSE_FLAGS,
    pub cchUsed: i32,
    pub nBaseShift: i32,
    pub nPwr10: i32,
}
impl ::core::marker::Copy for NUMPARSE {}
impl ::core::clone::Clone for NUMPARSE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NUMPARSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NUMPARSE").field("cDig", &self.cDig).field("dwInFlags", &self.dwInFlags).field("dwOutFlags", &self.dwOutFlags).field("cchUsed", &self.cchUsed).field("nBaseShift", &self.nBaseShift).field("nPwr10", &self.nPwr10).finish()
    }
}
unsafe impl ::windows::core::Abi for NUMPARSE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NUMPARSE {
    fn eq(&self, other: &Self) -> bool {
        self.cDig == other.cDig && self.dwInFlags == other.dwInFlags && self.dwOutFlags == other.dwOutFlags && self.cchUsed == other.cchUsed && self.nBaseShift == other.nBaseShift && self.nPwr10 == other.nPwr10
    }
}
impl ::core::cmp::Eq for NUMPARSE {}
impl ::core::default::Default for NUMPARSE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OBJECTDESCRIPTOR {
    pub cbSize: u32,
    pub clsid: ::windows::core::GUID,
    pub dwDrawAspect: u32,
    pub sizel: super::super::Foundation::SIZE,
    pub pointl: super::super::Foundation::POINTL,
    pub dwStatus: u32,
    pub dwFullUserTypeName: u32,
    pub dwSrcOfCopy: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OBJECTDESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OBJECTDESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for OBJECTDESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OBJECTDESCRIPTOR").field("cbSize", &self.cbSize).field("clsid", &self.clsid).field("dwDrawAspect", &self.dwDrawAspect).field("sizel", &self.sizel).field("pointl", &self.pointl).field("dwStatus", &self.dwStatus).field("dwFullUserTypeName", &self.dwFullUserTypeName).field("dwSrcOfCopy", &self.dwSrcOfCopy).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for OBJECTDESCRIPTOR {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OBJECTDESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.clsid == other.clsid && self.dwDrawAspect == other.dwDrawAspect && self.sizel == other.sizel && self.pointl == other.pointl && self.dwStatus == other.dwStatus && self.dwFullUserTypeName == other.dwFullUserTypeName && self.dwSrcOfCopy == other.dwSrcOfCopy
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OBJECTDESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OBJECTDESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OCPFIPARAMS {
    pub cbStructSize: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub x: i32,
    pub y: i32,
    pub lpszCaption: ::windows::core::PCWSTR,
    pub cObjects: u32,
    pub lplpUnk: *mut ::core::option::Option<::windows::core::IUnknown>,
    pub cPages: u32,
    pub lpPages: *mut ::windows::core::GUID,
    pub lcid: u32,
    pub dispidInitialProperty: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OCPFIPARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OCPFIPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for OCPFIPARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OCPFIPARAMS").field("cbStructSize", &self.cbStructSize).field("hWndOwner", &self.hWndOwner).field("x", &self.x).field("y", &self.y).field("lpszCaption", &self.lpszCaption).field("cObjects", &self.cObjects).field("lplpUnk", &self.lplpUnk).field("cPages", &self.cPages).field("lpPages", &self.lpPages).field("lcid", &self.lcid).field("dispidInitialProperty", &self.dispidInitialProperty).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for OCPFIPARAMS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OCPFIPARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.cbStructSize == other.cbStructSize && self.hWndOwner == other.hWndOwner && self.x == other.x && self.y == other.y && self.lpszCaption == other.lpszCaption && self.cObjects == other.cObjects && self.lplpUnk == other.lplpUnk && self.cPages == other.cPages && self.lpPages == other.lpPages && self.lcid == other.lcid && self.dispidInitialProperty == other.dispidInitialProperty
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OCPFIPARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OCPFIPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub struct OLECMD {
    pub cmdID: OLECMDID,
    pub cmdf: OLECMDF,
}
impl ::core::marker::Copy for OLECMD {}
impl ::core::clone::Clone for OLECMD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for OLECMD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OLECMD").field("cmdID", &self.cmdID).field("cmdf", &self.cmdf).finish()
    }
}
unsafe impl ::windows::core::Abi for OLECMD {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for OLECMD {
    fn eq(&self, other: &Self) -> bool {
        self.cmdID == other.cmdID && self.cmdf == other.cmdf
    }
}
impl ::core::cmp::Eq for OLECMD {}
impl ::core::default::Default for OLECMD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub struct OLECMDTEXT {
    pub cmdtextf: u32,
    pub cwActual: u32,
    pub cwBuf: u32,
    pub rgwz: [u16; 1],
}
impl ::core::marker::Copy for OLECMDTEXT {}
impl ::core::clone::Clone for OLECMDTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for OLECMDTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OLECMDTEXT").field("cmdtextf", &self.cmdtextf).field("cwActual", &self.cwActual).field("cwBuf", &self.cwBuf).field("rgwz", &self.rgwz).finish()
    }
}
unsafe impl ::windows::core::Abi for OLECMDTEXT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for OLECMDTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.cmdtextf == other.cmdtextf && self.cwActual == other.cwActual && self.cwBuf == other.cwBuf && self.rgwz == other.rgwz
    }
}
impl ::core::cmp::Eq for OLECMDTEXT {}
impl ::core::default::Default for OLECMDTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct OLEINPLACEFRAMEINFO {
    pub cb: u32,
    pub fMDIApp: super::super::Foundation::BOOL,
    pub hwndFrame: super::super::Foundation::HWND,
    pub haccel: super::super::UI::WindowsAndMessaging::HACCEL,
    pub cAccelEntries: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for OLEINPLACEFRAMEINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for OLEINPLACEFRAMEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::fmt::Debug for OLEINPLACEFRAMEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OLEINPLACEFRAMEINFO").field("cb", &self.cb).field("fMDIApp", &self.fMDIApp).field("hwndFrame", &self.hwndFrame).field("haccel", &self.haccel).field("cAccelEntries", &self.cAccelEntries).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for OLEINPLACEFRAMEINFO {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for OLEINPLACEFRAMEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.fMDIApp == other.fMDIApp && self.hwndFrame == other.hwndFrame && self.haccel == other.haccel && self.cAccelEntries == other.cAccelEntries
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for OLEINPLACEFRAMEINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for OLEINPLACEFRAMEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub struct OLEMENUGROUPWIDTHS {
    pub width: [i32; 6],
}
impl ::core::marker::Copy for OLEMENUGROUPWIDTHS {}
impl ::core::clone::Clone for OLEMENUGROUPWIDTHS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for OLEMENUGROUPWIDTHS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OLEMENUGROUPWIDTHS").field("width", &self.width).finish()
    }
}
unsafe impl ::windows::core::Abi for OLEMENUGROUPWIDTHS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for OLEMENUGROUPWIDTHS {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width
    }
}
impl ::core::cmp::Eq for OLEMENUGROUPWIDTHS {}
impl ::core::default::Default for OLEMENUGROUPWIDTHS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_Media\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media"))]
pub struct OLEUIBUSYA {
    pub cbStruct: u32,
    pub dwFlags: BUSY_DIALOG_FLAGS,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: ::windows::core::PCSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: ::windows::core::PCSTR,
    pub hResource: super::super::Foundation::HRSRC,
    pub hTask: super::super::Media::HTASK,
    pub lphWndDialog: *mut super::super::Foundation::HWND,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media"))]
impl ::core::marker::Copy for OLEUIBUSYA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media"))]
impl ::core::clone::Clone for OLEUIBUSYA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media"))]
impl ::core::fmt::Debug for OLEUIBUSYA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OLEUIBUSYA").field("cbStruct", &self.cbStruct).field("dwFlags", &self.dwFlags).field("hWndOwner", &self.hWndOwner).field("lpszCaption", &self.lpszCaption).field("lCustData", &self.lCustData).field("hInstance", &self.hInstance).field("lpszTemplate", &self.lpszTemplate).field("hResource", &self.hResource).field("hTask", &self.hTask).field("lphWndDialog", &self.lphWndDialog).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media"))]
unsafe impl ::windows::core::Abi for OLEUIBUSYA {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media"))]
impl ::core::default::Default for OLEUIBUSYA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_Media\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media"))]
pub struct OLEUIBUSYW {
    pub cbStruct: u32,
    pub dwFlags: BUSY_DIALOG_FLAGS,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: ::windows::core::PCWSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: ::windows::core::PCWSTR,
    pub hResource: super::super::Foundation::HRSRC,
    pub hTask: super::super::Media::HTASK,
    pub lphWndDialog: *mut super::super::Foundation::HWND,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media"))]
impl ::core::marker::Copy for OLEUIBUSYW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media"))]
impl ::core::clone::Clone for OLEUIBUSYW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media"))]
impl ::core::fmt::Debug for OLEUIBUSYW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OLEUIBUSYW").field("cbStruct", &self.cbStruct).field("dwFlags", &self.dwFlags).field("hWndOwner", &self.hWndOwner).field("lpszCaption", &self.lpszCaption).field("lCustData", &self.lCustData).field("hInstance", &self.hInstance).field("lpszTemplate", &self.lpszTemplate).field("hResource", &self.hResource).field("hTask", &self.hTask).field("lphWndDialog", &self.lphWndDialog).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media"))]
unsafe impl ::windows::core::Abi for OLEUIBUSYW {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media"))]
impl ::core::default::Default for OLEUIBUSYW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OLEUICHANGEICONA {
    pub cbStruct: u32,
    pub dwFlags: CHANGE_ICON_FLAGS,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: ::windows::core::PCSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: ::windows::core::PCSTR,
    pub hResource: super::super::Foundation::HRSRC,
    pub hMetaPict: isize,
    pub clsid: ::windows::core::GUID,
    pub szIconExe: [super::super::Foundation::CHAR; 260],
    pub cchIconExe: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OLEUICHANGEICONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OLEUICHANGEICONA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for OLEUICHANGEICONA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OLEUICHANGEICONA").field("cbStruct", &self.cbStruct).field("dwFlags", &self.dwFlags).field("hWndOwner", &self.hWndOwner).field("lpszCaption", &self.lpszCaption).field("lCustData", &self.lCustData).field("hInstance", &self.hInstance).field("lpszTemplate", &self.lpszTemplate).field("hResource", &self.hResource).field("hMetaPict", &self.hMetaPict).field("clsid", &self.clsid).field("szIconExe", &self.szIconExe).field("cchIconExe", &self.cchIconExe).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for OLEUICHANGEICONA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OLEUICHANGEICONA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OLEUICHANGEICONW {
    pub cbStruct: u32,
    pub dwFlags: CHANGE_ICON_FLAGS,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: ::windows::core::PCWSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: ::windows::core::PCWSTR,
    pub hResource: super::super::Foundation::HRSRC,
    pub hMetaPict: isize,
    pub clsid: ::windows::core::GUID,
    pub szIconExe: [u16; 260],
    pub cchIconExe: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OLEUICHANGEICONW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OLEUICHANGEICONW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for OLEUICHANGEICONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OLEUICHANGEICONW").field("cbStruct", &self.cbStruct).field("dwFlags", &self.dwFlags).field("hWndOwner", &self.hWndOwner).field("lpszCaption", &self.lpszCaption).field("lCustData", &self.lCustData).field("hInstance", &self.hInstance).field("lpszTemplate", &self.lpszTemplate).field("hResource", &self.hResource).field("hMetaPict", &self.hMetaPict).field("clsid", &self.clsid).field("szIconExe", &self.szIconExe).field("cchIconExe", &self.cchIconExe).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for OLEUICHANGEICONW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OLEUICHANGEICONW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Controls_Dialogs\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls_Dialogs"))]
pub struct OLEUICHANGESOURCEA {
    pub cbStruct: u32,
    pub dwFlags: CHANGE_SOURCE_FLAGS,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: ::windows::core::PCSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: ::windows::core::PCSTR,
    pub hResource: super::super::Foundation::HRSRC,
    pub lpOFN: *mut super::super::UI::Controls::Dialogs::OPENFILENAMEA,
    pub dwReserved1: [u32; 4],
    pub lpOleUILinkContainer: ::windows::core::ManuallyDrop<IOleUILinkContainerA>,
    pub dwLink: u32,
    pub lpszDisplayName: ::windows::core::PSTR,
    pub nFileLength: u32,
    pub lpszFrom: ::windows::core::PSTR,
    pub lpszTo: ::windows::core::PSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls_Dialogs"))]
impl ::core::clone::Clone for OLEUICHANGESOURCEA {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls_Dialogs"))]
impl ::core::fmt::Debug for OLEUICHANGESOURCEA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OLEUICHANGESOURCEA")
            .field("cbStruct", &self.cbStruct)
            .field("dwFlags", &self.dwFlags)
            .field("hWndOwner", &self.hWndOwner)
            .field("lpszCaption", &self.lpszCaption)
            .field("lCustData", &self.lCustData)
            .field("hInstance", &self.hInstance)
            .field("lpszTemplate", &self.lpszTemplate)
            .field("hResource", &self.hResource)
            .field("lpOFN", &self.lpOFN)
            .field("dwReserved1", &self.dwReserved1)
            .field("lpOleUILinkContainer", &self.lpOleUILinkContainer)
            .field("dwLink", &self.dwLink)
            .field("lpszDisplayName", &self.lpszDisplayName)
            .field("nFileLength", &self.nFileLength)
            .field("lpszFrom", &self.lpszFrom)
            .field("lpszTo", &self.lpszTo)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls_Dialogs"))]
unsafe impl ::windows::core::Abi for OLEUICHANGESOURCEA {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls_Dialogs"))]
impl ::core::default::Default for OLEUICHANGESOURCEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Controls_Dialogs\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls_Dialogs"))]
pub struct OLEUICHANGESOURCEW {
    pub cbStruct: u32,
    pub dwFlags: CHANGE_SOURCE_FLAGS,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: ::windows::core::PCWSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: ::windows::core::PCWSTR,
    pub hResource: super::super::Foundation::HRSRC,
    pub lpOFN: *mut super::super::UI::Controls::Dialogs::OPENFILENAMEW,
    pub dwReserved1: [u32; 4],
    pub lpOleUILinkContainer: ::windows::core::ManuallyDrop<IOleUILinkContainerW>,
    pub dwLink: u32,
    pub lpszDisplayName: ::windows::core::PWSTR,
    pub nFileLength: u32,
    pub lpszFrom: ::windows::core::PWSTR,
    pub lpszTo: ::windows::core::PWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls_Dialogs"))]
impl ::core::clone::Clone for OLEUICHANGESOURCEW {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls_Dialogs"))]
impl ::core::fmt::Debug for OLEUICHANGESOURCEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OLEUICHANGESOURCEW")
            .field("cbStruct", &self.cbStruct)
            .field("dwFlags", &self.dwFlags)
            .field("hWndOwner", &self.hWndOwner)
            .field("lpszCaption", &self.lpszCaption)
            .field("lCustData", &self.lCustData)
            .field("hInstance", &self.hInstance)
            .field("lpszTemplate", &self.lpszTemplate)
            .field("hResource", &self.hResource)
            .field("lpOFN", &self.lpOFN)
            .field("dwReserved1", &self.dwReserved1)
            .field("lpOleUILinkContainer", &self.lpOleUILinkContainer)
            .field("dwLink", &self.dwLink)
            .field("lpszDisplayName", &self.lpszDisplayName)
            .field("nFileLength", &self.nFileLength)
            .field("lpszFrom", &self.lpszFrom)
            .field("lpszTo", &self.lpszTo)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls_Dialogs"))]
unsafe impl ::windows::core::Abi for OLEUICHANGESOURCEW {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls_Dialogs"))]
impl ::core::default::Default for OLEUICHANGESOURCEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OLEUICONVERTA {
    pub cbStruct: u32,
    pub dwFlags: UI_CONVERT_FLAGS,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: ::windows::core::PCSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: ::windows::core::PCSTR,
    pub hResource: super::super::Foundation::HRSRC,
    pub clsid: ::windows::core::GUID,
    pub clsidConvertDefault: ::windows::core::GUID,
    pub clsidActivateDefault: ::windows::core::GUID,
    pub clsidNew: ::windows::core::GUID,
    pub dvAspect: u32,
    pub wFormat: u16,
    pub fIsLinkedObject: super::super::Foundation::BOOL,
    pub hMetaPict: isize,
    pub lpszUserType: ::windows::core::PSTR,
    pub fObjectsIconChanged: super::super::Foundation::BOOL,
    pub lpszDefLabel: ::windows::core::PSTR,
    pub cClsidExclude: u32,
    pub lpClsidExclude: *mut ::windows::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OLEUICONVERTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OLEUICONVERTA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for OLEUICONVERTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OLEUICONVERTA")
            .field("cbStruct", &self.cbStruct)
            .field("dwFlags", &self.dwFlags)
            .field("hWndOwner", &self.hWndOwner)
            .field("lpszCaption", &self.lpszCaption)
            .field("lCustData", &self.lCustData)
            .field("hInstance", &self.hInstance)
            .field("lpszTemplate", &self.lpszTemplate)
            .field("hResource", &self.hResource)
            .field("clsid", &self.clsid)
            .field("clsidConvertDefault", &self.clsidConvertDefault)
            .field("clsidActivateDefault", &self.clsidActivateDefault)
            .field("clsidNew", &self.clsidNew)
            .field("dvAspect", &self.dvAspect)
            .field("wFormat", &self.wFormat)
            .field("fIsLinkedObject", &self.fIsLinkedObject)
            .field("hMetaPict", &self.hMetaPict)
            .field("lpszUserType", &self.lpszUserType)
            .field("fObjectsIconChanged", &self.fObjectsIconChanged)
            .field("lpszDefLabel", &self.lpszDefLabel)
            .field("cClsidExclude", &self.cClsidExclude)
            .field("lpClsidExclude", &self.lpClsidExclude)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for OLEUICONVERTA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OLEUICONVERTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OLEUICONVERTW {
    pub cbStruct: u32,
    pub dwFlags: UI_CONVERT_FLAGS,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: ::windows::core::PCWSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: ::windows::core::PCWSTR,
    pub hResource: super::super::Foundation::HRSRC,
    pub clsid: ::windows::core::GUID,
    pub clsidConvertDefault: ::windows::core::GUID,
    pub clsidActivateDefault: ::windows::core::GUID,
    pub clsidNew: ::windows::core::GUID,
    pub dvAspect: u32,
    pub wFormat: u16,
    pub fIsLinkedObject: super::super::Foundation::BOOL,
    pub hMetaPict: isize,
    pub lpszUserType: ::windows::core::PWSTR,
    pub fObjectsIconChanged: super::super::Foundation::BOOL,
    pub lpszDefLabel: ::windows::core::PWSTR,
    pub cClsidExclude: u32,
    pub lpClsidExclude: *mut ::windows::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OLEUICONVERTW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OLEUICONVERTW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for OLEUICONVERTW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OLEUICONVERTW")
            .field("cbStruct", &self.cbStruct)
            .field("dwFlags", &self.dwFlags)
            .field("hWndOwner", &self.hWndOwner)
            .field("lpszCaption", &self.lpszCaption)
            .field("lCustData", &self.lCustData)
            .field("hInstance", &self.hInstance)
            .field("lpszTemplate", &self.lpszTemplate)
            .field("hResource", &self.hResource)
            .field("clsid", &self.clsid)
            .field("clsidConvertDefault", &self.clsidConvertDefault)
            .field("clsidActivateDefault", &self.clsidActivateDefault)
            .field("clsidNew", &self.clsidNew)
            .field("dvAspect", &self.dvAspect)
            .field("wFormat", &self.wFormat)
            .field("fIsLinkedObject", &self.fIsLinkedObject)
            .field("hMetaPict", &self.hMetaPict)
            .field("lpszUserType", &self.lpszUserType)
            .field("fObjectsIconChanged", &self.fObjectsIconChanged)
            .field("lpszDefLabel", &self.lpszDefLabel)
            .field("cClsidExclude", &self.cClsidExclude)
            .field("lpClsidExclude", &self.lpClsidExclude)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for OLEUICONVERTW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OLEUICONVERTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OLEUIEDITLINKSA {
    pub cbStruct: u32,
    pub dwFlags: EDIT_LINKS_FLAGS,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: ::windows::core::PCSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: ::windows::core::PCSTR,
    pub hResource: super::super::Foundation::HRSRC,
    pub lpOleUILinkContainer: ::windows::core::ManuallyDrop<IOleUILinkContainerA>,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OLEUIEDITLINKSA {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for OLEUIEDITLINKSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OLEUIEDITLINKSA").field("cbStruct", &self.cbStruct).field("dwFlags", &self.dwFlags).field("hWndOwner", &self.hWndOwner).field("lpszCaption", &self.lpszCaption).field("lCustData", &self.lCustData).field("hInstance", &self.hInstance).field("lpszTemplate", &self.lpszTemplate).field("hResource", &self.hResource).field("lpOleUILinkContainer", &self.lpOleUILinkContainer).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for OLEUIEDITLINKSA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OLEUIEDITLINKSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OLEUIEDITLINKSW {
    pub cbStruct: u32,
    pub dwFlags: EDIT_LINKS_FLAGS,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: ::windows::core::PCWSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: ::windows::core::PCWSTR,
    pub hResource: super::super::Foundation::HRSRC,
    pub lpOleUILinkContainer: ::windows::core::ManuallyDrop<IOleUILinkContainerW>,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OLEUIEDITLINKSW {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for OLEUIEDITLINKSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OLEUIEDITLINKSW").field("cbStruct", &self.cbStruct).field("dwFlags", &self.dwFlags).field("hWndOwner", &self.hWndOwner).field("lpszCaption", &self.lpszCaption).field("lCustData", &self.lCustData).field("hInstance", &self.hInstance).field("lpszTemplate", &self.lpszTemplate).field("hResource", &self.hResource).field("lpOleUILinkContainer", &self.lpOleUILinkContainer).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for OLEUIEDITLINKSW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OLEUIEDITLINKSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_Controls\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct OLEUIGNRLPROPSA {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub dwReserved1: [u32; 2],
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub dwReserved2: [u32; 3],
    pub lpOP: *mut OLEUIOBJECTPROPSA,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for OLEUIGNRLPROPSA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for OLEUIGNRLPROPSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::fmt::Debug for OLEUIGNRLPROPSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OLEUIGNRLPROPSA").field("cbStruct", &self.cbStruct).field("dwFlags", &self.dwFlags).field("dwReserved1", &self.dwReserved1).field("lCustData", &self.lCustData).field("dwReserved2", &self.dwReserved2).field("lpOP", &self.lpOP).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for OLEUIGNRLPROPSA {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for OLEUIGNRLPROPSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_Controls\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct OLEUIGNRLPROPSW {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub dwReserved1: [u32; 2],
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub dwReserved2: [u32; 3],
    pub lpOP: *mut OLEUIOBJECTPROPSW,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for OLEUIGNRLPROPSW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for OLEUIGNRLPROPSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::fmt::Debug for OLEUIGNRLPROPSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OLEUIGNRLPROPSW").field("cbStruct", &self.cbStruct).field("dwFlags", &self.dwFlags).field("dwReserved1", &self.dwReserved1).field("lCustData", &self.lCustData).field("dwReserved2", &self.dwReserved2).field("lpOP", &self.lpOP).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for OLEUIGNRLPROPSW {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for OLEUIGNRLPROPSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub struct OLEUIINSERTOBJECTA {
    pub cbStruct: u32,
    pub dwFlags: INSERT_OBJECT_FLAGS,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: ::windows::core::PCSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: ::windows::core::PCSTR,
    pub hResource: super::super::Foundation::HRSRC,
    pub clsid: ::windows::core::GUID,
    pub lpszFile: ::windows::core::PSTR,
    pub cchFile: u32,
    pub cClsidExclude: u32,
    pub lpClsidExclude: *mut ::windows::core::GUID,
    pub iid: ::windows::core::GUID,
    pub oleRender: u32,
    pub lpFormatEtc: *mut super::Com::FORMATETC,
    pub lpIOleClientSite: ::windows::core::ManuallyDrop<IOleClientSite>,
    pub lpIStorage: ::windows::core::ManuallyDrop<super::Com::StructuredStorage::IStorage>,
    pub ppvObj: *mut *mut ::core::ffi::c_void,
    pub sc: i32,
    pub hMetaPict: isize,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for OLEUIINSERTOBJECTA {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::fmt::Debug for OLEUIINSERTOBJECTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OLEUIINSERTOBJECTA")
            .field("cbStruct", &self.cbStruct)
            .field("dwFlags", &self.dwFlags)
            .field("hWndOwner", &self.hWndOwner)
            .field("lpszCaption", &self.lpszCaption)
            .field("lCustData", &self.lCustData)
            .field("hInstance", &self.hInstance)
            .field("lpszTemplate", &self.lpszTemplate)
            .field("hResource", &self.hResource)
            .field("clsid", &self.clsid)
            .field("lpszFile", &self.lpszFile)
            .field("cchFile", &self.cchFile)
            .field("cClsidExclude", &self.cClsidExclude)
            .field("lpClsidExclude", &self.lpClsidExclude)
            .field("iid", &self.iid)
            .field("oleRender", &self.oleRender)
            .field("lpFormatEtc", &self.lpFormatEtc)
            .field("lpIOleClientSite", &self.lpIOleClientSite)
            .field("lpIStorage", &self.lpIStorage)
            .field("ppvObj", &self.ppvObj)
            .field("sc", &self.sc)
            .field("hMetaPict", &self.hMetaPict)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
unsafe impl ::windows::core::Abi for OLEUIINSERTOBJECTA {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for OLEUIINSERTOBJECTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub struct OLEUIINSERTOBJECTW {
    pub cbStruct: u32,
    pub dwFlags: INSERT_OBJECT_FLAGS,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: ::windows::core::PCWSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: ::windows::core::PCWSTR,
    pub hResource: super::super::Foundation::HRSRC,
    pub clsid: ::windows::core::GUID,
    pub lpszFile: ::windows::core::PWSTR,
    pub cchFile: u32,
    pub cClsidExclude: u32,
    pub lpClsidExclude: *mut ::windows::core::GUID,
    pub iid: ::windows::core::GUID,
    pub oleRender: u32,
    pub lpFormatEtc: *mut super::Com::FORMATETC,
    pub lpIOleClientSite: ::windows::core::ManuallyDrop<IOleClientSite>,
    pub lpIStorage: ::windows::core::ManuallyDrop<super::Com::StructuredStorage::IStorage>,
    pub ppvObj: *mut *mut ::core::ffi::c_void,
    pub sc: i32,
    pub hMetaPict: isize,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for OLEUIINSERTOBJECTW {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::fmt::Debug for OLEUIINSERTOBJECTW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OLEUIINSERTOBJECTW")
            .field("cbStruct", &self.cbStruct)
            .field("dwFlags", &self.dwFlags)
            .field("hWndOwner", &self.hWndOwner)
            .field("lpszCaption", &self.lpszCaption)
            .field("lCustData", &self.lCustData)
            .field("hInstance", &self.hInstance)
            .field("lpszTemplate", &self.lpszTemplate)
            .field("hResource", &self.hResource)
            .field("clsid", &self.clsid)
            .field("lpszFile", &self.lpszFile)
            .field("cchFile", &self.cchFile)
            .field("cClsidExclude", &self.cClsidExclude)
            .field("lpClsidExclude", &self.lpClsidExclude)
            .field("iid", &self.iid)
            .field("oleRender", &self.oleRender)
            .field("lpFormatEtc", &self.lpFormatEtc)
            .field("lpIOleClientSite", &self.lpIOleClientSite)
            .field("lpIStorage", &self.lpIStorage)
            .field("ppvObj", &self.ppvObj)
            .field("sc", &self.sc)
            .field("hMetaPict", &self.hMetaPict)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
unsafe impl ::windows::core::Abi for OLEUIINSERTOBJECTW {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for OLEUIINSERTOBJECTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_Controls\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct OLEUILINKPROPSA {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub dwReserved1: [u32; 2],
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub dwReserved2: [u32; 3],
    pub lpOP: *mut OLEUIOBJECTPROPSA,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for OLEUILINKPROPSA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for OLEUILINKPROPSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::fmt::Debug for OLEUILINKPROPSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OLEUILINKPROPSA").field("cbStruct", &self.cbStruct).field("dwFlags", &self.dwFlags).field("dwReserved1", &self.dwReserved1).field("lCustData", &self.lCustData).field("dwReserved2", &self.dwReserved2).field("lpOP", &self.lpOP).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for OLEUILINKPROPSA {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for OLEUILINKPROPSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_Controls\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct OLEUILINKPROPSW {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub dwReserved1: [u32; 2],
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub dwReserved2: [u32; 3],
    pub lpOP: *mut OLEUIOBJECTPROPSW,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for OLEUILINKPROPSW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for OLEUILINKPROPSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::fmt::Debug for OLEUILINKPROPSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OLEUILINKPROPSW").field("cbStruct", &self.cbStruct).field("dwFlags", &self.dwFlags).field("dwReserved1", &self.dwReserved1).field("lCustData", &self.lCustData).field("dwReserved2", &self.dwReserved2).field("lpOP", &self.lpOP).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for OLEUILINKPROPSW {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for OLEUILINKPROPSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_Controls\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct OLEUIOBJECTPROPSA {
    pub cbStruct: u32,
    pub dwFlags: OBJECT_PROPERTIES_FLAGS,
    pub lpPS: *mut super::super::UI::Controls::PROPSHEETHEADERA_V2,
    pub dwObject: u32,
    pub lpObjInfo: ::windows::core::ManuallyDrop<IOleUIObjInfoA>,
    pub dwLink: u32,
    pub lpLinkInfo: ::windows::core::ManuallyDrop<IOleUILinkInfoA>,
    pub lpGP: *mut OLEUIGNRLPROPSA,
    pub lpVP: *mut OLEUIVIEWPROPSA,
    pub lpLP: *mut OLEUILINKPROPSA,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for OLEUIOBJECTPROPSA {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::fmt::Debug for OLEUIOBJECTPROPSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OLEUIOBJECTPROPSA").field("cbStruct", &self.cbStruct).field("dwFlags", &self.dwFlags).field("lpPS", &self.lpPS).field("dwObject", &self.dwObject).field("lpObjInfo", &self.lpObjInfo).field("dwLink", &self.dwLink).field("lpLinkInfo", &self.lpLinkInfo).field("lpGP", &self.lpGP).field("lpVP", &self.lpVP).field("lpLP", &self.lpLP).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for OLEUIOBJECTPROPSA {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for OLEUIOBJECTPROPSA {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.dwFlags == other.dwFlags && self.lpPS == other.lpPS && self.dwObject == other.dwObject && self.lpObjInfo == other.lpObjInfo && self.dwLink == other.dwLink && self.lpLinkInfo == other.lpLinkInfo && self.lpGP == other.lpGP && self.lpVP == other.lpVP && self.lpLP == other.lpLP
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for OLEUIOBJECTPROPSA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for OLEUIOBJECTPROPSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_Controls\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct OLEUIOBJECTPROPSW {
    pub cbStruct: u32,
    pub dwFlags: OBJECT_PROPERTIES_FLAGS,
    pub lpPS: *mut super::super::UI::Controls::PROPSHEETHEADERW_V2,
    pub dwObject: u32,
    pub lpObjInfo: ::windows::core::ManuallyDrop<IOleUIObjInfoW>,
    pub dwLink: u32,
    pub lpLinkInfo: ::windows::core::ManuallyDrop<IOleUILinkInfoW>,
    pub lpGP: *mut OLEUIGNRLPROPSW,
    pub lpVP: *mut OLEUIVIEWPROPSW,
    pub lpLP: *mut OLEUILINKPROPSW,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for OLEUIOBJECTPROPSW {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::fmt::Debug for OLEUIOBJECTPROPSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OLEUIOBJECTPROPSW").field("cbStruct", &self.cbStruct).field("dwFlags", &self.dwFlags).field("lpPS", &self.lpPS).field("dwObject", &self.dwObject).field("lpObjInfo", &self.lpObjInfo).field("dwLink", &self.dwLink).field("lpLinkInfo", &self.lpLinkInfo).field("lpGP", &self.lpGP).field("lpVP", &self.lpVP).field("lpLP", &self.lpLP).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for OLEUIOBJECTPROPSW {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for OLEUIOBJECTPROPSW {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.dwFlags == other.dwFlags && self.lpPS == other.lpPS && self.dwObject == other.dwObject && self.lpObjInfo == other.lpObjInfo && self.dwLink == other.dwLink && self.lpLinkInfo == other.lpLinkInfo && self.lpGP == other.lpGP && self.lpVP == other.lpVP && self.lpLP == other.lpLP
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for OLEUIOBJECTPROPSW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for OLEUIOBJECTPROPSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct OLEUIPASTEENTRYA {
    pub fmtetc: super::Com::FORMATETC,
    pub lpstrFormatName: ::windows::core::PCSTR,
    pub lpstrResultText: ::windows::core::PCSTR,
    pub dwFlags: u32,
    pub dwScratchSpace: u32,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for OLEUIPASTEENTRYA {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for OLEUIPASTEENTRYA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for OLEUIPASTEENTRYA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OLEUIPASTEENTRYA").field("fmtetc", &self.fmtetc).field("lpstrFormatName", &self.lpstrFormatName).field("lpstrResultText", &self.lpstrResultText).field("dwFlags", &self.dwFlags).field("dwScratchSpace", &self.dwScratchSpace).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for OLEUIPASTEENTRYA {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for OLEUIPASTEENTRYA {
    fn eq(&self, other: &Self) -> bool {
        self.fmtetc == other.fmtetc && self.lpstrFormatName == other.lpstrFormatName && self.lpstrResultText == other.lpstrResultText && self.dwFlags == other.dwFlags && self.dwScratchSpace == other.dwScratchSpace
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for OLEUIPASTEENTRYA {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for OLEUIPASTEENTRYA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct OLEUIPASTEENTRYW {
    pub fmtetc: super::Com::FORMATETC,
    pub lpstrFormatName: ::windows::core::PCWSTR,
    pub lpstrResultText: ::windows::core::PCWSTR,
    pub dwFlags: u32,
    pub dwScratchSpace: u32,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for OLEUIPASTEENTRYW {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for OLEUIPASTEENTRYW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for OLEUIPASTEENTRYW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OLEUIPASTEENTRYW").field("fmtetc", &self.fmtetc).field("lpstrFormatName", &self.lpstrFormatName).field("lpstrResultText", &self.lpstrResultText).field("dwFlags", &self.dwFlags).field("dwScratchSpace", &self.dwScratchSpace).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for OLEUIPASTEENTRYW {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for OLEUIPASTEENTRYW {
    fn eq(&self, other: &Self) -> bool {
        self.fmtetc == other.fmtetc && self.lpstrFormatName == other.lpstrFormatName && self.lpstrResultText == other.lpstrResultText && self.dwFlags == other.dwFlags && self.dwScratchSpace == other.dwScratchSpace
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for OLEUIPASTEENTRYW {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for OLEUIPASTEENTRYW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct OLEUIPASTESPECIALA {
    pub cbStruct: u32,
    pub dwFlags: PASTE_SPECIAL_FLAGS,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: ::windows::core::PCSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: ::windows::core::PCSTR,
    pub hResource: super::super::Foundation::HRSRC,
    pub lpSrcDataObj: ::windows::core::ManuallyDrop<super::Com::IDataObject>,
    pub arrPasteEntries: *mut OLEUIPASTEENTRYA,
    pub cPasteEntries: i32,
    pub arrLinkTypes: *mut u32,
    pub cLinkTypes: i32,
    pub cClsidExclude: u32,
    pub lpClsidExclude: *mut ::windows::core::GUID,
    pub nSelectedIndex: i32,
    pub fLink: super::super::Foundation::BOOL,
    pub hMetaPict: isize,
    pub sizel: super::super::Foundation::SIZE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for OLEUIPASTESPECIALA {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for OLEUIPASTESPECIALA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OLEUIPASTESPECIALA")
            .field("cbStruct", &self.cbStruct)
            .field("dwFlags", &self.dwFlags)
            .field("hWndOwner", &self.hWndOwner)
            .field("lpszCaption", &self.lpszCaption)
            .field("lCustData", &self.lCustData)
            .field("hInstance", &self.hInstance)
            .field("lpszTemplate", &self.lpszTemplate)
            .field("hResource", &self.hResource)
            .field("lpSrcDataObj", &self.lpSrcDataObj)
            .field("arrPasteEntries", &self.arrPasteEntries)
            .field("cPasteEntries", &self.cPasteEntries)
            .field("arrLinkTypes", &self.arrLinkTypes)
            .field("cLinkTypes", &self.cLinkTypes)
            .field("cClsidExclude", &self.cClsidExclude)
            .field("lpClsidExclude", &self.lpClsidExclude)
            .field("nSelectedIndex", &self.nSelectedIndex)
            .field("fLink", &self.fLink)
            .field("hMetaPict", &self.hMetaPict)
            .field("sizel", &self.sizel)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for OLEUIPASTESPECIALA {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for OLEUIPASTESPECIALA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct OLEUIPASTESPECIALW {
    pub cbStruct: u32,
    pub dwFlags: PASTE_SPECIAL_FLAGS,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: ::windows::core::PCWSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: ::windows::core::PCWSTR,
    pub hResource: super::super::Foundation::HRSRC,
    pub lpSrcDataObj: ::windows::core::ManuallyDrop<super::Com::IDataObject>,
    pub arrPasteEntries: *mut OLEUIPASTEENTRYW,
    pub cPasteEntries: i32,
    pub arrLinkTypes: *mut u32,
    pub cLinkTypes: i32,
    pub cClsidExclude: u32,
    pub lpClsidExclude: *mut ::windows::core::GUID,
    pub nSelectedIndex: i32,
    pub fLink: super::super::Foundation::BOOL,
    pub hMetaPict: isize,
    pub sizel: super::super::Foundation::SIZE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for OLEUIPASTESPECIALW {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for OLEUIPASTESPECIALW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OLEUIPASTESPECIALW")
            .field("cbStruct", &self.cbStruct)
            .field("dwFlags", &self.dwFlags)
            .field("hWndOwner", &self.hWndOwner)
            .field("lpszCaption", &self.lpszCaption)
            .field("lCustData", &self.lCustData)
            .field("hInstance", &self.hInstance)
            .field("lpszTemplate", &self.lpszTemplate)
            .field("hResource", &self.hResource)
            .field("lpSrcDataObj", &self.lpSrcDataObj)
            .field("arrPasteEntries", &self.arrPasteEntries)
            .field("cPasteEntries", &self.cPasteEntries)
            .field("arrLinkTypes", &self.arrLinkTypes)
            .field("cLinkTypes", &self.cLinkTypes)
            .field("cClsidExclude", &self.cClsidExclude)
            .field("lpClsidExclude", &self.lpClsidExclude)
            .field("nSelectedIndex", &self.nSelectedIndex)
            .field("fLink", &self.fLink)
            .field("hMetaPict", &self.hMetaPict)
            .field("sizel", &self.sizel)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for OLEUIPASTESPECIALW {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for OLEUIPASTESPECIALW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_Controls\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct OLEUIVIEWPROPSA {
    pub cbStruct: u32,
    pub dwFlags: VIEW_OBJECT_PROPERTIES_FLAGS,
    pub dwReserved1: [u32; 2],
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub dwReserved2: [u32; 3],
    pub lpOP: *mut OLEUIOBJECTPROPSA,
    pub nScaleMin: i32,
    pub nScaleMax: i32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for OLEUIVIEWPROPSA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for OLEUIVIEWPROPSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::fmt::Debug for OLEUIVIEWPROPSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OLEUIVIEWPROPSA").field("cbStruct", &self.cbStruct).field("dwFlags", &self.dwFlags).field("dwReserved1", &self.dwReserved1).field("lCustData", &self.lCustData).field("dwReserved2", &self.dwReserved2).field("lpOP", &self.lpOP).field("nScaleMin", &self.nScaleMin).field("nScaleMax", &self.nScaleMax).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for OLEUIVIEWPROPSA {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for OLEUIVIEWPROPSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_Controls\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct OLEUIVIEWPROPSW {
    pub cbStruct: u32,
    pub dwFlags: VIEW_OBJECT_PROPERTIES_FLAGS,
    pub dwReserved1: [u32; 2],
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub dwReserved2: [u32; 3],
    pub lpOP: *mut OLEUIOBJECTPROPSW,
    pub nScaleMin: i32,
    pub nScaleMax: i32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for OLEUIVIEWPROPSW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for OLEUIVIEWPROPSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::fmt::Debug for OLEUIVIEWPROPSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OLEUIVIEWPROPSW").field("cbStruct", &self.cbStruct).field("dwFlags", &self.dwFlags).field("dwReserved1", &self.dwReserved1).field("lCustData", &self.lCustData).field("dwReserved2", &self.dwReserved2).field("lpOP", &self.lpOP).field("nScaleMin", &self.nScaleMin).field("nScaleMax", &self.nScaleMax).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for OLEUIVIEWPROPSW {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for OLEUIVIEWPROPSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub struct OLEVERB {
    pub lVerb: OLEIVERB,
    pub lpszVerbName: ::windows::core::PWSTR,
    pub fuFlags: super::super::UI::WindowsAndMessaging::MENU_ITEM_FLAGS,
    pub grfAttribs: OLEVERBATTRIB,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::marker::Copy for OLEVERB {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::clone::Clone for OLEVERB {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::fmt::Debug for OLEVERB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OLEVERB").field("lVerb", &self.lVerb).field("lpszVerbName", &self.lpszVerbName).field("fuFlags", &self.fuFlags).field("grfAttribs", &self.grfAttribs).finish()
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
unsafe impl ::windows::core::Abi for OLEVERB {
    type Abi = Self;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::cmp::PartialEq for OLEVERB {
    fn eq(&self, other: &Self) -> bool {
        self.lVerb == other.lVerb && self.lpszVerbName == other.lpszVerbName && self.fuFlags == other.fuFlags && self.grfAttribs == other.grfAttribs
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::cmp::Eq for OLEVERB {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::default::Default for OLEVERB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OLE_HANDLE(pub u32);
impl OLE_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == 0
    }
}
impl ::core::default::Default for OLE_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for OLE_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for OLE_HANDLE {}
impl ::core::fmt::Debug for OLE_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OLE_HANDLE").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<OLE_HANDLE>> for OLE_HANDLE {
    fn from(optional: ::core::option::Option<OLE_HANDLE>) -> OLE_HANDLE {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for OLE_HANDLE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub struct PAGERANGE {
    pub nFromPage: i32,
    pub nToPage: i32,
}
impl ::core::marker::Copy for PAGERANGE {}
impl ::core::clone::Clone for PAGERANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PAGERANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PAGERANGE").field("nFromPage", &self.nFromPage).field("nToPage", &self.nToPage).finish()
    }
}
unsafe impl ::windows::core::Abi for PAGERANGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PAGERANGE {
    fn eq(&self, other: &Self) -> bool {
        self.nFromPage == other.nFromPage && self.nToPage == other.nToPage
    }
}
impl ::core::cmp::Eq for PAGERANGE {}
impl ::core::default::Default for PAGERANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PAGESET {
    pub cbStruct: u32,
    pub fOddPages: super::super::Foundation::BOOL,
    pub fEvenPages: super::super::Foundation::BOOL,
    pub cPageRange: u32,
    pub rgPages: [PAGERANGE; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PAGESET {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PAGESET {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PAGESET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PAGESET").field("cbStruct", &self.cbStruct).field("fOddPages", &self.fOddPages).field("fEvenPages", &self.fEvenPages).field("cPageRange", &self.cPageRange).field("rgPages", &self.rgPages).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PAGESET {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PAGESET {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.fOddPages == other.fOddPages && self.fEvenPages == other.fEvenPages && self.cPageRange == other.cPageRange && self.rgPages == other.rgPages
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PAGESET {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PAGESET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct PARAMDATA {
    pub szName: ::windows::core::PWSTR,
    pub vt: super::Com::VARENUM,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for PARAMDATA {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for PARAMDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for PARAMDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PARAMDATA").field("szName", &self.szName).field("vt", &self.vt).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for PARAMDATA {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for PARAMDATA {
    fn eq(&self, other: &Self) -> bool {
        self.szName == other.szName && self.vt == other.vt
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for PARAMDATA {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for PARAMDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct PARAMDESC {
    pub pparamdescex: *mut PARAMDESCEX,
    pub wParamFlags: PARAMFLAGS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for PARAMDESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for PARAMDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for PARAMDESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PARAMDESC").field("pparamdescex", &self.pparamdescex).field("wParamFlags", &self.wParamFlags).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for PARAMDESC {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for PARAMDESC {
    fn eq(&self, other: &Self) -> bool {
        self.pparamdescex == other.pparamdescex && self.wParamFlags == other.wParamFlags
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for PARAMDESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for PARAMDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct PARAMDESCEX {
    pub cBytes: u32,
    pub varDefaultValue: super::Com::VARIANT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for PARAMDESCEX {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for PARAMDESCEX {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for PARAMDESCEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct PICTDESC {
    pub cbSizeofstruct: u32,
    pub picType: PICTYPE,
    pub Anonymous: PICTDESC_0,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PICTDESC {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PICTDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for PICTDESC {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PICTDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PICTDESC_0 {
    pub bmp: PICTDESC_0_0,
    pub wmf: PICTDESC_0_3,
    pub icon: PICTDESC_0_2,
    pub emf: PICTDESC_0_1,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PICTDESC_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PICTDESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for PICTDESC_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PICTDESC_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct PICTDESC_0_0 {
    pub hbitmap: super::super::Graphics::Gdi::HBITMAP,
    pub hpal: super::super::Graphics::Gdi::HPALETTE,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PICTDESC_0_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PICTDESC_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::fmt::Debug for PICTDESC_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PICTDESC_0_0").field("hbitmap", &self.hbitmap).field("hpal", &self.hpal).finish()
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for PICTDESC_0_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for PICTDESC_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.hbitmap == other.hbitmap && self.hpal == other.hpal
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for PICTDESC_0_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PICTDESC_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct PICTDESC_0_1 {
    pub hemf: super::super::Graphics::Gdi::HENHMETAFILE,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PICTDESC_0_1 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PICTDESC_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::fmt::Debug for PICTDESC_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PICTDESC_0_1").field("hemf", &self.hemf).finish()
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for PICTDESC_0_1 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for PICTDESC_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.hemf == other.hemf
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for PICTDESC_0_1 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PICTDESC_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct PICTDESC_0_2 {
    pub hicon: super::super::UI::WindowsAndMessaging::HICON,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PICTDESC_0_2 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PICTDESC_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::fmt::Debug for PICTDESC_0_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PICTDESC_0_2").field("hicon", &self.hicon).finish()
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for PICTDESC_0_2 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for PICTDESC_0_2 {
    fn eq(&self, other: &Self) -> bool {
        self.hicon == other.hicon
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for PICTDESC_0_2 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PICTDESC_0_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct PICTDESC_0_3 {
    pub hmeta: super::super::Graphics::Gdi::HMETAFILE,
    pub xExt: i32,
    pub yExt: i32,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PICTDESC_0_3 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PICTDESC_0_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::fmt::Debug for PICTDESC_0_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PICTDESC_0_3").field("hmeta", &self.hmeta).field("xExt", &self.xExt).field("yExt", &self.yExt).finish()
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for PICTDESC_0_3 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for PICTDESC_0_3 {
    fn eq(&self, other: &Self) -> bool {
        self.hmeta == other.hmeta && self.xExt == other.xExt && self.yExt == other.yExt
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for PICTDESC_0_3 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PICTDESC_0_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub struct POINTF {
    pub x: f32,
    pub y: f32,
}
impl ::core::marker::Copy for POINTF {}
impl ::core::clone::Clone for POINTF {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for POINTF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POINTF").field("x", &self.x).field("y", &self.y).finish()
    }
}
unsafe impl ::windows::core::Abi for POINTF {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for POINTF {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl ::core::cmp::Eq for POINTF {}
impl ::core::default::Default for POINTF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PROPPAGEINFO {
    pub cb: u32,
    pub pszTitle: ::windows::core::PWSTR,
    pub size: super::super::Foundation::SIZE,
    pub pszDocString: ::windows::core::PWSTR,
    pub pszHelpFile: ::windows::core::PWSTR,
    pub dwHelpContext: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PROPPAGEINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PROPPAGEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PROPPAGEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROPPAGEINFO").field("cb", &self.cb).field("pszTitle", &self.pszTitle).field("size", &self.size).field("pszDocString", &self.pszDocString).field("pszHelpFile", &self.pszHelpFile).field("dwHelpContext", &self.dwHelpContext).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PROPPAGEINFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PROPPAGEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.pszTitle == other.pszTitle && self.size == other.size && self.pszDocString == other.pszDocString && self.pszHelpFile == other.pszHelpFile && self.dwHelpContext == other.dwHelpContext
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PROPPAGEINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PROPPAGEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub struct QACONTAINER {
    pub cbSize: u32,
    pub pClientSite: ::windows::core::ManuallyDrop<IOleClientSite>,
    pub pAdviseSink: ::windows::core::ManuallyDrop<IAdviseSinkEx>,
    pub pPropertyNotifySink: ::windows::core::ManuallyDrop<IPropertyNotifySink>,
    pub pUnkEventSink: ::windows::core::ManuallyDrop<::windows::core::IUnknown>,
    pub dwAmbientFlags: QACONTAINERFLAGS,
    pub colorFore: u32,
    pub colorBack: u32,
    pub pFont: ::windows::core::ManuallyDrop<IFont>,
    pub pUndoMgr: ::windows::core::ManuallyDrop<IOleUndoManager>,
    pub dwAppearance: u32,
    pub lcid: i32,
    pub hpal: super::super::Graphics::Gdi::HPALETTE,
    pub pBindHost: ::windows::core::ManuallyDrop<super::Com::IBindHost>,
    pub pOleControlSite: ::windows::core::ManuallyDrop<IOleControlSite>,
    pub pServiceProvider: ::windows::core::ManuallyDrop<super::Com::IServiceProvider>,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for QACONTAINER {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for QACONTAINER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QACONTAINER")
            .field("cbSize", &self.cbSize)
            .field("pClientSite", &self.pClientSite)
            .field("pAdviseSink", &self.pAdviseSink)
            .field("pPropertyNotifySink", &self.pPropertyNotifySink)
            .field("pUnkEventSink", &self.pUnkEventSink)
            .field("dwAmbientFlags", &self.dwAmbientFlags)
            .field("colorFore", &self.colorFore)
            .field("colorBack", &self.colorBack)
            .field("pFont", &self.pFont)
            .field("pUndoMgr", &self.pUndoMgr)
            .field("dwAppearance", &self.dwAppearance)
            .field("lcid", &self.lcid)
            .field("hpal", &self.hpal)
            .field("pBindHost", &self.pBindHost)
            .field("pOleControlSite", &self.pOleControlSite)
            .field("pServiceProvider", &self.pServiceProvider)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for QACONTAINER {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for QACONTAINER {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.pClientSite == other.pClientSite && self.pAdviseSink == other.pAdviseSink && self.pPropertyNotifySink == other.pPropertyNotifySink && self.pUnkEventSink == other.pUnkEventSink && self.dwAmbientFlags == other.dwAmbientFlags && self.colorFore == other.colorFore && self.colorBack == other.colorBack && self.pFont == other.pFont && self.pUndoMgr == other.pUndoMgr && self.dwAppearance == other.dwAppearance && self.lcid == other.lcid && self.hpal == other.hpal && self.pBindHost == other.pBindHost && self.pOleControlSite == other.pOleControlSite && self.pServiceProvider == other.pServiceProvider
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for QACONTAINER {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl ::core::default::Default for QACONTAINER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub struct QACONTROL {
    pub cbSize: u32,
    pub dwMiscStatus: OLEMISC,
    pub dwViewStatus: VIEWSTATUS,
    pub dwEventCookie: u32,
    pub dwPropNotifyCookie: u32,
    pub dwPointerActivationPolicy: POINTERINACTIVE,
}
impl ::core::marker::Copy for QACONTROL {}
impl ::core::clone::Clone for QACONTROL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QACONTROL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QACONTROL").field("cbSize", &self.cbSize).field("dwMiscStatus", &self.dwMiscStatus).field("dwViewStatus", &self.dwViewStatus).field("dwEventCookie", &self.dwEventCookie).field("dwPropNotifyCookie", &self.dwPropNotifyCookie).field("dwPointerActivationPolicy", &self.dwPointerActivationPolicy).finish()
    }
}
unsafe impl ::windows::core::Abi for QACONTROL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for QACONTROL {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwMiscStatus == other.dwMiscStatus && self.dwViewStatus == other.dwViewStatus && self.dwEventCookie == other.dwEventCookie && self.dwPropNotifyCookie == other.dwPropNotifyCookie && self.dwPointerActivationPolicy == other.dwPointerActivationPolicy
    }
}
impl ::core::cmp::Eq for QACONTROL {}
impl ::core::default::Default for QACONTROL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct SAFEARRAYUNION {
    pub sfType: u32,
    pub u: SAFEARRAYUNION_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for SAFEARRAYUNION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for SAFEARRAYUNION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for SAFEARRAYUNION {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for SAFEARRAYUNION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub union SAFEARRAYUNION_0 {
    pub BstrStr: SAFEARR_BSTR,
    pub UnknownStr: SAFEARR_UNKNOWN,
    pub DispatchStr: SAFEARR_DISPATCH,
    pub VariantStr: SAFEARR_VARIANT,
    pub RecordStr: SAFEARR_BRECORD,
    pub HaveIidStr: SAFEARR_HAVEIID,
    pub ByteStr: super::Com::BYTE_SIZEDARR,
    pub WordStr: super::Com::WORD_SIZEDARR,
    pub LongStr: super::Com::DWORD_SIZEDARR,
    pub HyperStr: super::Com::HYPER_SIZEDARR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for SAFEARRAYUNION_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for SAFEARRAYUNION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for SAFEARRAYUNION_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for SAFEARRAYUNION_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub struct SAFEARR_BRECORD {
    pub Size: u32,
    pub aRecord: *mut *mut _wireBRECORD,
}
impl ::core::marker::Copy for SAFEARR_BRECORD {}
impl ::core::clone::Clone for SAFEARR_BRECORD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SAFEARR_BRECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SAFEARR_BRECORD").field("Size", &self.Size).field("aRecord", &self.aRecord).finish()
    }
}
unsafe impl ::windows::core::Abi for SAFEARR_BRECORD {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SAFEARR_BRECORD {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.aRecord == other.aRecord
    }
}
impl ::core::cmp::Eq for SAFEARR_BRECORD {}
impl ::core::default::Default for SAFEARR_BRECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct SAFEARR_BSTR {
    pub Size: u32,
    pub aBstr: *mut *mut super::Com::FLAGGED_WORD_BLOB,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for SAFEARR_BSTR {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for SAFEARR_BSTR {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for SAFEARR_BSTR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SAFEARR_BSTR").field("Size", &self.Size).field("aBstr", &self.aBstr).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for SAFEARR_BSTR {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for SAFEARR_BSTR {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.aBstr == other.aBstr
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for SAFEARR_BSTR {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for SAFEARR_BSTR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct SAFEARR_DISPATCH {
    pub Size: u32,
    pub apDispatch: *mut ::core::option::Option<super::Com::IDispatch>,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for SAFEARR_DISPATCH {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for SAFEARR_DISPATCH {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for SAFEARR_DISPATCH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SAFEARR_DISPATCH").field("Size", &self.Size).field("apDispatch", &self.apDispatch).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for SAFEARR_DISPATCH {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for SAFEARR_DISPATCH {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.apDispatch == other.apDispatch
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for SAFEARR_DISPATCH {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for SAFEARR_DISPATCH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub struct SAFEARR_HAVEIID {
    pub Size: u32,
    pub apUnknown: *mut ::core::option::Option<::windows::core::IUnknown>,
    pub iid: ::windows::core::GUID,
}
impl ::core::marker::Copy for SAFEARR_HAVEIID {}
impl ::core::clone::Clone for SAFEARR_HAVEIID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SAFEARR_HAVEIID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SAFEARR_HAVEIID").field("Size", &self.Size).field("apUnknown", &self.apUnknown).field("iid", &self.iid).finish()
    }
}
unsafe impl ::windows::core::Abi for SAFEARR_HAVEIID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SAFEARR_HAVEIID {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.apUnknown == other.apUnknown && self.iid == other.iid
    }
}
impl ::core::cmp::Eq for SAFEARR_HAVEIID {}
impl ::core::default::Default for SAFEARR_HAVEIID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub struct SAFEARR_UNKNOWN {
    pub Size: u32,
    pub apUnknown: *mut ::core::option::Option<::windows::core::IUnknown>,
}
impl ::core::marker::Copy for SAFEARR_UNKNOWN {}
impl ::core::clone::Clone for SAFEARR_UNKNOWN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SAFEARR_UNKNOWN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SAFEARR_UNKNOWN").field("Size", &self.Size).field("apUnknown", &self.apUnknown).finish()
    }
}
unsafe impl ::windows::core::Abi for SAFEARR_UNKNOWN {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SAFEARR_UNKNOWN {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.apUnknown == other.apUnknown
    }
}
impl ::core::cmp::Eq for SAFEARR_UNKNOWN {}
impl ::core::default::Default for SAFEARR_UNKNOWN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct SAFEARR_VARIANT {
    pub Size: u32,
    pub aVariant: *mut *mut _wireVARIANT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for SAFEARR_VARIANT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for SAFEARR_VARIANT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for SAFEARR_VARIANT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SAFEARR_VARIANT").field("Size", &self.Size).field("aVariant", &self.aVariant).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for SAFEARR_VARIANT {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for SAFEARR_VARIANT {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.aVariant == other.aVariant
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for SAFEARR_VARIANT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for SAFEARR_VARIANT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct UDATE {
    pub st: super::super::Foundation::SYSTEMTIME,
    pub wDayOfYear: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for UDATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for UDATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for UDATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UDATE").field("st", &self.st).field("wDayOfYear", &self.wDayOfYear).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for UDATE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for UDATE {
    fn eq(&self, other: &Self) -> bool {
        self.st == other.st && self.wDayOfYear == other.wDayOfYear
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for UDATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for UDATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`*"]
pub struct _wireBRECORD {
    pub fFlags: u32,
    pub clSize: u32,
    pub pRecInfo: ::windows::core::ManuallyDrop<IRecordInfo>,
    pub pRecord: *mut u8,
}
impl ::core::clone::Clone for _wireBRECORD {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::core::fmt::Debug for _wireBRECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("_wireBRECORD").field("fFlags", &self.fFlags).field("clSize", &self.clSize).field("pRecInfo", &self.pRecInfo).field("pRecord", &self.pRecord).finish()
    }
}
unsafe impl ::windows::core::Abi for _wireBRECORD {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for _wireBRECORD {
    fn eq(&self, other: &Self) -> bool {
        self.fFlags == other.fFlags && self.clSize == other.clSize && self.pRecInfo == other.pRecInfo && self.pRecord == other.pRecord
    }
}
impl ::core::cmp::Eq for _wireBRECORD {}
impl ::core::default::Default for _wireBRECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct _wireSAFEARRAY {
    pub cDims: u16,
    pub fFeatures: u16,
    pub cbElements: u32,
    pub cLocks: u32,
    pub uArrayStructs: SAFEARRAYUNION,
    pub rgsabound: [super::Com::SAFEARRAYBOUND; 1],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for _wireSAFEARRAY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for _wireSAFEARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for _wireSAFEARRAY {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for _wireSAFEARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct _wireVARIANT {
    pub clSize: u32,
    pub rpcReserved: u32,
    pub vt: u16,
    pub wReserved1: u16,
    pub wReserved2: u16,
    pub wReserved3: u16,
    pub Anonymous: _wireVARIANT_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for _wireVARIANT {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for _wireVARIANT {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for _wireVARIANT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub union _wireVARIANT_0 {
    pub llVal: i64,
    pub lVal: i32,
    pub bVal: u8,
    pub iVal: i16,
    pub fltVal: f32,
    pub dblVal: f64,
    pub boolVal: super::super::Foundation::VARIANT_BOOL,
    pub scode: i32,
    pub cyVal: super::Com::CY,
    pub date: f64,
    pub bstrVal: *mut super::Com::FLAGGED_WORD_BLOB,
    pub punkVal: ::std::mem::ManuallyDrop<::core::option::Option<::windows::core::IUnknown>>,
    pub pdispVal: ::std::mem::ManuallyDrop<::core::option::Option<super::Com::IDispatch>>,
    pub parray: *mut *mut _wireSAFEARRAY,
    pub brecVal: *mut _wireBRECORD,
    pub pbVal: *mut u8,
    pub piVal: *mut i16,
    pub plVal: *mut i32,
    pub pllVal: *mut i64,
    pub pfltVal: *mut f32,
    pub pdblVal: *mut f64,
    pub pboolVal: *mut super::super::Foundation::VARIANT_BOOL,
    pub pscode: *mut i32,
    pub pcyVal: *mut super::Com::CY,
    pub pdate: *mut f64,
    pub pbstrVal: *mut *mut super::Com::FLAGGED_WORD_BLOB,
    pub ppunkVal: *mut ::core::option::Option<::windows::core::IUnknown>,
    pub ppdispVal: *mut ::core::option::Option<super::Com::IDispatch>,
    pub pparray: *mut *mut *mut _wireSAFEARRAY,
    pub pvarVal: *mut *mut _wireVARIANT,
    pub cVal: super::super::Foundation::CHAR,
    pub uiVal: u16,
    pub ulVal: u32,
    pub ullVal: u64,
    pub intVal: i32,
    pub uintVal: u32,
    pub decVal: super::super::Foundation::DECIMAL,
    pub pdecVal: *mut super::super::Foundation::DECIMAL,
    pub pcVal: ::windows::core::PSTR,
    pub puiVal: *mut u16,
    pub pulVal: *mut u32,
    pub pullVal: *mut u64,
    pub pintVal: *mut i32,
    pub puintVal: *mut u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for _wireVARIANT_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for _wireVARIANT_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for _wireVARIANT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPFNOLEUIHOOK = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::HWND, param1: u32, param2: super::super::Foundation::WPARAM, param3: super::super::Foundation::LPARAM) -> u32>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
