FROM tiangolo/uwsgi-nginx-flask:python3.8-alpine

RUN pip install flask
RUN pip install xlwt
RUN pip install smtplib
RUN pip install requests
ADD ./ /app

