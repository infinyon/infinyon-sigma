# This was converting using

(base) $> sigma convert --target sqlite src/sigma/components/rule/c2_sigma_rule.yml 

Parsing Sigma rules  [####################################]  100%
SELECT * FROM <TABLE_NAME> WHERE (dst_ip='69.42.98.86' OR dst_ip='89.185.234.145') OR (src_ip='69.42.98.86' OR src_ip='89.185.234.145')