mod ThreadPool_rs;
pub use ThreadPool_rs::ThreadPool;
pub use ThreadPool_rs::Future_rs::Future;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        let mut thre = ThreadPool::new(2, 3, 5);

        let mut vc = Vec::<Future>::new();
        for i in 0..10 {
            let efef = thre.executor(|| {
                println!("aaaaaaaa");


            });
            vc.push(efef);
        }

        println!("LLLLLLLLLLLLLLLLLLL");

        vc.into_iter().map(|x| {
            x.get();
        }).count();




    }
}