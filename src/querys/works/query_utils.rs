use crate::utils::{error::MyError, sql::get_tables};
use sqlx_oldapi::MssqlPool;
pub async fn build_query_sql(
    sn_list: &str,
    pool: &MssqlPool,
) -> anyhow::Result<String, MyError> {
    // 定义字段
    let testtype = "SN,Ith,Pf,Vop,Im,Rs,Se,Sen,Res,ICC,Vbr,Kink,imkink,TestDate,Idark,Result,ProductBill,iop,ixtalk,MDPId,testtype";
    let testtype_12 = "SN,Ith,Po,Vf,Im,Rs,Pslop,Sen,Res,ICC,Vbr,Kink_I,kinkim_i,TestDate,Idark,Result,ProductBill,io,xtalk,Te,testtype";
    let sql_10 = format!(
        "SELECT {0} FROM [BOSAautotest_Data].[dbo].[MAC_10GBOSADATA] ",
        testtype
    );
    let mut sql_text = String::from(&sql_10);
    let tables = get_tables(pool).await?;
    for i in tables {
        let s = format!("UNION ALL SELECT {} FROM {} ", testtype_12, i);
        sql_text.push_str(&s);
    }
    if sql_text.ends_with(" UNION ALL ") {
        sql_text.truncate(sql_text.len() - " UNION ALL ".len());
    }
    let query_ty = if sn_list.is_empty() {
        format!("WHERE Result = 'OK' ORDER BY TestDate DESC")
    } else {
        format!(
            "WHERE SN IN ({}) AND Result = 'OK' ORDER BY TestDate DESC",
            sn_list
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_text, query_ty))
}

pub async fn build_query_sql_res_all(
    sn_list: &str,
    pool: &MssqlPool,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Pf,Vop,Im,Rs,Se,Sen,Res,ICC,Vbr,Kink,imkink,TestDate,Idark,Result,ProductBill,iop,ixtalk,MDPId,testtype";
    let testtype_12 = "SN,Ith,Po,Vf,Im,Rs,Pslop,Sen,Res,ICC,Vbr,Kink_I,kinkim_i,TestDate,Idark,Result,ProductBill,io,xtalk,Te,testtype";
    let sql_10 = format!(
        "SELECT {0} FROM [BOSAautotest_Data].[dbo].[MAC_10GBOSADATA] ",
        testtype
    );
    let mut sql_text = String::from(&sql_10);
    let tables = get_tables(pool).await?;
    for i in tables {
        let s = format!("UNION ALL SELECT {} FROM {} ", testtype_12, i);
        sql_text.push_str(&s);
    }
    if sql_text.ends_with(" UNION ALL ") {
        sql_text.truncate(sql_text.len() - " UNION ALL ".len());
    }
    let query_ty = if sn_list.is_empty() {
        format!("ORDER BY TestDate DESC")
    } else {
        format!("WHERE SN IN ({}) ORDER BY TestDate DESC", sn_list)
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_text, query_ty))
}

pub async fn build_query_sql_res_ng(
    sn_list: &str,
    pool: &MssqlPool,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Pf,Vop,Im,Rs,Se,Sen,Res,ICC,Vbr,Kink,imkink,TestDate,Idark,Result,ProductBill,iop,ixtalk,MDPId,testtype";
    let testtype_12 = "SN,Ith,Po,Vf,Im,Rs,Pslop,Sen,Res,ICC,Vbr,Kink_I,kinkim_i,TestDate,Idark,Result,ProductBill,io,xtalk,Te,testtype";
    let sql_10 = format!(
        "SELECT {0} FROM [BOSAautotest_Data].[dbo].[MAC_10GBOSADATA] ",
        testtype
    );
    let mut sql_text = String::from(&sql_10);
    let tables = get_tables(pool).await?;
    for i in tables {
        let s = format!("UNION ALL SELECT {} FROM {} ", testtype_12, i);
        sql_text.push_str(&s);
    }
    if sql_text.ends_with(" UNION ALL ") {
        sql_text.truncate(sql_text.len() - " UNION ALL ".len());
    }
    let query_ty = if sn_list.is_empty() {
        format!("WHERE Result = 'NG' ORDER BY TestDate DESC")
    } else {
        format!(
            "WHERE SN IN ({}) AND Result = 'NG' ORDER BY TestDate DESC",
            sn_list
        )
    };

    Ok(format!("SELECT * FROM ({}) tmp {}", sql_text, query_ty))
}

pub async fn build_query_sql_with_time(
    sn_list: &str,
    start_time: &str,
    end_time: &str,
    pool: &MssqlPool,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Pf,Vop,Im,Rs,Se,Sen,Res,ICC,Vbr,Kink,imkink,TestDate,Idark,Result,ProductBill,iop,ixtalk,MDPId,testtype";
    let testtype_12 = "SN,Ith,Po,Vf,Im,Rs,Pslop,Sen,Res,ICC,Vbr,Kink_I,kinkim_i,TestDate,Idark,Result,ProductBill,io,xtalk,Te,testtype";
    let sql_10 = format!(
        "SELECT {0} FROM [BOSAautotest_Data].[dbo].[MAC_10GBOSADATA] ",
        testtype
    );
    let mut sql_text = String::from(&sql_10);
    let tables = get_tables(pool).await?;
    for i in tables {
        let s = format!("UNION ALL SELECT {} FROM {} ", testtype_12, i);
        sql_text.push_str(&s);
    }
    if sql_text.ends_with(" UNION ALL ") {
        sql_text.truncate(sql_text.len() - " UNION ALL ".len());
    }
    let query_ty = if sn_list.is_empty() {
        format!(
            "WHERE Result = 'OK' AND TestDate BETWEEN '{}' AND '{}' ORDER BY TestDate DESC",
            start_time, end_time
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND Result = 'OK' AND TestDate BETWEEN '{}' AND '{}' ORDER BY TestDate DESC",
            sn_list, start_time, end_time
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_text, query_ty))
}

pub async fn build_query_sql_res_all_with_time(
    sn_list: &str,
    start_time: &str,
    end_time: &str,
    pool: &MssqlPool,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Pf,Vop,Im,Rs,Se,Sen,Res,ICC,Vbr,Kink,imkink,TestDate,Idark,Result,ProductBill,iop,ixtalk,MDPId,testtype";
    let testtype_12 = "SN,Ith,Po,Vf,Im,Rs,Pslop,Sen,Res,ICC,Vbr,Kink_I,kinkim_i,TestDate,Idark,Result,ProductBill,io,xtalk,Te,testtype";
    let sql_10 = format!(
        "SELECT {0} FROM [BOSAautotest_Data].[dbo].[MAC_10GBOSADATA] ",
        testtype
    );
    let mut sql_text = String::from(&sql_10);
    let tables = get_tables(pool).await?;
    for i in tables {
        let s = format!("UNION ALL SELECT {} FROM {} ", testtype_12, i);
        sql_text.push_str(&s);
    }
    if sql_text.ends_with(" UNION ALL ") {
        sql_text.truncate(sql_text.len() - " UNION ALL ".len());
    }
    let query_ty = if sn_list.is_empty() {
        format!(
            "WHERE TestDate BETWEEN '{}' AND '{}' ORDER BY TestDate DESC",
            start_time, end_time
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND TestDate BETWEEN '{}' AND '{}' ORDER BY TestDate DESC",
            sn_list, start_time, end_time
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_text, query_ty))
}

pub async fn build_query_sql_res_ng_with_time(
    sn_list: &str,
    start_time: &str,
    end_time: &str,
    pool: &MssqlPool,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Pf,Vop,Im,Rs,Se,Sen,Res,ICC,Vbr,Kink,imkink,TestDate,Idark,Result,ProductBill,iop,ixtalk,MDPId,testtype";
    let testtype_12 = "SN,Ith,Po,Vf,Im,Rs,Pslop,Sen,Res,ICC,Vbr,Kink_I,kinkim_i,TestDate,Idark,Result,ProductBill,io,xtalk,Te,testtype";
    let sql_10 = format!(
        "SELECT {0} FROM [BOSAautotest_Data].[dbo].[MAC_10GBOSADATA] ",
        testtype
    );
    let mut sql_text = String::from(&sql_10);
    let tables = get_tables(pool).await?;
    for i in tables {
        let s = format!("UNION ALL SELECT {} FROM {} ", testtype_12, i);
        sql_text.push_str(&s);
    }
    if sql_text.ends_with(" UNION ALL ") {
        sql_text.truncate(sql_text.len() - " UNION ALL ".len());
    }
    let query_ty = if sn_list.is_empty() {
        format!(
            "WHERE Result = 'NG' AND TestDate BETWEEN '{}' AND '{}' ORDER BY TestDate DESC",
            start_time, end_time
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND Result = 'NG' AND TestDate BETWEEN '{}' AND '{}' ORDER BY TestDate DESC",
            sn_list, start_time, end_time
        )
    };

    Ok(format!("SELECT * FROM ({}) tmp {}", sql_text, query_ty))
}

pub async fn build_query_sql_with_testtype(
    sn_list: &str,
    testtype_param: &str,
    pool: &MssqlPool,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Pf,Vop,Im,Rs,Se,Sen,Res,ICC,Vbr,Kink,imkink,TestDate,Idark,Result,ProductBill,iop,ixtalk,MDPId,testtype";
    let testtype_12 = "SN,Ith,Po,Vf,Im,Rs,Pslop,Sen,Res,ICC,Vbr,Kink_I,kinkim_i,TestDate,Idark,Result,ProductBill,io,xtalk,Te,testtype";
    let sql_10 = format!(
        "SELECT {0} FROM [BOSAautotest_Data].[dbo].[MAC_10GBOSADATA] ",
        testtype
    );
    let mut sql_text = String::from(&sql_10);
    let tables = get_tables(pool).await?;
    for i in tables {
        let s = format!("UNION ALL SELECT {} FROM {} ", testtype_12, i);
        sql_text.push_str(&s);
    }
    if sql_text.ends_with(" UNION ALL ") {
        sql_text.truncate(sql_text.len() - " UNION ALL ".len());
    }
    let query_ty = if sn_list.is_empty() {
        format!(
            "WHERE Result = 'OK' AND testtype = '{}' ORDER BY TestDate DESC",
            testtype_param
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND Result = 'OK' AND testtype = '{}' ORDER BY TestDate DESC",
            sn_list, testtype_param
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_text, query_ty))
}

