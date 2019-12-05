use criterion::{criterion_group, criterion_main, Criterion};
use database::search::{DictSearch, load_search, PronunciationType};

pub fn benchmark_search(c: &mut Criterion) {
    let pool = database::create_db_pool();

    let connection = &pool.get_connection();

    println!("fetched from db");
    let mut search = DictSearch::new();
    load_search(&mut search, connection);

    println!("indexing done");

    c.bench_function("search pronunciation", |b| {
        b.iter(|| search.search_pronunciation("? sei2", PronunciationType::Jyutping));
    });
    c.bench_function("search char", |b| {
        b.iter(|| search.search_characters("?好"));
    });
}

criterion_group!(benches, benchmark_search);
criterion_main!(benches);