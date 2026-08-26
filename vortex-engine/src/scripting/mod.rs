// -- leaked by @azixi0 on github
use std::collections::BTreeMap;
use std::time::Duration;

pub const PRELUDE: &str = include_str!("../../assets/prelude.lua");
pub const TASK_RUNTIME: &str = include_str!("../../assets/task_runtime.lua");
pub const RECOVERED_SERVICES: &[&str] = &["Workspace", "RunService"];
pub const RECOVERED_INSTANCE_METHODS: &[&str] = &[
    "GetPlayers", "IsDead", "GetChildren", "GetDescendants", "FindFirstChild",
    "FindFirstChildOfClass", "IsA", "Destroy",
];
pub const MAX_SIGNAL_HANDLERS: usize = 512;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScriptKind { LocalScript, Script, ModuleScript }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScriptState { Ready, Running, Waiting, Stopped, Failed(String) }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScriptRuntime {
    pub name: String,
    pub kind: ScriptKind,
    pub state: ScriptState,
    pub elapsed_this_frame: Duration,
}

#[derive(Debug)]
pub struct ScriptRuntimes {
    runtimes: BTreeMap<String, ScriptRuntime>,
    max_scripts: usize,
    frame_budget: Duration,
}

impl ScriptRuntimes {
    pub fn new(max_scripts: usize, frame_budget: Duration) -> Self {
        Self { runtimes: BTreeMap::new(), max_scripts, frame_budget }
    }

    pub fn insert(&mut self, runtime: ScriptRuntime) -> Result<(), &'static str> {
        if self.runtimes.len() >= self.max_scripts { return Err("script limit reached"); }
        self.runtimes.insert(runtime.name.clone(), runtime);
        Ok(())
    }

    pub fn charge(&mut self, name: &str, elapsed: Duration) -> Result<(), &'static str> {
        let runtime = self.runtimes.get_mut(name).ok_or("script runtime not found")?;
        runtime.elapsed_this_frame += elapsed;
        if runtime.elapsed_this_frame > self.frame_budget {
            runtime.state = ScriptState::Stopped;
            return Err("script exceeded the frame budget and was stopped");
        }
        Ok(())
    }

    pub fn begin_frame(&mut self) {
        for runtime in self.runtimes.values_mut() { runtime.elapsed_this_frame = Duration::ZERO; }
    }
}
