use sxd_document_no_unsafe::__internal::StringPool;

fn string_cannot_outlive_the_pool() {
    let _s = {
        let pool = StringPool::new();
        pool.intern("hello")
    };
}

fn main() {}