pub async fn build_query_sql_res_all_with_testtype(
    sn_list: &str,
    testtype_param: &str,
    pool: &MssqlPool,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Pf,Vop,Im,Rs,Se,Sen,Res,ICC,Vbr,Kink,imkink,TestDate,Idark,Result,ProductBill,iop,ixtalk,MDPId,testtype";
    let testtype_12 = "SN,Ith,Po,Vf,Im,Rs,Pslop,Sen,Res,ICC,Vbr,Kink_I,kinkim_i,TestDate,Idark,Result,ProductBill,io,xtalk,Te,testtype";
    let sql_10 = format!(
        "SELECT {0} FROM [BOSAautotest_Data].[dbo].[MAC_10GBOSADATA] ",
        testtype
    );
    let mut sql_text = String::from(&sql_10);
    let tables = get_tables(pool).await?;
    for i in tables {
        let s = format!("UNION ALL SELECT {} FROM {} ", testtype_12, i);
        sql_text.push_str(&s);
    }
    if sql_text.ends_with(" UNION ALL ") {
        sql_text.truncate(sql_text.len() - " UNION ALL ".len());
    }
    let query_ty = if sn_list.is_empty() {
        format!(
            "WHERE testtype = '{}' ORDER BY TestDate DESC",
            testtype_param
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND testtype = '{}' ORDER BY TestDate DESC",
            sn_list, testtype_param
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_text, query_ty))
}

pub async fn build_query_sql_res_ng_with_testtype(
    sn_list: &str,
    testtype_param: &str,
    pool: &MssqlPool,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Pf,Vop,Im,Rs,Se,Sen,Res,ICC,Vbr,Kink,imkink,TestDate,Idark,Result,ProductBill,iop,ixtalk,MDPId,testtype";
    let testtype_12 = "SN,Ith,Po,Vf,Im,Rs,Pslop,Sen,Res,ICC,Vbr,Kink_I,kinkim_i,TestDate,Idark,Result,ProductBill,io,xtalk,Te,testtype";
    let sql_10 = format!(
        "SELECT {0} FROM [BOSAautotest_Data].[dbo].[MAC_10GBOSADATA] ",
        testtype
    );
    let mut sql_text = String::from(&sql_10);
    let tables = get_tables(pool).await?;
    for i in tables {
        let s = format!("UNION ALL SELECT {} FROM {} ", testtype_12, i);
        sql_text.push_str(&s);
    }
    if sql_text.ends_with(" UNION ALL ") {
        sql_text.truncate(sql_text.len() - " UNION ALL ".len());
    }
    let query_ty = if sn_list.is_empty() {
        format!(
            "WHERE Result = 'NG' AND testtype = '{}' ORDER BY TestDate DESC",
            testtype_param
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND Result = 'NG' AND testtype = '{}' ORDER BY TestDate DESC",
            sn_list, testtype_param
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_text, query_ty))
}

pub async fn build_query_sql_with_time_and_testtype(
    sn_list: &str,
    start_time: &str,
    end_time: &str,
    testtype_param: &str,
    pool: &MssqlPool,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Pf,Vop,Im,Rs,Se,Sen,Res,ICC,Vbr,Kink,imkink,TestDate,Idark,Result,ProductBill,iop,ixtalk,MDPId,testtype";
    let testtype_12 = "SN,Ith,Po,Vf,Im,Rs,Pslop,Sen,Res,ICC,Vbr,Kink_I,kinkim_i,TestDate,Idark,Result,ProductBill,io,xtalk,Te,testtype";
    let sql_10 = format!(
        "SELECT {0} FROM [BOSAautotest_Data].[dbo].[MAC_10GBOSADATA] ",
        testtype
    );
    let mut sql_text = String::from(&sql_10);
    let tables = get_tables(pool).await?;
    for i in tables {
        let s = format!("UNION ALL SELECT {} FROM {} ", testtype_12, i);
        sql_text.push_str(&s);
    }
    if sql_text.ends_with(" UNION ALL ") {
        sql_text.truncate(sql_text.len() - " UNION ALL ".len());
    }
    let query_ty = if sn_list.is_empty() {
        format!(
            "WHERE Result = 'OK' AND TestDate BETWEEN '{}' AND '{}' AND testtype = '{}' ORDER BY TestDate DESC",
            start_time, end_time, testtype_param
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND Result = 'OK' AND TestDate BETWEEN '{}' AND '{}' AND testtype = '{}' ORDER BY TestDate DESC",
            sn_list, start_time, end_time, testtype_param
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_text, query_ty))
}

pub async fn build_query_sql_res_all_with_time_and_testtype(
    sn_list: &str,
    start_time: &str,
    end_time: &str,
    testtype_param: &str,
    pool: &MssqlPool,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Pf,Vop,Im,Rs,Se,Sen,Res,ICC,Vbr,Kink,imkink,TestDate,Idark,Result,ProductBill,iop,ixtalk,MDPId,testtype";
    let testtype_12 = "SN,Ith,Po,Vf,Im,Rs,Pslop,Sen,Res,ICC,Vbr,Kink_I,kinkim_i,TestDate,Idark,Result,ProductBill,io,xtalk,Te,testtype";
    let sql_10 = format!(
        "SELECT {0} FROM [BOSAautotest_Data].[dbo].[MAC_10GBOSADATA] ",
        testtype
    );
    let mut sql_text = String::from(&sql_10);
    let tables = get_tables(pool).await?;
    for i in tables {
        let s = format!("UNION ALL SELECT {} FROM {} ", testtype_12, i);
        sql_text.push_str(&s);
    }
    if sql_text.ends_with(" UNION ALL ") {
        sql_text.truncate(sql_text.len() - " UNION ALL ".len());
    }
    let query_ty = if sn_list.is_empty() {
        format!(
            "WHERE TestDate BETWEEN '{}' AND '{}' AND testtype = '{}' ORDER BY TestDate DESC",
            start_time, end_time, testtype_param
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND TestDate BETWEEN '{}' AND '{}' AND testtype = '{}' ORDER BY TestDate DESC",
            sn_list, start_time, end_time, testtype_param
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_text, query_ty))
}

pub async fn build_query_sql_res_ng_with_time_and_testtype(
    sn_list: &str,
    start_time: &str,
    end_time: &str,
    testtype_param: &str,
    pool: &MssqlPool,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Pf,Vop,Im,Rs,Se,Sen,Res,ICC,Vbr,Kink,imkink,TestDate,Idark,Result,ProductBill,iop,ixtalk,MDPId,testtype";
    let testtype_12 = "SN,Ith,Po,Vf,Im,Rs,Pslop,Sen,Res,ICC,Vbr,Kink_I,kinkim_i,TestDate,Idark,Result,ProductBill,io,xtalk,Te,testtype";
    let sql_10 = format!(
        "SELECT {0} FROM [BOSAautotest_Data].[dbo].[MAC_10GBOSADATA] ",
        testtype
    );
    let mut sql_text = String::from(&sql_10);
    let tables = get_tables(pool).await?;
    for i in tables {
        let s = format!("UNION ALL SELECT {} FROM {} ", testtype_12, i);
        sql_text.push_str(&s);
    }
    if sql_text.ends_with(" UNION ALL ") {
        sql_text.truncate(sql_text.len() - " UNION ALL ".len());
    }
    let query_ty = if sn_list.is_empty() {
        format!(
            "WHERE Result = 'NG' AND TestDate BETWEEN '{}' AND '{}' AND testtype = '{}' ORDER BY TestDate DESC",
            start_time, end_time, testtype_param
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND Result = 'NG' AND TestDate BETWEEN '{}' AND '{}' AND testtype = '{}' ORDER BY TestDate DESC",
            sn_list, start_time, end_time, testtype_param
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_text, query_ty))
}

pub async fn build_query_sql_with_worker(
    sn_list: &str,
    worker: &str,
    pool: &MssqlPool,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Pf,Vop,Im,Rs,Se,Sen,Res,ICC,Vbr,Kink,imkink,TestDate,Idark,Result,ProductBill,iop,ixtalk,MDPId,testtype";
    let testtype_12 = "SN,Ith,Po,Vf,Im,Rs,Pslop,Sen,Res,ICC,Vbr,Kink_I,kinkim_i,TestDate,Idark,Result,ProductBill,io,xtalk,Te,testtype";
    let sql_10 = format!(
        "SELECT {0} FROM [BOSAautotest_Data].[dbo].[MAC_10GBOSADATA] ",
        testtype
    );
    let mut sql_text = String::from(&sql_10);
    let tables = get_tables(pool).await?;
    for i in tables {
        let s = format!("UNION ALL SELECT {} FROM {} ", testtype_12, i);
        sql_text.push_str(&s);
    }
    if sql_text.ends_with(" UNION ALL ") {
        sql_text.truncate(sql_text.len() - " UNION ALL ".len());
    }
    let query_ty = if sn_list.is_empty() {
        format!(
            "WHERE Result = 'OK' AND ProductBill = '{}' ORDER BY TestDate DESC",
            worker
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND Result = 'OK' AND ProductBill = '{}' ORDER BY TestDate DESC",
            sn_list, worker
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_text, query_ty))
}

pub async fn build_query_sql_res_all_with_worker(
    sn_list: &str,
    worker: &str,
    pool: &MssqlPool,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Pf,Vop,Im,Rs,Se,Sen,Res,ICC,Vbr,Kink,imkink,TestDate,Idark,Result,ProductBill,iop,ixtalk,MDPId,testtype";
    let testtype_12 = "SN,Ith,Po,Vf,Im,Rs,Pslop,Sen,Res,ICC,Vbr,Kink_I,kinkim_i,TestDate,Idark,Result,ProductBill,io,xtalk,Te,testtype";
    let sql_10 = format!(
        "SELECT {0} FROM [BOSAautotest_Data].[dbo].[MAC_10GBOSADATA] ",
        testtype
    );
    let mut sql_text = String::from(&sql_10);
    let tables = get_tables(pool).await?;
    for i in tables {
        let s = format!("UNION ALL SELECT {} FROM {} ", testtype_12, i);
        sql_text.push_str(&s);
    }
    if sql_text.ends_with(" UNION ALL ") {
        sql_text.truncate(sql_text.len() - " UNION ALL ".len());
    }
    let query_ty = if sn_list.is_empty() {
        format!("WHERE ProductBill = '{}' ORDER BY TestDate DESC", worker)
    } else {
        format!(
            "WHERE SN IN ({}) AND ProductBill = '{}' ORDER BY TestDate DESC",
            sn_list, worker
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_text, query_ty))
}

