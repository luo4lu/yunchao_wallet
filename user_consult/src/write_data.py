import os,base64
import xlwt
import time, datetime
from flask import Flask,request,json,make_response
#写入excel
def write_excel(excel_name, dic):
    workbook = xlwt.Workbook(encoding = 'utf-8')
    worksheet = workbook.add_sheet('My Worksheet')
    #写入 行  列 值
    worksheet.write(0, 0, '名称')
    worksheet.write(0, 1, '值')
    bus_type = dic.get("bus_type")
    worksheet.write(1,0,'单位类型')
    worksheet.write(1,1,bus_type)
    bus_name = dic.get("bus_name")
    worksheet.write(2,0,'企业名称')
    worksheet.write(2,1,bus_name)
    credit_code = dic.get("credit_code")
    worksheet.write(3,0,'社会信用代码')
    worksheet.write(3,1,credit_code)
    validity_begin = dic.get("validity_begin")
    validity_end = dic.get("validity_end")
    worksheet.write(4, 0, '营业执照有效期')
    if validity_begin is not None:
        date_vali1 = datetime.datetime.fromtimestamp(validity_begin)
        time1 = date_vali1.strftime("%Y--%m--%d %H:%M:%S")
        worksheet.write(4, 1, time1)
    if validity_end is not None and validity_end > 0 :
        date_vali2 = datetime.datetime.fromtimestamp(validity_end)
        time2 = date_vali2.strftime("%Y--%m--%d %H:%M:%S")
        worksheet.write(4, 2, time2)
    else:
        worksheet.write(4, 2, '长期')
    reg_addr = dic.get("reg_addr")
    worksheet.write(5, 0, '企业注册地址')
    worksheet.write(5, 1, reg_addr)
    bus_addr = dic.get("bus_addr")
    worksheet.write(6, 0, '企业经营地址')
    worksheet.write(6, 1, bus_addr)
    bus_info = dic.get("bus_info")
    worksheet.write(7, 0, '经营范围')
    worksheet.write(7, 1, bus_info)
    reg_capital = dic.get("reg_capital")
    worksheet.write(8, 0, '注册资本')
    worksheet.write(8, 1, reg_capital)
    linkman = dic.get("linkman")
    worksheet.write(9, 0, '企业联系人姓名')
    worksheet.write(9, 1, linkman)
    telephone = dic.get("telephone")
    worksheet.write(10, 0, '企业联系人手机')
    worksheet.write(10, 1, telephone)
    link_email = dic.get("link_email")
    worksheet.write(11, 0, '企业联系人邮箱')
    worksheet.write(11, 1, link_email)
    account_note1 = dic.get("account_note1")
    worksheet.write(12, 0, '账户备注1')
    worksheet.write(12, 1, account_note1)
    account_note2 = dic.get("account_note2")
    worksheet.write(13, 0, '账户备注2')
    worksheet.write(13, 1, account_note2)
    account_note3 = dic.get("account_note3")
    worksheet.write(14, 0, '账户备注3')
    worksheet.write(14, 1, account_note3)
    bus_connect = dic.get("bus_connect")
    worksheet.write(15, 0, '企业关系')
    worksheet.write(15, 1, bus_connect)
    
    person_json = dic.get("person_info")
    identity = person_json.get("identity")
    worksheet.write(17, 0, "填写人")
    if identity == "agency":
        worksheet.write(17, 1, '代理人')
    else:
        worksheet.write(17, 1, '法定代表人')
    legal_name = person_json.get("legal_name")
    worksheet.write(18, 0, '法定代表人姓名')
    worksheet.write(18, 1, legal_name)
    legal_voucher_type = person_json.get("legal_voucher_type")
    worksheet.write(19, 0, '证件类型')
    worksheet.write(19, 1, legal_voucher_type)
    legal_voucher_num = person_json.get("legal_voucher_num")
    worksheet.write(20, 0, '证件号码')
    worksheet.write(20, 1, legal_voucher_num)
    legal_validity_begin = person_json.get("legal_validity_begin")
    legal_validity_end = person_json.get("legal_validity_end")
    worksheet.write(21, 0, '证件有效期')
    if legal_validity_begin is not None:
        date_vali3 = datetime.datetime.fromtimestamp(legal_validity_begin)
        time3 = date_vali3.strftime("%Y--%m--%d %H:%M:%S")
        worksheet.write(21, 1, time3)
    if legal_validity_begin is not None and legal_validity_end > 0:
        date_vali4 = datetime.datetime.fromtimestamp(legal_validity_end)
        time4 = date_vali4.strftime("%Y--%m--%d %H:%M:%S")
        worksheet.write(21, 2, time4)
    else:
        worksheet.write(21, 2, '长期')
    legal_phone = person_json.get("legal_phone")
    worksheet.write(22, 0, '法定代表人手机')
    worksheet.write(22, 1, legal_phone)
    control_preson = person_json.get("control_preson")
    worksheet.write(23, 0, '实际控制人身份')
    worksheet.write(23, 1, control_preson)

    num = 24
    if identity == "agency":
        num +=1
        agency_name = person_json.get("agency_name")
        worksheet.write(num, 0, '代理人姓名')
        worksheet.write(num, 1, agency_name)
        agency_voucher_type = person_json.get("agency_voucher_type")
        num +=1
        worksheet.write(num, 0, '证件类型')
        worksheet.write(num, 1, agency_voucher_type)
        agency_voucher_num = person_json.get("agency_voucher_num")
        num +=1
        worksheet.write(num, 0, '证件号码')
        worksheet.write(num, 1, agency_voucher_num)
        agency_validity_begin = person_json.get("agency_validity_begin")
        agency_validity_end = person_json.get("agency_validity_end")
        num +=1
        date_vali5 = datetime.datetime.fromtimestamp(agency_validity_begin)
        worksheet.write(num, 0, '证件有效期')
        if agency_validity_begin is not None:
            time5 = date_vali5.strftime("%Y--%m--%d %H:%M:%S")
            worksheet.write(num, 1, time5)
        if agency_validity_end is not None and agency_validity_end > 0:
            date_vali6 = datetime.datetime.fromtimestamp(agency_validity_end)
            time6 = date_vali6.strftime("%Y--%m--%d %H:%M:%S")
            worksheet.write(num, 2, time6)
        else:
            worksheet.write(num, 2, '长期')
        agency_phone = person_json.get("agency_phone")
        num +=1
        worksheet.write(num, 0, '代理人手机')
        worksheet.write(num, 1, agency_phone)
        num +=1

    bank_json = dic.get("bank_info")
    account_type = bank_json.get("account_type")
    num +=1
    worksheet.write(num, 0, '银行账户类型')
    worksheet.write(num, 1, account_type)
    account_name = bank_json.get("account_name")
    num +=1
    worksheet.write(num, 0, '银行账户名称')
    worksheet.write(num, 1, account_name)
    account_number = bank_json.get("account_number")
    num +=1
    worksheet.write(num, 0, '银行账号')
    worksheet.write(num, 1, account_number)
    deposit_bank = bank_json.get("deposit_bank")
    num +=1
    worksheet.write(num, 0, '开户银行')
    worksheet.write(num, 1, deposit_bank)
    area = bank_json.get("area")
    num +=1
    worksheet.write(num, 0, '开户行所在地')
    worksheet.write(num, 1, area)
    sub_branch = bank_json.get("sub_branch")
    num +=1
    worksheet.write(num, 0, '开户支行名称')
    worksheet.write(num, 1, sub_branch)
    #保存
    file_name = excel_name + '.xls'
    workbook.save(file_name)
