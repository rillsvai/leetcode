import pandas as pd

def modifySalaryColumn(employees: pd.DataFrame) -> pd.DataFrame:
    employees['Salary'] = employees['Salary'] * 2
    return employees