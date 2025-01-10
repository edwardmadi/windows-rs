windows_core::imp::define_interface!(IJsonArray, IJsonArray_Vtbl, 0x08c1ddb6_0cbd_4a9a_b5d3_2f852dc37e81);
impl windows_core::RuntimeType for IJsonArray {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IJsonArray_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetObjectAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetObjectAt: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetArrayAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetArrayAt: usize,
    pub GetStringAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNumberAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut f64) -> windows_core::HRESULT,
    pub GetBooleanAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IJsonArrayStatics, IJsonArrayStatics_Vtbl, 0xdb1434a9_e164_499f_93e2_8a8f49bb90ba);
impl windows_core::RuntimeType for IJsonArrayStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IJsonArrayStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Parse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Parse: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub TryParse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TryParse: usize,
}
windows_core::imp::define_interface!(IJsonErrorStatics2, IJsonErrorStatics2_Vtbl, 0x404030da_87d0_436c_83ab_fc7b12c0cc26);
impl windows_core::RuntimeType for IJsonErrorStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IJsonErrorStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetJsonStatus: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut JsonErrorStatus) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IJsonObject, IJsonObject_Vtbl, 0x064e24dd_29c2_4f83_9ac1_9ee11578beb3);
impl windows_core::RuntimeType for IJsonObject {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IJsonObject_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetNamedValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetNamedValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetNamedObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetNamedObject: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetNamedArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetNamedArray: usize,
    pub GetNamedString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNamedNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub GetNamedBoolean: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IJsonObjectStatics, IJsonObjectStatics_Vtbl, 0x2289f159_54de_45d8_abcc_22603fa066a0);
impl windows_core::RuntimeType for IJsonObjectStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IJsonObjectStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Parse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Parse: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub TryParse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TryParse: usize,
}
windows_core::imp::define_interface!(IJsonObjectWithDefaultValues, IJsonObjectWithDefaultValues_Vtbl, 0xd960d2a2_b7f0_4f00_8e44_d82cf415ea13);
impl windows_core::RuntimeType for IJsonObjectWithDefaultValues {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IJsonObjectWithDefaultValues_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetNamedValueOrDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetNamedObjectOrDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetNamedObjectOrDefault: usize,
    pub GetNamedStringOrDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetNamedArrayOrDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetNamedArrayOrDefault: usize,
    pub GetNamedNumberOrDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, f64, *mut f64) -> windows_core::HRESULT,
    pub GetNamedBooleanOrDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, bool, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IJsonValue, IJsonValue_Vtbl, 0xa3219ecb_f0b3_4dcd_beee_19d48cd3ed1e);
