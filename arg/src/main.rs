use std::env;
use std::ffi::{CString};

#[derive(Debug)]
enum Error {
  BadCmdLine,
}

struct DBLogin {                                                            
  pub d_dbName: CString,                                                            
  pub d_userName: CString,                                                          
  pub d_sslPath: CString,                                                           
  pub d_hostName: CString,                                                          
  pub d_hostIPV4Address: CString,                                                   
  pub d_hostIPV4Port: CString,                                                      
}

impl DBLogin {
  pub fn new(dbName: &str, param: String) -> Self {
    Self {                                                                      
      d_dbName: CString::new(dbName).unwrap(),
      d_userName: CString::new(param).unwrap(),
      d_sslPath: CString::new("").unwrap(),
      d_hostName: CString::new("").unwrap(),
      d_hostIPV4Address: CString::new("").unwrap(),
      d_hostIPV4Port: CString::new("").unwrap(),
    }                                                                           
  }

  pub fn newAdmin(dbName: &str) -> Self {
    Self {                                                                      
      d_dbName: CString::new(dbName).unwrap(),                                  
      d_userName: CString::new("admin_app").unwrap(),
      d_sslPath: CString::new("").unwrap(),
      d_hostName: CString::new("").unwrap(),
      d_hostIPV4Address: CString::new("").unwrap(),
      d_hostIPV4Port: CString::new("").unwrap(),
    }                                                                           
  }

  pub fn newWorkflow(dbName: &str, readOnly: bool) -> Self {
    let mut obj = Self {                                                                      
      d_dbName: CString::new(dbName).unwrap(),                                  
      d_userName: CString::new("workflow_app_rw").unwrap(),
      d_sslPath: CString::new("").unwrap(),
      d_hostName: CString::new("").unwrap(),
      d_hostIPV4Address: CString::new("").unwrap(),
      d_hostIPV4Port: CString::new("").unwrap(),
    };
    if readOnly {
      obj.d_userName = CString::new("workflow_app_ro").unwrap();
    }
    return obj;
  }

  pub fn newBackend(dbName: &str, readOnly: bool) -> Self {
    let mut obj = Self {                                                                      
      d_dbName: CString::new(dbName).unwrap(),                                  
      d_userName: CString::new("backend_app_rw").unwrap(),
      d_sslPath: CString::new("").unwrap(),
      d_hostName: CString::new("").unwrap(),
      d_hostIPV4Address: CString::new("").unwrap(),
      d_hostIPV4Port: CString::new("").unwrap(),
    };                                                                         
    if readOnly {
      obj.d_userName = CString::new("backend_app_ro").unwrap();
    }
    return obj;
  }
}

struct Param {
  pub databaseName            : Option<String>,
  pub tmpParentDirPath        : Option<String>,
  pub pipeInPath              : Option<String>,
  pub pipeOutPath             : Option<String>,
  pub generalAllocMemoryBytes : Option<u64>,
  pub dbLogin                 : Vec<DBLogin>,
  pub cpu                     : Vec<u32>,
  pub queueRestriction        : Vec<String>,
}

impl Param {
  pub fn new() -> Self {
    Self {
      databaseName            : None,
      tmpParentDirPath        : None,
      pipeInPath              : None,
      pipeOutPath             : None,
      generalAllocMemoryBytes : None,
      dbLogin                 : vec![],
      cpu                     : vec![],
      queueRestriction        : vec![],
    }
  }
}

