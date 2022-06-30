extern "C" {
    pub fn DSS_ResetStringBuffer();
}
extern "C" {
    pub fn DSS_Dispose_PByte(p: *mut *mut i8);
}
extern "C" {
    pub fn DSS_Dispose_PDouble(p: *mut *mut f64);
}
extern "C" {
    pub fn DSS_Dispose_PInteger(p: *mut *mut i32);
}
extern "C" {
    pub fn DSS_Dispose_PPAnsiChar(p: *mut *mut *mut ::std::os::raw::c_char, cnt: i32);
}
extern "C" {
    pub fn DSS_Get_PAnsiChar(
        p: *mut ::std::os::raw::c_void,
        index: i32,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn DSS_DisposeGRData();
}
extern "C" {
    pub fn DSS_GetGRPointers(
        DataPtr_PPAnsiChar: *mut *mut *mut *mut ::std::os::raw::c_char,
        DataPtr_PDouble: *mut *mut *mut f64,
        DataPtr_PInteger: *mut *mut *mut i32,
        DataPtr_PByte: *mut *mut *mut i8,
        CountPtr_PPAnsiChar: *mut *mut i32,
        CountPtr_PDouble: *mut *mut i32,
        CountPtr_PInteger: *mut *mut i32,
        CountPtr_PByte: *mut *mut i32,
    );
}
extern "C" {
    pub fn DSS_GR_DataPtr_PDouble() -> *mut f64;
}
extern "C" {
    pub fn DSS_GR_DataPtr_PInteger() -> *mut i32;
}
extern "C" {
    pub fn DSS_GR_DataPtr_PByte() -> *mut i8;
}
extern "C" {
    pub fn DSS_GR_CountPtr_PDouble() -> *mut i32;
}
extern "C" {
    pub fn DSS_GR_CountPtr_PInteger() -> *mut i32;
}
extern "C" {
    pub fn DSS_GR_CountPtr_PByte() -> *mut i32;
}
extern "C" {
    pub fn DSS_RegisterPlotCallback(cb: dss_callback_plot_t);
}
extern "C" {
    pub fn DSS_RegisterMessageCallback(cb: dss_callback_message_t);
}
extern "C" {
    pub fn DSS_NewCircuit(Value: *mut ::std::os::raw::c_char);
}

extern "C" {
    pub fn DSS_Get_NumCircuits() -> i32;
}
extern "C" {
    pub fn DSS_ClearAll();
}
extern "C" {
    pub fn DSS_Get_Version() -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn DSS_Start(code: i32) -> u16;
}
extern "C" {
    pub fn DSS_Get_Classes(ResultPtr: *mut *mut *mut ::std::os::raw::c_char, ResultCount: *mut i32);
}
extern "C" {
    pub fn DSS_Get_Classes_GR();
}
extern "C" {
    pub fn DSS_Get_UserClasses(
        ResultPtr: *mut *mut *mut ::std::os::raw::c_char,
        ResultCount: *mut i32,
    );
}
extern "C" {
    pub fn DSS_Get_UserClasses_GR();
}
extern "C" {
    pub fn DSS_Get_NumClasses() -> i32;
}
extern "C" {
    pub fn DSS_Get_NumUserClasses() -> i32;
}
extern "C" {
    pub fn DSS_Get_DataPath() -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn DSS_Set_DataPath(Value: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn DSS_Reset();
}
extern "C" {
    pub fn DSS_Get_DefaultEditor() -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn DSS_SetActiveClass(ClassName: *mut ::std::os::raw::c_char) -> i32;
}
extern "C" {
    pub fn DSS_Get_AllowForms() -> u16;
}
extern "C" {
    pub fn DSS_Set_AllowForms(Value: u16);
}
extern "C" {
    pub fn DSS_Get_AllowEditor() -> u16;
}
extern "C" {
    pub fn DSS_Set_AllowEditor(Value: u16);
}
extern "C" {
    pub fn DSS_Get_LegacyModels() -> u16;
}
extern "C" {
    pub fn DSS_Set_LegacyModels(Value: u16);
}
extern "C" {
    pub fn DSS_Get_AllowDOScmd() -> u16;
}
extern "C" {
    pub fn DSS_Set_AllowDOScmd(Value: u16);
}
extern "C" {
    pub fn DSS_Get_AllowChangeDir() -> u16;
}
extern "C" {
    pub fn DSS_Set_AllowChangeDir(Value: u16);
}
extern "C" {
    pub fn DSS_Get_COMErrorResults() -> u16;
}
extern "C" {
    pub fn DSS_Set_COMErrorResults(Value: u16);
}
extern "C" {
    pub fn DSSElement_Get_AllPropertyNames(
        ResultPtr: *mut *mut *mut ::std::os::raw::c_char,
        ResultCount: *mut i32,
    );
}
extern "C" {
    pub fn DSSElement_Get_AllPropertyNames_GR();
}
extern "C" {
    pub fn DSSElement_Get_Name() -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn DSSElement_Get_NumProperties() -> i32;
}
extern "C" {
    pub fn DSSimComs_BusVoltagepu(ResultPtr: *mut *mut f64, ResultCount: *mut i32, Index: size_t);
}
extern "C" {
    pub fn DSSimComs_BusVoltagepu_GR(Index: size_t);
}
extern "C" {
    pub fn DSSimComs_BusVoltage(ResultPtr: *mut *mut f64, ResultCount: *mut i32, Index: size_t);
}
extern "C" {
    pub fn DSSimComs_BusVoltage_GR(Index: size_t);
}
extern "C" {
    pub fn DSSProgress_Close();
}
extern "C" {
    pub fn DSSProgress_Set_Caption(Value: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn DSSProgress_Set_PctProgress(Value: i32);
}
extern "C" {
    pub fn DSSProgress_Show();
}
extern "C" {
    pub fn DSSProperty_Get_Description() -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn DSSProperty_Get_Name() -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn DSSProperty_Get_Val() -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn DSSProperty_Set_Val(Value: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn DSSProperty_Set_Name(Value: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn DSSProperty_Set_Index(Value: i32);
}
extern "C" {
    pub fn DSS_ExtractSchema(ctx: *mut ::std::os::raw::c_void) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn DSS_Dispose_String(S: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn DSS_Dispose_PPointer(p: *mut *mut *mut ::std::os::raw::c_void);
}
