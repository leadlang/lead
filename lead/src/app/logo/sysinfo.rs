use windows::{
  core::*,
  Win32::{
    Foundation::*,
    System::{
      Com::*,
      SystemInformation::*,
      Variant::{VariantToStringAlloc, VARIANT},
      Wmi::*,
    },
  },
};

pub static IS_SUPPORTED_SYSTEM: bool = true;

pub struct System {
  pub(self) memory: MEMORYSTATUSEX,
}

impl System {
  pub fn new_all() -> Self {
    unsafe {
      CoInitializeEx(None, COINIT_MULTITHREADED);

      CoInitializeSecurity(
        None,
        -1,
        None,
        None,
        RPC_C_AUTHN_LEVEL_DEFAULT,
        RPC_C_IMP_LEVEL_IMPERSONATE,
        None,
        EOAC_NONE,
        None,
      );
    }

    let mut memory = MEMORYSTATUSEX::default();
    memory.dwLength = size_of_val(&memory) as u32;

    unsafe {
      GlobalMemoryStatusEx(&mut memory);
    }

    Self { memory }
  }

  pub fn refresh_all(&mut self) -> &mut Self {
    self
  }

  pub fn cpus(&self) -> [Cpu; 1] {
    [Cpu]
  }

  pub fn used_memory(&self) -> u64 {
    self.total_memory() - self.memory.ullAvailPhys
  }

  pub fn total_memory(&self) -> u64 {
    self.memory.ullTotalPhys
  }
}

pub struct Cpu;

impl Cpu {
  pub fn brand(&self) -> String {
    unsafe {
      let wmi: IWbemLocator = CoCreateInstance(&WbemLocator, None, CLSCTX_INPROC_SERVER)
        .expect("Unable to get WbemLocator");

      let service = wmi
        .ConnectServer(
          &BSTR::from("ROOT\\CIMV2"),
          &BSTR::default(),
          &BSTR::default(),
          &BSTR::default(),
          0,
          &BSTR::default(),
          None,
        )
        .expect("Error");

      let query = service
        .ExecQuery(
          &BSTR::from("WQL"),
          &BSTR::from("SELECT Name FROM Win32_Processor"),
          WBEM_FLAG_FORWARD_ONLY,
          None,
        )
        .expect("Error getting cpu name");

      let mut objs: [Option<IWbemClassObject>; 1] = [None];
      let mut returned = 0;

      query.Next(WBEM_INFINITE, &mut objs, &mut returned).unwrap();

      let [obj] = objs;
      let obj = obj.unwrap();

      let mut value = VARIANT::default();
      obj.Get(w!("Name"), 0, &mut value, None, None);

      let mut resp = PWSTR::null();

      let typ = value.vt();
      let brand = VariantToStringAlloc(&value).unwrap().to_string().unwrap();

      brand
    }
  }
}
