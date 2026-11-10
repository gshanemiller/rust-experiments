#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MyErr {
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
  fn update(&mut self, newStatus: Status) -> Result<(), MyErr>;
}

struct JobStatusManager<'a> {
  d_status          : Status,
  d_baseSchemaName  : &'a String,
}

impl<'a> JobStatusManager<'a> {
  pub fn new(status: Status, baseSchemaName: &'a String) -> Self {
    debug_assert!(!baseSchemaName.is_empty());
    Self {
      d_status          : status,
      d_baseSchemaName  : baseSchemaName,
    }
  }
}

impl<'a> StatusManager for JobStatusManager<'a> {
  fn status(&self) -> Status {
    return self.d_status;
  }

  fn baseSchemaName(&mut self) -> &str {
    return self.d_baseSchemaName.as_str();
  }

  fn update(&mut self, newStatus: Status) -> Result<(), MyErr> {
    match self.d_status {
      Status::Queued    if newStatus==Status::Reserved  => self.d_status = newStatus,
      Status::Reserved  if newStatus==Status::Invoked   => self.d_status = newStatus,
      Status::Reserved  if newStatus==Status::Invalid   => self.d_status = newStatus,
      Status::Invoked   if newStatus==Status::Running   => self.d_status = newStatus,
      Status::Invoked   if newStatus==Status::Invalid   => self.d_status = newStatus,
      Status::Running   if newStatus==Status::Complete  => self.d_status = newStatus,
      Status::Running   if newStatus==Status::Failed    => self.d_status = newStatus,
      _ => {
        println!("status transition to '{}' from '{}' bad: revert to 'Invalid'\n",
          newStatus.as_str(), self.d_status.as_str());
        self.d_status = Status::Invalid;
        return Err(MyErr::JobError);
      },
    }

    return Ok(());
  }
}

struct ParentStatusManager<'a> {
  d_status          : Status,
  d_baseSchemaName  : &'a String,
}

impl<'a> ParentStatusManager<'a> {
  pub fn new(status: Status, baseSchemaName: &'a String) -> Self {
    debug_assert!(!baseSchemaName.is_empty());
    Self {
      d_status          : status,
      d_baseSchemaName  : baseSchemaName,
    }
  }
}

impl<'a> StatusManager for ParentStatusManager<'a> {
  fn status(&self) -> Status {
    return self.d_status;
  }

  fn baseSchemaName(&mut self) -> &str {
    return self.d_baseSchemaName.as_str();
  }

  fn update(&mut self, newStatus: Status) -> Result<(), MyErr> {
    match self.d_status {
      Status::Queued    if newStatus==Status::Reserved  => self.d_status = newStatus,
      Status::Reserved  if newStatus==Status::Running   => self.d_status = newStatus,
      Status::Reserved  if newStatus==Status::Invalid   => self.d_status = newStatus,
      Status::Running   if newStatus==Status::Complete  => self.d_status = newStatus,
      Status::Running   if newStatus==Status::Failed    => self.d_status = newStatus,
      Status::Running   if newStatus==Status::Invalid   => self.d_status = newStatus,
      _ => {
        println!("status transition to '{}' from '{}' for parent bad: revert to 'Invalid'\n",
          newStatus.as_str(), self.d_status.as_str());
        self.d_status = Status::Invalid;
        return Err(MyErr::JobError);
      },
    }

    return Ok(());
  }
}

struct TaskTransition<'a> {
  d_currStatus        : Status,
  d_nextStatus        : Status,
  d_parentStatus      : &'a [Status],
  d_nextParentStatus  : Option<Status>,
}

struct TaskStatusManager<'a> {
  d_status            : Status,
  d_baseSchemaName    : &'a String,
  d_parent            : &'a mut ParentStatusManager<'a>,
  d_transition        : &'a [TaskTransition<'a>; 7],
}

impl<'a> TaskStatusManager<'a> {
  pub fn new(status: Status, baseSchemaName: &'a String, parentStatus: &'a mut ParentStatusManager<'a>) -> Self {
    Self {
      d_status          : status,
      d_baseSchemaName  : baseSchemaName,
      d_parent          : parentStatus,    
      d_transition      : &[
        TaskTransition {
          d_currStatus        : Status::Queued,
          d_nextStatus        : Status::Reserved,
          d_parentStatus      : &[Status::Reserved],
          d_nextParentStatus  : None,
        },
        TaskTransition {
          d_currStatus        : Status::Reserved,
          d_nextStatus        : Status::Invoked,
          d_parentStatus      : &[Status::Reserved, Status::Running],
          d_nextParentStatus  : Some(Status::Running),
        },
        TaskTransition {
          d_currStatus        : Status::Reserved,
          d_nextStatus        : Status::Invalid,
          d_parentStatus      : &[Status::Reserved, Status::Running],
          d_nextParentStatus  : Some(Status::Invalid),
        },
        TaskTransition {
          d_currStatus        : Status::Invoked,
          d_nextStatus        : Status::Running,
          d_parentStatus      : &[Status::Running],
          d_nextParentStatus  : None,
        },
        TaskTransition {
          d_currStatus        : Status::Invoked,
          d_nextStatus        : Status::Invalid,
          d_parentStatus      : &[Status::Running],
          d_nextParentStatus  : Some(Status::Invalid),
        },
        TaskTransition {
          d_currStatus        : Status::Running,
          d_nextStatus        : Status::Complete,
          d_parentStatus      : &[Status::Running],
          d_nextParentStatus  : None,
        },
        TaskTransition {
          d_currStatus        : Status::Running,
          d_nextStatus        : Status::Failed,
          d_parentStatus      : &[Status::Running],
          d_nextParentStatus  : Some(Status::Failed),
        },
      ],
    }
  }
}

impl<'a> StatusManager for TaskStatusManager<'a> {
  fn status(&self) -> Status {
    return self.d_status;
  }

  fn baseSchemaName(&mut self) -> &str {
    return self.d_baseSchemaName.as_str();
  }

  fn update(&mut self, newStatus: Status) -> Result<(), MyErr> {
    for srcIdx in 0..self.d_transition.len() {
      if self.d_status == self.d_transition[srcIdx].d_currStatus && 
        newStatus == self.d_transition[srcIdx].d_nextStatus {
        for prtIdx in 0..self.d_transition[srcIdx].d_parentStatus.len() {
          if self.d_parent.status() == self.d_transition[srcIdx].d_parentStatus[prtIdx] {
            self.d_status = newStatus;
            if let Some(status) = self.d_transition[srcIdx].d_nextParentStatus {
              return self.d_parent.update(status);
            }
            return Ok(());
          }
        }
      }
    }
    return Err(MyErr::JobError);
  }
}

fn main() {
  let schemaName = String::from("foo");
  let mut parentStatus = ParentStatusManager::new(Status::Queued, &schemaName);
  let mut taskStatus = TaskStatusManager::new(Status::Queued, &schemaName, &mut parentStatus);
}
