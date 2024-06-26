use accessibility_rs::{audit, AuditConfig};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
mod mock;

/// bench audit speed
#[cfg(all(not(feature = "tokio"), not(feature = "spider")))]
pub fn bench_speed(c: &mut Criterion) {
    let mut group = c.benchmark_group("audit-speed/core");

    group.sample_size(50);

    group.bench_function(format!("audit: {}", "small html"), |b| {
        b.iter(|| black_box(audit(&AuditConfig::basic(mock::MOCK_WEBSITE_SMALL_HTML))))
    });

    group.bench_function(format!("audit: {}", "medium html"), |b| {
        b.iter(|| {
            black_box(audit(&AuditConfig::basic(
                mock::MOCK_WEBSITE_A11YWATCH_HTML,
            )))
        })
    });

    group.bench_function(format!("audit: {}", "medium-large html"), |b| {
        b.iter(|| black_box(audit(&AuditConfig::basic(mock::MOCK_WEBSITE_HTML))))
    });

    group.bench_function(format!("audit: {}", "large-xlarge html"), |b| {
        b.iter(|| black_box(audit(&AuditConfig::basic(mock::MOCK_WEBSITE_LARGE_HTML))))
    });

    group.finish();
}

/// bench audit speed
#[cfg(all(feature = "tokio", not(feature = "spider")))]
pub fn bench_speed(c: &mut Criterion) {
    let mut group = c.benchmark_group("audit-speed/core");
    let rt = tokio::runtime::Runtime::new().unwrap();

    group.bench_function(format!("audit: {}", "small html"), |b| {
        b.to_async(&rt)
            .iter(|| black_box(audit(&AuditConfig::basic(mock::MOCK_WEBSITE_SMALL_HTML))))
    });

    group.bench_function(format!("audit: {}", "medium html"), |b| {
        b.to_async(&rt).iter(|| {
            black_box(audit(&AuditConfig::basic(
                mock::MOCK_WEBSITE_A11YWATCH_HTML,
            )))
        })
    });

    group.bench_function(format!("audit: {}", "medium-large html"), |b| {
        b.to_async(&rt)
            .iter(|| black_box(audit(&AuditConfig::basic(mock::MOCK_WEBSITE_HTML))))
    });

    group.bench_function(format!("audit: {}", "large-xlarge html"), |b| {
        b.to_async(&rt)
            .iter(|| black_box(audit(&AuditConfig::basic(mock::MOCK_WEBSITE_LARGE_HTML))))
    });

    group.finish();
}

/// bench audit speed
#[cfg(feature = "spider")]
pub fn bench_speed(c: &mut Criterion) {
    let mut group = c.benchmark_group("audit-speed/core");
    let rt = accessibility_rs::spider::tokio::runtime::Runtime::new().unwrap();

    group.bench_function(format!("audit: {}", "small html"), |b| {
        b.to_async(&rt)
            .iter(|| black_box(audit(&AuditConfig::basic(mock::MOCK_WEBSITE_SMALL_HTML))))
    });

    group.bench_function(format!("audit: {}", "medium html"), |b| {
        b.to_async(&rt).iter(|| {
            black_box(audit(&AuditConfig::basic(
                mock::MOCK_WEBSITE_A11YWATCH_HTML,
            )))
        })
    });

    group.bench_function(format!("audit: {}", "medium-large html"), |b| {
        b.to_async(&rt)
            .iter(|| black_box(audit(&AuditConfig::basic(mock::MOCK_WEBSITE_HTML))))
    });

    group.bench_function(format!("audit: {}", "large-xlarge html"), |b| {
        b.to_async(&rt)
            .iter(|| black_box(audit(&AuditConfig::basic(mock::MOCK_WEBSITE_LARGE_HTML))))
    });

    group.bench_function(format!("audit: {}", "spider audit html"), |b| {
        b.to_async(&rt).iter(|| {
            black_box(audit(&AuditConfig::new_website(
                "https://choosealicense.com",
                "",
                false,
                "",
            )))
        })
    });

    group.finish();
}

criterion_group!(benches, bench_speed);
criterion_main!(benches);
