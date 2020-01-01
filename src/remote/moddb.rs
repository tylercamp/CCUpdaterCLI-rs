use model::ModDB;

// Abstraction to allow for different sources, should return some
// promise for a CCModDB
trait ModDBSource {
    fn fetch(); // return type?
}

struct GithubCCModDBSource {}
impl ModDBSource for GithubCCModDBSource {
    fn fetch() {
        // TODO
    }
}

impl ModDB {
    
}