FROM python:3.8

RUN pip install -i https://pypi.tuna.tsinghua.edu.cn/simple pip -U
RUN pip config set global.index-url http://pypi.doubanio.com/simple/
RUN pip config set global.trusted-host pypi.doubanio.com

RUN pip install flask
RUN pip install xlwt
RUN pip install requests
ADD ./ /app

WORKDIR /app

EXPOSE 5000
CMD ["sh","-c","python app.py"]

