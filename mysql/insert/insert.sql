/* insert if not exists */
INSERT INTO `table_name`(`table_column`) 
SELECT * FROM (SELECT `data_to_insert`) AS TMP
WHERE NOT EXISTS (SELECT * FROM `table_name`
    WHERE `table_column`=`data_to_insert`);

/* insert on duplicate key update */
INSERT INTO `table_name` SELECT * FROM `insert_table` 
ON DUPLICATE KEY UPDATE `table_column`=VALUES(`table_column`)