pub async fn build_query_sql_res_ng_with_worker(
    sn_list: &str,
    worker: &str,
    pool: &MssqlPool,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Pf,Vop,Im,Rs,Se,Sen,Res,ICC,Vbr,Kink,imkink,TestDate,Idark,Result,ProductBill,iop,ixtalk,MDPId,testtype";
    let testtype_12 = "SN,Ith,Po,Vf,Im,Rs,Pslop,Sen,Res,ICC,Vbr,Kink_I,kinkim_i,TestDate,Idark,Result,ProductBill,io,xtalk,Te,testtype";
    let sql_10 = format!(
        "SELECT {0} FROM [BOSAautotest_Data].[dbo].[MAC_10GBOSADATA] ",
        testtype
    );
    let mut sql_text = String::from(&sql_10);
    let tables = get_tables(pool).await?;
    for i in tables {
        let s = format!("UNION ALL SELECT {} FROM {} ", testtype_12, i);
        sql_text.push_str(&s);
    }
    if sql_text.ends_with(" UNION ALL ") {
        sql_text.truncate(sql_text.len() - " UNION ALL ".len());
    }
    let query_ty = if sn_list.is_empty() {
        format!(
            "WHERE Result = 'NG' AND ProductBill = '{}' ORDER BY TestDate DESC",
            worker
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND Result = 'NG' AND ProductBill = '{}' ORDER BY TestDate DESC",
            sn_list, worker
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_text, query_ty))
}

pub async fn build_query_sql_with_time_and_worker(
    sn_list: &str,
    start_time: &str,
    end_time: &str,
    worker: &str,
    pool: &MssqlPool,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Pf,Vop,Im,Rs,Se,Sen,Res,ICC,Vbr,Kink,imkink,TestDate,Idark,Result,ProductBill,iop,ixtalk,MDPId,testtype";
    let testtype_12 = "SN,Ith,Po,Vf,Im,Rs,Pslop,Sen,Res,ICC,Vbr,Kink_I,kinkim_i,TestDate,Idark,Result,ProductBill,io,xtalk,Te,testtype";
    let sql_10 = format!(
        "SELECT {0} FROM [BOSAautotest_Data].[dbo].[MAC_10GBOSADATA] ",
        testtype
    );
    let mut sql_text = String::from(&sql_10);
    let tables = get_tables(pool).await?;
    for i in tables {
        let s = format!("UNION ALL SELECT {} FROM {} ", testtype_12, i);
        sql_text.push_str(&s);
    }
    if sql_text.ends_with(" UNION ALL ") {
        sql_text.truncate(sql_text.len() - " UNION ALL ".len());
    }
    let query_ty = if sn_list.is_empty() {
        format!(
            "WHERE Result = 'OK' AND TestDate BETWEEN '{}' AND '{}' AND ProductBill = '{}' ORDER BY TestDate DESC",
            start_time, end_time, worker
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND Result = 'OK' AND TestDate BETWEEN '{}' AND '{}' AND ProductBill = '{}' ORDER BY TestDate DESC",
            sn_list, start_time, end_time, worker
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_text, query_ty))
}

pub async fn build_query_sql_res_all_with_time_and_worker(
    sn_list: &str,
    start_time: &str,
    end_time: &str,
    worker: &str,
    pool: &MssqlPool,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Pf,Vop,Im,Rs,Se,Sen,Res,ICC,Vbr,Kink,imkink,TestDate,Idark,Result,ProductBill,iop,ixtalk,MDPId,testtype";
    let testtype_12 = "SN,Ith,Po,Vf,Im,Rs,Pslop,Sen,Res,ICC,Vbr,Kink_I,kinkim_i,TestDate,Idark,Result,ProductBill,io,xtalk,Te,testtype";
    let sql_10 = format!(
        "SELECT {0} FROM [BOSAautotest_Data].[dbo].[MAC_10GBOSADATA] ",
        testtype
    );
    let mut sql_text = String::from(&sql_10);
    let tables = get_tables(pool).await?;
    for i in tables {
        let s = format!("UNION ALL SELECT {} FROM {} ", testtype_12, i);
        sql_text.push_str(&s);
    }
    if sql_text.ends_with(" UNION ALL ") {
        sql_text.truncate(sql_text.len() - " UNION ALL ".len());
    }
    let query_ty = if sn_list.is_empty() {
        format!(
            "WHERE TestDate BETWEEN '{}' AND '{}' AND ProductBill = '{}' ORDER BY TestDate DESC",
            start_time, end_time, worker
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND TestDate BETWEEN '{}' AND '{}' AND ProductBill = '{}' ORDER BY TestDate DESC",
            sn_list, start_time, end_time, worker
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_text, query_ty))
}

pub async fn build_query_sql_res_ng_with_time_and_worker(
    sn_list: &str,
    start_time: &str,
    end_time: &str,
    worker: &str,
    pool: &MssqlPool,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Pf,Vop,Im,Rs,Se,Sen,Res,ICC,Vbr,Kink,imkink,TestDate,Idark,Result,ProductBill,iop,ixtalk,MDPId,testtype";
    let testtype_12 = "SN,Ith,Po,Vf,Im,Rs,Pslop,Sen,Res,ICC,Vbr,Kink_I,kinkim_i,TestDate,Idark,Result,ProductBill,io,xtalk,Te,testtype";
    let sql_10 = format!(
        "SELECT {0} FROM [BOSAautotest_Data].[dbo].[MAC_10GBOSADATA] ",
        testtype
    );
    let mut sql_text = String::from(&sql_10);
    let tables = get_tables(pool).await?;
    for i in tables {
        let s = format!("UNION ALL SELECT {} FROM {} ", testtype_12, i);
        sql_text.push_str(&s);
    }
    if sql_text.ends_with(" UNION ALL ") {
        sql_text.truncate(sql_text.len() - " UNION ALL ".len());
    }
    let query_ty = if sn_list.is_empty() {
        format!(
            "WHERE Result = 'NG' AND TestDate BETWEEN '{}' AND '{}' AND ProductBill = '{}' ORDER BY TestDate DESC",
            start_time, end_time, worker
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND Result = 'NG' AND TestDate BETWEEN '{}' AND '{}' AND ProductBill = '{}' ORDER BY TestDate DESC",
            sn_list, start_time, end_time, worker
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_text, query_ty))
}

pub async fn build_query_sql_with_testtype_and_worker(
    sn_list: &str,
    testtype_param: &str,
    worker: &str,
    pool: &MssqlPool,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Pf,Vop,Im,Rs,Se,Sen,Res,ICC,Vbr,Kink,imkink,TestDate,Idark,Result,ProductBill,iop,ixtalk,MDPId,testtype";
    let testtype_12 = "SN,Ith,Po,Vf,Im,Rs,Pslop,Sen,Res,ICC,Vbr,Kink_I,kinkim_i,TestDate,Idark,Result,ProductBill,io,xtalk,Te,testtype";
    let sql_10 = format!(
        "SELECT {0} FROM [BOSAautotest_Data].[dbo].[MAC_10GBOSADATA] ",
        testtype
    );
    let mut sql_text = String::from(&sql_10);
    let tables = get_tables(pool).await?;
    for i in tables {
        let s = format!("UNION ALL SELECT {} FROM {} ", testtype_12, i);
        sql_text.push_str(&s);
    }
    if sql_text.ends_with(" UNION ALL ") {
        sql_text.truncate(sql_text.len() - " UNION ALL ".len());
    }
    let query_ty = if sn_list.is_empty() {
        format!(
            "WHERE Result = 'OK' AND testtype = '{}' AND ProductBill = '{}' ORDER BY TestDate DESC",
            testtype_param, worker
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND Result = 'OK' AND testtype = '{}' AND ProductBill = '{}' ORDER BY TestDate DESC",
            sn_list, testtype_param, worker
        )
    };

    Ok(format!("SELECT * FROM ({}) tmp {}", sql_text, query_ty))
}

pub async fn build_query_sql_res_all_with_testtype_and_worker(
    sn_list: &str,
    testtype_param: &str,
    worker: &str,
    pool: &MssqlPool,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Pf,Vop,Im,Rs,Se,Sen,Res,ICC,Vbr,Kink,imkink,TestDate,Idark,Result,ProductBill,iop,ixtalk,MDPId,testtype";
    let testtype_12 = "SN,Ith,Po,Vf,Im,Rs,Pslop,Sen,Res,ICC,Vbr,Kink_I,kinkim_i,TestDate,Idark,Result,ProductBill,io,xtalk,Te,testtype";
    let sql_10 = format!(
        "SELECT {0} FROM [BOSAautotest_Data].[dbo].[MAC_10GBOSADATA] ",
        testtype
    );
    let mut sql_text = String::from(&sql_10);
    let tables = get_tables(pool).await?;
    for i in tables {
        let s = format!("UNION ALL SELECT {} FROM {} ", testtype_12, i);
        sql_text.push_str(&s);
    }
    if sql_text.ends_with(" UNION ALL ") {
        sql_text.truncate(sql_text.len() - " UNION ALL ".len());
    }
    let query_ty = if sn_list.is_empty() {
        format!(
            "WHERE testtype = '{}' AND ProductBill = '{}' ORDER BY TestDate DESC",
            testtype_param, worker
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND testtype = '{}' AND ProductBill = '{}' ORDER BY TestDate DESC",
            sn_list, testtype_param, worker
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_text, query_ty))
}

pub async fn build_query_sql_res_ng_with_testtype_and_worker(
    sn_list: &str,
    testtype_param: &str,
    worker: &str,
    pool: &MssqlPool,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Pf,Vop,Im,Rs,Se,Sen,Res,ICC,Vbr,Kink,imkink,TestDate,Idark,Result,ProductBill,iop,ixtalk,MDPId,testtype";
    let testtype_12 = "SN,Ith,Po,Vf,Im,Rs,Pslop,Sen,Res,ICC,Vbr,Kink_I,kinkim_i,TestDate,Idark,Result,ProductBill,io,xtalk,Te,testtype";
    let sql_10 = format!(
        "SELECT {0} FROM [BOSAautotest_Data].[dbo].[MAC_10GBOSADATA] ",
        testtype
    );
    let mut sql_text = String::from(&sql_10);
    let tables = get_tables(pool).await?;
    for i in tables {
        let s = format!("UNION ALL SELECT {} FROM {} ", testtype_12, i);
        sql_text.push_str(&s);
    }
    if sql_text.ends_with(" UNION ALL ") {
        sql_text.truncate(sql_text.len() - " UNION ALL ".len());
    }
    let query_ty = if sn_list.is_empty() {
        format!(
            "WHERE Result = 'NG' AND testtype = '{}' AND ProductBill = '{}' ORDER BY TestDate DESC",
            testtype_param, worker
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND Result = 'NG' AND testtype = '{}' AND ProductBill = '{}' ORDER BY TestDate DESC",
            sn_list, testtype_param, worker
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_text, query_ty))
}