impl windows_core::RuntimeType for IJsonValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IJsonValue, windows_core::IUnknown, windows_core::IInspectable);
impl IJsonValue {
    pub fn ValueType(&self) -> windows_core::Result<JsonValueType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ValueType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Stringify(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Stringify)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn GetString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetString)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn GetNumber(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetNumber)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetBoolean(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetBoolean)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetArray(&self) -> windows_core::Result<JsonArray> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetArray)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetObject(&self) -> windows_core::Result<JsonObject> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetObject)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for IJsonValue {
    const NAME: &'static str = "Windows.Data.Json.IJsonValue";
}
#[cfg(feature = "Foundation_Collections")]
pub trait IJsonValue_Impl: windows_core::IUnknownImpl {
    fn ValueType(&self) -> windows_core::Result<JsonValueType>;
    fn Stringify(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn GetString(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn GetNumber(&self) -> windows_core::Result<f64>;
    fn GetBoolean(&self) -> windows_core::Result<bool>;
    fn GetArray(&self) -> windows_core::Result<JsonArray>;
    fn GetObject(&self) -> windows_core::Result<JsonObject>;
}
#[cfg(feature = "Foundation_Collections")]
impl IJsonValue_Vtbl {
    pub const fn new<Identity: IJsonValue_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ValueType<Identity: IJsonValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut JsonValueType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IJsonValue_Impl::ValueType(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Stringify<Identity: IJsonValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IJsonValue_Impl::Stringify(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetString<Identity: IJsonValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IJsonValue_Impl::GetString(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNumber<Identity: IJsonValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IJsonValue_Impl::GetNumber(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBoolean<Identity: IJsonValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IJsonValue_Impl::GetBoolean(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetArray<Identity: IJsonValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IJsonValue_Impl::GetArray(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetObject<Identity: IJsonValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IJsonValue_Impl::GetObject(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IJsonValue, OFFSET>(),
            ValueType: ValueType::<Identity, OFFSET>,
            Stringify: Stringify::<Identity, OFFSET>,
            GetString: GetString::<Identity, OFFSET>,
            GetNumber: GetNumber::<Identity, OFFSET>,
            GetBoolean: GetBoolean::<Identity, OFFSET>,
            GetArray: GetArray::<Identity, OFFSET>,
            GetObject: GetObject::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IJsonValue as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct IJsonValue_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ValueType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut JsonValueType) -> windows_core::HRESULT,
    pub Stringify: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub GetBoolean: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetObject: usize,
}
windows_core::imp::define_interface!(IJsonValueStatics, IJsonValueStatics_Vtbl, 0x5f6b544a_2f53_48e1_91a3_f78b50a6345c);
impl windows_core::RuntimeType for IJsonValueStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IJsonValueStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Parse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryParse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub CreateBooleanValue: unsafe extern "system" fn(*mut core::ffi::c_void, bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateNumberValue: unsafe extern "system" fn(*mut core::ffi::c_void, f64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateStringValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IJsonValueStatics2, IJsonValueStatics2_Vtbl, 0x1d9ecbe4_3fe8_4335_8392_93d8e36865f0);
impl windows_core::RuntimeType for IJsonValueStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IJsonValueStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateNullValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct JsonArray(windows_core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
windows_core::imp::interface_hierarchy!(JsonArray, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
windows_core::imp::required_hierarchy!(JsonArray, super::super::Foundation::Collections::IIterable<IJsonValue>, IJsonValue, super::super::Foundation::IStringable, super::super::Foundation::Collections::IVector<IJsonValue>);
#[cfg(feature = "Foundation_Collections")]
impl JsonArray {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<JsonArray, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn First(&self) -> windows_core::Result<super::super::Foundation::Collections::IIterator<IJsonValue>> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::Collections::IIterable<IJsonValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetObjectAt(&self, index: u32) -> windows_core::Result<JsonObject> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetObjectAt)(windows_core::Interface::as_raw(this), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetArrayAt(&self, index: u32) -> windows_core::Result<JsonArray> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetArrayAt)(windows_core::Interface::as_raw(this), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetStringAt(&self, index: u32) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetStringAt)(windows_core::Interface::as_raw(this), index, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn GetNumberAt(&self, index: u32) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetNumberAt)(windows_core::Interface::as_raw(this), index, &mut result__).map(|| result__)
        }
    }
    pub fn GetBooleanAt(&self, index: u32) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetBooleanAt)(windows_core::Interface::as_raw(this), index, &mut result__).map(|| result__)
        }
    }
    pub fn Parse(input: &windows_core::HSTRING) -> windows_core::Result<JsonArray> {
        Self::IJsonArrayStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parse)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn TryParse(input: &windows_core::HSTRING, result: &mut Option<JsonArray>) -> windows_core::Result<bool> {
        Self::IJsonArrayStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryParse)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), result as *mut _ as _, &mut result__).map(|| result__)
        })
    }
    pub fn ValueType(&self) -> windows_core::Result<JsonValueType> {
        let this = &windows_core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ValueType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Stringify(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Stringify)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn GetString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetString)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn GetNumber(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetNumber)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetBoolean(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetBoolean)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetArray(&self) -> windows_core::Result<JsonArray> {
        let this = &windows_core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetArray)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetObject(&self) -> windows_core::Result<JsonObject> {
        let this = &windows_core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetObject)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ToString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToString)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn GetAt(&self, index: u32) -> windows_core::Result<IJsonValue> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAt)(windows_core::Interface::as_raw(this), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetView(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<IJsonValue>> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<IJsonValue>,
    {
        let this = &windows_core::Interface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IndexOf)(windows_core::Interface::as_raw(this), value.param().abi(), index, &mut result__).map(|| result__)
        }
    }
    pub fn SetAt<P1>(&self, index: u32, value: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IJsonValue>,
    {
        let this = &windows_core::Interface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAt)(windows_core::Interface::as_raw(this), index, value.param().abi()).ok() }
    }
    pub fn InsertAt<P1>(&self, index: u32, value: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IJsonValue>,
    {
        let this = &windows_core::Interface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).InsertAt)(windows_core::Interface::as_raw(this), index, value.param().abi()).ok() }
    }
    pub fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveAt)(windows_core::Interface::as_raw(this), index).ok() }
    }
    pub fn Append<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IJsonValue>,
    {
        let this = &windows_core::Interface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Append)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn RemoveAtEnd(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveAtEnd)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GetMany(&self, startindex: u32, items: &mut [Option<IJsonValue>]) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMany)(windows_core::Interface::as_raw(this), startindex, items.len().try_into().unwrap(), core::mem::transmute_copy(&items), &mut result__).map(|| result__)
        }
    }
    pub fn ReplaceAll(&self, items: &[Option<IJsonValue>]) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReplaceAll)(windows_core::Interface::as_raw(this), items.len().try_into().unwrap(), core::mem::transmute(items.as_ptr())).ok() }
    }
    fn IJsonArrayStatics<R, F: FnOnce(&IJsonArrayStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<JsonArray, IJsonArrayStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeType for JsonArray {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IJsonArray>();
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl windows_core::Interface for JsonArray {
    type Vtable = <IJsonArray as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IJsonArray as windows_core::Interface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for JsonArray {
    const NAME: &'static str = "Windows.Data.Json.JsonArray";
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl Send for JsonArray {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl Sync for JsonArray {}
#[cfg(feature = "Foundation_Collections")]
impl IntoIterator for JsonArray {
    type Item = IJsonValue;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl IntoIterator for &JsonArray {
    type Item = IJsonValue;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
pub struct JsonError;
impl JsonError {
    pub fn GetJsonStatus(hresult: i32) -> windows_core::Result<JsonErrorStatus> {
        Self::IJsonErrorStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetJsonStatus)(windows_core::Interface::as_raw(this), hresult, &mut result__).map(|| result__)
        })
    }
    fn IJsonErrorStatics2<R, F: FnOnce(&IJsonErrorStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<JsonError, IJsonErrorStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for JsonError {
    const NAME: &'static str = "Windows.Data.Json.JsonError";
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct JsonErrorStatus(pub i32);
impl JsonErrorStatus {
    pub const Unknown: Self = Self(0i32);
    pub const InvalidJsonString: Self = Self(1i32);
    pub const InvalidJsonNumber: Self = Self(2i32);
    pub const JsonValueNotFound: Self = Self(3i32);
    pub const ImplementationLimit: Self = Self(4i32);
}
impl windows_core::TypeKind for JsonErrorStatus {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for JsonErrorStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Data.Json.JsonErrorStatus;i4)");
}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct JsonObject(windows_core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
windows_core::imp::interface_hierarchy!(JsonObject, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
windows_core::imp::required_hierarchy ! ( JsonObject , super::super::Foundation::Collections:: IIterable < super::super::Foundation::Collections:: IKeyValuePair < windows_core::HSTRING , IJsonValue > > , IJsonValue , super::super::Foundation::Collections:: IMap < windows_core::HSTRING , IJsonValue > , super::super::Foundation:: IStringable );
#[cfg(feature = "Foundation_Collections")]
impl JsonObject {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<JsonObject, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn First(&self) -> windows_core::Result<super::super::Foundation::Collections::IIterator<super::super::Foundation::Collections::IKeyValuePair<windows_core::HSTRING, IJsonValue>>> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<windows_core::HSTRING, IJsonValue>>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetNamedValue(&self, name: &windows_core::HSTRING) -> windows_core::Result<JsonValue> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetNamedValue)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetNamedValue<P1>(&self, name: &windows_core::HSTRING, value: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IJsonValue>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetNamedValue)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.param().abi()).ok() }
    }
    pub fn GetNamedObject(&self, name: &windows_core::HSTRING) -> windows_core::Result<JsonObject> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetNamedObject)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetNamedArray(&self, name: &windows_core::HSTRING) -> windows_core::Result<JsonArray> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetNamedArray)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetNamedString(&self, name: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetNamedString)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn GetNamedNumber(&self, name: &windows_core::HSTRING) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetNamedNumber)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), &mut result__).map(|| result__)
        }
    }
    pub fn GetNamedBoolean(&self, name: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetNamedBoolean)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), &mut result__).map(|| result__)
        }
    }
    pub fn Parse(input: &windows_core::HSTRING) -> windows_core::Result<JsonObject> {
        Self::IJsonObjectStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parse)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn TryParse(input: &windows_core::HSTRING, result: &mut Option<JsonObject>) -> windows_core::Result<bool> {
        Self::IJsonObjectStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryParse)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), result as *mut _ as _, &mut result__).map(|| result__)
        })
    }
    pub fn GetNamedValueOrDefault<P1>(&self, name: &windows_core::HSTRING, defaultvalue: P1) -> windows_core::Result<JsonValue>
    where
        P1: windows_core::Param<JsonValue>,
    {
        let this = &windows_core::Interface::cast::<IJsonObjectWithDefaultValues>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetNamedValueOrDefault)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), defaultvalue.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetNamedObjectOrDefault<P1>(&self, name: &windows_core::HSTRING, defaultvalue: P1) -> windows_core::Result<JsonObject>
    where
        P1: windows_core::Param<JsonObject>,
    {
        let this = &windows_core::Interface::cast::<IJsonObjectWithDefaultValues>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetNamedObjectOrDefault)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), defaultvalue.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetNamedStringOrDefault(&self, name: &windows_core::HSTRING, defaultvalue: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IJsonObjectWithDefaultValues>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetNamedStringOrDefault)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), core::mem::transmute_copy(defaultvalue), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn GetNamedArrayOrDefault<P1>(&self, name: &windows_core::HSTRING, defaultvalue: P1) -> windows_core::Result<JsonArray>
    where
        P1: windows_core::Param<JsonArray>,
    {
        let this = &windows_core::Interface::cast::<IJsonObjectWithDefaultValues>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetNamedArrayOrDefault)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), defaultvalue.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetNamedNumberOrDefault(&self, name: &windows_core::HSTRING, defaultvalue: f64) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IJsonObjectWithDefaultValues>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetNamedNumberOrDefault)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), defaultvalue, &mut result__).map(|| result__)
        }
    }
    pub fn GetNamedBooleanOrDefault(&self, name: &windows_core::HSTRING, defaultvalue: bool) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IJsonObjectWithDefaultValues>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetNamedBooleanOrDefault)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), defaultvalue, &mut result__).map(|| result__)
        }
    }
    pub fn ValueType(&self) -> windows_core::Result<JsonValueType> {
        let this = &windows_core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ValueType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Stringify(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Stringify)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn GetString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetString)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn GetNumber(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetNumber)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetBoolean(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetBoolean)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetArray(&self) -> windows_core::Result<JsonArray> {
        let this = &windows_core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetArray)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetObject(&self) -> windows_core::Result<JsonObject> {
        let this = &windows_core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetObject)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Lookup(&self, key: &windows_core::HSTRING) -> windows_core::Result<IJsonValue> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::Collections::IMap<windows_core::HSTRING, IJsonValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Lookup)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::Collections::IMap<windows_core::HSTRING, IJsonValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HasKey(&self, key: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::Collections::IMap<windows_core::HSTRING, IJsonValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasKey)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), &mut result__).map(|| result__)
        }
    }
    pub fn GetView(&self) -> windows_core::Result<super::super::Foundation::Collections::IMapView<windows_core::HSTRING, IJsonValue>> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::Collections::IMap<windows_core::HSTRING, IJsonValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Insert<P1>(&self, key: &windows_core::HSTRING, value: P1) -> windows_core::Result<bool>
    where
        P1: windows_core::Param<IJsonValue>,
    {
        let this = &windows_core::Interface::cast::<super::super::Foundation::Collections::IMap<windows_core::HSTRING, IJsonValue>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Insert)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn Remove(&self, key: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::Collections::IMap<windows_core::HSTRING, IJsonValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Remove)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key)).ok() }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::Collections::IMap<windows_core::HSTRING, IJsonValue>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ToString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToString)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    fn IJsonObjectStatics<R, F: FnOnce(&IJsonObjectStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<JsonObject, IJsonObjectStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeType for JsonObject {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IJsonObject>();
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl windows_core::Interface for JsonObject {
    type Vtable = <IJsonObject as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IJsonObject as windows_core::Interface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for JsonObject {
    const NAME: &'static str = "Windows.Data.Json.JsonObject";
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl Send for JsonObject {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl Sync for JsonObject {}
#[cfg(feature = "Foundation_Collections")]
impl IntoIterator for JsonObject {
    type Item = super::super::Foundation::Collections::IKeyValuePair<windows_core::HSTRING, IJsonValue>;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl IntoIterator for &JsonObject {
    type Item = super::super::Foundation::Collections::IKeyValuePair<windows_core::HSTRING, IJsonValue>;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct JsonValue(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(JsonValue, windows_core::IUnknown, windows_core::IInspectable, IJsonValue);
windows_core::imp::required_hierarchy!(JsonValue, super::super::Foundation::IStringable);
impl JsonValue {
    pub fn ValueType(&self) -> windows_core::Result<JsonValueType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ValueType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Stringify(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Stringify)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn GetString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetString)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn GetNumber(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetNumber)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetBoolean(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetBoolean)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetArray(&self) -> windows_core::Result<JsonArray> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetArray)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetObject(&self) -> windows_core::Result<JsonObject> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetObject)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Parse(input: &windows_core::HSTRING) -> windows_core::Result<JsonValue> {
        Self::IJsonValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parse)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn TryParse(input: &windows_core::HSTRING, result: &mut Option<JsonValue>) -> windows_core::Result<bool> {
        Self::IJsonValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryParse)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), result as *mut _ as _, &mut result__).map(|| result__)
        })
    }
    pub fn CreateBooleanValue(input: bool) -> windows_core::Result<JsonValue> {
        Self::IJsonValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateBooleanValue)(windows_core::Interface::as_raw(this), input, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateNumberValue(input: f64) -> windows_core::Result<JsonValue> {
        Self::IJsonValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateNumberValue)(windows_core::Interface::as_raw(this), input, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateStringValue(input: &windows_core::HSTRING) -> windows_core::Result<JsonValue> {
        Self::IJsonValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateStringValue)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(input), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateNullValue() -> windows_core::Result<JsonValue> {
        Self::IJsonValueStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateNullValue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ToString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToString)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    fn IJsonValueStatics<R, F: FnOnce(&IJsonValueStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<JsonValue, IJsonValueStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IJsonValueStatics2<R, F: FnOnce(&IJsonValueStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<JsonValue, IJsonValueStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for JsonValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IJsonValue>();
}
unsafe impl windows_core::Interface for JsonValue {
    type Vtable = <IJsonValue as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IJsonValue as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for JsonValue {
    const NAME: &'static str = "Windows.Data.Json.JsonValue";
}
unsafe impl Send for JsonValue {}
unsafe impl Sync for JsonValue {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct JsonValueType(pub i32);
impl JsonValueType {
    pub const Null: Self = Self(0i32);
    pub const Boolean: Self = Self(1i32);
    pub const Number: Self = Self(2i32);
    pub const String: Self = Self(3i32);
    pub const Array: Self = Self(4i32);
    pub const Object: Self = Self(5i32);
}
impl windows_core::TypeKind for JsonValueType {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for JsonValueType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Data.Json.JsonValueType;i4)");
}
