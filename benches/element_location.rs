use bryndza::{Locator, Session};
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use std::time::Duration;

/// Benchmark element location performance
/// This helps identify performance bottlenecks in element finding algorithms
fn bench_element_location(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    // Setup session once for all benchmarks
    let session = rt.block_on(async {
        let mut session = Session::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap();
        session.start().await.unwrap();
        session
    });

    let mut group = c.benchmark_group("element_location");

    // Benchmark ID locator
    group.bench_function("find_by_id", |b| {
        b.iter(|| {
            rt.block_on(async {
                let locator = black_box(Locator::id("test-button"));
                let _ = session.find_element(&locator).await;
            })
        })
    });

    // Benchmark class name locator
    group.bench_function("find_by_class", |b| {
        b.iter(|| {
            rt.block_on(async {
                let locator = black_box(Locator::class_name("button"));
                let _ = session.find_element(&locator).await;
            })
        })
    });

    // Benchmark text locator
    group.bench_function("find_by_text", |b| {
        b.iter(|| {
            rt.block_on(async {
                let locator = black_box(Locator::text("Click me"));
                let _ = session.find_element(&locator).await;
            })
        })
    });

    // Benchmark XPath locator
    group.bench_function("find_by_xpath", |b| {
        b.iter(|| {
            rt.block_on(async {
                let locator = black_box(Locator::xpath("//button[@id='test']"));
                let _ = session.find_element(&locator).await;
            })
        })
    });

    // Benchmark CSS selector locator
    group.bench_function("find_by_css", |b| {
        b.iter(|| {
            rt.block_on(async {
                let locator = black_box(Locator::css_selector("#test-button"));
                let _ = session.find_element(&locator).await;
            })
        })
    });

    // Benchmark complex locator (AND combination)
    group.bench_function("find_by_complex_and", |b| {
        b.iter(|| {
            rt.block_on(async {
                let locator = black_box(Locator::And(vec![
                    Locator::class_name("button"),
                    Locator::text("Submit"),
                ]));
                let _ = session.find_element(&locator).await;
            })
        })
    });

    group.finish();
}

/// Benchmark screenshot operations
fn bench_screenshot(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    let session = rt.block_on(async {
        let mut session = Session::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap();
        session.start().await.unwrap();
        session
    });

    let mut group = c.benchmark_group("screenshot");

    // Benchmark full screen screenshot
    group.bench_function("full_screen", |b| {
        b.iter(|| {
            rt.block_on(async {
                let _ = session.screenshot().await;
            })
        })
    });

    group.finish();
}

/// Benchmark wait strategies
fn bench_wait_strategies(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    let session = rt.block_on(async {
        let mut session = Session::builder()
            .timeout(Duration::from_secs(1))
            .build()
            .unwrap();
        session.start().await.unwrap();
        session
    });

    let mut group = c.benchmark_group("wait_strategies");

    // Benchmark wait for element that doesn't exist (will timeout quickly)
    group.bench_function("wait_for_element", |b| {
        b.iter(|| {
            rt.block_on(async {
                let locator = black_box(Locator::id("non-existent-element"));
                let _ = session.wait_for_element(&locator).await;
            })
        })
    });

    group.finish();
}

/// Benchmark image comparison operations
fn bench_image_comparison(c: &mut Criterion) {
    use bryndza::utils::image::{ComparisonAlgorithm, ImageComparison};

    // Create dummy image data for benchmarking
    let image1 = vec![0u8; 1024 * 768 * 3]; // RGB image data
    let image2 = vec![0u8; 1024 * 768 * 3]; // RGB image data

    let mut group = c.benchmark_group("image_comparison");

    // Benchmark pixel-by-pixel comparison
    group.bench_function("pixel_by_pixel", |b| {
        b.iter(|| {
            let _ = ImageComparison::compare_images(
                black_box(&image1),
                black_box(&image2),
                black_box(ComparisonAlgorithm::PixelByPixel),
            );
        })
    });

    // Benchmark SSIM comparison
    group.bench_function("ssim", |b| {
        b.iter(|| {
            let _ = ImageComparison::compare_images(
                black_box(&image1),
                black_box(&image2),
                black_box(ComparisonAlgorithm::StructuralSimilarity),
            );
        })
    });

    // Benchmark perceptual hash comparison
    group.bench_function("perceptual_hash", |b| {
        b.iter(|| {
            let _ = ImageComparison::compare_images(
                black_box(&image1),
                black_box(&image2),
                black_box(ComparisonAlgorithm::PerceptualHash),
            );
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_element_location,
    bench_screenshot,
    bench_wait_strategies,
    bench_image_comparison
);
criterion_main!(benches);
