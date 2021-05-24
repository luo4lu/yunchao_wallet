use crate::config;
use crate::authen_info::{personal_cartification, enterprise_cartification};
use crate::response::ResponseBody;
use actix_web::{post, web, HttpResponse, Responder};
use actix::spawn;
//use tokio::task;
use log::{warn};
use serde::{Deserialize, Serialize};
use mysql_async::{ params, Pool, Row};
use mysql_async::prelude::Queryable;
use chrono::prelude::*;
use chrono::{DateTime};
use uuid::v1::{Context, Timestamp};
use uuid::Uuid;
use std::fs::File;
use std::io::BufReader;

#[derive(Deserialize, PartialEq, Eq, Debug, Clone)]
 pub struct Payment{
	id: String,
    bus_type: String, // 单位类型
    bus_name: String, //企业名称
    credit_code: String, //社会信用代码
    validity_begin: Option<i64>,//有效期开始时间
    validity_end: i64,//有效期开始时间 0表示长期
    reg_addr: String, //企业注册地址
    bus_addr: String, //企业经营地址
    bus_info: String, //企业经营范围
    reg_capital: Option<i64>,//注册资金
    linkman: String, //企业联系人姓名
    telephone: String, //企业联系人电话
    link_email: String, //企业联系人邮箱
    account_note1: Option<String>,//账户备注1
    account_note2: Option<String>,//账户备注1
    account_note3: Option<String>,//账户备注1
    bus_connect: String, //企业关系(base、sub、branch、part、shop)
	person_info: PersonInfo, //填写人信息
    bank_info: BankInfo //公司银行账户信息
 }
#[derive(Deserialize, Serialize, Debug, Clone)]
 pub struct BusinessRequest{
    bus_type: String, // 单位类型
    bus_name: String, //企业名称
    credit_code: String, //社会信用代码
    business: String, //营业执照
    validity_begin: Option<i64>,//有效期开始时间
    validity_end: i64,//有效期开始时间 0表示长期
    reg_addr: String, //企业注册地址
    bus_addr: String, //企业经营地址
    bus_info: String, //企业经营范围
    reg_capital: Option<i64>,//注册资金
    linkman: String, //企业联系人姓名
    telephone: String, //企业联系人电话
    link_email: String, //企业联系人邮箱
    account_note1: Option<String>,//账户备注1
    account_note2: Option<String>,//账户备注1
    account_note3: Option<String>,//账户备注1
    attached: Option<String>,//附件,base64格式字符串。
    bus_connect: String, //企业关系(base、sub、branch、part、shop)
    person_info: PersonInfo, //填写人信息
    bank_info: BankInfo //公司银行账户信息
 }
 
 //填写人信息
 #[derive(Deserialize, Serialize,PartialEq, Eq, Debug, Clone)]
 pub struct PersonInfo{
    identity: String,// 填写人身份（legal、agency)
    legal_name: String,//法人姓名
    legal_voucher_type: String, //证件类型
    legal_voucher_num: String, //证件号码
    legal_photo_p: String, //身份证正面（base64字符串）
    legal_photo_r: Option<String>, //份证反面（base64字符串）
    legal_validity_begin: Option<i64>,
    legal_validity_end: i64, //长期为0
    legal_phone: String, //法人手机号码
    control_preson: String, //实际控制人（legal、other)
    agency_name: Option<String>, //代理人姓名
    agency_voucher_type: Option<String>, //代理人证件类型
    agency_voucher_num: Option<String>, //代理人证件号码
    agency_photo_p: Option<String>, //代理人身份证正面（base64字符串）
    agency_photo_r: Option<String>, //代理人份证反面（base64字符串）
    agency_validity_begin: Option<i64>, //代理人有效期开始时间
    agency_validity_end: Option<i64>, //代理人有效期结束时间（填0为长期）
    agency_phone: Option<String>, //代理人手机号码
    authorization: Option<String> //代理人委托书（base64字符串）
 }
 //开户银行信息
 #[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Clone)]
 pub struct BankInfo{
    account_type: String,//银行卡账户类型（company、private)
    account_name: String, //银行账户名称
    account_number: String, //银行账号
    deposit_bank: String, //开户银行
    area: String, //开户行所在地
    sub_branch: String //支行名称
 }
 #[derive(Deserialize, Serialize, Debug)]
 pub struct UserInfoResponse{
    pub appid: String, // 用户应用id
    pub web_url: String
 }

