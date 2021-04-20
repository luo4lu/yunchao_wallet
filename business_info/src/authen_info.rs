use log::{warn};
use serde::{Deserialize, Serialize};
use std::str;


//个人认证服务请求结构体
#[derive(Deserialize, Debug)]
pub struct PersonalInfo{
    name: String,   //姓名
    phone: String,   //使用电话号
    idcard: String,     //身份证
}
//第三方请求数据结构
#[derive(Serialize, Debug)]
pub struct JuheRequest{
    pub key: String,
    pub realname: String,
    pub idcard: String,
    pub mobile:String,
}

//第三方响应数据中的result字段
#[derive(Deserialize, Serialize, Debug)]
pub struct ResultValue{
    pub realname:String,
    pub mobile:String,
    pub idcard:String,
    pub res:i32,
    pub resmsg: String,
}
//第三方响应数据结构
#[derive(Deserialize, Serialize, Debug)]
pub struct JuheRespones{
    pub reason: String,
    pub result: serde_json::Value,
    pub error_code: i32,
}

/*
 *function: 用户实名认证（个人）
 *param: 
 *name: 姓名
 *phone: 电话号码
 *idcard:身份证号
 *
 *return：响应数据code=1成功,返回0标识认证过程请求失败，2表示三要素信息不匹配
*/
pub async fn personal_cartification(name: &String,phone: &String, idcard: &String) -> i32{
//发送验证信息
    //认证信息发送地址
    let addr = String::from("http://v.juhe.cn/telecom/query");
    let key = String::from("abff3ccf515d944cbe91354257f2265b");
    let params = JuheRequest{
        key,
        realname: name.to_string(),
        idcard: idcard.to_string(),
        mobile: phone.to_string(),
    };
    let user_client = reqwest::Client::new();
    let res = user_client.post(&addr).query(&params).send().await.unwrap();
    let response: JuheRespones = match res.json().await{
        Ok(value) =>{
            value
        }
        Err(error) =>{
            warn!("聚合链接请求失败:{}",error.to_string());
            return 0;
        }
    };
    if response.error_code == 220807{
        warn!("个人信息认证失败,身份信息不合法:{}",response.reason);
        return 0;
    }
    let return_result: ResultValue = match serde_json::from_value(response.result){
        Ok(value) => {
            value
        }
        Err(error) => {
            warn!("聚合认证数据解析失败:{}",error.to_string());
            return 0;
        }
    };
    return return_result.res;
}

/*
 *function: 用户实名认证（企业）
 *param: 
 *corporate: 企业法人
 *name: 企业名称
 *code:注册号/统一社会信用代码
 *
 *return：响应数据code=0成功，其他值参照错误列表 
*/
//第三方请求数据结构（企业)
#[derive(Deserialize, Serialize, Debug)]
pub struct EnterpriseRequest{
    pub key: String,
    pub keyword: String,
}
//数据存储info字段
#[derive(Deserialize, Serialize, Debug)]
pub struct ResultData{
    sign: String,
    data: serde_json::Value,
}

pub async fn enterprise_cartification(corporate: &String, code: &String) -> i32{
    
    //认证地址
    let addr = String::from("http://japi.juhe.cn/enterprise/getDetailByName");
    let key = String::from("a1363901d10216188683d5f985bc479a");
    let params = EnterpriseRequest{
        key,
        keyword: code.to_string(),
    };
    let user_client = reqwest::Client::new();
    let res = user_client.post(&addr).query(&params).send().await.unwrap();
    let res_value: JuheRespones = match res.json().await{
        Ok(value) =>{
            value
        }
        Err(error) =>{
            warn!("request enterprise carfitication info failde!:{}",error.to_string());
            return 1;
        }
    };
    if res_value.error_code != 0 {
        warn!("certification failed,error code={}---reason={}",res_value.error_code, res_value.reason);
        return res_value.error_code;
    }
    let result_data:ResultData = serde_json::from_value(res_value.result.clone()).unwrap();
    let result_type: serde_json::Value = serde_json::from_value(result_data.data).unwrap();
    let result_oper_name: String = result_type["oper_name"].to_string();
    let input_corporate:String = "\"".to_owned() + &corporate +"\"";
    if input_corporate != result_oper_name{
        warn!("enterprise corporate inconformity：input:{} != output: {}",input_corporate, result_oper_name);
        return 1;
    }
    return res_value.error_code;
}