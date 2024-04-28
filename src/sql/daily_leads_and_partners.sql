select date_id, make_name, COUNT (DISTINCT lead_id) as unique_leads, COUNT (DISTINCT partner_id) as unique_partners from dailysales
group by date_id, make_name 