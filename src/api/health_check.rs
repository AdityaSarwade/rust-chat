#[get("/health-check")]
pub fn health_check() -> &'static str {
    "Server is Online."
}
