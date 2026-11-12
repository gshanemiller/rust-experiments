#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MyErr {
  JobError
}

#[repr(u32)]
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
      Self::Failed =>   "FAILED",
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

  pub fn from_u32(value: u32) -> Option<Status> {
    match value {
      0 => Some(Self::Queued),
      1 => Some(Self::Reserved),
      2 => Some(Self::Invoked),
      3 => Some(Self::Running),
      4 => Some(Self::Complete),
      5 => Some(Self::Failed),
      6 => Some(Self::Invalid),
      _ => None,
    }
  }

  const fn jobInitialState() -> Status {
    return Status::Queued;
  }

  const fn parentInitialState() -> Status {
    return Status::Reserved;
  }

  const fn taskInitialState() -> Status {
    return Status::Reserved;
  }

  fn updateAsJob(&mut self, newStatus: Status) -> Result<(), MyErr> {
    match *self {
      Status::Queued    if newStatus==Status::Reserved  => *self = newStatus,
      Status::Reserved  if newStatus==Status::Invoked   => *self = newStatus,
      Status::Reserved  if newStatus==Status::Invalid   => *self = newStatus,
      Status::Invoked   if newStatus==Status::Running   => *self = newStatus,
      Status::Invoked   if newStatus==Status::Invalid   => *self = newStatus,
      Status::Running   if newStatus==Status::Complete  => *self = newStatus,
      Status::Running   if newStatus==Status::Failed    => *self = newStatus,
      _ => {
        println!("jobStatus {:?} -> {:?} invalid", *self, newStatus);
        *self = Status::Invalid;
        return Err(MyErr::JobError);
      },
    }
    return Ok(());
  }

  fn updateAsTask(&mut self, newStatus: Status) -> Result<(), MyErr> {
    match *self {
      Status::Reserved  if newStatus==Status::Invoked   => *self = newStatus,
      Status::Reserved  if newStatus==Status::Invalid   => *self = newStatus,
      Status::Invoked   if newStatus==Status::Running   => *self = newStatus,
      Status::Invoked   if newStatus==Status::Invalid   => *self = newStatus,
      Status::Running   if newStatus==Status::Complete  => *self = newStatus,
      Status::Running   if newStatus==Status::Failed    => *self = newStatus,
      _ => {
        println!("taskStatus {:?} -> {:?} invalid", *self, newStatus);
        *self = Status::Invalid;
        return Err(MyErr::JobError);
      },
    }
    return Ok(());
  }

  fn updateAsParent(&mut self, taskStatus: &Vec<Status>) -> Result<(), MyErr> {
    let mut isInvalid = false;
    let mut isFailed  = false;
    let mut isComplete = true;
    let mut invokedCount: usize = 0;
    let mut reservedCount: usize = 0;
    let mut needTransition = false;
    let mut nextStatus = Status::Invalid;

    // Summarize task statuses
    for i in 0..taskStatus.len() {
      // Count tasks in Invoked state
      invokedCount += (taskStatus[i]==Status::Invoked) as usize;
      // Count tasks in Reserve state
      reservedCount += (taskStatus[i]==Status::Reserved) as usize;
      // Any invalid task makes parent Invalid
      isInvalid = isInvalid || taskStatus[i]==Status::Invalid;
      // Any failed task makes parent Failed provided no task Invalid
      isFailed = isFailed || taskStatus[i]==Status::Failed;
      // Parent is complete if all tasks Complete
      isComplete = isComplete && taskStatus[i]==Status::Complete;
    }

    // println!("Computed final: invokedCount {} reservedCount {} isInvalid {} isFailed {} isComplete {}",
    //  invokedCount, reservedCount, isInvalid, isFailed, isComplete);

    if *self==Status::Reserved && invokedCount==1 && reservedCount==taskStatus.len()-1 {
      // First task invocation transitions parent Reserved -> Running
      needTransition = true;
      nextStatus = Status::Running;
    } else if isInvalid {
      if *self != Status::Invalid {
        // Any Invalid task state makes parent x -> Invalid
        needTransition = true;
        nextStatus = Status::Invalid;
      }
    } else if isFailed && !isComplete && *self != Status::Failed {
      // Any Failed task state makes parent x -> Failed provided not Invalid
      // That is, Invalid takes precedence over Failed
      needTransition = true;
      nextStatus = Status::Failed;
    } else if isComplete && !isFailed && *self != Status::Complete {
      // If all tasks Complete parent is Complete
      needTransition = true;
      nextStatus = Status::Complete;
    }

    // Validate and enforce 'nextStatus' as needed
    if needTransition {
      match *self {
        Status::Reserved if nextStatus==Status::Running  => *self = nextStatus,
        Status::Reserved if nextStatus==Status::Invalid  => *self = nextStatus,
        Status::Running  if nextStatus==Status::Complete => *self = nextStatus,
        Status::Running  if nextStatus==Status::Failed   => *self = nextStatus,
        Status::Running  if nextStatus==Status::Invalid  => *self = nextStatus,
        Status::Failed   if nextStatus==Status::Invalid  => *self = nextStatus,
        _ => {
          println!("parentStatus {:?} -> {:?} invalid", *self, nextStatus);
          *self = Status::Invalid;
          return Err(MyErr::JobError);
        },
      }
    }
    return Ok(());
  }
}