pub async fn build_query_sql_with_time_and_testtype_and_worker(
    sn_list: &str,
    start_time: &str,
    end_time: &str,
    testtype_param: &str,
    worker: &str,
    pool: &MssqlPool,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Pf,Vop,Im,Rs,Se,Sen,Res,ICC,Vbr,Kink,imkink,TestDate,Idark,Result,ProductBill,iop,ixtalk,MDPId,testtype";
    let testtype_12 = "SN,Ith,Po,Vf,Im,Rs,Pslop,Sen,Res,ICC,Vbr,Kink_I,kinkim_i,TestDate,Idark,Result,ProductBill,io,xtalk,Te,testtype";
    let sql_10 = format!(
        "SELECT {0} FROM [BOSAautotest_Data].[dbo].[MAC_10GBOSADATA] ",
        testtype
    );
    let mut sql_text = String::from(&sql_10);
    let tables = get_tables(pool).await?;
    for i in tables {
        let s = format!("UNION ALL SELECT {} FROM {} ", testtype_12, i);
        sql_text.push_str(&s);
    }
    if sql_text.ends_with(" UNION ALL ") {
        sql_text.truncate(sql_text.len() - " UNION ALL ".len());
    }
    let query_ty = if sn_list.is_empty() {
        format!(
            "WHERE Result = 'OK' AND TestDate BETWEEN '{}' AND '{}' AND testtype = '{}' AND ProductBill = '{}' ORDER BY TestDate DESC",
            start_time, end_time, testtype_param, worker
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND Result = 'OK' AND TestDate BETWEEN '{}' AND '{}' AND testtype = '{}' AND ProductBill = '{}' ORDER BY TestDate DESC",
            sn_list, start_time, end_time, testtype_param, worker
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_text, query_ty))
}

pub async fn build_query_sql_res_all_with_time_and_testtype_and_worker(
    sn_list: &str,
    start_time: &str,
    end_time: &str,
    testtype_param: &str,
    worker: &str,
    pool: &MssqlPool,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Pf,Vop,Im,Rs,Se,Sen,Res,ICC,Vbr,Kink,imkink,TestDate,Idark,Result,ProductBill,iop,ixtalk,MDPId,testtype";
    let testtype_12 = "SN,Ith,Po,Vf,Im,Rs,Pslop,Sen,Res,ICC,Vbr,Kink_I,kinkim_i,TestDate,Idark,Result,ProductBill,io,xtalk,Te,testtype";
    let sql_10 = format!(
        "SELECT {0} FROM [BOSAautotest_Data].[dbo].[MAC_10GBOSADATA] ",
        testtype
    );
    let mut sql_text = String::from(&sql_10);
    let tables = get_tables(pool).await?;
    for i in tables {
        let s = format!("UNION ALL SELECT {} FROM {} ", testtype_12, i);
        sql_text.push_str(&s);
    }
    if sql_text.ends_with(" UNION ALL ") {
        sql_text.truncate(sql_text.len() - " UNION ALL ".len());
    }
    let query_ty = if sn_list.is_empty() {
        format!(
            "WHERE TestDate BETWEEN '{}' AND '{}' AND testtype = '{}' AND ProductBill = '{}' ORDER BY TestDate DESC",
            start_time, end_time, testtype_param, worker
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND TestDate BETWEEN '{}' AND '{}' AND testtype = '{}' AND ProductBill = '{}' ORDER BY TestDate DESC",
            sn_list, start_time, end_time, testtype_param, worker
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_text, query_ty))
}

pub async fn build_query_sql_res_ng_with_time_and_testtype_and_worker(
    sn_list: &str,
    start_time: &str,
    end_time: &str,
    testtype_param: &str,
    worker: &str,
    pool: &MssqlPool,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Pf,Vop,Im,Rs,Se,Sen,Res,ICC,Vbr,Kink,imkink,TestDate,Idark,Result,ProductBill,iop,ixtalk,MDPId,testtype";
    let testtype_12 = "SN,Ith,Po,Vf,Im,Rs,Pslop,Sen,Res,ICC,Vbr,Kink_I,kinkim_i,TestDate,Idark,Result,ProductBill,io,xtalk,Te,testtype";
    let sql_10 = format!(
        "SELECT {0} FROM [BOSAautotest_Data].[dbo].[MAC_10GBOSADATA] ",
        testtype
    );
    let mut sql_text = String::from(&sql_10);
    let tables = get_tables(pool).await?;
    for i in tables {
        let s = format!("UNION ALL SELECT {} FROM {} ", testtype_12, i);
        sql_text.push_str(&s);
    }
    if sql_text.ends_with(" UNION ALL ") {
        sql_text.truncate(sql_text.len() - " UNION ALL ".len());
    }
    let query_ty = if sn_list.is_empty() {
        format!(
            "WHERE Result = 'NG' AND TestDate BETWEEN '{}' AND '{}' AND testtype = '{}' AND ProductBill = '{}' ORDER BY TestDate DESC",
            start_time, end_time, testtype_param, worker
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND Result = 'NG' AND TestDate BETWEEN '{}' AND '{}' AND testtype = '{}' AND ProductBill = '{}' ORDER BY TestDate DESC",
            sn_list, start_time, end_time, testtype_param, worker
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_text, query_ty))
}

pub async fn build_query_sql_10g(sn_list: &str) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Pf,Vop,Im,Rs,Se,Sen,Res,ICC,Vbr,Kink,imkink,TestDate,Idark,Result,ProductBill,iop,ixtalk,MDPId,testtype";
    let sql_10 = format!(
        "SELECT {0} FROM [BOSAautotest_Data].[dbo].[MAC_10GBOSADATA] ",
        testtype
    );
    let query_ty = if sn_list.is_empty() {
        format!("WHERE Result = 'OK' ORDER BY TestDate DESC")
    } else {
        format!(
            "WHERE SN IN ({}) AND Result = 'OK' ORDER BY TestDate DESC",
            sn_list
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_10, query_ty))
}

pub async fn build_query_sql_10g_res_all(sn_list: &str) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Pf,Vop,Im,Rs,Se,Sen,Res,ICC,Vbr,Kink,imkink,TestDate,Idark,Result,ProductBill,iop,ixtalk,MDPId,testtype";
    let sql_10 = format!(
        "SELECT {0} FROM [BOSAautotest_Data].[dbo].[MAC_10GBOSADATA] ",
        testtype
    );
    let query_ty = if sn_list.is_empty() {
        format!("ORDER BY TestDate DESC")
    } else {
        format!("WHERE SN IN ({}) ORDER BY TestDate DESC", sn_list)
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_10, query_ty))
}

pub async fn build_query_sql_10g_res_ng(sn_list: &str) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Pf,Vop,Im,Rs,Se,Sen,Res,ICC,Vbr,Kink,imkink,TestDate,Idark,Result,ProductBill,iop,ixtalk,MDPId,testtype";
    let sql_10 = format!(
        "SELECT {0} FROM [BOSAautotest_Data].[dbo].[MAC_10GBOSADATA] ",
        testtype
    );
    let query_ty = if sn_list.is_empty() {
        format!("WHERE Result = 'NG' ORDER BY TestDate DESC")
    } else {
        format!(
            "WHERE SN IN ({}) AND Result = 'NG' ORDER BY TestDate DESC",
            sn_list
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_10, query_ty))
}

pub async fn build_query_sql_10g_with_time(
    sn_list: &str,
    start_time: &str,
    end_time: &str,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Pf,Vop,Im,Rs,Se,Sen,Res,ICC,Vbr,Kink,imkink,TestDate,Idark,Result,ProductBill,iop,ixtalk,MDPId,testtype";
    let sql_10 = format!(
        "SELECT {0} FROM [BOSAautotest_Data].[dbo].[MAC_10GBOSADATA] ",
        testtype
    );
    let query_ty = if sn_list.is_empty() {
        format!(
            "WHERE  Result = 'OK' AND TestDate BETWEEN '{}' AND '{}' ORDER BY TestDate DESC",
            start_time, end_time
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND Result = 'OK' AND TestDate BETWEEN '{}' AND '{}' ORDER BY TestDate DESC",
            sn_list, start_time, end_time
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_10, query_ty))
}

pub async fn build_query_sql_10g_res_all_with_time(
    sn_list: &str,
    start_time: &str,
    end_time: &str,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Pf,Vop,Im,Rs,Se,Sen,Res,ICC,Vbr,Kink,imkink,TestDate,Idark,Result,ProductBill,iop,ixtalk,MDPId,testtype";
    let sql_10 = format!(
        "SELECT {0} FROM [BOSAautotest_Data].[dbo].[MAC_10GBOSADATA] ",
        testtype
    );
    let query_ty = if sn_list.is_empty() {
        format!(
            "WHERE TestDate BETWEEN '{}' AND '{}' ORDER BY TestDate DESC",
            start_time, end_time
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND TestDate BETWEEN '{}' AND '{}' ORDER BY TestDate DESC",
            sn_list, start_time, end_time
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_10, query_ty))
}

pub async fn build_query_sql_10g_res_ng_with_time(
    sn_list: &str,
    start_time: &str,
    end_time: &str,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Pf,Vop,Im,Rs,Se,Sen,Res,ICC,Vbr,Kink,imkink,TestDate,Idark,Result,ProductBill,iop,ixtalk,MDPId,testtype";
    let sql_10 = format!(
        "SELECT {0} FROM [BOSAautotest_Data].[dbo].[MAC_10GBOSADATA] ",
        testtype
    );
    let query_ty = if sn_list.is_empty() {
        format!(
            "WHERE Result = 'NG' AND TestDate BETWEEN '{}' AND '{}' ORDER BY TestDate DESC",
            start_time, end_time
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND Result = 'NG' AND TestDate BETWEEN '{}' AND '{}' ORDER BY TestDate DESC",
            sn_list, start_time, end_time
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_10, query_ty))
}

pub async fn build_query_sql_10g_with_testtype(
    sn_list: &str,
    testtype_param: &str,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Pf,Vop,Im,Rs,Se,Sen,Res,ICC,Vbr,Kink,imkink,TestDate,Idark,Result,ProductBill,iop,ixtalk,MDPId,testtype";
    let sql_10 = format!(
        "SELECT {0} FROM [BOSAautotest_Data].[dbo].[MAC_10GBOSADATA] ",
        testtype
    );
    let query_ty = if sn_list.is_empty() {
        format!(
            "WHERE Result = 'OK' AND testtype = '{}' ORDER BY TestDate DESC",
            testtype_param
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND Result = 'OK' AND testtype = '{}' ORDER BY TestDate DESC",
            sn_list, testtype_param
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_10, query_ty))
}

