#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `UI_Shell`*"]
pub struct AdaptiveCardBuilder {}
impl AdaptiveCardBuilder {
    #[doc = "*Required features: `UI_Shell`*"]
    pub fn CreateAdaptiveCardFromJson<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(value: Param0) -> ::windows::runtime::Result<IAdaptiveCard> {
        Self::IAdaptiveCardBuilderStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<IAdaptiveCard>(result__)
        })
    }
    pub fn IAdaptiveCardBuilderStatics<R, F: FnOnce(&IAdaptiveCardBuilderStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<AdaptiveCardBuilder, IAdaptiveCardBuilderStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for AdaptiveCardBuilder {
    const NAME: &'static str = "Windows.UI.Shell.AdaptiveCardBuilder";
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `UI_Shell`*"]
pub struct IAdaptiveCard(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAdaptiveCard {
    type Vtable = IAdaptiveCard_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1926256268, 41588, 16845, [130, 168, 152, 157, 64, 185, 176, 94]);
}
impl IAdaptiveCard {
    #[doc = "*Required features: `UI_Shell`*"]
    pub fn ToJson(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IAdaptiveCard {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{72d0568c-a274-41cd-82a8-989d40b9b05e}");
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveCard_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `UI_Shell`*"]
pub struct IAdaptiveCardBuilderStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAdaptiveCardBuilderStatics {
    type Vtable = IAdaptiveCardBuilderStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1986891528, 54270, 17223, [160, 188, 185, 234, 154, 109, 194, 142]);
}
impl IAdaptiveCardBuilderStatics {
    #[doc = "*Required features: `UI_Shell`*"]
    pub fn CreateAdaptiveCardFromJson<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<IAdaptiveCard> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<IAdaptiveCard>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IAdaptiveCardBuilderStatics {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{766d8f08-d3fe-4347-a0bc-b9ea9a6dc28e}");
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveCardBuilderStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISecurityAppManager(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISecurityAppManager {
    type Vtable = ISecurityAppManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2527875084, 44756, 22045, [189, 232, 149, 53, 32, 52, 58, 45]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecurityAppManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, kind: SecurityAppKind, displayname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, detailsuri: ::windows::runtime::RawPtr, registerperuser: bool, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, kind: SecurityAppKind, guidregistration: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, kind: SecurityAppKind, guidregistration: ::windows::runtime::GUID, state: SecurityAppState, substatus: SecurityAppSubstatus, detailsuri: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IShareWindowCommandEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IShareWindowCommandEventArgs {
    type Vtable = IShareWindowCommandEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1165548553, 42275, 22358, [169, 149, 228, 254, 185, 145, 255, 240]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShareWindowCommandEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::WindowId) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ShareWindowCommand) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ShareWindowCommand) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IShareWindowCommandSource(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IShareWindowCommandSource {
    type Vtable = IShareWindowCommandSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3409672931, 27548, 22046, [188, 204, 97, 230, 142, 10, 191, 239]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShareWindowCommandSource_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IShareWindowCommandSourceStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IShareWindowCommandSourceStatics {
    type Vtable = IShareWindowCommandSourceStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2968217174, 40108, 20860, [182, 199, 142, 247, 21, 8, 66, 149]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShareWindowCommandSourceStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITaskbarManager(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITaskbarManager {
    type Vtable = ITaskbarManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2269710873, 6873, 18932, [178, 232, 134, 115, 141, 197, 172, 64]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITaskbarManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, applistentry: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Core", feature = "Foundation")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, applistentry: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Core", feature = "Foundation")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITaskbarManager2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITaskbarManager2 {
    type Vtable = ITaskbarManager2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2045812846, 31490, 18705, [145, 140, 222, 224, 187, 210, 11, 164]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITaskbarManager2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, tileid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_StartScreen"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, secondarytile: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_StartScreen")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, tileid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITaskbarManagerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITaskbarManagerStatics {
    type Vtable = ITaskbarManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3677530996, 56914, 20454, [183, 182, 149, 255, 159, 131, 149, 223]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITaskbarManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `UI_Shell`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SecurityAppKind(pub i32);
impl SecurityAppKind {
    pub const WebProtection: SecurityAppKind = SecurityAppKind(0i32);
}
impl ::std::convert::From<i32> for SecurityAppKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SecurityAppKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SecurityAppKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Shell.SecurityAppKind;i4)");
}
impl ::windows::runtime::DefaultType for SecurityAppKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Shell`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct SecurityAppManager(::windows::runtime::IInspectable);
impl SecurityAppManager {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SecurityAppManager, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Shell`, `Foundation`*"]
    pub fn Register<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, kind: SecurityAppKind, displayname: Param1, detailsuri: Param2, registerperuser: bool) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), kind, displayname.into_param().abi(), detailsuri.into_param().abi(), registerperuser, &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `UI_Shell`*"]
    pub fn Unregister<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, kind: SecurityAppKind, guidregistration: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), kind, guidregistration.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Shell`, `Foundation`*"]
    pub fn UpdateState<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, kind: SecurityAppKind, guidregistration: Param1, state: SecurityAppState, substatus: SecurityAppSubstatus, detailsuri: Param4) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), kind, guidregistration.into_param().abi(), state, substatus, detailsuri.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SecurityAppManager {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Shell.SecurityAppManager;{96ac500c-aed4-561d-bde8-953520343a2d})");
}
unsafe impl ::windows::runtime::Interface for SecurityAppManager {
    type Vtable = ISecurityAppManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2527875084, 44756, 22045, [189, 232, 149, 53, 32, 52, 58, 45]);
}
impl ::windows::runtime::RuntimeName for SecurityAppManager {
    const NAME: &'static str = "Windows.UI.Shell.SecurityAppManager";
}
unsafe impl ::std::marker::Send for SecurityAppManager {}
unsafe impl ::std::marker::Sync for SecurityAppManager {}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct SecurityAppManagerContract(pub u8);
#[doc = "*Required features: `UI_Shell`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SecurityAppState(pub i32);
impl SecurityAppState {
    pub const Disabled: SecurityAppState = SecurityAppState(0i32);
    pub const Enabled: SecurityAppState = SecurityAppState(1i32);
}
impl ::std::convert::From<i32> for SecurityAppState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SecurityAppState {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SecurityAppState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Shell.SecurityAppState;i4)");
}
impl ::windows::runtime::DefaultType for SecurityAppState {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Shell`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SecurityAppSubstatus(pub i32);
impl SecurityAppSubstatus {
    pub const Undetermined: SecurityAppSubstatus = SecurityAppSubstatus(0i32);
    pub const NoActionNeeded: SecurityAppSubstatus = SecurityAppSubstatus(1i32);
    pub const ActionRecommended: SecurityAppSubstatus = SecurityAppSubstatus(2i32);
    pub const ActionNeeded: SecurityAppSubstatus = SecurityAppSubstatus(3i32);
}
impl ::std::convert::From<i32> for SecurityAppSubstatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SecurityAppSubstatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SecurityAppSubstatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Shell.SecurityAppSubstatus;i4)");
}
impl ::windows::runtime::DefaultType for SecurityAppSubstatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Shell`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ShareWindowCommand(pub i32);
impl ShareWindowCommand {
    pub const None: ShareWindowCommand = ShareWindowCommand(0i32);
    pub const StartSharing: ShareWindowCommand = ShareWindowCommand(1i32);
    pub const StopSharing: ShareWindowCommand = ShareWindowCommand(2i32);
}
impl ::std::convert::From<i32> for ShareWindowCommand {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ShareWindowCommand {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ShareWindowCommand {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Shell.ShareWindowCommand;i4)");
}
impl ::windows::runtime::DefaultType for ShareWindowCommand {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Shell`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ShareWindowCommandEventArgs(::windows::runtime::IInspectable);
impl ShareWindowCommandEventArgs {
    #[doc = "*Required features: `UI_Shell`*"]
    pub fn WindowId(&self) -> ::windows::runtime::Result<super::WindowId> {
        let this = self;
        unsafe {
            let mut result__: super::WindowId = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::WindowId>(result__)
        }
    }
    #[doc = "*Required features: `UI_Shell`*"]
    pub fn Command(&self) -> ::windows::runtime::Result<ShareWindowCommand> {
        let this = self;
        unsafe {
            let mut result__: ShareWindowCommand = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ShareWindowCommand>(result__)
        }
    }
    #[doc = "*Required features: `UI_Shell`*"]
    pub fn SetCommand(&self, value: ShareWindowCommand) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ShareWindowCommandEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Shell.ShareWindowCommandEventArgs;{4578dc09-a523-5756-a995-e4feb991fff0})");
}
unsafe impl ::windows::runtime::Interface for ShareWindowCommandEventArgs {
    type Vtable = IShareWindowCommandEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1165548553, 42275, 22358, [169, 149, 228, 254, 185, 145, 255, 240]);
}
impl ::windows::runtime::RuntimeName for ShareWindowCommandEventArgs {
    const NAME: &'static str = "Windows.UI.Shell.ShareWindowCommandEventArgs";
}
unsafe impl ::std::marker::Send for ShareWindowCommandEventArgs {}
unsafe impl ::std::marker::Sync for ShareWindowCommandEventArgs {}
#[doc = "*Required features: `UI_Shell`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ShareWindowCommandSource(::windows::runtime::IInspectable);
impl ShareWindowCommandSource {
    #[doc = "*Required features: `UI_Shell`*"]
    pub fn Start(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Shell`*"]
    pub fn Stop(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `UI_Shell`*"]
    pub fn ReportCommandChanged(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Shell`, `Foundation`*"]
    pub fn CommandRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<ShareWindowCommandSource, ShareWindowCommandEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Shell`, `Foundation`*"]
    pub fn RemoveCommandRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Shell`, `Foundation`*"]
    pub fn CommandInvoked<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<ShareWindowCommandSource, ShareWindowCommandEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Shell`, `Foundation`*"]
    pub fn RemoveCommandInvoked<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Shell`*"]
    pub fn GetForCurrentView() -> ::windows::runtime::Result<ShareWindowCommandSource> {
        Self::IShareWindowCommandSourceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ShareWindowCommandSource>(result__)
        })
    }
    pub fn IShareWindowCommandSourceStatics<R, F: FnOnce(&IShareWindowCommandSourceStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ShareWindowCommandSource, IShareWindowCommandSourceStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ShareWindowCommandSource {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Shell.ShareWindowCommandSource;{cb3b7ae3-6b9c-561e-bccc-61e68e0abfef})");
}
unsafe impl ::windows::runtime::Interface for ShareWindowCommandSource {
    type Vtable = IShareWindowCommandSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3409672931, 27548, 22046, [188, 204, 97, 230, 142, 10, 191, 239]);
}
impl ::windows::runtime::RuntimeName for ShareWindowCommandSource {
    const NAME: &'static str = "Windows.UI.Shell.ShareWindowCommandSource";
}
unsafe impl ::std::marker::Send for ShareWindowCommandSource {}
unsafe impl ::std::marker::Sync for ShareWindowCommandSource {}
#[doc = "*Required features: `UI_Shell`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct TaskbarManager(::windows::runtime::IInspectable);
impl TaskbarManager {
    #[doc = "*Required features: `UI_Shell`*"]
    pub fn IsSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Shell`*"]
    pub fn IsPinningAllowed(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Shell`, `Foundation`*"]
    pub fn IsCurrentAppPinnedAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation"))]
    #[doc = "*Required features: `UI_Shell`, `ApplicationModel_Core`, `Foundation`*"]
    pub fn IsAppListEntryPinnedAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::ApplicationModel::Core::AppListEntry>>(&self, applistentry: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), applistentry.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Shell`, `Foundation`*"]
    pub fn RequestPinCurrentAppAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation"))]
    #[doc = "*Required features: `UI_Shell`, `ApplicationModel_Core`, `Foundation`*"]
    pub fn RequestPinAppListEntryAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::ApplicationModel::Core::AppListEntry>>(&self, applistentry: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), applistentry.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `UI_Shell`*"]
    pub fn GetDefault() -> ::windows::runtime::Result<TaskbarManager> {
        Self::ITaskbarManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<TaskbarManager>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Shell`, `Foundation`*"]
    pub fn IsSecondaryTilePinnedAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, tileid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::runtime::Interface::cast::<ITaskbarManager2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), tileid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_StartScreen"))]
    #[doc = "*Required features: `UI_Shell`, `Foundation`, `UI_StartScreen`*"]
    pub fn RequestPinSecondaryTileAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::StartScreen::SecondaryTile>>(&self, secondarytile: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::runtime::Interface::cast::<ITaskbarManager2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), secondarytile.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Shell`, `Foundation`*"]
    pub fn TryUnpinSecondaryTileAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, tileid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::runtime::Interface::cast::<ITaskbarManager2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), tileid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn ITaskbarManagerStatics<R, F: FnOnce(&ITaskbarManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<TaskbarManager, ITaskbarManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TaskbarManager {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Shell.TaskbarManager;{87490a19-1ad9-49f4-b2e8-86738dc5ac40})");
}
unsafe impl ::windows::runtime::Interface for TaskbarManager {
    type Vtable = ITaskbarManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2269710873, 6873, 18932, [178, 232, 134, 115, 141, 197, 172, 64]);
}
impl ::windows::runtime::RuntimeName for TaskbarManager {
    const NAME: &'static str = "Windows.UI.Shell.TaskbarManager";
}
unsafe impl ::std::marker::Send for TaskbarManager {}
unsafe impl ::std::marker::Sync for TaskbarManager {}
