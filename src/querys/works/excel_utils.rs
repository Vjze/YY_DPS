use rand_distr::{Distribution as _, Normal};
pub fn before_vf_num(vf: f64) -> f64 {
    let stddev = vf * 0.005;
    let mut rng = rand::rng();
    let normal = Normal::new(vf, stddev).unwrap();
    let fluctuation = normal.sample(&mut rng);
    let mut result = fluctuation;

    if result < 0.7 {
        result *= 1.15;
    } else if result > 1.5 {
        result *= 0.85;
    }
    if result < (vf - 0.5) {
        result = vf - 0.5;
    } else if result > (vf + 0.5) {
        result = vf + 0.5;
    }
    result
}
pub fn before_ith_num(ith: f64) -> f64 {
    let normal_dist = Normal::new(ith, 0.1).unwrap();

    // 生成符合正态分布的随机浮动
    let mut rng = rand::rng();
    let fluctuation = normal_dist.sample(&mut rng); // 获取一个符合正态分布的随机数

    let mut result = fluctuation;

    // 处理值大于 15 或小于 7 的情况
    if result > 15.0 {
        result *= 0.85; // 缩小值
    } else if result < 7.0 {
        result *= 1.15; // 放大值
    }

    // 最后再进行检查，避免结果超出范围
    if result < (ith - 1.0) {
        result = ith - 1.0;
    } else if result > (ith + 1.0) {
        result = ith + 1.0;
    }
    result
}
pub fn before_im_num(im: f64) -> f64 {
    let mut rng = rand::rng();
    let stddev = 30.0; // 30.0的标准差
    let normal = Normal::new(im, stddev).unwrap();
    let fluctuation = normal.sample(&mut rng);
    let mut result = fluctuation;
    if result > 1195.0 {
        result *= 0.85;
    } else if result < 105.0 {
        result *= 1.15;
    }
    // 最后再进行检查，避免结果超出范围
    if result < (im - 100.0) {
        result = im - 100.0;
    } else if result > (im + 100.0) {
        result = im + 100.0;
    }
    result
}

pub fn before_po(base_value: f64) -> f64 {
    // 基准值的标准差，基准值的5%作为标准差
    let stddev = base_value * 0.05;

    // 创建正态分布
    let normal_dist = Normal::new(base_value, stddev).unwrap();

    // 获取正态分布随机值
    let mut rng = rand::rng();
    let mut result = normal_dist.sample(&mut rng);

    if result > 3.35 {
        result *= 0.85; // 超过3.35，下调15%
    } else if result < 1.5 {
        result *= 1.15; // 小于1.5，上调15%
    }
    // 最后再进行检查，避免结果超出范围
    if result < (base_value - 0.5) {
        result = base_value - 0.5;
    } else if result > (base_value + 0.5) {
        result = base_value + 0.5;
    }
    result
}
pub fn before_po_303(base_value: f64) -> f64 {
    // 基准值的标准差，基准值的5%作为标准差
    let stddev = base_value * 0.05;

    // 创建正态分布
    let normal_dist = Normal::new(base_value, stddev).unwrap();

    // 获取正态分布随机值
    let mut rng = rand::rng();
    let mut result = normal_dist.sample(&mut rng);

    if result > 2.00 {
        result *= 0.85; // 超过3.35，下调15%
    } else if result < 1.2 {
        result *= 1.15; // 小于1.5，上调15%
    }
    // 最后再进行检查，避免结果超出范围
    if result < (base_value - 0.5) {
        result = base_value - 0.5;
    } else if result > (base_value + 0.5) {
        result = base_value + 0.5;
    }
    result
}
pub fn calculate_tc(pf_initial: f64, pf_final: f64) -> f64 {
    // 计算 Pf 比值
    let pf_ratio = pf_final / pf_initial;

    // 使用对数计算TC，取 10 为底的对数
    let tc = 10.0 * pf_ratio.log10();

    tc
}
