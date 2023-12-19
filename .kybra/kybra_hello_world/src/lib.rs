#![allow(warnings, unused)]
use candid::{Decode, Encode};
use kybra_vm_value_derive::{CdkActTryFromVmValue, CdkActTryIntoVmValue};
use rustpython_vm::{
    class::PyClassImpl as _KybraTraitPyClassImpl, convert::ToPyObject as _KybraTraitToPyObject,
    function::IntoFuncArgs as _KybraTraitIntoFuncArgs, AsObject as _KybraTraitAsObject,
    TryFromObject as _KybraTraitTryFromObject,
};
use serde::{
    de::{DeserializeSeed as _KybraTraitDeserializeSeed, Visitor as _KybraTraitVisitor},
    ser::{
        Serialize as _KybraTraitSerialize, SerializeMap as _KybraTraitSerializeMap,
        SerializeSeq as _KybraTraitSerializeSeq, SerializeTuple as _KybraTraitSerializeTuple,
    },
};
use slotmap::Key as _KybraTraitSlotMapKey;
use std::{convert::TryInto as _KybraTraitTryInto, str::FromStr as _KybraTraitFromStr};
thread_local! { static PYTHON_STDLIB_STABLE_REF_CELL : std :: cell :: RefCell < ic_stable_structures :: cell :: Cell < Vec < u8 > , ic_stable_structures :: memory_manager :: VirtualMemory < ic_stable_structures :: DefaultMemoryImpl > > > = std :: cell :: RefCell :: new (ic_stable_structures :: cell :: Cell :: init (MEMORY_MANAGER_REF_CELL . with (| m | m . borrow () . get (ic_stable_structures :: memory_manager :: MemoryId :: new (253u8))) , vec ! []) . unwrap_or_trap ()) ; static CANISTER_INITIALIZED_REF_CELL : std :: cell :: RefCell < ic_stable_structures :: cell :: Cell < u8 , ic_stable_structures :: memory_manager :: VirtualMemory < ic_stable_structures :: DefaultMemoryImpl > > > = std :: cell :: RefCell :: new (ic_stable_structures :: cell :: Cell :: init (MEMORY_MANAGER_REF_CELL . with (| m | m . borrow () . get (ic_stable_structures :: memory_manager :: MemoryId :: new (254u8))) , 0) . unwrap_or_trap ()) ; static RANDOMNESS_STABLE_REF_CELL : std :: cell :: RefCell < ic_stable_structures :: cell :: Cell < Vec < u8 > , ic_stable_structures :: memory_manager :: VirtualMemory < ic_stable_structures :: DefaultMemoryImpl > > > = std :: cell :: RefCell :: new (ic_stable_structures :: cell :: Cell :: init (MEMORY_MANAGER_REF_CELL . with (| m | m . borrow () . get (ic_stable_structures :: memory_manager :: MemoryId :: new (252u8))) , vec ! []) . unwrap_or_trap ()) ; }
trait ToCdkActTryIntoVmValueError {
    fn to_cdk_act_try_into_vm_value_error(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> CdkActTryIntoVmValueError;
}
impl ToCdkActTryIntoVmValueError for rustpython_vm::builtins::PyBaseExceptionRef {
    fn to_cdk_act_try_into_vm_value_error(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> CdkActTryIntoVmValueError {
        let py_object = self.to_pyobject(vm);
        let type_name = py_object.class().name().to_string();
        let err_message = match py_object.str(vm) {
            Ok(str) => str,
            Err(_) => {
                return CdkActTryIntoVmValueError(format!(
                    "Attribute Error: '{}' object has no attribute '__str__'",
                    type_name
                ));
            }
        };
        CdkActTryIntoVmValueError(format!("{}: {}", type_name, err_message))
    }
}
trait ToRustErrString {
    fn to_rust_err_string(self, vm: &rustpython::vm::VirtualMachine) -> String;
}
impl ToRustErrString for rustpython_vm::builtins::PyBaseExceptionRef {
    fn to_rust_err_string(self, vm: &rustpython::vm::VirtualMachine) -> String {
        let py_object = self.to_pyobject(vm);
        let type_name = py_object.class().name().to_string();
        match py_object.str(vm) {
            Ok(err_message) => format!("{type_name}: {}", err_message.to_string()),
            Err(_) => {
                format!("Attribute Error: '{type_name}' object has no attribute '__str__'")
            }
        }
    }
}
static mut INTERPRETER_OPTION: Option<rustpython_vm::Interpreter> = None;
static mut SCOPE_OPTION: Option<rustpython_vm::scope::Scope> = None;
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
thread_local! { static _CDK_RNG_REF_CELL : std :: cell :: RefCell < rand :: rngs :: StdRng > = std :: cell :: RefCell :: new (rand :: SeedableRng :: from_seed ([0u8 ; 32])) ; }
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
fn custom_getrandom(_buf: &mut [u8]) -> Result<(), getrandom::Error> {
    _CDK_RNG_REF_CELL.with(|rng_ref_cell| {
        let mut rng = rng_ref_cell.borrow_mut();
        rng.fill(_buf);
    });
    Ok(())
}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
getrandom::register_custom_getrandom!(custom_getrandom);
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
fn rng_seed() {
    ic_cdk::spawn(async move {
        let result: ic_cdk::api::call::CallResult<(Vec<u8>,)> =
            ic_cdk::api::management_canister::main::raw_rand().await;
        _CDK_RNG_REF_CELL.with(|rng_ref_cell| {
            let mut rng = rng_ref_cell.borrow_mut();
            match result {
                Ok(randomness) => {
                    *rng = rand::SeedableRng::from_seed(randomness.0[..].try_into().unwrap())
                }
                Err(err) => panic!(err),
            };
        });
    });
}
#[cfg(all(target_arch = "wasm32", target_os = "wasi"))]
fn rng_seed() {
    ic_cdk::spawn(async move {
        let result: ic_cdk::api::call::CallResult<(Vec<u8>,)> =
            ic_cdk::api::management_canister::main::raw_rand().await;
        match result {
            Ok(randomness) => unsafe { ic_wasi_polyfill::init_seed(&randomness.0) },
            Err(err) => panic!(err),
        };
    });
}
pub trait CdkActTryIntoVmValue<Context, VmValue> {
    fn try_into_vm_value(self, context: Context) -> Result<VmValue, CdkActTryIntoVmValueError>;
}
#[derive(Debug)]
pub struct CdkActTryIntoVmValueError(pub String);
impl
    CdkActTryFromVmValue<
        (),
        rustpython_vm::builtins::PyBaseExceptionRef,
        &rustpython::vm::VirtualMachine,
    > for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<(), rustpython_vm::builtins::PyBaseExceptionRef> {
        if self.is(&vm.ctx.none()) {
            Ok(())
        } else {
            let type_name = self.to_pyobject(vm).class().name().to_string();
            Err(vm.new_type_error(format!("expected NoneType but received {type_name}")))
        }
    }
}
impl
    CdkActTryFromVmValue<
        bool,
        rustpython_vm::builtins::PyBaseExceptionRef,
        &rustpython::vm::VirtualMachine,
    > for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<bool, rustpython_vm::builtins::PyBaseExceptionRef> {
        self.try_into_value(vm)
    }
}
impl
    CdkActTryFromVmValue<
        ic_cdk::export::candid::Empty,
        rustpython_vm::builtins::PyBaseExceptionRef,
        &rustpython::vm::VirtualMachine,
    > for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<ic_cdk::export::candid::Empty, rustpython_vm::builtins::PyBaseExceptionRef> {
        Err(vm.new_type_error("value cannot be converted to Empty".to_string()))
    }
}
impl
    CdkActTryFromVmValue<
        ic_cdk::export::candid::Func,
        rustpython_vm::builtins::PyBaseExceptionRef,
        &rustpython::vm::VirtualMachine,
    > for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<ic_cdk::export::candid::Func, rustpython_vm::builtins::PyBaseExceptionRef> {
        let tuple_self: rustpython_vm::builtins::PyTupleRef = self.try_into_value(vm)?;
        let principal = match tuple_self.get(0) {
            Some(principal) => principal,
            None => {
                return Err(vm.new_type_error(
                    "could not convert value to Func, missing Principal".to_string(),
                ))
            }
        };
        let method = match tuple_self.get(1) {
            Some(method) => method,
            None => {
                return Err(vm
                    .new_type_error("could not convert value to Func, missing method".to_string()))
            }
        };
        Ok(ic_cdk::export::candid::Func {
            principal: principal.clone().try_from_vm_value(vm)?,
            method: method.clone().try_from_vm_value(vm)?,
        })
    }
}
impl
    CdkActTryFromVmValue<
        ic_cdk::export::Principal,
        rustpython_vm::builtins::PyBaseExceptionRef,
        &rustpython::vm::VirtualMachine,
    > for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<ic_cdk::export::Principal, rustpython_vm::builtins::PyBaseExceptionRef> {
        let to_str = self.get_attr("to_str", vm)?;
        let result = to_str.call((), vm)?;
        let result_string: String = result.try_into_value(vm)?;
        match ic_cdk::export::Principal::from_text(result_string) {
            Ok(principal) => Ok(principal),
            Err(err) => Err(vm.new_type_error(format!(
                "could not convert value to Principal: {}",
                err.to_string()
            ))),
        }
    }
}
impl
    CdkActTryFromVmValue<
        ic_cdk::export::candid::Reserved,
        rustpython_vm::builtins::PyBaseExceptionRef,
        &rustpython::vm::VirtualMachine,
    > for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<ic_cdk::export::candid::Reserved, rustpython_vm::builtins::PyBaseExceptionRef> {
        Ok(ic_cdk::export::candid::Reserved)
    }
}
impl
    CdkActTryFromVmValue<
        ic_cdk_timers::TimerId,
        rustpython_vm::builtins::PyBaseExceptionRef,
        &rustpython::vm::VirtualMachine,
    > for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<ic_cdk_timers::TimerId, rustpython_vm::builtins::PyBaseExceptionRef> {
        let vm_value_as_u64: u64 = self.try_into_value(vm)?;
        Ok(ic_cdk_timers::TimerId::from(slotmap::KeyData::from_ffi(
            vm_value_as_u64,
        )))
    }
}
impl
    CdkActTryFromVmValue<
        String,
        rustpython_vm::builtins::PyBaseExceptionRef,
        &rustpython::vm::VirtualMachine,
    > for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<String, rustpython_vm::builtins::PyBaseExceptionRef> {
        self.try_into_value(vm)
    }
}
impl
    CdkActTryFromVmValue<
        Result<(), String>,
        rustpython_vm::builtins::PyBaseExceptionRef,
        &rustpython::vm::VirtualMachine,
    > for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<Result<(), String>, rustpython_vm::builtins::PyBaseExceptionRef> {
        let err = self.get_item("Err", vm);
        if let Ok(error_message) = err {
            return Ok(Err(error_message.try_from_vm_value(vm)?));
        }
        let ok = self.get_item("Ok", vm);
        if let Ok(value) = ok {
            let result: () = value.try_from_vm_value(vm)?;
            return Ok(Ok(()));
        }
        let type_name = self.to_pyobject(vm).class().name().to_string();
        Err(vm.new_type_error(format!("expected Result but received {type_name}")))
    }
}
impl<T>
    CdkActTryFromVmValue<
        (T,),
        rustpython_vm::builtins::PyBaseExceptionRef,
        &rustpython::vm::VirtualMachine,
    > for rustpython::vm::PyObjectRef
where
    rustpython::vm::PyObjectRef: for<'a> CdkActTryFromVmValue<
        T,
        rustpython_vm::builtins::PyBaseExceptionRef,
        &'a rustpython::vm::VirtualMachine,
    >,
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<(T,), rustpython_vm::builtins::PyBaseExceptionRef> {
        match self.try_from_vm_value(vm) {
            Ok(value) => Ok((value,)),
            Err(_) => Err(vm.new_type_error("Could not convert value to tuple".to_string())),
        }
    }
}
impl<T>
    CdkActTryFromVmValue<
        Box<T>,
        rustpython_vm::builtins::PyBaseExceptionRef,
        &rustpython::vm::VirtualMachine,
    > for rustpython::vm::PyObjectRef
where
    rustpython::vm::PyObjectRef: for<'a> CdkActTryFromVmValue<
        T,
        rustpython_vm::builtins::PyBaseExceptionRef,
        &'a rustpython::vm::VirtualMachine,
    >,
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<Box<T>, rustpython_vm::builtins::PyBaseExceptionRef> {
        Ok(Box::new(self.try_from_vm_value(vm)?))
    }
}
impl<T>
    CdkActTryFromVmValue<
        Option<T>,
        rustpython_vm::builtins::PyBaseExceptionRef,
        &rustpython::vm::VirtualMachine,
    > for rustpython::vm::PyObjectRef
where
    rustpython::vm::PyObjectRef: for<'a> CdkActTryFromVmValue<
        T,
        rustpython_vm::builtins::PyBaseExceptionRef,
        &'a rustpython::vm::VirtualMachine,
    >,
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<Option<T>, rustpython_vm::builtins::PyBaseExceptionRef> {
        if self.is(&vm.ctx.none()) {
            Ok(None)
        } else {
            Ok(Some(self.try_from_vm_value(vm)?))
        }
    }
}
impl<T>
    CdkActTryFromVmValue<
        ic_cdk::api::call::ManualReply<T>,
        rustpython_vm::builtins::PyBaseExceptionRef,
        &rustpython::vm::VirtualMachine,
    > for rustpython::vm::PyObjectRef
where
    rustpython::vm::PyObjectRef: for<'a> CdkActTryFromVmValue<
        T,
        rustpython_vm::builtins::PyBaseExceptionRef,
        &'a rustpython::vm::VirtualMachine,
    >,
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<ic_cdk::api::call::ManualReply<T>, rustpython_vm::builtins::PyBaseExceptionRef>
    {
        Ok(ic_cdk::api::call::ManualReply::empty())
    }
}
impl
    CdkActTryFromVmValue<
        f64,
        rustpython_vm::builtins::PyBaseExceptionRef,
        &rustpython::vm::VirtualMachine,
    > for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<f64, rustpython_vm::builtins::PyBaseExceptionRef> {
        self.try_into_value(vm)
    }
}
impl
    CdkActTryFromVmValue<
        _CdkFloat64,
        rustpython_vm::builtins::PyBaseExceptionRef,
        &rustpython::vm::VirtualMachine,
    > for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<_CdkFloat64, rustpython_vm::builtins::PyBaseExceptionRef> {
        Ok(_CdkFloat64(self.try_into_value(vm)?))
    }
}
impl
    CdkActTryFromVmValue<
        f32,
        rustpython_vm::builtins::PyBaseExceptionRef,
        &rustpython::vm::VirtualMachine,
    > for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<f32, rustpython_vm::builtins::PyBaseExceptionRef> {
        self.try_into_value(vm)
    }
}
impl
    CdkActTryFromVmValue<
        _CdkFloat32,
        rustpython_vm::builtins::PyBaseExceptionRef,
        &rustpython::vm::VirtualMachine,
    > for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<_CdkFloat32, rustpython_vm::builtins::PyBaseExceptionRef> {
        Ok(_CdkFloat32(self.try_into_value(vm)?))
    }
}
impl
    CdkActTryFromVmValue<
        ic_cdk::export::candid::Int,
        rustpython_vm::builtins::PyBaseExceptionRef,
        &rustpython::vm::VirtualMachine,
    > for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<ic_cdk::export::candid::Int, rustpython_vm::builtins::PyBaseExceptionRef> {
        let int_result: Result<rustpython_vm::builtins::PyIntRef, _> = self.try_into_value(vm);
        match int_result {
            Ok(int) => Ok(ic_cdk::export::candid::Int(int.as_bigint().clone())),
            Err(_) => Err(vm.new_type_error("PyObjectRef is not a PyIntRef".to_string())),
        }
    }
}
impl
    CdkActTryFromVmValue<
        i128,
        rustpython_vm::builtins::PyBaseExceptionRef,
        &rustpython::vm::VirtualMachine,
    > for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<i128, rustpython_vm::builtins::PyBaseExceptionRef> {
        self.try_into_value(vm)
    }
}
impl
    CdkActTryFromVmValue<
        i64,
        rustpython_vm::builtins::PyBaseExceptionRef,
        &rustpython::vm::VirtualMachine,
    > for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<i64, rustpython_vm::builtins::PyBaseExceptionRef> {
        self.try_into_value(vm)
    }
}
impl
    CdkActTryFromVmValue<
        i32,
        rustpython_vm::builtins::PyBaseExceptionRef,
        &rustpython::vm::VirtualMachine,
    > for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<i32, rustpython_vm::builtins::PyBaseExceptionRef> {
        self.try_into_value(vm)
    }
}
impl
    CdkActTryFromVmValue<
        i16,
        rustpython_vm::builtins::PyBaseExceptionRef,
        &rustpython::vm::VirtualMachine,
    > for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<i16, rustpython_vm::builtins::PyBaseExceptionRef> {
        self.try_into_value(vm)
    }
}
impl
    CdkActTryFromVmValue<
        i8,
        rustpython_vm::builtins::PyBaseExceptionRef,
        &rustpython::vm::VirtualMachine,
    > for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<i8, rustpython_vm::builtins::PyBaseExceptionRef> {
        self.try_into_value(vm)
    }
}
impl
    CdkActTryFromVmValue<
        ic_cdk::export::candid::Nat,
        rustpython_vm::builtins::PyBaseExceptionRef,
        &rustpython::vm::VirtualMachine,
    > for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<ic_cdk::export::candid::Nat, rustpython_vm::builtins::PyBaseExceptionRef> {
        let int: rustpython_vm::builtins::PyIntRef = self.try_into_value(vm)?;
        match ic_cdk::export::candid::Nat::from_str(&int.as_bigint().to_string()) {
            Ok(nat) => Ok(nat),
            Err(_) => Err(vm.new_type_error("Could not convert value to nat".to_string())),
        }
    }
}
impl
    CdkActTryFromVmValue<
        u128,
        rustpython_vm::builtins::PyBaseExceptionRef,
        &rustpython::vm::VirtualMachine,
    > for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<u128, rustpython_vm::builtins::PyBaseExceptionRef> {
        self.try_into_value(vm)
    }
}
impl
    CdkActTryFromVmValue<
        u64,
        rustpython_vm::builtins::PyBaseExceptionRef,
        &rustpython::vm::VirtualMachine,
    > for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<u64, rustpython_vm::builtins::PyBaseExceptionRef> {
        self.try_into_value(vm)
    }
}
impl
    CdkActTryFromVmValue<
        usize,
        rustpython_vm::builtins::PyBaseExceptionRef,
        &rustpython::vm::VirtualMachine,
    > for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<usize, rustpython_vm::builtins::PyBaseExceptionRef> {
        self.try_into_value(vm)
    }
}
impl
    CdkActTryFromVmValue<
        u32,
        rustpython_vm::builtins::PyBaseExceptionRef,
        &rustpython::vm::VirtualMachine,
    > for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<u32, rustpython_vm::builtins::PyBaseExceptionRef> {
        self.try_into_value(vm)
    }
}
impl
    CdkActTryFromVmValue<
        u16,
        rustpython_vm::builtins::PyBaseExceptionRef,
        &rustpython::vm::VirtualMachine,
    > for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<u16, rustpython_vm::builtins::PyBaseExceptionRef> {
        self.try_into_value(vm)
    }
}
impl
    CdkActTryFromVmValue<
        u8,
        rustpython_vm::builtins::PyBaseExceptionRef,
        &rustpython::vm::VirtualMachine,
    > for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<u8, rustpython_vm::builtins::PyBaseExceptionRef> {
        self.try_into_value(vm)
    }
}
impl
    CdkActTryFromVmValue<
        Vec<bool>,
        rustpython_vm::builtins::PyBaseExceptionRef,
        &rustpython::vm::VirtualMachine,
    > for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<Vec<bool>, rustpython_vm::builtins::PyBaseExceptionRef> {
        self.try_into_value(vm)
    }
}
impl
    CdkActTryFromVmValue<
        Vec<String>,
        rustpython_vm::builtins::PyBaseExceptionRef,
        &rustpython::vm::VirtualMachine,
    > for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<Vec<String>, rustpython_vm::builtins::PyBaseExceptionRef> {
        self.try_into_value(vm)
    }
}
impl
    CdkActTryFromVmValue<
        Vec<f64>,
        rustpython_vm::builtins::PyBaseExceptionRef,
        &rustpython::vm::VirtualMachine,
    > for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<Vec<f64>, rustpython_vm::builtins::PyBaseExceptionRef> {
        self.try_into_value(vm)
    }
}
impl
    CdkActTryFromVmValue<
        Vec<f32>,
        rustpython_vm::builtins::PyBaseExceptionRef,
        &rustpython::vm::VirtualMachine,
    > for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<Vec<f32>, rustpython_vm::builtins::PyBaseExceptionRef> {
        self.try_into_value(vm)
    }
}
impl
    CdkActTryFromVmValue<
        Vec<i128>,
        rustpython_vm::builtins::PyBaseExceptionRef,
        &rustpython::vm::VirtualMachine,
    > for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<Vec<i128>, rustpython_vm::builtins::PyBaseExceptionRef> {
        self.try_into_value(vm)
    }
}
impl
    CdkActTryFromVmValue<
        Vec<i64>,
        rustpython_vm::builtins::PyBaseExceptionRef,
        &rustpython::vm::VirtualMachine,
    > for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<Vec<i64>, rustpython_vm::builtins::PyBaseExceptionRef> {
        self.try_into_value(vm)
    }
}
impl
    CdkActTryFromVmValue<
        Vec<i32>,
        rustpython_vm::builtins::PyBaseExceptionRef,
        &rustpython::vm::VirtualMachine,
    > for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<Vec<i32>, rustpython_vm::builtins::PyBaseExceptionRef> {
        self.try_into_value(vm)
    }
}
impl
    CdkActTryFromVmValue<
        Vec<i16>,
        rustpython_vm::builtins::PyBaseExceptionRef,
        &rustpython::vm::VirtualMachine,
    > for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<Vec<i16>, rustpython_vm::builtins::PyBaseExceptionRef> {
        self.try_into_value(vm)
    }
}
impl
    CdkActTryFromVmValue<
        Vec<i8>,
        rustpython_vm::builtins::PyBaseExceptionRef,
        &rustpython::vm::VirtualMachine,
    > for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<Vec<i8>, rustpython_vm::builtins::PyBaseExceptionRef> {
        self.try_into_value(vm)
    }
}
impl
    CdkActTryFromVmValue<
        Vec<u128>,
        rustpython_vm::builtins::PyBaseExceptionRef,
        &rustpython::vm::VirtualMachine,
    > for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<Vec<u128>, rustpython_vm::builtins::PyBaseExceptionRef> {
        self.try_into_value(vm)
    }
}
impl
    CdkActTryFromVmValue<
        Vec<u64>,
        rustpython_vm::builtins::PyBaseExceptionRef,
        &rustpython::vm::VirtualMachine,
    > for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<Vec<u64>, rustpython_vm::builtins::PyBaseExceptionRef> {
        self.try_into_value(vm)
    }
}
impl
    CdkActTryFromVmValue<
        Vec<u32>,
        rustpython_vm::builtins::PyBaseExceptionRef,
        &rustpython::vm::VirtualMachine,
    > for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<Vec<u32>, rustpython_vm::builtins::PyBaseExceptionRef> {
        self.try_into_value(vm)
    }
}
impl
    CdkActTryFromVmValue<
        Vec<u16>,
        rustpython_vm::builtins::PyBaseExceptionRef,
        &rustpython::vm::VirtualMachine,
    > for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<Vec<u16>, rustpython_vm::builtins::PyBaseExceptionRef> {
        self.try_into_value(vm)
    }
}
impl
    CdkActTryFromVmValue<
        Vec<u8>,
        rustpython_vm::builtins::PyBaseExceptionRef,
        &rustpython::vm::VirtualMachine,
    > for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<Vec<u8>, rustpython_vm::builtins::PyBaseExceptionRef> {
        self.try_into_value(vm)
    }
}
trait KybraTryFromVec {}
impl<T> KybraTryFromVec for Vec<T> {}
impl<T> KybraTryFromVec for Box<T> {}
impl KybraTryFromVec for () {}
impl<T> KybraTryFromVec for Option<T> {}
impl KybraTryFromVec for ic_cdk::export::candid::Empty {}
impl KybraTryFromVec for ic_cdk::export::candid::Reserved {}
impl KybraTryFromVec for ic_cdk::export::candid::Func {}
impl KybraTryFromVec for ic_cdk::export::Principal {}
impl KybraTryFromVec for ic_cdk_timers::TimerId {}
impl KybraTryFromVec for ic_cdk::export::candid::Int {}
impl KybraTryFromVec for ic_cdk::export::candid::Nat {}
impl KybraTryFromVec for _CdkFloat32 {}
impl KybraTryFromVec for _CdkFloat64 {}
impl<T>
    CdkActTryFromVmValue<
        Vec<T>,
        rustpython_vm::builtins::PyBaseExceptionRef,
        &rustpython::vm::VirtualMachine,
    > for rustpython::vm::PyObjectRef