struct JobTest {
  curr  : Status,
  next  : Status,
}

const jobTest: [JobTest; 7] = [
  JobTest{curr: Status::jobInitialState(), next: Status::Reserved,},
  JobTest{curr: Status::Reserved, next: Status::Invoked,},
  JobTest{curr: Status::Reserved, next: Status::Invalid,},
  JobTest{curr: Status::Invoked, next: Status::Running,},
  JobTest{curr: Status::Invoked, next: Status::Invalid,},
  JobTest{curr: Status::Running, next: Status::Complete,},
  JobTest{curr: Status::Running, next: Status::Failed,},
];

struct Transition {
  runLhs  : bool,
  next    : Status,
  prnt    : Status,
}

struct Trace<'a> {
  enabled     : bool,
  sequence    : &'a [Transition],
}

const traceTest: [Trace; 150] = [
  // case 0
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Complete,},
  ],},
  // case 1
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Failed,},
  ],},
  // case 2
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Invalid,},
  ],},
  // case 3
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Invalid,},
  ],},
  // case 4
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Failed,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Failed,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Failed,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Failed,},
  ],},
  // case 5
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Failed,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Failed,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Failed,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Failed,},
  ],},
  // case 6
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Failed,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Failed,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Invalid,},
  ],},
  // case 7
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Failed,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Invalid,},
  ],},
  // case 8
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Complete,},
  ],},
  // case 9
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Failed,},
  ],},
  // case 10
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Invalid,},
  ],},
  // case 11
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Failed,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Failed,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Failed,},
  ],},
  // case 12
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Failed,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Failed,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Failed,},
  ],},
  // case 13
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Failed,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Invalid,},
  ],},
  // case 14
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Complete,},
  ],},
  // case 15
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Failed,},
  ],},
  // case 16
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Failed,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Failed,},
  ],},
  // case 17
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Failed,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Failed,},
  ],},
  // case 18
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Complete,},
  ],},
  // case 19
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Failed,},
  ],},
  // case 20
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Failed,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Failed,},
  ],},
  // case 21
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Failed,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Failed,},
  ],},
  // case 22
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Invalid,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Invalid,},
  ],},
  // case 23
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Invalid,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Invalid,},
  ],},
  // case 24
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Invalid,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Invalid,},
  ],},
  // case 25
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Invalid,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Invalid,},
  ],},
  // case 26
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Invalid,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Invalid,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Invalid,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Invalid,},
  ],},
  // case 27
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Invalid,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Invalid,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Invalid,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Invalid,},
  ],},
  // case 28
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Invalid,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Invalid,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Invalid,},
  ],},
  // case 29
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Invalid,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Invalid,},
  ],},
  // case 30
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Complete,},
  ],},
  // case 31
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Failed,},
  ],},
  // case 32
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Invalid,},
  ],},
  // case 33
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Failed,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Failed,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Failed,},
  ],},
  // case 34
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Failed,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Failed,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Failed,},
  ],},
  // case 35
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Failed,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Invalid,},
  ],},
  // case 36
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Complete,},
  ],},
  // case 37
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Failed,},
  ],},
  // case 38
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Failed,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Failed,},
  ],},
  // case 39
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Failed,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Failed,},
  ],},
  // case 40
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Complete,},
  ],},
  // case 41
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Failed,},
  ],},
  // case 42
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Failed,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Failed,},
  ],},
  // case 43
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Failed,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Failed,},
  ],},
  // case 44
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Invalid,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Invalid,},
  ],},
  // case 45
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Invalid,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Invalid,},
  ],},
  // case 46
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Invalid,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Invalid,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Invalid,},
  ],},
  // case 47
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Invalid,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Invalid,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Invalid,},
  ],},
  // case 48
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Invalid,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Invalid,},
  ],},
  // case 49
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Complete,},
  ],},
  // case 50
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Failed,},
  ],},
  // case 51
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Failed,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Failed,},
  ],},
  // case 52
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Failed,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Failed,},
  ],},
  // case 53
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Complete,},
  ],},
  // case 54
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Failed,},
  ],},
  // case 55
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Failed,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Failed,},
  ],},
  // case 56
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Failed,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Failed,},
  ],},
  // case 57
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Invalid,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Invalid,},
  ],},
  // case 58
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Invalid,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Invalid,},
  ],},
  // case 59
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Complete,},
  ],},
  // case 60
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Failed,},
  ],},
  // case 61
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Invalid,},
  ],},
  // case 62
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Failed,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Failed,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Failed,},
  ],},
  // case 63
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Failed,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Failed,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Failed,},
  ],},
  // case 64
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Failed,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Invalid,},
  ],},
  // case 65
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Invalid,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Invalid,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Invalid,},
  ],},
  // case 66
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Invalid,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Invalid,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Invalid,},
  ],},
  // case 67
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Invalid,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Invalid,},
  ],},
  // case 68
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Invalid,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Invalid,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Invalid,},
  ],},
  // case 69
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Invalid,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Invalid,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Invalid,},
  ],},
  // case 70
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Invalid,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Invalid,},
  ],},
  // case 71
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Invalid,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Invalid,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Invalid,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Invalid,},
  ],},
  // case 72
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Invalid,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Invalid,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Invalid,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Invalid,},
  ],},
  // case 73
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Invalid,},
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Invalid,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Invalid,},
  ],},
  // case 74
  Trace{enabled: true, sequence: &[
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Invalid,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Invalid,},
  ],},
  // case 75
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Complete,},
  ],},
  // case 76
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Failed,},
  ],},
  // case 77
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Invalid,},
  ],},
  // case 78
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Failed,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Failed,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Failed,},
  ],},
  // case 79
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Failed,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Failed,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Failed,},
  ],},
  // case 80
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Failed,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Invalid,},
  ],},
  // case 81
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Complete,},
  ],},
  // case 82
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Failed,},
  ],},
  // case 83
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Failed,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Failed,},
  ],},
  // case 84
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Failed,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Failed,},
  ],},
  // case 85
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Complete,},
  ],},
  // case 86
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Failed,},
  ],},
  // case 87
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Failed,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Failed,},
  ],},
  // case 88
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Failed,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Failed,},
  ],},
  // case 89
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Invalid,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Invalid,},
  ],},
  // case 90
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Invalid,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Invalid,},
  ],},
  // case 91
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Invalid,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Invalid,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Invalid,},
  ],},
  // case 92
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Invalid,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Invalid,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Invalid,},
  ],},
  // case 93
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Invalid,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Invalid,},
  ],},
  // case 94
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Complete,},
  ],},
  // case 95
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Failed,},
  ],},
  // case 96
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Failed,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Failed,},
  ],},
  // case 97
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Failed,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Failed,},
  ],},
  // case 98
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Complete,},
  ],},
  // case 99
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Failed,},
  ],},
  // case 100
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Failed,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Failed,},
  ],},
  // case 101
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Failed,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Failed,},
  ],},
  // case 102
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Invalid,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Invalid,},
  ],},
  // case 103
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Invalid,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Invalid,},
  ],},
  // case 104
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Complete,},
  ],},
  // case 105
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Failed,},
  ],},
  // case 106
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Invalid,},
  ],},
  // case 107
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Failed,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Failed,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Failed,},
  ],},
  // case 108
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Failed,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Failed,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Failed,},
  ],},
  // case 109
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Failed,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Invalid,},
  ],},
  // case 110
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Invalid,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Invalid,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Invalid,},
  ],},
  // case 111
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Invalid,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Invalid,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Invalid,},
  ],},
  // case 112
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Invalid,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Invalid,},
  ],},
  // case 113
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Invalid,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Invalid,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Invalid,},
  ],},
  // case 114
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Invalid,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Invalid,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Invalid,},
  ],},
  // case 115
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Invalid,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Invalid,},
  ],},
  // case 116
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Complete,},
  ],},
  // case 117
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Failed,},
  ],},
  // case 118
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Failed,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Failed,},
  ],},
  // case 119
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Failed,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Failed,},
  ],},
  // case 120
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Complete,},
  ],},
  // case 121
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Failed,},
  ],},
  // case 122
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Failed,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Failed,},
  ],},
  // case 123
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Failed,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Failed,},
  ],},
  // case 124
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Invalid,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Invalid,},
  ],},
  // case 125
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Invalid,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Invalid,},
  ],},
  // case 126
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Complete,},
  ],},
  // case 127
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Failed,},
  ],},
  // case 128
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Invalid,},
  ],},
  // case 129
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Failed,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Failed,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Failed,},
  ],},
  // case 130
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Failed,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Failed,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Failed,},
  ],},
  // case 131
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Failed,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Invalid,},
  ],},
  // case 132
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Invalid,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Invalid,},
  ],},
  // case 133
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Invalid,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Invalid,},
  ],},
  // case 134
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Complete,},
  ],},
  // case 135
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Failed,},
  ],},
  // case 136
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Invalid,},
  ],},
  // case 137
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Complete, prnt: Status::Running,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Invalid,},
  ],},
  // case 138
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Failed,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Failed,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Failed,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Failed,},
  ],},
  // case 139
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Failed,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Failed,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Failed,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Failed,},
  ],},
  // case 140
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Failed,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Failed,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Invalid,},
  ],},
  // case 141
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Running, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Failed, prnt: Status::Failed,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Invalid,},
  ],},
  // case 142
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Invalid,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Invalid,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Invalid,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Invalid,},
  ],},
  // case 143
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Invalid,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Invalid,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Invalid,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Invalid,},
  ],},
  // case 144
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Invalid,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Invalid,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Invalid,},
  ],},
  // case 145
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invoked, prnt: Status::Running,},
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Invalid,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Invalid,},
  ],},
  // case 146
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Invalid,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Invalid,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Invalid,},
    Transition{runLhs: true, next: Status::Complete, prnt: Status::Invalid,},
  ],},
  // case 147
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Invalid,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Invalid,},
    Transition{runLhs: true, next: Status::Running, prnt: Status::Invalid,},
    Transition{runLhs: true, next: Status::Failed, prnt: Status::Invalid,},
  ],},
  // case 148
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Invalid,},
    Transition{runLhs: true, next: Status::Invoked, prnt: Status::Invalid,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Invalid,},
  ],},
  // case 149
  Trace{enabled: true, sequence: &[
    Transition{runLhs: false, next: Status::Invalid, prnt: Status::Invalid,},
    Transition{runLhs: true, next: Status::Invalid, prnt: Status::Invalid,},
  ],},
];

