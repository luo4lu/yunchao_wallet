import smtplib
import os,base64
import xlwt
import shutil
from write_data import write_excel 
from email import encoders
from email.mime.base import MIMEBase #附件
from email.mime.text import MIMEText
from email.mime.multipart import MIMEMultipart
from email.header import Header
from flask import Flask,request,json,make_response
import requests
import zipfile

app = Flask(__name__)

def email_smtp(path_name, bus_name):
    with open('../config/config.json','r',encoding='utf8')as fp:
        json_data = json.load(fp)
        #第三方SMTP服务
        mail_host=json_data.get("mail_host") #设置服务器
        mail_user=json_data.get("mail_user") #用户名
        mail_pass=json_data.get("mail_pass") #密码或者授权码

        sender = json_data.get("mail_user")
        receivers = json_data.get("receivers")
        global msg_list
        msg_list = MIMEMultipart()
        msg_list['From'] = Header("企业信息审核",'utf-8')
        msg_list['To'] = Header("注册信息",'utf-8')
        msg_list['subject'] = Header(bus_name,'utf-8')
        
        with open(path_name,'rb') as f:
            file_name = bus_name + '.zip'
            mime = MIMEBase('zip', 'zip', filename = file_name)
            mime.add_header('Content-Disposition','attachment',filename=('gb2312', '', file_name))
            mime.add_header('Content-ID','<0>')
            mime.add_header('X-Attachment-Id','0')
            #把附件的内容读进来
            mime.set_payload(f.read())
            #用Base64编码
            encoders.encode_base64(mime)
            msg_list.attach(mime)
        try:
            smtpObj = smtplib.SMTP_SSL(mail_host)
            smtpObj.ehlo(mail_host)
            smtpObj.login(mail_user,mail_pass)
            smtpObj.sendmail(sender,receivers,msg_list.as_string())
            print("邮件发送成功")
        except smtplib.SMTPException:
            print ("Error: 无法发送邮件")
            return 1
    return 0


#字符串解码存储
def image_save(addr, ms):
    #imagedata = base64.b64decode(ms)
    file = open(addr, "wb")
    file.write(ms.content)
    file.close()
#文件压缩
def zip_ya(startdir, file_news):
    z = zipfile.ZipFile(file_news,'w',zipfile.ZIP_DEFLATED) #参数一：文件夹名
    for dirpath, dirnames, filenames in os.walk(startdir):
        fpath = dirpath.replace(startdir,'') #这一句很重要，不replace的话，就从根目录开始复制
        fpath = fpath and fpath + os.sep or ''#这句话理解我也点郁闷，实现当前文件夹以及包含的所有文件的压缩
        for filename in filenames:
            z.write(os.path.join(dirpath, filename),fpath+filename)
    z.close()


@app.route('/user/consult', methods=['POST'])
def email_server():
    if request.method == 'POST':
        recv_data = request.data.decode('utf-8')
        dic = json.loads(recv_data)
        person_json = dic.get("person_info")
        bus_name = dic.get("bus_name") #企业名称，创建对应名称文件夹
        #根据企业名称生成当前目录下文件夹
        path='./'+bus_name
        folder = os.path.exists(path)
        if not folder:
            os.makedirs(path)
        business = dic.get("business") #营业执照找片
        if business is not None:
            image_name = path+ '/营业执照.jpg'
            image_save(image_name, requests.get(business)) 
        attached = dic.get("attached") #附件
        if attached is not None:
            att_name = path+'/附件.jpg'
            image_save(att_name, requests.get(attached))
        legal_photo_p = person_json.get("legal_photo_p")#法人身份证正面
        if legal_photo_p is not None:
            l_photo_p = path+'/法人身份证正面.jpg'
            image_save(l_photo_p, requests.get(legal_photo_p))
        legal_photo_r = person_json.get("legal_photo_r")#法人身份证反面
        if legal_photo_r is not None:
            l_photo_r = path+'/法人身份证反面.jpg'
            image_save(l_photo_r, requests.get(legal_photo_r))
        identity = person_json.get("identity") #填写人身份
        if identity == 'agency':
            agency_photo_p = person_json.get("agency_photo_p")
            if agency_photo_p is not None:
                a_photo_p = path+'/代理人身份证正面.jpg' #代理人身份证正面
                image_save(a_photo_p,requests.get(agency_photo_p))
            agency_photo_r = person_json.get("agency_photo_r") #代理人身份证反面
            if agency_photo_r is not None:
                a_photo_r = path+'/代理人身份证反面.jpg' 
                image_save(a_photo_r, requests.get(agency_photo_r))
            authorization = person_json.get("authorization") #代理委托书
            if authorization is not None:
                auth = path+'/委托书.jpg'
                image_save(auth, requests.get(authorization))
        #数据写入excel
        excel_name = path + '/'+bus_name
        write_excel(excel_name, dic)
        #文件压缩
        file_news = path + '.zip'
        zip_ya(path, file_news)
        code = email_smtp(file_news, bus_name+'注册信息')
    resp_dict = {}
    resp_dict['code'] = 0
    resp_dict['message'] = 'success'
    if code != 0:
        resp_dict['code'] = 554
        resp_dict['message'] = '发送失败'
    
    os.remove(file_news)
    shutil.rmtree(path)
    response = make_response(json.dumps(resp_dict))
    return response

@app.route('/change/password', methods=['POST'])
def change_pass():
    if request.method == 'POST':
        recv_data = request.data.decode('utf-8')
        dic = json.loads(recv_data)
        wallet_id = dic.get("wallet_id")
        authen_file = dic.get("authen_file")
        path='./'+wallet_id
        folder = os.path.exists(path)
        if not folder:
            os.makedirs(path)
        if authen_file is not None:
            image_name = path+'/认证文件.jpg'
            image_save(image_name, authen_file)
         #文件压缩
        file_news = path + '.zip'
        zip_ya(path, file_news)
        code = email_smtp(file_news, wallet_id+'修改密码')
    resp_dict = {}
    resp_dict['code'] = 0
    resp_dict['message'] = 'success'
    if code != 0:
        resp_dict['code'] = 554
        resp_dict['message'] = '发送失败'
    resp_dict['data'] = {}
    resp_dict['data']['wallet_id'] = wallet_id
    os.remove(file_news)
    shutil.rmtree(path)
    response = make_response(json.dumps(resp_dict))
    return response


if __name__=="__main__":
    app.run("0.0.0.0",5000)