where
    T: KybraTryFromVec,
    rustpython::vm::PyObjectRef: for<'a> CdkActTryFromVmValue<
        T,
        rustpython_vm::builtins::PyBaseExceptionRef,
        &'a rustpython::vm::VirtualMachine,
    >,
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<Vec<T>, rustpython_vm::builtins::PyBaseExceptionRef> {
        try_from_vm_value_generic_array(self, vm)
    }
}
fn try_from_vm_value_generic_array<T>(
    py_object_ref: rustpython::vm::PyObjectRef,
    vm: &rustpython::vm::VirtualMachine,
) -> Result<Vec<T>, rustpython_vm::builtins::PyBaseExceptionRef>
where
    rustpython::vm::PyObjectRef: for<'a> CdkActTryFromVmValue<
        T,
        rustpython_vm::builtins::PyBaseExceptionRef,
        &'a rustpython::vm::VirtualMachine,
    >,
{
    let py_list: rustpython_vm::builtins::PyListRef = py_object_ref.try_into_value(vm)?;
    let vec = py_list.borrow_vec();
    vec.iter()
        .map(|item| match item.clone().try_from_vm_value(vm) {
            Ok(item) => Ok(item),
            Err(_) => Err(vm.new_type_error("Could not convert value to Vec".to_string())),
        })
        .collect()
}
pub trait CdkActTryFromVmValue<Ok, Err, Context> {
    fn try_from_vm_value(self, context: Context) -> Result<Ok, Err>;
}
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for () {
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        Ok(vm.ctx.none())
    }
}
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for bool {
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        Ok(self.to_pyobject(vm))
    }
}
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef>
    for ic_cdk::export::candid::Empty
{
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        Err(CdkActTryIntoVmValueError(
            "type \"empty\" cannot be represented in python".to_string(),
        ))
    }
}
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef>
    for ic_cdk::export::candid::Func
{
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        let principal = self.principal.try_into_vm_value(vm)?;
        let method = self.method.try_into_vm_value(vm)?;
        Ok(vm.ctx.new_tuple(vec![principal, method]).into())
    }
}
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef>
    for ic_cdk::export::Principal
{
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        let principal_class = vm
            .run_block_expr(
                vm.new_scope_with_builtins(),
                "from kybra import Principal; Principal",
            )
            .map_err(|err| err.to_cdk_act_try_into_vm_value_error(vm))?;
        let from_str = principal_class
            .get_attr("from_str", vm)
            .map_err(|err| err.to_cdk_act_try_into_vm_value_error(vm))?;
        let principal_instance = from_str
            .call((self.to_text(),), vm)
            .map_err(|err| err.to_cdk_act_try_into_vm_value_error(vm))?;
        Ok(principal_instance)
    }
}
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef>
    for ic_cdk::api::call::RejectionCode
{
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        let attribute = match self {
            ic_cdk::api::call::RejectionCode::NoError => "NoError",
            ic_cdk::api::call::RejectionCode::SysFatal => "SysFatal",
            ic_cdk::api::call::RejectionCode::SysTransient => "SysTransient",
            ic_cdk::api::call::RejectionCode::DestinationInvalid => "DestinationInvalid",
            ic_cdk::api::call::RejectionCode::CanisterReject => "CanisterReject",
            ic_cdk::api::call::RejectionCode::CanisterError => "CanisterError",
            ic_cdk::api::call::RejectionCode::Unknown => "Unknown",
        };
        let dict = vm.ctx.new_dict();
        dict.set_item(attribute, vm.ctx.none(), vm);
        Ok(dict.into())
    }
}
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef>
    for ic_cdk::export::candid::Reserved
{
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        Ok(vm.ctx.none())
    }
}
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef>
    for ic_cdk_timers::TimerId
{
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        Ok(self.data().as_ffi().to_pyobject(vm))
    }
}
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef>
    for ic_cdk::api::stable::StableMemoryError
{
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        let attribute = match self {
            ic_cdk::api::stable::StableMemoryError::OutOfMemory => "OutOfMemory",
            ic_cdk::api::stable::StableMemoryError::OutOfBounds => "OutOfBounds",
        };
        let dict = vm.ctx.new_dict();
        dict.set_item(attribute, vm.ctx.none(), vm);
        Ok(dict.into())
    }
}
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for String {
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        Ok(self.to_pyobject(vm))
    }
}
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef>
    for ic_stable_structures::btreemap::InsertError
{
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        match self {
            ic_stable_structures::btreemap::InsertError::KeyTooLarge { given, max } => {
                let dict = vm.ctx.new_dict();
                let key_too_large_dict = vm.ctx.new_dict();
                key_too_large_dict.set_item("given", given.try_into_vm_value(vm)?, vm);
                key_too_large_dict.set_item("max", max.try_into_vm_value(vm)?, vm);
                dict.set_item("KeyTooLarge", key_too_large_dict.into(), vm);
                Ok(dict.into())
            }
            ic_stable_structures::btreemap::InsertError::ValueTooLarge { given, max } => {
                let dict = vm.ctx.new_dict();
                let value_too_large_dict = vm.ctx.new_dict();
                value_too_large_dict.set_item("given", given.try_into_vm_value(vm)?, vm);
                value_too_large_dict.set_item("max", max.try_into_vm_value(vm)?, vm);
                dict.set_item("ValueTooLarge", value_too_large_dict.into(), vm);
                Ok(dict.into())
            }
        }
    }
}
impl<T> CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for (T,)
where
    T: for<'a> CdkActTryIntoVmValue<
        &'a rustpython::vm::VirtualMachine,
        rustpython::vm::PyObjectRef,
    >,
{
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        self.0.try_into_vm_value(vm)
    }
}
impl<T> CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef>
    for Box<T>
where
    T: for<'a> CdkActTryIntoVmValue<
        &'a rustpython::vm::VirtualMachine,
        rustpython::vm::PyObjectRef,
    >,
{
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        (*self).try_into_vm_value(vm)
    }
}
impl<T> CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef>
    for Option<T>
where
    T: for<'a> CdkActTryIntoVmValue<
        &'a rustpython::vm::VirtualMachine,
        rustpython::vm::PyObjectRef,
    >,
{
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        match self {
            Some(value) => value.try_into_vm_value(vm),
            None => Ok(().to_pyobject(vm)),
        }
    }
}
impl<T, K> CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef>
    for Result<T, K>
where
    T: for<'a> CdkActTryIntoVmValue<
        &'a rustpython::vm::VirtualMachine,
        rustpython::vm::PyObjectRef,
    >,
    K: for<'a> CdkActTryIntoVmValue<
        &'a rustpython::vm::VirtualMachine,
        rustpython::vm::PyObjectRef,
    >,
{
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        match self {
            Ok(ok) => {
                let dict = vm.ctx.new_dict();
                dict.set_item("Ok", ok.try_into_vm_value(vm)?, vm);
                Ok(dict.into())
            }
            Err(err) => {
                let dict = vm.ctx.new_dict();
                dict.set_item("Err", err.try_into_vm_value(vm)?, vm);
                Ok(dict.into())
            }
        }
    }
}
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for f64 {
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        Ok(self.to_pyobject(vm))
    }
}
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef>
    for _CdkFloat64
{
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        Ok(self.0.to_pyobject(vm))
    }
}
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for f32 {
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        Ok(self.to_pyobject(vm))
    }
}
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef>
    for _CdkFloat32
{
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        Ok(self.0.to_pyobject(vm))
    }
}
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef>
    for ic_cdk::export::candid::Int
{
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        Ok(vm.ctx.new_int(self.0).into())
    }
}
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for i128 {
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        Ok(self.to_pyobject(vm))
    }
}
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for i64 {
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        Ok(self.to_pyobject(vm))
    }
}
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for i32 {
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        Ok(self.to_pyobject(vm))
    }
}
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for i16 {
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        Ok(self.to_pyobject(vm))
    }
}
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for i8 {
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        Ok(self.to_pyobject(vm))
    }
}
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef>
    for ic_cdk::export::candid::Nat
{
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        Ok(vm.ctx.new_int(self.0).into())
    }
}
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for u128 {
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        Ok(self.to_pyobject(vm))
    }
}
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for u64 {
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        Ok(self.to_pyobject(vm))
    }
}
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for usize {
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        Ok(self.to_pyobject(vm))
    }
}
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for u32 {
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        Ok(self.to_pyobject(vm))
    }
}
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for u16 {
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        Ok(self.to_pyobject(vm))
    }
}
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef> for u8 {
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        Ok(self.to_pyobject(vm))
    }
}
trait KybraTryIntoVec {}
impl KybraTryIntoVec for () {}
impl KybraTryIntoVec for bool {}
impl KybraTryIntoVec for String {}
impl KybraTryIntoVec for ic_cdk::export::candid::Empty {}
impl KybraTryIntoVec for ic_cdk::export::candid::Reserved {}
impl KybraTryIntoVec for ic_cdk::export::candid::Func {}
impl KybraTryIntoVec for ic_cdk::export::Principal {}
impl KybraTryIntoVec for ic_cdk_timers::TimerId {}
impl KybraTryIntoVec for ic_cdk::api::call::RejectionCode {}
impl KybraTryIntoVec for f64 {}
impl KybraTryIntoVec for f32 {}
impl KybraTryIntoVec for _CdkFloat64 {}
impl KybraTryIntoVec for _CdkFloat32 {}
impl KybraTryIntoVec for ic_cdk::export::candid::Int {}
impl KybraTryIntoVec for i128 {}
impl KybraTryIntoVec for i64 {}
impl KybraTryIntoVec for i32 {}
impl KybraTryIntoVec for i16 {}
impl KybraTryIntoVec for i8 {}
impl KybraTryIntoVec for ic_cdk::export::candid::Nat {}
impl KybraTryIntoVec for u128 {}
impl KybraTryIntoVec for u64 {}
impl KybraTryIntoVec for usize {}
impl KybraTryIntoVec for u32 {}
impl KybraTryIntoVec for u16 {}
impl<T> KybraTryIntoVec for Option<T> {}
impl<T> KybraTryIntoVec for Box<T> {}
impl<T> KybraTryIntoVec for Vec<T> {}
impl<T> CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef>
    for Vec<T>
