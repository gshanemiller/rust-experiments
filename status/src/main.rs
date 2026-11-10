#[derive(Debug, PartialEq, Clone, Copy)]                                                                                
pub enum Err {
  JobError
}

#[derive(Debug, PartialEq, Clone, Copy)]                                                                                
pub enum Status {                                                                                                       
  Queued,                                                                                                               
  Reserved,                                                                                                             
  Invoked,                                                                                                              
  Running,                                                                                                              
  Complete,                                                                                                             
  Failed,                                                                                                               
  Invalid,                                                                                                              
}                                                                                                                       
                                                                                                                        
impl Status {                                                                                                           
  pub fn as_str(&self) -> &'static str {                                                                                
    match self {                                                                                                        
      Self::Queued =>   "QUEUED",                                                                                       
      Self::Reserved => "RESERVED",                                                                                     
      Self::Invoked =>  "INVOKED",                                                                                      
      Self::Running =>  "RUNNING",                                                                                      
      Self::Complete => "COMPLETE",                                                                                     
      // Invoked but failed                                                                                             
      Self::Failed =>   "FAILED",                                                                                       
      // Cannot be invoked                                                                                              
      Self::Invalid =>  "INVALID",                                                                                      
    }                                                                                                                   
  }                                                                                                                     
                                                                                                                        
  pub fn from_str(value: &str) -> Option<Status> {                                                                      
    match value {                                                                                                       
      "QUEUED" =>   Some(Self::Queued),                                                                                 
      "RESERVED" => Some(Self::Reserved),                                                                               
      "INVOKED" =>  Some(Self::Invoked),                                                                                
      "RUNNING" =>  Some(Self::Running),                                                                                
      "COMPLETE" => Some(Self::Complete),                                                                               
      "FAILED" =>   Some(Self::Failed),                                                                                 
      "INVALID" =>  Some(Self::Invalid),                                                                                
      _ => None,                                                                                                        
    }                                                                                                                   
  }                                                                                                                     
}                                                                                                                       

pub trait StatusManager {                                                                                               
  fn status(&self) -> Status;                                                                                           
  fn baseSchemaName(&mut self) -> &str;                                                                                 
  fn update(&mut self, newStatus: Status) -> Result<(), Error>;                                     
}                                                                                                                       

struct JobStatusManager {
  d_status          : Status,                                                                                           
  d_baseSchemaName  : &String,                                                                                          
}

impl JobStatusManager {                                                                                                 
  pub fn new(status: Status, &baseSchemaName: &String) -> Self {                                        
    debug_assert!(!baseSchemaName.is_empty());                                                                          
    Self {                                                                                                              
      d_status          : status,                                                                                       
      d_baseSchemaName  : baseSchemaName,                                                                               
    }                                                                                                                   
  }                                                                                                                     
}                                                                                                                       
                                                                                                                        
impl StatusManager for JobStatusManager {                                                                               
  pub fn status(&self) -> Status {                                                                                      
    return self.d_status;                                                                                               
  }                                                                                                                     
                                                                                                                        
  pub fn baseSchemaName(&mut self) -> &str {                                                                            
    return self.d_baseSchemaName.as_str();                                                                              
  }                                                                                                                     

 pub fn update(&mut self, newStatus: Status) -> Result<(), Error> {                                
    match self.d_status {                                                                                               
      Status::Queued    if value==Status::Reserved  => self.d_status = value,                                           
      Status::Reserved  if value==Status::Invoked   => self.d_status = value,                                           
      Status::Reserved  if value==Status::Invalid   => self.d_status = value,                                           
      Status::Invoked   if value==Status::Running   => self.d_status = value,                                           
      Status::Invoked   if value==Status::Invalid   => self.d_status = value,                                           
      Status::Running   if value==Status::Complete  => self.d_status = value,                                           
      Status::Running   if value==Status::Failed    => self.d_status = value,                                           
      _ => {                                                                                                            
        println!("status transition to '{}' from '{}' bad: revert to 'Invalid'\n",          
          newStatus.as_str(), self.d_status.as_str());                                                   
        self.d_status = Status::Invalid;                                                                                
      },
    }

    return Ok(());
  }
}

impl ParentStatusManager {                                                                                                 
  pub fn new(status: Status, &baseSchemaName: &String) -> Self {                                        
    debug_assert!(!baseSchemaName.is_empty());                                                                          
    Self {                                                                                                              
      d_status          : status,                                                                                       
      d_baseSchemaName  : baseSchemaName,                                                                               
    }                                                                                                                   
  }                                                                                                                     
}                                                                                                                       
                                                                                                                        