pub async fn build_query_sql_10g_res_all_with_testtype(
    sn_list: &str,
    testtype_param: &str,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Pf,Vop,Im,Rs,Se,Sen,Res,ICC,Vbr,Kink,imkink,TestDate,Idark,Result,ProductBill,iop,ixtalk,MDPId,testtype";
    let sql_10 = format!(
        "SELECT {0} FROM [BOSAautotest_Data].[dbo].[MAC_10GBOSADATA] ",
        testtype
    );
    let query_ty = if sn_list.is_empty() {
        format!(
            "WHERE testtype = '{}' ORDER BY TestDate DESC",
            testtype_param
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND testtype = '{}' ORDER BY TestDate DESC",
            sn_list, testtype_param
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_10, query_ty))
}

pub async fn build_query_sql_10g_res_ng_with_testtype(
    sn_list: &str,
    testtype_param: &str,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Pf,Vop,Im,Rs,Se,Sen,Res,ICC,Vbr,Kink,imkink,TestDate,Idark,Result,ProductBill,iop,ixtalk,MDPId,testtype";
    let sql_10 = format!(
        "SELECT {0} FROM [BOSAautotest_Data].[dbo].[MAC_10GBOSADATA] ",
        testtype
    );
    let query_ty = if sn_list.is_empty() {
        format!(
            "WHERE Result = 'NG' AND testtype = '{}' ORDER BY TestDate DESC",
            testtype_param
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND Result = 'NG' AND testtype = '{}' ORDER BY TestDate DESC",
            sn_list, testtype_param
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_10, query_ty))
}

pub async fn build_query_sql_10g_with_time_and_testtype(
    sn_list: &str,
    start_time: &str,
    end_time: &str,
    testtype_param: &str,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Pf,Vop,Im,Rs,Se,Sen,Res,ICC,Vbr,Kink,imkink,TestDate,Idark,Result,ProductBill,iop,ixtalk,MDPId,testtype";
    let sql_10 = format!(
        "SELECT {0} FROM [BOSAautotest_Data].[dbo].[MAC_10GBOSADATA] ",
        testtype
    );
    let query_ty = if sn_list.is_empty() {
        format!(
            "WHERE Result = 'OK' AND TestDate BETWEEN '{}' AND '{}' AND testtype = '{}' ORDER BY TestDate DESC",
            start_time, end_time, testtype_param
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND Result = 'OK' AND TestDate BETWEEN '{}' AND '{}' AND testtype = '{}' ORDER BY TestDate DESC",
            sn_list, start_time, end_time, testtype_param
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_10, query_ty))
}

pub async fn build_query_sql_10g_res_all_with_time_and_testtype(
    sn_list: &str,
    start_time: &str,
    end_time: &str,
    testtype_param: &str,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Pf,Vop,Im,Rs,Se,Sen,Res,ICC,Vbr,Kink,imkink,TestDate,Idark,Result,ProductBill,iop,ixtalk,MDPId,testtype";
    let sql_10 = format!(
        "SELECT {0} FROM [BOSAautotest_Data].[dbo].[MAC_10GBOSADATA] ",
        testtype
    );
    let query_ty = if sn_list.is_empty() {
        format!(
            "WHERE TestDate BETWEEN '{}' AND '{}' AND testtype = '{}' ORDER BY TestDate DESC",
            start_time, end_time, testtype_param
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND TestDate BETWEEN '{}' AND '{}' AND testtype = '{}' ORDER BY TestDate DESC",
            sn_list, start_time, end_time, testtype_param
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_10, query_ty))
}

pub async fn build_query_sql_10g_res_ng_with_time_and_testtype(
    sn_list: &str,
    start_time: &str,
    end_time: &str,
    testtype_param: &str,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Pf,Vop,Im,Rs,Se,Sen,Res,ICC,Vbr,Kink,imkink,TestDate,Idark,Result,ProductBill,iop,ixtalk,MDPId,testtype";
    let sql_10 = format!(
        "SELECT {0} FROM [BOSAautotest_Data].[dbo].[MAC_10GBOSADATA] ",
        testtype
    );
    let query_ty = if sn_list.is_empty() {
        format!(
            "WHERE Result = 'NG' AND TestDate BETWEEN '{}' AND '{}' AND testtype = '{}' ORDER BY TestDate DESC",
            start_time, end_time, testtype_param
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND Result = 'NG' AND TestDate BETWEEN '{}' AND '{}' AND testtype = '{}' ORDER BY TestDate DESC",
            sn_list, start_time, end_time, testtype_param
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_10, query_ty))
}

pub async fn build_query_sql_10g_with_worker(
    sn_list: &str,
    worker: &str,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Pf,Vop,Im,Rs,Se,Sen,Res,ICC,Vbr,Kink,imkink,TestDate,Idark,Result,ProductBill,iop,ixtalk,MDPId,testtype";
    let sql_10 = format!(
        "SELECT {0} FROM [BOSAautotest_Data].[dbo].[MAC_10GBOSADATA] ",
        testtype
    );
    let query_ty = if sn_list.is_empty() {
        format!(
            "WHERE Result = 'OK' AND ProductBill = '{}' ORDER BY TestDate DESC",
            worker
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND Result = 'OK' AND ProductBill = '{}' ORDER BY TestDate DESC",
            sn_list, worker
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_10, query_ty))
}

pub async fn build_query_sql_10g_res_all_with_worker(
    sn_list: &str,
    worker: &str,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Pf,Vop,Im,Rs,Se,Sen,Res,ICC,Vbr,Kink,imkink,TestDate,Idark,Result,ProductBill,iop,ixtalk,MDPId,testtype";
    let sql_10 = format!(
        "SELECT {0} FROM [BOSAautotest_Data].[dbo].[MAC_10GBOSADATA] ",
        testtype
    );
    let query_ty = if sn_list.is_empty() {
        format!("WHERE ProductBill = '{}' ORDER BY TestDate DESC", worker)
    } else {
        format!(
            "WHERE SN IN ({}) AND ProductBill = '{}' ORDER BY TestDate DESC",
            sn_list, worker
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_10, query_ty))
}

pub async fn build_query_sql_10g_res_ng_with_worker(
    sn_list: &str,
    worker: &str,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Pf,Vop,Im,Rs,Se,Sen,Res,ICC,Vbr,Kink,imkink,TestDate,Idark,Result,ProductBill,iop,ixtalk,MDPId,testtype";
    let sql_10 = format!(
        "SELECT {0} FROM [BOSAautotest_Data].[dbo].[MAC_10GBOSADATA] ",
        testtype
    );
    let query_ty = if sn_list.is_empty() {
        format!(
            "WHERE Result = 'NG' AND ProductBill = '{}' ORDER BY TestDate DESC",
            worker
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND Result = 'NG' AND ProductBill = '{}' ORDER BY TestDate DESC",
            sn_list, worker
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_10, query_ty))
}

pub async fn build_query_sql_10g_with_time_and_worker(
    sn_list: &str,
    start_time: &str,
    end_time: &str,
    worker: &str,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Pf,Vop,Im,Rs,Se,Sen,Res,ICC,Vbr,Kink,imkink,TestDate,Idark,Result,ProductBill,iop,ixtalk,MDPId,testtype";
    let sql_10 = format!(
        "SELECT {0} FROM [BOSAautotest_Data].[dbo].[MAC_10GBOSADATA] ",
        testtype
    );
    let query_ty = if sn_list.is_empty() {
        format!(
            "WHERE Result = 'OK' AND TestDate BETWEEN '{}' AND '{}' AND ProductBill = '{}' ORDER BY TestDate DESC",
            start_time, end_time, worker
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND Result = 'OK' AND TestDate BETWEEN '{}' AND '{}' AND ProductBill = '{}' ORDER BY TestDate DESC",
            sn_list, start_time, end_time, worker
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_10, query_ty))
}

pub async fn build_query_sql_10g_res_all_with_time_and_worker(
    sn_list: &str,
    start_time: &str,
    end_time: &str,
    worker: &str,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Pf,Vop,Im,Rs,Se,Sen,Res,ICC,Vbr,Kink,imkink,TestDate,Idark,Result,ProductBill,iop,ixtalk,MDPId,testtype";
    let sql_10 = format!(
        "SELECT {0} FROM [BOSAautotest_Data].[dbo].[MAC_10GBOSADATA] ",
        testtype
    );
    let query_ty = if sn_list.is_empty() {
        format!(
            "WHERE TestDate BETWEEN '{}' AND '{}' AND ProductBill = '{}' ORDER BY TestDate DESC",
            start_time, end_time, worker
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND TestDate BETWEEN '{}' AND '{}' AND ProductBill = '{}' ORDER BY TestDate DESC",
            sn_list, start_time, end_time, worker
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_10, query_ty))
}

pub async fn build_query_sql_10g_res_ng_with_time_and_worker(
    sn_list: &str,
    start_time: &str,
    end_time: &str,
    worker: &str,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Pf,Vop,Im,Rs,Se,Sen,Res,ICC,Vbr,Kink,imkink,TestDate,Idark,Result,ProductBill,iop,ixtalk,MDPId,testtype";
    let sql_10 = format!(
        "SELECT {0} FROM [BOSAautotest_Data].[dbo].[MAC_10GBOSADATA] ",
        testtype
    );
    let query_ty = if sn_list.is_empty() {
        format!(
            "WHERE Result = 'NG' AND TestDate BETWEEN '{}' AND '{}' AND ProductBill = '{}' ORDER BY TestDate DESC",
            start_time, end_time, worker
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND Result = 'NG' AND TestDate BETWEEN '{}' AND '{}' AND ProductBill = '{}' ORDER BY TestDate DESC",
            sn_list, start_time, end_time, worker
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_10, query_ty))
}

pub async fn build_query_sql_10g_with_testtype_and_worker(
    sn_list: &str,
    testtype_param: &str,
    worker: &str,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Pf,Vop,Im,Rs,Se,Sen,Res,ICC,Vbr,Kink,imkink,TestDate,Idark,Result,ProductBill,iop,ixtalk,MDPId,testtype";
    let sql_10 = format!(
        "SELECT {0} FROM [BOSAautotest_Data].[dbo].[MAC_10GBOSADATA] ",
        testtype
    );
    let query_ty = if sn_list.is_empty() {
        format!(
            "WHERE Result = 'OK' AND testtype = '{}' AND ProductBill = '{}' ORDER BY TestDate DESC",
            testtype_param, worker
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND Result = 'OK' AND testtype = '{}' AND ProductBill = '{}' ORDER BY TestDate DESC",
            sn_list, testtype_param, worker
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_10, query_ty))
}

pub async fn build_query_sql_10g_res_all_with_testtype_and_worker(
    sn_list: &str,
    testtype_param: &str,
    worker: &str,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Pf,Vop,Im,Rs,Se,Sen,Res,ICC,Vbr,Kink,imkink,TestDate,Idark,Result,ProductBill,iop,ixtalk,MDPId,testtype";
    let sql_10 = format!(
        "SELECT {0} FROM [BOSAautotest_Data].[dbo].[MAC_10GBOSADATA] ",
        testtype
    );
    let query_ty = if sn_list.is_empty() {
        format!(
            "WHERE testtype = '{}' AND ProductBill = '{}' ORDER BY TestDate DESC",
            testtype_param, worker
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND testtype = '{}' AND ProductBill = '{}' ORDER BY TestDate DESC",
            sn_list, testtype_param, worker
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_10, query_ty))
}

