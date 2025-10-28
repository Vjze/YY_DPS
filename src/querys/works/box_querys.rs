  use std::collections::{HashMap, HashSet};
  use sqlx_oldapi::{MssqlPool, query_as, Error as SqlxError, Row, query::QueryAs};
  use sqlx_oldapi::mssql::{MssqlRow, MssqlArguments};
  use futures::TryStreamExt;
  use chrono::NaiveDateTime;
  use crate::{structs::{Data, Datas, PackData}, utils::{error::MyError, sql::client}};
  
  #[derive(Debug)]
  struct QueryRow {
      pub sn: String,
      pub box_no: String, 
      pub yypn: String,   
      pub pack_worker: String, 
      pub create_time: NaiveDateTime, 
  }
  
  // 自动或手动实现 FromRow (此处假设使用自动实现，要求 SQL 字段名匹配)
  // 如果字段名不匹配（如Pack_no vs box_no），需要手动实现 FromRow 或在SQL中加AS别名
  impl sqlx_oldapi::FromRow<'_, MssqlRow> for QueryRow {
      // 假设旧 API 使用 from_row 签名
      fn from_row(row: &MssqlRow) -> Result<Self, SqlxError> {
          Ok(QueryRow {
              sn: row.try_get("sn")?,
              box_no: row.try_get("Pack_no")?,     // 假设 SQL 没有 AS 别名，使用原始列名
              yypn: row.try_get("pn")?,
              pack_worker: row.try_get("creator")?,
              create_time: row.try_get("createtime")?,
          })
      }
  }
  
  fn format_data(all_datas: Vec<Datas>) -> Vec<HashMap<String, String>> {
      let all = all_datas
          .into_iter()
          .map(|d| {
              // 展平 Datas 为 HashMap
              let mut map = HashMap::new();
              // PackData
              map.insert("box_no".to_string(), d.pack_data.box_no);
              map.insert("pack_worker".to_string(), d.pack_data.pack_worker);
              map.insert("pack_packtime".to_string(), d.pack_data.pack_packtime);
              // Data
              map.insert("sn".to_string(), d.sn_data.sn);
              map.insert("yypn".to_string(), d.sn_data.yypn);
              map
          })
          .collect::<Vec<HashMap<String, String>>>();
      all
  }
  pub async fn get_box_datas(
      box_no: String,
      use_time: bool,
      date_time_start: String,
      date_time_end: String,
      pn: String,
  ) -> anyhow::Result<Vec<HashMap<String, String>>, MyError> {
      
      // 假设 client().await? 返回 sqlx_oldapi::MssqlPool
      let pool: &MssqlPool = &client().await?;
      
      // 1. 预检查
      if box_no.is_empty() && pn.is_empty() && date_time_start.is_empty() && date_time_end.is_empty() && !use_time {
          return Err(MyError::Zdyknown("所有条件不能为空".to_string()));
      }
      
      // 2. 克隆输入参数用于绑定
      let box_no_cloned = box_no.clone();
      let pn_cloned = pn.clone();
      let date_time_start_cloned = date_time_start.clone();
      let date_time_end_cloned = date_time_end.clone();
      
      // 3. 核心逻辑：硬编码 if/else 分支来创建类型安全的查询对象
      let final_query: QueryAs<'_, sqlx_oldapi::Mssql, QueryRow, MssqlArguments>;
      
      // A. 仅按 box_no 查 (无 pn, 无时间)
      if !box_no_cloned.is_empty() && pn_cloned.is_empty() && !use_time {
          let sql = "SELECT sn, Pack_no, pn, creator, createtime FROM [mes_Factory].[dbo].[MaterialPackSn] WHERE Pack_no = @P1 AND PnOptionID = '-100' ORDER BY CreateTime DESC";
          final_query = query_as(sql).bind(&box_no_cloned);
      // B. 按 box_no 和 pn 查 (无时间)
      } else if !box_no_cloned.is_empty() && !pn_cloned.is_empty() && !use_time {
          let sql = "SELECT sn, Pack_no, pn, creator, createtime FROM [mes_Factory].[dbo].[MaterialPackSn] WHERE Pack_no = @P1 AND PnOptionID = '-100' AND pn = @P2 ORDER BY CreateTime DESC";
          final_query = query_as(sql).bind(&box_no_cloned).bind(&pn_cloned);
      // C. 按 box_no 和时间查 (无 pn)
      } else if !box_no_cloned.is_empty() && pn_cloned.is_empty() && use_time {
          let sql = "SELECT sn, Pack_no, pn, creator, createtime FROM [mes_Factory].[dbo].[MaterialPackSn] WHERE Pack_no = @P1 AND PnOptionID = '-100' AND createtime BETWEEN @P2 AND @P3 ORDER BY CreateTime DESC";
          final_query = query_as(sql).bind(&box_no_cloned).bind(&date_time_start_cloned).bind(&date_time_end_cloned);
      // D. 仅按 pn 和时间查 (无 box_no)
      } else if box_no_cloned.is_empty() && !pn_cloned.is_empty() && use_time {
          let sql = "SELECT sn, Pack_no, pn, creator, createtime FROM [mes_Factory].[dbo].[MaterialPackSn] WHERE pn = @P1 AND PnOptionID = '-100' AND createtime BETWEEN @P2 AND @P3 ORDER BY CreateTime DESC";
          final_query = query_as(sql).bind(&pn_cloned).bind(&date_time_start_cloned).bind(&date_time_end_cloned);
      // E. 按 box_no, pn, 和时间查 (所有条件)
      } else if !box_no_cloned.is_empty() && !pn_cloned.is_empty() && use_time {
          let sql = "SELECT sn, Pack_no, pn, creator, createtime FROM [mes_Factory].[dbo].[MaterialPackSn] WHERE Pack_no = @P1 AND pn = @P2 AND PnOptionID = '-100' AND createtime BETWEEN @P3 AND @P4 ORDER BY CreateTime DESC";
          final_query = query_as(sql).bind(&box_no_cloned).bind(&pn_cloned).bind(&date_time_start_cloned).bind(&date_time_end_cloned);
      // F. 仅按时间查 (无 box_no, 无 pn)
      } else if box_no_cloned.is_empty() && pn_cloned.is_empty() && use_time {
          let sql = "SELECT sn, Pack_no, pn, creator, createtime FROM [mes_Factory].[dbo].[MaterialPackSn] WHERE PnOptionID = '-100' AND createtime BETWEEN @P1 AND @P2 ORDER BY CreateTime DESC";
          final_query = query_as(sql).bind(&date_time_start_cloned).bind(&date_time_end_cloned);
      // G. 仅按 pn 查 (无 box_no, 无时间)
      } else if box_no_cloned.is_empty() && !pn_cloned.is_empty() && !use_time {
          let sql = "SELECT sn, Pack_no, pn, creator, createtime FROM [mes_Factory].[dbo].[MaterialPackSn] WHERE pn = @P1 AND PnOptionID = '-100' ORDER BY CreateTime DESC";
          final_query = query_as(sql).bind(&pn_cloned);
      } else {
          // 理论上被函数开头的检查阻止。如果逻辑运行到这里，可能是所有条件为空，但 use_time 为 false。
          // 为了避免编译器报错，提供一个默认的、可能返回空结果的查询。
          let sql = "SELECT sn, Pack_no, pn, creator, createtime FROM [mes_Factory].[dbo].[MaterialPackSn] WHERE PnOptionID = '-100' AND 1=0 ORDER BY CreateTime DESC";
          final_query = query_as(sql);
      }
      
      // 4. 执行查询
      let mut rows = final_query.fetch(pool);
      
      // 5. 处理结果流 (使用 QueryRow 结构体)
      let mut all_datas = Vec::new();
      let mut seen_sns = HashSet::new();
  
      while let Some(row) = rows.try_next().await.map_err(MyError::from)? {
          let sn = row.sn.clone();
          
          // 业务去重逻辑
          if seen_sns.contains(&sn) {
              continue;
          }
  
          let pack_data = PackData {
              box_no: row.box_no,
              pack_worker: row.pack_worker,
              pack_packtime: row.create_time.format("%Y-%m-%d %H:%M:%S").to_string(),
          };
          let sn_data = Data {
              sn: sn.clone(),
              yypn: row.yypn,
              ..Default::default()
          };
  
          let data = Datas {
              pack_data,
              sn_data,
              ..Default::default()
          };
          all_datas.push(data);
          seen_sns.insert(sn);
      }
  
      if all_datas.is_empty() {
          // 构造友好的错误信息
          let query_key = match (box_no.is_empty(), pn.is_empty()) {
              (false, _) => &box_no_cloned, // box_no 不为空
              (_, false) => &pn_cloned,     // box_no 为空，pn 不为空
              (true, true) => "时间范围", // 只有时间条件
          };
          return Err(MyError::Zdyknown(format!("查询条件 '{}' 没有找到数据", query_key)));
      }
      
      // 6. 格式化并返回
      let datas = format_data(all_datas);
      Ok(datas)
  }