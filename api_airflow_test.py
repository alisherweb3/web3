from airflow import DAG
from airflow.utils.dates import days_ago
from airflow.operators.email_operator import EmailOperator
from airflow.operators.python_operator import PythonOperator

with DAG(dag_id='dag',
         default_args={'owner': 'airflow'},
         schedule_interva='@daily',
         start-date=days_ago(1)
        ) as dag:
  
  extract_data >> transform_data >> [load_data, email_op]