fn parse(cmdArg: &mut Vec<String>, param: &mut Param) -> Result<(), Error> {
  let mut used = vec![];
  let mut i: usize = 1;
  while i<cmdArg.len() {
    if cmdArg[i]=="--databaseName" && (i+1)<cmdArg.len() {
      param.databaseName = Some(cmdArg[i+1].clone());
      used.push(i);
      used.push(i+1);
      i += 2;
    } else if cmdArg[i]=="--tmpParentDirPath" && (i+1)<cmdArg.len() {
      param.tmpParentDirPath = Some(cmdArg[i+1].clone());
      used.push(i);
      used.push(i+1);
      i += 2;
    } else if cmdArg[i]=="--pipeInPath" && (i+1)<cmdArg.len() {
      param.pipeInPath = Some(cmdArg[i+1].clone());
      used.push(i);
      used.push(i+1);
      i += 2;
    } else if cmdArg[i]=="--pipeOutPath" && (i+1)<cmdArg.len() {
      param.pipeOutPath = Some(cmdArg[i+1].clone());
      used.push(i);
      used.push(i+1);
      i += 2;
    } else if cmdArg[i]=="--generalAllocMemoryBytes" && (i+1)<cmdArg.len() {
      match u64::from_str_radix(cmdArg[i+1].as_str(), 10) {
        Ok(bytes) if bytes>0 => param.generalAllocMemoryBytes = Some(bytes),
        Ok(_) => return Err(Error::BadCmdLine),
        Err(_e) => return Err(Error::BadCmdLine),
      }
      used.push(i);
      used.push(i+1);
      i += 2;
    } else if cmdArg[i]=="--defaultDbLoginReadOnly" && (i+1)<cmdArg.len() {
      if let Some(ref dbName) = param.databaseName { 
        if cmdArg[i+1].as_str()=="backend" {
          param.dbLogin.push(DBLogin::newBackend(dbName.as_str(), true));
        } else if cmdArg[i+1].as_str()=="workflow" {
          param.dbLogin.push(DBLogin::newWorkflow(dbName.as_str(), true));
        } else {
          return Err(Error::BadCmdLine);
        }
      } else {
        return Err(Error::BadCmdLine);
      }
      used.push(i);
      used.push(i+1);
      i += 2;
    } else if cmdArg[i]=="--defaultDbLogin" && (i+1)<cmdArg.len() {
      if let Some(ref dbName) = param.databaseName { 
        if cmdArg[i+1].as_str()=="backend" {
          param.dbLogin.push(DBLogin::newBackend(dbName.as_str(), false));
        } else if cmdArg[i+1].as_str()=="workflow" {
          param.dbLogin.push(DBLogin::newWorkflow(dbName.as_str(), false));
        } else {
          return Err(Error::BadCmdLine);
        }
      } else {
        return Err(Error::BadCmdLine);
      }
      used.push(i);
      used.push(i+1);
      i += 2;
    } else if cmdArg[i]=="--adminDbLogin" {
      if let Some(ref dbName) = param.databaseName { 
        param.dbLogin.push(DBLogin::newAdmin(dbName.as_str()));
      } else {
        return Err(Error::BadCmdLine);
      }
      used.push(i);
      i += 1;
    } else if cmdArg[i]=="--dbLogin" && (i+1)<cmdArg.len() {
      if let Some(ref dbName) = param.databaseName { 
        param.dbLogin.push(DBLogin::new(dbName.as_str(), cmdArg[i+1].clone()));
      } else {
        return Err(Error::BadCmdLine);
      }
      used.push(i);
      used.push(i+1);
      i += 2;
    } else if cmdArg[i]=="--cpu" && (i+1)<cmdArg.len() {
      match u32::from_str_radix(cmdArg[i+1].as_str(), 10) {
        Ok(cpu) => param.cpu.push(cpu),
        Err(_e) => return Err(Error::BadCmdLine),
      }
      used.push(i);
      used.push(i+1);
      i += 2;
    } else if cmdArg[i]=="--queueRestriction" && (i+1)<cmdArg.len() {
      param.queueRestriction.push(cmdArg[i+1].clone());
      used.push(i);
      used.push(i+1);
      i += 2;
    } else {
      i += 1;
    }
  }

  // wdd
  for k in used.iter().rev() {
    cmdArg.remove(*k);
  }
  
  return Ok(());
}

