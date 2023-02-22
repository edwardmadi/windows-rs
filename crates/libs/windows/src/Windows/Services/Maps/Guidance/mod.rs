#[doc(hidden)]
#[repr(transparent)]
pub struct IGuidanceAudioNotificationRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGuidanceAudioNotificationRequestedEventArgs {
    type Vtable = IGuidanceAudioNotificationRequestedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IGuidanceAudioNotificationRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for IGuidanceAudioNotificationRequestedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca2aa24a_c7c2_4d4c_9d7c_499576bceddb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceAudioNotificationRequestedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AudioNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GuidanceAudioNotificationKind) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AudioFilePaths: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AudioFilePaths: usize,
    pub AudioText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGuidanceLaneInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGuidanceLaneInfo {
    type Vtable = IGuidanceLaneInfo_Vtbl;
}
impl ::core::clone::Clone for IGuidanceLaneInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for IGuidanceLaneInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8404d114_6581_43b7_ac15_c9079bf90df1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceLaneInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub LaneMarkers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GuidanceLaneMarkers) -> ::windows::core::HRESULT,
    pub IsOnRoute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGuidanceManeuver(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGuidanceManeuver {
    type Vtable = IGuidanceManeuver_Vtbl;
}
impl ::core::clone::Clone for IGuidanceManeuver {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for IGuidanceManeuver {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfc09326c_ecc9_4928_a2a1_7232b99b94a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceManeuver_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Geolocation")]
    pub StartLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    StartLocation: usize,
    pub DistanceFromRouteStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub DistanceFromPreviousManeuver: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub DepartureRoadName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub NextRoadName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DepartureShortRoadName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub NextShortRoadName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GuidanceManeuverKind) -> ::windows::core::HRESULT,
    pub StartAngle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub EndAngle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub RoadSignpost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub InstructionText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGuidanceMapMatchedCoordinate(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGuidanceMapMatchedCoordinate {
    type Vtable = IGuidanceMapMatchedCoordinate_Vtbl;
}
impl ::core::clone::Clone for IGuidanceMapMatchedCoordinate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for IGuidanceMapMatchedCoordinate {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb7acb168_2912_4a99_aff1_798609b981fe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceMapMatchedCoordinate_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Geolocation")]
    pub Location: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Location: usize,
    pub CurrentHeading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub CurrentSpeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub IsOnStreet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Road: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGuidanceNavigator(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGuidanceNavigator {
    type Vtable = IGuidanceNavigator_Vtbl;
}
impl ::core::clone::Clone for IGuidanceNavigator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for IGuidanceNavigator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08f17ef7_8e3f_4d9a_be8a_108f9a012c67);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceNavigator_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub StartNavigating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, route: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub StartSimulating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, route: *mut ::core::ffi::c_void, speedinmeterspersecond: i32) -> ::windows::core::HRESULT,
    pub StartTracking: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RepeatLastAudioNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AudioMeasurementSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GuidanceAudioMeasurementSystem) -> ::windows::core::HRESULT,
    pub SetAudioMeasurementSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: GuidanceAudioMeasurementSystem) -> ::windows::core::HRESULT,
    pub AudioNotifications: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GuidanceAudioNotifications) -> ::windows::core::HRESULT,
    pub SetAudioNotifications: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: GuidanceAudioNotifications) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GuidanceUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GuidanceUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveGuidanceUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveGuidanceUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub DestinationReached: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DestinationReached: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDestinationReached: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDestinationReached: usize,
    #[cfg(feature = "Foundation")]
    pub Rerouting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Rerouting: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRerouting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRerouting: usize,
    #[cfg(feature = "Foundation")]
    pub Rerouted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Rerouted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRerouted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRerouted: usize,
    #[cfg(feature = "Foundation")]
    pub RerouteFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RerouteFailed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRerouteFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRerouteFailed: usize,
    #[cfg(feature = "Foundation")]
    pub UserLocationLost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UserLocationLost: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUserLocationLost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUserLocationLost: usize,
    #[cfg(feature = "Foundation")]
    pub UserLocationRestored: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UserLocationRestored: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUserLocationRestored: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUserLocationRestored: usize,
    pub SetGuidanceVoice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, voiceid: i32, voicefolder: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Geolocation")]
    pub UpdateUserLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, userlocation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    UpdateUserLocation: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub UpdateUserLocationWithPositionOverride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, userlocation: *mut ::core::ffi::c_void, positionoverride: super::super::super::Devices::Geolocation::BasicGeoposition) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    UpdateUserLocationWithPositionOverride: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGuidanceNavigator2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGuidanceNavigator2 {
    type Vtable = IGuidanceNavigator2_Vtbl;
}
impl ::core::clone::Clone for IGuidanceNavigator2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for IGuidanceNavigator2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6cdc50d1_041c_4bf3_b633_a101fc2f6b57);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceNavigator2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub AudioNotificationRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AudioNotificationRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAudioNotificationRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAudioNotificationRequested: usize,
    pub IsGuidanceAudioMuted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsGuidanceAudioMuted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGuidanceNavigatorStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGuidanceNavigatorStatics {
    type Vtable = IGuidanceNavigatorStatics_Vtbl;
}
impl ::core::clone::Clone for IGuidanceNavigatorStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for IGuidanceNavigatorStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00fd9513_4456_4e66_a143_3add6be08426);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceNavigatorStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGuidanceNavigatorStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGuidanceNavigatorStatics2 {
    type Vtable = IGuidanceNavigatorStatics2_Vtbl;
}
impl ::core::clone::Clone for IGuidanceNavigatorStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for IGuidanceNavigatorStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x54c5c3e2_7784_4c85_8c95_d0c6efb43965);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceNavigatorStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub UseAppProvidedVoice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGuidanceReroutedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGuidanceReroutedEventArgs {
    type Vtable = IGuidanceReroutedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IGuidanceReroutedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for IGuidanceReroutedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x115d4008_d528_454e_bb94_a50341d2c9f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceReroutedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Route: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGuidanceRoadSegment(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGuidanceRoadSegment {
    type Vtable = IGuidanceRoadSegment_Vtbl;
}
impl ::core::clone::Clone for IGuidanceRoadSegment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for IGuidanceRoadSegment {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb32758a6_be78_4c63_afe7_6c2957479b3e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceRoadSegment_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub RoadName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ShortRoadName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SpeedLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TravelTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TravelTime: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Path: usize,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IsHighway: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsTunnel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsTollRoad: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGuidanceRoadSegment2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGuidanceRoadSegment2 {
    type Vtable = IGuidanceRoadSegment2_Vtbl;
}
impl ::core::clone::Clone for IGuidanceRoadSegment2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for IGuidanceRoadSegment2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2474a61d_1723_49f1_895b_47a2c4aa9c55);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceRoadSegment2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsScenic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGuidanceRoadSignpost(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGuidanceRoadSignpost {
    type Vtable = IGuidanceRoadSignpost_Vtbl;
}
impl ::core::clone::Clone for IGuidanceRoadSignpost {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for IGuidanceRoadSignpost {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf1a728b6_f77a_4742_8312_53300f9845f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceRoadSignpost_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ExitNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Exit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI")]
    pub BackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    BackgroundColor: usize,
    #[cfg(feature = "UI")]
    pub ForegroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    ForegroundColor: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ExitDirections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ExitDirections: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGuidanceRoute(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGuidanceRoute {
    type Vtable = IGuidanceRoute_Vtbl;
}
impl ::core::clone::Clone for IGuidanceRoute {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for IGuidanceRoute {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3a14545d_801a_40bd_a286_afb2010cce6c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceRoute_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    pub Distance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Maneuvers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Maneuvers: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub BoundingBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    BoundingBox: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Path: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RoadSegments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RoadSegments: usize,
    pub ConvertToMapRoute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGuidanceRouteStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGuidanceRouteStatics {
    type Vtable = IGuidanceRouteStatics_Vtbl;
}
impl ::core::clone::Clone for IGuidanceRouteStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for IGuidanceRouteStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf56d926a_55ed_49c1_b09c_4b8223b50db3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceRouteStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CanCreateFromMapRoute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maproute: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub TryCreateFromMapRoute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maproute: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGuidanceTelemetryCollector(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGuidanceTelemetryCollector {
    type Vtable = IGuidanceTelemetryCollector_Vtbl;
}
impl ::core::clone::Clone for IGuidanceTelemetryCollector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for IGuidanceTelemetryCollector {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb1f8da5_b878_4d92_98dd_347d23d38262);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceTelemetryCollector_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub ClearLocalData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SpeedTrigger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetSpeedTrigger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub UploadFrequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetUploadFrequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGuidanceTelemetryCollectorStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGuidanceTelemetryCollectorStatics {
    type Vtable = IGuidanceTelemetryCollectorStatics_Vtbl;
}
impl ::core::clone::Clone for IGuidanceTelemetryCollectorStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for IGuidanceTelemetryCollectorStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x36532047_f160_44fb_b578_94577ca05990);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceTelemetryCollectorStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGuidanceUpdatedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGuidanceUpdatedEventArgs {
    type Vtable = IGuidanceUpdatedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IGuidanceUpdatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for IGuidanceUpdatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfdac160b_9e8d_4de3_a9fa_b06321d18db9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidanceUpdatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Mode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GuidanceMode) -> ::windows::core::HRESULT,
    pub NextManeuver: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub NextManeuverDistance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub AfterNextManeuver: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AfterNextManeuverDistance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub DistanceToDestination: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub ElapsedDistance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ElapsedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ElapsedTime: usize,
    #[cfg(feature = "Foundation")]
    pub TimeToDestination: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TimeToDestination: usize,
    pub RoadName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Route: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CurrentLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsNewManeuver: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub LaneInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LaneInfo: usize,
}
#[doc = "*Required features: `\"Services_Maps_Guidance\"`*"]
#[repr(transparent)]
pub struct GuidanceAudioNotificationRequestedEventArgs(::windows::core::IUnknown);
impl GuidanceAudioNotificationRequestedEventArgs {
    pub fn AudioNotification(&self) -> ::windows::core::Result<GuidanceAudioNotificationKind> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GuidanceAudioNotificationKind>();
            (::windows::core::Vtable::vtable(this).AudioNotification)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AudioFilePaths(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>();
            (::windows::core::Vtable::vtable(this).AudioFilePaths)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AudioText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Vtable::vtable(this).AudioText)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GuidanceAudioNotificationRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GuidanceAudioNotificationRequestedEventArgs {}
impl ::core::fmt::Debug for GuidanceAudioNotificationRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GuidanceAudioNotificationRequestedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GuidanceAudioNotificationRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.Guidance.GuidanceAudioNotificationRequestedEventArgs;{ca2aa24a-c7c2-4d4c-9d7c-499576bceddb})");
}
impl ::core::clone::Clone for GuidanceAudioNotificationRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for GuidanceAudioNotificationRequestedEventArgs {
    type Vtable = IGuidanceAudioNotificationRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for GuidanceAudioNotificationRequestedEventArgs {
    const IID: ::windows::core::GUID = <IGuidanceAudioNotificationRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GuidanceAudioNotificationRequestedEventArgs {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.GuidanceAudioNotificationRequestedEventArgs";
}
::windows::core::interface_hierarchy!(GuidanceAudioNotificationRequestedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GuidanceAudioNotificationRequestedEventArgs {}
unsafe impl ::core::marker::Sync for GuidanceAudioNotificationRequestedEventArgs {}
#[doc = "*Required features: `\"Services_Maps_Guidance\"`*"]
#[repr(transparent)]
pub struct GuidanceLaneInfo(::windows::core::IUnknown);
impl GuidanceLaneInfo {
    pub fn LaneMarkers(&self) -> ::windows::core::Result<GuidanceLaneMarkers> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GuidanceLaneMarkers>();
            (::windows::core::Vtable::vtable(this).LaneMarkers)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsOnRoute(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Vtable::vtable(this).IsOnRoute)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GuidanceLaneInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GuidanceLaneInfo {}
impl ::core::fmt::Debug for GuidanceLaneInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GuidanceLaneInfo").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GuidanceLaneInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.Guidance.GuidanceLaneInfo;{8404d114-6581-43b7-ac15-c9079bf90df1})");
}
impl ::core::clone::Clone for GuidanceLaneInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for GuidanceLaneInfo {
    type Vtable = IGuidanceLaneInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for GuidanceLaneInfo {
    const IID: ::windows::core::GUID = <IGuidanceLaneInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GuidanceLaneInfo {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.GuidanceLaneInfo";
}
::windows::core::interface_hierarchy!(GuidanceLaneInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GuidanceLaneInfo {}
unsafe impl ::core::marker::Sync for GuidanceLaneInfo {}
#[doc = "*Required features: `\"Services_Maps_Guidance\"`*"]
#[repr(transparent)]
pub struct GuidanceManeuver(::windows::core::IUnknown);
impl GuidanceManeuver {
    #[doc = "*Required features: `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn StartLocation(&self) -> ::windows::core::Result<super::super::super::Devices::Geolocation::Geopoint> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Devices::Geolocation::Geopoint>();
            (::windows::core::Vtable::vtable(this).StartLocation)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DistanceFromRouteStart(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Vtable::vtable(this).DistanceFromRouteStart)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DistanceFromPreviousManeuver(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Vtable::vtable(this).DistanceFromPreviousManeuver)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DepartureRoadName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Vtable::vtable(this).DepartureRoadName)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NextRoadName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Vtable::vtable(this).NextRoadName)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DepartureShortRoadName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Vtable::vtable(this).DepartureShortRoadName)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NextShortRoadName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Vtable::vtable(this).NextShortRoadName)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<GuidanceManeuverKind> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GuidanceManeuverKind>();
            (::windows::core::Vtable::vtable(this).Kind)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StartAngle(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Vtable::vtable(this).StartAngle)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn EndAngle(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Vtable::vtable(this).EndAngle)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RoadSignpost(&self) -> ::windows::core::Result<GuidanceRoadSignpost> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GuidanceRoadSignpost>();
            (::windows::core::Vtable::vtable(this).RoadSignpost)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InstructionText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Vtable::vtable(this).InstructionText)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GuidanceManeuver {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GuidanceManeuver {}
impl ::core::fmt::Debug for GuidanceManeuver {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GuidanceManeuver").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GuidanceManeuver {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.Guidance.GuidanceManeuver;{fc09326c-ecc9-4928-a2a1-7232b99b94a1})");
}
impl ::core::clone::Clone for GuidanceManeuver {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for GuidanceManeuver {
    type Vtable = IGuidanceManeuver_Vtbl;
}
unsafe impl ::windows::core::Interface for GuidanceManeuver {
    const IID: ::windows::core::GUID = <IGuidanceManeuver as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GuidanceManeuver {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.GuidanceManeuver";
}
::windows::core::interface_hierarchy!(GuidanceManeuver, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GuidanceManeuver {}
unsafe impl ::core::marker::Sync for GuidanceManeuver {}
#[doc = "*Required features: `\"Services_Maps_Guidance\"`*"]
#[repr(transparent)]
pub struct GuidanceMapMatchedCoordinate(::windows::core::IUnknown);
impl GuidanceMapMatchedCoordinate {
    #[doc = "*Required features: `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn Location(&self) -> ::windows::core::Result<super::super::super::Devices::Geolocation::Geopoint> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Devices::Geolocation::Geopoint>();
            (::windows::core::Vtable::vtable(this).Location)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CurrentHeading(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Vtable::vtable(this).CurrentHeading)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CurrentSpeed(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Vtable::vtable(this).CurrentSpeed)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsOnStreet(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Vtable::vtable(this).IsOnStreet)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Road(&self) -> ::windows::core::Result<GuidanceRoadSegment> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GuidanceRoadSegment>();
            (::windows::core::Vtable::vtable(this).Road)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GuidanceMapMatchedCoordinate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GuidanceMapMatchedCoordinate {}
impl ::core::fmt::Debug for GuidanceMapMatchedCoordinate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GuidanceMapMatchedCoordinate").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GuidanceMapMatchedCoordinate {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.Guidance.GuidanceMapMatchedCoordinate;{b7acb168-2912-4a99-aff1-798609b981fe})");
}
impl ::core::clone::Clone for GuidanceMapMatchedCoordinate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for GuidanceMapMatchedCoordinate {
    type Vtable = IGuidanceMapMatchedCoordinate_Vtbl;
}
unsafe impl ::windows::core::Interface for GuidanceMapMatchedCoordinate {
    const IID: ::windows::core::GUID = <IGuidanceMapMatchedCoordinate as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GuidanceMapMatchedCoordinate {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.GuidanceMapMatchedCoordinate";
}
::windows::core::interface_hierarchy!(GuidanceMapMatchedCoordinate, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GuidanceMapMatchedCoordinate {}
unsafe impl ::core::marker::Sync for GuidanceMapMatchedCoordinate {}
#[doc = "*Required features: `\"Services_Maps_Guidance\"`*"]
#[repr(transparent)]
pub struct GuidanceNavigator(::windows::core::IUnknown);
impl GuidanceNavigator {
    pub fn StartNavigating(&self, route: &GuidanceRoute) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).StartNavigating)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(route)).ok() }
    }
    pub fn StartSimulating(&self, route: &GuidanceRoute, speedinmeterspersecond: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).StartSimulating)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(route), speedinmeterspersecond).ok() }
    }
    pub fn StartTracking(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).StartTracking)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Pause(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Pause)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Resume(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Resume)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Stop)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn RepeatLastAudioNotification(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RepeatLastAudioNotification)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn AudioMeasurementSystem(&self) -> ::windows::core::Result<GuidanceAudioMeasurementSystem> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GuidanceAudioMeasurementSystem>();
            (::windows::core::Vtable::vtable(this).AudioMeasurementSystem)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAudioMeasurementSystem(&self, value: GuidanceAudioMeasurementSystem) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetAudioMeasurementSystem)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn AudioNotifications(&self) -> ::windows::core::Result<GuidanceAudioNotifications> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GuidanceAudioNotifications>();
            (::windows::core::Vtable::vtable(this).AudioNotifications)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAudioNotifications(&self, value: GuidanceAudioNotifications) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetAudioNotifications)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GuidanceUpdated(&self, handler: &super::super::super::Foundation::TypedEventHandler<GuidanceNavigator, GuidanceUpdatedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Vtable::vtable(this).GuidanceUpdated)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveGuidanceUpdated(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveGuidanceUpdated)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DestinationReached(&self, handler: &super::super::super::Foundation::TypedEventHandler<GuidanceNavigator, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Vtable::vtable(this).DestinationReached)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDestinationReached(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveDestinationReached)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Rerouting(&self, handler: &super::super::super::Foundation::TypedEventHandler<GuidanceNavigator, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Vtable::vtable(this).Rerouting)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRerouting(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveRerouting)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Rerouted(&self, handler: &super::super::super::Foundation::TypedEventHandler<GuidanceNavigator, GuidanceReroutedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Vtable::vtable(this).Rerouted)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRerouted(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveRerouted)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RerouteFailed(&self, handler: &super::super::super::Foundation::TypedEventHandler<GuidanceNavigator, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Vtable::vtable(this).RerouteFailed)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRerouteFailed(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveRerouteFailed)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UserLocationLost(&self, handler: &super::super::super::Foundation::TypedEventHandler<GuidanceNavigator, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Vtable::vtable(this).UserLocationLost)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUserLocationLost(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveUserLocationLost)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UserLocationRestored(&self, handler: &super::super::super::Foundation::TypedEventHandler<GuidanceNavigator, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Vtable::vtable(this).UserLocationRestored)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUserLocationRestored(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveUserLocationRestored)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    pub fn SetGuidanceVoice(&self, voiceid: i32, voicefolder: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetGuidanceVoice)(::windows::core::Vtable::as_raw(this), voiceid, ::core::mem::transmute_copy(voicefolder)).ok() }
    }
    #[doc = "*Required features: `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn UpdateUserLocation(&self, userlocation: &super::super::super::Devices::Geolocation::Geocoordinate) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).UpdateUserLocation)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(userlocation)).ok() }
    }
    #[doc = "*Required features: `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn UpdateUserLocationWithPositionOverride(&self, userlocation: &super::super::super::Devices::Geolocation::Geocoordinate, positionoverride: super::super::super::Devices::Geolocation::BasicGeoposition) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).UpdateUserLocationWithPositionOverride)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(userlocation), positionoverride).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AudioNotificationRequested(&self, value: &super::super::super::Foundation::TypedEventHandler<GuidanceNavigator, GuidanceAudioNotificationRequestedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IGuidanceNavigator2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Vtable::vtable(this).AudioNotificationRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAudioNotificationRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGuidanceNavigator2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveAudioNotificationRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    pub fn IsGuidanceAudioMuted(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IGuidanceNavigator2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Vtable::vtable(this).IsGuidanceAudioMuted)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsGuidanceAudioMuted(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGuidanceNavigator2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsGuidanceAudioMuted)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn GetCurrent() -> ::windows::core::Result<GuidanceNavigator> {
        Self::IGuidanceNavigatorStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<GuidanceNavigator>();
            (::windows::core::Vtable::vtable(this).GetCurrent)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn UseAppProvidedVoice() -> ::windows::core::Result<bool> {
        Self::IGuidanceNavigatorStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Vtable::vtable(this).UseAppProvidedVoice)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGuidanceNavigatorStatics<R, F: FnOnce(&IGuidanceNavigatorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<GuidanceNavigator, IGuidanceNavigatorStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IGuidanceNavigatorStatics2<R, F: FnOnce(&IGuidanceNavigatorStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<GuidanceNavigator, IGuidanceNavigatorStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for GuidanceNavigator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GuidanceNavigator {}
impl ::core::fmt::Debug for GuidanceNavigator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GuidanceNavigator").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GuidanceNavigator {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.Guidance.GuidanceNavigator;{08f17ef7-8e3f-4d9a-be8a-108f9a012c67})");
}
impl ::core::clone::Clone for GuidanceNavigator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for GuidanceNavigator {
    type Vtable = IGuidanceNavigator_Vtbl;
}
unsafe impl ::windows::core::Interface for GuidanceNavigator {
    const IID: ::windows::core::GUID = <IGuidanceNavigator as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GuidanceNavigator {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.GuidanceNavigator";
}
::windows::core::interface_hierarchy!(GuidanceNavigator, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GuidanceNavigator {}
unsafe impl ::core::marker::Sync for GuidanceNavigator {}
#[doc = "*Required features: `\"Services_Maps_Guidance\"`*"]
#[repr(transparent)]
pub struct GuidanceReroutedEventArgs(::windows::core::IUnknown);
impl GuidanceReroutedEventArgs {
    pub fn Route(&self) -> ::windows::core::Result<GuidanceRoute> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GuidanceRoute>();
            (::windows::core::Vtable::vtable(this).Route)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GuidanceReroutedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GuidanceReroutedEventArgs {}
impl ::core::fmt::Debug for GuidanceReroutedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GuidanceReroutedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GuidanceReroutedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.Guidance.GuidanceReroutedEventArgs;{115d4008-d528-454e-bb94-a50341d2c9f1})");
}
impl ::core::clone::Clone for GuidanceReroutedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for GuidanceReroutedEventArgs {
    type Vtable = IGuidanceReroutedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for GuidanceReroutedEventArgs {
    const IID: ::windows::core::GUID = <IGuidanceReroutedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GuidanceReroutedEventArgs {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.GuidanceReroutedEventArgs";
}
::windows::core::interface_hierarchy!(GuidanceReroutedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GuidanceReroutedEventArgs {}
unsafe impl ::core::marker::Sync for GuidanceReroutedEventArgs {}
#[doc = "*Required features: `\"Services_Maps_Guidance\"`*"]
#[repr(transparent)]
pub struct GuidanceRoadSegment(::windows::core::IUnknown);
impl GuidanceRoadSegment {
    pub fn RoadName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Vtable::vtable(this).RoadName)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ShortRoadName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Vtable::vtable(this).ShortRoadName)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SpeedLimit(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Vtable::vtable(this).SpeedLimit)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TravelTime(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::TimeSpan>();
            (::windows::core::Vtable::vtable(this).TravelTime)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn Path(&self) -> ::windows::core::Result<super::super::super::Devices::Geolocation::Geopath> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Devices::Geolocation::Geopath>();
            (::windows::core::Vtable::vtable(this).Path)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsHighway(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Vtable::vtable(this).IsHighway)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsTunnel(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Vtable::vtable(this).IsTunnel)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsTollRoad(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Vtable::vtable(this).IsTollRoad)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsScenic(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IGuidanceRoadSegment2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Vtable::vtable(this).IsScenic)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GuidanceRoadSegment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GuidanceRoadSegment {}
impl ::core::fmt::Debug for GuidanceRoadSegment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GuidanceRoadSegment").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GuidanceRoadSegment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.Guidance.GuidanceRoadSegment;{b32758a6-be78-4c63-afe7-6c2957479b3e})");
}
impl ::core::clone::Clone for GuidanceRoadSegment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for GuidanceRoadSegment {
    type Vtable = IGuidanceRoadSegment_Vtbl;
}
unsafe impl ::windows::core::Interface for GuidanceRoadSegment {
    const IID: ::windows::core::GUID = <IGuidanceRoadSegment as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GuidanceRoadSegment {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.GuidanceRoadSegment";
}
::windows::core::interface_hierarchy!(GuidanceRoadSegment, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GuidanceRoadSegment {}
unsafe impl ::core::marker::Sync for GuidanceRoadSegment {}
#[doc = "*Required features: `\"Services_Maps_Guidance\"`*"]
#[repr(transparent)]
pub struct GuidanceRoadSignpost(::windows::core::IUnknown);
impl GuidanceRoadSignpost {
    pub fn ExitNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Vtable::vtable(this).ExitNumber)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Exit(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Vtable::vtable(this).Exit)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn BackgroundColor(&self) -> ::windows::core::Result<super::super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::UI::Color>();
            (::windows::core::Vtable::vtable(this).BackgroundColor)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn ForegroundColor(&self) -> ::windows::core::Result<super::super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::UI::Color>();
            (::windows::core::Vtable::vtable(this).ForegroundColor)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ExitDirections(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>();
            (::windows::core::Vtable::vtable(this).ExitDirections)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GuidanceRoadSignpost {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GuidanceRoadSignpost {}
impl ::core::fmt::Debug for GuidanceRoadSignpost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GuidanceRoadSignpost").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GuidanceRoadSignpost {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.Guidance.GuidanceRoadSignpost;{f1a728b6-f77a-4742-8312-53300f9845f0})");
}
impl ::core::clone::Clone for GuidanceRoadSignpost {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for GuidanceRoadSignpost {
    type Vtable = IGuidanceRoadSignpost_Vtbl;
}
unsafe impl ::windows::core::Interface for GuidanceRoadSignpost {
    const IID: ::windows::core::GUID = <IGuidanceRoadSignpost as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GuidanceRoadSignpost {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.GuidanceRoadSignpost";
}
::windows::core::interface_hierarchy!(GuidanceRoadSignpost, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GuidanceRoadSignpost {}
unsafe impl ::core::marker::Sync for GuidanceRoadSignpost {}
#[doc = "*Required features: `\"Services_Maps_Guidance\"`*"]
#[repr(transparent)]
pub struct GuidanceRoute(::windows::core::IUnknown);
impl GuidanceRoute {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::TimeSpan>();
            (::windows::core::Vtable::vtable(this).Duration)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Distance(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Vtable::vtable(this).Distance)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Maneuvers(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GuidanceManeuver>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Collections::IVectorView<GuidanceManeuver>>();
            (::windows::core::Vtable::vtable(this).Maneuvers)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn BoundingBox(&self) -> ::windows::core::Result<super::super::super::Devices::Geolocation::GeoboundingBox> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Devices::Geolocation::GeoboundingBox>();
            (::windows::core::Vtable::vtable(this).BoundingBox)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn Path(&self) -> ::windows::core::Result<super::super::super::Devices::Geolocation::Geopath> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Devices::Geolocation::Geopath>();
            (::windows::core::Vtable::vtable(this).Path)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RoadSegments(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GuidanceRoadSegment>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Collections::IVectorView<GuidanceRoadSegment>>();
            (::windows::core::Vtable::vtable(this).RoadSegments)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ConvertToMapRoute(&self) -> ::windows::core::Result<super::MapRoute> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::MapRoute>();
            (::windows::core::Vtable::vtable(this).ConvertToMapRoute)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CanCreateFromMapRoute(maproute: &super::MapRoute) -> ::windows::core::Result<bool> {
        Self::IGuidanceRouteStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Vtable::vtable(this).CanCreateFromMapRoute)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(maproute), &mut result__).from_abi(result__)
        })
    }
    pub fn TryCreateFromMapRoute(maproute: &super::MapRoute) -> ::windows::core::Result<GuidanceRoute> {
        Self::IGuidanceRouteStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<GuidanceRoute>();
            (::windows::core::Vtable::vtable(this).TryCreateFromMapRoute)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(maproute), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGuidanceRouteStatics<R, F: FnOnce(&IGuidanceRouteStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<GuidanceRoute, IGuidanceRouteStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for GuidanceRoute {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GuidanceRoute {}
impl ::core::fmt::Debug for GuidanceRoute {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GuidanceRoute").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GuidanceRoute {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.Guidance.GuidanceRoute;{3a14545d-801a-40bd-a286-afb2010cce6c})");
}
impl ::core::clone::Clone for GuidanceRoute {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for GuidanceRoute {
    type Vtable = IGuidanceRoute_Vtbl;
}
unsafe impl ::windows::core::Interface for GuidanceRoute {
    const IID: ::windows::core::GUID = <IGuidanceRoute as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GuidanceRoute {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.GuidanceRoute";
}
::windows::core::interface_hierarchy!(GuidanceRoute, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GuidanceRoute {}
unsafe impl ::core::marker::Sync for GuidanceRoute {}
#[doc = "*Required features: `\"Services_Maps_Guidance\"`*"]
#[repr(transparent)]
pub struct GuidanceTelemetryCollector(::windows::core::IUnknown);
impl GuidanceTelemetryCollector {
    pub fn Enabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Vtable::vtable(this).Enabled)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetEnabled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn ClearLocalData(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).ClearLocalData)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn SpeedTrigger(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Vtable::vtable(this).SpeedTrigger)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSpeedTrigger(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSpeedTrigger)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn UploadFrequency(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Vtable::vtable(this).UploadFrequency)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetUploadFrequency(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetUploadFrequency)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn GetCurrent() -> ::windows::core::Result<GuidanceTelemetryCollector> {
        Self::IGuidanceTelemetryCollectorStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<GuidanceTelemetryCollector>();
            (::windows::core::Vtable::vtable(this).GetCurrent)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGuidanceTelemetryCollectorStatics<R, F: FnOnce(&IGuidanceTelemetryCollectorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<GuidanceTelemetryCollector, IGuidanceTelemetryCollectorStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for GuidanceTelemetryCollector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GuidanceTelemetryCollector {}
impl ::core::fmt::Debug for GuidanceTelemetryCollector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GuidanceTelemetryCollector").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GuidanceTelemetryCollector {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.Guidance.GuidanceTelemetryCollector;{db1f8da5-b878-4d92-98dd-347d23d38262})");
}
impl ::core::clone::Clone for GuidanceTelemetryCollector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for GuidanceTelemetryCollector {
    type Vtable = IGuidanceTelemetryCollector_Vtbl;
}
unsafe impl ::windows::core::Interface for GuidanceTelemetryCollector {
    const IID: ::windows::core::GUID = <IGuidanceTelemetryCollector as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GuidanceTelemetryCollector {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.GuidanceTelemetryCollector";
}
::windows::core::interface_hierarchy!(GuidanceTelemetryCollector, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GuidanceTelemetryCollector {}
unsafe impl ::core::marker::Sync for GuidanceTelemetryCollector {}
#[doc = "*Required features: `\"Services_Maps_Guidance\"`*"]
#[repr(transparent)]
pub struct GuidanceUpdatedEventArgs(::windows::core::IUnknown);
impl GuidanceUpdatedEventArgs {
    pub fn Mode(&self) -> ::windows::core::Result<GuidanceMode> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GuidanceMode>();
            (::windows::core::Vtable::vtable(this).Mode)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NextManeuver(&self) -> ::windows::core::Result<GuidanceManeuver> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GuidanceManeuver>();
            (::windows::core::Vtable::vtable(this).NextManeuver)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NextManeuverDistance(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Vtable::vtable(this).NextManeuverDistance)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AfterNextManeuver(&self) -> ::windows::core::Result<GuidanceManeuver> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GuidanceManeuver>();
            (::windows::core::Vtable::vtable(this).AfterNextManeuver)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AfterNextManeuverDistance(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Vtable::vtable(this).AfterNextManeuverDistance)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DistanceToDestination(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Vtable::vtable(this).DistanceToDestination)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ElapsedDistance(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Vtable::vtable(this).ElapsedDistance)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ElapsedTime(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::TimeSpan>();
            (::windows::core::Vtable::vtable(this).ElapsedTime)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TimeToDestination(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::TimeSpan>();
            (::windows::core::Vtable::vtable(this).TimeToDestination)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RoadName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Vtable::vtable(this).RoadName)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Route(&self) -> ::windows::core::Result<GuidanceRoute> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GuidanceRoute>();
            (::windows::core::Vtable::vtable(this).Route)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CurrentLocation(&self) -> ::windows::core::Result<GuidanceMapMatchedCoordinate> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GuidanceMapMatchedCoordinate>();
            (::windows::core::Vtable::vtable(this).CurrentLocation)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsNewManeuver(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Vtable::vtable(this).IsNewManeuver)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn LaneInfo(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GuidanceLaneInfo>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Collections::IVectorView<GuidanceLaneInfo>>();
            (::windows::core::Vtable::vtable(this).LaneInfo)(::windows::core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GuidanceUpdatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GuidanceUpdatedEventArgs {}
impl ::core::fmt::Debug for GuidanceUpdatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GuidanceUpdatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GuidanceUpdatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.Guidance.GuidanceUpdatedEventArgs;{fdac160b-9e8d-4de3-a9fa-b06321d18db9})");
}
impl ::core::clone::Clone for GuidanceUpdatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for GuidanceUpdatedEventArgs {
    type Vtable = IGuidanceUpdatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for GuidanceUpdatedEventArgs {
    const IID: ::windows::core::GUID = <IGuidanceUpdatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GuidanceUpdatedEventArgs {
    const NAME: &'static str = "Windows.Services.Maps.Guidance.GuidanceUpdatedEventArgs";
}
::windows::core::interface_hierarchy!(GuidanceUpdatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GuidanceUpdatedEventArgs {}
unsafe impl ::core::marker::Sync for GuidanceUpdatedEventArgs {}
#[doc = "*Required features: `\"Services_Maps_Guidance\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GuidanceAudioMeasurementSystem(pub i32);
impl GuidanceAudioMeasurementSystem {
    pub const Meters: Self = Self(0i32);
    pub const MilesAndYards: Self = Self(1i32);
    pub const MilesAndFeet: Self = Self(2i32);
}
impl ::core::marker::Copy for GuidanceAudioMeasurementSystem {}
impl ::core::clone::Clone for GuidanceAudioMeasurementSystem {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GuidanceAudioMeasurementSystem {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for GuidanceAudioMeasurementSystem {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for GuidanceAudioMeasurementSystem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GuidanceAudioMeasurementSystem").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GuidanceAudioMeasurementSystem {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.Guidance.GuidanceAudioMeasurementSystem;i4)");
}
#[doc = "*Required features: `\"Services_Maps_Guidance\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GuidanceAudioNotificationKind(pub i32);
impl GuidanceAudioNotificationKind {
    pub const Maneuver: Self = Self(0i32);
    pub const Route: Self = Self(1i32);
    pub const Gps: Self = Self(2i32);
    pub const SpeedLimit: Self = Self(3i32);
    pub const Traffic: Self = Self(4i32);
    pub const TrafficCamera: Self = Self(5i32);
}
impl ::core::marker::Copy for GuidanceAudioNotificationKind {}
impl ::core::clone::Clone for GuidanceAudioNotificationKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GuidanceAudioNotificationKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for GuidanceAudioNotificationKind {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for GuidanceAudioNotificationKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GuidanceAudioNotificationKind").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GuidanceAudioNotificationKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.Guidance.GuidanceAudioNotificationKind;i4)");
}
#[doc = "*Required features: `\"Services_Maps_Guidance\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GuidanceAudioNotifications(pub u32);
impl GuidanceAudioNotifications {
    pub const None: Self = Self(0u32);
    pub const Maneuver: Self = Self(1u32);
    pub const Route: Self = Self(2u32);
    pub const Gps: Self = Self(4u32);
    pub const SpeedLimit: Self = Self(8u32);
    pub const Traffic: Self = Self(16u32);
    pub const TrafficCamera: Self = Self(32u32);
}
impl ::core::marker::Copy for GuidanceAudioNotifications {}
impl ::core::clone::Clone for GuidanceAudioNotifications {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GuidanceAudioNotifications {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for GuidanceAudioNotifications {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for GuidanceAudioNotifications {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GuidanceAudioNotifications").field(&self.0).finish()
    }
}
impl GuidanceAudioNotifications {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for GuidanceAudioNotifications {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for GuidanceAudioNotifications {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for GuidanceAudioNotifications {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for GuidanceAudioNotifications {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for GuidanceAudioNotifications {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows::core::RuntimeType for GuidanceAudioNotifications {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.Guidance.GuidanceAudioNotifications;u4)");
}
#[doc = "*Required features: `\"Services_Maps_Guidance\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GuidanceLaneMarkers(pub u32);
impl GuidanceLaneMarkers {
    pub const None: Self = Self(0u32);
    pub const LightRight: Self = Self(1u32);
    pub const Right: Self = Self(2u32);
    pub const HardRight: Self = Self(4u32);
    pub const Straight: Self = Self(8u32);
    pub const UTurnLeft: Self = Self(16u32);
    pub const HardLeft: Self = Self(32u32);
    pub const Left: Self = Self(64u32);
    pub const LightLeft: Self = Self(128u32);
    pub const UTurnRight: Self = Self(256u32);
    pub const Unknown: Self = Self(4294967295u32);
}
impl ::core::marker::Copy for GuidanceLaneMarkers {}
impl ::core::clone::Clone for GuidanceLaneMarkers {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GuidanceLaneMarkers {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for GuidanceLaneMarkers {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for GuidanceLaneMarkers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GuidanceLaneMarkers").field(&self.0).finish()
    }
}
impl GuidanceLaneMarkers {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for GuidanceLaneMarkers {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for GuidanceLaneMarkers {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for GuidanceLaneMarkers {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for GuidanceLaneMarkers {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for GuidanceLaneMarkers {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows::core::RuntimeType for GuidanceLaneMarkers {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.Guidance.GuidanceLaneMarkers;u4)");
}
#[doc = "*Required features: `\"Services_Maps_Guidance\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GuidanceManeuverKind(pub i32);
impl GuidanceManeuverKind {
    pub const None: Self = Self(0i32);
    pub const GoStraight: Self = Self(1i32);
    pub const UTurnRight: Self = Self(2i32);
    pub const UTurnLeft: Self = Self(3i32);
    pub const TurnKeepRight: Self = Self(4i32);
    pub const TurnLightRight: Self = Self(5i32);
    pub const TurnRight: Self = Self(6i32);
    pub const TurnHardRight: Self = Self(7i32);
    pub const KeepMiddle: Self = Self(8i32);
    pub const TurnKeepLeft: Self = Self(9i32);
    pub const TurnLightLeft: Self = Self(10i32);
    pub const TurnLeft: Self = Self(11i32);
    pub const TurnHardLeft: Self = Self(12i32);
    pub const FreewayEnterRight: Self = Self(13i32);
    pub const FreewayEnterLeft: Self = Self(14i32);
    pub const FreewayLeaveRight: Self = Self(15i32);
    pub const FreewayLeaveLeft: Self = Self(16i32);
    pub const FreewayKeepRight: Self = Self(17i32);
    pub const FreewayKeepLeft: Self = Self(18i32);
    pub const TrafficCircleRight1: Self = Self(19i32);
    pub const TrafficCircleRight2: Self = Self(20i32);
    pub const TrafficCircleRight3: Self = Self(21i32);
    pub const TrafficCircleRight4: Self = Self(22i32);
    pub const TrafficCircleRight5: Self = Self(23i32);
    pub const TrafficCircleRight6: Self = Self(24i32);
    pub const TrafficCircleRight7: Self = Self(25i32);
    pub const TrafficCircleRight8: Self = Self(26i32);
    pub const TrafficCircleRight9: Self = Self(27i32);
    pub const TrafficCircleRight10: Self = Self(28i32);
    pub const TrafficCircleRight11: Self = Self(29i32);
    pub const TrafficCircleRight12: Self = Self(30i32);
    pub const TrafficCircleLeft1: Self = Self(31i32);
    pub const TrafficCircleLeft2: Self = Self(32i32);
    pub const TrafficCircleLeft3: Self = Self(33i32);
    pub const TrafficCircleLeft4: Self = Self(34i32);
    pub const TrafficCircleLeft5: Self = Self(35i32);
    pub const TrafficCircleLeft6: Self = Self(36i32);
    pub const TrafficCircleLeft7: Self = Self(37i32);
    pub const TrafficCircleLeft8: Self = Self(38i32);
    pub const TrafficCircleLeft9: Self = Self(39i32);
    pub const TrafficCircleLeft10: Self = Self(40i32);
    pub const TrafficCircleLeft11: Self = Self(41i32);
    pub const TrafficCircleLeft12: Self = Self(42i32);
    pub const Start: Self = Self(43i32);
    pub const End: Self = Self(44i32);
    pub const TakeFerry: Self = Self(45i32);
    pub const PassTransitStation: Self = Self(46i32);
    pub const LeaveTransitStation: Self = Self(47i32);
}
impl ::core::marker::Copy for GuidanceManeuverKind {}
impl ::core::clone::Clone for GuidanceManeuverKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GuidanceManeuverKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for GuidanceManeuverKind {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for GuidanceManeuverKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GuidanceManeuverKind").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GuidanceManeuverKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.Guidance.GuidanceManeuverKind;i4)");
}
#[doc = "*Required features: `\"Services_Maps_Guidance\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GuidanceMode(pub i32);
impl GuidanceMode {
    pub const None: Self = Self(0i32);
    pub const Simulation: Self = Self(1i32);
    pub const Navigation: Self = Self(2i32);
    pub const Tracking: Self = Self(3i32);
}
impl ::core::marker::Copy for GuidanceMode {}
impl ::core::clone::Clone for GuidanceMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GuidanceMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for GuidanceMode {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for GuidanceMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GuidanceMode").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GuidanceMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.Guidance.GuidanceMode;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
