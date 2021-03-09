#![doc = "This file was automatically generated by the varlink rust generator"]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use serde_derive::{Deserialize, Serialize};
use serde_json;
use std::io::BufRead;
use std::sync::{Arc, RwLock};
use varlink::{self, CallTrait};
#[allow(dead_code)]
#[derive(Clone, PartialEq, Debug)]
pub enum ErrorKind {
    Varlink_Error,
    VarlinkReply_Error,
}
impl ::std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            ErrorKind::Varlink_Error => write!(f, "Varlink Error"),
            ErrorKind::VarlinkReply_Error => write!(f, "Varlink error reply"),
        }
    }
}
pub struct Error(
    pub ErrorKind,
    pub Option<Box<dyn std::error::Error + 'static + Send + Sync>>,
    pub Option<&'static str>,
);
impl Error {
    #[allow(dead_code)]
    pub fn kind(&self) -> &ErrorKind {
        &self.0
    }
}
impl From<ErrorKind> for Error {
    fn from(e: ErrorKind) -> Self {
        Error(e, None, None)
    }
}
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.1
            .as_ref()
            .map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}
impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use std::error::Error as StdError;
        if let Some(ref o) = self.2 {
            std::fmt::Display::fmt(o, f)?;
        }
        std::fmt::Debug::fmt(&self.0, f)?;
        if let Some(e) = self.source() {
            std::fmt::Display::fmt("\nCaused by:\n", f)?;
            std::fmt::Debug::fmt(&e, f)?;
        }
        Ok(())
    }
}
#[allow(dead_code)]
pub type Result<T> = std::result::Result<T, Error>;
impl From<varlink::Error> for Error {
    fn from(e: varlink::Error) -> Self {
        match e.kind() {
            varlink::ErrorKind::VarlinkErrorReply(r) => Error(
                ErrorKind::from(r),
                Some(Box::from(e)),
                Some(concat!(file!(), ":", line!(), ": ")),
            ),
            _ => Error(
                ErrorKind::Varlink_Error,
                Some(Box::from(e)),
                Some(concat!(file!(), ":", line!(), ": ")),
            ),
        }
    }
}
#[allow(dead_code)]
impl Error {
    pub fn source_varlink_kind(&self) -> Option<&varlink::ErrorKind> {
        use std::error::Error as StdError;
        let mut s: &dyn StdError = self;
        while let Some(c) = s.source() {
            let k = self
                .source()
                .and_then(|e| e.downcast_ref::<varlink::Error>())
                .and_then(|e| Some(e.kind()));
            if k.is_some() {
                return k;
            }
            s = c;
        }
        None
    }
}
impl From<&varlink::Reply> for ErrorKind {
    #[allow(unused_variables)]
    fn from(e: &varlink::Reply) -> Self {
        match e {
            _ => ErrorKind::VarlinkReply_Error,
        }
    }
}
pub trait VarlinkCallError: varlink::CallTrait {}
impl<'a> VarlinkCallError for varlink::Call<'a> {}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum r#Event_kind {
    r#section_end,
    r#started,
    r#completed,
    r#failure,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct r#Event {
    pub r#kind: Event_kind,
    pub r#section: Option<SectionMarker>,
    pub r#reason: Option<Reason>,
    pub r#result: Option<Outcome>,
    pub r#failure: Option<Failure>,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct r#ExitFail {
    pub r#command: String,
    pub r#status: Option<i64>,
    pub r#logs: Vec<String>,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum r#Failure_kind {
    r#io,
    r#spawn,
    r#exit,
    r#output,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct r#Failure {
    pub r#kind: Failure_kind,
    pub r#nix_file: String,
    pub r#io: Option<IOFail>,
    pub r#spawn: Option<SpawnFail>,
    pub r#exit: Option<ExitFail>,
    pub r#output: Option<OutputFail>,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct r#IOFail {
    pub r#message: String,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct r#Outcome {
    pub r#nix_file: String,
    pub r#project_root: String,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct r#OutputFail {
    pub r#message: String,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum r#Reason_kind {
    r#project_added,
    r#ping_received,
    r#files_changed,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct r#Reason {
    pub r#kind: Reason_kind,
    pub r#project: Option<String>,
    pub r#files: Option<Vec<String>>,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct r#SectionMarker {}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct r#SpawnFail {
    pub r#message: String,
    pub r#command: String,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Monitor_Reply {
    pub r#event: Event,
}
impl varlink::VarlinkReply for Monitor_Reply {}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Monitor_Args {}
pub trait Call_Monitor: VarlinkCallError {
    fn reply(&mut self, r#event: Event) -> varlink::Result<()> {
        self.reply_struct(Monitor_Reply { r#event }.into())
    }
}
impl<'a> Call_Monitor for varlink::Call<'a> {}
pub trait VarlinkInterface {
    fn monitor(&self, call: &mut dyn Call_Monitor) -> varlink::Result<()>;
    fn call_upgraded(
        &self,
        _call: &mut varlink::Call,
        _bufreader: &mut dyn BufRead,
    ) -> varlink::Result<Vec<u8>> {
        Ok(Vec::new())
    }
}
pub trait VarlinkClientInterface {
    fn monitor(&mut self) -> varlink::MethodCall<Monitor_Args, Monitor_Reply, Error>;
}
#[allow(dead_code)]
pub struct VarlinkClient {
    connection: Arc<RwLock<varlink::Connection>>,
}
impl VarlinkClient {
    #[allow(dead_code)]
    pub fn new(connection: Arc<RwLock<varlink::Connection>>) -> Self {
        VarlinkClient { connection }
    }
}
impl VarlinkClientInterface for VarlinkClient {
    fn monitor(&mut self) -> varlink::MethodCall<Monitor_Args, Monitor_Reply, Error> {
        varlink::MethodCall::<Monitor_Args, Monitor_Reply, Error>::new(
            self.connection.clone(),
            "org.nixos.lorri.Monitor",
            Monitor_Args {},
        )
    }
}
#[allow(dead_code)]
pub struct VarlinkInterfaceProxy {
    inner: Box<dyn VarlinkInterface + Send + Sync>,
}
#[allow(dead_code)]
pub fn new(inner: Box<dyn VarlinkInterface + Send + Sync>) -> VarlinkInterfaceProxy {
    VarlinkInterfaceProxy { inner }
}
impl varlink::Interface for VarlinkInterfaceProxy {
    fn get_description(&self) -> &'static str {
        "# The interface `lorri daemon` exposes.\ninterface org.nixos.lorri\n\n# Monitor the daemon. The method will reply with an Event update whenever a\n# build begins or ends.  Monitor will immediately reply with a snapshot of\n# known projects, then a marker event, indicating that the stream of events is\n# now \"live.\"\nmethod Monitor() -> (event: Event)\n\n# An event describing the behavior of Lorri across all known projects. There\n# are several kinds of Event, and each kind has a different type to represent\n# futher information\ntype Event (\n    # The kind of the event:\n    # - section_end: marks the break between the current state snapshot, and\n    #   live events.\n    # - started: a build has started but not completed\n    # - completed: a build completed successfully\n    # - failure: a build failed\n    kind: (section_end, started, completed, failure),\n    section: ?SectionMarker, # present iff kind == section_end\n    reason: ?Reason,         # present iff kind == started\n    result: ?Outcome,        # present iff kind == completed\n    failure: ?Failure        # present iff kind == failure\n)\n\n# An empty value - there is nothing further to distinguish the section end\n# event. This type (and its field on Event) exist as a ward against future\n# changes to the event, and to aid recipients in the meantime.\ntype SectionMarker ()\n\n# The impetus for a new build. Like Event, Reason has a kind, and each kind has\n# a unique field.\ntype Reason (\n    # The kind of build reason:\n    # - project_added: Lorri has been newly informed of a project\n    # - ping_received: A client requested a new build\n    # - files_changed: Lorri received a filesystem notification of changed files\n    kind: (project_added, ping_received, files_changed),\n    # The absolute path to the shell.nix file for the added project\n    project: ?string, # present iff kind == project_added\n    # A list of files that changed, triggering a new build\n    # This can be useful e.g. to debug Nix expressions bringing in too many\n    # files and thereby building too frequently\n    files: ?[]string  # present iff kind == files_changed\n)\n\n# Details about the built project.\ntype Outcome (\n    # The absolute path to the shell.nix file for the added project\n    nix_file: string,\n    # The root directory of the project\n    project_root: string\n)\n\ntype Failure (\n    # The kind of failure:\n    # - io: An I/O failure\n    # - spawn: The build process couldn't be spawned\n    # - exit: The build started but exited with a failure\n    # - output: the build completed, but Lorri wasn't able to interpret the\n    #   output\n    kind: (io, spawn, exit, output),\n    # The absolute path to the shell.nix file for the added project\n    nix_file: string,\n    io: ?IOFail,        # present iff kind == io\n    spawn: ?SpawnFail,  # present iff kind == spawn\n    exit: ?ExitFail,    # present iff kind == exit\n    output: ?OutputFail # present iff kind == output\n)\n\n# Describes a build failure related to opening files, usually the shell.nix file\ntype IOFail (\n    # A message describing the failure\n    message: string\n)\n\n# Describes a failure to launch the build process\ntype SpawnFail (\n    # A message describing the failure\n    message: string,\n    # The command Lorri attempted to execute\n    command: string\n)\n\n# Describes a failed build process\ntype ExitFail (\n    # The command executed by Lorri\n    command: string,\n    # The Unix exit status of the command, if available\n    status: ?int,\n    # stderr of the failed command.\n    logs: []string\n)\n\n# Describes a failure caused by output produced by the build that Lorri cannot\n# parse\ntype OutputFail (\n    # A message describing the failure\n    message: string\n)\n"
    }
    fn get_name(&self) -> &'static str {
        "org.nixos.lorri"
    }
    fn call_upgraded(
        &self,
        call: &mut varlink::Call,
        bufreader: &mut dyn BufRead,
    ) -> varlink::Result<Vec<u8>> {
        self.inner.call_upgraded(call, bufreader)
    }
    fn call(&self, call: &mut varlink::Call) -> varlink::Result<()> {
        let req = call.request.unwrap();
        match req.method.as_ref() {
            "org.nixos.lorri.Monitor" => self.inner.monitor(call as &mut dyn Call_Monitor),
            m => call.reply_method_not_found(String::from(m)),
        }
    }
}