fn main() {
  {
    let test: [&str; 30] = [
      "taskName",
      "--databaseName",               "dbName",
      "--tmpParentDirPath",           "/tmp",
      "--pipeInPath",                 "/tmp/in",
      "--pipeOutPath",                "/tmp/out",
      "--generalAllocMemoryBytes",    "100",
      "--defaultDbLoginReadOnly",     "backend",
      "--defaultDbLoginReadOnly",     "workflow",
      "--defaultDbLogin",             "backend",
      "--defaultDbLogin",             "workflow",
      "--adminDbLogin",
      "--dbLogin",                    "stuff11",
      "--cpu",                        "0",
      "--cpu",                        "32",
      "--queueRestriction",           "a",
      "--queueRestriction",           "b",
    ];

    let mut testCmdLine: Vec<String> = vec![];
    for s in test {
      testCmdLine.push(s.to_string());
    }

    let mut param = Param::new();
    if let Err(e) = parse(&mut testCmdLine, &mut param) {
      panic!("failed {:?}", e)
    }

    assert_eq!("dbName",      param.databaseName.unwrap().as_str());
    assert_eq!("/tmp",        param.tmpParentDirPath.unwrap().as_str());
    assert_eq!("/tmp/in",     param.pipeInPath.unwrap().as_str());
    assert_eq!("/tmp/out",    param.pipeOutPath.unwrap().as_str());
    assert_eq!(100,           param.generalAllocMemoryBytes.unwrap());
    assert_eq!(vec![0,32],    param.cpu);
    assert_eq!(vec!["a","b"], param.queueRestriction);
    assert_eq!(6,             param.dbLogin.len());

    assert_eq!(CString::new("dbName").unwrap(),          param.dbLogin[0].d_dbName);
    assert_eq!(CString::new("backend_app_ro").unwrap(),  param.dbLogin[0].d_userName);
    assert_eq!(CString::new("dbName").unwrap(),          param.dbLogin[1].d_dbName);
    assert_eq!(CString::new("workflow_app_ro").unwrap(), param.dbLogin[1].d_userName);
    assert_eq!(CString::new("dbName").unwrap(),          param.dbLogin[2].d_dbName);
    assert_eq!(CString::new("backend_app_rw").unwrap(),  param.dbLogin[2].d_userName);
    assert_eq!(CString::new("dbName").unwrap(),          param.dbLogin[3].d_dbName);
    assert_eq!(CString::new("workflow_app_rw").unwrap(), param.dbLogin[3].d_userName);
    assert_eq!(CString::new("dbName").unwrap(),          param.dbLogin[4].d_dbName);
    assert_eq!(CString::new("admin_app").unwrap(),       param.dbLogin[4].d_userName);
    assert_eq!(CString::new("dbName").unwrap(),          param.dbLogin[5].d_dbName);
    assert_eq!(CString::new("stuff11").unwrap(),         param.dbLogin[5].d_userName);

    // Everything left
    assert_eq!(vec!["taskName"], testCmdLine);
  }

  {
    let mut testCmdLine: Vec<String> = vec![];
    testCmdLine.push("taskName".to_string());
    
    let mut param = Param::new();
    if let Err(e) = parse(&mut testCmdLine, &mut param) {
      panic!("failed {:?}", e)
    }

    assert_eq!(true,          param.databaseName.is_none());
    assert_eq!(true,          param.tmpParentDirPath.is_none());
    assert_eq!(true,          param.pipeInPath.is_none());
    assert_eq!(true,          param.pipeOutPath.is_none());
    assert_eq!(true,          param.generalAllocMemoryBytes.is_none());
    assert_eq!(0,             param.cpu.len());
    assert_eq!(0,             param.queueRestriction.len());
    assert_eq!(0,             param.dbLogin.len());

    // Everything left
    assert_eq!(vec!["taskName"], testCmdLine);
  }

  {
    let mut param = Param::new();
    let mut testCmdLine: Vec<String> = vec![];

    let test: [&str; 7] = [
      "-1",
      "4294967296",
      "4294967297",
      "abc",
      "q",
      "!",
      "*",
    ];
    
    for i in 0..test.len() {
      testCmdLine.clear();
      testCmdLine.push("taskName".to_string());
      testCmdLine.push("--cpu".to_string());
      testCmdLine.push(String::new());
      testCmdLine[2] = test[i].to_string();
      let copy = testCmdLine.clone();

      if let Ok(()) = parse(&mut testCmdLine, &mut param) {
        panic!("unexpected pass");
      }

      assert_eq!(true,          param.databaseName.is_none());
      assert_eq!(true,          param.tmpParentDirPath.is_none());
      assert_eq!(true,          param.pipeInPath.is_none());
      assert_eq!(true,          param.pipeOutPath.is_none());
      assert_eq!(true,          param.generalAllocMemoryBytes.is_none());
      assert_eq!(0,             param.cpu.len());
      assert_eq!(0,             param.queueRestriction.len());
      assert_eq!(0,             param.dbLogin.len());

      // Everything left
      assert_eq!(copy, testCmdLine);
    }
  }

  {
    let mut param = Param::new();
    let mut testCmdLine: Vec<String> = vec![];

    let test: [&str; 5] = [
      "-1",
      "abc",
      "q",
      "!",
      "*",
    ];
    
    for i in 0..test.len() {
      testCmdLine.clear();
      testCmdLine.push("taskName".to_string());
      testCmdLine.push("--generalAllocMemoryBytes".to_string());
      testCmdLine.push(String::new());
      testCmdLine[2] = test[i].to_string();
      let copy = testCmdLine.clone();

      if let Ok(()) = parse(&mut testCmdLine, &mut param) {
        panic!("unexpected pass");
      }

      assert_eq!(true,          param.databaseName.is_none());
      assert_eq!(true,          param.tmpParentDirPath.is_none());
      assert_eq!(true,          param.pipeInPath.is_none());
      assert_eq!(true,          param.pipeOutPath.is_none());
      assert_eq!(true,          param.generalAllocMemoryBytes.is_none());
      assert_eq!(0,             param.cpu.len());
      assert_eq!(0,             param.queueRestriction.len());
      assert_eq!(0,             param.dbLogin.len());

      // Everything left
      assert_eq!(copy, testCmdLine);
    }
  }

  {
    let mut param = Param::new();
    let mut testCmdLine: Vec<String> = vec![];
    testCmdLine.push("taskName".to_string());
    testCmdLine.push("--defaultDbLoginReadOnly".to_string());
    testCmdLine.push("backend".to_string());
    let copy = testCmdLine.clone();

    // Omit dbName which makes login incomplete
    if let Ok(()) = parse(&mut testCmdLine, &mut param) {
      panic!("unexpected pass");
    }

    assert_eq!(true,          param.databaseName.is_none());
    assert_eq!(true,          param.tmpParentDirPath.is_none());
    assert_eq!(true,          param.pipeInPath.is_none());
    assert_eq!(true,          param.pipeOutPath.is_none());
    assert_eq!(true,          param.generalAllocMemoryBytes.is_none());
    assert_eq!(0,             param.cpu.len());
    assert_eq!(0,             param.queueRestriction.len());
    assert_eq!(0,             param.dbLogin.len());

    // Everything left
    assert_eq!(copy, testCmdLine);
  }

  {
    let mut param = Param::new();
    let mut testCmdLine: Vec<String> = vec![];
    testCmdLine.push("taskName".to_string());
    testCmdLine.push("--defaultDbLogin".to_string());
    testCmdLine.push("backend".to_string());
    let copy = testCmdLine.clone();

    // Omit dbName which makes login incomplete
    if let Ok(()) = parse(&mut testCmdLine, &mut param) {
      panic!("unexpected pass");
    }

    assert_eq!(true,          param.databaseName.is_none());
    assert_eq!(true,          param.tmpParentDirPath.is_none());
    assert_eq!(true,          param.pipeInPath.is_none());
    assert_eq!(true,          param.pipeOutPath.is_none());
    assert_eq!(true,          param.generalAllocMemoryBytes.is_none());
    assert_eq!(0,             param.cpu.len());
    assert_eq!(0,             param.queueRestriction.len());
    assert_eq!(0,             param.dbLogin.len());

    // Everything left
    assert_eq!(copy, testCmdLine);
  }

  {
    let mut param = Param::new();
    let mut testCmdLine: Vec<String> = vec![];
    testCmdLine.push("taskName".to_string());
    testCmdLine.push("--adminDbLogin".to_string());
    let copy = testCmdLine.clone();

    // Omit dbName which makes login incomplete
    if let Ok(()) = parse(&mut testCmdLine, &mut param) {
      panic!("unexpected pass");
    }

    assert_eq!(true,          param.databaseName.is_none());
    assert_eq!(true,          param.tmpParentDirPath.is_none());
    assert_eq!(true,          param.pipeInPath.is_none());
    assert_eq!(true,          param.pipeOutPath.is_none());
    assert_eq!(true,          param.generalAllocMemoryBytes.is_none());
    assert_eq!(0,             param.cpu.len());
    assert_eq!(0,             param.queueRestriction.len());
    assert_eq!(0,             param.dbLogin.len());

    // Everything left
    assert_eq!(copy, testCmdLine);
  }

  {
    let mut param = Param::new();
    let mut testCmdLine: Vec<String> = vec![];
    testCmdLine.push("taskName".to_string());
    testCmdLine.push("--dbLogin".to_string());
    testCmdLine.push("stuff11".to_string());
    let copy = testCmdLine.clone();

    // Omit dbName which makes login incomplete
    if let Ok(()) = parse(&mut testCmdLine, &mut param) {
      panic!("unexpected pass");
    }

    assert_eq!(true,          param.databaseName.is_none());
    assert_eq!(true,          param.tmpParentDirPath.is_none());
    assert_eq!(true,          param.pipeInPath.is_none());
    assert_eq!(true,          param.pipeOutPath.is_none());
    assert_eq!(true,          param.generalAllocMemoryBytes.is_none());
    assert_eq!(0,             param.cpu.len());
    assert_eq!(0,             param.queueRestriction.len());
    assert_eq!(0,             param.dbLogin.len());

    // Everything left
    assert_eq!(copy, testCmdLine);
  }
}
