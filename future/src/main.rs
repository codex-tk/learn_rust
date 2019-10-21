use std::future::Future;
use std::task::{Poll,Context,RawWaker,RawWakerVTable,Waker};
use std::pin::Pin;
use std::ptr;

struct PrintPolledFuture;

impl Future for PrintPolledFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output>
    {
        println!("Polled!");
        Poll::Ready(())            
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

fn raw_waker_clone(_a:*const())->std::task::RawWaker{
    panic!("unimplemented");
}

fn raw_waker_all(_a:*const()){
    panic!("unimplemented");
}

fn raw_waker_drop(_a: *const ()){}

fn main() {
    static RAW_WAKER_VTABLE : RawWakerVTable = RawWakerVTable::new(
                                                        raw_waker_clone,
                                                        raw_waker_all ,
                                                        raw_waker_all,
                                                        raw_waker_drop);
    let waker = unsafe {
        Waker::from_raw(
            RawWaker::new(ptr::null(),&RAW_WAKER_VTABLE))
    };
    let mut context = Context::from_waker(&waker);
    let mut fut = run_print_polled_future();
    let _ = unsafe {
         Pin::new_unchecked(&mut fut).poll(&mut context)
    };
}
