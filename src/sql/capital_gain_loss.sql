SELECT stock_name, SUM(CASE WHEN operation = 'Buy' THEN -price ELSE price END) as capital_gain_loss FROM stocks
GROUP BY stock_name