impl StatusManager for ParentStatusManager {                                                                               
  pub fn status(&self) -> Status {                                                                                      
    return self.d_status;                                                                                               
  }                                                                                                                     
                                                                                                                        
  pub fn baseSchemaName(&mut self) -> &str {                                                                            
    return self.d_baseSchemaName.as_str();                                                                              
  }                                                                                                                     

  pub fn update(&mut self, newStatus: Status) -> Result<(), Error> {
    match self.d_status {                                                                                               
      Status::Queued    if value==Status::Reserved  => self.d_status = value,                                           
      Status::Reserved  if value==Status::Running   => self.d_status = value,                                           
      Status::Reserved  if value==Status::Invalid   => self.d_status = value,                                           
      Status::Running   if value==Status::Complete  => self.d_status = value,                                           
      Status::Running   if value==Status::Failed    => self.d_status = value,                                           
      Status::Running   if value==Status::Invalid   => self.d_status = value,                                           
      _ => {                                                                                                            
        println("status transition to '{}' from '{}' for parent bad: revert to 'Invalid'\n",  
          newStatus.as_str(), self.d_status.as_str());                                                   
        self.d_status = Status::Invalid;                                                                                
      },                                                                                                                
    }

    return Ok(());
  }
}

struct TaskTransition {                                                                                        
  d_currStatus        : Status,                                                                                         
  d_nextStatus        : Status,                                                                                         
  d_parentStatus      : [Status],                                                                                      
  d_nextParentStatus  : Option<Status>,
}                                                                                                                       

struct TaskStatusManager {
  d_status          : Status,                                                                                           
  d_baseSchemaName  : &String,                                                                                          
  d_transition      : [TaskTransition; 7],
  d_parent          : &mut ParentStatusManager,
}
                                                                                                                        
impl TaskStatusManager {                                                                                       
  pub fn new(status: Status, baseSchemaName: &String, parentStatus: &mut ParentStatusManager) -> Self {
    Self {
      d_status          : Status,                                                                                           
      d_baseSchemaName  : &String,                                                                                          
      d_transition      : [TaskSetStatusTransition; 7] {
        TaskTransition {
          d_currStatus        : Status::Queued,
          d_nextStatus        : Status::Reserved,
          d_parentStatus      : [Status; 1] = [Status::Reserved],
          d_nextParentStatus  : None,
        },
        TaskTransition {
          d_currStatus        : Status::Reserved
          d_nextStatus        : Status::Invoked,
          d_parentStatus      : [Status; 2] = [Status::Reserved, Status::Running],
          d_nextParentStatus  : Some<Status::Running>,
        },
        TaskTransition {
          d_currStatus        : Status::Reserved
          d_nextStatus        : Status::Invalid,
          d_parentStatus      : [Status; 2] = [Status::Reserved, Status::Running],
          d_nextParentStatus  : Some<Status::Invalid>,
        },
        TaskTransition {
          d_currStatus        : Status::Invoked,
          d_nextStatus        : Status::Running,
          d_parentStatus      : [Status; 1] = [Status::Running],
          d_nextParentStatus  : None,
        },
        TaskTransition {
          d_currStatus        : Status::Invoked,
          d_nextStatus        : Status::Invalid,
          d_parentStatus      : [Status; 1] = [Status::Running],
          d_nextParentStatus  : Some<Status::Invalid>,
        },
        TaskTransition {
          d_currStatus        : Status::Running,
          d_nextStatus        : Status::Complete,
          d_parentStatus      : [Status; 1] = [Status::Running],
          d_nextParentStatus  : None,
        },
        TaskTransition {
          d_currStatus        : Status::Running,
          d_nextStatus        : Status::Failed,
          d_parentStatus      : [Status; 1] = [Status::Running],
          d_nextParentStatus  : Some<Status::Failed>,
        },
      },
    }
  }
}                                                                                                                       
                                                                                                                        
impl StatusManager for TaskStatusManager {                                                                               
  pub fn status(&self) -> Status {                                                                                      
    return self.d_status;                                                                                               
  }                                                                                                                     
                                                                                                                        
  pub fn baseSchemaName(&mut self) -> &str {                                                                            
    return self.d_baseSchemaName.as_str();                                                                              
  }                                                                                                                     

  pub fn update(&mut self, newStatus: Status) -> Result<(), Error> {
    return Ok(());
  }
}

fn main() {
}