#[post("/register/business")]
 pub async fn user_info_save(
     req: web::Json<BusinessRequest>
 )-> impl Responder {
    //连接数据库
    let pool: Pool = config::get_db();
    let mut conn = pool.get_conn().await.unwrap();
    let sql_select = format!("select id from business where credit_code=\'{}\'",&req.credit_code);
    let row: Vec<Row> = conn.query(sql_select).await.unwrap();
    if !row.is_empty(){
      warn!("enterprise info already exist!!");
      return HttpResponse::Ok().json(ResponseBody::<()>::enterprise_registed());
    }
    let file = File::open("./config/config_info.json").unwrap();
    let reader = BufReader::new(file);
    let value: serde_json::Value = serde_json::from_reader(reader).unwrap();
    let verify: bool = value["verify"].as_bool().unwrap();
    let local: DateTime<Local> = Local::now();
    let context = Context::new(42);
    let ts = Timestamp::from_unix(&context, local.second() as u64, local.nanosecond());
    let uuid = Uuid::new_v1(
        ts,
        &[
            'r' as u8, 'e' as u8, 'g' as u8, 'i' as u8, 's' as u8, 't' as u8,
        ],
    ).expect("failed to generate UUID");
    let uuid = uuid.to_string();
    //企业信息认证
    if verify {
      let enterprise_status = enterprise_cartification(&req.person_info.legal_name, &req.credit_code).await;
      if enterprise_status != 0{
         warn!("enterprise cartification failed,error code = {}",enterprise_status);
      //释放连接
         drop(conn);
         pool.disconnect().await.unwrap();
         return HttpResponse::Ok().json(ResponseBody::<()>::enterprise_info_error());
      }
      //个人信息认证
      let personal_status = personal_cartification(&req.person_info.legal_name, &req.person_info.legal_phone, &req.person_info.legal_voucher_num).await;
      if personal_status != 1{
         warn!("personal info cartification failed,error code = {}",personal_status);
      //释放连接
      drop(conn);
      pool.disconnect().await.unwrap();
         return HttpResponse::Ok().json(ResponseBody::<()>::personal_info_error());
      }
      //如果代理人填写，验证代理人身份
      if req.person_info.identity == String::from("agency"){
         if req.person_info.agency_name.is_some() && req.person_info.agency_phone.is_some() 
         && req.person_info.agency_voucher_num.is_some(){
            let agency_name: &String = &req.person_info.agency_name.clone().unwrap();
            let agency_phone: &String = &req.person_info.agency_phone.clone().unwrap();
            let agency_voucher_num: &String = &req.person_info.agency_voucher_num.clone().unwrap();
            let agency_status = personal_cartification(agency_name, agency_phone, agency_voucher_num).await;
            if agency_status != 1{
               warn!("personal info cartification failed,error code = {}",personal_status);
            //释放连接
            drop(conn);
            pool.disconnect().await.unwrap();
               return HttpResponse::Ok().json(ResponseBody::<()>::agency_info_error());
            }
         }else{
            warn!("agency personal request info incomplete!");
         //释放连接
         drop(conn);
         pool.disconnect().await.unwrap();
            return HttpResponse::Ok().json(ResponseBody::<()>::agency_info_error());
         }
      }
   }
   if req.person_info.identity == String::from("agency"){
      if !req.person_info.agency_name.is_some() || !req.person_info.agency_phone.is_some() || !req.person_info.agency_voucher_num.is_some()
      || !req.person_info.agency_voucher_type.is_some() || !req.person_info.agency_photo_p.is_some()
      || !req.person_info.agency_validity_end.is_some() || !req.person_info.authorization.is_some(){
         warn!("agency info missing,please input agency infomation");
         return HttpResponse::Ok().json(ResponseBody::<()>::agency_info_miss());
      }
   }
    let email_server: String = value["email_server"].as_str().unwrap().to_string();
    let server_addr = "http://".to_string() + &email_server + "/user/consult";
    let params: BusinessRequest = BusinessRequest{
      bus_type: req.bus_type.clone(), // 单位类型
      bus_name: req.bus_name.clone(), //企业名称
      credit_code: req.credit_code.clone(), //社会信用代码
      business: req.business.clone(), //营业执照
      validity_begin: req.validity_begin.clone(),//有效期开始时间
      validity_end: req.validity_end.clone(),//有效期开始时间 0表示长期
      reg_addr: req.reg_addr.clone(), //企业注册地址
      bus_addr: req.bus_addr.clone(), //企业经营地址
      bus_info: req.bus_info.clone(), //企业经营范围
      reg_capital: req.reg_capital.clone(),//注册资金
      linkman: req.linkman.clone(), //企业联系人姓名
      telephone: req.telephone.clone(), //企业联系人电话
      link_email: req.link_email.clone(), //企业联系人邮箱
      account_note1: req.account_note1.clone(),//账户备注1
      account_note2: req.account_note2.clone(),//账户备注1
      account_note3: req.account_note3.clone(),//账户备注1
      attached: req.attached.clone(),//附件,base64格式字符串。
      bus_connect: req.bus_connect.clone(), //企业关系(base、sub、branch、part、shop)
      person_info: req.person_info.clone(), //填写人信息
      bank_info: req.bank_info.clone() //公司银行账户信息
    };
    
    let tmp_params = params.clone();
    let _join_hand = spawn( async move {
      let http_client = reqwest::Client::new();
      match http_client.post(&server_addr)
      .json(&tmp_params)
      .send().await{
         Ok(_) => {},
         Err(error)=>{
            warn!("reqwest email server failed:{}",error.to_string());
            return 
            //return HttpResponse::Ok().json(ResponseBody::<String>::return_unwrap_error(error.to_string()));
         }
      };
   });
   //	let code_status = http_res.status().as_u16();
	/*if code_status != 200 {
		warn!("user data process and send eamil failed");
		//释放连接
		drop(conn);
		pool.disconnect().await.unwrap();
		return HttpResponse::Ok().json(ResponseBody::<()>::user_data_error());
	}
	let res_value: serde_json::Value = match http_res.json().await{
		Ok(v) => v,
		Err(error)=>{
			warn!("reqwest data analysis failed:{}",error.to_string());
			//释放连接
			drop(conn);
			pool.disconnect().await.unwrap();
            return HttpResponse::Ok().json(ResponseBody::<String>::return_unwrap_error(error.to_string()));
		}
	};
	let return_code = res_value["code"].as_i64().unwrap();
	if return_code != 0 {
		warn!("email server send eamil failed");
		//释放连接
		drop(conn);
		pool.disconnect().await.unwrap();
		return HttpResponse::Ok().json(ResponseBody::<()>::user_data_error());
	}*/
	
	let payments = vec![Payment{id:uuid, bus_type: req.bus_type.clone(), bus_name: req.bus_name.clone(),
		credit_code: req.credit_code.clone(), validity_begin: req.validity_begin, 
		validity_end: req.validity_end, reg_addr: req.reg_addr.clone(), bus_addr: req.bus_addr.clone(), bus_info: req.bus_info.clone(), 
		reg_capital: req.reg_capital, linkman: req.linkman.clone(), telephone: req.telephone.clone(), link_email: req.link_email.clone(),
		account_note1: req.account_note1.clone(), account_note2: req.account_note2.clone(), account_note3: req.account_note3.clone(),
		bus_connect: req.bus_connect.clone(), person_info: req.person_info.clone(), bank_info: req.bank_info.clone()}];
    let params_sql = payments.clone().into_iter().map(|payment| {
        params! {
            "id" => payment.id,"bus_type" => payment.bus_type,
			"bus_name" => payment.bus_name, "credit_code" => payment.credit_code,
			"validity_begin" => payment.validity_begin, "validity_end" => payment.validity_end, "reg_addr"=>payment.reg_addr,
			"bus_addr" => payment.bus_addr, "bus_info" => payment.bus_info, "reg_capital" => payment.reg_capital,
			"linkman" => payment.linkman, "telephone" => payment.telephone, "link_email" => payment.link_email,
			"account_note1" => payment.account_note1, "account_note2" => payment.account_note2, "account_note3" => payment.account_note3,
			"bus_connect" => payment.bus_connect, 
			"person_info" => serde_json::to_value(payment.person_info).unwrap(), "bank_info" => serde_json::to_value(payment.bank_info).unwrap()
        }
    });
	match conn.exec_batch(r"INSERT INTO business(id, bus_type, bus_name, credit_code, validity_begin, 
		validity_end, reg_addr, bus_addr, bus_info, reg_capital, linkman, telephone, link_email, account_note1,account_note2,account_note3, 
		bus_connect, person_info, bank_info)
	VALUES(:id, :bus_type, :bus_name, :credit_code, :validity_begin, :validity_end, :reg_addr, :bus_addr, 
		:bus_info, :reg_capital, :linkman, :telephone, :link_email, :account_note1, :account_note2, :account_note3, 
		:bus_connect, :person_info, :bank_info)",params_sql).await{
	   Ok(_) =>{},
	   Err(error) => {
		   warn!("1.insert business data failed for business of:{:?}",error);
		   return HttpResponse::Ok().json(ResponseBody::<String>::return_unwrap_error(error.to_string()));
	   }
	};
    return HttpResponse::Ok().json(ResponseBody::<BusinessRequest>::new_success(Some(params)));
 }
