from airflow.models import BaseOperator
from airflow.providers.postgres.hooks.postres import PostgresHook

class PostfresOperator(BaseOperator):
  
  def __init__(self, **kwargs,) -> None:
    super().init__(**kwwargs)
    
  def execute(self, context):
    # creating hook
    self.hook = PostgresHook(postgres_conn_id=self.postgres_conn_id, schema=self.database)
    # call a method
    self.hook.run(self.sql, self.autocommit, parameters=self.parameters)
