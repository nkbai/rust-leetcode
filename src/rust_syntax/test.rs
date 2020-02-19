#[derive(Debug)]
pub struct AveragedCollection {
    // struct 相当于 类
    list: Vec<i32>, //2个成员变量
    average: f64,   //平均值等私有64对象 这变量不能直接访问 内部成员变量
} //这两个是本体
  // 数据抽象抽象出结构

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value); // push东西进去
        self.update_average(); //调用 update_average时候让 ------>
    } //然后有内部方法

    pub fn remove(&mut self) -> Option<i32> {
        //remove是当前AveragedCollection返回 option值
        let result = self.list.pop(); //看里面有没东西      <--------- pop 从后面来
        match result {
            Some(value) => {
                //有东西就返回
                self.update_average();
                Some(value)
            }
            None => None, //如没东西返回 None
        } //外部方法对外接口 不能直接给外部的就不能给外部操作 否则符合接口
    }
    //4个成员函数  前三个对外操作
    #[allow(dead_code)]
    pub fn average(&self) -> f64 {
        //成员变量得用成员函数访问
        self.average //成员函数可访问内部变量 外部数据不可直接操作变量
    } //这就是封装

    fn update_average(&mut self) {
        //这是对内操作  每次加或者减可调用内部 update_average 方法
        let total: i32 = self.list.iter().sum(); // ---->  让total = self 里面iter 求sum  把总和求到
        self.average = total as f64 / self.list.len() as f64; //总和=元素数量 得平均值  重新计算average里得 平均值
    } //可以进行操作
}

//如想使用hashset代替vec 需要add 和 remove重写一遍可继续使用内库对外接口不变 仍然remove 和 average

#[cfg(test)]
mod tests {
    use super::AveragedCollection;

    #[test]
    fn it_works() {
        let v = vec![1, 2, 3]; //给vector
        let a: f64 = 2.0; //给数值
        let mut ac = AveragedCollection {
            //构造 AveragedCollection   mut可变的
            list: v,
            average: a,
        };

        //  去pop ----->
        ac.add(6);
        assert_eq!(ac.average, 3.0);

        ac.remove();
        assert_eq!(ac.average, 2.0);

        ac.remove();
        assert_eq!(ac.average, 1.5);

        ac.remove();
        assert_eq!(ac.average, 1.0);

        ac.remove(); //判断remove
        assert_eq!(ac.remove(), None);
    } // 1,2,3 得6       求这4个平均值
}
// RUST 是面向对象的语言
// git  cargo test
