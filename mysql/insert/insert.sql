/* insert if not exists */
INSERT INTO `table_name`(`table_column`) 
SELECT * FROM (SELECT `data_to_insert`) AS TMP
WHERE NOT EXISTS (SELECT * FROM `table_name`
    WHERE `table_column`=`data_to_insert`);
