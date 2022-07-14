pub fn learn_trait() {
    // 特征定义了一个可以被共享的行为，只要实现了特征，你就能使用该行为。

    // 定义特征
    // 如果不同的类型具有相同的行为，那么我们就可以定义一个特征，然后为这些类型实现该特征。定义特征是把一些方法组合在一起，目的是定义一个实现某些目标所必需的行为的集合。

    // 特征只定义行为看起来是什么样的，而不定义行为具体是怎么样的。因此，我们只定义特征方法的签名，而不进行实现，此时方法签名结尾是 ;，而不是一个 {}。
    pub trait Summary {
        fn summarize(&self) -> String;
    }
}
