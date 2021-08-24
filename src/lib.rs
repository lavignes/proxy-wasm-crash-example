use proxy_wasm::{types::*, traits::*};

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> { Box::new(CrashRoot) });
}

struct Crash;

impl Context for Crash {}

impl StreamContext for Crash {
    fn on_upstream_data(&mut self, _data_size: usize, _end_of_stream: bool) -> Action {
        let _ = self.get_property(vec!["filter_state"]);
        Action::Continue
    }
}

struct CrashRoot;

impl Context for CrashRoot {}

impl RootContext for CrashRoot {
    fn create_stream_context(&self, _context_id: u32) -> Option<Box<dyn StreamContext>> {
        Some(Box::new(Crash))
    }

    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::StreamContext)
    }
}
