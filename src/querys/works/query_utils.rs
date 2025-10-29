// query_utils.rs
use crate::utils::{error::MyError, sql::get_tables};
use sqlx_oldapi::MssqlPool;
use tracing::info;
/// 为 10G 表 [MAC_10GBOSADATA] 定义的列和别名
/// 别名 (e.g., Pf as Po) 用于匹配 SnQueryRow 结构体
const SELECT_10G: &str = "
    SN, Ith, Pf as Po, Vop as Vf, Im, Rs, Se, Sen, Res, ICC, Vbr, Kink, 
    imkink as Imkink, TestDate as testtime, Idark, Result, 
    ProductBill as Tester, iop as Iop, ixtalk as I_xtalk, 
    MDPId as Mdpid, testtype as Yypn
";

/// 为 2.5G 动态表 (e.g., BOSA_2G5DATA_202206) 定义的列和别名
/// 别名 (e.g., Pslop as Se) 用于匹配 SnQueryRow 结构体
const SELECT_2_5G: &str = "
    SN, Ith, Po, Vf, Im, Rs, Pslop as Se, Sen, Res, ICC, Vbr, Kink_I as Kink, 
    kinkim_i as Imkink, TestDate as testtime, Idark, Result, 
    ProductBill as Tester, io as Iop, xtalk as I_xtalk, 
    Te as Mdpid, testtype as Yypn
";

/**
 * 构建基础的 UNION ALL 查询.
 * 这个函数替换了原文件中所有 70+ 个重复的 'build_query_sql...' 函数.
 * * 它根据 test_devices 参数决定包含哪些表，并使用列别名来确保
 * 所有 UNION ALL 的部分具有完全相同的列结构，以便 SnQueryRow 可以正确解析.
 */
pub async fn build_base_union_query(
    test_devices: &str,
    pool: &MssqlPool,
) -> anyhow::Result<String, MyError> {
    let mut sql_parts = Vec::new();

    // 1. 如果是 "10G" 或 "全部"，添加 10G 表
    if test_devices == "10G" || test_devices == "全部" {
        sql_parts.push(format!(
            "SELECT {0} FROM [BOSAautotest_Data].[dbo].[MAC_10GBOSADATA]",
            SELECT_10G
        ));
    }

    // 2. 如果是 "2.5G" 或 "全部"，添加所有 2.5G 的动态表
    if test_devices == "2.5G" || test_devices == "全部" {
        let tables = get_tables(pool).await?;
        for table in tables {
            // 确保动态表名不是 10G 表 (如果 get_tables 可能会返回它)
            if table.to_uppercase() != "[BOSAautotest_Data].[dbo].[MAC_10GBOSADATA]" {
                sql_parts.push(format!("SELECT {} FROM {}", SELECT_2_5G, table));
            }
        }
    }

    // 3. 如果没有匹配的查询，返回错误
    if sql_parts.is_empty() {
        if test_devices == "10G" || test_devices == "2.5G" || test_devices == "全部" {
            // 这意味着 'get_tables' 可能返回了空列表
            info!("没有找到 '2.5G' 的动态数据表");
        } else {
            // 传入了无效的 test_devices 参数
            return Err(MyError::Zdyknown(format!(
                "无效的 test_devices 参数: {}",
                test_devices
            )));
        }
    }

    // 4. 使用 UNION ALL 连接所有部分
    Ok(sql_parts.join(" UNION ALL "))
}
