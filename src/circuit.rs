extern "C" {
    pub fn Circuit_Get_Name() -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn Circuit_Get_NumBuses() -> i32;
}
extern "C" {
    pub fn Circuit_Get_NumCktElements() -> i32;
}
extern "C" {
    pub fn Circuit_Get_NumNodes() -> i32;
}
extern "C" {
    pub fn Circuit_Get_LineLosses(ResultPtr: *mut *mut f64, ResultCount: *mut i32);
}
extern "C" {
    pub fn Circuit_Get_LineLosses_GR();
}
extern "C" {
    pub fn Circuit_Get_Losses(ResultPtr: *mut *mut f64, ResultCount: *mut i32);
}
extern "C" {
    pub fn Circuit_Get_Losses_GR();
}
extern "C" {
    pub fn Circuit_Get_AllBusVmag(ResultPtr: *mut *mut f64, ResultCount: *mut i32);
}
extern "C" {
    pub fn Circuit_Get_AllBusVmag_GR();
}
extern "C" {
    pub fn Circuit_Get_AllBusVolts(ResultPtr: *mut *mut f64, ResultCount: *mut i32);
}
extern "C" {
    pub fn Circuit_Get_AllBusVolts_GR();
}
extern "C" {
    pub fn Circuit_Get_AllElementNames(
        ResultPtr: *mut *mut *mut ::std::os::raw::c_char,
        ResultCount: *mut i32,
    );
}
extern "C" {
    pub fn Circuit_Get_AllElementNames_GR();
}
extern "C" {
    pub fn Circuit_Get_SubstationLosses(ResultPtr: *mut *mut f64, ResultCount: *mut i32);
}
extern "C" {
    pub fn Circuit_Get_SubstationLosses_GR();
}
extern "C" {
    pub fn Circuit_Get_TotalPower(ResultPtr: *mut *mut f64, ResultCount: *mut i32);
}
extern "C" {
    pub fn Circuit_Get_TotalPower_GR();
}
extern "C" {
    pub fn Circuit_Disable(Name: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn Circuit_Enable(Name: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn Circuit_FirstPCElement() -> i32;
}
extern "C" {
    pub fn Circuit_FirstPDElement() -> i32;
}
extern "C" {
    pub fn Circuit_NextPCElement() -> i32;
}
extern "C" {
    pub fn Circuit_NextPDElement() -> i32;
}
extern "C" {
    pub fn Circuit_Get_AllBusNames(
        ResultPtr: *mut *mut *mut ::std::os::raw::c_char,
        ResultCount: *mut i32,
    );
}
extern "C" {
    pub fn Circuit_Get_AllBusNames_GR();
}
extern "C" {
    pub fn Circuit_Get_AllElementLosses(ResultPtr: *mut *mut f64, ResultCount: *mut i32);
}
extern "C" {
    pub fn Circuit_Get_AllElementLosses_GR();
}
extern "C" {
    pub fn Circuit_Sample();
}
extern "C" {
    pub fn Circuit_SaveSample();
}
extern "C" {
    pub fn Circuit_SetActiveElement(FullName: *mut ::std::os::raw::c_char) -> i32;
}
extern "C" {
    pub fn Circuit_Capacity(Start: f64, Increment: f64) -> f64;
}
extern "C" {
    pub fn Circuit_Get_AllBusVmagPu(ResultPtr: *mut *mut f64, ResultCount: *mut i32);
}
extern "C" {
    pub fn Circuit_Get_AllBusVmagPu_GR();
}
extern "C" {
    pub fn Circuit_SetActiveBus(BusName: *mut ::std::os::raw::c_char) -> i32;
}
extern "C" {
    pub fn Circuit_SetActiveBusi(BusIndex: i32) -> i32;
}
extern "C" {
    pub fn Circuit_Get_AllNodeNames(
        ResultPtr: *mut *mut *mut ::std::os::raw::c_char,
        ResultCount: *mut i32,
    );
}
extern "C" {
    pub fn Circuit_Get_AllNodeNames_GR();
}
extern "C" {
    pub fn Circuit_Get_SystemY(ResultPtr: *mut *mut f64, ResultCount: *mut i32);
}
extern "C" {
    pub fn Circuit_Get_SystemY_GR();
}
extern "C" {
    pub fn Circuit_Get_AllBusDistances(ResultPtr: *mut *mut f64, ResultCount: *mut i32);
}
extern "C" {
    pub fn Circuit_Get_AllBusDistances_GR();
}
extern "C" {
    pub fn Circuit_Get_AllNodeDistances(ResultPtr: *mut *mut f64, ResultCount: *mut i32);
}
extern "C" {
    pub fn Circuit_Get_AllNodeDistances_GR();
}
extern "C" {
    pub fn Circuit_Get_AllNodeDistancesByPhase(
        ResultPtr: *mut *mut f64,
        ResultCount: *mut i32,
        Phase: i32,
    );
}
extern "C" {
    pub fn Circuit_Get_AllNodeDistancesByPhase_GR(Phase: i32);
}
extern "C" {
    pub fn Circuit_Get_AllNodeVmagByPhase(
        ResultPtr: *mut *mut f64,
        ResultCount: *mut i32,
        Phase: i32,
    );
}
extern "C" {
    pub fn Circuit_Get_AllNodeVmagByPhase_GR(Phase: i32);
}
extern "C" {
    pub fn Circuit_Get_AllNodeVmagPUByPhase(
        ResultPtr: *mut *mut f64,
        ResultCount: *mut i32,
        Phase: i32,
    );
}
extern "C" {
    pub fn Circuit_Get_AllNodeVmagPUByPhase_GR(Phase: i32);
}
extern "C" {
    pub fn Circuit_Get_AllNodeNamesByPhase(
        ResultPtr: *mut *mut *mut ::std::os::raw::c_char,
        ResultCount: *mut i32,
        Phase: i32,
    );
}
extern "C" {
    pub fn Circuit_Get_AllNodeNamesByPhase_GR(Phase: i32);
}
extern "C" {
    pub fn Circuit_SetActiveClass(ClassName: *mut ::std::os::raw::c_char) -> i32;
}
extern "C" {
    pub fn Circuit_FirstElement() -> i32;
}
extern "C" {
    pub fn Circuit_NextElement() -> i32;
}
extern "C" {
    pub fn Circuit_UpdateStorage();
}
extern "C" {
    pub fn Circuit_Get_ParentPDElement() -> i32;
}
extern "C" {
    pub fn Circuit_EndOfTimeStepUpdate();
}
extern "C" {
    pub fn Circuit_Get_YNodeOrder(
        ResultPtr: *mut *mut *mut ::std::os::raw::c_char,
        ResultCount: *mut i32,
    );
}
extern "C" {
    pub fn Circuit_Get_YNodeOrder_GR();
}
extern "C" {
    pub fn Circuit_Get_YCurrents(ResultPtr: *mut *mut f64, ResultCount: *mut i32);
}
extern "C" {
    pub fn Circuit_Get_YCurrents_GR();
}
extern "C" {
    pub fn Circuit_Get_YNodeVarray(ResultPtr: *mut *mut f64, ResultCount: *mut i32);
}
extern "C" {
    pub fn Circuit_Get_YNodeVarray_GR();
}
extern "C" {
    pub fn Circuit_SetCktElementName(Value: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn Circuit_SetCktElementIndex(Value: i32);
}