pub async fn build_query_sql_10g_res_ng_with_testtype_and_worker(
    sn_list: &str,
    testtype_param: &str,
    worker: &str,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Pf,Vop,Im,Rs,Se,Sen,Res,ICC,Vbr,Kink,imkink,TestDate,Idark,Result,ProductBill,iop,ixtalk,MDPId,testtype";
    let sql_10 = format!(
        "SELECT {0} FROM [BOSAautotest_Data].[dbo].[MAC_10GBOSADATA] ",
        testtype
    );
    let query_ty = if sn_list.is_empty() {
        format!(
            "WHERE Result = 'NG' AND testtype = '{}' AND ProductBill = '{}' ORDER BY TestDate DESC",
            testtype_param, worker
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND Result = 'NG' AND testtype = '{}' AND ProductBill = '{}' ORDER BY TestDate DESC",
            sn_list, testtype_param, worker
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_10, query_ty))
}

pub async fn build_query_sql_10g_with_time_and_testtype_and_worker(
    sn_list: &str,
    start_time: &str,
    end_time: &str,
    testtype_param: &str,
    worker: &str,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Pf,Vop,Im,Rs,Se,Sen,Res,ICC,Vbr,Kink,imkink,TestDate,Idark,Result,ProductBill,iop,ixtalk,MDPId,testtype";
    let sql_10 = format!(
        "SELECT {0} FROM [BOSAautotest_Data].[dbo].[MAC_10GBOSADATA] ",
        testtype
    );
    let query_ty = if sn_list.is_empty() {
        format!(
            "WHERE Result = 'OK' AND TestDate BETWEEN '{}' AND '{}' AND testtype = '{}' AND ProductBill = '{}' ORDER BY TestDate DESC",
            start_time, end_time, testtype_param, worker
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND Result = 'OK' AND TestDate BETWEEN '{}' AND '{}' AND testtype = '{}' AND ProductBill = '{}' ORDER BY TestDate DESC",
            sn_list, start_time, end_time, testtype_param, worker
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_10, query_ty))
}

pub async fn build_query_sql_10g_res_all_with_time_and_testtype_and_worker(
    sn_list: &str,
    start_time: &str,
    end_time: &str,
    testtype_param: &str,
    worker: &str,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Pf,Vop,Im,Rs,Se,Sen,Res,ICC,Vbr,Kink,imkink,TestDate,Idark,Result,ProductBill,iop,ixtalk,MDPId,testtype";
    let sql_10 = format!(
        "SELECT {0} FROM [BOSAautotest_Data].[dbo].[MAC_10GBOSADATA] ",
        testtype
    );
    let query_ty = if sn_list.is_empty() {
        format!(
            "WHERE TestDate BETWEEN '{}' AND '{}' AND testtype = '{}' AND ProductBill = '{}' ORDER BY TestDate DESC",
            start_time, end_time, testtype_param, worker
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND TestDate BETWEEN '{}' AND '{}' AND testtype = '{}' AND ProductBill = '{}' ORDER BY TestDate DESC",
            sn_list, start_time, end_time, testtype_param, worker
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_10, query_ty))
}

pub async fn build_query_sql_10g_res_ng_with_time_and_testtype_and_worker(
    sn_list: &str,
    start_time: &str,
    end_time: &str,
    testtype_param: &str,
    worker: &str,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Pf,Vop,Im,Rs,Se,Sen,Res,ICC,Vbr,Kink,imkink,TestDate,Idark,Result,ProductBill,iop,ixtalk,MDPId,testtype";
    let sql_10 = format!(
        "SELECT {0} FROM [BOSAautotest_Data].[dbo].[MAC_10GBOSADATA] ",
        testtype
    );
    let query_ty = if sn_list.is_empty() {
        format!(
            "WHERE Result = 'NG' AND TestDate BETWEEN '{}' AND '{}' AND testtype = '{}' AND ProductBill = '{}' ORDER BY TestDate DESC",
            start_time, end_time, testtype_param, worker
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND Result = 'NG' AND TestDate BETWEEN '{}' AND '{}' AND testtype = '{}' AND ProductBill = '{}' ORDER BY TestDate DESC",
            sn_list, start_time, end_time, testtype_param, worker
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_10, query_ty))
}

pub async fn build_query_sql_2(
    sn_list: &str,
    pool: &MssqlPool,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Po,Vf,Im,Rs,Pslop,Sen,Res,ICC,Vbr,Kink_I,kinkim_i,TestDate,Idark,Result,ProductBill,io,xtalk,Te,testtype";
    let mut sql_text = String::new();
    let tables = get_tables(pool).await?;
    for i in tables {
        let s = format!("SELECT {} FROM {} ", testtype, i);
        sql_text.push_str(&s);
        sql_text.push_str(" UNION ALL ");
    }
    if sql_text.ends_with(" UNION ALL ") {
        sql_text.truncate(sql_text.len() - " UNION ALL ".len());
    }
    let query_ty = if sn_list.is_empty() {
        format!("WHERE Result = 'OK' ORDER BY TestDate DESC")
    } else {
        format!(
            "WHERE SN IN ({}) AND Result = 'OK' ORDER BY TestDate DESC",
            sn_list
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_text, query_ty))
}

pub async fn build_query_sql_2_res_all(
    sn_list: &str,
    pool: &MssqlPool,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Po,Vf,Im,Rs,Pslop,Sen,Res,ICC,Vbr,Kink_I,kinkim_i,TestDate,Idark,Result,ProductBill,io,xtalk,Te,testtype";
    let mut sql_text = String::new();
    let tables = get_tables(pool).await?;
    for i in tables {
        let s = format!("SELECT {} FROM {} ", testtype, i);
        sql_text.push_str(&s);
        sql_text.push_str(" UNION ALL ");
    }
    if sql_text.ends_with(" UNION ALL ") {
        sql_text.truncate(sql_text.len() - " UNION ALL ".len());
    }
    let query_ty = if sn_list.is_empty() {
        format!("ORDER BY TestDate DESC")
    } else {
        format!("WHERE SN IN ({}) ORDER BY TestDate DESC", sn_list)
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_text, query_ty))
}

pub async fn build_query_sql_2_res_ng(
    sn_list: &str,
    pool: &MssqlPool,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Po,Vf,Im,Rs,Pslop,Sen,Res,ICC,Vbr,Kink_I,kinkim_i,TestDate,Idark,Result,ProductBill,io,xtalk,Te,testtype";
    let mut sql_text = String::new();
    let tables = get_tables(pool).await?;
    for i in tables {
        let s = format!("SELECT {} FROM {} ", testtype, i);
        sql_text.push_str(&s);
        sql_text.push_str(" UNION ALL ");
    }
    if sql_text.ends_with(" UNION ALL ") {
        sql_text.truncate(sql_text.len() - " UNION ALL ".len());
    }
    let query_ty = if sn_list.is_empty() {
        format!("where Result = 'NG' ORDER BY TestDate DESC")
    } else {
        format!(
            "WHERE SN IN ({}) AND Result = 'NG' ORDER BY TestDate DESC",
            sn_list
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_text, query_ty))
}

pub async fn build_query_sql_2_with_time(
    sn_list: &str,
    start_time: &str,
    end_time: &str,
    pool: &MssqlPool,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Po,Vf,Im,Rs,Pslop,Sen,Res,ICC,Vbr,Kink_I,kinkim_i,TestDate,Idark,Result,ProductBill,io,xtalk,Te,testtype";
    let mut sql_text = String::new();
    let tables = get_tables(pool).await?;
    for i in tables {
        let s = format!("SELECT {} FROM {} ", testtype, i);
        sql_text.push_str(&s);
        sql_text.push_str(" UNION ALL ");
    }
    if sql_text.ends_with(" UNION ALL ") {
        sql_text.truncate(sql_text.len() - " UNION ALL ".len());
    }
    let query_ty = if sn_list.is_empty() {
        format!(
            "where Result = 'OK' AND TestDate BETWEEN '{}' AND '{}' ORDER BY TestDate DESC",
            start_time, end_time
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND Result = 'OK' AND TestDate BETWEEN '{}' AND '{}' ORDER BY TestDate DESC",
            sn_list, start_time, end_time
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_text, query_ty))
}

pub async fn build_query_sql_2_res_all_with_time(
    sn_list: &str,
    start_time: &str,
    end_time: &str,
    pool: &MssqlPool,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Po,Vf,Im,Rs,Pslop,Sen,Res,ICC,Vbr,Kink_I,kinkim_i,TestDate,Idark,Result,ProductBill,io,xtalk,Te,testtype";
    let mut sql_text = String::new();
    let tables = get_tables(pool).await?;
    for i in tables {
        let s = format!("SELECT {} FROM {} ", testtype, i);
        sql_text.push_str(&s);
        sql_text.push_str(" UNION ALL ");
    }
    if sql_text.ends_with(" UNION ALL ") {
        sql_text.truncate(sql_text.len() - " UNION ALL ".len());
    }
    let query_ty = if sn_list.is_empty() {
        format!(
            "where TestDate BETWEEN '{}' AND '{}' ORDER BY TestDate DESC",
            start_time, end_time
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND TestDate BETWEEN '{}' AND '{}' ORDER BY TestDate DESC",
            sn_list, start_time, end_time
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_text, query_ty))
}

pub async fn build_query_sql_2_res_ng_with_time(
    sn_list: &str,
    start_time: &str,
    end_time: &str,
    pool: &MssqlPool,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Po,Vf,Im,Rs,Pslop,Sen,Res,ICC,Vbr,Kink_I,kinkim_i,TestDate,Idark,Result,ProductBill,io,xtalk,Te,testtype";
    let mut sql_text = String::new();
    let tables = get_tables(pool).await?;
    for i in tables {
        let s = format!("SELECT {} FROM {} ", testtype, i);
        sql_text.push_str(&s);
        sql_text.push_str(" UNION ALL ");
    }
    if sql_text.ends_with(" UNION ALL ") {
        sql_text.truncate(sql_text.len() - " UNION ALL ".len());
    }
    let query_ty = if sn_list.is_empty() {
        format!(
            "where Result = 'NG' AND TestDate BETWEEN '{}' AND '{}' ORDER BY TestDate DESC",
            start_time, end_time
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND Result = 'NG' AND TestDate BETWEEN '{}' AND '{}' ORDER BY TestDate DESC",
            sn_list, start_time, end_time
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_text, query_ty))
}

