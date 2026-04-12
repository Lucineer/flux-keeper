#![allow(dead_code)]
#[derive(Clone,Debug,PartialEq)]
pub enum CheckStatus { Ok, Warning, Critical, Unknown }
pub struct HealthCheck { id: u32, name: String, status: CheckStatus, last_ok: u64, interval: u64, timeout: u64, consecutive_failures: u32, max_failures: u32 }
pub struct Keeper { checks: Vec<HealthCheck>, alerts: Vec<String>, running: bool }

impl Keeper {
    pub fn new() -> Self { Self { checks: Vec::new(), alerts: Vec::new(), running: false } }
    pub fn running(&self) -> bool { self.running }
    pub fn add_check(&mut self, id: u32, name: &str, interval: u64, timeout: u64, max_failures: u32) {
        self.checks.push(HealthCheck { id, name: name.to_string(), status: CheckStatus::Unknown, last_ok: 0, interval, timeout, consecutive_failures: 0, max_failures });
    }
    pub fn report_ok(&mut self, id: u32, now: u64) {
        if let Some(c) = self.checks.iter_mut().find(|c| c.id == id) { c.status = CheckStatus::Ok; c.last_ok = now; c.consecutive_failures = 0; }
    }
    pub fn report_failure(&mut self, id: u32, now: u64) {
        if let Some(c) = self.checks.iter_mut().find(|c| c.id == id) {
            c.consecutive_failures += 1;
            c.status = if c.consecutive_failures >= c.max_failures { CheckStatus::Critical } else { CheckStatus::Warning };
            let msg = format!("[{}] {} failure #{} (max {})", now, c.name, c.consecutive_failures, c.max_failures);
            self.alerts.push(msg);
        }
    }
    pub fn tick(&mut self, now: u64) -> Vec<String> {
        self.running = true;
        let mut new_alerts = Vec::new();
        for c in &self.checks {
            if c.last_ok > 0 && now > c.last_ok + c.timeout && c.status != CheckStatus::Critical {
                let msg = format!("[{}] {} timeout after {}s", now, c.name, now - c.last_ok);
                new_alerts.push(msg.clone());
                self.alerts.push(msg);
            }
        }
        new_alerts
    }
    pub fn get_status(&self, id: u32) -> Option<&HealthCheck> { self.checks.iter().find(|c| c.id == id) }
    pub fn is_healthy(&self) -> bool { !self.checks.iter().any(|c| c.status == CheckStatus::Critical) }
    pub fn critical_count(&self) -> usize { self.checks.iter().filter(|c| c.status == CheckStatus::Critical).count() }
    pub fn check_ids(&self) -> Vec<u32> { self.checks.iter().map(|c| c.id).collect() }
    pub fn remove_check(&mut self, id: u32) { self.checks.retain(|c| c.id != id); }
    pub fn reset(&mut self, id: u32) {
        if let Some(c) = self.checks.iter_mut().find(|c| c.id == id) { c.status = CheckStatus::Unknown; c.consecutive_failures = 0; }
    }
    pub fn alert_count(&self) -> usize { self.alerts.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_new() { let k = Keeper::new(); assert!(!k.running()); assert_eq!(k.check_ids().len(), 0); }
    #[test] fn test_add_check() { let mut k = Keeper::new(); k.add_check(1, "ping", 60, 300, 3); assert_eq!(k.check_ids().len(), 1); }
    #[test] fn test_report_ok() { let mut k = Keeper::new(); k.add_check(1, "ping", 60, 300, 3); k.report_ok(1, 100); assert_eq!(k.get_status(1).unwrap().status, CheckStatus::Ok); }
    #[test] fn test_report_failure_warning() { let mut k = Keeper::new(); k.add_check(1, "ping", 60, 300, 3); k.report_failure(1, 100); assert_eq!(k.get_status(1).unwrap().status, CheckStatus::Warning); }
    #[test] fn test_report_failure_critical() { let mut k = Keeper::new(); k.add_check(1, "ping", 60, 300, 3); for _ in 0..3 { k.report_failure(1, 100); } assert_eq!(k.get_status(1).unwrap().status, CheckStatus::Critical); assert!(!k.is_healthy()); }
    #[test] fn test_healthy() { let k = Keeper::new(); assert!(k.is_healthy()); }
    #[test] fn test_critical_count() { let mut k = Keeper::new(); k.add_check(1, "a", 60, 300, 1); k.add_check(2, "b", 60, 300, 1); k.report_failure(1, 100); assert_eq!(k.critical_count(), 1); }
    #[test] fn test_remove_check() { let mut k = Keeper::new(); k.add_check(1, "ping", 60, 300, 3); k.remove_check(1); assert_eq!(k.check_ids().len(), 0); }
    #[test] fn test_reset() { let mut k = Keeper::new(); k.add_check(1, "ping", 60, 300, 3); k.report_failure(1, 100); k.reset(1); assert_eq!(k.get_status(1).unwrap().consecutive_failures, 0); }
    #[test] fn test_timeout() { let mut k = Keeper::new(); k.add_check(1, "ping", 60, 300, 3); k.report_ok(1, 100); let a = k.tick(500); assert_eq!(a.len(), 1); }
    #[test] fn test_alerts() { let mut k = Keeper::new(); k.add_check(1, "ping", 60, 300, 3); k.report_failure(1, 100); assert_eq!(k.alert_count(), 1); }
    #[test] fn test_ok_resets_failures() { let mut k = Keeper::new(); k.add_check(1, "p", 60, 300, 3); k.report_failure(1, 100); k.report_ok(1, 200); assert_eq!(k.get_status(1).unwrap().consecutive_failures, 0); }
}