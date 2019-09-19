//生命周期的这种用法还很少见
fn test<'a>(s: &'a &'static str) {
    let _local: &'a &'a str = s;
}