fn testJobStatus() {
  let max = (Status::Invalid as usize) + 1;
  for i in 0..max {
    for j in 0..max {
      let mut curr = Status::from_u32(i as u32).unwrap();
      let next = Status::from_u32(j as u32).unwrap();
      let mut found = false;
      println!("testJobStatus ({},{})", i, j); 
      for k in 0..jobTest.len() {
        if curr==jobTest[k].curr && next==jobTest[k].next {
          found = true;
          break;
        }
      }
      if found {
        assert_eq!(Ok(()), curr.updateAsJob(next));
        assert_eq!(next, curr);
      } else {
        assert_eq!(Err(MyErr::JobError), curr.updateAsJob(next));
        assert_eq!(Status::Invalid, curr);
      }
    }
  }
}

fn testSharedParentTaskStatus() {
  for i in 0..traceTest.len() {
    if !traceTest[i].enabled {
      continue;
    }
    let mut prnt = Status::parentInitialState();
    let mut taskStatus: Vec<Status> = vec![Status::taskInitialState(), Status::taskInitialState()];
    for j in 0..traceTest[i].sequence.len() {
      println!("testSharedParentTaskStatus: run case {} step {} prnt {:?}",i, j, traceTest[i].sequence[j].prnt);
      if traceTest[i].sequence[j].runLhs {
        assert_eq!(Ok(()), taskStatus[0].updateAsTask(traceTest[i].sequence[j].next));
        assert_eq!(Ok(()), prnt.updateAsParent(&taskStatus));
        assert_eq!(traceTest[i].sequence[j].next, taskStatus[0]);
        assert_eq!(traceTest[i].sequence[j].prnt, prnt);
      } else {
        assert_eq!(Ok(()), taskStatus[1].updateAsTask(traceTest[i].sequence[j].next));
        assert_eq!(Ok(()), prnt.updateAsParent(&taskStatus));
        assert_eq!(traceTest[i].sequence[j].next, taskStatus[1]);
        assert_eq!(traceTest[i].sequence[j].prnt, prnt);
      }
    }
  }
}

fn main() {
  testJobStatus();
  testSharedParentTaskStatus();
}
