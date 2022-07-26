pub(crate) struct PProf<'a> {
    #[cfg(target_os = "linux")]
    guard: Option<pprof::ProfilerGuard<'a>>,
    #[cfg(not(target_os = "linux"))]
    marker: std::marker::PhantomData<&'a bool>,
}

impl<'a> PProf<'a> {
    #[cfg(target_os = "linux")]
    pub fn new(enable: bool) -> Self {
        let guard = if enable {
            pprof::ProfilerGuardBuilder::default()
                .frequency(100)
                .blocklist(&["libc", "libgcc", "pthread"])
                .build()
                .ok()
        } else {
            None
        };
        Self { guard }
    }

    #[cfg(target_os = "linux")]
    pub fn report(&self, report_file: &str) -> pprof::Result<()> {
        if let Some(guard) = self.guard.as_ref() {
            println!("generating pprof {}", report_file);
            let report = guard
                .report()
                .build()?;
            let file = std::fs::File::create(report_file)?;
            report.flamegraph(file)?;
        }
        Ok(())
    }

    #[cfg(not(target_os = "linux"))]
    pub fn new(_enable: bool) -> Self {
        println!("pprof not supported on crrent platform");
        Self {
            marker: std::marker::PhantomData,
        }
    }

    #[cfg(not(target_os = "linux"))]
    pub fn report(&self, _report_file: &str) -> Result<()> {
        Ok(())
    }
}