pub async fn build_query_sql_2_with_testtype(
    sn_list: &str,
    testtype_param: &str,
    pool: &MssqlPool,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Po,Vf,Im,Rs,Pslop,Sen,Res,ICC,Vbr,Kink_I,kinkim_i,TestDate,Idark,Result,ProductBill,io,xtalk,Te,testtype";
    let mut sql_text = String::new();
    let tables = get_tables(pool).await?;
    for i in tables {
        let s = format!("SELECT {} FROM {} ", testtype, i);
        sql_text.push_str(&s);
        sql_text.push_str(" UNION ALL ");
    }
    if sql_text.ends_with(" UNION ALL ") {
        sql_text.truncate(sql_text.len() - " UNION ALL ".len());
    }
    let query_ty = if sn_list.is_empty() {
        format!(
            "where Result = 'OK' AND testtype = '{}' ORDER BY TestDate DESC",
            testtype_param
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND Result = 'OK' AND testtype = '{}' ORDER BY TestDate DESC",
            sn_list, testtype_param
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_text, query_ty))
}

pub async fn build_query_sql_2_res_all_with_testtype(
    sn_list: &str,
    testtype_param: &str,
    pool: &MssqlPool,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Po,Vf,Im,Rs,Pslop,Sen,Res,ICC,Vbr,Kink_I,kinkim_i,TestDate,Idark,Result,ProductBill,io,xtalk,Te,testtype";
    let mut sql_text = String::new();
    let tables = get_tables(pool).await?;
    for i in tables {
        let s = format!("SELECT {} FROM {} ", testtype, i);
        sql_text.push_str(&s);
        sql_text.push_str(" UNION ALL ");
    }
    if sql_text.ends_with(" UNION ALL ") {
        sql_text.truncate(sql_text.len() - " UNION ALL ".len());
    }
    let query_ty = if sn_list.is_empty() {
        format!(
            "where testtype = '{}' ORDER BY TestDate DESC",
            testtype_param
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND testtype = '{}' ORDER BY TestDate DESC",
            sn_list, testtype_param
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_text, query_ty))
}

pub async fn build_query_sql_2_res_ng_with_testtype(
    sn_list: &str,
    testtype_param: &str,
    pool: &MssqlPool,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Po,Vf,Im,Rs,Pslop,Sen,Res,ICC,Vbr,Kink_I,kinkim_i,TestDate,Idark,Result,ProductBill,io,xtalk,Te,testtype";
    let mut sql_text = String::new();
    let tables = get_tables(pool).await?;
    for i in tables {
        let s = format!("SELECT {} FROM {} ", testtype, i);
        sql_text.push_str(&s);
        sql_text.push_str(" UNION ALL ");
    }
    if sql_text.ends_with(" UNION ALL ") {
        sql_text.truncate(sql_text.len() - " UNION ALL ".len());
    }
    let query_ty = if sn_list.is_empty() {
        format!(
            "where Result = 'NG' AND testtype = '{}' ORDER BY TestDate DESC",
            testtype_param
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND Result = 'NG' AND testtype = '{}' ORDER BY TestDate DESC",
            sn_list, testtype_param
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_text, query_ty))
}

pub async fn build_query_sql_2_with_time_and_testtype(
    sn_list: &str,
    start_time: &str,
    end_time: &str,
    testtype_param: &str,
    pool: &MssqlPool,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Po,Vf,Im,Rs,Pslop,Sen,Res,ICC,Vbr,Kink_I,kinkim_i,TestDate,Idark,Result,ProductBill,io,xtalk,Te,testtype";
    let mut sql_text = String::new();
    let tables = get_tables(pool).await?;
    for i in tables {
        let s = format!("SELECT {} FROM {} ", testtype, i);
        sql_text.push_str(&s);
        sql_text.push_str(" UNION ALL ");
    }
    if sql_text.ends_with(" UNION ALL ") {
        sql_text.truncate(sql_text.len() - " UNION ALL ".len());
    }
    let query_ty = if sn_list.is_empty() {
        format!(
            "where  Result = 'OK' AND TestDate BETWEEN '{}' AND '{}' AND testtype = '{}' ORDER BY TestDate DESC",
            start_time, end_time, testtype_param
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND Result = 'OK' AND TestDate BETWEEN '{}' AND '{}' AND testtype = '{}' ORDER BY TestDate DESC",
            sn_list, start_time, end_time, testtype_param
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_text, query_ty))
}

pub async fn build_query_sql_2_res_all_with_time_and_testtype(
    sn_list: &str,
    start_time: &str,
    end_time: &str,
    testtype_param: &str,
    pool: &MssqlPool,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Po,Vf,Im,Rs,Pslop,Sen,Res,ICC,Vbr,Kink_I,kinkim_i,TestDate,Idark,Result,ProductBill,io,xtalk,Te,testtype";
    let mut sql_text = String::new();
    let tables = get_tables(pool).await?;
    for i in tables {
        let s = format!("SELECT {} FROM {} ", testtype, i);
        sql_text.push_str(&s);
        sql_text.push_str(" UNION ALL ");
    }
    if sql_text.ends_with(" UNION ALL ") {
        sql_text.truncate(sql_text.len() - " UNION ALL ".len());
    }
    let query_ty = if sn_list.is_empty() {
        format!(
            "where TestDate BETWEEN '{}' AND '{}' AND testtype = '{}' ORDER BY TestDate DESC",
            start_time, end_time, testtype_param
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND TestDate BETWEEN '{}' AND '{}' AND testtype = '{}' ORDER BY TestDate DESC",
            sn_list, start_time, end_time, testtype_param
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_text, query_ty))
}

pub async fn build_query_sql_2_res_ng_with_time_and_testtype(
    sn_list: &str,
    start_time: &str,
    end_time: &str,
    testtype_param: &str,
    pool: &MssqlPool,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Po,Vf,Im,Rs,Pslop,Sen,Res,ICC,Vbr,Kink_I,kinkim_i,TestDate,Idark,Result,ProductBill,io,xtalk,Te,testtype";
    let mut sql_text = String::new();
    let tables = get_tables(pool).await?;
    for i in tables {
        let s = format!("SELECT {} FROM {} ", testtype, i);
        sql_text.push_str(&s);
        sql_text.push_str(" UNION ALL ");
    }
    if sql_text.ends_with(" UNION ALL ") {
        sql_text.truncate(sql_text.len() - " UNION ALL ".len());
    }
    let query_ty = if sn_list.is_empty() {
        format!(
            "where Result = 'NG' AND  TestDate BETWEEN '{}' AND '{}' AND testtype = '{}' ORDER BY TestDate DESC",
            start_time, end_time, testtype_param
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND Result = 'NG' AND TestDate BETWEEN '{}' AND '{}' AND testtype = '{}' ORDER BY TestDate DESC",
            sn_list, start_time, end_time, testtype_param
        )
    };

    Ok(format!("SELECT * FROM ({}) tmp {}", sql_text, query_ty))
}

pub async fn build_query_sql_2_with_worker(
    sn_list: &str,
    worker: &str,
    pool: &MssqlPool,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Po,Vf,Im,Rs,Pslop,Sen,Res,ICC,Vbr,Kink_I,kinkim_i,TestDate,Idark,Result,ProductBill,io,xtalk,Te,testtype";
    let mut sql_text = String::new();
    let tables = get_tables(pool).await?;
    for i in tables {
        let s = format!("SELECT {} FROM {} ", testtype, i);
        sql_text.push_str(&s);
        sql_text.push_str(" UNION ALL ");
    }
    if sql_text.ends_with(" UNION ALL ") {
        sql_text.truncate(sql_text.len() - " UNION ALL ".len());
    }
    let query_ty = if sn_list.is_empty() {
        format!(
            "where Result = 'OK' AND ProductBill = '{}' ORDER BY TestDate DESC",
            worker
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND Result = 'OK' AND ProductBill = '{}' ORDER BY TestDate DESC",
            sn_list, worker
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_text, query_ty))
}

pub async fn build_query_sql_2_res_all_with_worker(
    sn_list: &str,
    worker: &str,
    pool: &MssqlPool,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Po,Vf,Im,Rs,Pslop,Sen,Res,ICC,Vbr,Kink_I,kinkim_i,TestDate,Idark,Result,ProductBill,io,xtalk,Te,testtype";
    let mut sql_text = String::new();
    let tables = get_tables(pool).await?;
    for i in tables {
        let s = format!("SELECT {} FROM {} ", testtype, i);
        sql_text.push_str(&s);
        sql_text.push_str(" UNION ALL ");
    }
    if sql_text.ends_with(" UNION ALL ") {
        sql_text.truncate(sql_text.len() - " UNION ALL ".len());
    }
    let query_ty = if sn_list.is_empty() {
        format!("where ProductBill = '{}' ORDER BY TestDate DESC", worker)
    } else {
        format!(
            "WHERE SN IN ({}) AND ProductBill = '{}' ORDER BY TestDate DESC",
            sn_list, worker
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_text, query_ty))
}

pub async fn build_query_sql_2_res_ng_with_worker(
    sn_list: &str,
    worker: &str,
    pool: &MssqlPool,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Po,Vf,Im,Rs,Pslop,Sen,Res,ICC,Vbr,Kink_I,kinkim_i,TestDate,Idark,Result,ProductBill,io,xtalk,Te,testtype";
    let mut sql_text = String::new();
    let tables = get_tables(pool).await?;
    for i in tables {
        let s = format!("SELECT {} FROM {} ", testtype, i);
        sql_text.push_str(&s);
        sql_text.push_str(" UNION ALL ");
    }
    if sql_text.ends_with(" UNION ALL ") {
        sql_text.truncate(sql_text.len() - " UNION ALL ".len());
    }
    let query_ty = if sn_list.is_empty() {
        format!(
            "where Result = 'NG' AND ProductBill = '{}' ORDER BY TestDate DESC",
            worker
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND Result = 'NG' AND ProductBill = '{}' ORDER BY TestDate DESC",
            sn_list, worker
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_text, query_ty))
}

pub async fn build_query_sql_2_with_time_and_worker(
    sn_list: &str,
    start_time: &str,
    end_time: &str,
    worker: &str,
    pool: &MssqlPool,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Po,Vf,Im,Rs,Pslop,Sen,Res,ICC,Vbr,Kink_I,kinkim_i,TestDate,Idark,Result,ProductBill,io,xtalk,Te,testtype";
    let mut sql_text = String::new();
    let tables = get_tables(pool).await?;
    for i in tables {
        let s = format!("SELECT {} FROM {} ", testtype, i);
        sql_text.push_str(&s);
        sql_text.push_str(" UNION ALL ");
    }
    if sql_text.ends_with(" UNION ALL ") {
        sql_text.truncate(sql_text.len() - " UNION ALL ".len());
    }
    let query_ty = if sn_list.is_empty() {
        format!(
            "where Result = 'OK' AND TestDate BETWEEN '{}' AND '{}' AND ProductBill = '{}' ORDER BY TestDate DESC",
            start_time, end_time, worker
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND Result = 'OK' AND TestDate BETWEEN '{}' AND '{}' AND ProductBill = '{}' ORDER BY TestDate DESC",
            sn_list, start_time, end_time, worker
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_text, query_ty))
}