where
    T: KybraTryIntoVec,
    T: for<'a> CdkActTryIntoVmValue<
        &'a rustpython::vm::VirtualMachine,
        rustpython::vm::PyObjectRef,
    >,
{
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        try_into_vm_value_generic_array(self, vm)
    }
}
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef>
    for Vec<u8>
{
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        Ok(vm.ctx.new_bytes(self).into())
    }
}
fn try_into_vm_value_generic_array<T>(
    generic_array: Vec<T>,
    vm: &rustpython::vm::VirtualMachine,
) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError>
where
    T: for<'a> CdkActTryIntoVmValue<
        &'a rustpython::vm::VirtualMachine,
        rustpython::vm::PyObjectRef,
    >,
{
    let py_object_refs_result: Result<Vec<rustpython_vm::PyObjectRef>, CdkActTryIntoVmValueError> =
        generic_array
            .into_iter()
            .map(|item| item.try_into_vm_value(vm))
            .collect();
    Ok(vm.ctx.new_list(py_object_refs_result?).into())
}
# [async_recursion :: async_recursion (? Send)]
async fn async_result_handler(
    vm: &rustpython::vm::VirtualMachine,
    py_object_ref: &rustpython::vm::PyObjectRef,
    arg: rustpython_vm::PyObjectRef,
) -> rustpython::vm::PyResult {
    if is_generator(vm, &py_object_ref) == false {
        return Ok(py_object_ref.clone());
    }
    let send_result = vm.call_method(&py_object_ref, "send", (arg.clone(),));
    let py_iter_return = rustpython_vm::protocol::PyIterReturn::from_pyresult(send_result, vm)?;
    match py_iter_return {
        rustpython_vm::protocol::PyIterReturn::Return(returned_py_object_ref) => {
            if is_generator(vm, &returned_py_object_ref) == true {
                let recursed_py_object_ref =
                    async_result_handler(vm, &returned_py_object_ref, vm.ctx.none()).await?;
                return async_result_handler(vm, py_object_ref, recursed_py_object_ref).await;
            }
            let name: String = returned_py_object_ref
                .get_attr("name", vm)?
                .try_from_vm_value(vm)?;
            let args: Vec<rustpython_vm::PyObjectRef> = returned_py_object_ref
                .get_attr("args", vm)?
                .try_into_value(vm)?;
            match &name[..] {
                "call" => async_result_handler_call(vm, py_object_ref, &args).await,
                "call_with_payment" => {
                    async_result_handler_call_with_payment(vm, py_object_ref, &args).await
                }
                "call_with_payment128" => {
                    async_result_handler_call_with_payment128(vm, py_object_ref, &args).await
                }
                "call_raw" => async_result_handler_call_raw(vm, py_object_ref, &args).await,
                "call_raw128" => async_result_handler_call_raw128(vm, py_object_ref, &args).await,
                _ => {
                    return Err(
                        vm.new_system_error(format!("async operation '{}' not supported", name))
                    )
                }
            }
        }
        rustpython_vm::protocol::PyIterReturn::StopIteration(returned_py_object_ref_option) => {
            let return_value: rustpython_vm::PyObjectRef = match returned_py_object_ref_option {
                Some(returned_py_object_ref) => returned_py_object_ref,
                None => vm.ctx.none(),
            };
            Ok(return_value)
        }
    }
}
fn is_generator(
    vm: &rustpython::vm::VirtualMachine,
    py_object_ref: &rustpython_vm::PyObjectRef,
) -> bool {
    if let Ok(_) = py_object_ref.get_attr("send", vm) {
        true
    } else {
        false
    }
}
async fn async_result_handler_call(
    vm: &rustpython::vm::VirtualMachine,
    py_object_ref: &rustpython_vm::PyObjectRef,
    args: &Vec<rustpython_vm::PyObjectRef>,
) -> rustpython_vm::PyResult {
    let canister_id_principal: ic_cdk::export::Principal = args[0].clone().try_from_vm_value(vm)?;
    let qual_name: String = args[1].clone().try_from_vm_value(vm)?;
    let cross_canister_call_function_name = format!("call_{}", qual_name.replace(".", "_"));
    let call_result_instance = match &cross_canister_call_function_name[..] {
        "call_ManagementCanister_bitcoin_get_balance" => {
            let canister_id_principal: ic_cdk::export::Principal =
                args[0].clone().try_from_vm_value(vm)?;
            let _cdk_user_defined_args: (GetBalanceArgs) =
                args[2usize].clone().try_from_vm_value(vm)?;
            create_call_result_instance(
                vm,
                call_ManagementCanister_bitcoin_get_balance(
                    canister_id_principal,
                    (_cdk_user_defined_args,),
                )
                .await,
            )?
        }
        "call_ManagementCanister_bitcoin_get_current_fee_percentiles" => {
            let canister_id_principal: ic_cdk::export::Principal =
                args[0].clone().try_from_vm_value(vm)?;
            let _cdk_user_defined_args: (GetCurrentFeePercentilesArgs) =
                args[2usize].clone().try_from_vm_value(vm)?;
            create_call_result_instance(
                vm,
                call_ManagementCanister_bitcoin_get_current_fee_percentiles(
                    canister_id_principal,
                    (_cdk_user_defined_args,),
                )
                .await,
            )?
        }
        "call_ManagementCanister_bitcoin_get_utxos" => {
            let canister_id_principal: ic_cdk::export::Principal =
                args[0].clone().try_from_vm_value(vm)?;
            let _cdk_user_defined_args: (GetUtxosArgs) =
                args[2usize].clone().try_from_vm_value(vm)?;
            create_call_result_instance(
                vm,
                call_ManagementCanister_bitcoin_get_utxos(
                    canister_id_principal,
                    (_cdk_user_defined_args,),
                )
                .await,
            )?
        }
        "call_ManagementCanister_bitcoin_send_transaction" => {
            let canister_id_principal: ic_cdk::export::Principal =
                args[0].clone().try_from_vm_value(vm)?;
            let _cdk_user_defined_args: (SendTransactionArgs) =
                args[2usize].clone().try_from_vm_value(vm)?;
            create_call_result_instance(
                vm,
                call_ManagementCanister_bitcoin_send_transaction(
                    canister_id_principal,
                    (_cdk_user_defined_args,),
                )
                .await,
            )?
        }
        "call_ManagementCanister_create_canister" => {
            let canister_id_principal: ic_cdk::export::Principal =
                args[0].clone().try_from_vm_value(vm)?;
            let _cdk_user_defined_args: (CreateCanisterArgs) =
                args[2usize].clone().try_from_vm_value(vm)?;
            create_call_result_instance(
                vm,
                call_ManagementCanister_create_canister(
                    canister_id_principal,
                    (_cdk_user_defined_args,),
                )
                .await,
            )?
        }
        "call_ManagementCanister_update_settings" => {
            let canister_id_principal: ic_cdk::export::Principal =
                args[0].clone().try_from_vm_value(vm)?;
            let _cdk_user_defined_args: (UpdateSettingsArgs) =
                args[2usize].clone().try_from_vm_value(vm)?;
            create_call_result_instance(
                vm,
                call_ManagementCanister_update_settings(
                    canister_id_principal,
                    (_cdk_user_defined_args,),
                )
                .await,
            )?
        }
        "call_ManagementCanister_install_code" => {
            let canister_id_principal: ic_cdk::export::Principal =
                args[0].clone().try_from_vm_value(vm)?;
            let _cdk_user_defined_args: (InstallCodeArgs) =
                args[2usize].clone().try_from_vm_value(vm)?;
            create_call_result_instance(
                vm,
                call_ManagementCanister_install_code(
                    canister_id_principal,
                    (_cdk_user_defined_args,),
                )
                .await,
            )?
        }
        "call_ManagementCanister_uninstall_code" => {
            let canister_id_principal: ic_cdk::export::Principal =
                args[0].clone().try_from_vm_value(vm)?;
            let _cdk_user_defined_args: (UninstallCodeArgs) =
                args[2usize].clone().try_from_vm_value(vm)?;
            create_call_result_instance(
                vm,
                call_ManagementCanister_uninstall_code(
                    canister_id_principal,
                    (_cdk_user_defined_args,),
                )
                .await,
            )?
        }
        "call_ManagementCanister_start_canister" => {
            let canister_id_principal: ic_cdk::export::Principal =
                args[0].clone().try_from_vm_value(vm)?;
            let _cdk_user_defined_args: (StartCanisterArgs) =
                args[2usize].clone().try_from_vm_value(vm)?;
            create_call_result_instance(
                vm,
                call_ManagementCanister_start_canister(
                    canister_id_principal,
                    (_cdk_user_defined_args,),
                )
                .await,
            )?
        }
        "call_ManagementCanister_stop_canister" => {
            let canister_id_principal: ic_cdk::export::Principal =
                args[0].clone().try_from_vm_value(vm)?;
            let _cdk_user_defined_args: (StopCanisterArgs) =
                args[2usize].clone().try_from_vm_value(vm)?;
            create_call_result_instance(
                vm,
                call_ManagementCanister_stop_canister(
                    canister_id_principal,
                    (_cdk_user_defined_args,),
                )
                .await,
            )?
        }
        "call_ManagementCanister_canister_status" => {
            let canister_id_principal: ic_cdk::export::Principal =
                args[0].clone().try_from_vm_value(vm)?;
            let _cdk_user_defined_args: (CanisterStatusArgs) =
                args[2usize].clone().try_from_vm_value(vm)?;
            create_call_result_instance(
                vm,
                call_ManagementCanister_canister_status(
                    canister_id_principal,
                    (_cdk_user_defined_args,),
                )
                .await,
            )?
        }
        "call_ManagementCanister_delete_canister" => {
            let canister_id_principal: ic_cdk::export::Principal =
                args[0].clone().try_from_vm_value(vm)?;
            let _cdk_user_defined_args: (DeleteCanisterArgs) =
                args[2usize].clone().try_from_vm_value(vm)?;
            create_call_result_instance(
                vm,
                call_ManagementCanister_delete_canister(
                    canister_id_principal,
                    (_cdk_user_defined_args,),
                )
                .await,
            )?
        }
        "call_ManagementCanister_deposit_cycles" => {
            let canister_id_principal: ic_cdk::export::Principal =
                args[0].clone().try_from_vm_value(vm)?;
            let _cdk_user_defined_args: (DepositCyclesArgs) =
                args[2usize].clone().try_from_vm_value(vm)?;
            create_call_result_instance(
                vm,
                call_ManagementCanister_deposit_cycles(
                    canister_id_principal,
                    (_cdk_user_defined_args,),
                )
                .await,
            )?
        }
        "call_ManagementCanister_raw_rand" => {
            let canister_id_principal: ic_cdk::export::Principal =
                args[0].clone().try_from_vm_value(vm)?;
            create_call_result_instance(
                vm,
                call_ManagementCanister_raw_rand(canister_id_principal, ()).await,
            )?
        }
        "call_ManagementCanister_provisional_create_canister_with_cycles" => {
            let canister_id_principal: ic_cdk::export::Principal =
                args[0].clone().try_from_vm_value(vm)?;
            let _cdk_user_defined_args: (ProvisionalCreateCanisterWithCyclesArgs) =
                args[2usize].clone().try_from_vm_value(vm)?;
            create_call_result_instance(
                vm,
                call_ManagementCanister_provisional_create_canister_with_cycles(
                    canister_id_principal,
                    (_cdk_user_defined_args,),
                )
                .await,
            )?
        }
        "call_ManagementCanister_provisional_top_up_canister" => {
            let canister_id_principal: ic_cdk::export::Principal =
                args[0].clone().try_from_vm_value(vm)?;
            let _cdk_user_defined_args: (ProvisionalTopUpCanisterArgs) =
                args[2usize].clone().try_from_vm_value(vm)?;
            create_call_result_instance(
                vm,
                call_ManagementCanister_provisional_top_up_canister(
                    canister_id_principal,
                    (_cdk_user_defined_args,),
                )
                .await,
            )?
        }
        "call_ManagementCanister_http_request" => {
            let canister_id_principal: ic_cdk::export::Principal =
                args[0].clone().try_from_vm_value(vm)?;
            let _cdk_user_defined_args: (HttpRequestArgs) =
                args[2usize].clone().try_from_vm_value(vm)?;
            create_call_result_instance(
                vm,
                call_ManagementCanister_http_request(
                    canister_id_principal,
                    (_cdk_user_defined_args,),
                )
                .await,
            )?
        }
        "call_ManagementCanister_ecdsa_public_key" => {
            let canister_id_principal: ic_cdk::export::Principal =
                args[0].clone().try_from_vm_value(vm)?;
            let _cdk_user_defined_args: (EcdsaPublicKeyArgs) =
                args[2usize].clone().try_from_vm_value(vm)?;
            create_call_result_instance(
                vm,
                call_ManagementCanister_ecdsa_public_key(
                    canister_id_principal,
                    (_cdk_user_defined_args,),
                )
                .await,
            )?
        }
        "call_ManagementCanister_sign_with_ecdsa" => {
            let canister_id_principal: ic_cdk::export::Principal =
                args[0].clone().try_from_vm_value(vm)?;
            let _cdk_user_defined_args: (SignWithEcdsaArgs) =
                args[2usize].clone().try_from_vm_value(vm)?;
            create_call_result_instance(
                vm,
                call_ManagementCanister_sign_with_ecdsa(
                    canister_id_principal,
                    (_cdk_user_defined_args,),
                )
                .await,
            )?
        }
        _ => {
            return Err(vm.new_attribute_error(format!(
                "canister '{}' has no attribute '{}'",
                canister_id_principal, qual_name
            )))
        }
    };
    async_result_handler(vm, py_object_ref, call_result_instance).await
}
async fn async_result_handler_call_with_payment(
    vm: &rustpython::vm::VirtualMachine,
    py_object_ref: &rustpython_vm::PyObjectRef,
    args: &Vec<rustpython_vm::PyObjectRef>,
) -> rustpython_vm::PyResult {
    let canister_id_principal: ic_cdk::export::Principal = args[0].clone().try_from_vm_value(vm)?;
    let qual_name: String = args[1].clone().try_from_vm_value(vm)?;
    let cross_canister_call_with_payment_function_name =
        format!("call_with_payment_{}", qual_name.replace(".", "_"));
    let call_result_instance = match &cross_canister_call_with_payment_function_name[..] {
        "call_with_payment_ManagementCanister_bitcoin_get_balance" => {
            let canister_id_principal: ic_cdk::export::Principal =
                args[0].clone().try_from_vm_value(vm)?;
            let _cdk_user_defined_args: (GetBalanceArgs) =
                args[2usize].clone().try_from_vm_value(vm)?;
            let payment: u64 = args[3usize].clone().try_from_vm_value(vm)?;
            create_call_result_instance(
                vm,
                call_with_payment_ManagementCanister_bitcoin_get_balance(
                    canister_id_principal,
                    (_cdk_user_defined_args,),
                    payment,
                )
                .await,
            )?
        }
        "call_with_payment_ManagementCanister_bitcoin_get_current_fee_percentiles" => {
            let canister_id_principal: ic_cdk::export::Principal =
                args[0].clone().try_from_vm_value(vm)?;
            let _cdk_user_defined_args: (GetCurrentFeePercentilesArgs) =
                args[2usize].clone().try_from_vm_value(vm)?;
            let payment: u64 = args[3usize].clone().try_from_vm_value(vm)?;
            create_call_result_instance(
                vm,
                call_with_payment_ManagementCanister_bitcoin_get_current_fee_percentiles(
                    canister_id_principal,
                    (_cdk_user_defined_args,),
                    payment,
                )
                .await,
            )?
        }
        "call_with_payment_ManagementCanister_bitcoin_get_utxos" => {
            let canister_id_principal: ic_cdk::export::Principal =
                args[0].clone().try_from_vm_value(vm)?;
            let _cdk_user_defined_args: (GetUtxosArgs) =
                args[2usize].clone().try_from_vm_value(vm)?;
            let payment: u64 = args[3usize].clone().try_from_vm_value(vm)?;
            create_call_result_instance(
                vm,
                call_with_payment_ManagementCanister_bitcoin_get_utxos(
                    canister_id_principal,
                    (_cdk_user_defined_args,),
                    payment,
                )
                .await,
            )?
        }
        "call_with_payment_ManagementCanister_bitcoin_send_transaction" => {
            let canister_id_principal: ic_cdk::export::Principal =
                args[0].clone().try_from_vm_value(vm)?;
            let _cdk_user_defined_args: (SendTransactionArgs) =
                args[2usize].clone().try_from_vm_value(vm)?;
            let payment: u64 = args[3usize].clone().try_from_vm_value(vm)?;
            create_call_result_instance(
                vm,
                call_with_payment_ManagementCanister_bitcoin_send_transaction(
                    canister_id_principal,
                    (_cdk_user_defined_args,),
                    payment,
                )
                .await,
            )?
        }
        "call_with_payment_ManagementCanister_create_canister" => {
            let canister_id_principal: ic_cdk::export::Principal =
                args[0].clone().try_from_vm_value(vm)?;
            let _cdk_user_defined_args: (CreateCanisterArgs) =
                args[2usize].clone().try_from_vm_value(vm)?;
            let payment: u64 = args[3usize].clone().try_from_vm_value(vm)?;
            create_call_result_instance(
                vm,
                call_with_payment_ManagementCanister_create_canister(
                    canister_id_principal,
                    (_cdk_user_defined_args,),
                    payment,
                )
                .await,
            )?
        }
        "call_with_payment_ManagementCanister_update_settings" => {
            let canister_id_principal: ic_cdk::export::Principal =
                args[0].clone().try_from_vm_value(vm)?;
            let _cdk_user_defined_args: (UpdateSettingsArgs) =
                args[2usize].clone().try_from_vm_value(vm)?;
            let payment: u64 = args[3usize].clone().try_from_vm_value(vm)?;
            create_call_result_instance(
                vm,
                call_with_payment_ManagementCanister_update_settings(
                    canister_id_principal,
                    (_cdk_user_defined_args,),
                    payment,
                )
                .await,
            )?
        }
        "call_with_payment_ManagementCanister_install_code" => {
            let canister_id_principal: ic_cdk::export::Principal =
                args[0].clone().try_from_vm_value(vm)?;
            let _cdk_user_defined_args: (InstallCodeArgs) =
                args[2usize].clone().try_from_vm_value(vm)?;
            let payment: u64 = args[3usize].clone().try_from_vm_value(vm)?;
            create_call_result_instance(
                vm,
                call_with_payment_ManagementCanister_install_code(
                    canister_id_principal,
                    (_cdk_user_defined_args,),
                    payment,
                )
                .await,
            )?
        }
        "call_with_payment_ManagementCanister_uninstall_code" => {
            let canister_id_principal: ic_cdk::export::Principal =
                args[0].clone().try_from_vm_value(vm)?;
            let _cdk_user_defined_args: (UninstallCodeArgs) =
                args[2usize].clone().try_from_vm_value(vm)?;
            let payment: u64 = args[3usize].clone().try_from_vm_value(vm)?;
            create_call_result_instance(
                vm,
                call_with_payment_ManagementCanister_uninstall_code(
                    canister_id_principal,
                    (_cdk_user_defined_args,),
                    payment,
                )
                .await,
            )?
        }
        "call_with_payment_ManagementCanister_start_canister" => {
            let canister_id_principal: ic_cdk::export::Principal =
                args[0].clone().try_from_vm_value(vm)?;
            let _cdk_user_defined_args: (StartCanisterArgs) =
                args[2usize].clone().try_from_vm_value(vm)?;
            let payment: u64 = args[3usize].clone().try_from_vm_value(vm)?;
            create_call_result_instance(
                vm,
                call_with_payment_ManagementCanister_start_canister(
                    canister_id_principal,
                    (_cdk_user_defined_args,),
                    payment,
                )
                .await,
            )?
        }
        "call_with_payment_ManagementCanister_stop_canister" => {
            let canister_id_principal: ic_cdk::export::Principal =
                args[0].clone().try_from_vm_value(vm)?;
            let _cdk_user_defined_args: (StopCanisterArgs) =
                args[2usize].clone().try_from_vm_value(vm)?;
            let payment: u64 = args[3usize].clone().try_from_vm_value(vm)?;
            create_call_result_instance(
                vm,
                call_with_payment_ManagementCanister_stop_canister(
                    canister_id_principal,
                    (_cdk_user_defined_args,),
                    payment,
                )
                .await,
            )?
        }
        "call_with_payment_ManagementCanister_canister_status" => {
            let canister_id_principal: ic_cdk::export::Principal =
                args[0].clone().try_from_vm_value(vm)?;
            let _cdk_user_defined_args: (CanisterStatusArgs) =
                args[2usize].clone().try_from_vm_value(vm)?;
            let payment: u64 = args[3usize].clone().try_from_vm_value(vm)?;
            create_call_result_instance(
                vm,
                call_with_payment_ManagementCanister_canister_status(
                    canister_id_principal,
                    (_cdk_user_defined_args,),
                    payment,
                )
                .await,
            )?
        }
        "call_with_payment_ManagementCanister_delete_canister" => {
            let canister_id_principal: ic_cdk::export::Principal =
                args[0].clone().try_from_vm_value(vm)?;
            let _cdk_user_defined_args: (DeleteCanisterArgs) =
                args[2usize].clone().try_from_vm_value(vm)?;
            let payment: u64 = args[3usize].clone().try_from_vm_value(vm)?;
            create_call_result_instance(
                vm,
                call_with_payment_ManagementCanister_delete_canister(
                    canister_id_principal,
                    (_cdk_user_defined_args,),
                    payment,
                )
                .await,
            )?
        }
        "call_with_payment_ManagementCanister_deposit_cycles" => {
            let canister_id_principal: ic_cdk::export::Principal =
                args[0].clone().try_from_vm_value(vm)?;
            let _cdk_user_defined_args: (DepositCyclesArgs) =
                args[2usize].clone().try_from_vm_value(vm)?;
            let payment: u64 = args[3usize].clone().try_from_vm_value(vm)?;
            create_call_result_instance(
                vm,
                call_with_payment_ManagementCanister_deposit_cycles(
                    canister_id_principal,
                    (_cdk_user_defined_args,),
                    payment,
                )
                .await,
            )?
        }
        "call_with_payment_ManagementCanister_raw_rand" => {
            let canister_id_principal: ic_cdk::export::Principal =
                args[0].clone().try_from_vm_value(vm)?;
            let payment: u64 = args[2usize].clone().try_from_vm_value(vm)?;
            create_call_result_instance(
                vm,
                call_with_payment_ManagementCanister_raw_rand(canister_id_principal, (), payment)
                    .await,
            )?
        }
        "call_with_payment_ManagementCanister_provisional_create_canister_with_cycles" => {
            let canister_id_principal: ic_cdk::export::Principal =
                args[0].clone().try_from_vm_value(vm)?;
            let _cdk_user_defined_args: (ProvisionalCreateCanisterWithCyclesArgs) =
                args[2usize].clone().try_from_vm_value(vm)?;
            let payment: u64 = args[3usize].clone().try_from_vm_value(vm)?;
            create_call_result_instance(
                vm,
                call_with_payment_ManagementCanister_provisional_create_canister_with_cycles(
                    canister_id_principal,
                    (_cdk_user_defined_args,),
                    payment,
                )
                .await,
            )?
        }
        "call_with_payment_ManagementCanister_provisional_top_up_canister" => {
            let canister_id_principal: ic_cdk::export::Principal =
                args[0].clone().try_from_vm_value(vm)?;
            let _cdk_user_defined_args: (ProvisionalTopUpCanisterArgs) =
                args[2usize].clone().try_from_vm_value(vm)?;
            let payment: u64 = args[3usize].clone().try_from_vm_value(vm)?;
            create_call_result_instance(
                vm,
                call_with_payment_ManagementCanister_provisional_top_up_canister(
                    canister_id_principal,
                    (_cdk_user_defined_args,),
                    payment,
                )
                .await,
            )?
        }
        "call_with_payment_ManagementCanister_http_request" => {
            let canister_id_principal: ic_cdk::export::Principal =
                args[0].clone().try_from_vm_value(vm)?;
            let _cdk_user_defined_args: (HttpRequestArgs) =
                args[2usize].clone().try_from_vm_value(vm)?;
            let payment: u64 = args[3usize].clone().try_from_vm_value(vm)?;
            create_call_result_instance(
                vm,
                call_with_payment_ManagementCanister_http_request(
                    canister_id_principal,
                    (_cdk_user_defined_args,),
                    payment,
                )
                .await,
            )?
        }
        "call_with_payment_ManagementCanister_ecdsa_public_key" => {
            let canister_id_principal: ic_cdk::export::Principal =
                args[0].clone().try_from_vm_value(vm)?;
            let _cdk_user_defined_args: (EcdsaPublicKeyArgs) =
                args[2usize].clone().try_from_vm_value(vm)?;
            let payment: u64 = args[3usize].clone().try_from_vm_value(vm)?;
            create_call_result_instance(
                vm,
                call_with_payment_ManagementCanister_ecdsa_public_key(
                    canister_id_principal,
                    (_cdk_user_defined_args,),
                    payment,
                )
                .await,
            )?
        }
        "call_with_payment_ManagementCanister_sign_with_ecdsa" => {
            let canister_id_principal: ic_cdk::export::Principal =
                args[0].clone().try_from_vm_value(vm)?;
            let _cdk_user_defined_args: (SignWithEcdsaArgs) =
                args[2usize].clone().try_from_vm_value(vm)?;
            let payment: u64 = args[3usize].clone().try_from_vm_value(vm)?;
            create_call_result_instance(
                vm,
                call_with_payment_ManagementCanister_sign_with_ecdsa(
                    canister_id_principal,
                    (_cdk_user_defined_args,),
                    payment,
                )
                .await,
            )?
        }
        _ => {
            return Err(vm.new_attribute_error(format!(
                "canister '{}' has no attribute '{}'",
                canister_id_principal, qual_name
            )))
        }
    };
    async_result_handler(vm, py_object_ref, call_result_instance).await
}
async fn async_result_handler_call_with_payment128(
    vm: &rustpython::vm::VirtualMachine,
    py_object_ref: &rustpython_vm::PyObjectRef,
    args: &Vec<rustpython_vm::PyObjectRef>,
) -> rustpython_vm::PyResult {
    let canister_id_principal: ic_cdk::export::Principal = args[0].clone().try_from_vm_value(vm)?;
    let qual_name: String = args[1].clone().try_from_vm_value(vm)?;
    let cross_canister_call_with_payment128_function_name =
        format!("call_with_payment128_{}", qual_name.replace(".", "_"));
    let call_result_instance = match &cross_canister_call_with_payment128_function_name[..] {
        "call_with_payment128_ManagementCanister_bitcoin_get_balance" => {
            let canister_id_principal: ic_cdk::export::Principal =
                args[0].clone().try_from_vm_value(vm)?;
            let _cdk_user_defined_args: (GetBalanceArgs) =
                args[2usize].clone().try_from_vm_value(vm)?;
            let payment: u128 = args[3usize].clone().try_from_vm_value(vm)?;
            create_call_result_instance(
                vm,
                call_with_payment128_ManagementCanister_bitcoin_get_balance(
                    canister_id_principal,
                    (_cdk_user_defined_args,),
                    payment,
                )
                .await,
            )?
        }
        "call_with_payment128_ManagementCanister_bitcoin_get_current_fee_percentiles" => {
            let canister_id_principal: ic_cdk::export::Principal =
                args[0].clone().try_from_vm_value(vm)?;
            let _cdk_user_defined_args: (GetCurrentFeePercentilesArgs) =
                args[2usize].clone().try_from_vm_value(vm)?;
            let payment: u128 = args[3usize].clone().try_from_vm_value(vm)?;
            create_call_result_instance(
                vm,
                call_with_payment128_ManagementCanister_bitcoin_get_current_fee_percentiles(
                    canister_id_principal,
                    (_cdk_user_defined_args,),
                    payment,
                )
                .await,
            )?
        }
        "call_with_payment128_ManagementCanister_bitcoin_get_utxos" => {
            let canister_id_principal: ic_cdk::export::Principal =
                args[0].clone().try_from_vm_value(vm)?;
            let _cdk_user_defined_args: (GetUtxosArgs) =
                args[2usize].clone().try_from_vm_value(vm)?;
            let payment: u128 = args[3usize].clone().try_from_vm_value(vm)?;
            create_call_result_instance(
                vm,
                call_with_payment128_ManagementCanister_bitcoin_get_utxos(
                    canister_id_principal,
                    (_cdk_user_defined_args,),
                    payment,
                )
                .await,
            )?
        }
        "call_with_payment128_ManagementCanister_bitcoin_send_transaction" => {
            let canister_id_principal: ic_cdk::export::Principal =
                args[0].clone().try_from_vm_value(vm)?;
            let _cdk_user_defined_args: (SendTransactionArgs) =
                args[2usize].clone().try_from_vm_value(vm)?;
            let payment: u128 = args[3usize].clone().try_from_vm_value(vm)?;
            create_call_result_instance(
                vm,
                call_with_payment128_ManagementCanister_bitcoin_send_transaction(
                    canister_id_principal,
                    (_cdk_user_defined_args,),
                    payment,
                )
                .await,
            )?
        }
        "call_with_payment128_ManagementCanister_create_canister" => {
            let canister_id_principal: ic_cdk::export::Principal =
                args[0].clone().try_from_vm_value(vm)?;
            let _cdk_user_defined_args: (CreateCanisterArgs) =
                args[2usize].clone().try_from_vm_value(vm)?;
            let payment: u128 = args[3usize].clone().try_from_vm_value(vm)?;
            create_call_result_instance(
                vm,
                call_with_payment128_ManagementCanister_create_canister(
                    canister_id_principal,
                    (_cdk_user_defined_args,),
                    payment,
                )
                .await,
            )?
        }
        "call_with_payment128_ManagementCanister_update_settings" => {
            let canister_id_principal: ic_cdk::export::Principal =
                args[0].clone().try_from_vm_value(vm)?;
            let _cdk_user_defined_args: (UpdateSettingsArgs) =
                args[2usize].clone().try_from_vm_value(vm)?;
            let payment: u128 = args[3usize].clone().try_from_vm_value(vm)?;
            create_call_result_instance(
                vm,
                call_with_payment128_ManagementCanister_update_settings(
                    canister_id_principal,
                    (_cdk_user_defined_args,),
                    payment,
                )
                .await,
            )?
        }
        "call_with_payment128_ManagementCanister_install_code" => {
            let canister_id_principal: ic_cdk::export::Principal =
                args[0].clone().try_from_vm_value(vm)?;
            let _cdk_user_defined_args: (InstallCodeArgs) =
                args[2usize].clone().try_from_vm_value(vm)?;
            let payment: u128 = args[3usize].clone().try_from_vm_value(vm)?;
            create_call_result_instance(
                vm,
                call_with_payment128_ManagementCanister_install_code(
                    canister_id_principal,
                    (_cdk_user_defined_args,),
                    payment,
                )
                .await,
            )?
        }
        "call_with_payment128_ManagementCanister_uninstall_code" => {
            let canister_id_principal: ic_cdk::export::Principal =
                args[0].clone().try_from_vm_value(vm)?;
            let _cdk_user_defined_args: (UninstallCodeArgs) =
                args[2usize].clone().try_from_vm_value(vm)?;
            let payment: u128 = args[3usize].clone().try_from_vm_value(vm)?;
            create_call_result_instance(
                vm,
                call_with_payment128_ManagementCanister_uninstall_code(
                    canister_id_principal,
                    (_cdk_user_defined_args,),
                    payment,
                )
                .await,
            )?
        }
        "call_with_payment128_ManagementCanister_start_canister" => {
            let canister_id_principal: ic_cdk::export::Principal =
                args[0].clone().try_from_vm_value(vm)?;
            let _cdk_user_defined_args: (StartCanisterArgs) =
                args[2usize].clone().try_from_vm_value(vm)?;
            let payment: u128 = args[3usize].clone().try_from_vm_value(vm)?;
            create_call_result_instance(
                vm,
                call_with_payment128_ManagementCanister_start_canister(
                    canister_id_principal,
                    (_cdk_user_defined_args,),
                    payment,
                )
                .await,
            )?
        }
        "call_with_payment128_ManagementCanister_stop_canister" => {
            let canister_id_principal: ic_cdk::export::Principal =
                args[0].clone().try_from_vm_value(vm)?;
            let _cdk_user_defined_args: (StopCanisterArgs) =
                args[2usize].clone().try_from_vm_value(vm)?;
            let payment: u128 = args[3usize].clone().try_from_vm_value(vm)?;
            create_call_result_instance(
                vm,
                call_with_payment128_ManagementCanister_stop_canister(
                    canister_id_principal,
                    (_cdk_user_defined_args,),
                    payment,
                )
                .await,
            )?
        }
        "call_with_payment128_ManagementCanister_canister_status" => {
            let canister_id_principal: ic_cdk::export::Principal =
                args[0].clone().try_from_vm_value(vm)?;
            let _cdk_user_defined_args: (CanisterStatusArgs) =
                args[2usize].clone().try_from_vm_value(vm)?;
            let payment: u128 = args[3usize].clone().try_from_vm_value(vm)?;
            create_call_result_instance(
                vm,
                call_with_payment128_ManagementCanister_canister_status(
                    canister_id_principal,
                    (_cdk_user_defined_args,),
                    payment,
                )
                .await,
            )?
        }
        "call_with_payment128_ManagementCanister_delete_canister" => {
            let canister_id_principal: ic_cdk::export::Principal =
                args[0].clone().try_from_vm_value(vm)?;
            let _cdk_user_defined_args: (DeleteCanisterArgs) =
                args[2usize].clone().try_from_vm_value(vm)?;
            let payment: u128 = args[3usize].clone().try_from_vm_value(vm)?;
            create_call_result_instance(
                vm,
                call_with_payment128_ManagementCanister_delete_canister(
                    canister_id_principal,
                    (_cdk_user_defined_args,),
                    payment,
                )
                .await,
            )?
        }
        "call_with_payment128_ManagementCanister_deposit_cycles" => {
            let canister_id_principal: ic_cdk::export::Principal =
                args[0].clone().try_from_vm_value(vm)?;
            let _cdk_user_defined_args: (DepositCyclesArgs) =
                args[2usize].clone().try_from_vm_value(vm)?;
            let payment: u128 = args[3usize].clone().try_from_vm_value(vm)?;
            create_call_result_instance(
                vm,
                call_with_payment128_ManagementCanister_deposit_cycles(
                    canister_id_principal,
                    (_cdk_user_defined_args,),
                    payment,
                )
                .await,
            )?
        }
        "call_with_payment128_ManagementCanister_raw_rand" => {
            let canister_id_principal: ic_cdk::export::Principal =
                args[0].clone().try_from_vm_value(vm)?;
            let payment: u128 = args[2usize].clone().try_from_vm_value(vm)?;
            create_call_result_instance(
                vm,
                call_with_payment128_ManagementCanister_raw_rand(
                    canister_id_principal,
                    (),
                    payment,
                )
                .await,
            )?
        }
        "call_with_payment128_ManagementCanister_provisional_create_canister_with_cycles" => {
            let canister_id_principal: ic_cdk::export::Principal =
                args[0].clone().try_from_vm_value(vm)?;
            let _cdk_user_defined_args: (ProvisionalCreateCanisterWithCyclesArgs) =
                args[2usize].clone().try_from_vm_value(vm)?;
            let payment: u128 = args[3usize].clone().try_from_vm_value(vm)?;
            create_call_result_instance(
                vm,
                call_with_payment128_ManagementCanister_provisional_create_canister_with_cycles(
                    canister_id_principal,
                    (_cdk_user_defined_args,),
                    payment,
                )
                .await,
            )?
        }
        "call_with_payment128_ManagementCanister_provisional_top_up_canister" => {
            let canister_id_principal: ic_cdk::export::Principal =
                args[0].clone().try_from_vm_value(vm)?;
            let _cdk_user_defined_args: (ProvisionalTopUpCanisterArgs) =
                args[2usize].clone().try_from_vm_value(vm)?;
            let payment: u128 = args[3usize].clone().try_from_vm_value(vm)?;
            create_call_result_instance(
                vm,
                call_with_payment128_ManagementCanister_provisional_top_up_canister(
                    canister_id_principal,
                    (_cdk_user_defined_args,),
                    payment,
                )
                .await,
            )?
        }
        "call_with_payment128_ManagementCanister_http_request" => {
            let canister_id_principal: ic_cdk::export::Principal =
                args[0].clone().try_from_vm_value(vm)?;
            let _cdk_user_defined_args: (HttpRequestArgs) =
                args[2usize].clone().try_from_vm_value(vm)?;
            let payment: u128 = args[3usize].clone().try_from_vm_value(vm)?;
            create_call_result_instance(
                vm,
                call_with_payment128_ManagementCanister_http_request(
                    canister_id_principal,
                    (_cdk_user_defined_args,),
                    payment,
                )
                .await,
            )?
        }
        "call_with_payment128_ManagementCanister_ecdsa_public_key" => {
            let canister_id_principal: ic_cdk::export::Principal =
                args[0].clone().try_from_vm_value(vm)?;
            let _cdk_user_defined_args: (EcdsaPublicKeyArgs) =
                args[2usize].clone().try_from_vm_value(vm)?;
            let payment: u128 = args[3usize].clone().try_from_vm_value(vm)?;
            create_call_result_instance(
                vm,
                call_with_payment128_ManagementCanister_ecdsa_public_key(
                    canister_id_principal,
                    (_cdk_user_defined_args,),
                    payment,
                )
                .await,
            )?
        }
        "call_with_payment128_ManagementCanister_sign_with_ecdsa" => {
            let canister_id_principal: ic_cdk::export::Principal =
                args[0].clone().try_from_vm_value(vm)?;
            let _cdk_user_defined_args: (SignWithEcdsaArgs) =
                args[2usize].clone().try_from_vm_value(vm)?;
            let payment: u128 = args[3usize].clone().try_from_vm_value(vm)?;
            create_call_result_instance(
                vm,
                call_with_payment128_ManagementCanister_sign_with_ecdsa(
                    canister_id_principal,
                    (_cdk_user_defined_args,),
                    payment,
                )
                .await,
            )?
        }
        _ => {
            return Err(vm.new_attribute_error(format!(
                "canister '{}' has no attribute '{}'",
                canister_id_principal, qual_name
            )))
        }
    };
    async_result_handler(vm, py_object_ref, call_result_instance).await
}
async fn async_result_handler_call_raw(
    vm: &rustpython::vm::VirtualMachine,
    py_object_ref: &rustpython_vm::PyObjectRef,
    args: &Vec<rustpython_vm::PyObjectRef>,
) -> rustpython_vm::PyResult {
    let canister_id_principal: ic_cdk::export::Principal = args[0].clone().try_from_vm_value(vm)?;
    let method_string: String = args[1].clone().try_from_vm_value(vm)?;
    let args_raw_vec: Vec<u8> = args[2].clone().try_from_vm_value(vm)?;
    let payment: u64 = args[3].clone().try_from_vm_value(vm)?;
    let call_raw_result = ic_cdk::api::call::call_raw(
        canister_id_principal,
        &method_string,
        &args_raw_vec,
        payment,
    )
    .await;
    async_result_handler(
        vm,
        py_object_ref,
        create_call_result_instance(vm, call_raw_result)?,
    )
    .await
}
async fn async_result_handler_call_raw128(
    vm: &rustpython::vm::VirtualMachine,
    py_object_ref: &rustpython_vm::PyObjectRef,
    args: &Vec<rustpython_vm::PyObjectRef>,
) -> rustpython_vm::PyResult {
    let canister_id_principal: ic_cdk::export::Principal = args[0].clone().try_from_vm_value(vm)?;
    let method_string: String = args[1].clone().try_from_vm_value(vm)?;
    let args_raw_vec: Vec<u8> = args[2].clone().try_from_vm_value(vm)?;
    let payment: u128 = args[3].clone().try_from_vm_value(vm)?;
    let call_raw_result = ic_cdk::api::call::call_raw128(
        canister_id_principal,
        &method_string,
        &args_raw_vec,
        payment,
    )
    .await;
    async_result_handler(
        vm,
        py_object_ref,
        create_call_result_instance(vm, call_raw_result)?,
    )
    .await
}
fn create_call_result_instance<T>(
    vm: &rustpython::vm::VirtualMachine,
    call_result: ic_cdk::api::call::CallResult<T>,
) -> rustpython_vm::PyResult
where
    T: for<'a> CdkActTryIntoVmValue<
        &'a rustpython::vm::VirtualMachine,
        rustpython::vm::PyObjectRef,
    >,
{
    let call_result_class = vm.run_block_expr(
        vm.new_scope_with_builtins(),
        format!("from kybra import CallResult; CallResult").as_str(),
    )?;
    match call_result {
        Ok(ok) => {
            let ok_value = ok
                .try_into_vm_value(vm)
                .map_err(|vmc_err| vm.new_type_error(vmc_err.0))?;
            call_result_class.call((ok_value, vm.ctx.none()), vm)
        }
        Err(err) => {
            let err_string = format!(
                "Rejection code {rejection_code}, {error_message}",
                rejection_code = (err.0 as i32).to_string(),
                error_message = err.1
            )
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))?;
            call_result_class.call((vm.ctx.none(), err_string), vm)
        }
    }
}
async fn call_global_python_function<'a, T>(
    function_name: &str,
    args: impl _KybraTraitIntoFuncArgs,
) -> Result<T, String>
where
    for<'b> rustpython::vm::PyObjectRef: CdkActTryFromVmValue<
        T,
        rustpython_vm::builtins::PyBaseExceptionRef,
        &'b rustpython::vm::VirtualMachine,
    >,
{
    let interpreter = unsafe { INTERPRETER_OPTION.as_mut() }
        .ok_or_else(|| "SystemError: missing python interpreter".to_string())?;
    let scope = unsafe { SCOPE_OPTION.as_mut() }
        .ok_or_else(|| "SystemError: missing python scope".to_string())?;
    let vm = &interpreter.vm;
    let py_object_ref = scope
        .globals
        .get_item(function_name, vm)
        .map_err(|py_base_exception| py_base_exception.to_rust_err_string(vm))?
        .call(args, vm)
        .map_err(|py_base_exception| py_base_exception.to_rust_err_string(vm))?;
    async_result_handler(vm, &py_object_ref, vm.ctx.none())
        .await
        .map_err(|py_base_exception| py_base_exception.to_rust_err_string(vm))?
        .try_from_vm_value(vm)
        .map_err(|py_base_exception| py_base_exception.to_rust_err_string(vm))
}
fn call_global_python_function_sync<'a, T>(
    function_name: &str,
    args: impl _KybraTraitIntoFuncArgs,
) -> Result<T, String>
where
    for<'b> rustpython::vm::PyObjectRef: CdkActTryFromVmValue<
        T,
        rustpython_vm::builtins::PyBaseExceptionRef,
        &'b rustpython::vm::VirtualMachine,
    >,
{
    let interpreter = unsafe { INTERPRETER_OPTION.as_mut() }
        .ok_or_else(|| "SystemError: missing python interpreter".to_string())?;
    let scope = unsafe { SCOPE_OPTION.as_mut() }
        .ok_or_else(|| "SystemError: missing python scope".to_string())?;
    interpreter.enter(|vm| {
        scope
            .globals
            .get_item(function_name, vm)
            .map_err(|py_base_exception| py_base_exception.to_rust_err_string(vm))?
            .call(args, vm)
            .map_err(|py_base_exception| py_base_exception.to_rust_err_string(vm))?
            .try_from_vm_value(vm)
            .map_err(|py_base_exception| py_base_exception.to_rust_err_string(vm))
    })
}
#[ic_cdk_macros::query]
fn __get_candid_interface_tmp_hack() -> String {
    __export_service()
}
#[ic_cdk_macros::query]
pub fn does_interpreter_exist() -> bool {
    if unsafe { INTERPRETER_OPTION.as_mut() }.is_none() {
        false
    } else {
        true
    }
}
#[rustpython_derive::pyclass(module = false, name = "ic")]
#[derive(Debug, rustpython_derive :: PyPayload)]
struct Ic {}
#[rustpython_derive::pyclass]
impl Ic {
    #[pymethod]
    fn accept_message(&self, vm: &rustpython_vm::VirtualMachine) -> rustpython_vm::PyResult {
        ic_cdk::api::call::accept_message()
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn arg_data_raw(&self, vm: &rustpython_vm::VirtualMachine) -> rustpython_vm::PyResult {
        ic_cdk::api::call::arg_data_raw()
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn arg_data_raw_size(&self, vm: &rustpython_vm::VirtualMachine) -> rustpython_vm::PyResult {
        ic_cdk::api::call::arg_data_raw_size()
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn caller(&self, vm: &rustpython_vm::VirtualMachine) -> rustpython_vm::PyResult {
        ic_cdk::api::caller()
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn candid_decode(
        &self,
        candid_encoded_py_object_ref: rustpython_vm::PyObjectRef,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let candid_encoded: Vec<u8> = candid_encoded_py_object_ref.try_from_vm_value(vm)?;
        let candid_args = candid::IDLArgs::from_bytes(&candid_encoded)
            .map_err(|candid_error| CandidError::new(vm, candid_error.to_string()))?;
        candid_args
            .to_string()
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn candid_encode(
        &self,
        candid_string_py_object_ref: rustpython_vm::PyObjectRef,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let candid_string: String = candid_string_py_object_ref.try_from_vm_value(vm)?;
        let candid_args: candid::IDLArgs = candid_string
            .parse::<candid::IDLArgs>()
            .map_err(|candid_error| CandidError::new(vm, candid_error.to_string()))?;
        let candid_encoded: Vec<u8> = candid_args
            .to_bytes()
            .map_err(|candid_error| CandidError::new(vm, candid_error.to_string()))?;
        candid_encoded
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn canister_balance(&self, vm: &rustpython_vm::VirtualMachine) -> rustpython_vm::PyResult {
        ic_cdk::api::canister_balance()
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn canister_balance128(&self, vm: &rustpython_vm::VirtualMachine) -> rustpython_vm::PyResult {
        ic_cdk::api::canister_balance128()
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn clear_timer(
        &self,
        timer_id_py_object_ref: rustpython_vm::PyObjectRef,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let timer_id: ic_cdk_timers::TimerId = timer_id_py_object_ref.try_from_vm_value(vm)?;
        ic_cdk_timers::clear_timer(timer_id)
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn data_certificate(&self, vm: &rustpython_vm::VirtualMachine) -> rustpython_vm::PyResult {
        ic_cdk::api::data_certificate()
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn id(&self, vm: &rustpython_vm::VirtualMachine) -> rustpython_vm::PyResult {
        ic_cdk::api::id()
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn method_name(&self, vm: &rustpython_vm::VirtualMachine) -> rustpython_vm::PyResult {
        ic_cdk::api::call::method_name()
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn msg_cycles_accept(
        &self,
        max_amount_py_object_ref: rustpython_vm::PyObjectRef,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let max_amount: u64 = max_amount_py_object_ref.try_from_vm_value(vm)?;
        ic_cdk::api::call::msg_cycles_accept(max_amount)
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn msg_cycles_accept128(
        &self,
        max_amount_py_object_ref: rustpython_vm::PyObjectRef,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let max_amount: u128 = max_amount_py_object_ref.try_from_vm_value(vm)?;
        ic_cdk::api::call::msg_cycles_accept128(max_amount)
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn msg_cycles_available(&self, vm: &rustpython_vm::VirtualMachine) -> rustpython_vm::PyResult {
        ic_cdk::api::call::msg_cycles_available()
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn msg_cycles_available128(
        &self,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        ic_cdk::api::call::msg_cycles_available128()
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn msg_cycles_refunded(&self, vm: &rustpython_vm::VirtualMachine) -> rustpython_vm::PyResult {
        ic_cdk::api::call::msg_cycles_refunded()
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn msg_cycles_refunded128(
        &self,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        ic_cdk::api::call::msg_cycles_refunded128()
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn notify_ManagementCanister_bitcoin_get_balance_wrapper(
        &self,
        args_py_object_refs: Vec<rustpython_vm::PyObjectRef>,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let canister_id_principal: ic_cdk::export::Principal =
            args_py_object_refs[0].clone().try_from_vm_value(vm)?;
        let _cdk_user_defined_args: (GetBalanceArgs) =
            args_py_object_refs[2usize].clone().try_from_vm_value(vm)?;
        let notify_result = notify_ManagementCanister_bitcoin_get_balance(
            canister_id_principal,
            (_cdk_user_defined_args,),
        );
        notify_result
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn notify_ManagementCanister_bitcoin_get_current_fee_percentiles_wrapper(
        &self,
        args_py_object_refs: Vec<rustpython_vm::PyObjectRef>,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let canister_id_principal: ic_cdk::export::Principal =
            args_py_object_refs[0].clone().try_from_vm_value(vm)?;
        let _cdk_user_defined_args: (GetCurrentFeePercentilesArgs) =
            args_py_object_refs[2usize].clone().try_from_vm_value(vm)?;
        let notify_result = notify_ManagementCanister_bitcoin_get_current_fee_percentiles(
            canister_id_principal,
            (_cdk_user_defined_args,),
        );
        notify_result
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn notify_ManagementCanister_bitcoin_get_utxos_wrapper(
        &self,
        args_py_object_refs: Vec<rustpython_vm::PyObjectRef>,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let canister_id_principal: ic_cdk::export::Principal =
            args_py_object_refs[0].clone().try_from_vm_value(vm)?;
        let _cdk_user_defined_args: (GetUtxosArgs) =
            args_py_object_refs[2usize].clone().try_from_vm_value(vm)?;
        let notify_result = notify_ManagementCanister_bitcoin_get_utxos(
            canister_id_principal,
            (_cdk_user_defined_args,),
        );
        notify_result
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn notify_ManagementCanister_bitcoin_send_transaction_wrapper(
        &self,
        args_py_object_refs: Vec<rustpython_vm::PyObjectRef>,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let canister_id_principal: ic_cdk::export::Principal =
            args_py_object_refs[0].clone().try_from_vm_value(vm)?;
        let _cdk_user_defined_args: (SendTransactionArgs) =
            args_py_object_refs[2usize].clone().try_from_vm_value(vm)?;
        let notify_result = notify_ManagementCanister_bitcoin_send_transaction(
            canister_id_principal,
            (_cdk_user_defined_args,),
        );
        notify_result
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn notify_ManagementCanister_create_canister_wrapper(
        &self,
        args_py_object_refs: Vec<rustpython_vm::PyObjectRef>,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let canister_id_principal: ic_cdk::export::Principal =
            args_py_object_refs[0].clone().try_from_vm_value(vm)?;
        let _cdk_user_defined_args: (CreateCanisterArgs) =
            args_py_object_refs[2usize].clone().try_from_vm_value(vm)?;
        let notify_result = notify_ManagementCanister_create_canister(
            canister_id_principal,
            (_cdk_user_defined_args,),
        );
        notify_result
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn notify_ManagementCanister_update_settings_wrapper(
        &self,
        args_py_object_refs: Vec<rustpython_vm::PyObjectRef>,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let canister_id_principal: ic_cdk::export::Principal =
            args_py_object_refs[0].clone().try_from_vm_value(vm)?;
        let _cdk_user_defined_args: (UpdateSettingsArgs) =
            args_py_object_refs[2usize].clone().try_from_vm_value(vm)?;
        let notify_result = notify_ManagementCanister_update_settings(
            canister_id_principal,
            (_cdk_user_defined_args,),
        );
        notify_result
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn notify_ManagementCanister_install_code_wrapper(
        &self,
        args_py_object_refs: Vec<rustpython_vm::PyObjectRef>,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let canister_id_principal: ic_cdk::export::Principal =
            args_py_object_refs[0].clone().try_from_vm_value(vm)?;
        let _cdk_user_defined_args: (InstallCodeArgs) =
            args_py_object_refs[2usize].clone().try_from_vm_value(vm)?;
        let notify_result = notify_ManagementCanister_install_code(
            canister_id_principal,
            (_cdk_user_defined_args,),
        );
        notify_result
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn notify_ManagementCanister_uninstall_code_wrapper(
        &self,
        args_py_object_refs: Vec<rustpython_vm::PyObjectRef>,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let canister_id_principal: ic_cdk::export::Principal =
            args_py_object_refs[0].clone().try_from_vm_value(vm)?;
        let _cdk_user_defined_args: (UninstallCodeArgs) =
            args_py_object_refs[2usize].clone().try_from_vm_value(vm)?;
        let notify_result = notify_ManagementCanister_uninstall_code(
            canister_id_principal,
            (_cdk_user_defined_args,),
        );
        notify_result
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn notify_ManagementCanister_start_canister_wrapper(
        &self,
        args_py_object_refs: Vec<rustpython_vm::PyObjectRef>,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let canister_id_principal: ic_cdk::export::Principal =
            args_py_object_refs[0].clone().try_from_vm_value(vm)?;
        let _cdk_user_defined_args: (StartCanisterArgs) =
            args_py_object_refs[2usize].clone().try_from_vm_value(vm)?;
        let notify_result = notify_ManagementCanister_start_canister(
            canister_id_principal,
            (_cdk_user_defined_args,),
        );
        notify_result
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn notify_ManagementCanister_stop_canister_wrapper(
        &self,
        args_py_object_refs: Vec<rustpython_vm::PyObjectRef>,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let canister_id_principal: ic_cdk::export::Principal =
            args_py_object_refs[0].clone().try_from_vm_value(vm)?;
        let _cdk_user_defined_args: (StopCanisterArgs) =
            args_py_object_refs[2usize].clone().try_from_vm_value(vm)?;
        let notify_result = notify_ManagementCanister_stop_canister(
            canister_id_principal,
            (_cdk_user_defined_args,),
        );
        notify_result
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn notify_ManagementCanister_canister_status_wrapper(
        &self,
        args_py_object_refs: Vec<rustpython_vm::PyObjectRef>,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let canister_id_principal: ic_cdk::export::Principal =
            args_py_object_refs[0].clone().try_from_vm_value(vm)?;
        let _cdk_user_defined_args: (CanisterStatusArgs) =
            args_py_object_refs[2usize].clone().try_from_vm_value(vm)?;
        let notify_result = notify_ManagementCanister_canister_status(
            canister_id_principal,
            (_cdk_user_defined_args,),
        );
        notify_result
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn notify_ManagementCanister_delete_canister_wrapper(
        &self,
        args_py_object_refs: Vec<rustpython_vm::PyObjectRef>,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let canister_id_principal: ic_cdk::export::Principal =
            args_py_object_refs[0].clone().try_from_vm_value(vm)?;
        let _cdk_user_defined_args: (DeleteCanisterArgs) =
            args_py_object_refs[2usize].clone().try_from_vm_value(vm)?;
        let notify_result = notify_ManagementCanister_delete_canister(
            canister_id_principal,
            (_cdk_user_defined_args,),
        );
        notify_result
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn notify_ManagementCanister_deposit_cycles_wrapper(
        &self,
        args_py_object_refs: Vec<rustpython_vm::PyObjectRef>,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let canister_id_principal: ic_cdk::export::Principal =
            args_py_object_refs[0].clone().try_from_vm_value(vm)?;
        let _cdk_user_defined_args: (DepositCyclesArgs) =
            args_py_object_refs[2usize].clone().try_from_vm_value(vm)?;
        let notify_result = notify_ManagementCanister_deposit_cycles(
            canister_id_principal,
            (_cdk_user_defined_args,),
        );
        notify_result
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn notify_ManagementCanister_raw_rand_wrapper(
        &self,
        args_py_object_refs: Vec<rustpython_vm::PyObjectRef>,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let canister_id_principal: ic_cdk::export::Principal =
            args_py_object_refs[0].clone().try_from_vm_value(vm)?;
        let notify_result = notify_ManagementCanister_raw_rand(canister_id_principal, ());
        notify_result
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn notify_ManagementCanister_provisional_create_canister_with_cycles_wrapper(
        &self,
        args_py_object_refs: Vec<rustpython_vm::PyObjectRef>,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let canister_id_principal: ic_cdk::export::Principal =
            args_py_object_refs[0].clone().try_from_vm_value(vm)?;
        let _cdk_user_defined_args: (ProvisionalCreateCanisterWithCyclesArgs) =
            args_py_object_refs[2usize].clone().try_from_vm_value(vm)?;
        let notify_result = notify_ManagementCanister_provisional_create_canister_with_cycles(
            canister_id_principal,
            (_cdk_user_defined_args,),
        );
        notify_result
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn notify_ManagementCanister_provisional_top_up_canister_wrapper(
        &self,
        args_py_object_refs: Vec<rustpython_vm::PyObjectRef>,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let canister_id_principal: ic_cdk::export::Principal =
            args_py_object_refs[0].clone().try_from_vm_value(vm)?;
        let _cdk_user_defined_args: (ProvisionalTopUpCanisterArgs) =
            args_py_object_refs[2usize].clone().try_from_vm_value(vm)?;
        let notify_result = notify_ManagementCanister_provisional_top_up_canister(
            canister_id_principal,
            (_cdk_user_defined_args,),
        );
        notify_result
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn notify_ManagementCanister_http_request_wrapper(
        &self,
        args_py_object_refs: Vec<rustpython_vm::PyObjectRef>,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let canister_id_principal: ic_cdk::export::Principal =
            args_py_object_refs[0].clone().try_from_vm_value(vm)?;
        let _cdk_user_defined_args: (HttpRequestArgs) =
            args_py_object_refs[2usize].clone().try_from_vm_value(vm)?;
        let notify_result = notify_ManagementCanister_http_request(
            canister_id_principal,
            (_cdk_user_defined_args,),
        );
        notify_result
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn notify_ManagementCanister_ecdsa_public_key_wrapper(
        &self,
        args_py_object_refs: Vec<rustpython_vm::PyObjectRef>,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let canister_id_principal: ic_cdk::export::Principal =
            args_py_object_refs[0].clone().try_from_vm_value(vm)?;
        let _cdk_user_defined_args: (EcdsaPublicKeyArgs) =
            args_py_object_refs[2usize].clone().try_from_vm_value(vm)?;
        let notify_result = notify_ManagementCanister_ecdsa_public_key(
            canister_id_principal,
            (_cdk_user_defined_args,),
        );
        notify_result
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn notify_ManagementCanister_sign_with_ecdsa_wrapper(
        &self,
        args_py_object_refs: Vec<rustpython_vm::PyObjectRef>,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let canister_id_principal: ic_cdk::export::Principal =
            args_py_object_refs[0].clone().try_from_vm_value(vm)?;
        let _cdk_user_defined_args: (SignWithEcdsaArgs) =
            args_py_object_refs[2usize].clone().try_from_vm_value(vm)?;
        let notify_result = notify_ManagementCanister_sign_with_ecdsa(
            canister_id_principal,
            (_cdk_user_defined_args,),
        );
        notify_result
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn notify_raw(
        &self,
        canister_id_py_object_ref: rustpython_vm::PyObjectRef,
        method_py_object_ref: rustpython_vm::PyObjectRef,
        args_raw_py_object_ref: rustpython_vm::PyObjectRef,
        payment_py_object_ref: rustpython_vm::PyObjectRef,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let canister_id_principal: ic_cdk::export::Principal =
            canister_id_py_object_ref.try_from_vm_value(vm)?;
        let method_string: String = method_py_object_ref.try_from_vm_value(vm)?;
        let args_raw_vec: Vec<u8> = args_raw_py_object_ref.try_from_vm_value(vm)?;
        let payment: u128 = payment_py_object_ref.try_from_vm_value(vm)?;
        let notify_result = ic_cdk::api::call::notify_raw(
            canister_id_principal,
            &method_string,
            &args_raw_vec,
            payment,
        );
        notify_result
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn notify_with_payment128_ManagementCanister_bitcoin_get_balance_wrapper(
        &self,
        args_py_object_refs: Vec<rustpython_vm::PyObjectRef>,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let canister_id_principal: ic_cdk::export::Principal =
            args_py_object_refs[0].clone().try_from_vm_value(vm)?;
        let _cdk_user_defined_args: (GetBalanceArgs) =
            args_py_object_refs[2usize].clone().try_from_vm_value(vm)?;
        let cycles: u128 = args_py_object_refs[args_py_object_refs.len() - 1]
            .clone()
            .try_from_vm_value(vm)?;
        let notify_result = notify_with_payment128_ManagementCanister_bitcoin_get_balance(
            canister_id_principal,
            (_cdk_user_defined_args,),
            cycles,
        );
        notify_result
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn notify_with_payment128_ManagementCanister_bitcoin_get_current_fee_percentiles_wrapper(
        &self,
        args_py_object_refs: Vec<rustpython_vm::PyObjectRef>,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let canister_id_principal: ic_cdk::export::Principal =
            args_py_object_refs[0].clone().try_from_vm_value(vm)?;
        let _cdk_user_defined_args: (GetCurrentFeePercentilesArgs) =
            args_py_object_refs[2usize].clone().try_from_vm_value(vm)?;
        let cycles: u128 = args_py_object_refs[args_py_object_refs.len() - 1]
            .clone()
            .try_from_vm_value(vm)?;
        let notify_result =
            notify_with_payment128_ManagementCanister_bitcoin_get_current_fee_percentiles(
                canister_id_principal,
                (_cdk_user_defined_args,),
                cycles,
            );
        notify_result
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn notify_with_payment128_ManagementCanister_bitcoin_get_utxos_wrapper(
        &self,
        args_py_object_refs: Vec<rustpython_vm::PyObjectRef>,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let canister_id_principal: ic_cdk::export::Principal =
            args_py_object_refs[0].clone().try_from_vm_value(vm)?;
        let _cdk_user_defined_args: (GetUtxosArgs) =
            args_py_object_refs[2usize].clone().try_from_vm_value(vm)?;
        let cycles: u128 = args_py_object_refs[args_py_object_refs.len() - 1]
            .clone()
            .try_from_vm_value(vm)?;
        let notify_result = notify_with_payment128_ManagementCanister_bitcoin_get_utxos(
            canister_id_principal,
            (_cdk_user_defined_args,),
            cycles,
        );
        notify_result
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn notify_with_payment128_ManagementCanister_bitcoin_send_transaction_wrapper(
        &self,
        args_py_object_refs: Vec<rustpython_vm::PyObjectRef>,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let canister_id_principal: ic_cdk::export::Principal =
            args_py_object_refs[0].clone().try_from_vm_value(vm)?;
        let _cdk_user_defined_args: (SendTransactionArgs) =
            args_py_object_refs[2usize].clone().try_from_vm_value(vm)?;
        let cycles: u128 = args_py_object_refs[args_py_object_refs.len() - 1]
            .clone()
            .try_from_vm_value(vm)?;
        let notify_result = notify_with_payment128_ManagementCanister_bitcoin_send_transaction(
            canister_id_principal,
            (_cdk_user_defined_args,),
            cycles,
        );
        notify_result
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn notify_with_payment128_ManagementCanister_create_canister_wrapper(
        &self,
        args_py_object_refs: Vec<rustpython_vm::PyObjectRef>,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let canister_id_principal: ic_cdk::export::Principal =
            args_py_object_refs[0].clone().try_from_vm_value(vm)?;
        let _cdk_user_defined_args: (CreateCanisterArgs) =
            args_py_object_refs[2usize].clone().try_from_vm_value(vm)?;
        let cycles: u128 = args_py_object_refs[args_py_object_refs.len() - 1]
            .clone()
            .try_from_vm_value(vm)?;
        let notify_result = notify_with_payment128_ManagementCanister_create_canister(
            canister_id_principal,
            (_cdk_user_defined_args,),
            cycles,
        );
        notify_result
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn notify_with_payment128_ManagementCanister_update_settings_wrapper(
        &self,
        args_py_object_refs: Vec<rustpython_vm::PyObjectRef>,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let canister_id_principal: ic_cdk::export::Principal =
            args_py_object_refs[0].clone().try_from_vm_value(vm)?;
        let _cdk_user_defined_args: (UpdateSettingsArgs) =
            args_py_object_refs[2usize].clone().try_from_vm_value(vm)?;
        let cycles: u128 = args_py_object_refs[args_py_object_refs.len() - 1]
            .clone()
            .try_from_vm_value(vm)?;
        let notify_result = notify_with_payment128_ManagementCanister_update_settings(
            canister_id_principal,
            (_cdk_user_defined_args,),
            cycles,
        );
        notify_result
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn notify_with_payment128_ManagementCanister_install_code_wrapper(
        &self,
        args_py_object_refs: Vec<rustpython_vm::PyObjectRef>,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let canister_id_principal: ic_cdk::export::Principal =
            args_py_object_refs[0].clone().try_from_vm_value(vm)?;
        let _cdk_user_defined_args: (InstallCodeArgs) =
            args_py_object_refs[2usize].clone().try_from_vm_value(vm)?;
        let cycles: u128 = args_py_object_refs[args_py_object_refs.len() - 1]
            .clone()
            .try_from_vm_value(vm)?;
        let notify_result = notify_with_payment128_ManagementCanister_install_code(
            canister_id_principal,
            (_cdk_user_defined_args,),
            cycles,
        );
        notify_result
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn notify_with_payment128_ManagementCanister_uninstall_code_wrapper(
        &self,
        args_py_object_refs: Vec<rustpython_vm::PyObjectRef>,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let canister_id_principal: ic_cdk::export::Principal =
            args_py_object_refs[0].clone().try_from_vm_value(vm)?;
        let _cdk_user_defined_args: (UninstallCodeArgs) =
            args_py_object_refs[2usize].clone().try_from_vm_value(vm)?;
        let cycles: u128 = args_py_object_refs[args_py_object_refs.len() - 1]
            .clone()
            .try_from_vm_value(vm)?;
        let notify_result = notify_with_payment128_ManagementCanister_uninstall_code(
            canister_id_principal,
            (_cdk_user_defined_args,),
            cycles,
        );
        notify_result
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn notify_with_payment128_ManagementCanister_start_canister_wrapper(
        &self,
        args_py_object_refs: Vec<rustpython_vm::PyObjectRef>,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let canister_id_principal: ic_cdk::export::Principal =
            args_py_object_refs[0].clone().try_from_vm_value(vm)?;
        let _cdk_user_defined_args: (StartCanisterArgs) =
            args_py_object_refs[2usize].clone().try_from_vm_value(vm)?;
        let cycles: u128 = args_py_object_refs[args_py_object_refs.len() - 1]
            .clone()
            .try_from_vm_value(vm)?;
        let notify_result = notify_with_payment128_ManagementCanister_start_canister(
            canister_id_principal,
            (_cdk_user_defined_args,),
            cycles,
        );
        notify_result
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn notify_with_payment128_ManagementCanister_stop_canister_wrapper(
        &self,
        args_py_object_refs: Vec<rustpython_vm::PyObjectRef>,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let canister_id_principal: ic_cdk::export::Principal =
            args_py_object_refs[0].clone().try_from_vm_value(vm)?;
        let _cdk_user_defined_args: (StopCanisterArgs) =
            args_py_object_refs[2usize].clone().try_from_vm_value(vm)?;
        let cycles: u128 = args_py_object_refs[args_py_object_refs.len() - 1]
            .clone()
            .try_from_vm_value(vm)?;
        let notify_result = notify_with_payment128_ManagementCanister_stop_canister(
            canister_id_principal,
            (_cdk_user_defined_args,),
            cycles,
        );
        notify_result
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn notify_with_payment128_ManagementCanister_canister_status_wrapper(
        &self,
        args_py_object_refs: Vec<rustpython_vm::PyObjectRef>,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let canister_id_principal: ic_cdk::export::Principal =
            args_py_object_refs[0].clone().try_from_vm_value(vm)?;
        let _cdk_user_defined_args: (CanisterStatusArgs) =
            args_py_object_refs[2usize].clone().try_from_vm_value(vm)?;
        let cycles: u128 = args_py_object_refs[args_py_object_refs.len() - 1]
            .clone()
            .try_from_vm_value(vm)?;
        let notify_result = notify_with_payment128_ManagementCanister_canister_status(
            canister_id_principal,
            (_cdk_user_defined_args,),
            cycles,
        );
        notify_result
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn notify_with_payment128_ManagementCanister_delete_canister_wrapper(
        &self,
        args_py_object_refs: Vec<rustpython_vm::PyObjectRef>,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let canister_id_principal: ic_cdk::export::Principal =
            args_py_object_refs[0].clone().try_from_vm_value(vm)?;
        let _cdk_user_defined_args: (DeleteCanisterArgs) =
            args_py_object_refs[2usize].clone().try_from_vm_value(vm)?;
        let cycles: u128 = args_py_object_refs[args_py_object_refs.len() - 1]
            .clone()
            .try_from_vm_value(vm)?;
        let notify_result = notify_with_payment128_ManagementCanister_delete_canister(
            canister_id_principal,
            (_cdk_user_defined_args,),
            cycles,
        );
        notify_result
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn notify_with_payment128_ManagementCanister_deposit_cycles_wrapper(
        &self,
        args_py_object_refs: Vec<rustpython_vm::PyObjectRef>,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let canister_id_principal: ic_cdk::export::Principal =
            args_py_object_refs[0].clone().try_from_vm_value(vm)?;
        let _cdk_user_defined_args: (DepositCyclesArgs) =
            args_py_object_refs[2usize].clone().try_from_vm_value(vm)?;
        let cycles: u128 = args_py_object_refs[args_py_object_refs.len() - 1]
            .clone()
            .try_from_vm_value(vm)?;
        let notify_result = notify_with_payment128_ManagementCanister_deposit_cycles(
            canister_id_principal,
            (_cdk_user_defined_args,),
            cycles,
        );
        notify_result
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn notify_with_payment128_ManagementCanister_raw_rand_wrapper(
        &self,
        args_py_object_refs: Vec<rustpython_vm::PyObjectRef>,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let canister_id_principal: ic_cdk::export::Principal =
            args_py_object_refs[0].clone().try_from_vm_value(vm)?;
        let cycles: u128 = args_py_object_refs[args_py_object_refs.len() - 1]
            .clone()
            .try_from_vm_value(vm)?;
        let notify_result =
            notify_with_payment128_ManagementCanister_raw_rand(canister_id_principal, (), cycles);
        notify_result
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn notify_with_payment128_ManagementCanister_provisional_create_canister_with_cycles_wrapper(
        &self,
        args_py_object_refs: Vec<rustpython_vm::PyObjectRef>,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let canister_id_principal: ic_cdk::export::Principal =
            args_py_object_refs[0].clone().try_from_vm_value(vm)?;
        let _cdk_user_defined_args: (ProvisionalCreateCanisterWithCyclesArgs) =
            args_py_object_refs[2usize].clone().try_from_vm_value(vm)?;
        let cycles: u128 = args_py_object_refs[args_py_object_refs.len() - 1]
            .clone()
            .try_from_vm_value(vm)?;
        let notify_result =
            notify_with_payment128_ManagementCanister_provisional_create_canister_with_cycles(
                canister_id_principal,
                (_cdk_user_defined_args,),
                cycles,
            );
        notify_result
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn notify_with_payment128_ManagementCanister_provisional_top_up_canister_wrapper(
        &self,
        args_py_object_refs: Vec<rustpython_vm::PyObjectRef>,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let canister_id_principal: ic_cdk::export::Principal =
            args_py_object_refs[0].clone().try_from_vm_value(vm)?;
        let _cdk_user_defined_args: (ProvisionalTopUpCanisterArgs) =
            args_py_object_refs[2usize].clone().try_from_vm_value(vm)?;
        let cycles: u128 = args_py_object_refs[args_py_object_refs.len() - 1]
            .clone()
            .try_from_vm_value(vm)?;
        let notify_result = notify_with_payment128_ManagementCanister_provisional_top_up_canister(
            canister_id_principal,
            (_cdk_user_defined_args,),
            cycles,
        );
        notify_result
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn notify_with_payment128_ManagementCanister_http_request_wrapper(
        &self,
        args_py_object_refs: Vec<rustpython_vm::PyObjectRef>,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let canister_id_principal: ic_cdk::export::Principal =
            args_py_object_refs[0].clone().try_from_vm_value(vm)?;
        let _cdk_user_defined_args: (HttpRequestArgs) =
            args_py_object_refs[2usize].clone().try_from_vm_value(vm)?;
        let cycles: u128 = args_py_object_refs[args_py_object_refs.len() - 1]
            .clone()
            .try_from_vm_value(vm)?;
        let notify_result = notify_with_payment128_ManagementCanister_http_request(
            canister_id_principal,
            (_cdk_user_defined_args,),
            cycles,
        );
        notify_result
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn notify_with_payment128_ManagementCanister_ecdsa_public_key_wrapper(
        &self,
        args_py_object_refs: Vec<rustpython_vm::PyObjectRef>,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let canister_id_principal: ic_cdk::export::Principal =
            args_py_object_refs[0].clone().try_from_vm_value(vm)?;
        let _cdk_user_defined_args: (EcdsaPublicKeyArgs) =
            args_py_object_refs[2usize].clone().try_from_vm_value(vm)?;
        let cycles: u128 = args_py_object_refs[args_py_object_refs.len() - 1]
            .clone()
            .try_from_vm_value(vm)?;
        let notify_result = notify_with_payment128_ManagementCanister_ecdsa_public_key(
            canister_id_principal,
            (_cdk_user_defined_args,),
            cycles,
        );
        notify_result
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn notify_with_payment128_ManagementCanister_sign_with_ecdsa_wrapper(
        &self,
        args_py_object_refs: Vec<rustpython_vm::PyObjectRef>,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let canister_id_principal: ic_cdk::export::Principal =
            args_py_object_refs[0].clone().try_from_vm_value(vm)?;
        let _cdk_user_defined_args: (SignWithEcdsaArgs) =
            args_py_object_refs[2usize].clone().try_from_vm_value(vm)?;
        let cycles: u128 = args_py_object_refs[args_py_object_refs.len() - 1]
            .clone()
            .try_from_vm_value(vm)?;
        let notify_result = notify_with_payment128_ManagementCanister_sign_with_ecdsa(
            canister_id_principal,
            (_cdk_user_defined_args,),
            cycles,
        );
        notify_result
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn performance_counter(
        &self,
        counter_type_py_object_ref: rustpython_vm::PyObjectRef,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let counter_type: u32 = counter_type_py_object_ref.try_from_vm_value(vm)?;
        ic_cdk::api::call::performance_counter(counter_type)
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn print(
        &self,
        param_py_object_ref: rustpython_vm::PyObjectRef,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let param_string: String = param_py_object_ref.try_from_vm_value(vm)?;
        ic_cdk::println!("{:#?}", param_string)
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn reject(
        &self,
        reject_py_object_ref: rustpython_vm::PyObjectRef,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let reject_message: String = reject_py_object_ref.try_from_vm_value(vm)?;
        ic_cdk::api::call::reject(reject_message.as_str())
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn reject_code(&self, vm: &rustpython_vm::VirtualMachine) -> rustpython_vm::PyResult {
        ic_cdk::api::call::reject_code()
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn reject_message(&self, vm: &rustpython_vm::VirtualMachine) -> rustpython_vm::PyResult {
        ic_cdk::api::call::reject_message()
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn reply(
        &self,
        first_called_function_name_py_object_ref: rustpython_vm::PyObjectRef,
        reply_value_py_object_ref: rustpython_vm::PyObjectRef,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let first_called_function_name: String =
            first_called_function_name_py_object_ref.try_from_vm_value(vm)?;
        match &first_called_function_name[..] {
            _ => Err(vm.new_system_error(format!(
                "attempted to reply from \"{}\", but it does not appear to be a canister method",
                first_called_function_name
            ))),
        }
    }
    #[pymethod]
    fn reply_raw(
        &self,
        buf_vector_py_object_ref: rustpython_vm::PyObjectRef,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let buf_vector: Vec<u8> = buf_vector_py_object_ref.try_from_vm_value(vm)?;
        ic_cdk::api::call::reply_raw(&buf_vector)
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn set_certified_data(
        &self,
        data_py_object_ref: rustpython_vm::PyObjectRef,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let data: Vec<u8> = data_py_object_ref.try_from_vm_value(vm)?;
        ic_cdk::api::set_certified_data(&data)
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn set_timer(
        &self,
        delay_py_object_ref: rustpython_vm::PyObjectRef,
        func_py_object_ref: rustpython_vm::PyObjectRef,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let delay_as_u64: u64 = delay_py_object_ref.try_from_vm_value(vm)?;
        let delay = core::time::Duration::new(delay_as_u64, 0);
        let closure = move || {
            let interpreter = unsafe { INTERPRETER_OPTION.as_mut() }
                .unwrap_or_trap("SystemError: missing python interpreter");
            let scope = unsafe { SCOPE_OPTION.as_mut() }
                .unwrap_or_trap("SystemError: missing python scope");
            let vm = &interpreter.vm;
            let py_object_ref = func_py_object_ref.call((), vm).unwrap_or_trap(vm);
            ic_cdk::spawn(async move {
                async_result_handler(vm, &py_object_ref, vm.ctx.none()).await;
            });
        };
        ic_cdk_timers::set_timer(delay, closure)
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn set_timer_interval(
        &self,
        interval_py_object_ref: rustpython_vm::PyObjectRef,
        func_py_object_ref: rustpython_vm::PyObjectRef,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let interval_as_u64: u64 = interval_py_object_ref.try_from_vm_value(vm)?;
        let interval = core::time::Duration::new(interval_as_u64, 0);
        let closure = move || {
            let interpreter = unsafe { INTERPRETER_OPTION.as_mut() }
                .unwrap_or_trap("SystemError: missing python interpreter");
            let scope = unsafe { SCOPE_OPTION.as_mut() }
                .unwrap_or_trap("SystemError: missing python scope");
            let vm = &interpreter.vm;
            let py_object_ref = func_py_object_ref.call((), vm).unwrap_or_trap(vm);
            ic_cdk::spawn(async move {
                async_result_handler(vm, &py_object_ref, vm.ctx.none()).await;
            });
        };
        ic_cdk_timers::set_timer_interval(interval, closure)
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn stable_bytes(&self, vm: &rustpython_vm::VirtualMachine) -> rustpython_vm::PyResult {
        ic_cdk::api::stable::stable_bytes()
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn stable_grow(
        &self,
        new_pages_py_object_ref: rustpython_vm::PyObjectRef,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let new_pages: u32 = new_pages_py_object_ref.try_from_vm_value(vm)?;
        ic_cdk::api::stable::stable_grow(new_pages)
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn stable_read(
        &self,
        offset_py_object_ref: rustpython_vm::PyObjectRef,
        length_py_object_ref: rustpython_vm::PyObjectRef,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let offset: u32 = offset_py_object_ref.try_from_vm_value(vm)?;
        let length: u32 = length_py_object_ref.try_from_vm_value(vm)?;
        let mut buf: Vec<u8> = vec![0; length as usize];
        ic_cdk::api::stable::stable_read(offset, &mut buf);
        buf.try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn stable_size(&self, vm: &rustpython_vm::VirtualMachine) -> rustpython_vm::PyResult {
        ic_cdk::api::stable::stable_size()
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn stable_write(
        &self,
        offset_py_object_ref: rustpython_vm::PyObjectRef,
        buf_vector_py_object_ref: rustpython_vm::PyObjectRef,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let offset: u32 = offset_py_object_ref.try_from_vm_value(vm)?;
        let buf_vector: Vec<u8> = buf_vector_py_object_ref.try_from_vm_value(vm)?;
        let buf: &[u8] = &buf_vector[..];
        ic_cdk::api::stable::stable_write(offset, buf)
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn stable_b_tree_map_contains_key(
        &self,
        memory_id_py_object_ref: rustpython_vm::PyObjectRef,
        key_py_object_ref: rustpython_vm::PyObjectRef,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let memory_id: u8 = memory_id_py_object_ref.try_from_vm_value(vm)?;
        match memory_id {
            _ => Err(vm.new_lookup_error(format!(
                "memory_id {} does not have an associated StableBTreeMap",
                memory_id
            ))),
        }
    }
    #[pymethod]
    fn stable_b_tree_map_get(
        &self,
        memory_id_py_object_ref: rustpython_vm::PyObjectRef,
        key_py_object_ref: rustpython_vm::PyObjectRef,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let memory_id: u8 = memory_id_py_object_ref.try_from_vm_value(vm)?;
        match memory_id {
            _ => Err(vm.new_lookup_error(format!(
                "memory_id {} does not have an associated StableBTreeMap",
                memory_id
            ))),
        }
    }
    #[pymethod]
    fn stable_b_tree_map_insert(
        &self,
        memory_id_py_object_ref: rustpython_vm::PyObjectRef,
        key_py_object_ref: rustpython_vm::PyObjectRef,
        value_py_object_ref: rustpython_vm::PyObjectRef,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let memory_id: u8 = memory_id_py_object_ref.try_from_vm_value(vm)?;
        match memory_id {
            _ => Err(vm.new_lookup_error(format!(
                "memory_id {} does not have an associated StableBTreeMap",
                memory_id
            ))),
        }
    }
    #[pymethod]
    fn stable_b_tree_map_is_empty(
        &self,
        memory_id_py_object_ref: rustpython_vm::PyObjectRef,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let memory_id: u8 = memory_id_py_object_ref.try_from_vm_value(vm)?;
        match memory_id {
            _ => Err(vm.new_lookup_error(format!(
                "memory_id {} does not have an associated StableBTreeMap",
                memory_id
            ))),
        }
    }
    #[pymethod]
    fn stable_b_tree_map_items(
        &self,
        memory_id_py_object_ref: rustpython_vm::PyObjectRef,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let memory_id: u8 = memory_id_py_object_ref.try_from_vm_value(vm)?;
        match memory_id {
            _ => Err(vm.new_lookup_error(format!(
                "memory_id {} does not have an associated StableBTreeMap",
                memory_id
            ))),
        }
    }
    #[pymethod]
    fn stable_b_tree_map_keys(
        &self,
        memory_id_py_object_ref: rustpython_vm::PyObjectRef,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let memory_id: u8 = memory_id_py_object_ref.try_from_vm_value(vm)?;
        match memory_id {
            _ => Err(vm.new_lookup_error(format!(
                "memory_id {} does not have an associated StableBTreeMap",
                memory_id
            ))),
        }
    }
    #[pymethod]
    fn stable_b_tree_map_len(
        &self,
        memory_id_py_object_ref: rustpython_vm::PyObjectRef,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let memory_id: u8 = memory_id_py_object_ref.try_from_vm_value(vm)?;
        match memory_id {
            _ => Err(vm.new_lookup_error(format!(
                "memory_id {} does not have an associated StableBTreeMap",
                memory_id
            ))),
        }
    }
    #[pymethod]
    fn stable_b_tree_map_remove(
        &self,
        memory_id_py_object_ref: rustpython_vm::PyObjectRef,
        key_py_object_ref: rustpython_vm::PyObjectRef,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let memory_id: u8 = memory_id_py_object_ref.try_from_vm_value(vm)?;
        match memory_id {
            _ => Err(vm.new_lookup_error(format!(
                "memory_id {} does not have an associated StableBTreeMap",
                memory_id
            ))),
        }
    }
    #[pymethod]
    fn stable_b_tree_map_values(
        &self,
        memory_id_py_object_ref: rustpython_vm::PyObjectRef,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let memory_id: u8 = memory_id_py_object_ref.try_from_vm_value(vm)?;
        match memory_id {
            _ => Err(vm.new_lookup_error(format!(
                "memory_id {} does not have an associated StableBTreeMap",
                memory_id
            ))),
        }
    }
    #[pymethod]
    fn stable64_grow(
        &self,
        new_pages_py_object_ref: rustpython_vm::PyObjectRef,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let new_pages: u64 = new_pages_py_object_ref.try_from_vm_value(vm)?;
        ic_cdk::api::stable::stable64_grow(new_pages)
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn stable64_read(
        &self,
        offset_py_object_ref: rustpython_vm::PyObjectRef,
        length_py_object_ref: rustpython_vm::PyObjectRef,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let offset: u64 = offset_py_object_ref.try_from_vm_value(vm)?;
        let length: u64 = length_py_object_ref.try_from_vm_value(vm)?;
        let mut buf: Vec<u8> = vec![0; length as usize];
        ic_cdk::api::stable::stable64_read(offset, &mut buf);
        buf.try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn stable64_size(&self, vm: &rustpython_vm::VirtualMachine) -> rustpython_vm::PyResult {
        ic_cdk::api::stable::stable64_size()
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn stable64_write(
        &self,
        offset_py_object_ref: rustpython_vm::PyObjectRef,
        buf_vector_py_object_ref: rustpython_vm::PyObjectRef,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let offset: u64 = offset_py_object_ref.try_from_vm_value(vm)?;
        let buf_vector: Vec<u8> = buf_vector_py_object_ref.try_from_vm_value(vm)?;
        let buf: &[u8] = &buf_vector[..];
        ic_cdk::api::stable::stable64_write(offset, buf)
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn time(&self, vm: &rustpython_vm::VirtualMachine) -> rustpython_vm::PyResult {
        ic_cdk::api::time()
            .try_into_vm_value(vm)
            .map_err(|vmc_err| vm.new_type_error(vmc_err.0))
    }
    #[pymethod]
    fn trap(
        &self,
        message_py_object_ref: rustpython_vm::PyObjectRef,
        vm: &rustpython_vm::VirtualMachine,
    ) -> rustpython_vm::PyResult {
        let message: String = message_py_object_ref.try_from_vm_value(vm)?;
        ic_cdk::api::trap(&message)
    }
}
thread_local! { static MEMORY_MANAGER_REF_CELL : std :: cell :: RefCell < ic_stable_structures :: memory_manager :: MemoryManager < ic_stable_structures :: DefaultMemoryImpl > > = std :: cell :: RefCell :: new (ic_stable_structures :: memory_manager :: MemoryManager :: init (ic_stable_structures :: DefaultMemoryImpl :: default ())) ; }
pub trait UnwrapOrTrapWithMessage<T> {
    fn unwrap_or_trap(self, err_message: &str) -> T;
}
impl<T> UnwrapOrTrapWithMessage<T> for Option<T> {
    fn unwrap_or_trap(self, err_message: &str) -> T {
        match self {
            Some(some) => some,
            None => ic_cdk::trap(err_message),
        }
    }
}
pub trait UnwrapOrTrap<T> {
    fn unwrap_or_trap(self) -> T;
}
impl<T> UnwrapOrTrap<T> for Result<T, CdkActTryIntoVmValueError> {
    fn unwrap_or_trap(self) -> T {
        match self {
            Ok(ok) => ok,
            Err(err) => ic_cdk::trap(&err.0),
        }
    }
}
impl<T> UnwrapOrTrap<T> for Result<T, ic_stable_structures::cell::ValueError> {
    fn unwrap_or_trap(self) -> T {
        match self {
            Ok(ok) => ok,
            Err(err) => ic_cdk::trap(&match err {
                ic_stable_structures::cell::ValueError::ValueTooLarge { value_size } => {
                    format!("ValueError: ValueTooLarge {value_size}")
                }
            }),
        }
    }
}
impl<T> UnwrapOrTrap<T> for Result<T, ic_stable_structures::cell::InitError> {
    fn unwrap_or_trap(self) -> T {
        match self {
            Ok(ok) => ok,
            Err(err) => ic_cdk::trap(&init_error_to_string(&err)),
        }
    }
}
impl<T> UnwrapOrTrap<T> for candid::Result<T> {
    fn unwrap_or_trap(self) -> T {
        match self {
            Ok(ok) => ok,
            Err(err) => ic_cdk::trap(&format!("CandidError: {}", err.to_string())),
        }
    }
}
impl<T> UnwrapOrTrap<T> for Result<T, String> {
    fn unwrap_or_trap(self) -> T {
        match self {
            Ok(ok) => ok,
            Err(err) => ic_cdk::trap(&err),
        }
    }
}
pub trait UnwrapOrTrapWithVm<T> {
    fn unwrap_or_trap(self, vm: &rustpython::vm::VirtualMachine) -> T;
}
impl<T> UnwrapOrTrapWithVm<T>
    for Result<T, rustpython::vm::PyRef<rustpython_vm::builtins::PyBaseException>>
{
    fn unwrap_or_trap(self, vm: &rustpython::vm::VirtualMachine) -> T {
        match self {
            Ok(ok) => ok,
            Err(err) => {
                let py_object = err.to_pyobject(vm);
                let type_name = py_object.class().name().to_string();
                let err_message = match py_object.str(vm) {
                    Ok(str) => str,
                    Err(_) => ic_cdk::trap(
                        format!("Attribute Error: '{type_name}' object has no attribute '__str__'")
                            .as_str(),
                    ),
                };
                ic_cdk::trap(format!("{type_name}: {err_message}").as_str())
            }
        }
    }
}
fn init_error_to_string(err: &ic_stable_structures::cell::InitError) -> String {
    match err { ic_stable_structures :: cell :: InitError :: IncompatibleVersion { last_supported_version , decoded_version , } => format ! ("InitError: IncompatibleVersion, last_supported_version {last_supported_version}, decoded_version {decoded_version}") , ic_stable_structures :: cell :: InitError :: ValueTooLarge { value_size } => { format ! ("InitError: ValueTooLarge {value_size}") } }
}
struct KybraError {}
impl KybraError {
    fn new(
        vm: &rustpython_vm::VirtualMachine,
        message: String,
    ) -> rustpython_vm::builtins::PyBaseExceptionRef {
        KybraError::subtype(vm, "Error", message)
    }
    fn subtype(
        vm: &rustpython_vm::VirtualMachine,
        subtype: &str,
        message: String,
    ) -> rustpython_vm::builtins::PyBaseExceptionRef {
        let kybra_error_class = match vm.run_block_expr(
            vm.new_scope_with_builtins(),
            format!("from kybra import {subtype}; {subtype}").as_str(),
        ) {
            Ok(kybra_error_class) => kybra_error_class,
            Err(py_base_exception) => return py_base_exception,
        };
        let py_type_ref =
            match rustpython_vm::builtins::PyTypeRef::try_from_object(vm, kybra_error_class) {
                Ok(py_type_ref) => py_type_ref,
                Err(py_base_exception) => return py_base_exception,
            };
        vm.new_exception_msg(py_type_ref, message)
    }
}
struct CandidError {}
impl CandidError {
    fn new(
        vm: &rustpython_vm::VirtualMachine,
        message: String,
    ) -> rustpython_vm::builtins::PyBaseExceptionRef {
        KybraError::subtype(vm, "CandidError", message)
    }
}
#[ic_cdk_macros::init()]
#[candid::candid_method(init)]
fn init() {}
#[ic_cdk_macros::post_upgrade()]
fn post_upgrade() {
    ic_cdk_timers::set_timer(std::time::Duration::from_secs(0), move || {
        let randomness = RANDOMNESS_STABLE_REF_CELL
            .with(|randomness_stable_ref_cell| randomness_stable_ref_cell.borrow().get().clone());
        if randomness.len() == 0 {
            ic_cdk::trap("Post Upgrade Error: randomness cannot have length 0");
        }
        unsafe {
            ic_wasi_polyfill::init(&randomness);
        };
        let interpreter = rustpython_vm::Interpreter::with_init(Default::default(), |vm| {
            vm.add_native_modules(rustpython_stdlib::get_module_inits());
            vm.add_frozen(rustpython_vm::py_freeze!(dir = "python_source"));
            PYTHON_STDLIB_STABLE_REF_CELL.with(|python_stdlib_stable_ref_cell| {
                let mut python_stdlib_stable_ref = python_stdlib_stable_ref_cell.borrow();
                let python_stdlib_bytes_reference: &'static [u8] =
                    python_stdlib_stable_ref.get().clone().leak();
                vm.add_frozen(rustpython_compiler_core::frozen_lib::FrozenLib::from_ref(
                    python_stdlib_bytes_reference,
                ));
            });
        });
        let scope = interpreter.enter(|vm| vm.new_scope_with_builtins());
        let vm = &interpreter.vm;
        Ic::make_class(&vm.ctx);
        vm.builtins
            .set_attr("_kybra_ic", vm.new_pyobj(Ic {}), vm)
            .unwrap_or_trap(vm);
        vm.run_code_string(
            scope.clone(),
            &format!("from {} import *", "main"),
            "".to_owned(),
        )
        .unwrap_or_trap(vm);
        unsafe {
            INTERPRETER_OPTION = Some(interpreter);
            SCOPE_OPTION = Some(scope);
        };
        let interpreter = unsafe { INTERPRETER_OPTION.as_mut() }
            .unwrap_or_trap("SystemError: missing python interpreter");
        let vm = &interpreter.vm;
        if CANISTER_INITIALIZED_REF_CELL
            .with(|canister_initialized_ref_cell| *canister_initialized_ref_cell.borrow().get())
            == 0
        {
        } else {
        }
        CANISTER_INITIALIZED_REF_CELL.with(|canister_initialized_ref_cell| {
            canister_initialized_ref_cell
                .borrow_mut()
                .set(1)
                .unwrap_or_trap()
        });
    });
}
#[ic_cdk_macros::query(name = "get_message")]
#[candid::candid_method(query, rename = "get_message")]
async fn _cdk_user_defined_get_message() -> (String) {
    let interpreter = unsafe { INTERPRETER_OPTION.as_mut() }
        .unwrap_or_trap("SystemError: missing python interpreter");
    let vm = &interpreter.vm;
    let params = ();
    call_global_python_function("get_message", params)
        .await
        .unwrap_or_trap()
}
#[ic_cdk_macros::query(name = "get_topic")]
#[candid::candid_method(query, rename = "get_topic")]
async fn _cdk_user_defined_get_topic() -> (String) {
    let interpreter = unsafe { INTERPRETER_OPTION.as_mut() }
        .unwrap_or_trap("SystemError: missing python interpreter");
    let vm = &interpreter.vm;
    let params = ();
    call_global_python_function("get_topic", params)
        .await
        .unwrap_or_trap()
}
#[ic_cdk_macros::update(name = "set_message")]
#[candid::candid_method(update, rename = "set_message")]
async fn _cdk_user_defined_set_message(_cdk_user_defined_new_message: String) -> () {
    let interpreter = unsafe { INTERPRETER_OPTION.as_mut() }
        .unwrap_or_trap("SystemError: missing python interpreter");
    let vm = &interpreter.vm;
    let params = (_cdk_user_defined_new_message
        .try_into_vm_value(vm)
        .unwrap_or_trap(),);
    call_global_python_function("set_message", params)
        .await
        .unwrap_or_trap()
}
#[ic_cdk_macros::update(name = "set_prompt")]
#[candid::candid_method(update, rename = "set_prompt")]
async fn _cdk_user_defined_set_prompt(
    _cdk_user_defined_faculty: String,
    _cdk_user_defined_year_of_study: candid::Int,
    _cdk_user_defined_program_of_study: String,
    _cdk_user_defined_domain: String,
) -> () {
    let interpreter = unsafe { INTERPRETER_OPTION.as_mut() }
        .unwrap_or_trap("SystemError: missing python interpreter");
    let vm = &interpreter.vm;
    let params = (
        _cdk_user_defined_faculty
            .try_into_vm_value(vm)
            .unwrap_or_trap(),
        _cdk_user_defined_year_of_study
            .try_into_vm_value(vm)
            .unwrap_or_trap(),
        _cdk_user_defined_program_of_study
            .try_into_vm_value(vm)
            .unwrap_or_trap(),
        _cdk_user_defined_domain
            .try_into_vm_value(vm)
            .unwrap_or_trap(),
    );
    call_global_python_function("set_prompt", params)
        .await
        .unwrap_or_trap()
}
candid :: define_function ! (pub HttpTransformFunc : (HttpTransformArgs) -> (HttpResponse) query);
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef>
    for HttpTransformFunc
{
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        self.0.try_into_vm_value(vm)
    }
}
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef>
    for Vec<HttpTransformFunc>
{
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        try_into_vm_value_generic_array(self, vm)
    }
}
impl
    CdkActTryFromVmValue<
        HttpTransformFunc,
        rustpython_vm::builtins::PyBaseExceptionRef,
        &rustpython::vm::VirtualMachine,
    > for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<HttpTransformFunc, rustpython_vm::builtins::PyBaseExceptionRef> {
        let candid_func: ic_cdk::export::candid::Func = self.try_from_vm_value(vm)?;
        Ok(HttpTransformFunc::new(
            candid_func.principal,
            candid_func.method,
        ))
    }
}
impl
    CdkActTryFromVmValue<
        Vec<HttpTransformFunc>,
        rustpython_vm::builtins::PyBaseExceptionRef,
        &rustpython::vm::VirtualMachine,
    > for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<Vec<HttpTransformFunc>, rustpython_vm::builtins::PyBaseExceptionRef> {
        try_from_vm_value_generic_array(self, vm)
    }
}
impl std::cmp::Ord for HttpTransformFunc {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
    }
}
impl std::cmp::PartialOrd for HttpTransformFunc {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        None
    }
}
#[derive(
    serde :: Deserialize,
    Debug,
    candid :: CandidType,
    Clone,
    CdkActTryIntoVmValue,
    CdkActTryFromVmValue,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
)]
struct GetBalanceArgs {
    address: Box<BitcoinAddress>,
    min_confirmations: Box<Option<u32>>,
    network: Box<BitcoinNetwork>,
}
#[derive(
    serde :: Deserialize,
    Debug,
    candid :: CandidType,
    Clone,
    CdkActTryIntoVmValue,
    CdkActTryFromVmValue,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
)]
struct GetCurrentFeePercentilesArgs {
    network: Box<BitcoinNetwork>,
}
#[derive(
    serde :: Deserialize,
    Debug,
    candid :: CandidType,
    Clone,
    CdkActTryIntoVmValue,
    CdkActTryFromVmValue,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
)]
struct GetUtxosArgs {
    address: Box<BitcoinAddress>,
    filter: Box<Option<UtxosFilter>>,
    network: Box<BitcoinNetwork>,
}
#[derive(
    serde :: Deserialize,
    Debug,
    candid :: CandidType,
    Clone,
    CdkActTryIntoVmValue,
    CdkActTryFromVmValue,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
)]
struct Outpoint {
    txid: Box<Vec<u8>>,
    vout: Box<u32>,
}
#[derive(
    serde :: Deserialize,
    Debug,
    candid :: CandidType,
    Clone,
    CdkActTryIntoVmValue,
    CdkActTryFromVmValue,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
)]
struct Utxo {
    height: Box<u32>,
    outpoint: Box<Outpoint>,
    value: Box<Satoshi>,
}
#[derive(
    serde :: Deserialize,
    Debug,
    candid :: CandidType,
    Clone,
    CdkActTryIntoVmValue,
    CdkActTryFromVmValue,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
)]
struct GetUtxosResult {
    next_page: Box<Option<Page>>,
    tip_block_hash: Box<BlockHash>,
    tip_height: Box<u32>,
    utxos: Box<Vec<Utxo>>,
}
#[derive(
    serde :: Deserialize,
    Debug,
    candid :: CandidType,
    Clone,
    CdkActTryIntoVmValue,
    CdkActTryFromVmValue,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
)]
struct SendTransactionArgs {
    transaction: Box<Vec<u8>>,
    network: Box<BitcoinNetwork>,
}
#[derive(
    serde :: Deserialize,
    Debug,
    candid :: CandidType,
    Clone,
    CdkActTryIntoVmValue,
    CdkActTryFromVmValue,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
)]
struct HttpHeader {
    name: Box<String>,
    value: Box<String>,
}
#[derive(
    serde :: Deserialize,
    Debug,
    candid :: CandidType,
    Clone,
    CdkActTryIntoVmValue,
    CdkActTryFromVmValue,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
)]
struct HttpResponse {
    status: Box<candid::Nat>,
    headers: Box<Vec<HttpHeader>>,
    body: Box<Vec<u8>>,
}
#[derive(
    serde :: Deserialize,
    Debug,
    candid :: CandidType,
    Clone,
    CdkActTryIntoVmValue,
    CdkActTryFromVmValue,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
)]
struct HttpTransformArgs {
    response: Box<HttpResponse>,
    context: Box<Vec<u8>>,
}
#[derive(
    serde :: Deserialize,
    Debug,
    candid :: CandidType,
    Clone,
    CdkActTryIntoVmValue,
    CdkActTryFromVmValue,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
)]
struct HttpTransform {
    function: Box<HttpTransformFunc>,
    context: Box<Vec<u8>>,
}
#[derive(
    serde :: Deserialize,
    Debug,
    candid :: CandidType,
    Clone,
    CdkActTryIntoVmValue,
    CdkActTryFromVmValue,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
)]
struct HttpRequestArgs {
    url: Box<String>,
    max_response_bytes: Box<Option<u64>>,
    method: Box<HttpMethod>,
    headers: Box<Vec<HttpHeader>>,
    body: Box<Option<Vec<u8>>>,
    transform: Box<Option<HttpTransform>>,
}
#[derive(
    serde :: Deserialize,
    Debug,
    candid :: CandidType,
    Clone,
    CdkActTryIntoVmValue,
    CdkActTryFromVmValue,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
)]
struct KeyId {
    curve: Box<EcdsaCurve>,
    name: Box<String>,
}
#[derive(
    serde :: Deserialize,
    Debug,
    candid :: CandidType,
    Clone,
    CdkActTryIntoVmValue,
    CdkActTryFromVmValue,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
)]
struct EcdsaPublicKeyArgs {
    canister_id: Box<Option<candid::Principal>>,
    derivation_path: Box<Vec<Vec<u8>>>,
    key_id: Box<KeyId>,
}
#[derive(
    serde :: Deserialize,
    Debug,
    candid :: CandidType,
    Clone,
    CdkActTryIntoVmValue,
    CdkActTryFromVmValue,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
)]
struct SignWithEcdsaArgs {
    message_hash: Box<Vec<u8>>,
    derivation_path: Box<Vec<Vec<u8>>>,
    key_id: Box<KeyId>,
}
#[derive(
    serde :: Deserialize,
    Debug,
    candid :: CandidType,
    Clone,
    CdkActTryIntoVmValue,
    CdkActTryFromVmValue,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
)]
struct EcdsaPublicKeyResult {
    public_key: Box<Vec<u8>>,
    chain_code: Box<Vec<u8>>,
}
#[derive(
    serde :: Deserialize,
    Debug,
    candid :: CandidType,
    Clone,
    CdkActTryIntoVmValue,
    CdkActTryFromVmValue,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
)]
struct SignWithEcdsaResult {
    signature: Box<Vec<u8>>,
}
#[derive(
    serde :: Deserialize,
    Debug,
    candid :: CandidType,
    Clone,
    CdkActTryIntoVmValue,
    CdkActTryFromVmValue,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
)]
struct CreateCanisterArgs {
    settings: Box<Option<CanisterSettings>>,
}
#[derive(
    serde :: Deserialize,
    Debug,
    candid :: CandidType,
    Clone,
    CdkActTryIntoVmValue,
    CdkActTryFromVmValue,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
)]
struct CanisterSettings {
    controllers: Box<Option<Vec<candid::Principal>>>,
    compute_allocation: Box<Option<candid::Nat>>,
    memory_allocation: Box<Option<candid::Nat>>,
    freezing_threshold: Box<Option<candid::Nat>>,
}
#[derive(
    serde :: Deserialize,
    Debug,
    candid :: CandidType,
    Clone,
    CdkActTryIntoVmValue,
    CdkActTryFromVmValue,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
)]
struct DefiniteCanisterSettings {
    controllers: Box<Vec<candid::Principal>>,
    compute_allocation: Box<candid::Nat>,
    memory_allocation: Box<candid::Nat>,
    freezing_threshold: Box<candid::Nat>,
}
#[derive(
    serde :: Deserialize,
    Debug,
    candid :: CandidType,
    Clone,
    CdkActTryIntoVmValue,
    CdkActTryFromVmValue,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
)]
struct CreateCanisterResult {
    canister_id: Box<candid::Principal>,
}
#[derive(
    serde :: Deserialize,
    Debug,
    candid :: CandidType,
    Clone,
    CdkActTryIntoVmValue,
    CdkActTryFromVmValue,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
)]
struct UpdateSettingsArgs {
    canister_id: Box<candid::Principal>,
    settings: Box<CanisterSettings>,
}
#[derive(
    serde :: Deserialize,
    Debug,
    candid :: CandidType,
    Clone,
    CdkActTryIntoVmValue,
    CdkActTryFromVmValue,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
)]
struct InstallCodeArgs {
    mode: Box<InstallCodeMode>,
    canister_id: Box<candid::Principal>,
    wasm_module: Box<Vec<u8>>,
    arg: Box<Vec<u8>>,
}
#[derive(
    serde :: Deserialize,
    Debug,
    candid :: CandidType,
    Clone,
    CdkActTryIntoVmValue,
    CdkActTryFromVmValue,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
)]
struct UninstallCodeArgs {
    canister_id: Box<candid::Principal>,
}
#[derive(
    serde :: Deserialize,
    Debug,
    candid :: CandidType,
    Clone,
    CdkActTryIntoVmValue,
    CdkActTryFromVmValue,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
)]
struct StartCanisterArgs {
    canister_id: Box<candid::Principal>,
}
#[derive(
    serde :: Deserialize,
    Debug,
    candid :: CandidType,
    Clone,
    CdkActTryIntoVmValue,
    CdkActTryFromVmValue,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
)]
struct StopCanisterArgs {
    canister_id: Box<candid::Principal>,
}
#[derive(
    serde :: Deserialize,
    Debug,
    candid :: CandidType,
    Clone,
    CdkActTryIntoVmValue,
    CdkActTryFromVmValue,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
)]
struct CanisterStatusArgs {
    canister_id: Box<candid::Principal>,
}
#[derive(
    serde :: Deserialize,
    Debug,
    candid :: CandidType,
    Clone,
    CdkActTryIntoVmValue,
    CdkActTryFromVmValue,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
)]
struct CanisterStatusResult {
    status: Box<CanisterStatus>,
    settings: Box<DefiniteCanisterSettings>,
    module_hash: Box<Option<Vec<u8>>>,
    memory_size: Box<candid::Nat>,
    cycles: Box<candid::Nat>,
}
#[derive(
    serde :: Deserialize,
    Debug,
    candid :: CandidType,
    Clone,
    CdkActTryIntoVmValue,
    CdkActTryFromVmValue,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
)]
struct DeleteCanisterArgs {
    canister_id: Box<candid::Principal>,
}
#[derive(
    serde :: Deserialize,
    Debug,
    candid :: CandidType,
    Clone,
    CdkActTryIntoVmValue,
    CdkActTryFromVmValue,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
)]
struct DepositCyclesArgs {
    canister_id: Box<candid::Principal>,
}
#[derive(
    serde :: Deserialize,
    Debug,
    candid :: CandidType,
    Clone,
    CdkActTryIntoVmValue,
    CdkActTryFromVmValue,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
)]
struct ProvisionalCreateCanisterWithCyclesArgs {
    amount: Box<Option<candid::Nat>>,
    settings: Box<Option<CanisterSettings>>,
}
#[derive(
    serde :: Deserialize,
    Debug,
    candid :: CandidType,
    Clone,
    CdkActTryIntoVmValue,
    CdkActTryFromVmValue,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
)]
struct ProvisionalCreateCanisterWithCyclesResult {
    canister_id: Box<candid::Principal>,
}
#[derive(
    serde :: Deserialize,
    Debug,
    candid :: CandidType,
    Clone,
    CdkActTryIntoVmValue,
    CdkActTryFromVmValue,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
)]
struct ProvisionalTopUpCanisterArgs {
    canister_id: Box<candid::Principal>,
    amount: Box<candid::Nat>,
}
#[derive(
    serde :: Deserialize,
    Debug,
    candid :: CandidType,
    Clone,
    CdkActTryIntoVmValue,
    CdkActTryFromVmValue,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
)]
struct KeyTooLarge {
    given: Box<u32>,
    max: Box<u32>,
}
#[derive(
    serde :: Deserialize,
    Debug,
    candid :: CandidType,
    Clone,
    CdkActTryIntoVmValue,
    CdkActTryFromVmValue,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
)]
struct ValueTooLarge {
    given: Box<u32>,
    max: Box<u32>,
}
candid :: define_service ! (ManagementCanister : { "bitcoin_get_balance" : candid :: func ! ((GetBalanceArgs) -> (Satoshi)) ; "bitcoin_get_current_fee_percentiles" : candid :: func ! ((GetCurrentFeePercentilesArgs) -> (Vec < MillisatoshiPerByte >)) ; "bitcoin_get_utxos" : candid :: func ! ((GetUtxosArgs) -> (GetUtxosResult)) ; "bitcoin_send_transaction" : candid :: func ! ((SendTransactionArgs) -> ()) ; "create_canister" : candid :: func ! ((CreateCanisterArgs) -> (CreateCanisterResult)) ; "update_settings" : candid :: func ! ((UpdateSettingsArgs) -> ()) ; "install_code" : candid :: func ! ((InstallCodeArgs) -> ()) ; "uninstall_code" : candid :: func ! ((UninstallCodeArgs) -> ()) ; "start_canister" : candid :: func ! ((StartCanisterArgs) -> ()) ; "stop_canister" : candid :: func ! ((StopCanisterArgs) -> ()) ; "canister_status" : candid :: func ! ((CanisterStatusArgs) -> (CanisterStatusResult)) ; "delete_canister" : candid :: func ! ((DeleteCanisterArgs) -> ()) ; "deposit_cycles" : candid :: func ! ((DepositCyclesArgs) -> ()) ; "raw_rand" : candid :: func ! (() -> (Vec < u8 >)) ; "provisional_create_canister_with_cycles" : candid :: func ! ((ProvisionalCreateCanisterWithCyclesArgs) -> (ProvisionalCreateCanisterWithCyclesResult)) ; "provisional_top_up_canister" : candid :: func ! ((ProvisionalTopUpCanisterArgs) -> ()) ; "http_request" : candid :: func ! ((HttpRequestArgs) -> (HttpResponse)) ; "ecdsa_public_key" : candid :: func ! ((EcdsaPublicKeyArgs) -> (EcdsaPublicKeyResult)) ; "sign_with_ecdsa" : candid :: func ! ((SignWithEcdsaArgs) -> (SignWithEcdsaResult)) });
impl std::cmp::Ord for ManagementCanister {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
    }
}
impl std::cmp::PartialOrd for ManagementCanister {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        None
    }
}
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef>
    for ManagementCanister
{
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        unsafe {
            let scope = match SCOPE_OPTION.clone() {
                Some(scope) => scope,
                None => {
                    return Err(CdkActTryIntoVmValueError(
                        "SystemError: missing python scope".to_string(),
                    ))
                }
            };
            Ok(vm
                .run_block_expr(
                    scope,
                    format!(
                        "from kybra import Principal; {}(Principal.from_str('{}'))",
                        stringify!(ManagementCanister),
                        self.0.principal.to_string()
                    )
                    .as_str(),
                )
                .map_err(|err| err.to_cdk_act_try_into_vm_value_error(vm))?)
        }
    }
}
impl CdkActTryIntoVmValue<&rustpython::vm::VirtualMachine, rustpython::vm::PyObjectRef>
    for Vec<ManagementCanister>
{
    fn try_into_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<rustpython::vm::PyObjectRef, CdkActTryIntoVmValueError> {
        try_into_vm_value_generic_array(self, vm)
    }
}
impl
    CdkActTryFromVmValue<
        ManagementCanister,
        rustpython_vm::builtins::PyBaseExceptionRef,
        &rustpython::vm::VirtualMachine,
    > for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<ManagementCanister, rustpython_vm::builtins::PyBaseExceptionRef> {
        let canister_id = self.get_attr("canister_id", vm)?;
        let to_str = canister_id.get_attr("to_str", vm)?;
        let result = to_str.call((), vm)?;
        let result_string: String = result.try_into_value(vm)?;
        match ic_cdk::export::Principal::from_text(result_string) {
            Ok(principal) => Ok(ManagementCanister::new(principal)),
            Err(err) => Err(vm.new_type_error(format!(
                "TypeError: Could not convert value to Principal: {}",
                err.to_string()
            ))),
        }
    }
}
impl
    CdkActTryFromVmValue<
        Vec<ManagementCanister>,
        rustpython_vm::builtins::PyBaseExceptionRef,
        &rustpython::vm::VirtualMachine,
    > for rustpython::vm::PyObjectRef
{
    fn try_from_vm_value(
        self,
        vm: &rustpython::vm::VirtualMachine,
    ) -> Result<Vec<ManagementCanister>, rustpython_vm::builtins::PyBaseExceptionRef> {
        try_from_vm_value_generic_array(self, vm)
    }
}
#[allow(non_snake_case)]
async fn call_ManagementCanister_bitcoin_get_balance(
    canister_id_principal: candid::Principal,
    params: (GetBalanceArgs,),
) -> ic_cdk::api::call::CallResult<(Satoshi,)> {
    ic_cdk::api::call::call(canister_id_principal, "bitcoin_get_balance", params).await
}
#[allow(non_snake_case)]
async fn call_with_payment_ManagementCanister_bitcoin_get_balance(
    canister_id_principal: candid::Principal,
    params: (GetBalanceArgs,),
    cycles: u64,
) -> ic_cdk::api::call::CallResult<(Satoshi,)> {
    ic_cdk::api::call::call_with_payment(
        canister_id_principal,
        "bitcoin_get_balance",
        params,
        cycles,
    )
    .await
}
#[allow(non_snake_case)]
async fn call_with_payment128_ManagementCanister_bitcoin_get_balance(
    canister_id_principal: candid::Principal,
    params: (GetBalanceArgs,),
    cycles: u128,
) -> ic_cdk::api::call::CallResult<(Satoshi,)> {
    ic_cdk::api::call::call_with_payment128(
        canister_id_principal,
        "bitcoin_get_balance",
        params,
        cycles,
    )
    .await
}
#[allow(non_snake_case)]
fn notify_ManagementCanister_bitcoin_get_balance(
    canister_id_principal: candid::Principal,
    params: (GetBalanceArgs,),
) -> Result<(), ic_cdk::api::call::RejectionCode> {
    ic_cdk::api::call::notify(canister_id_principal, "bitcoin_get_balance", params)
}
#[allow(non_snake_case)]
fn notify_with_payment128_ManagementCanister_bitcoin_get_balance(
    canister_id_principal: candid::Principal,
    params: (GetBalanceArgs,),
    cycles: u128,
) -> Result<(), ic_cdk::api::call::RejectionCode> {
    ic_cdk::api::call::notify_with_payment128(
        canister_id_principal,
        "bitcoin_get_balance",
        params,
        cycles,
    )
}
#[allow(non_snake_case)]
async fn call_ManagementCanister_bitcoin_get_current_fee_percentiles(
    canister_id_principal: candid::Principal,
    params: (GetCurrentFeePercentilesArgs,),
) -> ic_cdk::api::call::CallResult<(Vec<MillisatoshiPerByte>,)> {
    ic_cdk::api::call::call(
        canister_id_principal,
        "bitcoin_get_current_fee_percentiles",
        params,
    )
    .await
}
#[allow(non_snake_case)]
async fn call_with_payment_ManagementCanister_bitcoin_get_current_fee_percentiles(
    canister_id_principal: candid::Principal,
    params: (GetCurrentFeePercentilesArgs,),
    cycles: u64,
) -> ic_cdk::api::call::CallResult<(Vec<MillisatoshiPerByte>,)> {
    ic_cdk::api::call::call_with_payment(
        canister_id_principal,
        "bitcoin_get_current_fee_percentiles",
        params,
        cycles,
    )
    .await
}
#[allow(non_snake_case)]
async fn call_with_payment128_ManagementCanister_bitcoin_get_current_fee_percentiles(
    canister_id_principal: candid::Principal,
    params: (GetCurrentFeePercentilesArgs,),
    cycles: u128,
) -> ic_cdk::api::call::CallResult<(Vec<MillisatoshiPerByte>,)> {
    ic_cdk::api::call::call_with_payment128(
        canister_id_principal,
        "bitcoin_get_current_fee_percentiles",
        params,
        cycles,
    )
    .await
}
#[allow(non_snake_case)]
fn notify_ManagementCanister_bitcoin_get_current_fee_percentiles(
    canister_id_principal: candid::Principal,
    params: (GetCurrentFeePercentilesArgs,),
) -> Result<(), ic_cdk::api::call::RejectionCode> {
    ic_cdk::api::call::notify(
        canister_id_principal,
        "bitcoin_get_current_fee_percentiles",
        params,
    )
}
#[allow(non_snake_case)]
fn notify_with_payment128_ManagementCanister_bitcoin_get_current_fee_percentiles(
    canister_id_principal: candid::Principal,
    params: (GetCurrentFeePercentilesArgs,),
    cycles: u128,
) -> Result<(), ic_cdk::api::call::RejectionCode> {
    ic_cdk::api::call::notify_with_payment128(
        canister_id_principal,
        "bitcoin_get_current_fee_percentiles",
        params,
        cycles,
    )
}
#[allow(non_snake_case)]
async fn call_ManagementCanister_bitcoin_get_utxos(
    canister_id_principal: candid::Principal,
    params: (GetUtxosArgs,),
) -> ic_cdk::api::call::CallResult<(GetUtxosResult,)> {
    ic_cdk::api::call::call(canister_id_principal, "bitcoin_get_utxos", params).await
}
#[allow(non_snake_case)]
async fn call_with_payment_ManagementCanister_bitcoin_get_utxos(
    canister_id_principal: candid::Principal,
    params: (GetUtxosArgs,),
    cycles: u64,
) -> ic_cdk::api::call::CallResult<(GetUtxosResult,)> {
    ic_cdk::api::call::call_with_payment(canister_id_principal, "bitcoin_get_utxos", params, cycles)
        .await
}
#[allow(non_snake_case)]
async fn call_with_payment128_ManagementCanister_bitcoin_get_utxos(
    canister_id_principal: candid::Principal,
    params: (GetUtxosArgs,),
    cycles: u128,
) -> ic_cdk::api::call::CallResult<(GetUtxosResult,)> {
    ic_cdk::api::call::call_with_payment128(
        canister_id_principal,
        "bitcoin_get_utxos",
        params,
        cycles,
    )
    .await
}
#[allow(non_snake_case)]
fn notify_ManagementCanister_bitcoin_get_utxos(
    canister_id_principal: candid::Principal,
    params: (GetUtxosArgs,),
) -> Result<(), ic_cdk::api::call::RejectionCode> {
    ic_cdk::api::call::notify(canister_id_principal, "bitcoin_get_utxos", params)
}
#[allow(non_snake_case)]
fn notify_with_payment128_ManagementCanister_bitcoin_get_utxos(
    canister_id_principal: candid::Principal,
    params: (GetUtxosArgs,),
    cycles: u128,
) -> Result<(), ic_cdk::api::call::RejectionCode> {
    ic_cdk::api::call::notify_with_payment128(
        canister_id_principal,
        "bitcoin_get_utxos",
        params,
        cycles,
    )
}
#[allow(non_snake_case)]
async fn call_ManagementCanister_bitcoin_send_transaction(
    canister_id_principal: candid::Principal,
    params: (SendTransactionArgs,),
) -> ic_cdk::api::call::CallResult<()> {
    ic_cdk::api::call::call(canister_id_principal, "bitcoin_send_transaction", params).await
}
#[allow(non_snake_case)]
async fn call_with_payment_ManagementCanister_bitcoin_send_transaction(
    canister_id_principal: candid::Principal,
    params: (SendTransactionArgs,),
    cycles: u64,
) -> ic_cdk::api::call::CallResult<()> {
    ic_cdk::api::call::call_with_payment(
        canister_id_principal,
        "bitcoin_send_transaction",
        params,
        cycles,
    )
    .await
}
#[allow(non_snake_case)]
async fn call_with_payment128_ManagementCanister_bitcoin_send_transaction(
    canister_id_principal: candid::Principal,
    params: (SendTransactionArgs,),
    cycles: u128,
) -> ic_cdk::api::call::CallResult<()> {
    ic_cdk::api::call::call_with_payment128(
        canister_id_principal,
        "bitcoin_send_transaction",
        params,
        cycles,
    )
    .await
}
#[allow(non_snake_case)]
fn notify_ManagementCanister_bitcoin_send_transaction(
    canister_id_principal: candid::Principal,
    params: (SendTransactionArgs,),
) -> Result<(), ic_cdk::api::call::RejectionCode> {
    ic_cdk::api::call::notify(canister_id_principal, "bitcoin_send_transaction", params)
}
#[allow(non_snake_case)]
fn notify_with_payment128_ManagementCanister_bitcoin_send_transaction(
    canister_id_principal: candid::Principal,
    params: (SendTransactionArgs,),
    cycles: u128,
) -> Result<(), ic_cdk::api::call::RejectionCode> {
    ic_cdk::api::call::notify_with_payment128(
        canister_id_principal,
        "bitcoin_send_transaction",
        params,
        cycles,
    )
}
#[allow(non_snake_case)]
async fn call_ManagementCanister_create_canister(
    canister_id_principal: candid::Principal,
    params: (CreateCanisterArgs,),
) -> ic_cdk::api::call::CallResult<(CreateCanisterResult,)> {
    ic_cdk::api::call::call(canister_id_principal, "create_canister", params).await
}
#[allow(non_snake_case)]
async fn call_with_payment_ManagementCanister_create_canister(
    canister_id_principal: candid::Principal,
    params: (CreateCanisterArgs,),
    cycles: u64,
) -> ic_cdk::api::call::CallResult<(CreateCanisterResult,)> {
    ic_cdk::api::call::call_with_payment(canister_id_principal, "create_canister", params, cycles)
        .await
}
#[allow(non_snake_case)]
async fn call_with_payment128_ManagementCanister_create_canister(
    canister_id_principal: candid::Principal,
    params: (CreateCanisterArgs,),
    cycles: u128,
) -> ic_cdk::api::call::CallResult<(CreateCanisterResult,)> {
    ic_cdk::api::call::call_with_payment128(
        canister_id_principal,
        "create_canister",
        params,
        cycles,
    )
    .await
}
#[allow(non_snake_case)]
fn notify_ManagementCanister_create_canister(
    canister_id_principal: candid::Principal,
    params: (CreateCanisterArgs,),
) -> Result<(), ic_cdk::api::call::RejectionCode> {
    ic_cdk::api::call::notify(canister_id_principal, "create_canister", params)
}
#[allow(non_snake_case)]
fn notify_with_payment128_ManagementCanister_create_canister(
    canister_id_principal: candid::Principal,
    params: (CreateCanisterArgs,),
    cycles: u128,
) -> Result<(), ic_cdk::api::call::RejectionCode> {
    ic_cdk::api::call::notify_with_payment128(
        canister_id_principal,
        "create_canister",
        params,
        cycles,
    )
}
#[allow(non_snake_case)]
async fn call_ManagementCanister_update_settings(
    canister_id_principal: candid::Principal,
    params: (UpdateSettingsArgs,),
) -> ic_cdk::api::call::CallResult<()> {
    ic_cdk::api::call::call(canister_id_principal, "update_settings", params).await
}
#[allow(non_snake_case)]
async fn call_with_payment_ManagementCanister_update_settings(
    canister_id_principal: candid::Principal,
    params: (UpdateSettingsArgs,),
    cycles: u64,
) -> ic_cdk::api::call::CallResult<()> {
    ic_cdk::api::call::call_with_payment(canister_id_principal, "update_settings", params, cycles)
        .await
}
#[allow(non_snake_case)]
async fn call_with_payment128_ManagementCanister_update_settings(
    canister_id_principal: candid::Principal,
    params: (UpdateSettingsArgs,),
    cycles: u128,
) -> ic_cdk::api::call::CallResult<()> {
    ic_cdk::api::call::call_with_payment128(
        canister_id_principal,
        "update_settings",
        params,
        cycles,
    )
    .await
}
#[allow(non_snake_case)]
fn notify_ManagementCanister_update_settings(
    canister_id_principal: candid::Principal,
    params: (UpdateSettingsArgs,),
) -> Result<(), ic_cdk::api::call::RejectionCode> {
    ic_cdk::api::call::notify(canister_id_principal, "update_settings", params)
}
#[allow(non_snake_case)]
fn notify_with_payment128_ManagementCanister_update_settings(
    canister_id_principal: candid::Principal,
    params: (UpdateSettingsArgs,),
    cycles: u128,
) -> Result<(), ic_cdk::api::call::RejectionCode> {
    ic_cdk::api::call::notify_with_payment128(
        canister_id_principal,
        "update_settings",
        params,
        cycles,
    )
}
#[allow(non_snake_case)]
async fn call_ManagementCanister_install_code(
    canister_id_principal: candid::Principal,
    params: (InstallCodeArgs,),
) -> ic_cdk::api::call::CallResult<()> {
    ic_cdk::api::call::call(canister_id_principal, "install_code", params).await
}
#[allow(non_snake_case)]
async fn call_with_payment_ManagementCanister_install_code(
    canister_id_principal: candid::Principal,
    params: (InstallCodeArgs,),
    cycles: u64,
) -> ic_cdk::api::call::CallResult<()> {
    ic_cdk::api::call::call_with_payment(canister_id_principal, "install_code", params, cycles)
        .await
}
#[allow(non_snake_case)]
async fn call_with_payment128_ManagementCanister_install_code(
    canister_id_principal: candid::Principal,
    params: (InstallCodeArgs,),
    cycles: u128,
) -> ic_cdk::api::call::CallResult<()> {
    ic_cdk::api::call::call_with_payment128(canister_id_principal, "install_code", params, cycles)
        .await
}
#[allow(non_snake_case)]
fn notify_ManagementCanister_install_code(
    canister_id_principal: candid::Principal,
    params: (InstallCodeArgs,),
) -> Result<(), ic_cdk::api::call::RejectionCode> {
    ic_cdk::api::call::notify(canister_id_principal, "install_code", params)
}
#[allow(non_snake_case)]
fn notify_with_payment128_ManagementCanister_install_code(
    canister_id_principal: candid::Principal,
    params: (InstallCodeArgs,),
    cycles: u128,
) -> Result<(), ic_cdk::api::call::RejectionCode> {
    ic_cdk::api::call::notify_with_payment128(canister_id_principal, "install_code", params, cycles)
}
#[allow(non_snake_case)]
async fn call_ManagementCanister_uninstall_code(
    canister_id_principal: candid::Principal,
    params: (UninstallCodeArgs,),
) -> ic_cdk::api::call::CallResult<()> {
    ic_cdk::api::call::call(canister_id_principal, "uninstall_code", params).await
}
#[allow(non_snake_case)]
async fn call_with_payment_ManagementCanister_uninstall_code(
    canister_id_principal: candid::Principal,
    params: (UninstallCodeArgs,),
    cycles: u64,
) -> ic_cdk::api::call::CallResult<()> {
    ic_cdk::api::call::call_with_payment(canister_id_principal, "uninstall_code", params, cycles)
        .await
}
#[allow(non_snake_case)]
async fn call_with_payment128_ManagementCanister_uninstall_code(
    canister_id_principal: candid::Principal,
    params: (UninstallCodeArgs,),
    cycles: u128,
) -> ic_cdk::api::call::CallResult<()> {
    ic_cdk::api::call::call_with_payment128(canister_id_principal, "uninstall_code", params, cycles)
        .await
}
#[allow(non_snake_case)]
fn notify_ManagementCanister_uninstall_code(
    canister_id_principal: candid::Principal,
    params: (UninstallCodeArgs,),
) -> Result<(), ic_cdk::api::call::RejectionCode> {
    ic_cdk::api::call::notify(canister_id_principal, "uninstall_code", params)
}
#[allow(non_snake_case)]
fn notify_with_payment128_ManagementCanister_uninstall_code(
    canister_id_principal: candid::Principal,
    params: (UninstallCodeArgs,),
    cycles: u128,
) -> Result<(), ic_cdk::api::call::RejectionCode> {
    ic_cdk::api::call::notify_with_payment128(
        canister_id_principal,
        "uninstall_code",
        params,
        cycles,
    )
}
#[allow(non_snake_case)]
async fn call_ManagementCanister_start_canister(
    canister_id_principal: candid::Principal,
    params: (StartCanisterArgs,),
) -> ic_cdk::api::call::CallResult<()> {
    ic_cdk::api::call::call(canister_id_principal, "start_canister", params).await
}
#[allow(non_snake_case)]
async fn call_with_payment_ManagementCanister_start_canister(
    canister_id_principal: candid::Principal,
    params: (StartCanisterArgs,),
    cycles: u64,
) -> ic_cdk::api::call::CallResult<()> {
    ic_cdk::api::call::call_with_payment(canister_id_principal, "start_canister", params, cycles)
        .await
}
#[allow(non_snake_case)]
async fn call_with_payment128_ManagementCanister_start_canister(
    canister_id_principal: candid::Principal,
    params: (StartCanisterArgs,),
    cycles: u128,
) -> ic_cdk::api::call::CallResult<()> {
    ic_cdk::api::call::call_with_payment128(canister_id_principal, "start_canister", params, cycles)
        .await
}
#[allow(non_snake_case)]
fn notify_ManagementCanister_start_canister(
    canister_id_principal: candid::Principal,
    params: (StartCanisterArgs,),
) -> Result<(), ic_cdk::api::call::RejectionCode> {
    ic_cdk::api::call::notify(canister_id_principal, "start_canister", params)
}
#[allow(non_snake_case)]
fn notify_with_payment128_ManagementCanister_start_canister(
    canister_id_principal: candid::Principal,
    params: (StartCanisterArgs,),
    cycles: u128,
) -> Result<(), ic_cdk::api::call::RejectionCode> {
    ic_cdk::api::call::notify_with_payment128(
        canister_id_principal,
        "start_canister",
        params,
        cycles,
    )
}
#[allow(non_snake_case)]
async fn call_ManagementCanister_stop_canister(
    canister_id_principal: candid::Principal,
    params: (StopCanisterArgs,),
) -> ic_cdk::api::call::CallResult<()> {
    ic_cdk::api::call::call(canister_id_principal, "stop_canister", params).await
}
#[allow(non_snake_case)]
async fn call_with_payment_ManagementCanister_stop_canister(
    canister_id_principal: candid::Principal,
    params: (StopCanisterArgs,),
    cycles: u64,
) -> ic_cdk::api::call::CallResult<()> {
    ic_cdk::api::call::call_with_payment(canister_id_principal, "stop_canister", params, cycles)
        .await
}
#[allow(non_snake_case)]
async fn call_with_payment128_ManagementCanister_stop_canister(
    canister_id_principal: candid::Principal,
    params: (StopCanisterArgs,),
    cycles: u128,
) -> ic_cdk::api::call::CallResult<()> {
    ic_cdk::api::call::call_with_payment128(canister_id_principal, "stop_canister", params, cycles)
        .await
}
#[allow(non_snake_case)]
fn notify_ManagementCanister_stop_canister(
    canister_id_principal: candid::Principal,
    params: (StopCanisterArgs,),
) -> Result<(), ic_cdk::api::call::RejectionCode> {
    ic_cdk::api::call::notify(canister_id_principal, "stop_canister", params)
}
#[allow(non_snake_case)]
fn notify_with_payment128_ManagementCanister_stop_canister(
    canister_id_principal: candid::Principal,
    params: (StopCanisterArgs,),
    cycles: u128,
) -> Result<(), ic_cdk::api::call::RejectionCode> {
    ic_cdk::api::call::notify_with_payment128(
        canister_id_principal,
        "stop_canister",
        params,
        cycles,
    )
}
#[allow(non_snake_case)]
async fn call_ManagementCanister_canister_status(
    canister_id_principal: candid::Principal,
    params: (CanisterStatusArgs,),
) -> ic_cdk::api::call::CallResult<(CanisterStatusResult,)> {
    ic_cdk::api::call::call(canister_id_principal, "canister_status", params).await
}
#[allow(non_snake_case)]
async fn call_with_payment_ManagementCanister_canister_status(
    canister_id_principal: candid::Principal,
    params: (CanisterStatusArgs,),
    cycles: u64,
) -> ic_cdk::api::call::CallResult<(CanisterStatusResult,)> {
    ic_cdk::api::call::call_with_payment(canister_id_principal, "canister_status", params, cycles)
        .await
}
#[allow(non_snake_case)]
async fn call_with_payment128_ManagementCanister_canister_status(
    canister_id_principal: candid::Principal,
    params: (CanisterStatusArgs,),
    cycles: u128,
) -> ic_cdk::api::call::CallResult<(CanisterStatusResult,)> {
    ic_cdk::api::call::call_with_payment128(
        canister_id_principal,
        "canister_status",
        params,
        cycles,
    )
    .await
}
#[allow(non_snake_case)]
fn notify_ManagementCanister_canister_status(
    canister_id_principal: candid::Principal,
    params: (CanisterStatusArgs,),
) -> Result<(), ic_cdk::api::call::RejectionCode> {
    ic_cdk::api::call::notify(canister_id_principal, "canister_status", params)
}
#[allow(non_snake_case)]
fn notify_with_payment128_ManagementCanister_canister_status(
    canister_id_principal: candid::Principal,
    params: (CanisterStatusArgs,),
    cycles: u128,
) -> Result<(), ic_cdk::api::call::RejectionCode> {
    ic_cdk::api::call::notify_with_payment128(
        canister_id_principal,
        "canister_status",
        params,
        cycles,
    )
}
#[allow(non_snake_case)]
async fn call_ManagementCanister_delete_canister(
    canister_id_principal: candid::Principal,
    params: (DeleteCanisterArgs,),
) -> ic_cdk::api::call::CallResult<()> {
    ic_cdk::api::call::call(canister_id_principal, "delete_canister", params).await
}
#[allow(non_snake_case)]
async fn call_with_payment_ManagementCanister_delete_canister(
    canister_id_principal: candid::Principal,
    params: (DeleteCanisterArgs,),
    cycles: u64,
) -> ic_cdk::api::call::CallResult<()> {
    ic_cdk::api::call::call_with_payment(canister_id_principal, "delete_canister", params, cycles)
        .await
}
#[allow(non_snake_case)]
async fn call_with_payment128_ManagementCanister_delete_canister(
    canister_id_principal: candid::Principal,
    params: (DeleteCanisterArgs,),
    cycles: u128,
) -> ic_cdk::api::call::CallResult<()> {
    ic_cdk::api::call::call_with_payment128(
        canister_id_principal,
        "delete_canister",
        params,
        cycles,
    )
    .await
}
#[allow(non_snake_case)]
fn notify_ManagementCanister_delete_canister(
    canister_id_principal: candid::Principal,
    params: (DeleteCanisterArgs,),
) -> Result<(), ic_cdk::api::call::RejectionCode> {
    ic_cdk::api::call::notify(canister_id_principal, "delete_canister", params)
}
#[allow(non_snake_case)]
fn notify_with_payment128_ManagementCanister_delete_canister(
    canister_id_principal: candid::Principal,
    params: (DeleteCanisterArgs,),
    cycles: u128,
) -> Result<(), ic_cdk::api::call::RejectionCode> {
    ic_cdk::api::call::notify_with_payment128(
        canister_id_principal,
        "delete_canister",
        params,
        cycles,
    )
}
#[allow(non_snake_case)]
async fn call_ManagementCanister_deposit_cycles(
    canister_id_principal: candid::Principal,
    params: (DepositCyclesArgs,),
) -> ic_cdk::api::call::CallResult<()> {
    ic_cdk::api::call::call(canister_id_principal, "deposit_cycles", params).await
}
#[allow(non_snake_case)]
async fn call_with_payment_ManagementCanister_deposit_cycles(
    canister_id_principal: candid::Principal,
    params: (DepositCyclesArgs,),
    cycles: u64,
) -> ic_cdk::api::call::CallResult<()> {
    ic_cdk::api::call::call_with_payment(canister_id_principal, "deposit_cycles", params, cycles)
        .await
}
#[allow(non_snake_case)]
async fn call_with_payment128_ManagementCanister_deposit_cycles(
    canister_id_principal: candid::Principal,
    params: (DepositCyclesArgs,),
    cycles: u128,
) -> ic_cdk::api::call::CallResult<()> {
    ic_cdk::api::call::call_with_payment128(canister_id_principal, "deposit_cycles", params, cycles)
        .await
}
#[allow(non_snake_case)]
fn notify_ManagementCanister_deposit_cycles(
    canister_id_principal: candid::Principal,
    params: (DepositCyclesArgs,),
) -> Result<(), ic_cdk::api::call::RejectionCode> {
    ic_cdk::api::call::notify(canister_id_principal, "deposit_cycles", params)
}
#[allow(non_snake_case)]
fn notify_with_payment128_ManagementCanister_deposit_cycles(
    canister_id_principal: candid::Principal,
    params: (DepositCyclesArgs,),
    cycles: u128,
) -> Result<(), ic_cdk::api::call::RejectionCode> {
    ic_cdk::api::call::notify_with_payment128(
        canister_id_principal,
        "deposit_cycles",
        params,
        cycles,
    )
}
#[allow(non_snake_case)]
async fn call_ManagementCanister_raw_rand(
    canister_id_principal: candid::Principal,
    params: (),
) -> ic_cdk::api::call::CallResult<(Vec<u8>,)> {
    ic_cdk::api::call::call(canister_id_principal, "raw_rand", params).await
}
#[allow(non_snake_case)]
async fn call_with_payment_ManagementCanister_raw_rand(
    canister_id_principal: candid::Principal,
    params: (),
    cycles: u64,
) -> ic_cdk::api::call::CallResult<(Vec<u8>,)> {
    ic_cdk::api::call::call_with_payment(canister_id_principal, "raw_rand", params, cycles).await
}
#[allow(non_snake_case)]
async fn call_with_payment128_ManagementCanister_raw_rand(
    canister_id_principal: candid::Principal,
    params: (),
    cycles: u128,
) -> ic_cdk::api::call::CallResult<(Vec<u8>,)> {
    ic_cdk::api::call::call_with_payment128(canister_id_principal, "raw_rand", params, cycles).await
}
#[allow(non_snake_case)]
fn notify_ManagementCanister_raw_rand(
    canister_id_principal: candid::Principal,
    params: (),
) -> Result<(), ic_cdk::api::call::RejectionCode> {
    ic_cdk::api::call::notify(canister_id_principal, "raw_rand", params)
}
#[allow(non_snake_case)]
fn notify_with_payment128_ManagementCanister_raw_rand(
    canister_id_principal: candid::Principal,
    params: (),
    cycles: u128,
) -> Result<(), ic_cdk::api::call::RejectionCode> {
    ic_cdk::api::call::notify_with_payment128(canister_id_principal, "raw_rand", params, cycles)
}
#[allow(non_snake_case)]
async fn call_ManagementCanister_provisional_create_canister_with_cycles(
    canister_id_principal: candid::Principal,
    params: (ProvisionalCreateCanisterWithCyclesArgs,),
) -> ic_cdk::api::call::CallResult<(ProvisionalCreateCanisterWithCyclesResult,)> {
    ic_cdk::api::call::call(
        canister_id_principal,
        "provisional_create_canister_with_cycles",
        params,
    )
    .await
}
#[allow(non_snake_case)]
async fn call_with_payment_ManagementCanister_provisional_create_canister_with_cycles(
    canister_id_principal: candid::Principal,
    params: (ProvisionalCreateCanisterWithCyclesArgs,),
    cycles: u64,
) -> ic_cdk::api::call::CallResult<(ProvisionalCreateCanisterWithCyclesResult,)> {
    ic_cdk::api::call::call_with_payment(
        canister_id_principal,
        "provisional_create_canister_with_cycles",
        params,
        cycles,
    )
    .await
}
#[allow(non_snake_case)]
async fn call_with_payment128_ManagementCanister_provisional_create_canister_with_cycles(
    canister_id_principal: candid::Principal,
    params: (ProvisionalCreateCanisterWithCyclesArgs,),
    cycles: u128,
) -> ic_cdk::api::call::CallResult<(ProvisionalCreateCanisterWithCyclesResult,)> {
    ic_cdk::api::call::call_with_payment128(
        canister_id_principal,
        "provisional_create_canister_with_cycles",
        params,
        cycles,
    )
    .await
}
#[allow(non_snake_case)]
fn notify_ManagementCanister_provisional_create_canister_with_cycles(
    canister_id_principal: candid::Principal,
    params: (ProvisionalCreateCanisterWithCyclesArgs,),
) -> Result<(), ic_cdk::api::call::RejectionCode> {
    ic_cdk::api::call::notify(
        canister_id_principal,
        "provisional_create_canister_with_cycles",
        params,
    )
}
#[allow(non_snake_case)]
fn notify_with_payment128_ManagementCanister_provisional_create_canister_with_cycles(
    canister_id_principal: candid::Principal,
    params: (ProvisionalCreateCanisterWithCyclesArgs,),
    cycles: u128,
) -> Result<(), ic_cdk::api::call::RejectionCode> {
    ic_cdk::api::call::notify_with_payment128(
        canister_id_principal,
        "provisional_create_canister_with_cycles",
        params,
        cycles,
    )
}
#[allow(non_snake_case)]
async fn call_ManagementCanister_provisional_top_up_canister(
    canister_id_principal: candid::Principal,
    params: (ProvisionalTopUpCanisterArgs,),
) -> ic_cdk::api::call::CallResult<()> {
    ic_cdk::api::call::call(canister_id_principal, "provisional_top_up_canister", params).await
}
#[allow(non_snake_case)]
async fn call_with_payment_ManagementCanister_provisional_top_up_canister(
    canister_id_principal: candid::Principal,
    params: (ProvisionalTopUpCanisterArgs,),
    cycles: u64,
) -> ic_cdk::api::call::CallResult<()> {
    ic_cdk::api::call::call_with_payment(
        canister_id_principal,
        "provisional_top_up_canister",
        params,
        cycles,
    )
    .await
}
#[allow(non_snake_case)]
async fn call_with_payment128_ManagementCanister_provisional_top_up_canister(
    canister_id_principal: candid::Principal,
    params: (ProvisionalTopUpCanisterArgs,),
    cycles: u128,
) -> ic_cdk::api::call::CallResult<()> {
    ic_cdk::api::call::call_with_payment128(
        canister_id_principal,
        "provisional_top_up_canister",
        params,
        cycles,
    )
    .await
}
#[allow(non_snake_case)]
fn notify_ManagementCanister_provisional_top_up_canister(
    canister_id_principal: candid::Principal,
    params: (ProvisionalTopUpCanisterArgs,),
) -> Result<(), ic_cdk::api::call::RejectionCode> {
    ic_cdk::api::call::notify(canister_id_principal, "provisional_top_up_canister", params)
}
#[allow(non_snake_case)]
fn notify_with_payment128_ManagementCanister_provisional_top_up_canister(
    canister_id_principal: candid::Principal,
    params: (ProvisionalTopUpCanisterArgs,),
    cycles: u128,
) -> Result<(), ic_cdk::api::call::RejectionCode> {
    ic_cdk::api::call::notify_with_payment128(
        canister_id_principal,
        "provisional_top_up_canister",
        params,
        cycles,
    )
}
#[allow(non_snake_case)]
async fn call_ManagementCanister_http_request(
    canister_id_principal: candid::Principal,
    params: (HttpRequestArgs,),
) -> ic_cdk::api::call::CallResult<(HttpResponse,)> {
    ic_cdk::api::call::call(canister_id_principal, "http_request", params).await
}
#[allow(non_snake_case)]
async fn call_with_payment_ManagementCanister_http_request(
    canister_id_principal: candid::Principal,
    params: (HttpRequestArgs,),
    cycles: u64,
) -> ic_cdk::api::call::CallResult<(HttpResponse,)> {
    ic_cdk::api::call::call_with_payment(canister_id_principal, "http_request", params, cycles)
        .await
}
#[allow(non_snake_case)]
async fn call_with_payment128_ManagementCanister_http_request(
    canister_id_principal: candid::Principal,
    params: (HttpRequestArgs,),
    cycles: u128,
) -> ic_cdk::api::call::CallResult<(HttpResponse,)> {
    ic_cdk::api::call::call_with_payment128(canister_id_principal, "http_request", params, cycles)
        .await
}
#[allow(non_snake_case)]
fn notify_ManagementCanister_http_request(
    canister_id_principal: candid::Principal,
    params: (HttpRequestArgs,),
) -> Result<(), ic_cdk::api::call::RejectionCode> {
    ic_cdk::api::call::notify(canister_id_principal, "http_request", params)
}
#[allow(non_snake_case)]
fn notify_with_payment128_ManagementCanister_http_request(
    canister_id_principal: candid::Principal,
    params: (HttpRequestArgs,),
    cycles: u128,
) -> Result<(), ic_cdk::api::call::RejectionCode> {
    ic_cdk::api::call::notify_with_payment128(canister_id_principal, "http_request", params, cycles)
}
#[allow(non_snake_case)]
async fn call_ManagementCanister_ecdsa_public_key(
    canister_id_principal: candid::Principal,
    params: (EcdsaPublicKeyArgs,),
) -> ic_cdk::api::call::CallResult<(EcdsaPublicKeyResult,)> {
    ic_cdk::api::call::call(canister_id_principal, "ecdsa_public_key", params).await
}
#[allow(non_snake_case)]
async fn call_with_payment_ManagementCanister_ecdsa_public_key(
    canister_id_principal: candid::Principal,
    params: (EcdsaPublicKeyArgs,),
    cycles: u64,
) -> ic_cdk::api::call::CallResult<(EcdsaPublicKeyResult,)> {
    ic_cdk::api::call::call_with_payment(canister_id_principal, "ecdsa_public_key", params, cycles)
        .await
}
#[allow(non_snake_case)]
async fn call_with_payment128_ManagementCanister_ecdsa_public_key(
    canister_id_principal: candid::Principal,
    params: (EcdsaPublicKeyArgs,),
    cycles: u128,
) -> ic_cdk::api::call::CallResult<(EcdsaPublicKeyResult,)> {
    ic_cdk::api::call::call_with_payment128(
        canister_id_principal,
        "ecdsa_public_key",
        params,
        cycles,
    )
    .await
}
#[allow(non_snake_case)]
fn notify_ManagementCanister_ecdsa_public_key(
    canister_id_principal: candid::Principal,
    params: (EcdsaPublicKeyArgs,),
) -> Result<(), ic_cdk::api::call::RejectionCode> {
    ic_cdk::api::call::notify(canister_id_principal, "ecdsa_public_key", params)
}
#[allow(non_snake_case)]
fn notify_with_payment128_ManagementCanister_ecdsa_public_key(
    canister_id_principal: candid::Principal,
    params: (EcdsaPublicKeyArgs,),
    cycles: u128,
) -> Result<(), ic_cdk::api::call::RejectionCode> {
    ic_cdk::api::call::notify_with_payment128(
        canister_id_principal,
        "ecdsa_public_key",
        params,
        cycles,
    )
}
#[allow(non_snake_case)]
async fn call_ManagementCanister_sign_with_ecdsa(
    canister_id_principal: candid::Principal,
    params: (SignWithEcdsaArgs,),
) -> ic_cdk::api::call::CallResult<(SignWithEcdsaResult,)> {
    ic_cdk::api::call::call(canister_id_principal, "sign_with_ecdsa", params).await
}
#[allow(non_snake_case)]
async fn call_with_payment_ManagementCanister_sign_with_ecdsa(
    canister_id_principal: candid::Principal,
    params: (SignWithEcdsaArgs,),
    cycles: u64,
) -> ic_cdk::api::call::CallResult<(SignWithEcdsaResult,)> {
    ic_cdk::api::call::call_with_payment(canister_id_principal, "sign_with_ecdsa", params, cycles)
        .await
}
#[allow(non_snake_case)]
async fn call_with_payment128_ManagementCanister_sign_with_ecdsa(
    canister_id_principal: candid::Principal,
    params: (SignWithEcdsaArgs,),
    cycles: u128,
) -> ic_cdk::api::call::CallResult<(SignWithEcdsaResult,)> {
    ic_cdk::api::call::call_with_payment128(
        canister_id_principal,
        "sign_with_ecdsa",
        params,
        cycles,
    )
    .await
}
#[allow(non_snake_case)]
fn notify_ManagementCanister_sign_with_ecdsa(
    canister_id_principal: candid::Principal,
    params: (SignWithEcdsaArgs,),
) -> Result<(), ic_cdk::api::call::RejectionCode> {
    ic_cdk::api::call::notify(canister_id_principal, "sign_with_ecdsa", params)
}
#[allow(non_snake_case)]
fn notify_with_payment128_ManagementCanister_sign_with_ecdsa(
    canister_id_principal: candid::Principal,
    params: (SignWithEcdsaArgs,),
    cycles: u128,
) -> Result<(), ic_cdk::api::call::RejectionCode> {
    ic_cdk::api::call::notify_with_payment128(
        canister_id_principal,
        "sign_with_ecdsa",
        params,
        cycles,
    )
}
type BitcoinAddress = (String);
type BlockHash = (Vec<u8>);
type Page = (Vec<u8>);
type Satoshi = (u64);
type MillisatoshiPerByte = (u64);
type TimerId = (u64);
type Duration = (u64);
#[derive(
    serde :: Deserialize,
    Debug,
    candid :: CandidType,
    Clone,
    CdkActTryIntoVmValue,
    CdkActTryFromVmValue,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
)]
enum BitcoinNetwork {
    Mainnet(()),
    Regtest(()),
    Testnet(()),
}
#[derive(
    serde :: Deserialize,
    Debug,
    candid :: CandidType,
    Clone,
    CdkActTryIntoVmValue,
    CdkActTryFromVmValue,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
)]
enum UtxosFilter {
    MinConfirmations(u32),
    Page(Box<Page>),
}
#[derive(
    serde :: Deserialize,
    Debug,
    candid :: CandidType,
    Clone,
    CdkActTryIntoVmValue,
    CdkActTryFromVmValue,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
)]
enum SendTransactionError {
    MalformedTransaction(()),
    QueueFull(()),
}
#[derive(
    serde :: Deserialize,
    Debug,
    candid :: CandidType,
    Clone,
    CdkActTryIntoVmValue,
    CdkActTryFromVmValue,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
)]
enum HttpMethod {
    get(()),
    head(()),
    post(()),
}
#[derive(
    serde :: Deserialize,
    Debug,
    candid :: CandidType,
    Clone,
    CdkActTryIntoVmValue,
    CdkActTryFromVmValue,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
)]
enum EcdsaCurve {
    secp256k1(()),
}
#[derive(
    serde :: Deserialize,
    Debug,
    candid :: CandidType,
    Clone,
    CdkActTryIntoVmValue,
    CdkActTryFromVmValue,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
)]
enum InstallCodeMode {
    install(()),
    reinstall(()),
    upgrade(()),
}
#[derive(
    serde :: Deserialize,
    Debug,
    candid :: CandidType,
    Clone,
    CdkActTryIntoVmValue,
    CdkActTryFromVmValue,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
)]
enum CanisterStatus {
    running(()),
    stopping(()),
    stopped(()),
}
#[derive(
    serde :: Deserialize,
    Debug,
    candid :: CandidType,
    Clone,
    CdkActTryIntoVmValue,
    CdkActTryFromVmValue,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
)]
enum GuardResult {
    Ok(()),
    Err(String),
}
#[derive(
    serde :: Deserialize,
    Debug,
    candid :: CandidType,
    Clone,
    CdkActTryIntoVmValue,
    CdkActTryFromVmValue,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
)]
enum RejectionCode {
    NoError(()),
    SysFatal(()),
    SysTransient(()),
    DestinationInvalid(()),
    CanisterReject(()),
    CanisterError(()),
    Unknown(()),
}
#[derive(
    serde :: Deserialize,
    Debug,
    candid :: CandidType,
    Clone,
    CdkActTryIntoVmValue,
    CdkActTryFromVmValue,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
)]
enum NotifyResult {
    Ok(()),
    Err(Box<RejectionCode>),
}
#[derive(
    serde :: Deserialize,
    Debug,
    candid :: CandidType,
    Clone,
    CdkActTryIntoVmValue,
    CdkActTryFromVmValue,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
)]
enum StableMemoryError {
    OutOfMemory(()),
    OutOfBounds(()),
}
#[derive(
    serde :: Deserialize,
    Debug,
    candid :: CandidType,
    Clone,
    CdkActTryIntoVmValue,
    CdkActTryFromVmValue,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
)]
enum StableGrowResult {
    Ok(u32),
    Err(Box<StableMemoryError>),
}
#[derive(
    serde :: Deserialize,
    Debug,
    candid :: CandidType,
    Clone,
    CdkActTryIntoVmValue,
    CdkActTryFromVmValue,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
)]
enum Stable64GrowResult {
    Ok(u64),
    Err(Box<StableMemoryError>),
}
candid::export_service!();
#[no_mangle]
pub fn get_candid_pointer() -> *mut std::os::raw::c_char {
    let c_string = std::ffi::CString::new(__export_service()).unwrap();
    c_string.into_raw()
}
#[derive(serde :: Deserialize, Clone, Debug, candid :: CandidType)]
struct _CdkFloat64(f64);
impl std::cmp::Ord for _CdkFloat64 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Less)
    }
}
impl std::cmp::PartialOrd for _CdkFloat64 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl std::cmp::Eq for _CdkFloat64 {}
impl std::cmp::PartialEq for _CdkFloat64 {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}
#[derive(serde :: Deserialize, Clone, Debug, candid :: CandidType)]
struct _CdkFloat32(f32);
impl std::cmp::Ord for _CdkFloat32 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Less)
    }
}
impl std::cmp::PartialOrd for _CdkFloat32 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl std::cmp::Eq for _CdkFloat32 {}
impl std::cmp::PartialEq for _CdkFloat32 {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}
