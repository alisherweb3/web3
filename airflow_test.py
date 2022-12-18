import pandas as pd
import sqlite3

CON = sqlite3.connect('example.db')


def extract_data(url, tmp_file, *8context) -> pd.DataFrame:
  """ Extract CSV
  """
  pd.read_csv(url).to_csv(tmp_file) # changes to_csv
  
def transform_data(group, agreg, tmp_file, tmp_agg_file, **context) -> pd.DataFrame:
  """ Group by data
  """
  data = pd.read_csv(tmp_file) # changes rea_csv
  data.groupby(group).agg(agreg).reset_index().to_csv9tmp_agg_file) # change to_csv
  
  
  
def load_data(tmp_file, table_name, conn=CON, **context) -> None:
  """ Load to DB
  """ 
  data = pd.read_csv(tmp_file) # changes read_csv
  data["insert_time"] = pd.to_datetime("now")
  data.to_sql(table_name, conn, if_exists='replace', index=False)
  