pub async fn build_query_sql_2_res_all_with_time_and_worker(
    sn_list: &str,
    start_time: &str,
    end_time: &str,
    worker: &str,
    pool: &MssqlPool,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Po,Vf,Im,Rs,Pslop,Sen,Res,ICC,Vbr,Kink_I,kinkim_i,TestDate,Idark,Result,ProductBill,io,xtalk,Te,testtype";
    let mut sql_text = String::new();
    let tables = get_tables(pool).await?;
    for i in tables {
        let s = format!("SELECT {} FROM {} ", testtype, i);
        sql_text.push_str(&s);
        sql_text.push_str(" UNION ALL ");
    }
    if sql_text.ends_with(" UNION ALL ") {
        sql_text.truncate(sql_text.len() - " UNION ALL ".len());
    }
    let query_ty = if sn_list.is_empty() {
        format!(
            "where TestDate BETWEEN '{}' AND '{}' AND ProductBill = '{}' ORDER BY TestDate DESC",
            start_time, end_time, worker
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND TestDate BETWEEN '{}' AND '{}' AND ProductBill = '{}' ORDER BY TestDate DESC",
            sn_list, start_time, end_time, worker
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_text, query_ty))
}

pub async fn build_query_sql_2_res_ng_with_time_and_worker(
    sn_list: &str,
    start_time: &str,
    end_time: &str,
    worker: &str,
    pool: &MssqlPool,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Po,Vf,Im,Rs,Pslop,Sen,Res,ICC,Vbr,Kink_I,kinkim_i,TestDate,Idark,Result,ProductBill,io,xtalk,Te,testtype";
    let mut sql_text = String::new();
    let tables = get_tables(pool).await?;
    for i in tables {
        let s = format!("SELECT {} FROM {} ", testtype, i);
        sql_text.push_str(&s);
        sql_text.push_str(" UNION ALL ");
    }
    if sql_text.ends_with(" UNION ALL ") {
        sql_text.truncate(sql_text.len() - " UNION ALL ".len());
    }
    let query_ty = if sn_list.is_empty() {
        format!(
            "where Result = 'NG' AND TestDate BETWEEN '{}' AND '{}' AND ProductBill = '{}' ORDER BY TestDate DESC",
            start_time, end_time, worker
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND Result = 'NG' AND TestDate BETWEEN '{}' AND '{}' AND ProductBill = '{}' ORDER BY TestDate DESC",
            sn_list, start_time, end_time, worker
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_text, query_ty))
}

pub async fn build_query_sql_2_with_testtype_and_worker(
    sn_list: &str,
    testtype_param: &str,
    worker: &str,
    pool: &MssqlPool,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Po,Vf,Im,Rs,Pslop,Sen,Res,ICC,Vbr,Kink_I,kinkim_i,TestDate,Idark,Result,ProductBill,io,xtalk,Te,testtype";
    let mut sql_text = String::new();
    let tables = get_tables(pool).await?;
    for i in tables {
        let s = format!("SELECT {} FROM {} ", testtype, i);
        sql_text.push_str(&s);
        sql_text.push_str(" UNION ALL ");
    }
    if sql_text.ends_with(" UNION ALL ") {
        sql_text.truncate(sql_text.len() - " UNION ALL ".len());
    }
    let query_ty = if sn_list.is_empty() {
        format!(
            "where Result = 'OK' AND testtype = '{}' AND ProductBill = '{}' ORDER BY TestDate DESC",
            testtype_param, worker
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND Result = 'OK' AND testtype = '{}' AND ProductBill = '{}' ORDER BY TestDate DESC",
            sn_list, testtype_param, worker
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_text, query_ty))
}

pub async fn build_query_sql_2_res_all_with_testtype_and_worker(
    sn_list: &str,
    testtype_param: &str,
    worker: &str,
    pool: &MssqlPool,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Po,Vf,Im,Rs,Pslop,Sen,Res,ICC,Vbr,Kink_I,kinkim_i,TestDate,Idark,Result,ProductBill,io,xtalk,Te,testtype";
    let mut sql_text = String::new();
    let tables = get_tables(pool).await?;
    for i in tables {
        let s = format!("SELECT {} FROM {} ", testtype, i);
        sql_text.push_str(&s);
        sql_text.push_str(" UNION ALL ");
    }
    if sql_text.ends_with(" UNION ALL ") {
        sql_text.truncate(sql_text.len() - " UNION ALL ".len());
    }
    let query_ty = if sn_list.is_empty() {
        format!(
            "where testtype = '{}' AND ProductBill = '{}' ORDER BY TestDate DESC",
            testtype_param, worker
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND testtype = '{}' AND ProductBill = '{}' ORDER BY TestDate DESC",
            sn_list, testtype_param, worker
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_text, query_ty))
}

pub async fn build_query_sql_2_res_ng_with_testtype_and_worker(
    sn_list: &str,
    testtype_param: &str,
    worker: &str,
    pool: &MssqlPool,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Po,Vf,Im,Rs,Pslop,Sen,Res,ICC,Vbr,Kink_I,kinkim_i,TestDate,Idark,Result,ProductBill,io,xtalk,Te,testtype";
    let mut sql_text = String::new();
    let tables = get_tables(pool).await?;
    for i in tables {
        let s = format!("SELECT {} FROM {} ", testtype, i);
        sql_text.push_str(&s);
        sql_text.push_str(" UNION ALL ");
    }
    if sql_text.ends_with(" UNION ALL ") {
        sql_text.truncate(sql_text.len() - " UNION ALL ".len());
    }
    let query_ty = if sn_list.is_empty() {
        format!(
            "where Result = 'NG' AND testtype = '{}' AND ProductBill = '{}' ORDER BY TestDate DESC",
            testtype_param, worker
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND Result = 'NG' AND testtype = '{}' AND ProductBill = '{}' ORDER BY TestDate DESC",
            sn_list, testtype_param, worker
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_text, query_ty))
}

pub async fn build_query_sql_2_with_time_and_testtype_and_worker(
    sn_list: &str,
    start_time: &str,
    end_time: &str,
    testtype_param: &str,
    worker: &str,
    pool: &MssqlPool,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Po,Vf,Im,Rs,Pslop,Sen,Res,ICC,Vbr,Kink_I,kinkim_i,TestDate,Idark,Result,ProductBill,io,xtalk,Te,testtype";
    let mut sql_text = String::new();
    let tables = get_tables(pool).await?;
    for i in tables {
        let s = format!("SELECT {} FROM {} ", testtype, i);
        sql_text.push_str(&s);
        sql_text.push_str(" UNION ALL ");
    }
    if sql_text.ends_with(" UNION ALL ") {
        sql_text.truncate(sql_text.len() - " UNION ALL ".len());
    }
    let query_ty = if sn_list.is_empty() {
        format!(
            "where Result = 'OK' AND TestDate BETWEEN '{}' AND '{}' AND testtype = '{}' AND ProductBill = '{}' ORDER BY TestDate DESC",
            start_time, end_time, testtype_param, worker
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND Result = 'OK' AND TestDate BETWEEN '{}' AND '{}' AND testtype = '{}' AND ProductBill = '{}' ORDER BY TestDate DESC",
            sn_list, start_time, end_time, testtype_param, worker
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_text, query_ty))
}

pub async fn build_query_sql_2_res_all_with_time_and_testtype_and_worker(
    sn_list: &str,
    start_time: &str,
    end_time: &str,
    testtype_param: &str,
    worker: &str,
    pool: &MssqlPool,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Po,Vf,Im,Rs,Pslop,Sen,Res,ICC,Vbr,Kink_I,kinkim_i,TestDate,Idark,Result,ProductBill,io,xtalk,Te,testtype";
    let mut sql_text = String::new();
    let tables = get_tables(pool).await?;
    for i in tables {
        let s = format!("SELECT {} FROM {} ", testtype, i);
        sql_text.push_str(&s);
        sql_text.push_str(" UNION ALL ");
    }
    if sql_text.ends_with(" UNION ALL ") {
        sql_text.truncate(sql_text.len() - " UNION ALL ".len());
    }
    let query_ty = if sn_list.is_empty() {
        format!(
            "where TestDate BETWEEN '{}' AND '{}' AND testtype = '{}' AND ProductBill = '{}' ORDER BY TestDate DESC",
            start_time, end_time, testtype_param, worker
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND TestDate BETWEEN '{}' AND '{}' AND testtype = '{}' AND ProductBill = '{}' ORDER BY TestDate DESC",
            sn_list, start_time, end_time, testtype_param, worker
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_text, query_ty))
}

pub async fn build_query_sql_2_res_ng_with_time_and_testtype_and_worker(
    sn_list: &str,
    start_time: &str,
    end_time: &str,
    testtype_param: &str,
    worker: &str,
    pool: &MssqlPool,
) -> anyhow::Result<String, MyError> {
    let testtype = "SN,Ith,Po,Vf,Im,Rs,Pslop,Sen,Res,ICC,Vbr,Kink_I,kinkim_i,TestDate,Idark,Result,ProductBill,io,xtalk,Te,testtype";
    let mut sql_text = String::new();
    let tables = get_tables(pool).await?;
    for i in tables {
        let s = format!("SELECT {} FROM {} ", testtype, i);
        sql_text.push_str(&s);
        sql_text.push_str(" UNION ALL ");
    }
    if sql_text.ends_with(" UNION ALL ") {
        sql_text.truncate(sql_text.len() - " UNION ALL ".len());
    }
    let query_ty = if sn_list.is_empty() {
        format!(
            "where Result = 'NG' AND TestDate BETWEEN '{}' AND '{}' AND testtype = '{}' AND ProductBill = '{}' ORDER BY TestDate DESC",
            start_time, end_time, testtype_param, worker
        )
    } else {
        format!(
            "WHERE SN IN ({}) AND Result = 'NG' AND TestDate BETWEEN '{}' AND '{}' AND testtype = '{}' AND ProductBill = '{}' ORDER BY TestDate DESC",
            sn_list, start_time, end_time, testtype_param, worker
        )
    };
    Ok(format!("SELECT * FROM ({}) tmp {}", sql_text, query_ty))
}
