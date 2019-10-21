use std::future::Future;
use std::task::{Poll,Context,RawWaker,RawWakerVTable,Waker};
use std::pin::Pin;
use std::ptr;

struct PrintPolledFuture;

impl Future for PrintPolledFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("Polled!");
        Poll::Ready(())            
    }
}

struct Executor{
    waker : Waker,
}

impl Executor {
    fn new() -> Executor {

        fn raw_waker_clone(_a:*const())->std::task::RawWaker{
            panic!("unimplemented");
        }

        fn raw_waker_all(_a:*const()){
            panic!("unimplemented");
        }

        fn raw_waker_drop(_a: *const ()){}

        static RAW_WAKER_VTABLE : RawWakerVTable = RawWakerVTable::new(
                                                        raw_waker_clone,
                                                        raw_waker_all ,
                                                        raw_waker_all,
                                                        raw_waker_drop);
        Executor{
            waker : unsafe {
                Waker::from_raw(
                    RawWaker::new(ptr::null(),&RAW_WAKER_VTABLE))
            }
        }
    }

    fn block_on<T>(&mut self, mut f : T ) -> T::Output
        where T : Future {
        loop{
            let mut context = Context::from_waker(&self.waker);
            match unsafe { Pin::new_unchecked(&mut f).poll(&mut context)} {
                Poll::Ready(val) => return val,
                Poll::Pending => continue,
            }
        }
    }
}

async fn run_print_polled_future(){
    let v = PrintPolledFuture{};
    v.await;

    let v = async {
        println!("RawAsync")
    };
    v.await;
}


fn main() {
    Executor::new().block_on(run_print_polled_future());
}
