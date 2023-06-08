FROM python:3.11

COPY . .
RUN pip install -r requirements.txt

EXPOSE 4433

CMD ["python", "http3_server.py", "--certificate", "ssl_cert.pem", "--private-key", "ssl_key.pem"]